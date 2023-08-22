import XCTest
@testable import SwisstronikSwift

extension String {

/// Create `Data` from hexadecimal string representation
///
/// This takes a hexadecimal representation and creates a `Data` object. Note, if the string has any spaces or non-hex characters (e.g. starts with '<' and with a '>'), those are ignored and only hex characters are processed.
///
/// - returns: Data represented by this hexadecimal string.

func hexadecimal() -> Data? {
    var data = Data(capacity: self.count / 2)

    let regex = try! NSRegularExpression(pattern: "[0-9a-f]{1,2}", options: .caseInsensitive)
    regex.enumerateMatches(in: self, options: [], range: NSMakeRange(0, self.count)) { match, flags, stop in
        let byteString = (self as NSString).substring(with: match!.range)
        var num = UInt8(byteString, radix: 16)!
        data.append(&num, count: 1)
    }

    guard data.count > 0 else {
        return nil
    }

    return data
 }
}
extension Data {
    struct HexEncodingOptions: OptionSet {
        let rawValue: Int
        static let upperCase = HexEncodingOptions(rawValue: 1 << 0)
    }

    func hexEncodedString(options: HexEncodingOptions = []) -> String {
        let format = options.contains(.upperCase) ? "%02hhX" : "%02hhx"
        return self.map { String(format: format, $0) }.joined()
    }
}
class SwisstronikSwiftTests: XCTestCase {

    func secureRandomData(count: Int) throws -> Data {
        var bytes = [Int8](repeating: 0, count: count)

        // Fill bytes with secure random data
        let _ = SecRandomCopyBytes(
            kSecRandomDefault,
            count,
            &bytes
        )
        return Data(bytes: bytes, count: count)
    }

    override func setUp() {
        super.setUp()
        // Put setup code here. This method is called before the invocation of each test method in the class.
    }

    override func tearDown() {
        // Put teardown code here. This method is called after the invocation of each test method in the class.
        super.tearDown()
    }
    func testsEncryptDecrypt() throws {
        let userPrivateKey = "C516DC17D909EFBB64A0C4A9EE1720E10D47C1BF3590A257D86EEB5FFC644D43".hexadecimal()!
        let nodePublicKey = "86477673c1c6fd9d061e884f56d440b2ce03fa2fe39a2a4882357a451a7f490e".hexadecimal()!
        let plaintext = try secureRandomData(count: 128)
        print("Plaintext: \(plaintext.hexEncodedString())")
        let encrypted = try SwisstronikEncrypt(privateKey: userPrivateKey, nodePublicKey: nodePublicKey, data: plaintext)
        print("Encrypted: \(encrypted!.hexEncodedString())")
        let decrypted = try SwisstronikDecrypt(privateKey: userPrivateKey, nodePublicKey: nodePublicKey, encryptedData: encrypted!)
        print("Decrypted: \(decrypted!.hexEncodedString())")
        XCTAssertEqual(plaintext, decrypted!)
    }
}
