use crate::cmd::modulemap;
use anyhow::{anyhow, bail, Context, Result};
use camino_fs::Utf8PathBuf;
use cargo_metadata::{Metadata, MetadataCommand, Package, TargetKind};

use super::{CliArgs, LibType, XCFrameworkConfiguration};

#[derive(Debug)]
pub struct Configuration {
    pub cargo_section: XCFrameworkConfiguration,
    pub cli: CliArgs,
    pub lib_type: LibType,
    // Name of the library (used for the compiled artifacts)
    pub lib_name: String,
    /// Directory for all generated artifacts
    pub target_dir: Utf8PathBuf,
    /// Directory where the xcframework will be built
    pub build_dir: Utf8PathBuf,
}

impl Configuration {
    pub fn new(
        metadata: &Metadata,
        package: &Package,
        mut cli: CliArgs,
        xc_conf: XCFrameworkConfiguration,
    ) -> Result<Self> {
        // Use the target directory from the CLI, or the one from the Cargo.toml
        let target_dir = cli
            .target_dir
            .as_ref()
            .unwrap_or(&metadata.target_directory)
            .clone();

        let build_dir = target_dir.join("xcframework");
        let wanted_lib_type = cli.lib_type.clone().or_else(|| xc_conf.lib_type.clone());

        let (lib_type, target) = get_libtype(package, wanted_lib_type)?;

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
            target_dir,
            build_dir,
        })
    }

    pub fn load(cli: CliArgs) -> Result<Self> {
        let manifest_path = cli
            .manifest_path
            .clone()
            .unwrap_or_else(|| Utf8PathBuf::from("Cargo.toml"));
        let mut dir = manifest_path.clone();
        dir.pop();

        let metadata = MetadataCommand::new().manifest_path(manifest_path).exec()?;

        let package = if let Some(package) = &cli.package {
            metadata
                .workspace_packages()
                .iter()
                .find(|p| &p.name == package)
                .ok_or(anyhow!("Could not find package '{package}'"))?
        } else {
            metadata
                .root_package()
                .ok_or(anyhow!("Could not find root package in metadata"))?
        };

        let Some(section) = package.metadata.get("xcframework") else {
            bail!("Missing [package.metadata.xcframework] section in Cargo.toml");
        };

        let xc_conf = XCFrameworkConfiguration::parse(section, &dir, true)
            .context("Error in Cargo.toml section [package.metadata.xcframework]")?;

        Self::new(&metadata, package, cli, xc_conf)
    }

    pub fn module_name(&self) -> Result<String> {
        modulemap::get_module_name(self)
    }

    pub fn profile(&self) -> &str {
        if self.cli.release {
            "release"
        } else {
            self.cli.profile.as_deref().unwrap_or("debug")
        }
    }
}

fn get_libtype(
    package: &Package,
    libtype: Option<LibType>,
) -> Result<(LibType, &cargo_metadata::Target)> {
    let staticlib = package.targets.iter().find(|t| {
        t.kind.contains(&TargetKind::StaticLib) || t.kind.contains(&TargetKind::StaticLib)
    });
    let dylib = package
        .targets
        .iter()
        .find(|t| t.kind.contains(&TargetKind::CDyLib) || t.kind.contains(&TargetKind::CDyLib));
    use LibType::*;
    Ok(match (staticlib, dylib, libtype) {
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
    })
}
