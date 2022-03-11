
use exitfailure::ExitFailure;
use tui::layout::{Constraint,Direction,Layout};

use crate::app::App;
use crate::config::InitTheme;


use self::color::Theme;
use self::music_list::draw_music_list;
use self::play_music_list::draw_play_music_list;
use self::playing_music::draw_playing_music;

pub mod color;
mod display;
mod music_list;
mod play_music_list;
mod playing_music;

pub fn handle_theme(init_theme:InitTheme) -> Theme {
    Theme::new(init_theme)
}

pub fn draw(app:&mut App,theme:&Theme) -> Result<(),ExitFailure> {

}