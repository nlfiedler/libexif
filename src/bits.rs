//
// Copyright (c) 2016 David Cuddeback
//
use internal::*;
use libexif_sys::*;

/// Defines the byte order of binary values.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum ByteOrder {
    /// Most significant bytes come first.
    ///
    /// An integer value of `0x1234ABCD` will be represented in memory as `12 34 AB CD`.
    BigEndian,

    /// Least significant bytes come first.
    ///
    /// An integer value of `0x1234ABCD` will be represented in memory as `CD AB 34 12`.
    LittleEndian,
}

impl FromLibExif<ExifByteOrder> for ByteOrder {
    fn from_libexif(byte_order: ExifByteOrder) -> Self {
        match byte_order {
            ExifByteOrder_EXIF_BYTE_ORDER_MOTOROLA => ByteOrder::BigEndian,
            ExifByteOrder_EXIF_BYTE_ORDER_INTEL => ByteOrder::LittleEndian,
            _ => panic!("illegal byte order value"),
        }
    }
}

impl ToLibExif<ExifByteOrder> for ByteOrder {
    fn to_libexif(&self) -> ExifByteOrder {
        match *self {
            ByteOrder::BigEndian => ExifByteOrder_EXIF_BYTE_ORDER_MOTOROLA,
            ByteOrder::LittleEndian => ExifByteOrder_EXIF_BYTE_ORDER_INTEL,
        }
    }
}

/// Defines the encoding used to represent EXIF data.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DataEncoding {
    Chunky,
    Planar,
    Ycc,
    Compressed,
    Unknown,
}

impl FromLibExif<ExifDataType> for DataEncoding {
    fn from_libexif(data_type: ExifDataType) -> Self {
        match data_type {
            ExifDataType_EXIF_DATA_TYPE_UNCOMPRESSED_CHUNKY => DataEncoding::Chunky,
            ExifDataType_EXIF_DATA_TYPE_UNCOMPRESSED_PLANAR => DataEncoding::Planar,
            ExifDataType_EXIF_DATA_TYPE_UNCOMPRESSED_YCC => DataEncoding::Ycc,
            ExifDataType_EXIF_DATA_TYPE_COMPRESSED => DataEncoding::Compressed,
            ExifDataType_EXIF_DATA_TYPE_UNKNOWN => DataEncoding::Unknown,
            _ => panic!("illegal data type value"),
        }
    }
}

impl ToLibExif<ExifDataType> for DataEncoding {
    fn to_libexif(&self) -> ExifDataType {
        match *self {
            DataEncoding::Chunky => ExifDataType_EXIF_DATA_TYPE_UNCOMPRESSED_CHUNKY,
            DataEncoding::Planar => ExifDataType_EXIF_DATA_TYPE_UNCOMPRESSED_PLANAR,
            DataEncoding::Ycc => ExifDataType_EXIF_DATA_TYPE_UNCOMPRESSED_YCC,
            DataEncoding::Compressed => ExifDataType_EXIF_DATA_TYPE_COMPRESSED,
            DataEncoding::Unknown => ExifDataType_EXIF_DATA_TYPE_UNKNOWN,
        }
    }
}

/// Options that affect the behavior of [`Data`](struct.Data.html).
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DataOption {
    /// Act as though unknown tags don't exist.
    IgnoreUnknownTags,
    /// Automatically fix discrepencies in EXIF tags to follow the spec.
    FollowSpecification,
    /// Leave the maker note alone.
    DontChangeMakerNote,
}

impl FromLibExif<ExifDataOption> for DataOption {
    fn from_libexif(data_option: ExifDataOption) -> Self {
        match data_option {
            ExifDataOption_EXIF_DATA_OPTION_IGNORE_UNKNOWN_TAGS => DataOption::IgnoreUnknownTags,
            ExifDataOption_EXIF_DATA_OPTION_FOLLOW_SPECIFICATION => DataOption::FollowSpecification,
            ExifDataOption_EXIF_DATA_OPTION_DONT_CHANGE_MAKER_NOTE => {
                DataOption::DontChangeMakerNote
            }
            _ => panic!("illegal data option value"),
        }
    }
}

impl ToLibExif<ExifDataOption> for DataOption {
    fn to_libexif(&self) -> ExifDataOption {
        match *self {
            DataOption::IgnoreUnknownTags => ExifDataOption_EXIF_DATA_OPTION_IGNORE_UNKNOWN_TAGS,
            DataOption::FollowSpecification => ExifDataOption_EXIF_DATA_OPTION_FOLLOW_SPECIFICATION,
            DataOption::DontChangeMakerNote => {
                ExifDataOption_EXIF_DATA_OPTION_DONT_CHANGE_MAKER_NOTE
            }
        }
    }
}

/// EXIF tag data formats.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum DataType {
    /// Tag contains text.
    Text,
    /// Tag contains unsigned bytes.
    U8,
    /// Tag contains signed bytes.
    I8,
    /// Tag contains unsigned 16-bit integers.
    U16,
    /// Tag contains signed 16-bit integers.
    I16,
    /// Tag contains unsigned 32-bit integers.
    U32,
    /// Tag contains signed 32-bit integers.
    I32,
    /// Tag contains unsigned rational numbers.
    URational,
    /// Tag contains signed rational numbers.
    IRational,
    /// Tag contains undefined data type.
    Undefined,
}

impl DataType {
    pub(crate) fn size(&self) -> usize {
        unsafe { exif_format_get_size(self.to_libexif()) as usize }
    }
}

impl FromLibExif<ExifFormat> for DataType {
    fn from_libexif(format: ExifFormat) -> Self {
        match format {
            ExifFormat_EXIF_FORMAT_ASCII => DataType::Text,
            ExifFormat_EXIF_FORMAT_BYTE => DataType::U8,
            ExifFormat_EXIF_FORMAT_SBYTE => DataType::I8,
            ExifFormat_EXIF_FORMAT_SHORT => DataType::U16,
            ExifFormat_EXIF_FORMAT_SSHORT => DataType::I16,
            ExifFormat_EXIF_FORMAT_LONG => DataType::U32,
            ExifFormat_EXIF_FORMAT_SLONG => DataType::I32,
            ExifFormat_EXIF_FORMAT_RATIONAL => DataType::URational,
            ExifFormat_EXIF_FORMAT_SRATIONAL => DataType::IRational,
            _ => DataType::Undefined,
        }
    }
}

impl ToLibExif<ExifFormat> for DataType {
    fn to_libexif(&self) -> ExifFormat {
        match *self {
            DataType::Text => ExifFormat_EXIF_FORMAT_ASCII,
            DataType::U8 => ExifFormat_EXIF_FORMAT_BYTE,
            DataType::I8 => ExifFormat_EXIF_FORMAT_SBYTE,
            DataType::U16 => ExifFormat_EXIF_FORMAT_SHORT,
            DataType::I16 => ExifFormat_EXIF_FORMAT_SSHORT,
            DataType::U32 => ExifFormat_EXIF_FORMAT_LONG,
            DataType::I32 => ExifFormat_EXIF_FORMAT_SLONG,
            DataType::URational => ExifFormat_EXIF_FORMAT_RATIONAL,
            DataType::IRational => ExifFormat_EXIF_FORMAT_SRATIONAL,
            DataType::Undefined => ExifFormat_EXIF_FORMAT_UNDEFINED,
        }
    }
}

/// Image file directory types.
///
/// An image file directory (IFD) is a group of related EXIF tags.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum IFD {
    /// IFD contents describe the primary image.
    Image,
    /// IFD contents describe the thumbnail image.
    Thumbnail,
    /// IFD contents contain EXIF-specific attributes.
    EXIF,
    /// IFD contents contain GPS data.
    GPS,
    /// IFD contents contain tags used for interoperability.
    Interoperability,
}

impl FromLibExif<ExifIfd> for IFD {
    fn from_libexif(ifd: ExifIfd) -> Self {
        match ifd {
            ExifIfd_EXIF_IFD_0 => IFD::Image,
            ExifIfd_EXIF_IFD_1 => IFD::Thumbnail,
            ExifIfd_EXIF_IFD_EXIF => IFD::EXIF,
            ExifIfd_EXIF_IFD_GPS => IFD::GPS,
            ExifIfd_EXIF_IFD_INTEROPERABILITY => IFD::Interoperability,
            _ => panic!("unknonw ifd value"),
        }
    }
}

impl ToLibExif<ExifIfd> for IFD {
    fn to_libexif(&self) -> ExifIfd {
        match *self {
            IFD::Image => ExifIfd_EXIF_IFD_0,
            IFD::Thumbnail => ExifIfd_EXIF_IFD_1,
            IFD::EXIF => ExifIfd_EXIF_IFD_EXIF,
            IFD::GPS => ExifIfd_EXIF_IFD_GPS,
            IFD::Interoperability => ExifIfd_EXIF_IFD_INTEROPERABILITY,
        }
    }
}

/// Requirement specificatoins for standard EXIF tags.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum SupportLevel {
    /// EXIF tag is mandatory for the given IFD.
    Required,
    /// EXIF tag is optional for the given IFD.
    Optional,
    /// EXIF tag is not allowed for the given IFD.
    NotAllowed,
    /// EXIF tag is not known.
    Unknown,
}

impl FromLibExif<ExifSupportLevel> for SupportLevel {
    fn from_libexif(support_level: ExifSupportLevel) -> Self {
        match support_level {
            ExifSupportLevel_EXIF_SUPPORT_LEVEL_MANDATORY => SupportLevel::Required,
            ExifSupportLevel_EXIF_SUPPORT_LEVEL_OPTIONAL => SupportLevel::Optional,
            ExifSupportLevel_EXIF_SUPPORT_LEVEL_NOT_RECORDED => SupportLevel::NotAllowed,
            ExifSupportLevel_EXIF_SUPPORT_LEVEL_UNKNOWN => SupportLevel::Unknown,
            _ => panic!("illegal support level value"),
        }
    }
}

impl ToLibExif<ExifSupportLevel> for SupportLevel {
    fn to_libexif(&self) -> ExifSupportLevel {
        match *self {
            SupportLevel::Required => ExifSupportLevel_EXIF_SUPPORT_LEVEL_MANDATORY,
            SupportLevel::Optional => ExifSupportLevel_EXIF_SUPPORT_LEVEL_OPTIONAL,
            SupportLevel::NotAllowed => ExifSupportLevel_EXIF_SUPPORT_LEVEL_NOT_RECORDED,
            SupportLevel::Unknown => ExifSupportLevel_EXIF_SUPPORT_LEVEL_UNKNOWN,
        }
    }
}
