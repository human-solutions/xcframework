use anyhow::Context;
use camino::Utf8PathBuf;
use clap::Parser;

use crate::{
    config::{self, LibType},
    core::{self, modulemap},
};

/// Compile a package into a cross-platform Apple XCFramework
#[derive(Debug, Parser)]
#[clap(version)]
pub struct XcframeworkOp {
    #[command(flatten)]
    pub manifest: clap_cargo::Manifest,

    #[command(flatten)]
    pub workspace: clap_cargo::Workspace,

    /// Chose library type to build when Cargo.toml `crate-type` has both.
    #[arg(long)]
    pub lib_type: Option<LibType>,

    /// Unstable (nightly-only) flags to Cargo, see 'cargo -Z help' for details
    #[arg(short = 'Z', value_name = "FLAG")]
    pub unstable_flags: Option<String>,

    /// Directory for all generated artifacts
    #[arg(long, value_name = "DIRECTORY")]
    pub target_dir: Option<Utf8PathBuf>,

    /// Build artifacts with the specified profile
    #[clap(long, name = "PROFILE_NAME")]
    pub profile: Option<String>,
}

impl XcframeworkOp {
    pub fn run(&self) -> anyhow::Result<()> {
        let metadata = &self.manifest.metadata().exec()?;
        let root_package = &metadata.root_package().context("Missing root package")?;
        let config = config::load_package_config(root_package)?;

        config.check_rustup()?;

        let targets = config.targets();
        let lib_type = self
            .lib_type
            .clone()
            .or_else(|| config.lib_type.clone())
            .unwrap_or_else(|| {
                if root_package.targets.iter().any(|t| t.is_cdylib()) {
                    LibType::Cdylib
                } else {
                    LibType::Staticlib
                }
            });
        let sequentially = false;
        let pkg = &root_package.name;
        let libname = root_package
            .targets
            .iter()
            .find(|t| match lib_type {
                LibType::Cdylib => t.is_cdylib(),
                LibType::Staticlib => t.is_staticlib(),
            })
            .context("lib not found")?
            .name
            .as_str();
        let profile = &self.profile.clone().unwrap_or_else(|| "debug".to_string());

        let platform_lib_paths = core::build::build_targets(
            self.manifest.metadata(),
            pkg,
            libname,
            profile,
            &lib_type,
            targets,
            sequentially,
        )?;

        let root = &metadata.workspace_root;
        let include_dir = resolve_path(root, &config.include_dir);
        let bundle_name = {
            let this = &config;
            if let Some(name) = this.module_name.clone() {
                name
            } else {
                let mm_path = include_dir.join("module.modulemap");
                modulemap::get_module_name(&mm_path)?
            }
        };

        let libs_dir = metadata.target_directory.join("libs");
        std::fs::create_dir_all(&libs_dir)?;

        let libs = crate::core::lipo_create_platform_libraries(
            &platform_lib_paths,
            &bundle_name,
            &libs_dir,
        )?;

        let (header_paths, module_paths) = separate_include_dir(&include_dir);

        println!("{:?}", header_paths);
        println!("{:?}", module_paths);

        let frameworks_dir = metadata.target_directory.join("frameworks");
        std::fs::create_dir_all(&frameworks_dir)?;

        let framework_paths = libs
            .into_iter()
            .map(|(platform, lib_path)| {
                std::fs::create_dir_all(&frameworks_dir)?;

                crate::core::wrap_as_framework(
                    platform,
                    &lib_type,
                    &lib_path,
                    &header_paths,
                    &module_paths,
                    &bundle_name,
                    &frameworks_dir,
                )
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

        let output_dir = metadata.target_directory.join("xcframeworks");
        let xcframework_path =
            crate::core::create_xcframework(framework_paths, &bundle_name, &output_dir)?;

        if let Some(target_dir) = &self.target_dir {
            let to = target_dir.join(format!("{bundle_name}.xcframework"));
            if to.exists() {
                std::fs::remove_dir_all(&to)?;
            }
            std::fs::rename(&xcframework_path, &to)?;
        }

        println!("✅ Created XCFramework at {:?}", xcframework_path);

        Ok(())
    }
}

fn resolve_path(root: &Utf8PathBuf, path: &Utf8PathBuf) -> Utf8PathBuf {
    if path.is_absolute() {
        path.clone()
    } else {
        root.join(path)
    }
}

pub fn separate_include_dir(include_dir: &Utf8PathBuf) -> (Vec<Utf8PathBuf>, Vec<Utf8PathBuf>) {
    let header_paths = get_paths(include_dir, "h");
    let module_paths = get_paths(include_dir, "modulemap");
    (header_paths, module_paths)
}

fn get_paths(include_dir: &Utf8PathBuf, extension: &str) -> Vec<Utf8PathBuf> {
    let mut paths = Vec::new();
    // Use **/*.h for headers and **/*.modulemap for module maps
    let pattern = format!("{}/**/*.{}", include_dir, extension);

    for entry in glob::glob(&pattern).expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                if let Ok(utf8_path) = Utf8PathBuf::from_path_buf(path) {
                    paths.push(utf8_path);
                }
            }
            Err(e) => eprintln!("Error processing `{}`: {:?}", pattern, e),
        }
    }
    paths
}

impl clap::ValueEnum for LibType {
    fn value_variants<'a>() -> &'a [Self] {
        &[LibType::Staticlib, LibType::Cdylib]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            LibType::Staticlib => Some(clap::builder::PossibleValue::new("staticlib")),
            LibType::Cdylib => Some(clap::builder::PossibleValue::new("cdylib")),
        }
    }
}
