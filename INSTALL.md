# Installation

The `README.md` covers the basic requirements for building and using this crate. In theory, if you have all of the requisite tools, libexif, and Rust installed, then simply adding this crate as a dependency should be enough. If that did not work as expected, then this guide should help.

## In an ideal world

If everything is set up just right, then add the crate and proceed as usual.

```shell
cargo add libexif
cargo build
```

But we don't live in an ideal world, so keep reading.

## Installing on Ubuntu Linux

Install build tools, Clang, `pkg-config`, and libexif.

```shell
sudo apt-get install build-essential clang pkg-config libexif-dev
```

## Installing on RockyLinux

Install the basic development tools and enable the CRB repository so we can install the libexif development files (headers, `.pc` file, etc).

```shell
sudo dnf group install "Development Tools"
sudo dnf config-manager --set-enabled crb
sudo yum install clang pkgconf-pkg-config libexif libexif-devel
```

## Installing on FreeBSD

Install git, Rust, pkg-config, the Clang libraries, and libexif.

```shell
sudo pkg install git rust libexif pkgconf opencl-clang-llvm17
```

## Installing on Windows

Visit the [libexif](https://libexif.github.io/) site and choose whatever option is best suited for you in order to install the binaries for Windows. Note that the vcpkg option takes quite a while since it pulls in everything needed to build libexif from source.

### Using MSYS2

Using [MSYS2](https://www.msys2.org/) the instructions are as follows.

```shell
pacman -S git mingw-w64-x86_64-clang mingw-w64-x86_64-rust mingw-w64-clang-x86_64-libexif
export PATH=$PATH:/mingw64/bin
export PKG_CONFIG_PATH=$PKG_CONFIG_PATH:/clang64/lib/pkgconfig
```

## Creating an Example

Create an example that will be used to test the build process.

```shell
cargo new --bin exifdump
cd exifdump
cargo add libexif
```

Replace the `src/main.rs` with the example code from `examples/dump.rs` in this repository, then build and run the binary using an image that contains EXIF data.

```shell
cargo build
cargo run -- dcp_1069.jpg
```

Hopefully that output a bunch of EXIF data.
