// swift-tools-version:5.3
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "tauri-plugin-lnxdxtf-thermal-printer",
    platforms: [
        .iOS(.v13),
    ],
    products: [
        // Products define the executables and libraries a package produces, and make them visible to other packages.
        .library(
            name: "tauri-plugin-lnxdxtf-thermal-printer",
            type: .static,
            targets: ["tauri-plugin-lnxdxtf-thermal-printer"]),
    ],
    dependencies: [
        .package(name: "Tauri", path: "../.tauri/tauri-api")
    ],
    targets: [
        // Targets are the basic building blocks of a package. A target can define a module or a test suite.
        // Targets can depend on other targets in this package, and on products in packages this package depends on.
        .target(
            name: "tauri-plugin-lnxdxtf-thermal-printer",
            dependencies: [
                .byName(name: "Tauri")
            ],
            path: "Sources")
    ]
)
