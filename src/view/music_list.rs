use tui::backend::Backend;
use tui::layout::{Alignment,Constraint,Direction,Layout,Rect};
use tui::style::{Color,Style};
use tui::text::{Span,Spans,Text};
use tui::widgets::{Block,BorderType,Borders,Paragraph};
use tui::Frame;

use crate::file_ops::DirectoryItem;
use crate::utils::split_path::split_path_to_name;

use super::color::Theme;
use super::display::Display;

pub fn draw_music_list<B:Backend>(
    frame:&mut Frame<B>,
    area:Rect,
    theme:&Theme,
    window_height:usize,
    files:&Vec<DirectoryItem>,
    selected_index:&Option<usize>,
    search_string:&str,
    command_string:&str,
    error:&Option<String>,
) {
    let selected_index = match selected_index {
        Some(index) => *index,
        None => 0,
    };

    let display = Display::new(window_height,files.len(),selected_index);
    let mut music_names:Vec<Spans> = Vec::new();

    //list block
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(Spans::from(vec![
            Span::styled("Music list",Style::default().fg(theme.list_title_color)),
            Span::styled(
                format!("Page: {}/{}",display.page.0,display.page.1),
                Style::default().fg(theme.list_title_page_color),
            ),
        ]))
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(theme.list_border_color));
    frame.render_widget(block,area);

    // init music list
    if files.len() > 0 {

    }
}