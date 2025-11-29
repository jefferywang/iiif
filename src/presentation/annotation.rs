use serde::{Deserialize, Serialize};

use crate::presentation::{LangMap, Resource};

/// AnnotationPage：Annotation 的有序列表。
///
/// AnnotationPage: an ordered list of Annotations.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnnotationPage {
    pub id: String,

    #[serde(default = "annotation_page_type")]
    pub r#type: String,

    /// 注解页的标题（可多语言、多值）。
    ///
    /// Human-readable label of the annotation page (multi-language, multi-value).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<LangMap>,

    pub items: Vec<Annotation>,
}

fn annotation_page_type() -> String {
    "AnnotationPage".to_string()
}

impl Default for AnnotationPage {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            r#type: annotation_page_type(),
            label: None,
            items: Vec::new(),
        }
    }
}

/// Annotation：描述 body 如何与 target（通常是 Canvas 或其片段）关联。
///
/// Annotation: describes how a body resource relates to a target (typically a Canvas or fragment).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Annotation {
    pub id: String,

    #[serde(default = "annotation_type")]
    pub r#type: String,

    /// 注解的标题（可多语言、多值）。
    ///
    /// Human-readable label of the annotation (multi-language, multi-value).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<LangMap>,

    /// 注解的动机，通常为 "painting" 或 "supplementing"。
    ///
    /// Motivation of the annotation, typically "painting" or "supplementing".
    pub motivation: String,

    /// 目标 Canvas 或其片段 URI。
    ///
    /// Target Canvas or fragment URI.
    pub target: String,

    /// body 可以是资源或文本，这里使用 Resource 简化建模。
    ///
    /// Body can be a resource or text, modeled here as a Resource for simplicity.
    pub body: Resource,
}

fn annotation_type() -> String {
    "Annotation".to_string()
}

impl Default for Annotation {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            r#type: annotation_type(),
            label: None,
            motivation: "".to_string(),
            target: "".to_string(),
            body: Resource::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_annotation_page_default() {
        let annotation_page = AnnotationPage::default();
        assert_eq!(annotation_page.id, "");
        assert_eq!(annotation_page.r#type, "AnnotationPage");
    }

    #[test]
    fn test_annotation_default() {
        let annotation = Annotation::default();
        assert_eq!(annotation.id, "");
        assert_eq!(annotation.r#type, "Annotation");
    }
}
