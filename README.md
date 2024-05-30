# libexif

The `libexif` crate provides a safe wrapper around the native `libexif` C library by use of the `libexif-sys` crate. All of the dependencies listed below are brought about by the underlying crate.

## Dependencies

* [Rust](https://www.rust-lang.org) stable (version 1.70 or higher)
* [libexif](https://libexif.github.io) (version 0.6.24)
* [Clang](https://clang.llvm.org) (version 5.0 or higher, as dictated by [rust-bindgen](https://github.com/rust-lang/rust-bindgen))
* For now, `pkg-config` is required to facilitate linking with libexif.

## License

While this crate is distributed under the [MIT License](LICENSE), the underlying [libexif](https://libexif.github.io) C library is licensed under the [LGPL version 2.1](http://www.gnu.org/licenses/old-licenses/lgpl-2.1.html#TOC1).
