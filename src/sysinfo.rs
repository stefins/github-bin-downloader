use nix::sys::utsname;

pub const APPLE: [&str; 4] = ["macos", "darwin", "mac", "dmg"];
pub const LINUX: [&str; 1] = ["linux"];
pub const AMD64: [&str; 4] = ["x64", "x86_64", "amd64", "64bit"];
pub const ARM64: [&str; 2] = ["aarch64", "arm64"];

#[derive(Debug, Default)]
pub struct SystemInfo {
    pub arch: String,
    pub os: String,
}

#[derive(Debug)]
pub enum PlatformOS {
    Windows,
    Linux,
    Darwin,
    Unknown,
}

#[derive(Debug)]
pub enum PlatformArch {
    X8664,
    Arm64,
    Unknown,
}

impl SystemInfo {
    pub fn new() -> Self {
        let uts_name = utsname::uname();
        Self {
            os: uts_name.sysname().to_lowercase(),
            arch: uts_name.machine().to_lowercase(),
        }
    }

    pub fn platform_os(&self) -> PlatformOS {
        for apple in APPLE.iter() {
            if self.os.contains(apple) {
                return PlatformOS::Darwin;
            }
        }
        for linux in LINUX.iter() {
            if self.os.contains(linux) {
                return PlatformOS::Linux;
            }
        }
        PlatformOS::Unknown
    }

    pub fn platform_arch(&self) -> PlatformArch {
        for amd64 in AMD64.iter() {
            if self.arch.contains(amd64) {
                return PlatformArch::X8664;
            }
        }

        for arm64 in ARM64.iter() {
            if self.arch.contains(arm64) {
                return PlatformArch::Arm64;
            }
        }
        PlatformArch::Unknown
    }
}
