//
// Copyright (c) 2016 David Cuddeback
//
use crate::bits::*;
use libexif_sys::*;
use std::ffi::CStr;

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
    /// The name of the EXIF tag when found in the given IFD.
    pub fn name(&self, ifd: IFD) -> &'static str {
        let ptr = unsafe { exif_tag_get_name_in_ifd(self.inner, ifd.into()) };
        assert!(!ptr.is_null());
        let cstr = unsafe { CStr::from_ptr(ptr) };
        cstr.to_str().expect("invalid UTF-8")
    }

    /// The title of the EXIF tag when found in the given IFD.
    pub fn title(&self, ifd: IFD) -> &'static str {
        let ptr = unsafe { exif_tag_get_title_in_ifd(self.inner, ifd.into()) };
        assert!(!ptr.is_null());
        let cstr = unsafe { CStr::from_ptr(ptr) };
        cstr.to_str().expect("invalid UTF-8")
    }

    /// A verbose description of the EXIF tag when found in the given IFD.
    pub fn description(&self, ifd: IFD) -> &'static str {
        let ptr = unsafe { exif_tag_get_description_in_ifd(self.inner, ifd.into()) };
        assert!(!ptr.is_null());
        let cstr = unsafe { CStr::from_ptr(ptr) };
        cstr.to_str().expect("invalid UTF-8")
    }

    /// The EXIF tag's support level with the given IFD and encoding.
    ///
    /// This method returns the tag's support level according to the EXIF specification.
    pub fn support_level(&self, ifd: IFD, encoding: DataEncoding) -> SupportLevel {
        let support_level =
            unsafe { exif_tag_get_support_level_in_ifd(self.inner, ifd.into(), encoding.into()) };
        SupportLevel::from(support_level)
    }
}
