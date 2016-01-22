//
//  RustByteSlice.swift
//  CopyingViewModel
//
//  Created by John Gallagher on 1/18/16.
//  Copyright © 2016 Big Nerd Ranch. All rights reserved.
//

import Foundation

extension rust_byte_slice {
    func asUnsafeBufferPointer() -> UnsafeBufferPointer<UInt8> {
        return UnsafeBufferPointer(start: bytes, count: len)
    }
}

extension String {
    init?(bytes: rust_byte_slice, encoding: NSStringEncoding = NSUTF8StringEncoding) {
        let buffer = bytes.asUnsafeBufferPointer()
        self.init(bytes: buffer, encoding: encoding)
    }
}
