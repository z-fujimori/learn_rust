// ファイルを分けて書いてみる　./key.rs ./user.rs
mod key;
mod user;

// ディレクトリを分けて書いてみる　./domain
mod domain;
use domain::{entity::Human, value_object::IdNumber};

#[derive(Debug)]
struct ID {
    value: String,
}
impl ID {
    fn new(value: &str) -> Self{
        Self {
            value: value.to_string(),
        }
    }
}
#[derive(Debug)]
struct Node {
    id: ID,
    label: String,
}
impl Node {
    fn new(id: ID, label: &str) -> Self {
        Self {
            id, 
            label: label.to_string(),
        }
    }
}

fn main() {
    let node = Node::new(ID::new("1"), "Node 1");
    println!("Hello, module: {:?}\n", node);

    let test_user = user::User::new(key::KEY::new("aabbcc"), "tahichi fukuoka");
    println!("Hello, user: {:?}\n", test_user);

    let mut hogeHuman = Human::new(IdNumber::new("a111"), "Kai Harada", 21);
    println!("Hello, human: {:?}", hogeHuman);
    hogeHuman.berthday();
    println!("Hello, human: {:?}\n", hogeHuman);
    println!("name: {} age: {}",hogeHuman.name, hogeHuman.age);
}

