use std::{fmt::Display, str::FromStr};

use image::DynamicImage;
use image::ImageEncoder;
use image::codecs::gif::GifEncoder;
use image::codecs::jpeg::JpegEncoder;
use image::codecs::png::PngEncoder;
use image::codecs::tiff::TiffEncoder;
use image::codecs::webp::WebPEncoder;
use lopdf::{Document, Object, Stream, dictionary};
use std::io::Cursor;

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

impl Format {
    pub fn get_content_type(&self) -> &str {
        match self {
            Self::Jpg => "image/jpeg",
            Self::Png => "image/png",
            Self::Gif => "image/gif",
            Self::Webp => "image/webp",
            Self::Tif => "image/tiff",
            Self::Jp2 => "image/jp2",
            Self::Pdf => "application/pdf",
        }
    }

    pub fn process(&self, image: DynamicImage) -> Result<Vec<u8>, IiifError> {
        let mut bytes = Vec::new();

        match self {
            Format::Jpg => {
                let rgb = image.to_rgb8();
                let mut cursor = Cursor::new(&mut bytes);
                let encoder = JpegEncoder::new(&mut cursor);
                encoder
                    .write_image(
                        rgb.as_raw(),
                        rgb.width(),
                        rgb.height(),
                        image::ExtendedColorType::Rgb8,
                    )
                    .map_err(|e| IiifError::ImageEncodeFailed(e.to_string()))?;
            }
            Format::Png => {
                let rgba = image.to_rgba8();
                let mut cursor = Cursor::new(&mut bytes);
                let encoder = PngEncoder::new(&mut cursor);
                encoder
                    .write_image(
                        rgba.as_raw(),
                        rgba.width(),
                        rgba.height(),
                        image::ExtendedColorType::Rgba8,
                    )
                    .map_err(|e| IiifError::ImageEncodeFailed(e.to_string()))?;
            }
            Format::Webp => {
                let rgba = image.to_rgba8();
                let mut cursor = Cursor::new(&mut bytes);
                let encoder = WebPEncoder::new_lossless(&mut cursor);
                encoder
                    .write_image(
                        rgba.as_raw(),
                        rgba.width(),
                        rgba.height(),
                        image::ExtendedColorType::Rgba8,
                    )
                    .map_err(|e| IiifError::ImageEncodeFailed(e.to_string()))?;
            }
            Format::Gif => {
                let rgba = image.to_rgba8();
                let mut cursor = Cursor::new(&mut bytes);
                let encoder = GifEncoder::new(&mut cursor);
                encoder
                    .write_image(
                        rgba.as_raw(),
                        rgba.width(),
                        rgba.height(),
                        image::ExtendedColorType::Rgba8,
                    )
                    .map_err(|e| IiifError::ImageEncodeFailed(e.to_string()))?;
            }
            Format::Tif => {
                let rgba = image.to_rgba8();
                let mut cursor = Cursor::new(&mut bytes);
                let encoder = TiffEncoder::new(&mut cursor);
                encoder
                    .write_image(
                        rgba.as_raw(),
                        rgba.width(),
                        rgba.height(),
                        image::ExtendedColorType::Rgba8,
                    )
                    .map_err(|e| IiifError::ImageEncodeFailed(e.to_string()))?;
            }
            Format::Jp2 => {
                return Err(IiifError::ImageEncodeFailed(
                    "JPEG 2000 encoding not yet implemented".to_string(),
                ));
            }
            Format::Pdf => {
                // 将图像转换为 JPEG 格式（PDF 中 JPEG 更小）
                let rgb = image.to_rgb8();
                let mut jpeg_data = Vec::new();
                {
                    let mut jpeg_cursor = Cursor::new(&mut jpeg_data);
                    let encoder = JpegEncoder::new(&mut jpeg_cursor);
                    encoder
                        .write_image(
                            rgb.as_raw(),
                            rgb.width(),
                            rgb.height(),
                            image::ExtendedColorType::Rgb8,
                        )
                        .map_err(|e| IiifError::ImageEncodeFailed(e.to_string()))?;
                }

                // 创建 PDF 文档
                let mut doc = Document::with_version("1.5");

                // 创建图像字典
                let width = rgb.width() as f64;
                let height = rgb.height() as f64;

                // 创建图像 XObject
                let image_dict = dictionary! {
                    "Type" => "XObject",
                    "Subtype" => "Image",
                    "Width" => rgb.width() as i64,
                    "Height" => rgb.height() as i64,
                    "ColorSpace" => "DeviceRGB",
                    "BitsPerComponent" => 8,
                    "Filter" => "DCTDecode", // JPEG 使用 DCTDecode
                };

                let image_stream = Stream::new(image_dict, jpeg_data);
                let image_id = doc.add_object(image_stream);

                // 创建页面内容流
                // q: 保存图形状态, cm: 变换矩阵, Do: 绘制XObject, Q: 恢复图形状态
                let content = format!("q\n{} 0 0 {} 0 0 cm\n/Im1 Do\nQ", width, height);
                let content_stream = Stream::new(dictionary! {}, content.into_bytes());
                let content_id = doc.add_object(content_stream);

                // 先创建页面树（空），获取其 ID
                let pages_id = doc.new_object_id();
                let pages = dictionary! {
                    "Type" => "Pages",
                    "Kids" => vec![],
                    "Count" => 0,
                };
                doc.objects.insert(pages_id, Object::Dictionary(pages));

                // 创建页面对象，并设置父引用
                let page = dictionary! {
                    "Type" => "Page",
                    "Parent" => Object::Reference(pages_id),
                    "MediaBox" => vec![0.into(), 0.into(), width.into(), height.into()],
                    "Resources" => dictionary! {
                        "XObject" => dictionary! {
                            "Im1" => image_id,
                        },
                    },
                    "Contents" => content_id,
                };
                let page_id = doc.add_object(page);

                // 更新页面树，添加页面引用并更新计数
                if let Ok(pages_dict) = doc.get_dictionary_mut(pages_id) {
                    if let Ok(kids) = pages_dict.get_mut(b"Kids") {
                        if let Ok(kids_array) = kids.as_array_mut() {
                            kids_array.push(Object::Reference(page_id));
                        } else {
                            // 如果 Kids 不存在，创建它
                            pages_dict.set("Kids", vec![Object::Reference(page_id)]);
                        }
                    } else {
                        pages_dict.set("Kids", vec![Object::Reference(page_id)]);
                    }
                    pages_dict.set("Count", 1);
                }

                // 创建目录
                let catalog = dictionary! {
                    "Type" => "Catalog",
                    "Pages" => Object::Reference(pages_id),
                };
                let catalog_id = doc.add_object(catalog);

                // 设置文档根和 trailer
                doc.trailer.set("Root", Object::Reference(catalog_id));
                doc.trailer.set("Size", (doc.objects.len() + 1) as i64);

                // 将文档写入字节流
                doc.save_to(&mut bytes)
                    .map_err(|e| IiifError::ImageEncodeFailed(e.to_string()))?;
            }
        }

        Ok(bytes)
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
