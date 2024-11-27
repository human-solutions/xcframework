use xcframework::CliArgs;

fn main() {
    let args = CliArgs::from_env_or_exit();

    if let Err(e) = xcframework::build_from_cli(args) {
        eprintln!("{:?}", e);
        std::process::exit(1);
    }
}
