mod configuration;
mod xc_cli;
mod xcframework;

pub use crate::cli::xcframework::{LibType, XCFrameworkConfiguration};
pub use configuration::Configuration;
pub use xc_cli::XcCli;
