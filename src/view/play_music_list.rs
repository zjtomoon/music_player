use tui::widgets::{Block, BorderType, Borders, Paragraph};
use tui::Frame;

use crate::music::Music;

use super::color::Theme;
use tui::backend::Backend;
use tui::layout::Rect;

#[rustfmt::skip]
const MUSIC_PLAYER_PIC: &[&str] = &[
    "███╗   ███╗","██╗   ██╗","███████╗","██╗"," ██████╗", "",
    "████╗ ████║","██║   ██║","██╔════╝","██║","██╔════╝", "",
    "██╔████╔██║","██║   ██║","███████╗","██║","██║     ", "",
    "██║╚██╔╝██║","██║   ██║","╚════██║","██║","██║     ", "",
    "██║ ╚═╝ ██║","╚██████╔╝","███████║","██║","╚██████╗", "",
    "╚═╝     ╚═╝"," ╚═════╝ ","╚══════╝","╚═╝"," ╚═════╝",
];

const CUT_OFF_RULE: &str = "   ";

#[rustfmt::skip]
const USAGE: &[&str] = &[
    "Move selection up              ", "[k, K] ", " ",
    "Move selection down            ", "[j, J] ", " ",
    "Move selection to the top      ", "[g]    ", " ",
    "Move selection to the bottom   ", "[G]    ", " ",
    "Next page                      ", "[n]    ", " ",
    "Previous page                  ", "[N]    ", " ",
    "Open folder                    ", "[l]    ", " ",
    "Back to previous folder        ", "[h]    ", " ",
    "Enter search mode              ", "[|]    ", " ",
    "Enter command mode             ", "[:]    ", " ",
    "Exit program                   ", "[q]    ", " ",
    "Exit search or command mode    ", "[Esc]  ", " ",
    "Pause or resume the music      ", "[Space]", " ",
    "Decrease volume                ", "[-]    ", " ",
    "Increase volume                ", "[+, =] ", " ",
    "Add music to the playlist      ", "[Enter]",
];

pub fn draw_play_music_list<B:Backend>(
    frame:&mut Frame<B>,
    area:Rect,
    theme:&Theme,
    music_list:&Vec<Music>,
    playing_music:&Option<Music>,
    is_paused:bool,
) {

}