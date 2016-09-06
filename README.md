# EXIF

The `exif` crate provides a safe wrapper around the native `libexif` library.

## Dependencies

In order to use the `exif` crate, you must have the `libexif` library installed where it can be
found by `pkg-config`.

## Usage

Add `exif` as a dependency in `Cargo.toml`:

```toml
[dependencies]
exif = "0.0.1"
```

Import the `exif` crate.

```rust
extern crate exif;

use std::io;
use std::path::Path;

fn dump_exif<P: AsRef<Path>>(file_name: P) -> io::Result<()> {
    let data = try!(exif::Data::open("image.jpg"));

    for content in data.contents() {
        println!("[{:=>32}{:=>46}]", format!(" {:?} ", content.ifd()), "");

        for entry in content.entries() {
            println!("  {:<30} = {}",
                    entry.tag().title(content.ifd()),
                    entry.text_value());
        }
    }

    Ok(())
}
```

## License

Copyright © 2016 David Cuddeback

Distributed under the [MIT License](LICENSE).

*Note:* By using this crate, your executable will link to the `libexif` C library, which is licensed
under the [LGPL version 2.1](http://www.gnu.org/licenses/old-licenses/lgpl-2.1.html#TOC1).
