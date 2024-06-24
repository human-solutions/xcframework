import ProjectDescription

let project = Project(
    name: "App-macOS",
    targets: [
        .target(
            name: "macOSApp",
            destinations: .macOS,
            product: .app,
            bundleId: "cargo.xcframework.macOSApp",
            infoPlist: .default,
            sources: ["Sources/**"],
            dependencies: [
                .project(target: "Shared", path: "../Shared"),
            ]
        ),
        .target(
            name: "macOSAppTests",
            destinations: .macOS,
            product: .unitTests,
            bundleId: "cargo.xcframework.macOSAppTests",
            infoPlist: .default,
            sources: ["Tests/**"],
            dependencies: [.target(name: "macOSApp")]
        ),
    ]
)
