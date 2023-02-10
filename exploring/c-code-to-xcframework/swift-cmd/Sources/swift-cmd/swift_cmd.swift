import MyMath

@main
public struct swift_cmd {
    public private(set) var text = "Hello, World!"

    public static func main() {
        print("\(swift_cmd().text) -> 4^2 = \(MyMath.PowerOf2(4))")
    }
}
