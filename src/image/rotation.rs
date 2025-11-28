use std::{fmt::Display, str::FromStr};

use image::DynamicImage;
use image::GenericImageView;

use crate::IiifError;

/// Rotation 旋转角度定义
///
/// This module defines the `Rotation` enum for IIIF image rotation.
///
/// ```
/// use i3f::image::Rotation;
/// use std::str::FromStr;
///
/// let rotation_90 = Rotation::from_str("90").unwrap();
/// assert_eq!(rotation_90, Rotation::Degrees(90.0));
///
/// let ratation: Rotation = "45.5".parse().unwrap();
/// assert_eq!(ratation, Rotation::Degrees(45.5));
/// ```
#[derive(Debug, PartialEq)]
pub enum Rotation {
    /// Format: `n`
    /// The degrees of clockwise rotation from 0 up to 360.
    ///
    /// 旋转角度，顺时针方向，从 0 到 360 度。
    Degrees(f32), // 旋转角度

    /// Format: `!n`
    /// The image should be mirrored and then rotated as above.
    ///
    /// 镜像旋转，先进行镜像处理，然后再进行旋转。
    MirrorDegrees(f32), // 镜像旋转角度
}

impl Rotation {
    pub fn process(&self, image: DynamicImage) -> Result<DynamicImage, IiifError> {
        match self {
            Rotation::Degrees(angle) => {
                if *angle < 0.0 || *angle > 360.0 {
                    return Err(IiifError::BadRequest(
                        "Rotation angle is out of range".to_string(),
                    ));
                }
                if is_multiple_of_90(*angle) {
                    return Ok(standard_rotate(image, *angle));
                }
                Ok(rotate(image, *angle))
            }
            Rotation::MirrorDegrees(angle) => {
                if *angle < 0.0 || *angle > 360.0 {
                    return Err(IiifError::BadRequest(
                        "Rotation angle is out of range".to_string(),
                    ));
                }
                let image = image.fliph();
                if is_multiple_of_90(*angle) {
                    return Ok(standard_rotate(image, *angle));
                }
                Ok(rotate(image, *angle))
            }
        }
    }
}

// 判断是否是 0/90/180/270 的倍数
fn is_multiple_of_90(angle: f32) -> bool {
    angle % 90.0 == 0.0
}

// 针对 0/90/180/270 的倍数，进行优化
fn standard_rotate(image: DynamicImage, angle: f32) -> DynamicImage {
    match angle {
        0.0 => image,
        90.0 => image.rotate90(),
        180.0 => image.rotate180(),
        270.0 => image.rotate270(),
        _ => image,
    }
}

fn rotate(image: DynamicImage, angle: f32) -> DynamicImage {
    // 旋转角度转换为弧度
    let angle = angle * std::f32::consts::PI / 180.0;
    // 计算旋转后的图片大小
    let new_width =
        (image.width() as f32 * angle.cos() + image.height() as f32 * angle.sin()).round() as u32;
    let new_height =
        (image.width() as f32 * angle.sin() + image.height() as f32 * angle.cos()).round() as u32;
    let mut rotated_image = image::ImageBuffer::new(new_width, new_height);
    for x in 0..image.width() {
        for y in 0..image.height() {
            let new_x = x + ((new_width as f32 - image.width() as f32) / 2.0).round() as u32;
            let new_y = y + ((new_height as f32 - image.height() as f32) / 2.0).round() as u32;
            let pixel = image.get_pixel(x, y);
            rotated_image.put_pixel(new_x, new_y, pixel);
        }
    }
    let rotated_image = imageproc::geometric_transformations::rotate_about_center(
        &rotated_image,
        angle,
        imageproc::geometric_transformations::Interpolation::Bicubic,
        image::Rgba([0, 0, 0, 0]),
    );
    image::DynamicImage::ImageRgba8(rotated_image)
}

impl FromStr for Rotation {
    type Err = IiifError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s_trimmed = s.trim();
        if s_trimmed.is_empty() {
            return Err(IiifError::BadRequest("Invalid rotation".to_string()));
        }

        let (is_mirror, angle_str) = if let Some(angle_str) = s.strip_prefix('!') {
            (true, angle_str)
        } else {
            (false, s)
        };

        let angle = angle_str
            .parse::<f32>()
            .map_err(|_| IiifError::BadRequest("Invalid rotation".to_string()))?;

        Ok(if is_mirror {
            Rotation::MirrorDegrees(angle)
        } else {
            Rotation::Degrees(angle)
        })
    }
}

impl Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rotation::Degrees(angle) => write!(f, "{angle}"),
            Rotation::MirrorDegrees(angle) => write!(f, "!{angle}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::storage::LocalStorage;
    use crate::storage::Storage;

    use super::*;

    #[test]
    fn test_rotation_from_str() {
        assert_eq!(Rotation::from_str("90").unwrap(), Rotation::Degrees(90.0));
        assert_eq!(Rotation::from_str("45.5").unwrap(), Rotation::Degrees(45.5));
        assert_eq!(
            Rotation::from_str("!180").unwrap(),
            Rotation::MirrorDegrees(180.0)
        );
        assert_eq!(
            Rotation::from_str("!22.5").unwrap(),
            Rotation::MirrorDegrees(22.5)
        );

        // 错误情况
        assert!(Rotation::from_str("").is_err());
        assert!(Rotation::from_str("invalid").is_err());
        assert!(Rotation::from_str("!").is_err());
        assert!(Rotation::from_str("!abc").is_err());
    }

    #[test]
    fn test_rotation_display() {
        assert_eq!(format!("{}", Rotation::Degrees(45.0)), "45");
        assert_eq!(format!("{}", Rotation::Degrees(22.5)), "22.5");
        assert_eq!(format!("{}", Rotation::MirrorDegrees(270.0)), "!270");
        assert_eq!(format!("{}", Rotation::MirrorDegrees(15.75)), "!15.75");
    }

    #[test]
    fn test_rotation_process() {
        let storage = LocalStorage::new("./fixtures");
        let cases = vec![
            ("0", 300, 200),
            ("180", 300, 200),
            ("90", 200, 300),
            ("!0", 300, 200),
            ("!180", 300, 200),
            ("22.5", 354, 300),
        ];
        for case in cases {
            let rotation = case.0.parse::<Rotation>().unwrap();
            let image = image::open(storage.get_file_path("demo.jpg")).unwrap();
            let rotated_image = rotation.process(image).unwrap();
            assert_eq!(rotated_image.width(), case.1);
            assert_eq!(rotated_image.height(), case.2);
        }
    }
}
