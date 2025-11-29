use serde::{Deserialize, Serialize};

use crate::presentation::{Canvas, Context, LangMap, Metadata, Range, Resource};

/// Manifest 结构，尽量覆盖 Presentation 3 规范中的主要字段。
///
/// Manifest structure, covering the main fields from Presentation API 3.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Manifest {
    #[serde(rename = "@context", default = "Context::presentation_default")]
    pub context: Context,

    pub id: String,

    #[serde(default = "manifest_type")]
    pub r#type: String,

    /// 清单的标题（可多语言、多值）。
    ///
    /// Human-readable label of the manifest (multi-language, multi-value).
    pub label: LangMap,

    /// 清单的简要摘要（可多语言）。
    ///
    /// Short summary/description of the manifest (multi-language).
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

    /// 清单使用的语言代码列表。
    ///
    /// List of language codes used in this manifest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Vec<String>>,

    /// 清单的行为提示（如 paged、continuous 等）。
    ///
    /// Behavioral hints for the manifest (e.g. `paged`, `continuous`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior: Option<Vec<String>>,

    /// 查看方向（如 left-to-right、right-to-left 等）。
    ///
    /// Viewing direction (e.g. left-to-right, right-to-left, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub viewing_direction: Option<String>,

    /// 提供该资源的机构或主体。
    ///
    /// Providers (institutions, organizations) of this manifest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<Vec<Resource>>,

    /// 清单的 logo 资源。
    ///
    /// Logo resources for the manifest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<Vec<Resource>>,

    /// 清单的缩略图资源。
    ///
    /// Thumbnail resources for the manifest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<Vec<Resource>>,

    /// 清单的主页资源。
    ///
    /// Homepage resources for the manifest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homepage: Option<Vec<Resource>>,

    /// 相关外部机器可读资源。
    ///
    /// Machine-readable external resources related to this manifest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub see_also: Option<Vec<Resource>>,

    /// 清单的其他渲染形式（如 PDF）。
    ///
    /// Alternative renderings of this manifest (e.g. PDF).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering: Option<Vec<Resource>>,

    /// 与该清单相关的服务（例如 ImageService、SearchService 等）。
    ///
    /// Services related to this manifest (e.g. ImageService, SearchService).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<Vec<Resource>>,

    /// 该清单所属的上级集合。
    ///
    /// Parent collections this manifest is part of.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part_of: Option<Vec<Resource>>,

    /// 与该清单一起呈现的伴随画布。
    ///
    /// Accompanying canvas presented together with this manifest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accompanying_canvas: Option<Resource>,

    /// 占位用的画布，例如缩略预览。
    ///
    /// Placeholder canvas for this manifest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder_canvas: Option<Resource>,

    /// 用于导航的日期。
    ///
    /// Navigation date used for sorting or navigation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nav_date: Option<String>,

    /// 清单中的结构（Range 列表，表示章节/目录）。
    ///
    /// Ranges that describe the structure (table of contents) of this manifest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structures: Option<Vec<Range>>,

    /// 清单的补充资源（例如附录、相关材料）。
    ///
    /// Supplementary resources for this manifest.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplementary: Option<Vec<Resource>>,

    /// 推荐的起始项（通常是某个 Canvas 或 Range）。
    ///
    /// Recommended start item (usually a Canvas or Range).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<Resource>,

    /// 画布列表。
    ///
    /// List of canvases in this manifest.
    pub items: Vec<Canvas>,
}

fn manifest_type() -> String {
    "Manifest".to_string()
}

impl Default for Manifest {
    fn default() -> Self {
        Self {
            context: Context::presentation_default(),
            id: "".to_string(),
            r#type: manifest_type(),
            label: LangMap::default(),
            summary: None,
            metadata: None,
            required_statement: None,
            rights: None,
            language: None,
            behavior: None,
            viewing_direction: None,
            provider: None,
            logo: None,
            thumbnail: None,
            homepage: None,
            see_also: None,
            rendering: None,
            service: None,
            part_of: None,
            accompanying_canvas: None,
            placeholder_canvas: None,
            nav_date: None,
            structures: None,
            supplementary: None,
            start: None,
            items: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_default() {
        let manifest = Manifest::default();
        assert_eq!(manifest.id, "");
        assert_eq!(manifest.r#type, "Manifest");
    }
}
