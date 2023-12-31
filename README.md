## debugging-helpers
Small collection of simple functions to make debugging easier. 
Nothing specular but stuff I don't want to have write repeatedly for each project.
Doesn't contain any production ready code. 
It's mostly workarounds that you can use during debugging.
For example it allows you to compare  for equality two types that do not support [Eq][Eq] but do support [Debug][Debug].
It accomplishes this by comparing their debug output.
This is terribly inefficient and only works if the debug output includes the part of the type you are interested in checking if it is the same.
While this is not suitable for production, this allows you to quickly check for equality when debugging a type that only implements [Debug][Debug] but not [Eq][Eq].

## License

All code in this repository is dual-licensed under either:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
This means you can select the license you prefer!
This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are very good reasons to include both as noted in
this [issue](https://github.com/bevyengine/bevy/issues/2373) on [Bevy](https://bevyengine.org)'s repo.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[Eq]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
[Debug]: https://doc.rust-lang.org/std/fmt/trait.Debug.html