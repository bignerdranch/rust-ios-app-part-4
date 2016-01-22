//
//  ViewModelHandle.swift
//  CopyingViewModel
//
//  Created by John Gallagher on 1/18/16.
//  Copyright © 2016 Big Nerd Ranch. All rights reserved.
//

import Foundation

protocol ViewModelHandleObserver: class {
    func viewModelHandle(handle: ViewModelHandle, insertedItemAtIndex index: Int)
    func viewModelHandle(handle: ViewModelHandle, removedItemAtIndex index: Int)
    func viewModelHandle(handle: ViewModelHandle, modifiedItemAtIndex index: Int)
}

final class WeakHolder<T: AnyObject> {
    weak var object: T?

    init(_ object: T) {
        self.object = object
    }
}

final class ViewModelHandle {
    private var raw: COpaquePointer!

    private(set) var viewModel: ViewModel!
    weak var observer: ViewModelHandleObserver?

    init(numberOfWorkerThreads: Int) {
        let weakSelf = UnsafeMutablePointer<Void>(Unmanaged.passRetained(WeakHolder(self)).toOpaque())
        let observer = view_model_observer(user: weakSelf, destroy_user: freeViewModelHandle, inserted_item: handleInsertedItem, removed_item: handleRemovedItem, modified_item: handleModifiedItem)

        var rawViewModel: COpaquePointer = nil
        raw = view_model_handle_new(numberOfWorkerThreads, observer, &rawViewModel)
        viewModel = ViewModel(rawViewModel)
    }

    deinit {
        view_model_handle_destroy(raw)
    }
}

extension ViewModelHandle : CustomDebugStringConvertible {
    var debugDescription: String {
        return "ViewModelHandle{\(raw)}"
    }
}

private func freeViewModelHandle(ptr: UnsafeMutablePointer<Void>) {
    let _ = Unmanaged<WeakHolder<ViewModelHandle>>.fromOpaque(COpaquePointer(ptr)).takeRetainedValue()
}

private func handleInsertedItem(ptr: UnsafeMutablePointer<Void>, viewModel: COpaquePointer, index: Int) {
    autoreleasepool {
        let handle = Unmanaged<WeakHolder<ViewModelHandle>>.fromOpaque(COpaquePointer(ptr)).takeUnretainedValue()
        dispatch_async(dispatch_get_main_queue()) {
            guard let handle = handle.object else { return }
            handle.viewModel = ViewModel(viewModel)
            handle.observer?.viewModelHandle(handle, insertedItemAtIndex: index)
        }
    }
}

private func handleRemovedItem(ptr: UnsafeMutablePointer<Void>, viewModel: COpaquePointer, index: Int) {
    autoreleasepool {
        let handle = Unmanaged<WeakHolder<ViewModelHandle>>.fromOpaque(COpaquePointer(ptr)).takeUnretainedValue()
        dispatch_async(dispatch_get_main_queue()) {
            guard let handle = handle.object else { return }
            handle.viewModel = ViewModel(viewModel)
            handle.observer?.viewModelHandle(handle, removedItemAtIndex: index)
        }
    }
}

private func handleModifiedItem(ptr: UnsafeMutablePointer<Void>, viewModel: COpaquePointer, index: Int) {
    autoreleasepool {
        let handle = Unmanaged<WeakHolder<ViewModelHandle>>.fromOpaque(COpaquePointer(ptr)).takeUnretainedValue()
        dispatch_async(dispatch_get_main_queue()) {
            guard let handle = handle.object else { return }
            handle.viewModel = ViewModel(viewModel)
            handle.observer?.viewModelHandle(handle, modifiedItemAtIndex: index)
        }
    }
}