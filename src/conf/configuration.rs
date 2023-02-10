use crate::Cli;
use anyhow::{bail, Result};
use camino::Utf8PathBuf;
use cargo_metadata::MetadataCommand;

use super::{LibType, XCFrameworkConfiguration};

#[derive(Debug)]
pub struct Configuration {
    pub dir: Utf8PathBuf,
    pub cargo_section: XCFrameworkConfiguration,
    pub cli: Cli,
    pub lib_type: LibType,
    pub lib_name: String,
    pub build_dir: Utf8PathBuf,
}

impl Configuration {
    pub fn load(cli: Cli) -> Result<Self> {
        let metadata = MetadataCommand::new().manifest_path("Cargo.toml").exec()?;
        let Some(package) = metadata.root_package() else {
            anyhow::bail!("Could not find root package in metadata");
        };

        let staticlib = package.targets.iter().find(|t| {
            t.kind.contains(&"staticlib".to_string()) || t.kind.contains(&"staticlib".to_string())
        });
        let dylib = package.targets.iter().find(|t| {
            t.kind.contains(&"cdylib".to_string()) || t.kind.contains(&"staticlib".to_string())
        });

        let xc_conf = XCFrameworkConfiguration::parse(&package.metadata)?;

        use LibType::*;
        let (lib_type, target) = match (staticlib, dylib, &xc_conf.lib_type) {
            (Some(staticlib), None, None) => (StaticLib, staticlib),
            (Some(staticlib), None, Some(StaticLib)) => (StaticLib, staticlib),
            (Some(_staticlib), None, Some(CDyLib)) => {
                bail!("please set '[lib] crate-type = \"cdylib\"' in Cargo.toml")
            }
            (None, Some(dylib), None) => (CDyLib, dylib),
            (_, Some(dylib), Some(CDyLib)) => (CDyLib, dylib),
            (_, Some(_dylib), Some(StaticLib)) => {
                bail!("please set '[lib] crate-type = \"staticlib\"' in Cargo.toml")
            }
            (Some(_), Some(_), None) => {
                bail!("please set '[package.metadata.xcframework] lib-type' in Cargo.toml")
            }
            (None, None, _) => bail!("missing '[lib] crate-type' in Cargo.toml"),
        };
        let mut dir = package.manifest_path.clone();
        dir.pop();

        Ok(Self {
            dir,
            cargo_section: xc_conf,
            cli,
            lib_type,
            lib_name: target.name.clone(),
            build_dir: Utf8PathBuf::from("target").join("xcframework"),
        })
    }

    pub fn profile(&self) -> &str {
        self.cli.profile.as_deref().unwrap_or("debug")
    }
}
