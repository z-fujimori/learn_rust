use crate::key::KEY;

#[derive(Debug)]
pub struct User {
    #[allow(dead_code)] // 使用されていない関数が存在すると出る警告を無視させる
    key: KEY,
    #[allow(dead_code)] // 使用されていない関数が存在すると出る警告を無視させる
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