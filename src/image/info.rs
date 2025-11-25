use serde::{Deserialize, Serialize};

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
    /// Its value must be either the URI http://iiif.io/api/image/3/context.json or a JSON array with
    /// the URI http://iiif.io/api/image/3/context.json as the last item. The @context tells Linked
    /// Data processors how to interpret the image information. If extensions are used then their
    /// context definitions should be included in this top-level @context property.
    #[serde(rename = "@context", default = "Context::default")]
    context: Context,

    /// 图像的基础 URI 在 [URI 语法](https://iiif.io/api/image/3.0/#2-uri-syntax)中定义，包括方案、服务器、前缀和标识符，无尾斜杠。
    ///
    /// The base URI of the image as defined in [URI Syntax](https://iiif.io/api/image/3.0/#2-uri-syntax),
    /// including scheme, server, prefix and identifier without a trailing slash.
    id: String,

    /// Image API 的类型。该值必须是字符串 `ImageService3`。
    ///
    /// The type for the Image API. The value must be the string `ImageService3`.
    #[serde(default = "InfoType::default")]
    r#type: InfoType,

    /// URI `http://iiif.io/api/image`，可用于确定该文档描述的是一个图像服务，该服务是 IIIF 图像 API 的一个版本。
    ///
    /// The URI `http://iiif.io/api/image` which can be used to determine that the document describes
    /// an image service which is a version of the IIIF Image API.
    #[serde(default = "Protocol::default")]
    protocol: Protocol,

    /// 字符串表示服务完全支持的最高[合规等级](https://iiif.io/api/image/3.0/#6-compliance-level-and-profile-document) 。
    /// 该值必须是 `level0`、`level1` 或 `level2` 之一。
    ///
    /// A string indicating the highest [compliance level](https://iiif.io/api/image/3.0/#6-compliance-level-and-profile-document)
    /// which is fully supported by the service. The value must be one of `level0`, `level1`, or `level2`.
    #[serde(default = "Profile::default")]
    profile: Profile,

    /// 图像的宽度，以像素为单位。
    ///
    /// The width of the image, in pixels.
    width: u32,

    /// 图像的高度，以像素为单位。
    ///
    /// The height of the image, in pixels.
    height: u32,

    /// 该图像支持的最大像素宽度。客户端不应期望支持宽度大于此值的请求。如果指定了 `maxHeight`， 则必须指定 `maxWidth`。
    ///
    /// The maximum width in pixels supported for this image. Clients must not expect requests with a width
    /// greater than this value to be supported. `maxWidth` must be specified if `maxHeight` is specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    max_width: Option<u32>,

    /// 该图像支持的最大像素高度。客户端不应期望高度超过此值的请求会被支持。如果指定了 `maxWidth`，而未指定 `maxHeight`，
    /// 那么客户端应推断 `maxHeight = maxWidth`。
    ///
    /// The maximum height in pixels supported for this image. Clients must not expect requests with
    /// a height greater than this value to be supported. If `maxWidth` is specified and `maxHeight` is not,
    /// then clients should infer that `maxHeight = maxWidth`.
    #[serde(skip_serializing_if = "Option::is_none")]
    max_height: Option<u32>,

    /// 该图像支持的最大像素面积。客户端不应期望`width * height`大于此值的请求会被支持。
    ///
    /// The maximum area in pixels supported for this image. Clients must not
    /// expect requests with a `width * height` greater than this value to be supported.
    #[serde(skip_serializing_if = "Option::is_none")]
    max_area: Option<u32>,
}

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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct InfoType(String);

impl Default for InfoType {
    fn default() -> Self {
        Self("ImageService3".to_string())
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Protocol(String);

impl Default for Protocol {
    fn default() -> Self {
        Self("http://iiif.io/api/image".to_string())
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Profile {
    #[default]
    Level0,
    Level1,
    Level2,
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
