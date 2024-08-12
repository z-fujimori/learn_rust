use std::fmt;

// #[derive(Debug)]
pub struct IdNumber {
    #[allow(dead_code)] // 使用されていない関数が存在すると出る警告を無視させる
    value: String,
}
impl IdNumber {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}

impl fmt::Debug for IdNumber {
    fn fmt(&self, f:&mut fmt::Formatter) -> Result<(), fmt::Error>{
        let visible_length = 2;
        let masked = {
            let start = self.value.len() - visible_length;
            format!("{:*<20}", &self.value[0..start])
        };
        f.debug_tuple("Id of Human").field(&masked).finish()
    }
}

