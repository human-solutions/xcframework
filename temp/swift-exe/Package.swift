// swift-tools-version:5.7
import PackageDescription
let package = Package(
	name: "SwiftExe",
	products: [
		.library(
			name: "MyMath",
			targets: ["MyMath"]),
	],
	dependencies: [],
	targets: [
		.executableTarget(
				name: "swift-cmd",
				dependencies: ["MyMath"]),
		.binaryTarget(
				name: "MyMath",
				path: "../mymath-lib/target/xcframework/MyMath.xcframework.zip"
		),
	]
)
