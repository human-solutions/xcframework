# Tuist iOS Example

This is an example project demonstrating how to use the `cargo-xcframework` plugin with a Tuist generated iOS project.

## Getting Started

1. **Install Tuist**: If you haven't already, first [install Tuist](https://docs.tuist.io/guide/introduction/installation.html#installation).
2. **Build and Run**: Use the `tuist run TuistIos` command, just like `cargo run`. Cool, right?

## Static library

First, build the XCFramework:

```bash
# In the root of the repo:
cargo run -- --manifest-path examples/with-tuist/mymath-lib/Cargo.toml --lib-type staticlib
```

Then, run the Swift executable:

```bash
# In the tuist-ios directory:
tuist run
```

## Dynamic library

First, build the XCFramework:

```bash
# In the root of the repo:
cargo run -- --manifest-path examples/with-tuist/mymath-lib/Cargo.toml --lib-type cdylib
```

Then, run the Swift executable:

```bash
# In the tuist-ios directory:
tuist run
```

## Contributing

Welcome contributions to this example! If you have a feature you'd like to add, or you've found a bug, feel free to [open a pull request](https://github.com/human-solutions/xcframework/pulls).
