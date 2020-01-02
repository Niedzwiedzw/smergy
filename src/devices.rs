use crate::media_file::MediaFile;
use chrono::{DateTime, Utc, NaiveDateTime, Duration};

pub type DeviceDatetimeGetter = fn(&MediaFile) -> Option<NaiveDateTime>; // returns creation time

fn android_10(device: &MediaFile) -> Option<NaiveDateTime> {
    let creation = device
        .ffmpeg_data_raw()?
        .format.as_ref()?
        .tags.as_ref()?
        .creation_time.as_ref()?;
    Some(NaiveDateTime::parse_from_str(creation.as_str(), "%FT%T%.fZ").ok()? + Duration::hours(1))
}

fn filesystem(device: &MediaFile) -> Option<NaiveDateTime> {
    match device.entry.metadata().ok()?.created() {
        Ok(time) => Some(NaiveDateTime::from_timestamp(time.elapsed().ok()?.as_secs() as i64, 0)),
        Err(_) => None,
    }
}

fn zoom_h6(device: &MediaFile) -> Option<NaiveDateTime> {
    let tags = device.ffmpeg_data_raw().as_ref()?.format.as_ref()?.tags.as_ref()?;
    let date = tags.date.as_ref()?;
    let time = tags.creation_time.as_ref()?;
    let datetime = format!("{} {}", date, time); // example: 2019-12-07-15:03:44
    NaiveDateTime::parse_from_str(
        datetime.as_str().trim(),
        "%F %T",
    ).ok()
}

pub fn device_metadata_parsers() -> Vec<DeviceDatetimeGetter> {
    vec![
        android_10,
        zoom_h6,
        filesystem, // this should go last as it's the least informative one (works only for media created on your machine)
    ]
}