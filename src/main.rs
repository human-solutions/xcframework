use clap::Parser;
use xcframework::XcframeworkOp;

fn main() -> anyhow::Result<()> {
    let Command::Xcframework(ref matches) = Command::parse();

    matches.xcframework.run()?;

    Ok(())
}

#[derive(Debug, Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
#[command(styles = clap_cargo::style::CLAP_STYLING)]
pub enum Command {
    #[command(name = "xcframework")]
    #[command(about, author, version)]
    Xcframework(XcframeworkOpt),
}

/// Automated build XCFramework for Rust
#[derive(Debug, clap::Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct XcframeworkOpt {
    #[command(flatten)]
    pub xcframework: XcframeworkOp,

    #[command(flatten)]
    pub logging: Verbosity,
}

#[derive(clap::Args, Debug, Clone)]
#[command(next_help_heading = None)]
pub struct Verbosity {
    /// Pass many times for less log output
    #[arg(long, short, action = clap::ArgAction::Count, global = true)]
    quiet: u8,

    /// Pass many times for more log output
    ///
    /// By default, it'll report info. Passing `-v` one time adds debug
    /// logs, `-vv` adds trace logs.
    #[arg(long, short, action = clap::ArgAction::Count, global = true)]
    verbose: u8,
}

impl Verbosity {
    /// Get the log level.
    pub fn log_level(&self) -> log::Level {
        let verbosity = 2 - (self.quiet as i8) + (self.verbose as i8);

        match verbosity {
            i8::MIN..=0 => log::Level::Error,
            1 => log::Level::Warn,
            2 => log::Level::Info,
            3 => log::Level::Debug,
            4..=i8::MAX => log::Level::Trace,
        }
    }
}
