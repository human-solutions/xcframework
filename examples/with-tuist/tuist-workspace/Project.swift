import ProjectDescription

let project = Project(
    name: "Project",
    targets: [
        .target(
            name: "App",
            destinations: [.iPhone, .iPad, .mac, .macCatalyst],
            product: .app,
            bundleId: "cargo.xcframework.App",
            infoPlist: .default,
            sources: ["App/Sources/**"],
            resources: ["App/Resources/**"],
            dependencies: [
                .xcframework(path: "../mymath-lib/target/MyMath.xcframework")
            ]
        ),
        .target(
            name: "AppTests",
            destinations: .iOS,
            product: .unitTests,
            bundleId: "cargo.xcframework.AppTests",
            infoPlist: .default,
            sources: ["App/Tests/**"],
            resources: [],
            dependencies: [.target(name: "App")]
        ),
    ]
)
