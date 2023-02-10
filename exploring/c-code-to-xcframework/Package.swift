// swift-tools-version:5.7
import PackageDescription
let package = Package(
	name: "libmymath",
	products: [
		.library(
			name: "libmymath",
			targets: ["libmymath"]),
	],
	dependencies: [],
	targets: [
		.binaryTarget(
			name: "libmymath",
			path: "libmymath.xcframework"
		),
	]
)
