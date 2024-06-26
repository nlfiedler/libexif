//
// Copyright (c) 2016 David Cuddeback
//
//! Types and functions not related to libexif itself.

pub trait FromLibExif<T> {
    fn from_libexif(inner: T) -> Self;
}
