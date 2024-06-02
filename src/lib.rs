//
// Copyright (c) 2016 David Cuddeback
//
//! The `libexif` crate provides a safe wrapper around the `libexif` C library.
//! It provides the ability to read EXIF data from image files. Note that the
//! API provided by this crate is fairly low-level, as such you may find that
//! the [kamadak-exif](https://crates.io/crates/kamadak-exif) crate is easier to
//! use in most cases.
//!
//! The entry point for inspecting a file's EXIF data is
//! [`Data::open()`](struct.Data.html#method.open). EXIF data can be inspected
//! by iterating over the data's [`contents`](struct.Content.html) and
//! [`entries`](struct.Entry.html):
//!
//! ```
//! # use std::io;
//! # use std::path::Path;
//! fn dump_exif<P: AsRef<Path>>(file_name: P) -> io::Result<()> {
//!     let data = libexif::Data::open(file_name)?;
//!     for content in data.contents() {
//!         let ifd = content.ifd().unwrap();
//!         println!("[{:=>32}{:=>46}]", format!(" {:?} ", content.ifd()), "");
//!         for entry in content.entries() {
//!             let tag = entry.tag();
//!             let title = tag.title(ifd).unwrap_or("error");
//!             let value = entry.text_value().unwrap_or("error".into());
//!             println!("  {:<30} = {}", title, value);
//!         }
//!     }
//!     Ok(())
//! }
//! ```

#![allow(non_upper_case_globals)]

extern crate libc;
extern crate libexif_sys;

///
/// Various types of errors that may occur while reading EXIF data.
///
#[derive(Debug, thiserror::Error)]
pub enum ExifError {
    #[error("illegal byte order value")]
    IllegalByteOrder,
    #[error("illegal data type value")]
    IllegalDataType,
    #[error("illegal data option value")]
    IllegalDataOption,
    #[error("illegal support level value")]
    IllegalSupportLevel,
    #[error("unknown IFD value")]
    UnknownIFD,
}

pub use bits::*;
pub use content::*;
pub use data::*;
pub use entry::*;
pub use tag::*;
pub use value::*;

mod internal;

mod bits;
mod content;
mod data;
mod entry;
mod loader;
mod tag;
mod value;

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;

    #[test]
    fn test_empty_content() -> io::Result<()> {
        let data = Data::open("tests/fixtures/f2t.jpg")?;
        // this image contains several empty contents in this order
        let ifds = vec![IFD::Thumbnail, IFD::GPS, IFD::Interoperability];
        let mut ifds_iterator = ifds.iter();
        for content in data.contents() {
            // find the contents that are empty and ensure that none of the
            // member functions panic
            if content.len() == 0 {
                let (size, maybe_size) = content.entries().size_hint();
                assert_eq!(size, 0);
                assert_eq!(maybe_size, Some(0));
                let ifd = content.ifd().unwrap();
                assert_eq!(&ifd, ifds_iterator.next().unwrap());
            }
        }
        assert!(ifds_iterator.next().is_none());
        Ok(())
    }

    #[test]
    fn test_get_orientation() -> io::Result<()> {
        let data = Data::open("tests/fixtures/f2t.jpg")?;
        let byte_order = data.byte_order().unwrap();
        assert_eq!(byte_order, ByteOrder::LittleEndian);
        for content in data.contents() {
            for entry in content.entries() {
                // Orientation is 274
                if entry.tag().code() == 274 {
                    match entry.value(byte_order).unwrap() {
                        Value::U16(v) => println!("v: {:?}", v),
                        _ => panic!("wrong type of value"),
                    }
                }
            }
        }
        Ok(())
    }
}
