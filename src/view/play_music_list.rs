use tui::widgets::{Block, BorderType, Borders, Paragraph};
use tui::Frame;
use tui::backend::Backend;
use tui::layout::{Alignment,Rect,Direction,Layout,Constraint};
use tui::style::{Color,Modifier,Style};
use tui::text::{Span,Spans};

use crate::music::Music;

use super::color::Theme;


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
    let mut all_music_dur:u64 = 0;
    for music in music_list {
        all_music_dur += music.total_duration.as_secs();
    }
    if let Some(playing_music) = playing_music {
        let playing_total_dur = playing_music.total_duration.as_secs();
        let playing_position_dur = playing_music.play_position.as_secs();
        if playing_position_dur <= playing_total_dur {
            all_music_dur += playing_total_dur - playing_position_dur;
        }
    }

    let all_music_dur_str = format!(
        "{}h {:0>2}m {:>2}s",
        (all_music_dur / 60 /60),
        (all_music_dur / 60 % 60),
        (all_music_dur % 60),
    );

    let mut title_spans = Vec::new();
    if music_list.len() > 0 || playing_music != &None {
        let mut total_music = music_list.len();
        if playing_music != &None {
            total_music += 1;
        }

        title_spans.push(Span::styled(
            "Play list",
            Style::default().fg(theme.play_music_list_title_color),
        ));
        title_spans.push(Span::styled(" | ",Style::default().fg(Color::Yellow)));
        title_spans.push(Span::styled(
            format!("{} songs",total_music),
            Style::default().fg(theme.play_music_list_title_color),
        ));
        title_spans.push(Span::styled(" | ",Style::default().fg(Color::Yellow)));
        title_spans.push(Span::styled(
            format!(" {} ",all_music_dur_str),
            Style::default().fg(theme.play_music_list_title_color),
        ));
    };
    // play music list block
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(title_spans)
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(theme.play_music_list_border_color));
    frame.render_widget(block,area);

    // todo: draw_home_page()
}

fn drew_home_page<B:Backend>(frame:&mut Frame<B>,area:&Rect,theme:&Theme) {
    let inner_rect = Rect::new(area.x + 1,area.y + 2,area.width -2,area.height -3);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(6),Constraint::Min(3)])
        .split(inner_rect);

    // Display music picture

}