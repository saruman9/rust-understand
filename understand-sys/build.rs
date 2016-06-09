use std::env;
use std::fmt;
use std::path::PathBuf;
use std::process::Command;

struct UnderstandVersion {
    major: i32,
    minor: i32,
    build: i32,
    os: OS,
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
enum OS {
    MacOSX,
    Linux,
    LinuxLegacy,
    Solaris,
    Windows,
}

impl fmt::Display for OS {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OS::MacOSX => write!(f, "MacOSX"),
            OS::Linux => write!(f, "Linux"),
            OS::LinuxLegacy => write!(f, "LinuxLegacy"),
            OS::Solaris => write!(f, "Solaris"),
            OS::Windows => write!(f, "Windows")
        }
    }
}

const UNDERSTAND_RELEASE: UnderstandVersion = UnderstandVersion {
    major: 4,
    minor: 0,
    build: 833,
    os: OS::Linux,
    arch: Arch::Bit64
};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let ext_pathbuf: PathBuf = PathBuf::from(cargo_manifest_dir.to_owned()).join("ext");

    let ext_type = match UNDERSTAND_RELEASE.os {
        OS::Linux | OS::LinuxLegacy | OS::Solaris => "tgz",
        OS::MacOSX => "dmg",
        OS::Windows => "zip"
    };
    let understand_tarball: String = format!("Understand-{major}.{minor}.{build}-{os}-{arch}.{ext}",
                                            major = UNDERSTAND_RELEASE.major,
                                            minor = UNDERSTAND_RELEASE.minor,
                                            build = UNDERSTAND_RELEASE.build,
                                            os = UNDERSTAND_RELEASE.os,
                                            arch = UNDERSTAND_RELEASE.arch,
                                            ext = ext_type);

    let understand_pathbuf: PathBuf = PathBuf::from(ext_pathbuf.to_path_buf()).join(understand_tarball.to_owned());

    let und_build_base_url = "http://builds.scitools.com/all_builds";
    let url_download = format!("{base}/b{build}/Understand/{tarball}",
                               base = und_build_base_url,
                               build = UNDERSTAND_RELEASE.build,
                               tarball = understand_tarball);

    if !understand_pathbuf.exists() {
        let status = Command::new("curl")
            .arg("-OL")
            .arg("-C")
            .arg("-")
            .arg(url_download)
            .current_dir(ext_pathbuf.as_path())
            .status().unwrap_or_else(|e| { panic!("failed to execute curl: {}", e) });
        if !status.success() {
            panic!("curl executed with error: {}", status);
        }
    }

    /*
    println!("understand_pathbuf: {}", understand_pathbuf.to_string_lossy());
    println!("out_dir: {}", out_dir);
    println!("cargo_manifest_dir: {}", cargo_manifest_dir);
    */
}
