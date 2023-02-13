// swift-tools-version:5.7
import PackageDescription
let package = Package(
	name: "mymath",
	products: [
		.library(
			name: "mymath",
			targets: ["mymath"]),
	],
	dependencies: [],
	targets: [
		.executableTarget(
				name: "swift-cmd",
				dependencies: ["mymath"]),
		.binaryTarget(
				name: "mymath",
				path: "../mymath-lib/target/xcframework/mymath.xcframework"
		),
	]
)
