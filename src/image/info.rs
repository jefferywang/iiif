use serde::{Deserialize, Serialize};

use crate::Format;

const IIIF_IMAGE_3_CONTEXT: &str = "http://iiif.io/api/image/3/context.json";

/// ImageInfo 定义了 IIIF 图像的基本信息
///
/// Several technical properties
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ImageInfo {
    /// `@context` 属性应作为 JSON 表示的第一个键值对出现。它的值必须是 URI `http://iiif.io/api/image/3/context.json`
    /// 或以 URI `http://iiif.io/api/image/3/context.json` 为最后一项的 JSON 数组。`@context` 告诉链接数据处理器如何
    /// 解读图像信息。如果使用扩展，则其上下文定义应包含在这个顶层 `@context` 属性中。
    ///
    /// The @context property should appear as the very first key-value pair of the JSON representation.
    /// Its value must be either the URI `http://iiif.io/api/image/3/context.json` or a JSON array with
    /// the URI `http://iiif.io/api/image/3/context.json` as the last item. The @context tells Linked
    /// Data processors how to interpret the image information. If extensions are used then their
    /// context definitions should be included in this top-level @context property.
    #[serde(rename = "@context", default = "Context::default")]
    pub context: Context,

    /// 图像的基础 URI 在 [URI 语法](https://iiif.io/api/image/3.0/#2-uri-syntax)中定义，包括方案、服务器、前缀和标识符，无尾斜杠。
    ///
    /// The base URI of the image as defined in [URI Syntax](https://iiif.io/api/image/3.0/#2-uri-syntax),
    /// including scheme, server, prefix and identifier without a trailing slash.
    pub id: String,

    /// Image API 的类型。该值必须是字符串 `ImageService3`。
    ///
    /// The type for the Image API. The value must be the string `ImageService3`.
    #[serde(default = "InfoType::default")]
    pub r#type: InfoType,

    /// URI `http://iiif.io/api/image`，可用于确定该文档描述的是一个图像服务，该服务是 IIIF 图像 API 的一个版本。
    ///
    /// The URI `http://iiif.io/api/image` which can be used to determine that the document describes
    /// an image service which is a version of the IIIF Image API.
    #[serde(default = "Protocol::default")]
    pub protocol: Protocol,

    /// 字符串表示服务完全支持的最高[合规等级](https://iiif.io/api/image/3.0/#6-compliance-level-and-profile-document) 。
    /// 该值必须是 `level0`、`level1` 或 `level2` 之一。
    ///
    /// A string indicating the highest [compliance level](https://iiif.io/api/image/3.0/#6-compliance-level-and-profile-document)
    /// which is fully supported by the service. The value must be one of `level0`, `level1`, or `level2`.
    #[serde(default = "Profile::default")]
    pub profile: Profile,

    /// 图像的宽度，以像素为单位。
    ///
    /// The width of the image, in pixels.
    pub width: u32,

    /// 图像的高度，以像素为单位。
    ///
    /// The height of the image, in pixels.
    pub height: u32,

    /// 该图像支持的最大像素宽度。客户端不应期望支持宽度大于此值的请求。如果指定了 `maxHeight`， 则必须指定 `maxWidth`。
    ///
    /// The maximum width in pixels supported for this image. Clients must not expect requests with a width
    /// greater than this value to be supported. `maxWidth` must be specified if `maxHeight` is specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_width: Option<u32>,

    /// 该图像支持的最大像素高度。客户端不应期望高度超过此值的请求会被支持。如果指定了 `maxWidth`，而未指定 `maxHeight`，
    /// 那么客户端应推断 `maxHeight = maxWidth`。
    ///
    /// The maximum height in pixels supported for this image. Clients must not expect requests with
    /// a height greater than this value to be supported. If `maxWidth` is specified and `maxHeight` is not,
    /// then clients should infer that `maxHeight = maxWidth`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_height: Option<u32>,

    /// 该图像支持的最大像素面积。客户端不应期望`width * height`大于此值的请求会被支持。
    ///
    /// The maximum area in pixels supported for this image. Clients must not
    /// expect requests with a `width * height` greater than this value to be supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_area: Option<u32>,

    /// 用于描述完整图像表示时的首选`width`和`height`的组合列表。对于不支持任意大小请求的服务器，这些可能是唯一可用的大小。
    /// 用这些大小的 `w,h` 语法构建的请求必须由服务器支持，即使任意宽度和高度不支持。
    ///
    /// An array of JSON objects with the height and width properties. These sizes specify preferred values
    /// to be provided in the w,h syntax of the size request parameter for scaled versions of the full image.
    /// In the case of servers that do not support requests for arbitrary sizes, these may be the only sizes
    /// available. A request constructed with the w,h syntax using these sizes must be supported by the server,
    /// even if arbitrary width and height are not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sizes: Option<Vec<SizeInfo>>,

    /// 支持的格式列表
    ///
    /// extra formats supported by the service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_formats: Option<Vec<Format>>,

    /// 首选格式参数值，按偏好顺序排列。所列格式参数值必须在参考配置文件中指定的值或 `extraFormats` 属性中列出。
    /// (参见[额外功能](https://iiif.io/api/image/3.0/#57-extra-functionality))
    ///
    /// An array of strings that are the preferred format parameter values, arranged in order of preference.
    /// The format parameter values listed must be among those specified in the referenced profile or listed
    /// in the extraFormats property (see [Extra Functionality](https://iiif.io/api/image/3.0/#57-extra-functionality)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_formats: Option<Vec<Format>>,

    /// 适用于该图片内容的许可或权利声明。该属性的价值必须是从[知识共享](https://creativecommons.org/licenses/)许可 URI 集合、
    /// [RightsStatements.org](http://rightsstatements.org/page/1.0/) 权利声明 URI 或通过[已知扩展](https://iiif.io/api/registry/)
    /// 注册机制添加的字符串中提取。该属性的加入具有信息价值，例如可以用来展示代表权利主张的图标。
    ///
    /// A license or rights statement applicable to the content of the image. The value of this property must be a string drawn
    /// from the set of [Creative Commons](https://creativecommons.org/licenses/) license URIs, the
    /// [RightsStatements.org](http://rightsstatements.org/page/1.0/) rights statement URIs,
    /// or those added via the [Registry of Known Extensions](https://iiif.io/api/registry/) mechanism.
    /// The inclusion of this property is informative, and for example could be used to display an
    /// icon representing the rights assertions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rights: Option<String>,
}

/// `@context` 属性应作为 JSON 表示的第一个键值对出现。它的值必须是 URI `http://iiif.io/api/image/3/context.json`
/// 或以 URI `http://iiif.io/api/image/3/context.json` 为最后一项的 JSON 数组。`@context` 告诉链接数据处理器如何
/// 解读图像信息。如果使用扩展，则其上下文定义应包含在这个顶层 `@context` 属性中。
///
/// The @context property should appear as the very first key-value pair of the JSON representation.
/// Its value must be either the URI `http://iiif.io/api/image/3/context.json` or a JSON array with
/// the URI `http://iiif.io/api/image/3/context.json` as the last item. The @context tells Linked
/// Data processors how to interpret the image information. If extensions are used then their
/// context definitions should be included in this top-level @context property.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Context {
    // 单个 context
    Single(String),

    // 多个 context
    List(Vec<String>),
}

impl Default for Context {
    fn default() -> Self {
        Self::Single(IIIF_IMAGE_3_CONTEXT.to_string())
    }
}

impl Context {
    pub fn new_single() -> Self {
        Self::default()
    }

    pub fn new_list(contexts: &[String]) -> Self {
        // 如果包含 IIIF_IMAGE_3_CONTEXT，则删除，并放到最后面，以符合规范要求
        let mut contexts = contexts.to_vec();
        if let Some(index) = contexts.iter().position(|c| c == IIIF_IMAGE_3_CONTEXT) {
            contexts.remove(index);
        }
        contexts.push(IIIF_IMAGE_3_CONTEXT.to_string());
        Self::List(contexts)
    }
}

/// Image API 的类型。该值必须是字符串 `ImageService3`。
///
/// The type for the Image API. The value must be the string `ImageService3`.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct InfoType(String);

impl Default for InfoType {
    fn default() -> Self {
        Self("ImageService3".to_string())
    }
}

/// 协议，URI `http://iiif.io/api/image`，可用于确定该文档描述的是一个图像服务，该服务是 IIIF 图像 API 的一个版本。
///
/// The protocol, URI `http://iiif.io/api/image`, can be used to determine that the document describes
/// an image service which is a version of the IIIF Image API.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Protocol(String);

impl Default for Protocol {
    fn default() -> Self {
        Self("http://iiif.io/api/image".to_string())
    }
}

/// 服务完全支持的最高[合规等级](https://iiif.io/api/image/3.0/#6-compliance-level-and-profile-document)，
/// 该值必须是 `level0`、`level1` 或 `level2` 之一。
///
/// A string indicating the highest compliance level which is fully supported by the service.
/// The value must be one of `level0`, `level1`, or `level2`.
#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Profile {
    #[default]
    Level0,
    Level1,
    Level2,
}

/// 尺寸项，表示图像的宽度和高度。
///
/// A size item, representing the width and height of the image.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct SizeInfo {
    /// 对象类型，如果存在，值必须为字符串 `Size`。
    ///
    /// The object type, if present, must be the string `Size`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<SizeType>,

    /// 图像的宽度，以像素为单位。
    ///
    /// The width of the image, in pixels.
    pub width: u32,

    /// 图像的高度，以像素为单位。
    ///
    /// The height of the image, in pixels.
    pub height: u32,
}

/// 尺寸类型，如果存在，值必须为字符串 `size`。
///
/// The object type, if present, must be the string `size`.
#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub enum SizeType {
    #[default]
    #[serde(rename = "Size")]
    Size,
}

/// 瓦片项，表示图像的瓦片信息。
///
/// A tile item, representing the tile information of the image.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TileInfo {
    /// 对象类型，如果存在，值必须为字符串 `Tile`。
    ///
    /// The object type, if present, must be the string `Tile`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TileType>,

    /// 图像预定义图块的分辨率缩放因子集合，用正整数表示，用于整除图像的全尺寸。
    /// 例如，比例因子为 4 表示该服务能够高效地传输图像，图像比例为整幅图像的
    /// 1/4 或 25%。某个比例因子值应仅在`tiles`数组中出现一次。
    ///
    /// The set of resolution scaling factors for the image’s predefined tiles, expressed
    /// as positive integers by which to divide the full size of the image. For example,
    /// a scale factor of 4 indicates that the service can efficiently deliver images at
    /// 1/4 or 25% of the height and width of the full image. A particular scale factor
    /// value should appear only once in the `tiles` array.
    pub scale_factors: Vec<u8>,

    /// 预定义瓦片的像素宽度，以整数表示。
    ///
    /// The width in pixels of the predefined tiles to be requested, given as an integer.
    pub width: u32,

    /// 预定义图块的像素高度，以整数形式表示。如果 JSON 中没有指定，那么它默认与宽度相同，导致方形瓦片。
    ///
    /// The height in pixels of the predefined tiles to be requested, given as an integer.
    /// If it is not specified in the JSON, then it defaults to the same as width, resulting
    /// in square tiles.
    pub height: Option<u32>,
}

/// 瓦片类型，如果存在，值必须为字符串 `Tile`。
///
/// The object type, if present, must be the string `Tile`.
#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "PascalCase")]
pub enum TileType {
    #[default]
    Tile,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_default() {
        let mut info = ImageInfo::default();
        println!("{}", serde_json::to_string(&info).unwrap());

        info.context = Context::new_list(&["http://iiif.io/api/image/2/context.json".to_string()]);
        println!("{}", serde_json::to_string(&info).unwrap());
    }
}
