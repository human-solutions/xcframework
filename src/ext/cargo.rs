use anyhow::Result;
use camino::Utf8Path;
use cargo_metadata::{Metadata, MetadataCommand};

use super::PathBufExt;

pub trait MetadataExt {
    fn load_cleaned(manifest_path: &Utf8Path) -> Result<Metadata>;
}

impl MetadataExt for Metadata {
    fn load_cleaned(manifest_path: &Utf8Path) -> Result<Metadata> {
        let mut metadata = MetadataCommand::new().manifest_path(manifest_path).exec()?;
        metadata.workspace_root.clean_windows_path();
        metadata.target_directory.clean_windows_path();
        for package in &mut metadata.packages {
            package.manifest_path.clean_windows_path();
            for dependency in &mut package.dependencies {
                dependency.path.as_mut().map(|p| p.clean_windows_path());
            }
        }
        Ok(metadata)
    }
}
