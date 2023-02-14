mod cmd;
mod conf;
mod ext;

use crate::conf::Configuration;
use anyhow::Result;
use cmd::{cargo, lipo, rustup, xcodebuild, zip};
pub use conf::{XCFrameworkConfiguration, XcCli};
use ext::PathBufExt;

pub fn run(cli: XcCli) -> Result<()> {
    let conf = Configuration::load(cli)?;

    conf.build_dir.remove_dir_all_if_exists()?;

    rustup::check_needed(&conf)?;
    cargo::build(&conf)?;

    let libs = lipo::assemble_libs(&conf)?;
    xcodebuild::assemble(&conf, libs)?;
    if conf.cargo_section.zip {
        zip::xcframework(&conf)?;
    } else {
        let module_name = conf.module_name()?;
        let from = conf.build_dir.join(format!("{module_name}.xcframework"));
        let to = conf.target_dir.join(format!("{module_name}.xcframework"));
        fs_err::rename(from, to)?;
    }
    conf.build_dir.remove_dir_all_if_exists()?;

    Ok(())
}
