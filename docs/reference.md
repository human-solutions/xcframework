### Configuration Reference (Draft)

#### Overview

This document provides the structure and descriptions of the configuration used by `cargo-xcframework` to bundle an XCFramework for multiple platforms. The configuration is specified in a TOML file (`xcframework.toml` or within `Cargo.toml` under `[package.metadata.xcframework]`).

#### Example Configuration

```toml
# Example xcframework.toml

name = "ExampleFramework"

[platforms]
ios = { simulator = false, archs = ["aarch64", "x86_64"] }
macos = {} # use default platform configuration

header_paths = ["/path/to/header1.h", "/path/to/header2.h"]
module_paths = ["/path/to/module.modulemap"]
output_dir = "/path/to/output"
```

#### Configuration Fields

- `name` (optional, string): The name of the XCFramework.

##### Platforms

Defines the target platforms and their specific configurations. Platforms can either have detailed configurations or use default values by simply specifying the platform name.

- `platforms` (required, mixed table):
- `<os>` (table | string): If specified as a table, it includes specific configurations. If specified as a string, it uses the default configuration for the platform.
  - `simulator` (optional, boolean): Whether to enable building for the simulator. Defaults to `true`.
    - Note: This field is only applicable to the targets with `-sim` supported, e.g: `ios` .
  - `archs` (required, array of strings): Architectures to enable for the target platform.

###### Supported Platforms

- `ios`: iOS platform.
  - Default targets:
    - `aarch64-apple-ios`, `x86_64-apple-ios` and `aarch64-apple-ios-sim`
- `macos`: macOS platform.
  - Default targets: `x86_64-apple-darwin`, `aarch64-apple-darwin`.
- More platforms will be supported in the future. (Contributions are welcome!)

Under the hood, all the platforms are mapped to the corresponding `cargo` targets. For example, `ios` is mapped to `aarch64-apple-ios` and `x86_64-apple-ios`, `aarch64-apple-ios-sim` targets, respond to configurations for the iOS platform.

> Reference: [Rust Platform Support](https://doc.rust-lang.org/rustc/platform-support.html)

##### Example

```toml
[platforms]
ios = { simulator = false, archs = ["aarch64", "x86_64"] }
macos = {} # use default platform configuration
```

##### Paths

Defines paths to additional resources required for building the XCFramework.

- `header_paths` (optional, array of strings): Paths to header files needed for the build.
- `module_paths` (optional, array of strings): Paths to module files needed for the build.
- `output_dir` (optional, string): Directory where the output will be stored.
  - Note: If not specified, the output will be stored in the `target/xcframework` directory.

##### Example

```toml
header_paths = ["/path/to/header1", "/path/to/header2"]
module_paths = ["/path/to/module1", "/path/to/module2"]
output_dir = "/path/to/output"
```
