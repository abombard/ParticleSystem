static BUILTINS: phf::Map<&'static str, TargetInfo> = ::phf::Map {
    key: 9603444721912725599,
    disps: ::phf::Slice::Static(&[
        (2, 5),
        (2, 43),
        (12, 29),
        (45, 10),
        (9, 45),
        (0, 0),
        (0, 7),
        (0, 0),
        (0, 0),
        (1, 33),
    ]),
    entries: ::phf::Slice::Static(&[
        ("armv7-unknown-linux-musleabihf", TargetInfo { arch: B("arm"), vendor: B(""), os: B("linux"), env: B("musl"), endian: B("little"), pointer_width: B("32") }),
        ("s390x-unknown-linux-gnu", TargetInfo { arch: B("s390x"), vendor: B(""), os: B("linux"), env: B("gnu"), endian: B("big"), pointer_width: B("64") }),
        ("mips64el-unknown-linux-gnuabi64", TargetInfo { arch: B("mips64"), vendor: B(""), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("64") }),
        ("x86_64-unknown-dragonfly", TargetInfo { arch: B("x86_64"), vendor: B(""), os: B("dragonfly"), env: B(""), endian: B("little"), pointer_width: B("64") }),
        ("arm-unknown-linux-musleabihf", TargetInfo { arch: B("arm"), vendor: B(""), os: B("linux"), env: B("musl"), endian: B("little"), pointer_width: B("32") }),
        ("i586-pc-windows-msvc", TargetInfo { arch: B("x86"), vendor: B(""), os: B("windows"), env: B("msvc"), endian: B("little"), pointer_width: B("32") }),
        ("arm-linux-androideabi", TargetInfo { arch: B("arm"), vendor: B(""), os: B("android"), env: B(""), endian: B("little"), pointer_width: B("32") }),
        ("mipsel-unknown-linux-musl", TargetInfo { arch: B("mips"), vendor: B(""), os: B("linux"), env: B("musl"), endian: B("little"), pointer_width: B("32") }),
        ("armv7-linux-androideabi", TargetInfo { arch: B("arm"), vendor: B(""), os: B("android"), env: B(""), endian: B("little"), pointer_width: B("32") }),
        ("x86_64-rumprun-netbsd", TargetInfo { arch: B("x86_64"), vendor: B(""), os: B("netbsd"), env: B(""), endian: B("little"), pointer_width: B("64") }),
        ("x86_64-unknown-linux-musl", TargetInfo { arch: B("x86_64"), vendor: B(""), os: B("linux"), env: B("musl"), endian: B("little"), pointer_width: B("64") }),
        ("arm-unknown-linux-musleabi", TargetInfo { arch: B("arm"), vendor: B(""), os: B("linux"), env: B("musl"), endian: B("little"), pointer_width: B("32") }),
        ("i686-unknown-linux-gnu", TargetInfo { arch: B("x86"), vendor: B(""), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("32") }),
        ("mips-unknown-linux-uclibc", TargetInfo { arch: B("mips"), vendor: B(""), os: B("linux"), env: B("uclibc"), endian: B("big"), pointer_width: B("32") }),
        ("i586-unknown-linux-gnu", TargetInfo { arch: B("x86"), vendor: B(""), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("32") }),
        ("i686-pc-windows-msvc", TargetInfo { arch: B("x86"), vendor: B(""), os: B("windows"), env: B("msvc"), endian: B("little"), pointer_width: B("32") }),
        ("powerpc64-unknown-linux-gnu", TargetInfo { arch: B("powerpc64"), vendor: B(""), os: B("linux"), env: B("gnu"), endian: B("big"), pointer_width: B("64") }),
        ("x86_64-apple-darwin", TargetInfo { arch: B("x86_64"), vendor: B(""), os: B("macos"), env: B(""), endian: B("little"), pointer_width: B("64") }),
        ("x86_64-sun-solaris", TargetInfo { arch: B("x86_64"), vendor: B(""), os: B("solaris"), env: B(""), endian: B("little"), pointer_width: B("64") }),
        ("i686-unknown-freebsd", TargetInfo { arch: B("x86"), vendor: B(""), os: B("freebsd"), env: B(""), endian: B("little"), pointer_width: B("32") }),
        ("mips-unknown-linux-musl", TargetInfo { arch: B("mips"), vendor: B(""), os: B("linux"), env: B("musl"), endian: B("big"), pointer_width: B("32") }),
        ("x86_64-unknown-linux-gnu", TargetInfo { arch: B("x86_64"), vendor: B(""), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("64") }),
        ("armv7-unknown-linux-gnueabihf", TargetInfo { arch: B("arm"), vendor: B(""), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("32") }),
        ("powerpc64le-unknown-linux-gnu", TargetInfo { arch: B("powerpc64"), vendor: B(""), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("64") }),
        ("aarch64-unknown-linux-gnu", TargetInfo { arch: B("aarch64"), vendor: B(""), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("64") }),
        ("x86_64-unknown-netbsd", TargetInfo { arch: B("x86_64"), vendor: B(""), os: B("netbsd"), env: B(""), endian: B("little"), pointer_width: B("64") }),
        ("mips64-unknown-linux-gnuabi64", TargetInfo { arch: B("mips64"), vendor: B(""), os: B("linux"), env: B("gnu"), endian: B("big"), pointer_width: B("64") }),
        ("powerpc-unknown-linux-gnu", TargetInfo { arch: B("powerpc"), vendor: B(""), os: B("linux"), env: B("gnu"), endian: B("big"), pointer_width: B("32") }),
        ("i686_unknown_haiku", TargetInfo { arch: B("x86"), vendor: B(""), os: B("haiku"), env: B(""), endian: B("little"), pointer_width: B("32") }),
        ("i686-linux-android", TargetInfo { arch: B("x86"), vendor: B(""), os: B("android"), env: B(""), endian: B("little"), pointer_width: B("32") }),
        ("i686-unknown-dragonfly", TargetInfo { arch: B("x86"), vendor: B(""), os: B("dragonfly"), env: B(""), endian: B("little"), pointer_width: B("32") }),
        ("x86_64-pc-windows-gnu", TargetInfo { arch: B("x86_64"), vendor: B(""), os: B("windows"), env: B("gnu"), endian: B("little"), pointer_width: B("64") }),
        ("mipsel-unknown-linux-gnu", TargetInfo { arch: B("mips"), vendor: B(""), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("32") }),
        ("x86_64-unknown-openbsd", TargetInfo { arch: B("x86_64"), vendor: B(""), os: B("openbsd"), env: B(""), endian: B("little"), pointer_width: B("64") }),
        ("arm-unknown-linux-gnueabi", TargetInfo { arch: B("arm"), vendor: B(""), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("32") }),
        ("i686-apple-darwin", TargetInfo { arch: B("x86"), vendor: B(""), os: B("macos"), env: B(""), endian: B("little"), pointer_width: B("32") }),
        ("x86_64-unknown-bitrig", TargetInfo { arch: B("x86_64"), vendor: B(""), os: B("bitrig"), env: B(""), endian: B("little"), pointer_width: B("64") }),
        ("mipsel-unknown-linux-uclibc", TargetInfo { arch: B("mips"), vendor: B(""), os: B("linux"), env: B("uclibc"), endian: B("little"), pointer_width: B("32") }),
        ("x86_64-unknown-freebsd", TargetInfo { arch: B("x86_64"), vendor: B(""), os: B("freebsd"), env: B(""), endian: B("little"), pointer_width: B("64") }),
        ("aarch64-linux-android", TargetInfo { arch: B("aarch64"), vendor: B(""), os: B("android"), env: B(""), endian: B("little"), pointer_width: B("64") }),
        ("arm-unknown-linux-gnueabihf", TargetInfo { arch: B("arm"), vendor: B(""), os: B("linux"), env: B("gnu"), endian: B("little"), pointer_width: B("32") }),
        ("mips-unknown-linux-gnu", TargetInfo { arch: B("mips"), vendor: B(""), os: B("linux"), env: B("gnu"), endian: B("big"), pointer_width: B("32") }),
        ("i686-pc-windows-gnu", TargetInfo { arch: B("x86"), vendor: B(""), os: B("windows"), env: B("gnu"), endian: B("little"), pointer_width: B("32") }),
        ("i686-unknown-linux-musl", TargetInfo { arch: B("x86"), vendor: B(""), os: B("linux"), env: B("musl"), endian: B("little"), pointer_width: B("32") }),
        ("x86_64_unknown_haiku", TargetInfo { arch: B("x86_64"), vendor: B(""), os: B("haiku"), env: B(""), endian: B("little"), pointer_width: B("64") }),
        ("x86_64-pc-windows-msvc", TargetInfo { arch: B("x86_64"), vendor: B(""), os: B("windows"), env: B("msvc"), endian: B("little"), pointer_width: B("64") }),
    ]),
};