import SwiftUI
import MyMath
import Shared

@main
struct TuistMacOSApp: App {
    var body: some Scene {
        WindowGroup {
            ContentView().onAppear() {
                let res = MyMath.rust_add(1, 3)
                print(res)
            }
        }
    }
}
