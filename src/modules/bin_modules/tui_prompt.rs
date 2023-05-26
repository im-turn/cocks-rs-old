use cursive::{Cursive, views::{Dialog, SelectView, OnEventView}, align::HAlign, event::EventResult, view::{Scrollable, Resizable}};

use crate::{
    TUIDisplay, GetVariants,
    bin_modules::AppState,
    CockHandler, ID, InnerUser, modules::user
};

use super::UserData;

impl TUIDisplay for AppState {
    fn draw(&self, s: &mut Cursive) {
        match self {
            AppState::Home => draw_home(s),
            AppState::Result => draw_result(s),
            AppState::Id => draw_id(s),
            AppState::Size => draw_size(s),
            AppState::Abnormalities => draw_manual_options(s),
            AppState::Curvature => draw_manual_options(s),
            AppState::Shape => draw_manual_options(s),
            _ => draw_options(s),
        }
    }
}

pub fn draw_home(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::text("Welcome to Turn's Cock Tier Evaluator")
        .button("Begin", | s | {
            let mut val = s.user_data::<UserData>().unwrap().clone();
            val.state = val.state.next();
            s.set_user_data(val.clone());
            s.pop_layer();
            val.state.draw(s)
        })
    )
}

pub fn draw_result(siv: &mut Cursive) {
    let val = siv.user_data::<UserData>().unwrap().clone();
    let handler = CockHandler::new(val.user, val.cock);

    siv.add_layer(
        Dialog::text(format!("{handler}"))
        .title("COCK RESULTS")
        .button("Next", | s | {
            let mut val = s.user_data::<UserData>().unwrap().clone();
            val.state = val.state.next();
            s.set_user_data(val.clone());
            s.pop_layer();
            val.state.draw(s)
        })
        .button("Finish", Cursive::quit)
    )
}

pub fn draw_options(siv: &mut Cursive) {
    let val = siv.user_data::<UserData>().unwrap().clone();
    let options = val.state.options();
    let title = val.state.clone();
    
    let mut select = SelectView::new().h_align(HAlign::Center).autojump();

    select.add_all_str(options);
    select.set_on_submit(move |s: &mut Cursive, item: &str| {
        let mut val = s.user_data::<UserData>().unwrap().clone();
        val.state = val.state.next();
        val.cock.from_str_part(title.as_str(), item);
        s.set_user_data(val.clone());
        s.pop_layer();
        val.state.draw(s)
    });

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

    siv.add_layer(Dialog::around(
        select.scrollable().fixed_size((40, 20))).title(title.as_str())
        .button("Prev", | s | {
            let mut val = s.user_data::<UserData>().unwrap().clone();
            val.state = val.state.prev();
            s.set_user_data(val.clone());
            s.pop_layer();
            val.state.draw(s)
        })
    );
}

pub fn draw_id(siv: &mut Cursive) {
    let options = ID::get_variants();
    let mut select = SelectView::new().h_align(HAlign::Center).autojump();
    select.add_all_str(options);
    select.set_on_submit(move |s: &mut Cursive, item: &str| {
        if item == "User" {
            s.add_layer(
                Dialog::around(
                    cursive::views::EditView::new()
                        .on_submit(|s, username| {
                            let username = username.clone().to_string();
                            s.pop_layer();
                            s.add_layer(
                                Dialog::around(
                                    cursive::views::EditView::new()
                                        .on_submit(move |s, discord_username| {
                                            let mut val = s.user_data::<UserData>().unwrap().clone();
                                            val.user = ID::User(InnerUser {
                                                name: username.clone(),
                                                discord_name: discord_username.to_string(),
                                            });
                                            val.state = val.state.next();
                                            s.set_user_data(val.clone());
                                            s.pop_layer();
                                            val.state.draw(s);
                                        })
                                ).title("Input Discord Username")
                            );
                        })
                ).title("Input Username")
            );
        } else {
            // For anonymous user, just advance to the next state
            let mut val = s.user_data::<UserData>().unwrap().clone();
            val.user = ID::Anonymous;
            val.state = val.state.next();
            s.set_user_data(val.clone());
            s.pop_layer();
            val.state.draw(s)
        }
    });
    siv.add_layer(Dialog::around(select).title("ID Type"));
}

/// todo below
/// function should act similarly to draw_options, however,
/// it should allow for manual input of the data after selecting
/// the `SizeType` which should be either `Inches` or `Centimeters`.
/// The use should then be prompted for the data in the form of a
/// float, which should then be stored in the UserData struct under
/// the `cock` field using
pub fn draw_size(siv: &mut Cursive) {
    // placeholder functionality
    let val = siv.user_data::<UserData>().unwrap().clone();

    siv.add_layer(
        Dialog::text(format!("{:#?}", val))
        .button("Next", | s | {
            let mut val = s.user_data::<UserData>().unwrap().clone();
            val.state = val.state.next();
            s.set_user_data(val.clone());
            s.pop_layer();
            val.state.draw(s)
        })
        .button("Finish", Cursive::quit)
    )
}

/// todo below
/// function should act similarly to draw_options, however,
/// it should allow for manual input of the data when the
/// variant selected is "Other"
pub fn draw_manual_options(siv: &mut Cursive) {
    // placeholder functionality
    let val = siv.user_data::<UserData>().unwrap().clone();

    siv.add_layer(
        Dialog::text(format!("{:#?}", val))
        .button("Next", | s | {
            let mut val = s.user_data::<UserData>().unwrap().clone();
            val.state = val.state.next();
            s.set_user_data(val.clone());
            s.pop_layer();
            val.state.draw(s)
        })
        .button("Finish", Cursive::quit)
    )
}
