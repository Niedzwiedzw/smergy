use std::path::{Path};
use std::iter::Iterator;
use std::fmt::Write;

use walkdir::{WalkDir, DirEntry};
use chrono::{
    DateTime,
    NaiveDateTime,
    Duration,
    Utc,
    Local,
};

use crate::ffmpeg_wrapper::{Ffmpeg, FfmpegFileData};
use crate::devices::{DeviceDatetimeGetter, device_metadata_parsers};

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

impl MediaFile {
    pub fn from_entry(entry: DirEntry) -> Option<Self> {
        let mut media_file = MediaFile { entry, ffmpeg_metadata: None };
        media_file.ffmpeg_metadata = Ffmpeg::media_file_metadata_raw(media_file.entry.path());
        if media_file.validate() {
            Some(media_file)
        } else {
            None
        }
    }

    pub fn ffmpeg_data_raw(&self) -> Option<&FfmpegFileData> {
        self.ffmpeg_metadata.as_ref()
    }

    pub fn extension(&self) -> Option<String> {
        Some(self.entry.path().extension()?.to_str()?.to_lowercase())
    }

    fn starts_during(&self, other: &Self) -> Option<bool> {
        Some(other.start()? <= self.start()? && self.start()? <= other.end()?)
    }

    fn overlaps_option(&self, other: &Self) -> Option<bool> {
        Some(self.starts_during(other)? || other.starts_during(self)?)
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.overlaps_option(other).unwrap_or(false)
    }

    fn validate(&self) -> bool {
        let meta = match self.entry.metadata() {
            Ok(meta) => meta,
            Err(_) => return false,
        };
        let ext = match self.extension() {
            Some(e) => e,
            None => return false,
        };

        meta.is_file()
            && supported_extensions().any(|e| e == &ext)
            && self.duration_ts().is_some()
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

    fn creation_time(&self, fun: &DeviceDatetimeGetter) -> Option<NaiveDateTime> {
        fun(self)
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

    pub fn filename(&self) -> String {
        self.entry.file_name().to_os_string().into_string().expect("Failed to decode OS string")
    }
}
