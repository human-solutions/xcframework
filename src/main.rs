use xcframework::{build, CliArgs};

fn main() {
    let args = CliArgs::from_env_or_exit();

    if let Err(e) = crate::build(args) {
        eprintln!("{:?}", e);
        std::process::exit(1);
    }
}
