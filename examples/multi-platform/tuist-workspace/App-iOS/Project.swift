import ProjectDescription

let project = Project(
    name: "App-iOS",
    targets: [
        .target(
            name: "iOSApp",
            destinations: .iOS,
            product: .app,
            bundleId: "cargo.xcframework.iOSApp",
            infoPlist: .default,
            sources: ["Sources/**"],
            dependencies: [
                .project(target: "Shared", path: "../Shared"),
            ]
        ),
        .target(
            name: "iOSAppTests",
            destinations: .iOS,
            product: .unitTests,
            bundleId: "cargo.xcframework.iOSAppTests",
            infoPlist: .default,
            sources: ["Tests/**"],
            dependencies: [.target(name: "iOSApp")]
        ),
    ]
)
