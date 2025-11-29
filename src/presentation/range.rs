use serde::{Deserialize, Serialize};

use crate::presentation::{Canvas, LangMap, Metadata, Resource};

/// Range 结构：用于表示结构化的范围（如章节、目录等）。
///
/// Range structure: represents structured ranges such as chapters, table of contents, etc.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Range {
    pub id: String,

    #[serde(default = "range_type")]
    pub r#type: String,

    /// 范围的标题（可多语言、多值）。
    ///
    /// Human-readable label of the range (multi-language, multi-value).
    pub label: LangMap,

    /// 范围的简要摘要（可多语言）。
    ///
    /// Short summary/description of the range (multi-language).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<LangMap>,

    /// 范围的元数据条目列表。
    ///
    /// Metadata entries for the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Vec<Metadata>>,

    /// 范围的必需声明（如版权信息）。
    ///
    /// Required statement for the range (e.g. copyright).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_statement: Option<Metadata>,

    /// 权利或许可 URI。
    ///
    /// Rights or license URI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rights: Option<String>,

    /// 范围使用的语言代码列表。
    ///
    /// List of language codes used in this range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Vec<String>>,

    /// 提供该资源的机构或主体。
    ///
    /// Providers (institutions, organizations) of this range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<Vec<Resource>>,

    /// 范围的 logo 资源。
    ///
    /// Logo resources for the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<Vec<Resource>>,

    /// 范围的缩略图资源。
    ///
    /// Thumbnail resources for the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<Vec<Resource>>,

    /// 范围的主页资源。
    ///
    /// Homepage resources for the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<Vec<Resource>>,

    /// 相关的外部机器可读资源（如 RDF、METS 等）。
    ///
    /// Machine-readable external resources related to this range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub see_also: Option<Vec<Resource>>,

    /// 范围的其他渲染形式（如 PDF）。
    ///
    /// Alternative renderings of this range (e.g. PDF).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering: Option<Vec<Resource>>,

    /// 与该范围相关的服务（例如 ImageService、SearchService 等）。
    ///
    /// Services related to this range (e.g. ImageService, SearchService).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Vec<Resource>>,

    /// 该范围所属的上级资源（通常为 Manifest 或其他 Range）。
    ///
    /// Parent resources (typically a Manifest or other Range) this range is part of.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_of: Option<Vec<Resource>>,

    /// 行为提示（如 paged、continuous 等）。
    ///
    /// Behavioral hints such as `paged`, `continuous`, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior: Option<Vec<String>>,

    /// 查看方向（如 left-to-right、right-to-left 等）。
    ///
    /// Viewing direction (e.g. left-to-right, right-to-left, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewing_direction: Option<String>,

    /// 范围的子项列表，可以包含 Range 或 Canvas。
    ///
    /// List of child items, which can be Ranges or Canvases.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<RangeItem>>,

    /// 推荐的起始项（通常是某个 Canvas 或 Range）。
    ///
    /// Recommended start item (usually a Canvas or Range).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<Resource>,
}

fn range_type() -> String {
    "Range".to_string()
}

impl Default for Range {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            r#type: range_type(),
            label: LangMap::default(),
            summary: None,
            metadata: None,
            required_statement: None,
            rights: None,
            language: None,
            provider: None,
            logo: None,
            thumbnail: None,
            homepage: None,
            see_also: None,
            rendering: None,
            service: None,
            part_of: None,
            behavior: None,
            viewing_direction: None,
            items: None,
            start: None,
        }
    }
}

/// Range 的 items 可以是 Range 或 Canvas。
///
/// Range items can be either a Range or a Canvas.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RangeItem {
    /// 子范围。
    ///
    /// Child range.
    Range(Box<Range>),

    /// 画布。
    ///
    /// Canvas.
    Canvas(Canvas),
}
