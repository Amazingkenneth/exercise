use cursive::{
    traits::*,
    views::{Button, Dialog, DummyView, LinearLayout},
    Cursive,
};

pub fn run() {
    let mut siv = cursive::default();

    siv.add_global_callback('q', Cursive::quit);
    siv.add_layer(
        Dialog::text("This is a survey!\nPress <Next> when you're ready.")
            .title("Important survey")
            .button("Next", show_next),
    );

    siv.run();

    /*let buttons1 = LinearLayout::horizontal()
        .child(Button::new("Restart", restart))
        .child(Button::new("Hint", hint))
        .child(Button::new("Undo", undo))
        .child(Button::new("Redo", redo));

    let buttons2 = LinearLayout::horizontal()
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(DummyView)
        .child(Button::new("Help", help))
        .child(Button::new("Quit", Cursive::quit));

    let view = Dialog::around(
        LinearLayout::vertical()
            .child(board.with_name("board"))
            .child(buttons1)
            .child(buttons2),
    )
    .title("SUDOKU");*/
    //siv.add_layer(view);

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
/*fn restart(s: &mut Cursive) {
    s.call_on_name("board", |board: &mut MapBoard| {
        board.restart();
    });
}

fn hint(s: &mut Cursive) {
    s.call_on_name("board", |board: &mut MapBoard| {
        board.hint();
    });
}

fn undo(s: &mut Cursive) {
    s.call_on_name("board", |board: &mut MapBoard| {
        board.undo();
    });
}

fn redo(s: &mut Cursive) {
    s.call_on_name("board", |board: &mut MapBoard| {
        board.redo();
    });
}

fn help(s: &mut Cursive) {
    s.add_layer(Dialog::info("Use arrow keys/TAB/Shift+TAB/mouse wheel/mouse click to navigate.\nEnter number 0-9 to fill in.\nClick <Hint> or press <h> to obtain a hint.\nGood luck."))
}*/
