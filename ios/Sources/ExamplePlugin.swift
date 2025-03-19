import SwiftRs
import Tauri
import UIKit
import WebKit
import Security

class StoreArgs: Decodable {
  let key: String?
  let value: String?
}

class LoadArgs: Decodable {
  let key: String?
}

/// Errors that can be thrown when the Keychain is queried.
enum KeychainError: LocalizedError {
    /// The requested item was not found in the Keychain.
    case itemNotFound
    /// Attempted to save an item that already exists.
    /// Update the item instead.
    case duplicateItem
    /// The operation resulted in an unexpected status.
    case unexpectedStatus(OSStatus)
}

let service = "tauri.plugin.os.keystore"

class ExamplePlugin: Plugin {
  @objc public func store(_ invoke: Invoke) throws {
    let args = try invoke.parseArgs(StoreArgs.self)

    do {
        _ = try getToken(identifier: args.key!, service: service)
        try updateToken(args.value!.data(using: .utf8)!, identifier: args.key!, service: service)
    } catch KeychainError.itemNotFound {
        try insertToken(args.value!.data(using: .utf8)!, identifier: args.key!, service: service)
    }

    invoke.resolve()
  }

  @objc public func load(_ invoke: Invoke) throws {
    let args = try invoke.parseArgs(LoadArgs.self)

    invoke.resolve(["value": try getToken(identifier: args.key!, service: service)])
  }

  func updateToken(_ token: Data, identifier: String, service: String) throws {
    let query = [
        kSecClass: kSecClassGenericPassword,
        kSecAttrService: service,
        kSecAttrAccount: identifier
    ] as CFDictionary

    let attributes = [
        kSecValueData: token
    ] as CFDictionary

    let status = SecItemUpdate(query, attributes)
    guard status == errSecSuccess else {
        if status == errSecItemNotFound {
            throw KeychainError.itemNotFound
        }
        throw KeychainError.unexpectedStatus(status)
    }
  }

  func getToken(identifier: String, service: String) throws -> String {
    let query = [
        kSecClass: kSecClassGenericPassword,
        kSecAttrService: service,
        kSecAttrAccount: identifier,
        kSecMatchLimit: kSecMatchLimitOne,
        kSecReturnData: true
    ] as CFDictionary

    var result: AnyObject?
    let status = SecItemCopyMatching(query, &result)

    guard status == errSecSuccess else {
        if status == errSecItemNotFound {
            // Technically could make the return optional and return nil here
            // depending on how you like this to be taken care of
            throw KeychainError.itemNotFound
        }
        throw KeychainError.unexpectedStatus(status)
    }
    // Lots of bang operators here, due to the nature of Keychain functionality.
    // You could work with more guards/if let or others.
    return String(data: result as! Data, encoding: .utf8)!
  }

  func insertToken(_ token: Data, identifier: String, service: String) throws {
    let attributes = [
        kSecClass: kSecClassGenericPassword,
        kSecAttrService: service,
        kSecAttrAccount: identifier,
        kSecValueData: token
    ] as CFDictionary

    let status = SecItemAdd(attributes, nil)
    guard status == errSecSuccess else {
        if status == errSecDuplicateItem {
            throw KeychainError.duplicateItem
        }
        throw KeychainError.unexpectedStatus(status)
    }
  }
}

@_cdecl("init_plugin_os_keystore")
func initPlugin() -> Plugin {
  return ExamplePlugin()
}
