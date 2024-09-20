mod args;
mod configuration;
mod targets;
mod xcframework;

pub use crate::conf::xcframework::{LibType, XCFrameworkConfiguration};
pub use args::Xcframework as CliArgs;
pub use configuration::Configuration;
pub use targets::Target;
