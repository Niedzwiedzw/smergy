use std::path::{Path, PathBuf};
use std::str::FromStr;
use walkdir::{WalkDir, DirEntry};
use std::iter::Iterator;

pub static SUPPORTED_AUDIO: [&'static str; 1] = [
    "wav",
];

pub static SUPPORTED_VIDEO: [&'static str; 1] = [
    "mp4",
];

pub fn supported_extensions() -> impl Iterator<Item = &'static &'static str> {
    SUPPORTED_AUDIO.iter().chain(SUPPORTED_VIDEO.iter())
}

pub fn media_files<T: AsRef<Path>>(directory: &T) -> Vec<MediaFile> {
    WalkDir::new(directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter_map(|e| MediaFile::from_entry(e))
        .collect()
}

#[derive(Debug)]
pub struct MediaFile(DirEntry);

impl MediaFile {
    pub fn from_entry(entry: DirEntry) -> Option<Self> {
        let media_file = MediaFile(entry);
        if media_file.validate() {
            Some(media_file)
        } else {
            None
        }
    }

    fn validate(&self) -> bool {
        let meta = match self.0.metadata() {
            Ok(meta) => meta,
            Err(_) => return false,
        };
        let ext = match self.0.path().extension() {
            Some(e) => e.to_str().unwrap(),
            None => "UNSUPPORTED",
        };
        meta.is_file()
            && supported_extensions().any(|e| e == &ext.to_lowercase())
    }
}
