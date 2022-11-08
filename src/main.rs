mod board;
mod game;
use crate::game::run;
use serde::{Deserialize, Serialize};
use std::io::*;
// use serde_json::Result;
fn main() {
    preload();
    run();
}

#[derive(Debug, Serialize, Deserialize)]
struct Child {
    att: Vec<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Node {
    tp: String,
    zh: String,
    num: i32,
    ch: Vec<Child>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Dialog {
    num: i32,
    tp: i8,
    txt: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Index {
    dialog: Vec<Dialog>,
    idx: Vec<Node>,
}

fn preload() {
    let mut index_file = std::fs::File::open("./config.json").expect("Cannot open json file.");
    let mut contents = String::new();
    index_file.read_to_string(&mut contents).unwrap();
    let set: Index = serde_json::from_str(&contents).unwrap();
}
