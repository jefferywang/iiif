use serde::{Deserialize, Serialize};

use crate::presentation::LangMap;

/// 通用的资源引用结构，用于 `thumbnail` / `homepage` / `rendering` / `provider` / `service` 等。
///
/// Generic resource reference used for `thumbnail`, `homepage`, `rendering`, `provider`, `service`, etc.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Resource {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<LangMap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<String>,
}
