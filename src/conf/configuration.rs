use crate::{ext::PathBufExt, Cli};

use anyhow::{bail, Context, Result};
use camino::Utf8PathBuf;
use cargo_metadata::MetadataCommand;

use super::{LibType, XCFrameworkConfiguration};

#[derive(Debug)]
pub struct Configuration {
    pub dir: Utf8PathBuf,
    pub cargo_lib_target_name: String,
    pub release: bool,
    pub profile: String,
    pub manifest_path: Utf8PathBuf,
}

impl Configuration {
    pub fn load(cli: Cli) -> Result<Self> {
        let manifest_path = cli
            .manifest_path
            .to_owned()
            .unwrap_or_else(|| Utf8PathBuf::from("Cargo.toml"))
            .resolve_home_dir()
            .context(format!("manifest_path: {:?}", &cli.manifest_path))?;

        let metadata = MetadataCommand::new()
            .manifest_path(&manifest_path)
            .exec()?;
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
        let target = match (staticlib, dylib, xc_conf.lib_type) {
            (Some(staticlib), None, None) => staticlib,
            (Some(staticlib), None, Some(StaticLib)) => staticlib,
            (Some(_staticlib), None, Some(CDyLib)) => {
                bail!("please set '[lib] crate-type = \"cdylib\"' in Cargo.toml")
            }
            (None, Some(dylib), None) => dylib,
            (_, Some(dylib), Some(CDyLib)) => dylib,
            (_, Some(_dylib), Some(StaticLib)) => {
                bail!("please set '[lib] crate-type = \"staticlib\"' in Cargo.toml")
            }
            (Some(_), Some(_), None) => {
                bail!("please set '[package.metadata.xcframework] lib-type' in Cargo.toml")
            }
            (None, None, _) => bail!("missing '[lib] crate-type' in Cargo.toml"),
        };
        let profile = if cli.release { "release" } else { "debug" };
        let mut dir = package.manifest_path.clone();
        dir.pop();

        Ok(Self {
            dir,
            cargo_lib_target_name: target.name.clone(),
            profile: profile.to_string(),
            release: cli.release,
            manifest_path,
        })
    }
}
