import Foundation
import XCTest
import MyMath

final class TuistTests: XCTestCase {
    func test_twoPlusTwo_isFour() {
        XCTAssertEqual(MyMath.rust_add(2, 2), 4)
    }
}
