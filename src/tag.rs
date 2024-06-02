//
// Copyright (c) 2016 David Cuddeback
//
use crate::bits::*;
use libexif_sys::*;
use std::{ffi::CStr, str::Utf8Error};

/// EXIF tag.
pub struct Tag {
    inner: ExifTag,
}

impl From<ExifTag> for Tag {
    fn from(tag: ExifTag) -> Tag {
        Tag { inner: tag }
    }
}

impl Tag {
    /// Return the tag code (e.g. 274 for Orientation).
    pub fn code(&self) -> u32 {
        self.inner
    }

    /// The name of the EXIF tag when found in the given IFD.
    pub fn name(&self, ifd: IFD) -> Result<&str, Utf8Error> {
        let ptr = unsafe { exif_tag_get_name_in_ifd(self.inner, ifd.into()) };
        assert!(!ptr.is_null());
        let cstr = unsafe { CStr::from_ptr(ptr) };
        cstr.to_str()
    }

    /// The title of the EXIF tag when found in the given IFD.
    pub fn title(&self, ifd: IFD) -> Result<&str, Utf8Error> {
        let ptr = unsafe { exif_tag_get_title_in_ifd(self.inner, ifd.into()) };
        assert!(!ptr.is_null());
        let cstr = unsafe { CStr::from_ptr(ptr) };
        cstr.to_str()
    }

    /// A verbose description of the EXIF tag when found in the given IFD.
    pub fn description(&self, ifd: IFD) -> Result<&str, Utf8Error> {
        let ptr = unsafe { exif_tag_get_description_in_ifd(self.inner, ifd.into()) };
        assert!(!ptr.is_null());
        let cstr = unsafe { CStr::from_ptr(ptr) };
        cstr.to_str()
    }

    /// The EXIF tag's support level with the given IFD and encoding.
    ///
    /// This method returns the tag's support level according to the EXIF specification.
    pub fn support_level(
        &self,
        ifd: IFD,
        encoding: DataEncoding,
    ) -> Result<SupportLevel, super::ExifError> {
        let support_level =
            unsafe { exif_tag_get_support_level_in_ifd(self.inner, ifd.into(), encoding.into()) };
        SupportLevel::try_from(support_level)
    }
}
