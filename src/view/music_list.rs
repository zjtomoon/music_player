use tui::backend::Backend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::{Span, Spans, Text};
use tui::widgets::{Block, BorderType, Borders, Paragraph};
use tui::Frame;

use crate::file_ops::DirectoryItem;
use crate::utils::split_path::split_path_to_name;

use super::color::Theme;
use super::display::Display;

pub fn draw_music_list<B: Backend>(
    frame: &mut Frame<B>,
    area: Rect,
    theme: &Theme,
    window_height: usize,
    files: &Vec<DirectoryItem>,
    selected_index: &Option<usize>,
    search_string: &str,
    command_string: &str,
    error: &Option<String>,
) {
    let selected_index = match selected_index {
        Some(index) => *index,
        None => 0,
    };

    let display = Display::new(window_height, files.len(), selected_index);
    let mut music_names: Vec<Spans> = Vec::new();

    //list block
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .title(Spans::from(vec![
            Span::styled("Music list", Style::default().fg(theme.list_title_color)),
            Span::styled(
                format!("Page: {}/{}", display.page.0, display.page.1),
                Style::default().fg(theme.list_title_page_color),
            ),
        ]))
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(theme.list_border_color));
    frame.render_widget(block, area);

    // init music list
    if files.len() > 0 {
        // convert directoryitems to text
        for i in display.from..display.to {
            match &files[i] {
                DirectoryItem::File(path) => {
                    let name = split_path_to_name(path);
                    music_names.push() // todo: get_spans()
                }
            }
        }
    }
}

fn get_spans(icon: String, name: String, icon_color: Color, name_color: Color) -> Spans<'static> {
    Spans::from(vec![
        Span::styled(icon, Style::default().fg(icon_color)),
        Span::styled(format!("{}\n", name), Style::default().fg(name_color)),
    ])
}

fn draw_search<B: Backend>(frame: &mut Frame<B>, area: Rect, theme: &Theme, search_string: &str) {
    let text = Text::from(Spans::from(vec![
        Span::styled("🔍 ", Style::default().fg(theme.search_icon_color)),
        Span::styled(search_string, Style::default().fg(theme.search_font_color)),
    ]));

    let search = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().fg(theme.search_border_color)),
    );

    frame.render_widget(search, area)
}

fn draw_command<B: Backend>(frame: &mut Frame<B>, area: Rect, theme: &Theme, command_string: &str) {
    let text = Text::from(Spans::from(vec![Span::styled(
        command_string,
        Style::default().fg(theme.command_font_color),
    )]));

    let command_paragraph = Paragraph::new(text).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .title(" Command ")
            .style(Style::default().fg(theme.command_border_color)),
    );

    frame.render_widget(command_paragraph, area);
}


fn draw_error<B: Backend>(frame: &mut Frame<B>, area: Rect, error: &str) {

}