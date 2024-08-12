use crate::domain::value_object::IdNumber;

#[derive(Debug)]
pub struct Human {
    id: IdNumber,
    pub name: String,
    pub age: i32,
}
impl Human {
    pub fn new(id: IdNumber, name: &str, age: i32) -> Self {
        Self {
            id,
            name: name.to_string(),
            age,
        }
    }
    pub fn berthday(&mut self) {
        self.age += 1;
        println!("HappyBerthDay {}({}) !",self.name, self.age);
    }
}