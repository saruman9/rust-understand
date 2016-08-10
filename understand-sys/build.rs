use std::env;
use std::fmt;
use std::path::PathBuf;
use std::process::Command;

struct UnderstandVersion {
    major: i32,
    minor: i32,
    build: i32,
    os: Os,
    arch: Arch,
}

#[allow(dead_code)]
enum Arch {
    Bit32,
    Bit64,
    X86,
    Sparc,
}

impl fmt::Display for Arch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Arch::Bit32 => write!(f, "32bit"),
            Arch::Bit64 => write!(f, "64bit"),
            Arch::X86 => write!(f, "x86"),
            Arch::Sparc => write!(f, "Sparc")
        }
    }
}

#[allow(dead_code)]
enum Os {
    MacOSX,
    Linux,
    LinuxLegacy,
    Solaris,
    Windows,
}

impl fmt::Display for Os {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Os::MacOSX => write!(f, "MacOSX"),
            Os::Linux => write!(f, "Linux"),
            Os::LinuxLegacy => write!(f, "LinuxLegacy"),
            Os::Solaris => write!(f, "Solaris"),
            Os::Windows => write!(f, "Windows")
        }
    }
}

const UNDERSTAND_RELEASE: UnderstandVersion = UnderstandVersion {
    major: 4,
    minor: 0,
    build: 851,
    os: Os::Linux,
    arch: Arch::Bit64
};

fn main() {
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let ext_pathbuf: PathBuf = PathBuf::from(&cargo_manifest_dir).join("ext");

    let ext_type = match UNDERSTAND_RELEASE.os {
        Os::Linux | Os::LinuxLegacy | Os::Solaris => "tgz",
        Os::MacOSX => "dmg",
        Os::Windows => "zip"
    };
    let understand_tarball = format!("Understand-{major}.{minor}.{build}-{os}-{arch}.{ext}",
                                    major = UNDERSTAND_RELEASE.major,
                                    minor = UNDERSTAND_RELEASE.minor,
                                    build = UNDERSTAND_RELEASE.build,
                                    os = UNDERSTAND_RELEASE.os,
                                    arch = UNDERSTAND_RELEASE.arch,
                                    ext = ext_type);

    let understand_pathbuf: PathBuf = PathBuf::from(ext_pathbuf.to_path_buf()).join(&understand_tarball);
    let udb_api_pathbuf: PathBuf = PathBuf::from(ext_pathbuf.to_path_buf()).join("libudb_api.so");

    // TODO Replace pure Rust code
    if !udb_api_pathbuf.exists() {
        if !understand_pathbuf.exists() {
            let und_build_base_url = "http://builds.scitools.com/all_builds";
            let url_download = format!("{base}/b{build}/Understand/{tarball}",
                                    base = und_build_base_url,
                                    build = UNDERSTAND_RELEASE.build,
                                    tarball = understand_tarball);

            let output = Command::new("curl")
                .arg("-OL")
                .arg("-C")
                .arg("-")
                .arg(url_download)
                .current_dir(ext_pathbuf.as_path())
                .output()
                .unwrap_or_else(|e| {
                    panic!("failed to execute curl: {}", e)
                });
            if !output.status.success() {
                panic!("curl executed with error:\n{}",
                    String::from_utf8_lossy(&output.stderr))
            }
        }

        let os: &'static str = match UNDERSTAND_RELEASE.os {
            Os::Linux | Os::LinuxLegacy => "linux",
            Os::Windows => "windows",
            // TODO Solaris, MacOS, etc
            _ => "???"
        };
        let arch: &'static str = match UNDERSTAND_RELEASE.arch {
            Arch::Bit32 => "32",
            Arch::Bit64 => "64",
            // TODO Sparc, x86
            _ => "???"
        };
        let ext: &'static str = match UNDERSTAND_RELEASE.os {
            Os::Linux | Os::LinuxLegacy | Os::Solaris => "so",
            Os::MacOSX => "dylib",
            Os::Windows => "dll"
        };
        let libudb_api: String = format!("scitools/bin/{os}{arch}/libudb_api.{ext}",
                                            os = os,
                                            arch = arch,
                                            ext = ext
        );

        let output = Command::new("tar")
            .arg("xf")
            .arg(understand_pathbuf.as_os_str())
            .arg(libudb_api)
            .arg("--strip=3")
            .current_dir(ext_pathbuf.as_path())
            .output()
            .unwrap_or_else(|e| {
                panic!("tar executed with error:\n{}", e)
            });
        if !output.status.success() {
            panic!("tar executed with error:\n{}",
                String::from_utf8_lossy(&output.stderr))
        }
    }
    println!("cargo:rustc-link-lib=udb_api");
    println!("cargo:rustc-link-search={}", ext_pathbuf.display());
}
