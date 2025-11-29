use serde::{Deserialize, Serialize};

use crate::presentation::LangMap;

/// 公共的 metadata 条目：label + value，均为 LanguageMap。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub label: LangMap,
    pub value: LangMap,
}
