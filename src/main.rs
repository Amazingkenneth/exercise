use cursive::views::Dialog;
use cursive::Cursive;
use serde::{Deserialize, Serialize};
use std::io::*;
// use serde_json::Result;

fn main() {
    //read_json();
    let mut siv = cursive::default();

    siv.add_layer(
        Dialog::text("This is a survey!\nPress <Next> when you're ready.")
            .title("Important survey")
            .button("Next", show_next),
    );

    siv.run();
}

fn show_next(s: &mut Cursive) {
    s.pop_layer();
    s.add_layer(
        Dialog::text("Did you do the thing?")
            .title("Question 1")
            .button("Yes!", |s| show_answer(s, "I knew it! Well done!"))
            .button("No!", |s| show_answer(s, "I knew you couldn't be trusted!"))
            .button("Uh?", |s| s.add_layer(Dialog::info("Try again!"))),
    );
}

fn show_answer(s: &mut Cursive, msg: &str) {
    s.pop_layer();
    s.add_layer(
        Dialog::text(msg)
            .title("Results")
            .button("Finish", |s| s.quit()),
    );
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
    child: Vec<Child>,
}
#[derive(Debug, Serialize, Deserialize)]
struct Index {
    idx: Vec<Node>,
}
fn read_json() {
    let mut index_file = std::fs::File::open("map.json").expect("Cannot open json file.");
    let mut contents = String::new();
    index_file.read_to_string(&mut contents).unwrap();
    let set: Index = serde_json::from_str(&contents).unwrap();
}
