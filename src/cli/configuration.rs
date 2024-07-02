use std::cell::RefCell;

use crate::{cmd::modulemap, XcCli};
use anyhow::{bail, Result};
use camino::Utf8PathBuf;
use cargo_metadata::MetadataCommand;

use super::{LibType, XCFrameworkConfiguration};

#[derive(Debug)]
pub struct Configuration {
    pub cargo_section: XCFrameworkConfiguration,
    pub cli: XcCli,
    pub lib_type: LibType,
    // Name of the library (used for the compiled artifacts)
    pub lib_name: String,
    /// Name of the module, as defined in the modulemap. Used for naming the XCframework
    module_name: RefCell<Option<String>>,
    /// Directory for all generated artifacts
    pub target_dir: Utf8PathBuf,
    /// Directory where the xcframework will be built
    pub build_dir: Utf8PathBuf,
}

impl Configuration {
    pub fn load(mut cli: XcCli) -> Result<Self> {
        let manifest_path = cli.clap_cargo.manifest_path()?;
        let mut dir =
            Utf8PathBuf::from_path_buf(manifest_path.clone()).expect("manifest_path is valid");
        dir.pop();

        let target_dir = cli.target_dir.clone().unwrap_or_else(|| dir.join("target"));

        let build_dir = target_dir.join("xcframework");

        let metadata = MetadataCommand::new().manifest_path(manifest_path).exec()?;

        let Some(package) = metadata.root_package() else {
            anyhow::bail!("Could not find root package in metadata");
        };

        let staticlib = package.targets.iter().find(|t| t.is_staticlib());
        let dylib = package.targets.iter().find(|t| t.is_cdylib());

        let xc_conf = XCFrameworkConfiguration::parse(&package.metadata, &dir)?;

        let wanted_lib_type = cli.lib_type.clone().or_else(|| xc_conf.lib_type.clone());

        use LibType::*;
        let (lib_type, target) = match (staticlib, dylib, wanted_lib_type) {
            (Some(staticlib), None, None) => (StaticLib, staticlib),
            (Some(staticlib), _, Some(StaticLib)) => (StaticLib, staticlib),
            (Some(_staticlib), None, Some(CDyLib)) => {
                bail!("Please add 'cdylib' to '[lib] crate-type' in Cargo.toml")
            }
            (None, Some(dylib), None) => (CDyLib, dylib),
            (_, Some(dylib), Some(CDyLib)) => (CDyLib, dylib),
            (_, Some(_dylib), Some(StaticLib)) => {
                bail!("Please add 'staticlib' to '[lib] crate-type' in Cargo.toml")
            }
            (Some(_), Some(_), None) => {
                bail!("Please set '[package.metadata.xcframework] lib-type' in Cargo.toml")
            }
            (None, None, _) => bail!("Missing '[lib] crate-type' in Cargo.toml"),
        };

        if xc_conf.build_std && lib_type == LibType::StaticLib {
            let already_set = cli
                .unstable_flags
                .as_ref()
                .map(|f| f.contains("build-std=std"))
                .unwrap_or(false);
            if !already_set {
                if let Some(flag) = cli.unstable_flags.to_owned() {
                    cli.unstable_flags = Some(format!("{flag},build-std=std"));
                } else {
                    cli.unstable_flags = Some("build-std=std".to_string());
                }
            }
        }

        Ok(Self {
            cargo_section: xc_conf,
            cli,
            lib_type,
            lib_name: target.name.clone(),
            module_name: RefCell::new(None),
            target_dir,
            build_dir,
        })
    }

    pub fn module_name(&self) -> Result<String> {
        let name = self.module_name.borrow().clone();
        if let Some(name) = name {
            Ok(name)
        } else {
            let name = modulemap::get_module_name(self)?;
            *self.module_name.borrow_mut() = Some(name.clone());
            Ok(name)
        }
    }

    pub fn profile(&self) -> &str {
        self.cli.clap_cargo.cargo_build.profile()
    }
}
