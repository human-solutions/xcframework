use anyhow::{Context, Result};
use camino::Utf8PathBuf;

pub trait PathBufExt {
    fn resolve_home_dir(self) -> Result<Utf8PathBuf>;
}

impl PathBufExt for Utf8PathBuf {
    fn resolve_home_dir(self) -> Result<Utf8PathBuf> {
        if self.starts_with("~") {
            let home = std::env::var("HOME").context("Could not resolve $HOME")?;
            let home = Utf8PathBuf::from(home);
            Ok(home.join(self.strip_prefix("~").unwrap()))
        } else {
            Ok(self)
        }
    }
}
