use std::{collections::HashMap, path::Path};

use anyhow::{Context, Result};
use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};

mod internal;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SupportedTargetPlatform {
    #[serde(alias = "iOS")]
    IOS,
    #[serde(alias = "macOS")]
    MacOS,
    #[serde(alias = "tvOS")]
    TvOS,
    #[serde(alias = "watchOS")]
    WatchOS,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Architecture {
    Aarch64,
    X86_64,
    Arm64e,
    ArmV7k,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub struct TargetPlatformConfig {
    #[serde(default = "TargetPlatformConfig::default_enable")]
    enable: bool,
    #[serde(default = "TargetPlatformConfig::default_simulator")]
    simulator: bool,
    #[serde(default = "TargetPlatformConfig::default_archs")]
    archs: Vec<Architecture>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum TargetPlatformConfigVariant {
    Preset(bool),
    Custom(TargetPlatformConfig),
}

impl TargetPlatformConfig {
    fn default_enable() -> bool {
        true
    }

    fn default_simulator() -> bool {
        true
    }

    fn default_archs() -> Vec<Architecture> {
        vec![Architecture::Aarch64, Architecture::X86_64]
    }
}

/// The frameworks can be static or dynamic.
/// From rust perspective, it's crate type: cdylib or staticlib.
#[derive(Debug, Clone, Serialize, PartialEq, Eq, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum LibType {
    #[default]
    Staticlib,
    Cdylib,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    /// Name of the module, as defined in the modulemap. Used for naming the XCframework
    pub module_name: Option<String>,
    /// The include directory containing the headers and the module.modulemap file
    #[serde(default, alias = "include-dir")]
    pub include_dir: Utf8PathBuf,
    #[serde(alias = "lib-type")]
    pub lib_type: Option<LibType>,
    #[serde(flatten)]
    pub platforms: HashMap<SupportedTargetPlatform, TargetPlatformConfigVariant>,

    pub output_dir: Option<Utf8PathBuf>,
    /// Whether to zip the resulting XCFramework
    #[serde(default)]
    pub zip: bool,
}

impl Config {
    pub fn update(&mut self, source: &Config) {
        if let Some(module_name) = &source.module_name {
            self.module_name = Some(module_name.clone());
        }
        self.include_dir = source.include_dir.clone();
        if let Some(lib_type) = &source.lib_type {
            self.lib_type = Some(lib_type.clone());
        }
        if let Some(output_dir) = &source.output_dir {
            self.output_dir = Some(output_dir.clone());
        }
        for (key, value) in &source.platforms {
            self.platforms.insert(key.clone(), value.clone());
        }
        // self.module = source.module.clone();
        self.zip = source.zip;
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
struct CargoManifest {
    workspace: Option<CargoWorkspace>,
    package: Option<CargoPackage>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
struct CargoWorkspace {
    package: Option<CargoWorkspacePackage>,
    metadata: Option<CargoMetadata>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
struct CargoWorkspacePackage {}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
struct CargoPackage {
    version: Option<MaybeWorkspace<String>>,
    metadata: Option<CargoMetadata>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MaybeWorkspace<T> {
    Workspace(TomlWorkspaceField),
    Defined(T),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TomlWorkspaceField {
    workspace: bool,
}

impl CargoPackage {
    fn into_config(self) -> Option<Config> {
        self.metadata?.xcframework
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(default)]
struct CargoMetadata {
    xcframework: Option<Config>,
}

pub fn load_package_config(pkg: &cargo_metadata::Package) -> Result<Config> {
    let manifest_path = pkg.manifest_path.as_std_path();

    let mut xcframework_config = Config::default();

    let cfg = resolve_config(manifest_path)?;
    xcframework_config.update(&cfg);

    Ok(xcframework_config)
}

/// Try to resolve configuration source.
///
/// This tries the following sources in order, merging the results:
/// 1. $(crate)/xcframework.toml
/// 2. $(crate)/Cargo.toml `package.metadata.xcframework`
///
pub fn resolve_config(manifest_path: &Path) -> Result<Config> {
    let mut config = Config::default();

    // Crate config
    let crate_root = manifest_path.parent().unwrap_or_else(|| Path::new("."));
    let default_config = crate_root.join("xcframework.toml");
    let current_dir_config = get_config_from_file(&default_config)?;
    if let Some(cfg) = current_dir_config {
        config.update(&cfg);
    }

    // Cargo.toml config
    let current_dir_config = get_pkg_config_from_manifest(manifest_path)?;
    if let Some(cfg) = current_dir_config {
        config.update(&cfg);
    }

    Ok(config)
}

fn get_config_from_file(file_path: &Path) -> Result<Option<Config>> {
    if file_path.exists() {
        let c = std::fs::read_to_string(file_path)?;
        let config = toml::from_str(&c)
            .with_context(|| format!("Failed to parse `{}`", file_path.display()))?;
        Ok(Some(config))
    } else {
        Ok(None)
    }
}

fn get_pkg_config_from_manifest(manifest_path: &Path) -> Result<Option<Config>> {
    if manifest_path.exists() {
        let m = std::fs::read_to_string(manifest_path)?;
        let c: CargoManifest = toml::from_str(&m)
            .with_context(|| format!("Failed to parse `{}`", manifest_path.display()))?;

        Ok(c.package.and_then(|p| p.into_config()))
    } else {
        Ok(None)
    }
}
