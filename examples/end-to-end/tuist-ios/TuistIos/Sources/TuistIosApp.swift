import SwiftUI
import MyMath

@main
struct TuistIosApp: App {
    var body: some Scene {
        WindowGroup {
            ContentView().onAppear() {
                let res = MyMath.rust_add(1, 3)
                print(res)
            }
        }
    }
}
