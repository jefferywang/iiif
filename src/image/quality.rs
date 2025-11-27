use std::{fmt::Display, str::FromStr};

use image::DynamicImage;

use crate::IiifError;

/// Quality 画质定义
///
/// ```
/// use iiif::Quality;
/// use std::str::FromStr;
///
/// let quality_default = Quality::from_str("default").unwrap();
/// assert_eq!(quality_default, Quality::Default);
///
/// let quality_color: Quality = "color".parse().unwrap();
/// assert_eq!(quality_color, Quality::Color);
/// ```
#[derive(Debug, PartialEq)]
pub enum Quality {
    /// Format: `default`
    ///
    /// The image is returned using the server’s default quality (e.g. `color`, `gray` or `bitonal`) for the image.
    ///
    /// 图像将使用服务器为该图像设置的默认质量（例如 `color` 、 `gray` 或 `bitonal` ）返回。
    Default,

    /// Format: `color`
    ///
    /// The image is returned with all of its color information.
    ///
    /// 图像将完整保留所有色彩信息返回。
    Color,

    /// Format: `gray`
    ///
    /// The image is returned in grayscale, where each pixel is black, white or any shade of gray in between.
    ///
    /// 图像以灰度形式返回，每个像素点为黑色、白色或介于其间的任意灰度色阶。
    Gray,

    /// Format: `bitonal`
    ///
    /// The image returned is bitonal, where each pixel is either black or white.
    ///
    /// 返回的图像为双色调，每个像素点仅呈现纯黑或纯白。
    Bitonal,
}

impl FromStr for Quality {
    type Err = IiifError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s_trimmed = s.trim().to_lowercase();
        if s_trimmed.is_empty() {
            return Err(IiifError::InvalidQualityFormat(s.to_string()));
        }

        match s_trimmed.as_str() {
            "default" => Ok(Quality::Default),
            "color" => Ok(Quality::Color),
            "gray" => Ok(Quality::Gray),
            "bitonal" => Ok(Quality::Bitonal),
            _ => Err(IiifError::InvalidQualityFormat(s.to_string())),
        }
    }
}

impl Quality {
    pub fn process(&self, image: DynamicImage) -> Result<DynamicImage, IiifError> {
        match self {
            Quality::Default => Ok(image),
            Quality::Color => Ok(image),
            Quality::Gray => Ok(image.grayscale()),
            Quality::Bitonal => {
                // 先转换为灰度图
                let gray_image = image.to_luma8();

                // 二值化处理：阈值设为128，大于阈值的为白色(255)，小于等于阈值的为黑色(0)
                let threshold = 170u8;
                let binary_image = imageproc::map::map_pixels(&gray_image, |_x, _y, pixel| {
                    if pixel[0] > threshold {
                        image::Luma([255u8]) // 白色
                    } else {
                        image::Luma([0u8]) // 黑色
                    }
                });

                Ok(image::DynamicImage::ImageLuma8(binary_image))
            }
        }
    }
}

impl Display for Quality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Quality::Default => write!(f, "default"),
            Quality::Color => write!(f, "color"),
            Quality::Gray => write!(f, "gray"),
            Quality::Bitonal => write!(f, "bitonal"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{LocalStorage, Storage};

    use super::*;

    #[test]
    fn test_quality_from_str() {
        assert_eq!(Quality::from_str("default").unwrap(), Quality::Default);
        assert_eq!(Quality::from_str("color").unwrap(), Quality::Color);
        assert_eq!(Quality::from_str("gray").unwrap(), Quality::Gray);
        assert_eq!(Quality::from_str("bitonal").unwrap(), Quality::Bitonal);
        assert_eq!(Quality::from_str("Default").unwrap(), Quality::Default);

        // 错误情况
        assert!(Quality::from_str("").is_err());
        assert!(Quality::from_str("invalid").is_err());
    }

    #[test]
    fn test_quality_display() {
        assert_eq!(format!("{}", Quality::Default), "default");
        assert_eq!(format!("{}", Quality::Color), "color");
        assert_eq!(format!("{}", Quality::Gray), "gray");
        assert_eq!(format!("{}", Quality::Bitonal), "bitonal");
    }

    #[test]
    fn test_quality_process() {
        let storage = LocalStorage::new("./fixtures");
        let cases = vec![
            ("default", 300, 200),
            ("color", 300, 200),
            ("gray", 300, 200),
            ("bitonal", 300, 200),
        ];
        for case in cases {
            let quality = case.0.parse::<Quality>().unwrap();
            let image = image::open(storage.get_file_path("demo.jpg")).unwrap();
            let processed_image = quality.process(image).unwrap();
            assert_eq!(processed_image.width(), case.1);
            assert_eq!(processed_image.height(), case.2);
        }
    }
}
