import ProjectDescription

let project = Project(
    name: "TuistIos",
    targets: [
        .target(
            name: "TuistIos",
            destinations: .iOS,
            product: .app,
            bundleId: "io.tuist.TuistIos",
            infoPlist: .extendingDefault(
                with: [
                    "UILaunchStoryboardName": "LaunchScreen.storyboard",
                ]
            ),
            sources: ["TuistIos/Sources/**"],
            resources: ["TuistIos/Resources/**"],
            dependencies: [
                .xcframework(path: "../mymath-lib/target/MyMath.xcframework")
            ]
        ),
        .target(
            name: "TuistIosTests",
            destinations: .iOS,
            product: .unitTests,
            bundleId: "io.tuist.TuistIosTests",
            infoPlist: .default,
            sources: ["TuistIos/Tests/**"],
            resources: [],
            dependencies: [.target(name: "TuistIos")]
        ),
    ]
)
