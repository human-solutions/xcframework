use camino::Utf8PathBuf;
use clap::Parser;
use clap_cargo_extra::ClapCargo;

use crate::config::Config;

use super::LibType;

/// Compile a package into a cross-platform Apple XCFramework
#[derive(Debug, Parser)]
#[clap(version)]
pub struct XcCli {
    #[clap(flatten)]
    pub clap_cargo: ClapCargo,

    /// Chose library type to build when Cargo.toml `crate-type` has both.
    #[arg(long)]
    pub lib_type: Option<LibType>,

    /// Do not print cargo log messages
    #[arg(short, long)]
    pub quiet: bool,

    /// Use verbose output (-vv very verbose/build.rs output)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details
    #[arg(short = 'Z', value_name = "FLAG")]
    pub unstable_flags: Option<String>,

    /// Directory for all generated artifacts
    #[arg(long, value_name = "DIRECTORY")]
    pub target_dir: Option<Utf8PathBuf>,
}

impl XcCli {
    // TODO: Update this to use the new Config struct
    pub fn to_config(&self) -> Config {
        Config::default()
    }
}
