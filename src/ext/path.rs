use std::path::Path;

use anyhow::{Context, Result};
use camino::Utf8PathBuf;
use fs_extra::dir::CopyOptions;

pub trait PathBufExt {
    fn resolve_home_dir(self) -> Result<Utf8PathBuf>;

    fn create_dir_all_if_needed(&self) -> Result<()>;

    fn remove_dir_all_if_exists(&self) -> Result<()>;

    fn copy_dir<P: AsRef<Path>>(&self, to: P) -> Result<()>;

    fn copy_dir_contents<P: AsRef<Path>>(&self, to: P) -> Result<()>;
}

impl PathBufExt for Utf8PathBuf {
    fn create_dir_all_if_needed(&self) -> Result<()> {
        if !self.exists() {
            fs_err::create_dir_all(self)?;
        }
        Ok(())
    }

    fn resolve_home_dir(self) -> Result<Utf8PathBuf> {
        if self.starts_with("~") {
            let home = std::env::var("HOME").context("Could not resolve $HOME")?;
            let home = Utf8PathBuf::from(home);
            Ok(home.join(self.strip_prefix("~").unwrap()))
        } else {
            Ok(self)
        }
    }

    fn remove_dir_all_if_exists(&self) -> Result<()> {
        if self.exists() {
            fs_err::remove_dir_all(self)?;
        }
        Ok(())
    }

    fn copy_dir<P: AsRef<Path>>(&self, to: P) -> Result<()> {
        let to_path = to.as_ref();
        if !to_path.exists() {
            fs_err::create_dir_all(to_path)?;
        }
        fs_extra::dir::copy(self, to_path, &CopyOptions::new())?;

        Ok(())
    }

    fn copy_dir_contents<P: AsRef<Path>>(&self, to: P) -> Result<()> {
        let to_path = to.as_ref();
        if !to_path.exists() {
            fs_err::create_dir_all(to_path)?;
        }
        let options = CopyOptions::new().content_only(true);
        fs_extra::dir::copy(self, to_path, &options)?;

        Ok(())
    }
}
