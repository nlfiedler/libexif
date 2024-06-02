//
// Copyright (c) 2016 David Cuddeback
//
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

impl TryFrom<ExifByteOrder> for ByteOrder {
    type Error = super::ExifError;

    fn try_from(byte_order: ExifByteOrder) -> Result<Self, Self::Error> {
        match byte_order {
            ExifByteOrder_EXIF_BYTE_ORDER_MOTOROLA => Ok(ByteOrder::BigEndian),
            ExifByteOrder_EXIF_BYTE_ORDER_INTEL => Ok(ByteOrder::LittleEndian),
            _ => Err(super::ExifError::IllegalByteOrder),
        }
    }
}

impl Into<ExifByteOrder> for ByteOrder {
    fn into(self) -> ExifByteOrder {
        match self {
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

impl TryFrom<ExifDataType> for DataEncoding {
    type Error = super::ExifError;

    fn try_from(data_type: ExifDataType) -> Result<Self, Self::Error> {
        match data_type {
            ExifDataType_EXIF_DATA_TYPE_UNCOMPRESSED_CHUNKY => Ok(DataEncoding::Chunky),
            ExifDataType_EXIF_DATA_TYPE_UNCOMPRESSED_PLANAR => Ok(DataEncoding::Planar),
            ExifDataType_EXIF_DATA_TYPE_UNCOMPRESSED_YCC => Ok(DataEncoding::Ycc),
            ExifDataType_EXIF_DATA_TYPE_COMPRESSED => Ok(DataEncoding::Compressed),
            ExifDataType_EXIF_DATA_TYPE_UNKNOWN => Ok(DataEncoding::Unknown),
            _ => Err(super::ExifError::IllegalDataType),
        }
    }
}

impl Into<ExifDataType> for DataEncoding {
    fn into(self) -> ExifDataType {
        match self {
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

impl TryFrom<ExifDataOption> for DataOption {
    type Error = super::ExifError;

    fn try_from(data_option: ExifDataOption) -> Result<Self, Self::Error> {
        match data_option {
            ExifDataOption_EXIF_DATA_OPTION_IGNORE_UNKNOWN_TAGS => {
                Ok(DataOption::IgnoreUnknownTags)
            }
            ExifDataOption_EXIF_DATA_OPTION_FOLLOW_SPECIFICATION => {
                Ok(DataOption::FollowSpecification)
            }
            ExifDataOption_EXIF_DATA_OPTION_DONT_CHANGE_MAKER_NOTE => {
                Ok(DataOption::DontChangeMakerNote)
            }
            _ => Err(super::ExifError::IllegalDataOption),
        }
    }
}

impl Into<ExifDataOption> for DataOption {
    fn into(self) -> ExifDataOption {
        match self {
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
        unsafe { exif_format_get_size((*self).into()) as usize }
    }
}

impl TryFrom<ExifFormat> for DataType {
    type Error = super::ExifError;

    fn try_from(format: ExifFormat) -> Result<Self, Self::Error> {
        match format {
            ExifFormat_EXIF_FORMAT_ASCII => Ok(DataType::Text),
            ExifFormat_EXIF_FORMAT_BYTE => Ok(DataType::U8),
            ExifFormat_EXIF_FORMAT_SBYTE => Ok(DataType::I8),
            ExifFormat_EXIF_FORMAT_SHORT => Ok(DataType::U16),
            ExifFormat_EXIF_FORMAT_SSHORT => Ok(DataType::I16),
            ExifFormat_EXIF_FORMAT_LONG => Ok(DataType::U32),
            ExifFormat_EXIF_FORMAT_SLONG => Ok(DataType::I32),
            ExifFormat_EXIF_FORMAT_RATIONAL => Ok(DataType::URational),
            ExifFormat_EXIF_FORMAT_SRATIONAL => Ok(DataType::IRational),
            ExifFormat_EXIF_FORMAT_UNDEFINED => Ok(DataType::Undefined),
            _ => Err(super::ExifError::IllegalDataType)
        }
    }
}

impl Into<ExifFormat> for DataType {
    fn into(self) -> ExifFormat {
        match self {
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

impl TryFrom<ExifIfd> for IFD {
    type Error = super::ExifError;

    fn try_from(ifd: ExifIfd) -> Result<Self, Self::Error> {
        match ifd {
            ExifIfd_EXIF_IFD_0 => Ok(IFD::Image),
            ExifIfd_EXIF_IFD_1 => Ok(IFD::Thumbnail),
            ExifIfd_EXIF_IFD_EXIF => Ok(IFD::EXIF),
            ExifIfd_EXIF_IFD_GPS => Ok(IFD::GPS),
            ExifIfd_EXIF_IFD_INTEROPERABILITY => Ok(IFD::Interoperability),
            _ => Err(super::ExifError::UnknownIFD),
        }
    }
}

impl Into<ExifIfd> for IFD {
    fn into(self) -> ExifIfd {
        match self {
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

impl TryFrom<ExifSupportLevel> for SupportLevel {
    type Error = super::ExifError;

    fn try_from(support_level: ExifSupportLevel) -> Result<Self, Self::Error> {
        match support_level {
            ExifSupportLevel_EXIF_SUPPORT_LEVEL_MANDATORY => Ok(SupportLevel::Required),
            ExifSupportLevel_EXIF_SUPPORT_LEVEL_OPTIONAL => Ok(SupportLevel::Optional),
            ExifSupportLevel_EXIF_SUPPORT_LEVEL_NOT_RECORDED => Ok(SupportLevel::NotAllowed),
            ExifSupportLevel_EXIF_SUPPORT_LEVEL_UNKNOWN => Ok(SupportLevel::Unknown),
            _ => Err(super::ExifError::IllegalSupportLevel),
        }
    }
}

impl Into<ExifSupportLevel> for SupportLevel {
    fn into(self) -> ExifSupportLevel {
        match self {
            SupportLevel::Required => ExifSupportLevel_EXIF_SUPPORT_LEVEL_MANDATORY,
            SupportLevel::Optional => ExifSupportLevel_EXIF_SUPPORT_LEVEL_OPTIONAL,
            SupportLevel::NotAllowed => ExifSupportLevel_EXIF_SUPPORT_LEVEL_NOT_RECORDED,
            SupportLevel::Unknown => ExifSupportLevel_EXIF_SUPPORT_LEVEL_UNKNOWN,
        }
    }
}
