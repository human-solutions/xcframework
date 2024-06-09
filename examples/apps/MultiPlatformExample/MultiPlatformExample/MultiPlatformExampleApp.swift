//
//  MultiPlatformExampleApp.swift
//  MultiPlatformExample
//
//  Created by 王兴彬 on 2024/6/8.
//

import SwiftUI
import MyMathPackage

@main
struct MultiPlatformExampleApp: App {
    var body: some Scene {
        WindowGroup {
            ContentView()
                .onAppear() {
                    testMath()
                }
        }
    }
}
