use anyhow::{Context, Result};
use camino::Utf8PathBuf;

pub trait PathBufExt {
    /// cleaning the unc (illegible \\?\) start of windows paths. See dunce crate.
    fn clean_windows_path(&mut self);

    fn resolve_home_dir(self) -> Result<Utf8PathBuf>;
}

impl PathBufExt for Utf8PathBuf {
    fn clean_windows_path(&mut self) {
        if cfg!(windows) {
            let cleaned = dunce::simplified(self.as_ref());
            *self = Utf8PathBuf::from_path_buf(cleaned.to_path_buf()).unwrap();
        }
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
}
