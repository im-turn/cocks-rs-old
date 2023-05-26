use cock_tier::bin_modules::{
    tui_prompt::{draw::TUIDisplay, theme::popup_theme_menu},
    UserData,
};
use cursive::event::Key;

fn main() {
    // init cursive runnable instance
    let mut siv = cursive::default();

    // set `UserData` struct as user data for cursive instance
    siv.set_user_data(UserData::default());

    // global callback to exit the program
    siv.add_global_callback(Key::Esc, |s| s.quit());
    siv.add_global_callback('\\', |s| popup_theme_menu(s));

    // get the initial state set by `UserData::default()`
    let state = siv.user_data::<UserData>().unwrap().clone().state;

    // begin drawing the screens from default state
    state.draw(&mut siv);

    // run cursive instance
    siv.run();
}
