mod cmd;
mod conf;
mod ext;

use crate::conf::Configuration;
use anyhow::Result;
use camino::Utf8PathBuf;
use cmd::{cargo, lipo, rustup, xcodebuild, zip};
pub use conf::{XCFrameworkConfiguration, XcCli};
use ext::PathBufExt;

#[derive(Debug, PartialEq, Eq)]
pub struct Produced {
    pub module_name: String,
    pub path: Utf8PathBuf,
    pub is_zipped: bool,
}

pub fn build(cli: XcCli) -> Result<Produced> {
    let conf = Configuration::load(cli)?;

    conf.build_dir.remove_dir_all_if_exists()?;

    rustup::check_needed(&conf)?;
    cargo::build(&conf)?;

    let libs = lipo::assemble_libs(&conf)?;
    xcodebuild::assemble(&conf, libs)?;
    let module_name = conf.module_name()?;

    let (path, is_zipped) = if conf.cargo_section.zip {
        (zip::xcframework(&conf)?, true)
    } else {
        let from = conf.build_dir.join(format!("{module_name}.xcframework"));
        let to = conf.target_dir.join(format!("{module_name}.xcframework"));
        fs_err::rename(from, &to)?;
        (to, false)
    };

    conf.build_dir.remove_dir_all_if_exists()?;

    Ok(Produced {
        module_name,
        path,
        is_zipped,
    })
}
