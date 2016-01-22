//
//  ViewController.swift
//  CopyingViewModel
//
//  Created by John Gallagher on 1/18/16.
//  Copyright © 2016 Big Nerd Ranch. All rights reserved.
//

import UIKit

class ViewController: UIViewController {

    @IBOutlet var numberOfThreadsTextField: UITextField!
    var containedTableViewController: TableViewController!

    override func prepareForSegue(segue: UIStoryboardSegue, sender: AnyObject?) {
        assert(segue.identifier == "embedTableViewController", "unexpected segue")
        containedTableViewController = segue.destinationViewController as! TableViewController
    }

    @IBAction func startButtonPressed(sender: UIButton) {
        if let numThreads = Int(numberOfThreadsTextField.text ?? "") {
            containedTableViewController.handle = ViewModelHandle(numberOfWorkerThreads: numThreads)
            sender.setTitle("Restart", forState: .Normal)
        } else {
            let alert = UIAlertController(title: "\"\(numberOfThreadsTextField.text ?? "")\" is not a valid number of threads", message: nil, preferredStyle: .Alert)
            alert.addAction(UIAlertAction(title: "OK", style: .Default, handler: nil))
            presentViewController(alert, animated: true, completion: nil)
        }
    }

}
