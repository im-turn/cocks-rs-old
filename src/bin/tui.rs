use cock_tier::{
    TUIDisplay,
    bin_modules::UserData,
};

fn main() {
    let mut siv = cursive::default();

    siv.set_user_data(UserData::default());

    siv.add_global_callback('q', | s | s.quit());

    let data = siv.user_data::<UserData>().unwrap().clone();

    data.state.draw(&mut siv);

    siv.run();
}
