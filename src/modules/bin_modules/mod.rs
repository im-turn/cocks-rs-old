use crate::{CockStruct, ID, };

pub mod standard_prompt;
pub mod tui_prompt;

#[derive(Debug, Clone)]
pub struct UserData {
    pub user: ID,
    pub cock: CockStruct,
    pub state: AppState
}

impl UserData {
    pub fn default() -> UserData {
        UserData {
            user: ID::Anonymous,
            cock: CockStruct::default(),
            state: AppState::default()
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AppState {
    Home,
    Id,
    Abnormalities,
    Aesthetic,
    Balls,
    Circumcision,
    Curvature,
    Shape,
    Size,
    Veininess,
    Result,
}

impl AppState {
    pub fn default() -> AppState {
        AppState::Home
    }

    pub fn as_str(&self) -> &str {
        match self {
            AppState::Home => "Home",
            AppState::Id => "Id",
            AppState::Abnormalities => "Abnormalities",
            AppState::Aesthetic => "Aesthetic",
            AppState::Balls => "Balls",
            AppState::Circumcision => "Circumcision",
            AppState::Curvature => "Curvature",
            AppState::Shape => "Shape",
            AppState::Size => "Size",
            AppState::Veininess => "Veininess",
            AppState::Result => "Result",
        }
    }

    pub fn next(&self) -> AppState {
        match self {
            AppState::Home => AppState::Id,
            AppState::Id => AppState::Size,
            AppState::Size => AppState::Abnormalities,
            AppState::Abnormalities => AppState::Aesthetic,
            AppState::Aesthetic => AppState::Balls,
            AppState::Balls => AppState::Circumcision,
            AppState::Circumcision => AppState::Curvature,
            AppState::Curvature => AppState::Shape,
            AppState::Shape => AppState::Veininess,
            AppState::Veininess => AppState::Result,
            AppState::Result => AppState::Result,
        }
    }

    pub fn prev(&self) -> AppState {
        match self {
            AppState::Home => AppState::Home,
            AppState::Id => AppState::Home,
            AppState::Size => AppState::Id,
            AppState::Abnormalities => AppState::Size,
            AppState::Aesthetic => AppState::Abnormalities,
            AppState::Balls => AppState::Aesthetic,
            AppState::Circumcision => AppState::Balls,
            AppState::Curvature => AppState::Circumcision,
            AppState::Shape => AppState::Curvature,
            AppState::Veininess => AppState::Shape,
            AppState::Result => AppState::Veininess,
        }
    }
}
