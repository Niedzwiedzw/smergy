use std::path::{Path, PathBuf};
use std::iter::Iterator;
use std::fmt::Write;

use walkdir::{WalkDir, DirEntry};
use chrono::{
    NaiveDateTime,
    Duration,
};

use std::fs::{copy, create_dir};

use crate::ffmpeg_wrapper::{Ffmpeg, FfmpegFileData};
use crate::devices::{DeviceDatetimeGetter, device_metadata_parsers};
use serde::export::fmt::{Display, Error};
use serde::export::Formatter;

pub static SUPPORTED_AUDIO: [&'static str; 1] = [
    "wav",
];

pub static SUPPORTED_VIDEO: [&'static str; 1] = [
    "mp4",
];

pub fn supported_extensions() -> impl Iterator<Item = &'static &'static str> {
    SUPPORTED_AUDIO.iter().chain(SUPPORTED_VIDEO.iter())
}

pub enum MediaType {
    Audio,
    Video,
}

pub fn duration_pretty(duration: Duration) -> Option<String> {
    let mut s = duration.num_seconds();
    let mut out = String::new();
    if s >= 86400 {
        write!(out, "{}d", s / 86400).ok()?;
        s %= 86400;
    }

    if s >= 3600 {
        write!(out, "{}h", s / 3600).ok()?;
        s %= 3600;
    }

    if s >= 60 {
        write!(out, "{}m", s / 60).ok()?;
        s %= 60
    }

    write!(out, "{}s", s).ok()?;
    Some(out)
}

pub fn media_files<T: AsRef<Path>>(directory: &T) -> Vec<MediaFile> {
    WalkDir::new(directory)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter_map(|e| MediaFile::from_entry(e))
        .collect()
}

#[derive(Debug, Clone)]
pub struct MediaFile {
    pub entry: DirEntry,
    ffmpeg_metadata: Option<FfmpegFileData>,
}

pub fn get_tmp_entry(name: String) -> Option<PathBuf> {
    let path = PathBuf::from(format!("/tmp/{}/", name));
    create_dir(&path);
    if !path.exists() { return None }
    Some(path)
}

impl MediaFile {
    pub fn from_entry(entry: DirEntry) -> Option<Self> {
        let mut media_file = MediaFile { entry, ffmpeg_metadata: None };
        if !media_file.pre_validate() {
            return None
        }

        media_file.ffmpeg_metadata = Ffmpeg::media_file_metadata_raw(media_file.entry.path());

        if !media_file.validate() {
            return None
        }

        Some(media_file)
    }

    pub fn ffmpeg_data_raw(&self) -> Option<&FfmpegFileData> {
        self.ffmpeg_metadata.as_ref()
    }

    pub fn extension(&self) -> Option<String> {
        Some(self.entry.path().extension()?.to_str()?.to_lowercase())
    }

    pub fn base_name(&self) -> Option<String> {
        Some(self.filename().replace(&self.extension()?, ""))
    }

    fn starts_during(&self, other: &Self) -> Option<bool> {
        Some(other.start()? <= self.start()? && self.start()? <= other.end()?)
    }

    fn ends_during(&self, other: &Self) -> Option<bool> {
        Some(other.start()? <= self.end()? && self.end()? <= other.end()?)
    }

    fn contains(&self, other: &Self) -> bool {
        other.starts_during(self).unwrap_or(false) && other.ends_during(self).unwrap_or(false)
    }

    pub fn contained(&self, files: &Vec<Self>) -> Vec<Self> {
        files
            .iter()
            .filter(|other| self.contains(other))
            .map(|other| other.clone())
            .collect()
    }

    fn overlaps_option(&self, other: &Self) -> Option<bool> {
        Some(self.starts_during(other)? || other.starts_during(self)?)
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.overlaps_option(other).unwrap_or(false)
    }

    pub fn overlapping(&self, files: &Vec<Self>) -> Vec<Self> {
        files
            .iter()
            .filter(|other| self.overlaps(other))
            .map(|other| other.clone())
            .collect()
    }

    fn pre_validate(&self) -> bool {
        let meta = match self.entry.metadata() {
            Ok(meta) => meta,
            Err(_) => return false,
        };
        let ext = match self.extension() {
            Some(e) => e,
            None => return false,
        };

        meta.is_file() && supported_extensions().any(|e| e == &ext)
    }

    fn validate(&self) -> bool {
        self.duration_ts().is_some()
    }

    pub fn media_type(&self) -> Option<MediaType> {
        let ext = &self.extension()?;
        let ext = &ext.as_str();

        if SUPPORTED_AUDIO.contains(ext) {
            return Some(MediaType::Audio)
        }
        if SUPPORTED_VIDEO.contains(ext) {
            return Some(MediaType::Video)
        }
        None
    }

    fn duration_ts(&self) -> Option<u64> { // this is useless: https://stackoverflow.com/questions/43333542/what-is-video-timescale-timebase-or-timestamp-in-ffmpeg/43337235#43337235
        self.ffmpeg_data_raw()?.streams.iter().filter_map(|s| s.duration_ts).max()
    }

    fn duration_raw_microseconds(&self) -> Option<i64> {
        self.ffmpeg_data_raw()?
            .streams
            .iter()
            .filter_map(|s| s.duration.clone())
            .filter_map(|s| s.replace("\"", "").parse::<f64>().ok())
            .map(|v| (v*1000000.) as i64)
            .max()
    }

    pub fn duration(&self) -> Option<Duration> {
        Some(Duration::microseconds(self.duration_raw_microseconds()?))
    }

    pub fn duration_pretty(&self) -> Option<String> {
        duration_pretty(self.duration()?)
    }

    pub fn end(&self) -> Option<NaiveDateTime> {
        device_metadata_parsers()
            .iter()
            .filter_map(|f| f(self))
            .next()
    }

    pub fn start(&self) -> Option<NaiveDateTime> {
        Some(self.end()? - self.duration()?)
    }

    pub fn start_pretty(&self) -> Option<String> {
        Some(self.start()?.format("%Y-%m-%d %H:%M:%S").to_string())
    }

    pub fn filename(&self) -> String {
        self.entry.file_name().to_os_string().into_string().expect("Failed to decode OS string")
    }

    pub fn full_path(&self) -> &str {
        self.entry.path().to_str().unwrap_or("UNKNOWN_PATH")
    }

    pub fn cli_friendly_path(&self) -> String {
        self.full_path().clone().replace(" ", "\\ ")
    }

    pub fn tmp_entry(&self) -> Option<PathBuf> {
        let mut entry = get_tmp_entry(self.base_name()?)?;
        entry.push(PathBuf::from(self.filename()));
        Some(entry)
    }

    pub fn make_copy(&self) -> Option<Self> {
        let copy_path = self.tmp_entry()?;
        if !copy_path.exists() {
            println!("# copying {}...", self.filename());
            copy(self.entry.path(), copy_path).ok()?;
        }
        media_files(&self.tmp_entry()?.parent()?).into_iter().next()
    }
}

impl Display for MediaFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "[({}): \"{}\" ({})]",
            self.start_pretty().unwrap_or(String::from("unknown")),
            self.full_path(),
            self.duration_pretty().unwrap_or(String::from("unknown")),
        )
    }
}
