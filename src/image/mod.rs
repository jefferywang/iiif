mod format;
mod quality;
mod region;
mod rotation;
mod size;
mod storage;

use std::str::FromStr;

pub use format::*;
pub use quality::*;
pub use region::*;
pub use rotation::*;
pub use size::*;
pub use storage::*;
use url::Url;

/// IiifImage 定义了 IIIF 图像的基本信息
#[derive(Debug)]
pub struct IiifImage {
    pub identifier: String,
    pub region: Region,
    pub size: Size,
    pub rotation: Rotation,
    pub quality: Quality,
    pub format: Format,
}

impl TryFrom<Url> for IiifImage {
    type Error = crate::IiifError;

    fn try_from(url: Url) -> Result<Self, Self::Error> {
        // 获取 url path 的最后四项
        let url_segs = url
            .path_segments()
            .map(|segments| segments.collect::<Vec<_>>())
            .unwrap_or_else(|| vec![]);
        if url_segs.len() < 5 {
            return Err(crate::IiifError::InvalidIiifURL(
                "URL does not have enough segments".to_string(),
            ));
        }
        let params = &url_segs[url_segs.len() - 5..];
        // 解析质量和格式（最后一段包含点分隔符）
        let (quality_str, format_str) = Self::parse_quality_format(params[4])?;

        Ok(IiifImage {
            identifier: Self::validate_identifier(url_decode(params[0])?.as_str())?,
            region: Self::parse_param(params[1], "region")?,
            size: Self::parse_param(params[2], "size")?,
            rotation: Self::parse_param(params[3], "rotation")?,
            quality: Self::parse_param(quality_str, "quality")?,
            format: Self::parse_param(format_str, "format")?,
        })
    }
}

fn url_decode(value: &str) -> Result<String, crate::IiifError> {
    let decoded = urlencoding::decode(value).map_err(|_| {
        crate::IiifError::InvalidIdentifier(format!("Invalid identifier: {}", value))
    })?;
    Ok(decoded.to_string())
}

impl IiifImage {
    /// 验证标识符
    fn validate_identifier(identifier: &str) -> Result<String, crate::IiifError> {
        if identifier.is_empty() {
            Err(crate::IiifError::InvalidIiifURL(
                "Identifier cannot be empty".to_string(),
            ))
        } else {
            Ok(identifier.to_string())
        }
    }

    /// 解析质量和格式
    fn parse_quality_format(segment: &str) -> Result<(&str, &str), crate::IiifError> {
        let parts: Vec<&str> = segment.split('.').collect();
        if parts.len() != 2 {
            return Err(crate::IiifError::InvalidIiifURL(format!(
                "Invalid quality.format segment: {}",
                segment
            )));
        }
        Ok((parts[0], parts[1]))
    }

    /// 通用参数解析
    fn parse_param<T: FromStr>(value: &str, param_name: &str) -> Result<T, crate::IiifError>
    where
        T::Err: std::fmt::Debug,
    {
        value.parse().map_err(|_| {
            crate::IiifError::InvalidIiifURL(format!("Invalid {} format: {}", param_name, value))
        })
    }

    /// 获取内容类型
    ///
    /// Returns the content type of the image based on the format.
    ///
    /// Example:
    /// ```
    /// use iiif::IiifImage;
    /// use std::str::FromStr;
    ///
    /// let image = IiifImage::try_from(url).unwrap();
    /// assert_eq!(image.get_content_type(), "image/jpeg");
    /// ```
    ///
    /// Returns:
    /// - "image/jpeg" for Format::Jpg
    /// - "image/png" for Format::Png
    /// - "image/gif" for Format::Gif
    /// - "image/webp" for Format::Webp
    /// - "image/tiff" for Format::Tif
    /// - "image/jp2" for Format::Jp2
    /// - "application/pdf" for Format::Pdf
    pub fn get_content_type(&self) -> &str {
        match self.format {
            Format::Jpg => "image/jpeg",
            Format::Png => "image/png",
            Format::Gif => "image/gif",
            Format::Webp => "image/webp",
            Format::Tif => "image/tiff",
            Format::Jp2 => "image/jp2",
            Format::Pdf => "application/pdf",
        }
    }

    /// 对图片进行处理
    ///
    /// Returns the processed image data as a vector of bytes.
    ///
    /// Example:
    /// ```
    /// use iiif::IiifImage;
    /// use std::str::FromStr;
    ///
    /// let image = IiifImage::try_from(url).unwrap();
    /// let image_data = image.process().unwrap();
    /// assert_eq!(image_data, vec![0x00, 0x00, 0x00, 0x00]);
    /// ```
    pub fn process(&self, storage: &dyn Storage) -> Result<Vec<u8>, crate::IiifError> {
        let local_path = storage.get_file_path(&self.identifier);
        let mut image = image::open(local_path)
            .map_err(|e| crate::IiifError::ImageOpenFailed(e.to_string()))?;
        let (x, y, w, h) = self.region.get_region(image.width(), image.height())?;
        let image = image.crop(x, y, w, h);
        println!("x: {}, y: {}, w: {}, h: {}", x, y, w, h);
        image.save("./output/cropped.jpg").unwrap();
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_image() {
        // {scheme}://{server}{/prefix}/{identifier}/{region}/{size}/{rotation}/{quality}.{format}
        let url_data =
            Url::parse("https://example.org/image-service/abcd1234/full/max/0/default.jpg")
                .unwrap();
        // 获取 url path 的最后四项
        let iiif_image = IiifImage::try_from(url_data).unwrap();
        assert_eq!(iiif_image.identifier, "abcd1234");
        assert_eq!(iiif_image.region, Region::Full);
        assert_eq!(iiif_image.size, Size::Max);
        assert_eq!(iiif_image.rotation, Rotation::Degrees(0.0));
        assert_eq!(iiif_image.quality, Quality::Default);
        assert_eq!(iiif_image.format, Format::Jpg);

        let url_data1 = Url::parse(
            "https://example.org/image-service/data%2Faaa.jpg/full/max/0/default.jpg?a=1",
        )
        .unwrap();
        let iiif_image1 = IiifImage::try_from(url_data1).unwrap();
        assert_eq!(iiif_image1.identifier, "data/aaa.jpg");
    }

    #[test]
    fn test_process_image() {
        let storage = LocalStorage::new("./fixtures");
        let url_data = Url::parse(
            "https://example.org/image-service/image3.jpg/125,15,200,200/max/0/default.jpg",
        )
        .unwrap();
        let iiif_image = IiifImage::try_from(url_data).unwrap();
        let _ = iiif_image.process(&storage).unwrap();
    }
}
