use std::io::{self, Stdout};
use std::path::{self, PathBuf};
use std::time::{Duration, Instant};

use exitfailure::ExitFailure;
use rand::prelude::SliceRandom;
use rodio::{OutputStreamHandle, Sink};
use tui::backend::CrosstermBackend;
use tui::Terminal;

use crate::file_ops::{self, get_audio_source, DirectoryItem};
use crate::commands::process_command;
use crate::music::Music;


#[derive(PartialEq)]
pub enum Mode {
    Browse,
    Search,
    Command,
}

pub enum PlayStyle {
    PlayOrder,
    SingleCycle,
}

pub struct App<'a> {
    pub terminal: &'a Terminal<CrosstermBackend<Stdout>>,
    pub selection_index: Option<usize>,
    pub current_directory: path::PathBuf,
    pub directory_contents: Vec<DirectoryItem>,
    pub search_buffer: Vec<char>,
    pub command_buffer: Vec<char>,
    pub error: Option<String>,
    pub window_height: u16,
    pub play_music_list: Vec<Music>,
    pub playing_music: Option<Music>,
    pub stream_handle: OutputStreamHandle,
    pub player: Sink,
    pub mode: Mode,
    pub play_style: PlayStyle,

    max_file_selection: usize,
}

impl<'a> App<'a> {
    pub fn new(
        terminal: &'a mut Terminal<CrosstermBackend<Stdout>>,
        music_database: &str,
        stream_handle: OutputStreamHandle,
    ) -> Result<App<'a>, ExitFailure> {
        let window_height = terminal.size().unwrap().height - 5;
        let current_directory = path::PathBuf::from(music_database);

        let player = Sink::try_new(&stream_handle)?;

        let mut app = App {
            terminal,
            selection_index: None,
            current_directory,
            directory_contents: Vec::new(),
            search_buffer: Vec::new(),
            command_buffer: Vec::new(),
            error: None,
            window_height,
            play_music_list: Vec::new(),
            playing_music: None,
            stream_handle,
            player,
            mode: Mode::Browse,
            play_style: PlayStyle::PlayOrder,
            max_file_selection: 0,
        };

        // todo: populate_files()

        Ok(app)
    }


    fn new_sink(&mut self) -> Result<(), ExitFailure> {
        self.player = Sink::try_new(&self.stream_handle)?;
        Ok(())
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn set_play_style(&mut self, style: PlayStyle) {
        self.play_style = style;
    }

    pub fn add_to_search_buffer(&mut self, char: char) {
        self.search_buffer.push(char);
    }

    pub fn add_to_command_buffer(&mut self, char: char) {
        self.command_buffer.push(char);
    }

    pub fn update_window_height(&mut self) {
        self.window_height = self.terminal.size().unwrap().height - 5;
    }

    pub fn populate_files(&mut self) -> Result<(), io::Error> {
        let mut dir_items = file_ops::get_files_for_current_directory(self)?;
        dir_items.sort(); //todo :? 

        self.directory_contents = dir_items;
        self.max_file_selection = self.directory_contents.len();

        if self.max_file_selection == 0 {
            self.selection_index = None;
        } else {
            self.selection_index = Some(0);
        }

        Ok(())
    }

    pub fn populate_search_file(&mut self, astrict: &str) -> Result<(), io::Error> {
        let mut dir_items = file_ops::get_files_for_current_directory_astrict(self, astrict)?;
        dir_items.sort(); // todo: ?

        self.directory_contents = dir_items;
        self.max_file_selection = self.directory_contents.len();

        if self.max_file_selection == 0 {
            self.selection_index = None;
        } else {
            self.selection_index = Some(0);
        }
        Ok(())
    }

    pub fn get_search_string(&mut self) -> String {
        let mut search_string = String::new();
        for c in &self.command_buffer {
            search_string.push(*c)
        }
        search_string.trim().to_string()
    }

    pub fn get_command_string(&mut self) -> String {
        let mut command_string = String::new();
        for c in &self.command_buffer {
            command_string.push(*c);
        }
        command_string
    }

    pub fn get_selected_directory_item(&mut self) -> Option<DirectoryItem> {
        if let Some(index) = self.selection_index {
            Some(self.directory_contents[index].clone()) // todo: ? [derive(Clone)]
        } else {
            None
        }
    }

    pub fn move_select_top(&mut self) {
        if let Some(_) = self.selection_index {
            self.selection_index = Some(0);
        }
    }

    pub fn move_select_bottom(&mut self) {
        if let Some(_) = self.selection_index {
            self.selection_index = Some(self.max_file_selection - 1);
        }
    }

    pub fn move_select_up(&mut self, step: i32) {
        if let Some(selection_index) = self.selection_index {
            let diff = selection_index as i32 - step;
            if diff > 0 {
                self.selection_index = Some(diff as usize);
            } else {
                self.selection_index = Some(0);
            }
        }
    }

    pub fn move_select_down(&mut self, step: usize) {
        if let Some(selection_index) = self.selection_index {
            if selection_index + step < self.max_file_selection {
                self.selection_index = Some(selection_index + step);
            } else {
                self.selection_index = Some(self.max_file_selection - 1);
            }
        }
    }

    pub fn next_page(&mut self) {
        if let Some(selection_index) = self.selection_index {
            let height = self.window_height;
            // show items length
            let display_pages = ((self.directory_contents.len() as f32) / (height as f32)).ceil() as usize;

            if display_pages > 1 {
                let mut page = 0;
                for i in 0..display_pages {
                    if selection_index < (i + 1) * height as usize {
                        page = i + 1;
                        break;
                    }
                }

                if page < display_pages {
                    self.selection_index = Some(page * height as usize)
                }
            }
        }
    }

    pub fn previous_page(&mut self) {
        if let Some(selection_index) = self.selection_index {
            let height = self.window_height;
            let display_pages = ((self.directory_contents.len() as f32) / (height as f32)).ceil() as usize;

            if display_pages > 1 {
                let mut page = 0;
                for i in 0..display_pages {
                    if selection_index < (i + 1) * height as usize {
                        page = i + 1;
                        break;
                    }
                }
                if page > 1 {
                    self.selection_index = Some(selection_index - height as usize)
                }
            }
        }
    }

    pub fn open_folder(&mut self) {
        if let Some(selection_index) = self.selection_index {
            match &self.directory_contents[selection_index] {
                DirectoryItem::File(_) => {}
                DirectoryItem::File(path) => {
                    let previous_dir = self.current_directory.clone();
                    self.current_directory.push(path);
                    if let Err(err) = self.populate_files() {
                        self.current_directory = previous_dir;
                        self.error = Some(err.to_string());
                    } else {
                        if self.max_file_selection == 0 {
                            self.selection_index = None;
                        } else {
                            self.selection_index = Some(0);
                        }
                    };
                }
                _ => {}
            };
        }
    }

    pub fn back_previous_folder(&mut self, music_database: &str) {
        let music_database = PathBuf::from(music_database);
        let current_directory = self.current_directory.clone();

        if !current_directory.eq(&music_database) {
            let mut ancestors = current_directory.ancestors();
            ancestors.next();
            if let Some(previous) = ancestors.next() {
                self.current_directory = PathBuf::from(previous);
                if let Err(err) = self.populate_files() {
                    self.current_directory = current_directory;
                    self.error = Some(err.to_string());
                } else {
                    if self.max_file_selection == 0 {
                        self.selection_index = None;
                    } else {
                        self.selection_index = Some(0);
                    }
                };
            };
        }
    }

    pub fn add_music_to_list(&mut self) {
        match self.get_selected_directory_item() {
            Some(dir_item) => match dir_item {
                DirectoryItem::File(path) => {
                    match Music::new(&path) {
                        Ok(music) => self.play_music_list.push(music),
                        Err(err) => self.error = Some(err),
                    };
                }
                DirectoryItem::Directory(_) => self.error =Some(String::from("Is a directory")),
            }
            None => {}
        };
    }

    pub fn add_all_music_to_list(&mut self) {
        for item in self.directory_contents {
            match item {
                DirectoryItem::File(path) => {
                    match Music::new(&path) {
                        Ok(music) => self.play_music_list.push(music),
                        Err(err) => self.error = Some(err),
                    };
                }
                _ => {}
            }
        }
    }

    pub fn stop_or_start_play(&mut self) {
        if self.player.is_paused() {
            self.player.play();
            if let Some(music) = &mut self.playing_music {
                if let Some(stat_time) = &mut music.start_time {
                    *stat_time = Instant::now() - music.play_position;
                }
            }
        } else {
            self.player.pause();
        }
    }

}