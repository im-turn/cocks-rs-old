use cock_tier::{
    Abnormalities,
    Aesthetic,
    Balls,
    CockStruct,
    FromString,
    GetVariants,
    Curvature,
    CockHandler,
    ID,
};

use cursive::align::HAlign;
use cursive::event::EventResult;
use cursive::traits::*;
use cursive::views::{Dialog, OnEventView, SelectView};
use cursive::Cursive;

#[derive(Debug, Clone, Copy)]
enum AppState {
    Home,
    Aesthetic,
    Balls,
    Curvature,
    Abnormalities,
    Done
}

impl AppState {
    pub fn as_str(&self) -> &str {
        match self {
            AppState::Home => "Home",
            AppState::Abnormalities => "Abnormalities",
            AppState::Aesthetic => "Aesthetic",
            AppState::Balls => "Balls",
            AppState::Curvature => "Curvature",
            AppState::Done => "Done",
        }
    }
}

fn main() {
    let mut siv = cursive::default();

    siv.set_user_data((CockStruct::default(), AppState::Home));

    display(&mut siv);

    siv.run();
}

fn display(siv: &mut Cursive) {
    let app_state = siv.user_data::<(CockStruct, AppState)>().unwrap().1.clone();
    match app_state {
        AppState::Home => display_home(siv),
        AppState::Aesthetic => display_options::<Aesthetic>(siv, app_state.as_str()),
        AppState::Balls => display_options::<Balls>(siv, app_state.as_str()),
        AppState::Curvature => display_options::<Curvature>(siv, app_state.as_str()),
        AppState::Abnormalities => display_options::<Abnormalities>(siv, app_state.as_str()),
        AppState::Done => display_result(siv),
    };
}

fn display_home(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::text("Welcome to Turn's Cock Tier Evaluator")
        .button("Begin", | s | {
            let val = s.user_data::<(CockStruct, AppState)>().unwrap().0.clone();
            let state = AppState::Aesthetic;
            s.set_user_data((val, state));
            s.pop_layer();
            display(s)
        })
    )
}

fn display_result(siv: &mut Cursive) {
    
    let cock = siv.user_data::<(CockStruct, AppState)>().unwrap().0.clone();
    let handler = CockHandler::new(ID::Anonymous, cock);

    siv.add_layer(
        Dialog::text(format!("{handler}"))
        .title("COCK RESULTS")
        .button("Finish", Cursive::quit)
    )
}

fn display_options<T: GetVariants>(siv: &mut Cursive, text: &str) {
    siv.add_layer(
        Dialog::text(format!("Select an option for {text} then continue."))
            .title("Cock Evaluator")
            .button(format!("{text} options"), |s| {
                prompt_opts(s, T::get_variants(), "Options")
            })
            .button("Current Cock", move |s| {
                let (val, _) = s.user_data::<(CockStruct, AppState)>().unwrap().clone();
                s.add_layer(
                    Dialog::info(format!("Current Cock Stats\n{}", val))
                );
            })
            .button("Next", |s| {
                let (val, mut state) = s.user_data::<(CockStruct, AppState)>().unwrap().clone();
                match state {
                    AppState::Home => {state = AppState::Aesthetic},
                    AppState::Aesthetic => {state = AppState::Abnormalities;},
                    AppState::Abnormalities => {state = AppState::Balls;},
                    AppState::Balls => {state = AppState::Curvature;},
                    AppState::Curvature => {state = AppState::Done;},
                    AppState::Done => {state = AppState::Done},
                };
                s.set_user_data((val, state));
                s.pop_layer();
                display(s);
            })
            .button("Back", |s| {
                let (val, mut state) = s.user_data::<(CockStruct, AppState)>().unwrap().clone();
                match state {
                    AppState::Aesthetic => {state = AppState::Home},
                    AppState::Abnormalities => {state = AppState::Aesthetic;},
                    AppState::Balls => {state = AppState::Abnormalities;},
                    AppState::Curvature => {state = AppState::Balls;},
                    AppState::Done => {state = AppState::Curvature;},
                    _ => ()
                };
                s.set_user_data((val, state));
                s.pop_layer();
                display(s);
            })
            .button("Quit", Cursive::quit),
    );
}

fn prompt_opts(siv: &mut Cursive, options: Vec<String>, title: &str) {
    let mut select = SelectView::new().h_align(HAlign::Center).autojump();

    select.add_all_str(options);
    select.add_item_str("BACK");
    select.set_on_submit(get_choice);

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

    siv.add_layer(Dialog::around(select.scrollable().fixed_size((40, 20))).title(title));
}

fn get_choice(siv: &mut Cursive, option: &str) {
    let (mut data, app_state) = siv.user_data::<(CockStruct, AppState)>().unwrap().clone();
    match option {
        "BACK" => siv.pop_layer(),
        _ => {
            match app_state {
                AppState::Aesthetic => {data.aesthetic = Aesthetic::from_string(option);},
                AppState::Abnormalities => {data.abnormalities = Abnormalities::from_string(option);},
                AppState::Balls => {data.balls = Balls::from_string(option);},
                AppState::Curvature => {data.curvature = Curvature::from_string(option);},
                _ => (),
            };
            siv.set_user_data((data, app_state));
            siv.pop_layer()
        }
    };
}
