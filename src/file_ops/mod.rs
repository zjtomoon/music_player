use std::fs::read_dir;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use std::time::Duration;

use audiotags::{AudioTag,Tag};
use rodio::decoder::DecoderError;
use rodio::Decoder;
use rodio::Source;

use crate::app::App;
use crate::utils::split_path::split_path_to_name;

pub enum DirectoryItem {
    File(String),
    Directory(String),
}

pub struct Audio {
    pub artist:String,
    pub title:String,
    pub album:String,
    pub duration:Duration,
}

impl Audio {
    fn new(tag:Box<dyn AudioTag>,duration:Duration) -> Audio {

    }
}