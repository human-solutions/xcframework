use crate::conf::Configuration;
use anyhow::Result;
use zip_extensions;

pub fn xcframework(conf: &Configuration) -> Result<()> {
    let source = conf
        .build_dir
        .join(format!("{}.xcframework", conf.module_name()?));
    let dest = source.with_extension("xcframework.zip");

    zip_extensions::zip_create_from_directory(
        &dest.into_std_path_buf(),
        &source.clone().into_std_path_buf(),
    )?;

    fs_err::remove_dir_all(source)?;
    Ok(())
}
