# i3f

![GitHub License](https://img.shields.io/github/license/jefferywang/iiif?style=for-the-badge&logo=github)
[![Codecov](https://img.shields.io/codecov/c/github/jefferywang/iiif?style=for-the-badge&logo=codecov)](https://codecov.io/gh/jefferywang/iiif)
[![Coveralls](https://img.shields.io/coverallsCoverage/github/jefferywang/iiif?style=for-the-badge&logo=coveralls)](https://coveralls.io/github/jefferywang/iiif)
[![docs.rs](https://img.shields.io/docsrs/i3f?style=for-the-badge&logo=docsdotrs)](https://docs.rs/i3f/latest/i3f/)
[![Crates.io](https://img.shields.io/crates/v/i3f?style=for-the-badge&logo=rust)](https://crates.io/crates/i3f)
![Crates.io Size](https://img.shields.io/crates/size/i3f?style=for-the-badge)
![Crates.io Total Downloads](https://img.shields.io/crates/d/i3f?style=for-the-badge)
![Deps.rs Crate Dependencies (latest)](https://img.shields.io/deps-rs/i3f/latest?style=for-the-badge)

[English](#english) | [中文](#中文)

---

## 中文

### 简介

`i3f` 是一个用 Rust 实现的 IIIF (International Image Interoperability Framework) API 库，支持 IIIF Image API 3.0 和 Presentation API 3.0 规范。

### 功能特性

- ✅ **IIIF Image API 3.0** 完整支持

  - 图像区域裁剪（Region）：支持 `full`、`square`、像素坐标和百分比坐标
  - 图像尺寸调整（Size）：支持多种尺寸参数格式
  - 图像旋转（Rotation）：支持 90 度倍数旋转和任意角度旋转
  - 图像质量（Quality）：支持 `default`、`color`、`gray`、`bitonal`
  - 图像格式（Format）：支持 `jpg`、`png`、`gif`、`webp`、`tif`、`jp2`、`pdf`
  - 图像信息（Info）：完整的 `info.json` 结构支持

- ✅ **IIIF Presentation API 3.0** 完整支持

  - Collection（集合）
  - Manifest（清单）
  - Canvas（画布）
  - Range（范围）
  - Annotation（注解）
  - AnnotationPage（注解页）

- ✅ **图像处理能力**

  - 支持任意角度旋转（使用 `imageproc` 库）
  - 90 度倍数旋转优化（使用 `image` 库内置方法）
  - 透明背景填充
  - 自动边界框计算

- ✅ **存储抽象**
  - 本地文件系统存储（LocalStorage）
  - 可扩展的存储接口

### 安装

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
i3f = "*"
```

### 快速开始

#### 解析 IIIF Image URL

```rust
use i3f::image::IiifImage;
use url::Url;

let url = Url::parse("https://example.org/image-service/demo.jpg/full/max/0/default.jpg")?;
let image = IiifImage::try_from(url)?;

println!("Identifier: {}", image.identifier);
println!("Region: {}", image.region);
println!("Size: {}", image.size);
println!("Rotation: {}", image.rotation);
println!("Quality: {}", image.quality);
println!("Format: {}", image.format);
```

#### 处理图像

```rust
use i3f::image::{IiifImage, LocalStorage};
use url::Url;

let url = Url::parse("https://example.org/image-service/demo.jpg/full/max/45/default.jpg")?;
let image = IiifImage::try_from(url)?;

let storage = LocalStorage::new("./fixtures");
let processed_image = image.process(&storage)?;

// 保存处理后的图像
processed_image.save("./output/result.jpg")?;
```

#### 创建 Presentation Manifest

```rust
use i3f::presentation::{Manifest, Canvas, LangMap};
use std::collections::HashMap;

let mut label = LangMap::new();
label.insert("en".to_string(), vec!["My Manifest".to_string()]);
label.insert("zh".to_string(), vec!["我的清单".to_string()]);

let manifest = Manifest {
    context: i3f::presentation::Context::presentation_default(),
    id: "https://example.org/manifest.json".to_string(),
    r#type: "Manifest".to_string(),
    label,
    items: vec![],
    ..Default::default()
};

// 序列化为 JSON
let json = serde_json::to_string_pretty(&manifest)?;
println!("{}", json);
```

### 模块说明

- **`image`**: IIIF Image API 3.0 实现

  - `IiifImage`: 图像请求解析和处理
  - `Region`: 区域裁剪
  - `Size`: 尺寸调整
  - `Rotation`: 旋转处理
  - `Quality`: 质量设置
  - `Format`: 格式转换
  - `ImageInfo`: 图像信息结构

- **`presentation`**: IIIF Presentation API 3.0 实现

  - `Collection`: 集合结构
  - `Manifest`: 清单结构
  - `Canvas`: 画布结构
  - `Range`: 范围结构
  - `Annotation`: 注解结构
  - `AnnotationPage`: 注解页结构

- **`storage`**: 存储抽象

  - `Storage`: 存储接口
  - `LocalStorage`: 本地文件系统存储实现

- **`error`**: 错误类型
  - `IiifError`: IIIF 相关错误枚举

### 文档

- [IIIF Image API 3.0 规范](https://iiif.io/api/image/3.0/)
- [IIIF Presentation API 3.0 规范](https://iiif.io/api/presentation/3.0/)
- [API 文档](https://docs.rs/i3f)

### 许可证

MIT License

### 贡献

欢迎提交 Issue 和 Pull Request！

---

## English

### Introduction

`i3f` is a Rust implementation of the IIIF (International Image Interoperability Framework) API library, supporting IIIF Image API 3.0 and Presentation API 3.0 specifications.

### Features

- ✅ **Full IIIF Image API 3.0 Support**

  - Image Region: Supports `full`, `square`, pixel coordinates, and percentage coordinates
  - Image Size: Supports multiple size parameter formats
  - Image Rotation: Supports 90-degree multiples and arbitrary angle rotation
  - Image Quality: Supports `default`, `color`, `gray`, `bitonal`
  - Image Format: Supports `jpg`, `png`, `gif`, `webp`, `tif`, `jp2`, `pdf`
  - Image Info: Complete `info.json` structure support

- ✅ **Full IIIF Presentation API 3.0 Support**

  - Collection
  - Manifest
  - Canvas
  - Range
  - Annotation
  - AnnotationPage

- ✅ **Image Processing Capabilities**

  - Arbitrary angle rotation (using `imageproc` library)
  - Optimized 90-degree multiple rotation (using `image` library built-in methods)
  - Transparent background filling
  - Automatic bounding box calculation

- ✅ **Storage Abstraction**
  - Local file system storage (LocalStorage)
  - Extensible storage interface

### Installation

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
i3f = "*"
```

### Quick Start

#### Parse IIIF Image URL

```rust
use i3f::image::IiifImage;
use url::Url;

let url = Url::parse("https://example.org/image-service/demo.jpg/full/max/0/default.jpg")?;
let image = IiifImage::try_from(url)?;

println!("Identifier: {}", image.identifier);
println!("Region: {}", image.region);
println!("Size: {}", image.size);
println!("Rotation: {}", image.rotation);
println!("Quality: {}", image.quality);
println!("Format: {}", image.format);
```

#### Process Image

```rust
use i3f::image::{IiifImage, LocalStorage};
use url::Url;

let url = Url::parse("https://example.org/image-service/demo.jpg/full/max/45/default.jpg")?;
let image = IiifImage::try_from(url)?;

let storage = LocalStorage::new("./fixtures");
let processed_image = image.process(&storage)?;

// Save the processed image
processed_image.save("./output/result.jpg")?;
```

#### Create Presentation Manifest

```rust
use i3f::presentation::{Manifest, LangMap};
use std::collections::HashMap;

let mut label = LangMap::new();
label.insert("en".to_string(), vec!["My Manifest".to_string()]);
label.insert("zh".to_string(), vec!["我的清单".to_string()]);

let manifest = Manifest {
    context: i3f::presentation::Context::presentation_default(),
    id: "https://example.org/manifest.json".to_string(),
    r#type: "Manifest".to_string(),
    label,
    items: vec![],
    ..Default::default()
};

// Serialize to JSON
let json = serde_json::to_string_pretty(&manifest)?;
println!("{}", json);
```

### Modules

- **`image`**: IIIF Image API 3.0 implementation

  - `IiifImage`: Image request parsing and processing
  - `Region`: Region cropping
  - `Size`: Size adjustment
  - `Rotation`: Rotation processing
  - `Quality`: Quality settings
  - `Format`: Format conversion
  - `ImageInfo`: Image information structure

- **`presentation`**: IIIF Presentation API 3.0 implementation

  - `Collection`: Collection structure
  - `Manifest`: Manifest structure
  - `Canvas`: Canvas structure
  - `Range`: Range structure
  - `Annotation`: Annotation structure
  - `AnnotationPage`: Annotation page structure

- **`storage`**: Storage abstraction

  - `Storage`: Storage interface
  - `LocalStorage`: Local file system storage implementation

- **`error`**: Error types
  - `IiifError`: IIIF-related error enumeration

### Documentation

- [IIIF Image API 3.0 Specification](https://iiif.io/api/image/3.0/)
- [IIIF Presentation API 3.0 Specification](https://iiif.io/api/presentation/3.0/)
- [API Documentation](https://docs.rs/i3f)

### License

MIT License

### Contributing

Issues and Pull Requests are welcome!
