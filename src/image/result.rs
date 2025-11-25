/// iiif 处理结果
#[derive(Debug)]
pub struct ProcessResult {
    pub content_type: String,
    pub data: Vec<u8>,
}

/// ProcessResult 实现
impl ProcessResult {
    /// 创建新的 ProcessResult
    pub fn new(content_type: String, data: Vec<u8>) -> Self {
        Self { content_type, data }
    }
}
