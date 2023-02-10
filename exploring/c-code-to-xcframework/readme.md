# C code to XCFramework

Build with `build.sh` drag and drop into Xcode.

In your swift file use:

```swift
import MyMath

struct ContentView: View {
    init() {
        print("30^2 = \(PowerOf2(30))")
    }
}
```
