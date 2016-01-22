# Let's Build an iOS App in Rust, part 4

This is the repository accompanying the [Let's Build an iOS App in Rust, part
4](https://www.bignerdranch.com/blog/building-an-ios-app-in-rust-part-4/) blog post.

## Building and Running

The Rust static library is checked into the repository. Open up
`ios/CopyingViewModel/CopyingViewModel.xcodeproj` and run the app.

To rebuild the Rust library after cloning the repo, go into the
`rust/copying_view_model` directory and run `make`. This requires you to have a
working Rust-to-iOS toolchain; see [this blog
post](https://www.bignerdranch.com/blog/building-an-ios-app-in-rust-part-1/)
for instructions on setting up `multirust` appropriately.  (Don't forget to
`multirust override ios` this project directory.)

## Author

John Gallagher, jgallagher@bignerdranch.com

## License

This sample app is available under the MIT license. See the LICENSE file for
more info.
