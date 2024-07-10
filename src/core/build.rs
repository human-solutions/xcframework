use std::collections::HashMap;

use anyhow::Result;
use camino::Utf8PathBuf;

use super::platform::ApplePlatform;
use crate::config::LibType;

pub struct Target {
    pub triple: String,
    pub platform: ApplePlatform,
}

fn cargo_command() -> std::process::Command {
    let program = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
    std::process::Command::new(program)
}

/// Builds libraries for the specified targets and profile.
pub fn build_targets(
    metadata: cargo_metadata::MetadataCommand,
    pkg: &str,
    libname: &str,
    profile: &str,
    lib_type: &LibType,
    targets: Vec<Target>,
    sequentially: bool,
) -> Result<HashMap<ApplePlatform, Vec<Utf8PathBuf>>> {
    // TODO: Extract
    // Optimize: Consider using cargo_metadata crate to avoid running cargo build and parsing the output

    let mut cargo_command = cargo_command();
    let target_dir = metadata.exec()?.target_directory;

    if sequentially {
        for target in targets.iter().map(|t| t.triple.as_str()) {
            println!("🔨 Building for {target}, profile: {profile}");
            // sh.cmd("cargo")
            //     .arg("build")
            //     .args(["--manifest-path", manifaest_path])
            //     .args(["-p", pkg])
            //     .args(["--target", target])
            //     .args(["--profile", profile])
            //     .run()?;

            cargo_command
                .arg("build")
                .args(["-p", pkg])
                .args(["--target", target])
                .args(["--profile", profile])
                .output()?;
        }
    } else {
        // let mut cmd = sh
        //     .cmd("cargo")
        //     .arg("build")
        //     .args(["--manifest-path", manifaest_path])
        //     .args(["-p", pkg]);
        let mut cmd = cargo_command.arg("build").args(["-p", pkg]);
        for target in targets.iter().map(|t| t.triple.as_str()) {
            cmd = cmd.arg("--target").arg(target);
        }
        cmd = cmd.arg("--profile").arg(profile);

        println!("🔨  Building for {} targets", targets.len());
        println!("cmd: {:?}", cmd);
        cmd.output()?;
    }

    let libname = format!("lib{libname}.{}", lib_type.ext());
    let mut platform_build_paths = HashMap::new();
    for target in targets {
        let path = lib_path_for_target(&target_dir, target.triple.as_str(), profile, &libname);
        let paths = platform_build_paths
            .entry(target.platform.clone())
            .or_insert_with(Vec::new);
        paths.push(path);
    }

    Ok(platform_build_paths)
}

pub fn lib_path_for_target(
    target_dir: &Utf8PathBuf,
    triple: &str,
    profile: &str,
    libname: &str,
) -> Utf8PathBuf {
    target_dir.join(triple).join(profile).join(libname)
}
