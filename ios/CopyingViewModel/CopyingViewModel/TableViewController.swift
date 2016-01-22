//
//  TableViewController.swift
//  CopyingViewModel
//
//  Created by John Gallagher on 1/18/16.
//  Copyright © 2016 Big Nerd Ranch. All rights reserved.
//

import UIKit

class TableViewController: UITableViewController {

    var handle: ViewModelHandle? {
        didSet {
            // if we had an old handle, clear ourselves out as its observer in case it has
            // already queued up messages to us that we no longer care about
            oldValue?.observer = nil

            if isViewLoaded() {
                handle?.observer = self
                tableView.reloadData()
            }
        }
    }

    private var viewModel: ViewModel? {
        return handle?.viewModel
    }

    override func viewDidLoad() {
        super.viewDidLoad()
        handle?.observer = self
    }

    override func tableView(tableView: UITableView, numberOfRowsInSection section: Int) -> Int {
        return viewModel?.count ?? 0
    }

    override func tableView(tableView: UITableView, cellForRowAtIndexPath indexPath: NSIndexPath) -> UITableViewCell {
        let cell = tableView.dequeueReusableCellWithIdentifier("UITableViewCell", forIndexPath: indexPath)

        cell.textLabel!.text = viewModel![indexPath.row]

        return cell
    }

}

extension TableViewController: ViewModelHandleObserver {
    func viewModelHandle(handle: ViewModelHandle, insertedItemAtIndex index: Int) {
        tableView.insertRowsAtIndexPaths([NSIndexPath(forRow: index, inSection: 0)], withRowAnimation: .Automatic)
    }

    func viewModelHandle(handle: ViewModelHandle, removedItemAtIndex index: Int) {
        tableView.deleteRowsAtIndexPaths([NSIndexPath(forRow: index, inSection: 0)], withRowAnimation: .Automatic)
    }

    func viewModelHandle(handle: ViewModelHandle, modifiedItemAtIndex index: Int) {
        tableView.reloadRowsAtIndexPaths([NSIndexPath(forRow: index, inSection: 0)], withRowAnimation: .Automatic)
    }
}
