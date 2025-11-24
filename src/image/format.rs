use std::{fmt::Display, str::FromStr};

use crate::IiifError;

/// Format 格式定义
///
/// ```
/// use iiif::Format;
/// use std::str::FromStr;
///
/// let format_jpg = Format::from_str("jpg").unwrap();
/// println!("{:?}", format_jpg);
///
/// let format_png: Format = "png".parse().unwrap();
/// println!("{:?}", format_png);
/// ```
#[derive(Debug, PartialEq)]
pub enum Format {
    /// Format: `jpg`
    ///
    /// The image is returned in JPEG format.
    ///
    /// 图像将以 JPEG 格式返回。
    Jpg,

    /// Format: `tif`
    ///
    /// The image is returned in tif format.
    ///
    /// 图像将以 TIF 格式返回。
    Tif,

    /// Format: `png`
    ///
    /// The image is returned in PNG format.
    ///
    /// 图像将以 PNG 格式返回。
    Png,

    /// Format: `gif`
    ///
    /// The image is returned in GIF format.
    ///
    /// 图像将以 GIF 格式返回。
    Gif,

    /// Format: `jp2`
    ///
    /// The image is returned in JPEG 2000 format.
    ///
    /// 图像将以 JPEG 2000 格式返回。
    Jp2,

    /// Format: `pdf`
    ///
    /// The image is returned in PDF format.
    ///
    /// 图像将以 PDF 格式返回。
    Pdf,

    /// Format: `webp`
    ///
    /// The image is returned in WebP format.
    ///
    /// 图像将以 WebP 格式返回。
    Webp,
}

impl FromStr for Format {
    type Err = IiifError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s_trimmed = s.trim().to_lowercase();
        if s_trimmed.is_empty() {
            return Err(IiifError::InvalidFormat(s.to_string()));
        }

        match s_trimmed.as_str() {
            "jpg" => Ok(Format::Jpg),
            "tif" => Ok(Format::Tif),
            "png" => Ok(Format::Png),
            "gif" => Ok(Format::Gif),
            "jp2" => Ok(Format::Jp2),
            "pdf" => Ok(Format::Pdf),
            "webp" => Ok(Format::Webp),
            _ => Err(IiifError::InvalidFormat(s.to_string())),
        }
    }
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Format::Jpg => write!(f, "jpg"),
            Format::Tif => write!(f, "tif"),
            Format::Png => write!(f, "png"),
            Format::Gif => write!(f, "gif"),
            Format::Jp2 => write!(f, "jp2"),
            Format::Pdf => write!(f, "pdf"),
            Format::Webp => write!(f, "webp"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_from_str() {
        assert_eq!(Format::from_str("jpg").unwrap(), Format::Jpg);
        assert_eq!(Format::from_str("tif").unwrap(), Format::Tif);
        assert_eq!(Format::from_str("png").unwrap(), Format::Png);
        assert_eq!(Format::from_str("gif").unwrap(), Format::Gif);
        assert_eq!(Format::from_str("jp2").unwrap(), Format::Jp2);
        assert_eq!(Format::from_str("pdf").unwrap(), Format::Pdf);
        assert_eq!(Format::from_str("webp").unwrap(), Format::Webp);

        // 错误情况
        assert!(Format::from_str("").is_err());
        assert!(Format::from_str("invalid").is_err());
    }

    #[test]
    fn test_format_display() {
        assert_eq!(format!("{}", Format::Jpg), "jpg");
        assert_eq!(format!("{}", Format::Tif), "tif");
        assert_eq!(format!("{}", Format::Png), "png");
        assert_eq!(format!("{}", Format::Gif), "gif");
        assert_eq!(format!("{}", Format::Jp2), "jp2");
        assert_eq!(format!("{}", Format::Pdf), "pdf");
        assert_eq!(format!("{}", Format::Webp), "webp");
    }
}
