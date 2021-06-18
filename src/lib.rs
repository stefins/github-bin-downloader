pub type GBDResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub mod cli;
pub mod ghapi;
pub mod sysinfo;
pub mod utils;
