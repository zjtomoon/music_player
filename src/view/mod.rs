
use exitfailure::ExitFailure;
use tui::layout::{Constraint,Direction,Layout};

use crate::app::App;
use crate::config::InitTheme;

pub mod color;
mod display;
mod music_list;
mod play_music_list;

pub fn handle_theme(init_theme:InitTheme) -> Theme {

}