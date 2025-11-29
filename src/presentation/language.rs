use std::collections::HashMap;

/// 语言映射类型，键为语言代码（如 "en"、"zh-Hans"），值为该语言下的一组字符串。
pub type LangMap = HashMap<String, Vec<String>>;
