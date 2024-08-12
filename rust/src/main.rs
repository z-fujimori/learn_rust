// ファイルを分けて書いてみる　./key.rs ./user.rs
mod key;
mod user;

// ディレクトリを分けて書いてみる　./domain
mod domain;
use domain::{entity::Human, value_object::IdNumber};

#[derive(Debug)]
struct ID {
    #[allow(dead_code)] // 使用されていない関数が存在すると出る警告を無視させる
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
    #[allow(dead_code)] // 使用されていない関数が存在すると出る警告を無視させる
    id: ID,
    #[allow(dead_code)] // 使用されていない関数が存在すると出る警告を無視させる
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

    let mut hoge_human = Human::new(IdNumber::new("a111"), "Kai Harada", 21);
    println!("Hello, human: {:?}", hoge_human);
    hoge_human.berthday();
    println!("Hello, human: {:#?}\n", hoge_human);
    println!("name: {} age: {}",hoge_human.name, hoge_human.age);
}

