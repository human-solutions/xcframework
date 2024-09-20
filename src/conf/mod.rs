mod configuration;
mod targets;
mod xc_cli;
mod xcframework;

pub use crate::conf::xcframework::{LibType, XCFrameworkConfiguration};
pub use configuration::Configuration;
pub use targets::Target;
pub use xc_cli::XcCli;
