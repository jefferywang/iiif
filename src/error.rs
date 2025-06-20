use thiserror::Error;

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

    #[error("Invalid quality format: {0}")]
    InvalidQualityFormat(String),

    #[error("Invalid rotation format: {0}")]
    InvalidRotationFormat(String),

    #[error("Invalid image information document")]
    InvalidImageInfo,

    #[error("Image not found")]
    ImageNotFound,

    #[error("Internal server error")]
    InternalServerError,
}
