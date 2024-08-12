use crate::key::KEY;

#[derive(Debug)]
pub struct User {
    key: KEY,
    name: String,
}
impl User {
    pub fn new(key: KEY, name: &str) -> Self {
        Self {
            key,
            name: name.to_string(),
        }
    }
}