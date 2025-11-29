use serde::{Deserialize, Serialize};

use crate::presentation::{AnnotationPage, LangMap, Metadata, Resource};

/// Canvas 结构：定义一个时间/空间上的呈现平面。
///
/// Canvas structure: defines the spatial/temporal plane for rendering content.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Canvas {
    pub id: String,

    #[serde(default = "canvas_type")]
    pub r#type: String,

    /// 画布标题。
    ///
    /// Human-readable label of the canvas.
    pub label: LangMap,

    /// 画布的简要摘要（可多语言）。
    ///
    /// Short summary/description of the canvas (multi-language).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<LangMap>,

    /// 画布的元数据条目列表。
    ///
    /// Metadata entries for the canvas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Vec<Metadata>>,

    /// 画布的必需声明（如版权信息）。
    ///
    /// Required statement for the canvas (e.g. copyright).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_statement: Option<Metadata>,

    /// 权利或许可 URI。
    ///
    /// Rights or license URI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rights: Option<String>,

    /// 画布使用的语言代码列表。
    ///
    /// List of language codes used in this canvas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,

    /// 查看方向（如 left-to-right、right-to-left 等）。
    ///
    /// Viewing direction (e.g. left-to-right, right-to-left, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewing_direction: Option<String>,

    /// 行为提示（如 `paged`、`continuous`）。
    ///
    /// Behavioral hints for the canvas (e.g. `paged`, `continuous`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior: Option<Vec<String>>,

    /// 画布的缩略图资源。
    ///
    /// Thumbnail resources for the canvas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<Vec<Resource>>,

    /// 画布的主页资源。
    ///
    /// Homepage resources for the canvas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<Vec<Resource>>,

    /// 相关的外部机器可读资源。
    ///
    /// Machine-readable external resources related to this canvas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub see_also: Option<Vec<Resource>>,

    /// 画布的其他渲染形式。
    ///
    /// Alternative renderings of this canvas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering: Option<Vec<Resource>>,

    /// 与该画布相关的服务。
    ///
    /// Services related to this canvas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Vec<Resource>>,

    /// 该画布所属的上级资源（通常为 Manifest）。
    ///
    /// Parent resources (typically a Manifest) this canvas is part of.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_of: Option<Vec<Resource>>,

    /// painting 注解所在的 AnnotationPage 列表。
    ///
    /// Annotation pages with `painting` annotations.
    pub items: Vec<AnnotationPage>,

    /// `supplementing` 等其他注解。
    ///
    /// Additional annotation pages such as `supplementing`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<AnnotationPage>>,
}

fn canvas_type() -> String {
    "Canvas".to_string()
}

impl Default for Canvas {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            r#type: canvas_type(),
            label: LangMap::default(),
            summary: None,
            metadata: None,
            required_statement: None,
            rights: None,
            language: None,
            height: None,
            width: None,
            duration: None,
            viewing_direction: None,
            behavior: None,
            thumbnail: None,
            homepage: None,
            see_also: None,
            rendering: None,
            service: None,
            part_of: None,
            items: Vec::new(),
            annotations: None,
        }
    }
}
