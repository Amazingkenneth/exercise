use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
struct Child {
    att: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
    tp: String,
    zh: String,
    num: i32,
    ch: Vec<Child>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Dlog {
    num: i32,
    tp: i8,
    txt: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Index {
    dlog: Vec<Dlog>,
    pub idx: Vec<Node>,
}

impl Index {
    /*fn getnext(num: i32) -> Dialog {
        let a: Dialog = binary_search_by
    }*/
}
