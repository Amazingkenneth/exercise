use exercise::game::run;
use std::io::*;
use serde::{Deserialize, Serialize};
// use serde_json::Result;
fn main() {
    preload();
    run();
}

#[derive(Debug, Serialize, Deserialize)]
struct Child {
    att: Vec<i8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Node {
    tp: String,
    zh: String,
    num: i8,
    ch: Vec<Child>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Index {
    idx: Vec<Node>,
}

fn preload() {
    let mut index_file = std::fs::File::open("config.json").expect("Cannot open json file.");
    let mut contents = String::new();
    index_file.read_to_string(&mut contents).unwrap();
    let set: Index = serde_json::from_str(&contents).unwrap();
}
