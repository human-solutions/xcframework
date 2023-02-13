use crate::Cli;
use anyhow::{bail, Result};
use camino::Utf8PathBuf;
use cargo_metadata::MetadataCommand;

use super::{LibType, XCFrameworkConfiguration};

#[derive(Debug)]
pub struct Configuration {
    /// The root dir of the project
    pub dir: Utf8PathBuf,
    pub cargo_section: XCFrameworkConfiguration,
    pub cli: Cli,
    pub lib_type: LibType,
    pub lib_name: String,
    /// Directory for all generated artifacts
    pub target_dir: Utf8PathBuf,
    /// Directory where the xcframework will be built
    pub build_dir: Utf8PathBuf,
}

impl Configuration {
    pub fn load(cli: Cli) -> Result<Self> {
        let manifest_path = cli
            .manifest_path
            .clone()
            .unwrap_or_else(|| Utf8PathBuf::from("Cargo.toml"));
        let mut dir = manifest_path.clone();
        dir.pop();

        let target_dir = dir.join(
            cli.target_dir
                .clone()
                .unwrap_or_else(|| Utf8PathBuf::from("target")),
        );
        let build_dir = target_dir.join("xcframework");

        println!("target_dir: {:?}", target_dir);
        let metadata = MetadataCommand::new().manifest_path(manifest_path).exec()?;

        let Some(package) = metadata.root_package() else {
            anyhow::bail!("Could not find root package in metadata");
        };

        let staticlib = package.targets.iter().find(|t| {
            t.kind.contains(&"staticlib".to_string()) || t.kind.contains(&"staticlib".to_string())
        });
        let dylib = package.targets.iter().find(|t| {
            t.kind.contains(&"cdylib".to_string()) || t.kind.contains(&"cdylib".to_string())
        });

        let xc_conf = XCFrameworkConfiguration::parse(&package.metadata, &dir)?;

        let wanted_lib_type = cli.lib_type.clone().or_else(|| xc_conf.lib_type.clone());

        use LibType::*;
        let (lib_type, target) = match (staticlib, dylib, wanted_lib_type) {
            (Some(staticlib), None, None) => (StaticLib, staticlib),
            (Some(staticlib), _, Some(StaticLib)) => (StaticLib, staticlib),
            (Some(_staticlib), None, Some(CDyLib)) => {
                bail!("please add 'cdylib' to '[lib] crate-type' in Cargo.toml")
            }
            (None, Some(dylib), None) => (CDyLib, dylib),
            (_, Some(dylib), Some(CDyLib)) => (CDyLib, dylib),
            (_, Some(_dylib), Some(StaticLib)) => {
                bail!("please add 'staticlib' to '[lib] crate-type' in Cargo.toml")
            }
            (Some(_), Some(_), None) => {
                bail!("please set '[package.metadata.xcframework] crate-type' in Cargo.toml")
            }
            (None, None, _) => bail!("missing '[lib] crate-type' in Cargo.toml"),
        };

        Ok(Self {
            dir,
            cargo_section: xc_conf,
            cli,
            lib_type,
            lib_name: target.name.clone(),
            target_dir,
            build_dir,
        })
    }

    pub fn profile(&self) -> &str {
        self.cli.profile.as_deref().unwrap_or("debug")
    }
}
