use cursive::{
    traits::*,
    views::*,
    Cursive,
};
use cursive_tabs::*;
use crate::plots::entry;
use crate::graph::Index;
use std::io::Read;
// use serde_json::Error;
fn game_panel(s: &mut Cursive) {
    let mut panel = TabPanel::new()
        .with_tab(TextView::new("按 q 退出").with_name("介绍"))
        .with_tab(Dialog::text("按照 config.json 中的剧情安排进行游戏")
            .button("点此进入 LPF 的惊险奇妙故事", |s| {
                plot_config(s, 0);
            })
            .with_name("Plots"))
        .with_tab(TextView::new("This is the second view!").with_name("Online"));
    s.add_layer(panel);
}
pub fn start() {
    let mut siv = cursive::default();
    siv.add_global_callback('q', Cursive::quit);
    game_panel(&mut siv);
    siv.run();
}
fn plot_config(s: &mut Cursive, to: i32) {
    let mut index_file = std::fs::File::open("./config.json").expect("Cannot open json file.");
    let mut contents = String::new();
    index_file.read_to_string(&mut contents).unwrap();
    let ret: Result<Index, serde_json::Error> = serde_json::from_str(&contents);
    match ret {
        Ok(ret) => {
            match to {
                0 => {
                    entry(s, ret);
                },
                _ => (),
            }
        }
        Err(ret) => {
            s.add_layer(Dialog::text("请确认 config.json 的格式正确")
            .title("Error")
            .button("Return", |s| {
                s.pop_layer();
                game_panel(s)
            }));
        }
    };
}
