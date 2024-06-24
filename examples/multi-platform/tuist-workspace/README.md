# Multi-platform App Example with Tuist

This is an example project demonstrating how to use the `cargo-xcframework` plugin with a Tuist generated iOS/macOS project.

## Getting Started

1. Build the XCFramework of native Rust libraries.

Use `--lib-type` to specify the type of library you want to build. The options are `staticlib` and `cdylib`.

```bash
# In the root of the repo:
cargo run -- --manifest-path examples/multi-platform/mymath-lib/Cargo.toml --lib-type staticlib # or
cargo run -- --manifest-path examples/multi-platform/mymath-lib/Cargo.toml --lib-type cdylib
```

2. Then, run the iOS or macOS app with Tuist.

**Install Tuist**: If you haven't already, first [install Tuist](https://docs.tuist.io/guide/introduction/installation.html#installation).
**Build and Run**: Use the `tuist run` command, just like `cargo run`. Cool, right?

```bash
# In the tuist-workspace directory:
tuist run iOSApp # or
tuist run macOSApp
```

**Run Tests**: Use the `tuist test` command to run the tests.

## Project Structure

```sh
├── App-iOS
├── App-macOS
├── Shared
├── Tuist
└── Workspace.swift
```

> To know more about the project structure, check the [Tuist documentation](https://docs.tuist.io/guide/dependencies/dependencies.html#dependencies).

## Contributing

Welcome contributions to this example! If you have a feature you'd like to add, or you've found a bug, feel free to [open a pull request](https://github.com/human-solutions/xcframework/pulls).
