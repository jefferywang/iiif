use thiserror::Error;

/// IiifError 定义了 IIIF 相关的错误类型
#[derive(Debug, Error)]
pub enum IiifError {
    /// Invalid region format, accepted formats are `full`, `square`, `x,y,w,h` or `pct:x,y,w,h`.
    ///
    /// region 格式无效，支持的格式 `full`, `square`, `x,y,w,h` 或 `pct:x,y,w,h`。
    #[error("Invalid region format: {0}")]
    InvalidRegionFormat(String),

    /// Invalid size format, accepted formats include `max`, `^max`, `w,`, `^w,`, `,h`, `^,h`, `pct:n`,
    /// `^pct:n`, `w,h`, `^w,h`, `^!w,h`.
    #[error("Invalid size format: {0}")]
    InvalidSizeFormat(String),

    /// Invalid quality format, accepted formats include `default`, `bitonal`, `gray`, `color`.
    #[error("Invalid quality format: {0}")]
    InvalidQualityFormat(String),

    /// Invalid rotation format, accepted formats include `n` or `!n`.
    #[error("Invalid rotation format: {0}")]
    InvalidRotationFormat(String),

    /// Invalid format, accepted formats include `jpg`, `tif`, `png`, `gif`, `jp2`, `pdf`, `webp`.
    #[error("Invalid format: {0}")]
    InvalidFormat(String),

    #[error("Invalid IIIF image URL: {0}")]
    InvalidIiifURL(String),

    /// Invalid identifier.
    #[error("Invalid identifier: {0}")]
    InvalidIdentifier(String),

    #[error("Image open failed: {0}")]
    ImageOpenFailed(String),

    #[error("Region is invalid: {0}")]
    RegionIsInvalid(String),

    #[error("Image not found")]
    ImageNotFound,

    #[error("Internal server error")]
    InternalServerError,
}
