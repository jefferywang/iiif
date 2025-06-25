use std::{fmt::Display, str::FromStr};

use crate::IiifError;

/// Rotation 旋转角度定义
///
/// This module defines the `Rotation` enum for IIIF image rotation.
///
/// ```
/// use iiif::Rotation;
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

impl FromStr for Rotation {
    type Err = IiifError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s_trimmed = s.trim();
        if s_trimmed.is_empty() {
            return Err(IiifError::InvalidRotationFormat(s.to_string()));
        }

        let (is_mirror, angle_str) = if let Some(angle_str) = s.strip_prefix('!') {
            (true, angle_str)
        } else {
            (false, s)
        };

        let angle = angle_str
            .parse::<f32>()
            .map_err(|_| IiifError::InvalidRotationFormat(s.to_string()))?;

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
            Rotation::Degrees(angle) => write!(f, "{}", angle),
            Rotation::MirrorDegrees(angle) => write!(f, "!{}", angle),
        }
    }
}

#[cfg(test)]
mod tests {
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
}
