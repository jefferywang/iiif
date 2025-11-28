use serde::{Deserialize, Serialize};

use crate::{Format, Quality};

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

    /// 支持的 Quality 列表
    ///
    /// A list of supported Quality values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_qualities: Option<Vec<Quality>>,

    /// 支持的 Feature 列表
    ///
    /// A list of supported Feature values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_features: Option<Vec<Feature>>,

    /// 指向另一个引用该图像服务的资源的链接，例如指向画布或清单的链接。该值必须是 JSON 对象数组。每个项目必须具备 `id` 和 `type`属性，并且应拥有 `label` 属性。
    ///
    /// A link to another resource that references this image service, for example a link to a Canvas or Manifest. The value must be an array of
    /// JSON objects. Each item must have the `id` and `type` properties, and should have the `label` property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_of: Option<LinkInfo>,

    /// 指向与该资源相关的外部机器可读资源的链接，如 XML 或 RDF 描述。应提供外部资源的属性，帮助客户在多个描述（如提供）中选择，并合理利用文档。
    /// 文档的 URI 必须识别特定格式中数据的单一表示。该值必须是 JSON 对象数组。每个项目必须具备 `id` 和 `type` 属性，并应具备`label` 、
    /// `format`和 `profile` 文件属性。
    ///
    /// A link to an external, machine-readable resource that is related to this resource, such as an XML or RDF description.
    /// Properties of the external resource should be given to help the client select between multiple descriptions (if provided),
    /// and to make appropriate use of the document. The URI of the document must identify a single representation of the data
    /// in a particular format. The value must be an array of JSON objects. Each item must have the `id` and `type` properties,
    /// and should have the `label`, `format` and `profile` properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub see_also: Option<LinkInfo>,

    /// 对客户端可能直接交互以获取额外信息或功能的外部服务的引用，例如指向认证服务的链接。该值必须是 JSON 对象数组。每个对象会根据服务定义拥有属性，
    /// 但必须具备 `id` 和`type`属性，或 `@id` 和 `@type` 属性，以便向后兼容其他 IIIF API。每个对象都应该有一个配置文件属性。
    /// 请参阅[服务注册表](https://iiif.io/api/annex/services/)以了解已知的服务类型。
    ///
    /// A reference to an external service that the client might interact with directly to gain additional information or functionality,
    /// for example a link to an authentication service. The value must be an array of JSON objects. Each object will have properties
    /// depending on the service’s definition, but must have either the `id` and `type` properties, or the `@id` and `@type` properties
    /// for backwards compatibility with other IIIF APIs. Each object should have a profile property. See the
    /// [Service Registry](https://iiif.io/api/annex/services/) for known service types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<LinkInfo>,
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

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Feature {
    /// 服务的基础 URI 将重定向到图像信息文档
    ///
    /// The base URI of the service will redirect to the image information document.
    BaseUriRedirect,

    /// 图像响应中提供了规范的图像 URI HTTP 链接头。
    ///
    /// The canonical image URI HTTP link header is provided on image responses.
    CanonicalLinkHeader,

    /// 所有响应都提供了 CORSHTTP 头部。
    ///
    /// The CORS HTTP headers are provided on all responses.
    Cors,

    /// JSON-LD 媒体类型在请求时提供。
    ///
    /// The JSON-LD media type is provided when requested.
    JsonldMediaType,

    /// 图像可以绕垂直轴旋转，从而实现内容从左到右的镜像。
    ///
    /// The image may be rotated around the vertical axis, resulting in a left-to-right mirroring of the content.
    Mirroring,

    /// 配置文件 HTTP 链接头在图像响应中提供。
    ///
    /// The profile HTTP link header is provided on image responses.
    ProfileLinkHeader,

    /// 完整图像的区域可按百分比请求。
    ///
    /// Regions of the full image may be requested by percentage.
    RegionByPct,

    /// 像素尺寸可以请求完整图像的区域。
    ///
    /// Regions of the full image may be requested by pixel dimensions.
    RegionByPx,

    /// 可以请求一个方形区域，其中宽度和高度等于完整图像的较短尺寸。
    ///
    /// Image rotation may be requested using values other than multiples of 90 degrees.
    RegionSquare,

    /// 图像旋转请求可以90度的倍数进行。
    ///
    /// Image rotation may be requested in multiples of 90 degrees.
    RotationBy90s,

    /// 图片尺寸可通过 `!w,h` 格式请求。
    ///
    /// Image size may be requested in the form `!w,h`.
    SizeByConfinedWh,

    /// 图片尺寸可通过 `,h` 格式请求。
    ///
    /// Image size may be requested in the form `,h`.
    SizeByH,

    /// 图片尺寸可通过 `pct:n` 格式请求。
    ///
    /// Image size may be requested in the form `pct:n`.
    SizeByPct,

    /// 图片尺寸可通过 `w,` 格式请求。
    ///
    /// Image size may be requested in the form `w,`.
    SizeByW,

    /// 图片尺寸可通过 `w,h` 格式请求。
    ///
    /// Image size may be requested in the form `w,h`.
    SizeByWh,

    /// 可请求以 `^` 作为前缀的图像尺寸。
    ///
    /// Image sizes prefixed with `^` may be requested.
    SizeUpscaling,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkInfo {
    /// 外部资源的 URI。`@id` 属性可用于 `service` 对象中 ，如上所述，以实现向后兼容性。
    ///
    /// The URI of the external resource.
    /// The `@id` property may be used in `service` objects for backwards compatibility as described above.
    #[serde(rename = "@id")]
    id: String,

    /// 该资源的类型或类别。[Presentation API](https://iiif.io/api/presentation/3.0/#type) 中给出了基本类型的建议，如图片、
    /// 文本或音频。（`@type` 属性可用于`service`对象， 如上所述的向后兼容。）
    ///
    /// The type or class of this resource. Recommendations for basic types such as image, text or audio are given in
    /// the [Presentation API](https://iiif.io/api/presentation/3.0/#type).(The `@type` property may be used in `service`
    /// objects for backwards compatibility as described above.)
    #[serde(rename = "@type")]
    r#type: String,

    /// 为本资源提供一个易于阅读的标签。`label`属性可以完全国际化，每种语言可以有多个值。该模式在
    /// [Presentation API 的语言部分](https://iiif.io/api/presentation/3.0/#language-of-property-values)有更详细的描述。
    ///
    /// A human-readable label for this resource. The label property can be fully internationalized, and each language
    /// can have multiple values. This pattern is described in more detail in [the languages section of
    /// the Presentation API](https://iiif.io/api/presentation/3.0/#language-of-property-values).
    label: Option<String>,

    /// 该内容资源的特定媒体类型（通常称为 MIME 类型），例如“image/jpeg”。这对于区分同一整体资源的不同格式非常重要，例如区分 XML 文本和纯文本。
    /// 该值必须是字符串，并且应是该资源被取消引用时返回的 Content-Type 头部的值。
    ///
    /// The specific media type (often called a MIME type) for this content resource, for example “image/jpeg”. This is important
    /// for distinguishing different formats of the same overall type of resource, such as distinguishing text in XML from plain text.
    /// The value must be a string, and it should be the value of the Content-Type header returned when this resource is dereferenced.
    format: Option<String>,

    /// 该资源提供的模式或命名功能集。配置文件可以进一步明确外部资源的`type`和/或`format` 。该值必须是字符串，可以从[配置文件注册表](https://iiif.io/api/registry/)
    /// 中提取，也可以是 URI。
    ///
    /// A schema or named set of functionality available from this resource. The profile can further clarify the `type` and/or `format` of
    /// an external resource. The value must be a string, either taken from the Registry of Profiles or a URI.
    profile: Option<String>,
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
