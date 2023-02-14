use crate::{conf::Configuration, ext::PathBufExt};
use anyhow::Result;
use zip_extensions;

pub fn xcframework(conf: &Configuration) -> Result<()> {
    let module_name = conf.module_name()?;
    let source = &conf.build_dir;
    let dest = conf
        .target_dir
        .join(format!("{module_name}.xcframework.zip"));

    conf.build_dir.join("libs").remove_dir_all_if_exists()?;

    zip_extensions::zip_create_from_directory(
        &dest.clone().into_std_path_buf(),
        &source.clone().into_std_path_buf(),
    )?;

    Ok(())
}
