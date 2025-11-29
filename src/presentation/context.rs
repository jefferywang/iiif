use serde::{Deserialize, Serialize};

/// IIIF Presentation API 3.0 context地址
const IIIF_PRESENTATION_3_CONTEXT: &str = "https://iiif.io/api/presentation/3/context.json";

/// IIIF Image API 3.0 context地址
const IIIF_IMAGE_3_CONTEXT: &str = "http://iiif.io/api/image/3/context.json";

/// 顶层 @context，可以是单个字符串或字符串数组。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Context {
    Single(String),
    List(Vec<String>),
}

impl Context {
    pub fn presentation_default() -> Self {
        Self::Single(IIIF_PRESENTATION_3_CONTEXT.to_string())
    }

    pub fn image_default() -> Self {
        Self::Single(IIIF_IMAGE_3_CONTEXT.to_string())
    }

    pub fn new_presentation_list(contexts: &[String]) -> Self {
        Self::new_list(contexts, IIIF_PRESENTATION_3_CONTEXT)
    }

    pub fn new_image_list(contexts: &[String]) -> Self {
        Self::new_list(contexts, IIIF_IMAGE_3_CONTEXT)
    }

    /// 创建包含多个 context 的列表，并确保 Presentation 3 的 context 在最后。
    fn new_list(contexts: &[String], default_context: &str) -> Self {
        let mut contexts = contexts.to_vec();
        if let Some(idx) = contexts.iter().position(|c| c == default_context) {
            contexts.remove(idx);
        }
        contexts.push(default_context.to_string());
        Context::List(contexts)
    }
}
