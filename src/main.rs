#[macro_use] extern crate glium;
extern crate ocl;
#[macro_use] extern crate colorify;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

fn info () {
	use ocl::{Platform, Device, Context, Queue, Buffer, Image, Sampler, Program, Kernel, Event, EventList};
	use ocl::core::{self, PlatformInfo, DeviceInfo, ContextInfo, CommandQueueInfo, MemInfo, ImageInfo,
		SamplerInfo, ProgramInfo, ProgramBuildInfo, KernelInfo, KernelArgInfo, KernelWorkGroupInfo,
		EventInfo, ProfilingInfo};
	use ocl::util;

	const DIMS: [usize; 3] = [1024, 64, 16];
	const INFO_FORMAT_MULTILINE: bool = true;

	static SRC: &'static str = r#"
		__kernel void multiply(float coeff, __global float *buffer) {
			buffer[get_global_id(0)] *= coeff;
		}
	"#;

	let platforms = Platform::list ();
	for platform in platforms.iter () {
		for device in Device::list_all (&platform.clone ()) {
			let device_version = device.version ().unwrap ();

			let context = Context::builder ()
				.platform (platform.clone ())
				.devices (device)
				.build ()
				.unwrap ();
			let program = Program::builder ()
				.devices (device)
				.src (SRC)
				.build (&context)
				.unwrap ();
			let queue = Queue::new (&context, device)
				.unwrap ();
			let buffer = Buffer::<f32>::new (&queue.clone (),
											 None,
											 &DIMS,
											 None)
				.unwrap ();
			let image = Image::<u8>::builder ()
				.dims (&DIMS)
				.build (&queue)
				.unwrap ();
			let sampler = Sampler::with_defaults (&context)
				.unwrap ();
			let kernel = Kernel::new ("multiply", &program, &queue)
				.unwrap ()
				.gws (&DIMS)
				.arg_scl (10.0f32)
				.arg_buf (&buffer);

			let mut event_list = EventList::new ();
			kernel.cmd ().enew (&mut event_list)
				.enq ()
				.unwrap ();
			event_list.wait ().unwrap ();

			let mut event = Event::empty ();
			buffer.cmd ().write (&vec![0.0; DIMS[0]])
				.enew (&mut event)
				.enq ()
				.unwrap ();
			event.wait ().unwrap ();

			println!("### OpenCL Platform-Device Full Info ###");
			print!("\n");

			let (begin, delim, end) =
				if INFO_FORMAT_MULTILINE {
					("\n", "\n", "\n")
				} else {
					("{ ", ", ", " }")
				};

			println!("Platform:\n\
					 {t}Profile: {}\n\
					 {t}Version: {}\n\
					 {t}Name: {}\n\
					 {t}Vendor: {}\n\
					 {t}Extensions: {}\n",
				core::get_platform_info(context.platform ().unwrap (), PlatformInfo::Profile),
				core::get_platform_info(context.platform ().unwrap (), PlatformInfo::Version),
				core::get_platform_info(context.platform ().unwrap (), PlatformInfo::Name),
				core::get_platform_info(context.platform ().unwrap (), PlatformInfo::Vendor),
				core::get_platform_info(context.platform ().unwrap (), PlatformInfo::Extensions),
				t = util::colors::TAB,
            );
        }
    }
}

fn main () {
    use glium::{ DisplayBuild, Surface, VertexBuffer, Program };
	use glium::glutin::{GlRequest, Api, GlProfile, Event};

    let gl_request = GlRequest::Specific(Api::OpenGl, (4, 1));

    let display =
    glium::glutin::WindowBuilder::new ()
        .with_gl (gl_request)
        .with_gl_profile (GlProfile::Core)
        //.whith_vsync ()
        .build_glium ()
        .unwrap ();

    let v1 = Vertex { position: [-0.5, -0.5 ] };
    let v2 = Vertex { position: [ 0.0,  0.5 ] };
    let v3 = Vertex { position: [ 0.5, -0.25] };
    let shape = vec![v1, v2, v3];

    let vertex_buffer = VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 150

        in vec2 position;
        
        void main () {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 150

        out vec4 color;
        
        void main () {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    let program = Program::from_source(&display,
									   vertex_shader_src,
									   fragment_shader_src,
									   None)
		.unwrap();

    loop {
        let mut target = display.draw ();
        target.clear_color (0.0, 0.0, 1.0, 1.0);
        target.draw (&vertex_buffer,
                     &indices,
                     &program,
                     &glium::uniforms::EmptyUniforms,
                     &Default::default ()).unwrap ();
        target.finish ().unwrap ();

        for event in display.poll_events () {
            match event {
                Event::Closed => return,
                _ => ()
            }
        }
    }
}