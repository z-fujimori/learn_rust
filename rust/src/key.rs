#[derive(Debug)]
pub struct KEY {
    value: String,
}
impl KEY {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}