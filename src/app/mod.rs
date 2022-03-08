use std::io::{self, stdout, Stdout};
use std::path::{self,PathBuf};
use std::time::{Duration,Instant};

use exitfailure::ExitFailure;
use rand::prelude::SliceRandom;
use rodio::{OutputStreamHandle,Sink};
use tui::backend::CrosstermBackend;
use tui::Terminal;

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
    pub terminal:&'a Terminal<CrosstermBackend<Stdout>>,
    pub selection_index:Option<usize>,
    pub current_directory:path::PathBuf,
    pub directory_contents:Vec<DirectoryItem>,
    pub search_buffer: Vec<char>,
    pub command_buffer:Vec<char>,
    pub error:Option<String>,
    pub window_height:u16,
    pub play_music_list:Vec<Music>,
    pub playing_music:Option<Music>,
    pub stream_handle:OutputStreamHandle,
    pub player:Sink,
    pub mode:Mode,
    pub play_style:PlayStyle,

    max_file_selection:usize,
}

impl<'a> App<'a> {
    pub fn new(
        terminal:&'a mut Terminal<CrosstermBackend<Stdout>>,
        music_database:&str,
        stream_handle:OutputStreamHandle,
    ) -> Result<App<'a>,ExitFailure> {
        let window_height = terminal.size().unwrap().height - 5;
        let current_directory = path::PathBuf::from(music_database);

        let player = Sink::try_new(&stream_handle)?;

        let mut app = App {
            terminal,
            selection_index:None,
            current_directory,
            directory_contents:Vec::new(),
            search_buffer:Vec::new(),
            command_buffer:Vec::new(),
            error:None,
            window_height,
            play_music_list:Vec::new(),
            playing_music:None,
            stream_handle,
            player,
            mode:Mode::Browse,
            play_style:PlayStyle::PlayOrder,
            max_file_selection:0,
        };

        // todo: populate_files()

        Ok(app)
    }


    fn new_sink(&mut self) ->Result<(),ExitFailure> {
        self.player = Sink::try_new(&self.stream_handle)?;
        Ok(())
    }

    pub fn set_mode(&mut self,mode:Mode) {
        self.mode = mode;
    }

    pub fn set_play_style(&mut self,style:PlayStyle) {
        self.play_style = style;
    }

    pub fn add_to_search_buffer(&mut self,char:char) {
        self.search_buffer.push(char);
    }

    pub fn add_to_command_buffer(&mut self,char:char) {
        self.command_buffer.push(char);
    }

    pub fn update_window_height(&mut self) {
        self.window_height = self.terminal.size().unwrap().height -5;
    }

    pub fn populate_file(&mut self) -> Result<(),io::Error> {
        
    }
}