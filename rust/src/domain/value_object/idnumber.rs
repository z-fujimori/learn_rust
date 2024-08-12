#[derive(Debug)]
pub struct IdNumber {
    value: String,
}
impl IdNumber {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}