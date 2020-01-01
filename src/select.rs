use crate::media_file::{MediaFile, media_files, MediaType};
use std::path::Path;

pub struct Select {}
pub type VideoAudioGroup = (MediaFile, Vec<MediaFile>);

impl Select {
    fn media_files<T: AsRef<Path>>(directories: Vec<T>) -> Vec<MediaFile> {
        directories
            .iter()
            .map(media_files)
            .flatten()
            .into_iter()
            .collect()
    }

    /// returns tuples of (VideoFiles, AudioFiles) who have some overlap
    fn media_files_by_type(media_files: Vec<MediaFile>) -> (Vec<MediaFile>, Vec<MediaFile>) {
        let mut video = vec![];
        let mut audio = vec![];
        for file in media_files {
            let media_type = match file.media_type() {
                Some(t) => t,
                None => { continue; }
            };
            match media_type {
                MediaType::Video => video.push(file),
                MediaType::Audio => audio.push(file),
            }
        }
        (video, audio)
    }

    pub fn grouped_media<T: AsRef<Path>>(directories: Vec<T>) -> Vec<VideoAudioGroup> {
        let all_media: Vec<MediaFile> = Self::media_files(directories);
        let (audio_files, video_files): (Vec<MediaFile>, Vec<MediaFile>) = Self::media_files_by_type(all_media);
        video_files
            .into_iter()
            .map(|v| (
                v,
                audio_files
                    .into_iter()
                    .map(|a| a.clone())
                    .filter(|a| v.overlaps(&a)).collect(),
            )).collect()
    }
}
