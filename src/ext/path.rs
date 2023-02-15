use anyhow::{Context, Result};
use camino::Utf8PathBuf;

pub trait PathBufExt {
    fn resolve_home_dir(self) -> Result<Utf8PathBuf>;

    fn create_dir_all_if_needed(&self) -> Result<()>;

    fn remove_dir_all_if_exists(&self) -> Result<()>;
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
}
