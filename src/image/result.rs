pub struct ProcessResult {
    pub content_type: String,
    pub data: Vec<u8>,
}

impl ProcessResult {
    pub fn new(content_type: String, data: Vec<u8>) -> Self {
        Self { content_type, data }
    }
}
