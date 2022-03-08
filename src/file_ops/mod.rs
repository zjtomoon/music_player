use std::fs::read_dir;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use std::time::Duration;

use audiotags::{AudioTag, Tag};
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
    pub artist: String,
    pub title: String,
    pub album: String,
    pub duration: Duration,
}

impl Audio {
    fn new(tag: Box<dyn AudioTag>, duration: Duration) -> Audio {
        let artist = match tag.artist() {
            Some(s) => s.to_string(),
            None => "".to_string(),
        };
        let title = match tag.title() {
            Some(s) => s.to_string(),
            None => "".to_string(),
        };
        let album = match tag.album() {
            Some(album) => album.title.to_string(),
            None => "".to_string(),
        };

        Audio {
            artist,
            title,
            album,
            duration,
        }
    }
}


pub fn get_files_for_current_directory(app:&mut App) -> Result<Vec<DirectoryItem>,io::Error> {
    // get list,unwrap,and convert results to &path
    let dir_items:Vec<PathBuf> = match read_dir(&app.current_directory) {
        Ok(val) => val.map(|f| f.unwrap().path()).collect(),
        Err(err) => return Err(err),
    };

    // Convert items to DirectoryItem
    let mut files: Vec<DirectoryItem> = Vec::new();

    // todo: check_audio_file()
    for item in dir_items {
        if item.is_file() {
            if check_audio_file(&item)? {
                let file = DirectoryItem::File(String::from(item.to_string_lossy()));
                files.push(file);
            }
        } else {
            let file = DirectoryItem::Directory(String::from(item.to_string_lossy()));
            files.push(file);
        }
    }
    Ok(files)
}

pub fn get_files_for_current_directory_astrict(
    app:&mut App,
    astrict:&str,
) -> Result<Vec<DirectoryItem>,io::Error> {
    let astrict_low = astrict.to_ascii_lowercase();
    // get list unwrap and convert results to &path
    let dir_items:Vec<PathBuf> = match read_dir(&app.current_directory) {
        Ok(val) => val.map(|f| f.unwrap().path()).collect(),
        Err(err) => return Err(err),
    };


    // Convert items to DirectoryItem
    let mut files:Vec<DirectoryItem> = Vec::new();
    // todo : check_audio_file
    for item in  dir_items {
        let path_string = String::from(item.to_string_lossy());
        if !split_path_to_name(&path_string).to_ascii_lowercase().contains(&astrict_low)
        {
            continue;
        }
        if item.is_file() {
            if check_audio_file(&item)? {
                let file = DirectoryItem::File(path_string)
                files.push(file)
            }
        } else {
            let file = DirectoryItem::Directory(path_string);
            files.push(file);
        }
    }
    Ok(files)
}


pub fn check_audio_file(path:&PathBuf) -> Result<bool,io::Error> {
    if let Some(t) = infer::get_from_path(path.to_str().unwrap())? {
        let mine_tyoe = t.mime_type();
        return Ok(mine_tyoe.contains("audio") || mine_tyoe.contains("video"));
    }
    Ok(false)
}

