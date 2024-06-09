//
//  File.swift
//  
//
//  Created by Binlogo on 2024/6/8.
//

import Foundation
import MyMath

public func testMath() {
    assert(MyMath.rust_add(1, 1) == 2)
    print("MyMath.rust_add(4 + 2) = \(MyMath.rust_add(4, 2))")
}
