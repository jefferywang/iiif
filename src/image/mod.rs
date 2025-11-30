//! IIIF Image API 3.0 规范的实现(IIIF Image API 3.0 Implementation. )
//!
//! [官方文档(Official Documentation)](https://iiif.io/api/image/3.0/)
//!
mod format;
mod info;
mod quality;
mod region;
mod result;
mod rotation;
mod size;

use std::{fmt::Display, str::FromStr};

pub use format::*;
pub use info::*;
pub use quality::*;
pub use region::*;
pub use result::*;
pub use rotation::*;
pub use size::*;
use url::Url;

use crate::storage::Storage;

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
            .unwrap_or_default();
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
    let decoded = urlencoding::decode(value)
        .map_err(|_| crate::IiifError::BadRequest(format!("Invalid identifier: {value}")))?;
    Ok(decoded.to_string())
}

fn url_encode(value: &str) -> String {
    let encoded = urlencoding::encode(value);
    encoded.to_string()
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
                "Invalid quality.format segment: {segment}",
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
            crate::IiifError::InvalidIiifURL(format!("Invalid {param_name} format: {value}"))
        })
    }

    /// 对图片进行处理
    ///
    /// Returns the processed image data as a vector of bytes.
    ///
    /// Example:
    /// ```
    /// use i3f::image::IiifImage;
    /// use std::str::FromStr;
    /// use url::Url;
    /// use i3f::storage::Storage;
    /// use i3f::storage::LocalStorage;
    ///
    /// let url = Url::parse("https://example.org/image-service/demo.jpg/full/max/0/default.jpg").unwrap();
    /// let image = IiifImage::try_from(url).unwrap();
    /// let storage = LocalStorage::new("./fixtures", "./fixtures/out");
    /// let image_data = image.process(&storage).unwrap();
    /// ```
    pub fn process(&self, storage: &dyn Storage) -> Result<ProcessResult, crate::IiifError> {
        // 如果 iiif 文件存在，则直接返回
        if let Ok(iiif_file) = storage.get_iiif_file(self) {
            return Ok(iiif_file);
        }

        // 获取原始文件
        let origin_file = storage
            .get_origin_file(&self.identifier)
            .map_err(crate::IiifError::InternalServerError)?;
        let image = image::load_from_memory(&origin_file)
            .map_err(|e| crate::IiifError::InternalServerError(e.to_string()))?;
        // 处理 region 数据
        let image = self.region.process(image)?;
        // 处理 size 数据
        let image = self.size.process(image)?;
        // 处理 rotation 数据
        let image = self.rotation.process(image)?;
        let image = self.quality.process(image)?;
        let result = self.format.process(image)?;
        let content_type = self.format.get_content_type();

        // 保存 iiif 文件
        storage
            .save_iiif_file(self, &result)
            .map_err(crate::IiifError::InternalServerError)?;

        // 返回结果
        Ok(ProcessResult::new(content_type.to_string(), result))
    }
}

impl Display for IiifImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/{}/{}/{}/{}.{}",
            url_encode(&self.identifier),
            self.region,
            self.size,
            self.rotation,
            self.quality,
            self.format
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::storage::LocalStorage;

    use super::*;

    #[test]
    fn test_url_error() {
        let url = Url::parse("https://example.org/image-service/demo.jpg/full").unwrap();
        let result = IiifImage::try_from(url);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            crate::IiifError::InvalidIiifURL("URL does not have enough segments".to_string())
        );

        let url =
            Url::parse("https://example.org/image-service/demo.jpg/full/max/0/default").unwrap();
        let result = IiifImage::try_from(url);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            crate::IiifError::InvalidIiifURL("Invalid quality.format segment: default".to_string())
        );

        let url = Url::parse("https://example.org/image-service//full/max/0/default.jpg").unwrap();
        let result = IiifImage::try_from(url);
        assert!(result.is_err());
        assert_eq!(
            result.err().unwrap(),
            crate::IiifError::InvalidIiifURL("Identifier cannot be empty".to_string())
        );
    }

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

        let url_data2 = Url::parse(
            "https://example.org/image-service/data%2Faaa.jpg/full1/max/0/default.jpg?a=1",
        )
        .unwrap();
        let result = IiifImage::try_from(url_data2);
        assert!(result.is_err());
    }

    #[test]
    fn test_url_parse() {
        let cases = vec![
            (
                "id1/full/max/0/default.jpg",
                "id1",
                Region::Full,
                Size::Max,
                Rotation::Degrees(0.0),
                Quality::Default,
                Format::Jpg,
            ),
            (
                "id1/0,10,100,200/pct:50/90/default.png",
                "id1",
                Region::Rect(0, 10, 100, 200),
                Size::Pct { n: 50.0 },
                Rotation::Degrees(90.0),
                Quality::Default,
                Format::Png,
            ),
            (
                "id1/pct:10,10,80,80/50,/22.5/color.jpg",
                "id1",
                Region::Pct(10.0, 10.0, 80.0, 80.0),
                Size::W { w: 50 },
                Rotation::Degrees(22.5),
                Quality::Color,
                Format::Jpg,
            ),
            (
                "bb157hs6068/full/max/270/gray.jpg",
                "bb157hs6068",
                Region::Full,
                Size::Max,
                Rotation::Degrees(270.0),
                Quality::Gray,
                Format::Jpg,
            ),
            (
                "ark:%2F12025%2F654xz321/full/max/0/default.jpg",
                "ark:/12025/654xz321",
                Region::Full,
                Size::Max,
                Rotation::Degrees(0.0),
                Quality::Default,
                Format::Jpg,
            ),
            (
                "urn:foo:a123,456/full/max/0/default.jpg",
                "urn:foo:a123,456",
                Region::Full,
                Size::Max,
                Rotation::Degrees(0.0),
                Quality::Default,
                Format::Jpg,
            ),
            (
                "urn:sici:1046-8188(199501)13:1%253C69:FTTHBI%253E2.0.TX;2-4/full/max/0/default.jpg",
                "urn:sici:1046-8188(199501)13:1%3C69:FTTHBI%3E2.0.TX;2-4",
                Region::Full,
                Size::Max,
                Rotation::Degrees(0.0),
                Quality::Default,
                Format::Jpg,
            ),
            (
                "http:%2F%2Fexample.com%2F%3F54%23a/full/max/0/default.jpg",
                "http://example.com/?54#a",
                Region::Full,
                Size::Max,
                Rotation::Degrees(0.0),
                Quality::Default,
                Format::Jpg,
            ),
        ];
        for case in cases {
            let url = format!("https://example.org/{}", case.0);
            let url_data = Url::parse(&url).unwrap();
            let iiif_image = IiifImage::try_from(url_data).unwrap();
            assert_eq!(iiif_image.identifier, case.1);
            assert_eq!(iiif_image.region, case.2);
            assert_eq!(iiif_image.size, case.3);
            assert_eq!(iiif_image.rotation, case.4);
            assert_eq!(iiif_image.quality, case.5);
            assert_eq!(iiif_image.format, case.6);
        }
    }

    #[test]
    fn test_process() {
        let storage = LocalStorage::new("./fixtures", "./fixtures/out");
        let cases = vec![
            ("/square/150,/15/color.png", "image/png", 184, 184),
            ("/full/max/0/default.jpg", "image/jpeg", 300, 200),
        ];
        for case in cases {
            let url_str = format!("https://example.org/image-service/demo.jpg{}", case.0);
            let url_data = Url::parse(&url_str).unwrap();
            let image = IiifImage::try_from(url_data).unwrap();
            let result = image.process(&storage).unwrap();
            assert_eq!(result.content_type, case.1);

            // 将 vec<u8> 转换为 image::DynamicImage
            let image = image::load_from_memory(&result.data).unwrap();
            assert_eq!(image.width(), case.2);
            assert_eq!(image.height(), case.3);

            // remove the out directory
            if case.0.contains("square") {
                std::fs::remove_dir_all("./fixtures/out/demo.jpg/square/").unwrap();
            }
        }
    }

    #[test]
    fn test_iiif_image() {
        let url = Url::parse("https://example.org/image-service/demo.jpg/full/max/0/default.jpg")
            .unwrap();
        let image = IiifImage::try_from(url).unwrap();
        assert_eq!(image.to_string(), "demo.jpg/full/max/0/default.jpg");
    }
}
