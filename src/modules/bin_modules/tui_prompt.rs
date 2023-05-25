use cursive::Cursive;

use crate::{
    TUIDisplay,
    bin_modules::AppState,
    CockHandler,
};


use cursive::views::Dialog;

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

pub fn draw_id(siv: &mut Cursive) {
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

pub fn draw_size(siv: &mut Cursive) {
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

pub fn draw_options(siv: &mut Cursive) {
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

pub fn draw_manual_options(siv: &mut Cursive) {
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
