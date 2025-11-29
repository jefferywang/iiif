use serde::{Deserialize, Serialize};

use crate::presentation::{Context, LangMap, Metadata, Resource};

/// 顶层 Collection 结构
///
/// Top-level Collection structure, covering the main fields from Presentation API 3.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Collection {
    #[serde(rename = "@context", default = "Context::presentation_default")]
    pub context: Context,

    pub id: String,

    #[serde(default = "collection_type")]
    pub r#type: String,

    /// 集合的标题（可多语言、多值）。
    ///
    /// Human-readable label of the collection (multi-language, multi-value).
    pub label: LangMap,

    /// 集合的简要摘要（可多语言）。
    ///
    /// Short summary/description of the collection (multi-language).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<LangMap>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Vec<Metadata>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_statement: Option<Metadata>,

    /// 权利或许可 URI。
    ///
    /// Rights or license URI.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rights: Option<String>,

    /// 该集合的语言代码列表（如 "en"、"zh-Hans"）。
    ///
    /// List of language codes used in this collection (e.g. "en", "zh-Hans").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Vec<String>>,

    /// 提供该资源的机构或主体。
    ///
    /// Providers (institutions, organizations) of this resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<Vec<Resource>>,

    /// 集合的 logo 资源。
    ///
    /// Logo resources for the collection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<Vec<Resource>>,

    /// 集合的缩略图资源。
    ///
    /// Thumbnail resources for the collection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<Vec<Resource>>,

    /// 集合的主页链接。
    ///
    /// Homepage resources for the collection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<Vec<Resource>>,

    /// 相关的外部机器可读资源（如 RDF、METS 等）。
    ///
    /// Machine-readable external resources related to this collection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub see_also: Option<Vec<Resource>>,

    /// 该集合的可选渲染形式（如 PDF 导出）。
    ///
    /// Alternative renderings of this collection (e.g. PDF).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering: Option<Vec<Resource>>,

    /// 与此集合相关的服务（如搜索服务、认证服务等）。
    ///
    /// Services related to this collection (search, auth, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Vec<Resource>>,

    /// 上级集合或清单。
    ///
    /// Parent collections or manifests this collection is part of.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_of: Option<Vec<Resource>>,

    /// 行为提示（如 paged、continuous 等）。
    ///
    /// Behavioral hints such as `paged`, `continuous`, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior: Option<Vec<String>>,

    /// 子项可以是 Manifest 或子 Collection，这里统一建模为 Resource 引用。
    ///
    /// Items can be Manifests or child Collections, modeled as generic resources.
    pub items: Vec<Resource>,
}

fn collection_type() -> String {
    "Collection".to_string()
}

impl Default for Collection {
    fn default() -> Self {
        Self {
            context: Context::presentation_default(),
            id: "".to_string(),
            r#type: collection_type(),
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
            items: Vec::new(),
        }
    }
}
