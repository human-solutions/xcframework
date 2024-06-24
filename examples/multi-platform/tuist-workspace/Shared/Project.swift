import ProjectDescription

let project = Project(
    name: "Shared",
    targets: [
        .target(
            name: "Shared",
            destinations: [.iPhone, .mac],
            product: .framework,
            bundleId: "cargo.xcframework.shared",
            infoPlist: .default,
            sources: ["Sources/**"],
            dependencies: [
                .xcframework(path: "../../mymath-lib/target/MyMath.xcframework")
            ]
        ),
    ]
)
