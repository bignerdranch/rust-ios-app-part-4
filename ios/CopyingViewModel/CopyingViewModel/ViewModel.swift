//
//  ViewModel.swift
//  CopyingViewModel
//
//  Created by John Gallagher on 1/18/16.
//  Copyright © 2016 Big Nerd Ranch. All rights reserved.
//

import Foundation

class ViewModel {
    private let raw: COpaquePointer

    init(_ raw: COpaquePointer) {
        self.raw = raw
    }

    deinit {
        view_model_destroy(raw)
    }

    var count: Int {
        return view_model_len(raw)
    }

    subscript(index: Int) -> String {
        let byte_slice = view_model_value_at_index(raw, index)
        return String(bytes: byte_slice)!
    }
}
