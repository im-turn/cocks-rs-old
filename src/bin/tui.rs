use cock_tier::{
    ID,
    GetVariants,
};
use cursive::Cursive;
use cursive::align::HAlign;
use cursive::event::EventResult;
use cursive::traits::*;
use cursive::views::{Dialog, OnEventView, SelectView, TextView};

fn main() {
    let mut siv = cursive::default();

    // Sets callback for when `q` is pressed.
	siv.add_global_callback(
        'q',
        |s| s.quit()
    );

    siv.add_layer(
        Dialog::text("\n<Next> to begin.\n<q> to quit.")
        .title("COCK EVALUATION")
        .button(
            "Next",
            | s | next_prompt(s, ID::get_variants(), "Choose ID Type")
        )
        .h_align(HAlign::Center)
    );

    siv.run();
}

fn next_prompt(siv: &mut Cursive, options: Vec<String>, title: &str) {
    siv.pop_layer();
    
    let mut select = SelectView::new()
        // Center the text horizontally
        .h_align(HAlign::Center)
        // Use keyboard to jump to the pressed letters
        .autojump();

    // Read the list of cities from separate file, and fill the view with it.
    // (We include the file at compile-time to avoid runtime read errors.)
    select.add_all_str(options);

    // Sets the callback for when "Enter" is pressed.
    select.set_on_submit(show_choice);

    // `j` and `k` keys for navigation
    let select = OnEventView::new(select)
        .on_pre_event_inner('k', |s, _| {
            let cb = s.select_up(1);
            Some(EventResult::Consumed(Some(cb)))
        })
        .on_pre_event_inner('j', |s, _| {
            let cb = s.select_down(1);
            Some(EventResult::Consumed(Some(cb)))
        });

    // ResizedView to keep the list at a reasonable size
    // (it can scroll anyway).
    siv.add_layer(
        Dialog::around(
            select.scrollable().fixed_size((40, 20))
        ).title(title),
    );


}

fn show_choice(siv: &mut Cursive, option: &str) {
    siv.pop_layer();

    siv.add_layer(
        Dialog::around(
            TextView::new(option))
            .button("Quit", |s| s.quit()
        )
    );
}
