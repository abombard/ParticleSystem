extern crate glutin;
extern crate gl;

use gl::types::*;

// Vertex data
static VERTEX_DATA: [GLfloat; 6] = [
     0.0,  0.5,
     0.5, -0.5,
    -0.5, -0.5
];

// Shader sources
static VERTEX_SHADER_SOURCE: &'static str = "
    #version 150 core

    in vec2 position;

    void main() {
       gl_Position = vec4(position, 0.0, 1.0);
    }
    ";

static FRAGMENT_SHADER_SOURCE: &'static str = "
    #version 150 core

    out vec4 out_color;

    void main() {
       out_color = vec4(1.0, 1.0, 1.0, 1.0);
    }
    ";

fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader;
    unsafe
    {
        shader = gl::CreateShader(ty);

        // Attempt to compile shader
        let c_str = std::ffi::CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(
            shader,
            1,
            &c_str.as_ptr(),
            std::ptr::null()
        );
        gl::CompileShader(shader);

        // Get the compile status
        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(
            shader,
            gl::COMPILE_STATUS,
            &mut status
        );

        // handle error
        if status != (gl::TRUE as GLint) {

            let mut len = 0;
            gl::GetShaderiv(
                shader,
                gl::INFO_LOG_LENGTH,
                &mut len
            );

            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // skip '\0'
            gl::GetShaderInfoLog(
                shader,
                len,
                std::ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar
            );

            panic!("{}", std::str::from_utf8(&buf).ok().expect("ShaderInfoLog not valid utf8"));
        }
    }
    shader
}

fn link_program(vertex_shader: GLuint, fragment_shader: GLuint) -> GLuint {
    unsafe
    {
        let program = gl::CreateProgram();

        gl::AttachShader(
            program,
            vertex_shader
        );
        gl::AttachShader(
            program,
            fragment_shader
        );

        gl::LinkProgram(program);

        // Get the link status
        let mut status = gl::FALSE as GLint;

        gl::GetProgramiv(
            program,
            gl::LINK_STATUS,
            &mut status
        );

        // handle error
        if status != (gl::TRUE as GLint) {

            let mut len: GLint = 0;
            gl::GetProgramiv(
                program,
                gl::INFO_LOG_LENGTH,
                &mut len
            );

            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1); // skip '\0'
            gl::GetProgramInfoLog(
                program,
                len,
                std::ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar
            );

            panic!("{}", std::str::from_utf8(&buf).ok().expect("ProgramInfoLog not valid utf8"));
        }

        program
    }
}

fn main() {

    // Get a window
    let builder = glutin::WindowBuilder::new();

    let window = builder
        .with_title("Hello world!".to_string())
        .with_dimensions(480, 360)
        .with_gl(glutin::GlRequest::Latest)
        //.with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 1)));
        .build()
    .unwrap();

    // Make the context current
    unsafe { window.make_current().unwrap() };

    // Load the OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    // Create GLSL shaders
    let vertex_shader = compile_shader(
        VERTEX_SHADER_SOURCE,
        gl::VERTEX_SHADER
    );
    let fragment_shader = compile_shader(
        FRAGMENT_SHADER_SOURCE,
        gl::FRAGMENT_SHADER
    );
    // Create Program
    let program = link_program(
        vertex_shader,
        fragment_shader
    );

    let mut vao = 0;
    let mut vbo = 0;

    unsafe
    {
        // Create a Vertex Array Object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Create a Vertex Buffer Object and copy the vertex data to it
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (VERTEX_DATA.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
            std::mem::transmute(&VERTEX_DATA[0]),
            gl::STATIC_DRAW
        );
        
        // Use shader program
        gl::UseProgram(program);
        gl::BindFragDataLocation(
            program,
            0,
            std::ffi::CString::new("out_color").unwrap().as_ptr()
        );

        // Specify the layout of the vertex data
        let pos_attr = gl::GetAttribLocation(
            program,
            std::ffi::CString::new("position").unwrap().as_ptr()
        );
        gl::EnableVertexAttribArray(pos_attr as GLuint);
        gl::VertexAttribPointer(
            pos_attr as GLuint,
            2,
            gl::FLOAT,
            gl::FALSE as GLboolean,
            0,
            std::ptr::null()
        );

    }

    unsafe { gl::ClearColor(0.0, 0.0, 0.0, 0.0) };

    'main: loop {

        unsafe
        {
            // Clear screen
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // Draw a triangle (3 vertices)
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }

        window.swap_buffers().unwrap();

        // polling and handling events
        for event in window.poll_events() {
            match event {
                glutin::Event::Closed => break 'main,
                _ => ()
            }
        }

    }

    // Cleanup
    unsafe
    {
        gl::DeleteProgram(program);
        gl::DeleteShader(fragment_shader);
        gl::DeleteShader(vertex_shader);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteVertexArrays(1, &vao);
    }
}
