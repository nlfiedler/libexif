//
// Copyright (c) 2016 David Cuddeback
//
use crate::bits::*;
use crate::internal::*;
use crate::tag::Tag;
use crate::value::Value;
use libc::{self, c_char, c_uint};
use libexif_sys::*;
use std::ffi::CString;
use std::ffi::IntoStringError;
use std::slice;

/// Data found in a single EXIF tag.
pub struct Entry<'a> {
    inner: &'a mut ExifEntry,
}

impl<'a> FromLibExif<&'a mut ExifEntry> for Entry<'a> {
    fn from_libexif(entry: &'a mut ExifEntry) -> Self {
        Entry { inner: entry }
    }
}

impl<'a> Entry<'a> {
    /// EXIF tag for the entry.
    pub fn tag(&self) -> Tag {
        Tag::from(self.inner.tag)
    }

    /// Type of data contained in the entry.
    pub fn data_type(&self) -> Result<DataType, super::Error> {
        DataType::try_from(self.inner.format)
    }

    /// Number of data elements in the entry.
    pub fn components(&self) -> usize {
        self.inner.components as usize
    }

    /// Return the raw binary data for the entry's value.
    pub fn raw_data(&self) -> &[u8] {
        unsafe { slice::from_raw_parts(self.inner.data, self.inner.size as usize) }
    }

    /// Returns an interpreted value of the entry's data.
    pub fn value(&self, byte_order: ByteOrder) -> Result<Value, super::Error> {
        let data_type = self.data_type()?;
        Ok(Value::extract(
            self.raw_data(),
            data_type,
            self.components(),
            byte_order,
        ))
    }

    /// Returns a textual representation of the entry's data.
    pub fn text_value(&self) -> Result<String, IntoStringError> {
        let mut buffer = Vec::<u8>::with_capacity(256);
        let cstring = unsafe {
            let len = libc::strlen(exif_entry_get_value(
                self.inner as *const _ as *mut _,
                buffer.as_mut_ptr() as *mut c_char,
                buffer.capacity() as c_uint,
            ));
            buffer.set_len(len);
            CString::from_vec_unchecked(buffer)
        };
        cstring.into_string()
    }
}
