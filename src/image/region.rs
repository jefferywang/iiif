use image::DynamicImage;

use crate::error;
use std::fmt::Display;
use std::str::FromStr;

/// Region 裁剪区域的定义
///
/// 使用示例：
/// ```
/// use iiif::Region;
/// use std::str::FromStr;
///
/// let region_full = Region::from_str("full").unwrap();
/// let region_square = Region::from_str("square").unwrap();
/// let region_rect = Region::from_str("125,15,120,140").unwrap();
/// let region_pct = Region::from_str("pct:41.6,7.5,66.6,100").unwrap();
/// assert_eq!(region_full, Region::Full);
/// assert_eq!(region_square, Region::Square);
/// assert_eq!(region_rect, Region::Rect(125, 15, 120, 140));
/// assert_eq!(region_pct, Region::Pct(41.6, 7.5, 66.6, 100.0));
///
/// let region_full = "full".parse::<Region>().unwrap();
/// let region_squre = "square".parse::<Region>().unwrap();
/// let region_rect = "125,15,120,140".parse::<Region>().unwrap();
/// let region_pct = "pct:41.6,7.5,66.6,100".parse::<Region>().unwrap();
/// assert_eq!(region_full, Region::Full);
/// assert_eq!(region_squre, Region::Square);
/// assert_eq!(region_rect, Region::Rect(125, 15, 120, 140));
/// assert_eq!(region_pct, Region::Pct(41.6, 7.5, 66.6, 100.0));
/// ```
#[derive(Debug, PartialEq)]
pub enum Region {
    /// The full image is returned, without any cropping.
    ///
    /// 完整图像，不进行任何裁剪
    Full,
    /// The region is defined as an area where the width and height are both equal to the length of the shorter dimension of the full image.
    /// The region may be positioned anywhere in the longer dimension of the full image at the server's discretion,
    /// and centered is often a reasonable default.
    ///
    /// 正方形区域，该区域定义为宽度和高度均等于完整图像较短边长的区域。服务器可自行决定将该区域置于完整图像较长边上的任意位置，通常居中是一个合理的默认选项。
    Square,
    /// The region of the full image to be returned is specified in terms of absolute pixel values.
    /// The value of x represents the number of pixels from the 0 position on the horizontal axis.
    /// The value of y represents the number of pixels from the 0 position on the vertical axis.
    /// Thus the x,y position 0,0 is the upper left-most pixel of the image.
    /// w represents the width of the region and h represents the height of the region in pixels.
    ///
    /// 绝对像素值指定的矩形区域 (x, y, width, height)。
    /// 要返回的完整图像区域通过绝对像素值来指定。 x 表示水平轴上距离 0 位置的像素数。 y 表示垂直轴上距离 0 位置的像素数。
    /// 因此， x,y 位置 0,0 对应图像最左上角的像素。 w 表示该区域的宽度， h 表示该区域的高度（均以像素为单位）。
    Rect(u32, u32, u32, u32),
    /// The region to be returned is specified as a sequence of percentages of the full image's dimensions,
    /// as reported in the image information document. Thus,
    /// x represents the number of pixels from the 0 position on the horizontal axis,
    /// calculated as a percentage of the reported width.
    /// w represents the width of the region, also calculated as a percentage of the reported width.
    /// The same applies to y and h respectively.
    ///
    /// 百分比指定的区域 (x%, y%, width%, height%)
    /// 要返回的区域按图像信息文档中报告的完整图像尺寸百分比序列来指定。因此， x 表示水平轴上距离 0 位置的像素数，按报告宽度的百分比计算。
    /// w 表示该区域的宽度，同样按报告宽度的百分比计算。 y 和 h 也分别适用相同的计算方式。
    Pct(f32, f32, f32, f32),
}

/// 实现 FromStr trait 以支持从字符串解析 Region
impl FromStr for Region {
    type Err = error::IiifError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s_lower = s.trim().to_lowercase();
        match s_lower.as_str() {
            "full" => Ok(Region::Full),
            "square" => Ok(Region::Square),
            s if s.starts_with("pct:") => Self::parse_pct_coordinates(&s[4..]),
            s if s.contains(',') => Self::parse_rect_coordinates(s),
            _ => Err(error::IiifError::InvalidRegionFormat(s.to_string())),
        }
    }
}

impl Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Region::Full => write!(f, "full"),
            Region::Square => write!(f, "square"),
            Region::Rect(x, y, w, h) => write!(f, "{},{},{},{}", x, y, w, h),
            Region::Pct(x, y, w, h) => write!(f, "pct:{},{},{},{}", x, y, w, h),
        }
    }
}

impl Region {
    /// 解析矩形坐标字符串（整数像素值）
    fn parse_rect_coordinates(coords: &str) -> Result<Self, error::IiifError> {
        let parts: Vec<&str> = coords.split(',').collect();

        if parts.len() != 4 {
            return Err(error::IiifError::InvalidRegionFormat(coords.to_string()));
        }

        let values: Result<Vec<u32>, _> = parts.iter().map(|part| part.parse::<u32>()).collect();

        match values {
            Ok(vals) => Ok(Region::Rect(vals[0], vals[1], vals[2], vals[3])),
            Err(_) => Err(error::IiifError::InvalidRegionFormat(coords.to_string())),
        }
    }

    /// 解析百分比坐标字符串（浮点数百分比值）
    fn parse_pct_coordinates(coords: &str) -> Result<Self, error::IiifError> {
        let parts: Vec<&str> = coords.split(',').collect();

        if parts.len() != 4 {
            return Err(error::IiifError::InvalidRegionFormat(coords.to_string()));
        }

        let values: Result<Vec<f32>, _> = parts.iter().map(|part| part.parse::<f32>()).collect();

        match values {
            Ok(vals) => Ok(Region::Pct(vals[0], vals[1], vals[2], vals[3])),
            Err(_) => Err(error::IiifError::InvalidRegionFormat(coords.to_string())),
        }
    }

    /// 处理图片的裁剪，返回裁剪后的图片
    ///
    /// Process the image cropping, return the cropped image.
    ///
    /// Example:
    /// ```
    /// use iiif::Region;
    /// use image::DynamicImage;
    ///
    /// let region = Region::Full;
    /// let image = DynamicImage::new(100, 100, image::ColorType::Rgba8);
    /// let cropped_image = region.process(image).unwrap();
    /// ```
    pub fn process(&self, mut image: DynamicImage) -> Result<DynamicImage, error::IiifError> {
        let width = image.width();
        let height = image.height();
        let (x, y, w, h) = self.get_region(width, height)?;
        Ok(image.crop(x, y, w, h))
    }

    /// 获取裁剪的区域，返回 (x, y, w, h)
    ///
    /// Get the region to be cropped, return (x, y, w, h).
    fn get_region(
        &self,
        width: u32,
        height: u32,
    ) -> Result<(u32, u32, u32, u32), error::IiifError> {
        match self {
            Region::Full => Ok((0, 0, width, height)),
            Region::Square => {
                // 按短边居中裁剪
                let min = width.min(height);
                let x = (width - min) / 2;
                let y = (height - min) / 2;
                Ok((x, y, min, min))
            }
            Region::Rect(x, y, w, h) => {
                if *w == 0 || *h == 0 {
                    return Err(error::IiifError::RegionIsInvalid(format!(
                        "Width or height is 0: {}",
                        self
                    )));
                }
                if *x >= width || *y >= height {
                    return Err(error::IiifError::RegionIsInvalid(format!(
                        "X or Y is out of bounds: {}",
                        self
                    )));
                }
                // 检查区域是否超出边界，如果超出边界则直接到图片边缘
                let rw = (*w).min(width - *x);
                let rh = (*h).min(height - *y);
                Ok((*x, *y, rw, rh))
            }
            Region::Pct(x, y, w, h) => {
                if *w == 0.0 || *h == 0.0 {
                    return Err(error::IiifError::RegionIsInvalid(format!(
                        "Width or height is 0: {}",
                        self
                    )));
                }
                if *x >= 100.0 || *y >= 100.0 {
                    return Err(error::IiifError::RegionIsInvalid(format!(
                        "X or Y is out of bounds: {}",
                        self
                    )));
                }
                let rw = (*w).min(100.0 - *x);
                let rh = (*h).min(100.0 - *y);
                // 将百分比转换为像素值并裁剪到边界
                let px = (width as f32 * (*x / 100.0)).round() as u32;
                let py = (height as f32 * (*y / 100.0)).round() as u32;
                let pw = (width as f32 * (rw / 100.0)).round() as u32;
                let ph = (height as f32 * (rh / 100.0)).round() as u32;
                Ok((px, py, pw, ph))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_region_from_str() {
        assert_eq!(Region::from_str("full").unwrap(), Region::Full);
        assert_eq!(Region::from_str("square").unwrap(), Region::Square);
        assert_eq!(
            Region::from_str("125,15,120,140").unwrap(),
            Region::Rect(125, 15, 120, 140)
        );
        assert_eq!(
            Region::from_str("pct:10,20,30,40").unwrap(),
            Region::Pct(10.0, 20.0, 30.0, 40.0)
        );
        assert_eq!(
            Region::from_str("pct:41.6,7.5,40,70").unwrap(),
            Region::Pct(41.6, 7.5, 40.0, 70.0)
        );
        assert!(Region::from_str("invalid").is_err());
    }

    #[test]
    fn test_region_display() {
        assert_eq!(format!("{}", Region::Full), "full");
        assert_eq!(format!("{}", Region::Square), "square");
        assert_eq!(format!("{}", Region::Rect(10, 20, 30, 40)), "10,20,30,40");
        assert_eq!(
            format!("{}", Region::Pct(10.0, 20.0, 30.0, 40.0)),
            "pct:10,20,30,40"
        );
        assert_eq!(
            format!("{}", Region::Pct(41.6, 7.5, 40.0, 70.0)),
            "pct:41.6,7.5,40,70"
        );
        let a: Region = "pct:41.6,7.5,40,70".parse().unwrap();
        assert_eq!(a, Region::Pct(41.6, 7.5, 40.0, 70.0));
    }

    #[test]
    fn test_region_get_region() {
        let width = 300;
        let height = 200;
        let region1 = Region::Full;
        let (x, y, w, h) = region1.get_region(width, height).unwrap();
        assert_eq!((x, y, w, h), (0, 0, width, height));

        let region2 = Region::Square;
        let (x, y, w, h) = region2.get_region(width, height).unwrap();
        assert_eq!((x, y, w, h), (50, 0, 200, 200));

        let region3 = Region::Rect(125, 15, 120, 140);
        let (x, y, w, h) = region3.get_region(width, height).unwrap();
        assert_eq!((x, y, w, h), (125, 15, 120, 140));

        let region4 = Region::Pct(41.6, 7.5, 40.0, 70.0);
        let (x, y, w, h) = region4.get_region(width, height).unwrap();
        assert_eq!((x, y, w, h), (125, 15, 120, 140));

        let region5 = Region::Rect(125, 15, 200, 200);
        let (x, y, w, h) = region5.get_region(width, height).unwrap();
        assert_eq!((x, y, w, h), (125, 15, 175, 185));

        let region6 = Region::Pct(41.6, 7.5, 66.6, 100.0);
        let (x, y, w, h) = region6.get_region(width, height).unwrap();
        assert_eq!((x, y, w, h), (125, 15, 175, 185));
    }
}
