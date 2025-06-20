use std::{fmt::Display, str::FromStr};

use crate::IiifError;

/// iiif Size的定义
/// 
/// Example:
/// ```
/// use iiif::Size;
/// use std::str::FromStr;
/// 
/// let size = Size::from_str("max").unwrap();
/// assert_eq!(size, Size::Max);
/// 
/// let size: Size = "max".parse().unwrap();
/// assert_eq!(size, Size::Max);
/// ```
#[derive(Debug, PartialEq)]
pub enum Size {
    /// Format: `max`
    /// The extracted region is returned at the maximum size available, but will not be upscaled.
    /// The resulting image will have the pixel dimensions of the extracted region,
    /// unless it is constrained to a smaller size by `maxWidth`, `maxHeight`,
    /// or `maxArea` as defined in the Technical Properties section.
    ///
    /// 提取的区域将以最大可用尺寸返回，但不会进行放大处理。
    /// 除非受到技术属性章节中定义的 `maxWidth`, `maxHeight` 或 `maxArea` 参数限制而缩小尺寸，
    /// 否则生成图像的像素尺寸将与提取区域保持一致。
    Max,
    /// Format: `^max`
    /// The extracted region is scaled to the maximum size permitted by maxWidth, maxHeight, or maxArea as
    /// defined in the Technical Properties section. If the resulting dimensions are greater than the pixel
    /// width and height of the extracted region, the extracted region is upscaled.
    ///
    /// 提取的区域将缩放至技术属性章节中定义的 `maxWidth`, `maxHeight` 或 `maxArea` 所允许的最大尺寸。
    /// 若最终尺寸大于提取区域的像素宽度和高度，则对提取区域进行放大处理。
    CMax,
    /// Format: `w,`
    /// The extracted region should be scaled so that the width of the returned image is exactly equal to `w`.
    /// The value of `w` must not be greater than the width of the extracted region.
    ///
    /// 提取的区域应按比例缩放，使返回图像的宽度精确等于 `w` 。 `w` 的值不得大于提取区域的宽度。
    W { w: u32 },
    /// Format: `^w,`
    /// The extracted region should be scaled so that the width of the returned image is exactly equal to `w`.
    /// If `w` is greater than the pixel width of the extracted region, the extracted region is upscaled.
    ///
    /// 提取的区域应按比例缩放，使返回图像的宽度精确等于 `w` 。如果 `w` 大于提取区域的像素宽度，则提取区域将被放大。
    CW { w: u32 },
    /// Format: `,h`
    /// The extracted region should be scaled so that the height of the returned image is exactly equal to `h`.
    /// The value of `h` must not be greater than the height of the extracted region.
    ///
    /// 提取的区域应按比例缩放，使返回图像的高度精确等于 `h` 。 `h` 的值不得大于提取区域的高度。
    H { h: u32 },
    /// Format: `^,h`
    /// The extracted region should be scaled so that the height of the returned image is exactly equal to `h`.
    /// If `h` is greater than the pixel height of the extracted region, the extracted region is upscaled.
    ///
    /// 提取的区域应按比例缩放，使返回图像的高度精确等于 `h` 。如果 `h` 大于提取区域的像素高度，则提取区域将被放大。
    CH { h: u32 },
    /// Format: `pct:n`
    /// The width and height of the returned image is scaled to `n` percent of the width and height of the
    /// extracted region. The value of `n` must not be greater than 100.
    /// 
    /// 返回图像的宽度和高度将缩放至提取区域宽高的 `n` 百分比。 `n` 的取值不得超过 100。
    Pct { n: u32 },
    /// Format: `^pct:n`
    /// The width and height of the returned image is scaled to `n` percent of the width and height of the
    /// extracted region. For values of `n` greater than 100, the extracted region is upscaled.
    ///
    /// 返回图像的宽度和高度将缩放至提取区域宽高的 `n` 百分比。当 `n` 取值超过 100 时，提取区域将被放大。
    CPct { n: u32 },
    /// Format: `w,h`
    /// The width and height of the returned image are exactly `w` and `h`.
    /// The aspect ratio of the returned image may be significantly different than the extracted region,
    /// resulting in a distorted image. The values of `w` and `h` must not be greater than the corresponding
    /// pixel dimensions of the extracted region.
    ///
    /// 返回图像的宽度和高度严格限定为 `w` 和 `h` 。返回图像的宽高比可能与提取区域存在显著差异，导致图像变形。
    /// `w` 和 `h` 的取值不得超过提取区域对应的像素尺寸。
    WH { w: u32, h: u32 },
    /// Format: `^w,h`
    /// The width and height of the returned image are exactly `w` and `h`. The aspect ratio of the returned
    /// image may be significantly different than the extracted region, resulting in a distorted image.
    /// If `w` and/or `h` are greater than the corresponding pixel dimensions of the extracted region, the
    /// extracted region is upscaled.
    /// 
    /// 返回图像的宽度和高度精确为 `w` 和 `h` 。返回图像的宽高比可能与提取区域存在显著差异，导致图像变形。
    /// 若 `w` 和/或 `h` 大于提取区域的对应像素尺寸，则提取区域将被放大。
    CWH { w: u32, h: u32 },
    /// Format: `!w,h`
    /// The extracted region is scaled so that the width and height of the returned image are not greater
    /// than `w` and `h`, while maintaining the aspect ratio. The returned image must be as large as possible
    /// but not larger than the extracted region, `w` or `h`, or server-imposed limits.
    /// 
    /// 提取的区域会进行缩放，使返回图像的宽度和高度不超过 `w` 和 `h` ，同时保持宽高比。返回的图像应尽可能大，但不得超过提取区域、
    /// `w` 或 `h` 的尺寸，或服务器设定的限制。
    LWH { w: u32, h: u32 },
    /// Format: `^!w,h`
    /// The extracted region is scaled so that the width and height of the returned image are not greater than `w` and `h`,
    /// while maintaining the aspect ratio. The returned image must be as large as possible but not larger than `w`, `h`,
    /// or server-imposed limits.
    ///
    /// 提取的区域会进行缩放，使返回图像的宽度和高度不超过 `w` 和 `h` ，同时保持宽高比。返回的图像应尽可能大，但不得超过 `w` 、 `h` 的尺寸，
    /// 或服务器设定的限制。
    CLWH { w: u32, h: u32 },
}

impl FromStr for Size {
    type Err = IiifError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().to_lowercase();
        
        // 处理关键词
        match s.as_str() {
            "max" => return Ok(Size::Max),
            "^max" => return Ok(Size::CMax),
            _ => {}
        }

        // 分离 caret 前缀
        let (caret, content) = s.strip_prefix('^')
            .map(|c| (true, c))
            .unwrap_or((false, s.as_str()));

        // 解析具体格式
        Self::parse_content(content, caret)
            .ok_or_else(|| IiifError::InvalidSizeFormat(s))
    }
}

impl Size {
    fn parse_content(content: &str, caret: bool) -> Option<Self> {
        if let Some(pct) = content.strip_prefix("pct:") {
            Self::parse_pct(pct, caret)
        } else if let Some(coords) = content.strip_prefix('!') {
            Self::parse_fit(coords, caret)
        } else if content.contains(',') {
            Self::parse_dims(content, caret)
        } else {
            None
        }
    }

    fn parse_pct(pct_str: &str, caret: bool) -> Option<Self> {
        let n = pct_str.parse().ok()?;
        if caret {
            Some(Size::CPct { n })
        } else if n <= 100 {
            Some(Size::Pct { n })
        } else {
            None
        }
    }

    fn parse_fit(coords: &str, caret: bool) -> Option<Self> {
        let (w, h) = Self::parse_two_nums(coords)?;
        Some(if caret { 
            Size::CLWH { w, h } 
        } else { 
            Size::LWH { w, h } 
        })
    }

    fn parse_dims(content: &str, mut caret: bool) -> Option<Self> {
        let parts: Vec<&str> = content.split(',').collect();
        if parts.len() != 2 { return None; }

        let w = if parts[0].is_empty() { None } else { parts[0].parse().ok() };
        let h = if parts[1].is_empty() { 
            None 
        } else if let Some(h_str) = parts[1].strip_prefix('^') {
            caret = true;
            h_str.parse().ok()
        } else {
            parts[1].parse().ok()
        };

        match (w, h, caret) {
            (Some(w), Some(h), true) => Some(Size::CWH { w, h }),
            (Some(w), Some(h), false) => Some(Size::WH { w, h }),
            (Some(w), None, true) => Some(Size::CW { w }),
            (Some(w), None, false) => Some(Size::W { w }),
            (None, Some(h), true) => Some(Size::CH { h }),
            (None, Some(h), false) => Some(Size::H { h }),
            _ => None,
        }
    }

    fn parse_two_nums(coords: &str) -> Option<(u32, u32)> {
        let mut parts = coords.split(',');
        let w = parts.next()?.parse().ok()?;
        let h = parts.next()?.parse().ok()?;
        if parts.next().is_some() { return None; }
        Some((w, h))
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Size::Max => write!(f, "max"),
            Size::CMax => write!(f, "^max"),
            Size::W { w } => write!(f, "{},", w),
            Size::CW { w } => write!(f, "^{},", w),
            Size::H { h } => write!(f, ",{}", h),
            Size::CH { h } => write!(f, "^,{}", h),
            Size::Pct { n } => write!(f, "pct:{}", n),
            Size::CPct { n } => write!(f, "^pct:{}", n),
            Size::WH { w, h } => write!(f, "{},{}", w, h),
            Size::CWH { w, h } => write!(f, "^{},{}", w, h),
            Size::LWH { w, h } => write!(f, "!{},{}", w, h),
            Size::CLWH { w, h } => write!(f, "^!{},{}", w, h),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_parsing() {
        // 基本格式
        assert_eq!(Size::from_str("max").unwrap(), Size::Max);
        assert_eq!(Size::from_str("^max").unwrap(), Size::CMax);

        // 宽度/高度
        assert_eq!(Size::from_str("150,").unwrap(), Size::W { w: 150 });
        assert_eq!(Size::from_str("^360,").unwrap(), Size::CW { w: 360 });
        assert_eq!(Size::from_str(",150").unwrap(), Size::H { h: 150 });
        assert_eq!(Size::from_str("^,240").unwrap(), Size::CH { h: 240 });
        assert_eq!(Size::from_str(",^240").unwrap(), Size::CH { h: 240 });
        
        // 百分比
        assert_eq!(Size::from_str("pct:50").unwrap(), Size::Pct { n: 50 });
        assert!(Size::from_str("pct:150").is_err());
        assert_eq!(Size::from_str("^pct:150").unwrap(), Size::CPct { n: 150 });
        
        // 精确尺寸
        assert_eq!(Size::from_str("225,100").unwrap(), Size::WH { w: 225, h: 100 });
        assert_eq!(Size::from_str("^360,360").unwrap(), Size::CWH { w: 360, h: 360 });
        
        // 最佳适配
        assert_eq!(Size::from_str("!225,100").unwrap(), Size::LWH { w: 225, h: 100 });
        assert_eq!(Size::from_str("^!360,360").unwrap(), Size::CLWH { w: 360, h: 360 });
    }

    #[test]
    fn test_size_display() {
        assert_eq!(format!("{}", Size::Max), "max");
        assert_eq!(format!("{}", Size::Pct { n: 50 }), "pct:50");
        assert_eq!(format!("{}", Size::WH { w: 100, h: 200 }), "100,200");
        assert_eq!(format!("{}", Size::LWH { w: 100, h: 200 }), "!100,200");
    }

    #[test]
    fn test_roundtrip() {
        let cases = ["max", "^max", "150,", "^360,", ",150", "pct:50", "^pct:150", 
                    "225,100", "^360,360", "!225,100", "^!360,360"];
        
        for case in cases {
            let size = Size::from_str(case).unwrap();
            assert_eq!(format!("{}", size), case);
        }
    }
}

