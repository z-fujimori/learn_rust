#[derive(Debug)]
pub struct KEY {
    #[allow(dead_code)] // 使用されていない関数が存在すると出る警告を無視させる
    value: String,
}
impl KEY {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}