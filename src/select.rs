use crate::media_file::{MediaFile, media_files, MediaType};
use std::path::Path;

pub struct Select {}
pub type VideoAudioGroup = (MediaFile, Vec<MediaFile>);

impl Select {
    fn media_files<T: AsRef<Path>>(directories: &Vec<T>) -> Vec<MediaFile> {
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

    fn grouped_media<T: AsRef<Path>>(directories: &Vec<T>) -> Vec<VideoAudioGroup> {
        let all_media = Self::media_files(directories);
        let (video_files, audio_files ) = Self::media_files_by_type(all_media);
        video_files
            .into_iter()
            .map(|v| (v.clone(), v.overlapping(&audio_files)))
            .collect()
    }

    pub fn candidates<T: AsRef<Path>>(directories: &Vec<T>) -> Vec<VideoAudioGroup> {
        let mut media: Vec<_> = Self::grouped_media(directories)
            .into_iter()
            .filter(|(_v, a)| !a.is_empty())
            .collect();
        media.sort_by(
            |(one, _), (other, __)|
                one.start()
                    .unwrap()
                    .cmp(&other.start().unwrap())
        );
        media
    }
}
