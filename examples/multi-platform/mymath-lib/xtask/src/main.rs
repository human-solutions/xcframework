use anyhow::Context;
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Xtask {
    #[clap(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    BuildXcframework(XcframeworkArgs),
}

#[derive(Debug, Clone, clap::Args)]
pub struct XcframeworkArgs {
    #[command(flatten)]
    manifest: clap_cargo::Manifest,
}

impl XcframeworkArgs {
    fn run(&self) -> Result<()> {
        let metadata = self.manifest.metadata().exec()?;
        let pkg = metadata.root_package().context("No root package")?;
        let config = xcframework::config::load_package_config(pkg)?;

        println!("{:#?}", config);

        let lib_name = "mymath";
        let pkg = &pkg.name;
        let profile = "release";

        let name = "MyMath";
        let targets = config.targets();
        let lib_type = &config.lib_type;
        let sequentially = false;

        let libs_dir = metadata.target_directory.join("libs");
        std::fs::create_dir_all(&libs_dir)?;

        let platform_lib_paths = xcframework::core::build::build_targets(
            pkg,
            lib_name,
            profile,
            lib_type,
            targets,
            sequentially,
        )?;

        let libs = xcframework::core::lipo_create_platform_libraries(
            &platform_lib_paths,
            name,
            &libs_dir,
        )?;

        let header_paths = config.header_paths;
        let module_paths = config.module_paths;
        let frameworks_dir = metadata.target_directory.join("frameworks");
        std::fs::create_dir_all(&frameworks_dir)?;

        let framework_paths = libs
            .into_iter()
            .map(|(platform, lib_path)| {
                std::fs::create_dir_all(&frameworks_dir)?;

                xcframework::core::wrap_as_framework(
                    platform,
                    lib_type,
                    &lib_path,
                    &header_paths,
                    &module_paths,
                    name,
                    &frameworks_dir,
                )
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

        let output_dir = metadata.target_directory.join("xcframeworks");
        let xcframework_path =
            xcframework::core::create_xcframework(framework_paths, name, &output_dir)?;

        println!("✅ Created XCFramework at {:?}", xcframework_path);

        Ok(())
    }
}

fn main() -> Result<()> {
    match Xtask::parse().cmd {
        Command::BuildXcframework(xcframework) => xcframework.run(),
    }
}
