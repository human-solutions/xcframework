# Swift test of XCFramework

## Static library

First, build the XCFramework:

```bash
# In the root of the repo:
cargo run -- --manifest-path examples/project/mymath-lib/Cargo.toml --lib-type staticlib
```

Then, run the Swift executable:

```bash
# In the swift-exe directory:
swift run
```

## Dynamic library

First, build the XCFramework:

```bash
# In the root of the repo:
cargo run -- --manifest-path examples/project/mymath-lib/Cargo.toml --lib-type cdylib
```

Then, run the Swift executable:

```bash
# In the swift-exe directory:
swift run
```
