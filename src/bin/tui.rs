use cursive::views::Dialog;

fn main() {
    let mut siv = cursive::default();

	siv.add_global_callback('q', |s| s.quit());

    siv.add_layer(Dialog::text("Cock Evaluation\nPress <Next> when you're ready.\nPress <q> to quit.")
        .button("Next", | s | s.quit())
        .button("Quit", | s | s.quit()));

    siv.run();
}
