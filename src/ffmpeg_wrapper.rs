use std::path::{Path};
use std::process::Command;
use std::ffi::OsStr;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

use serde::{Serialize, Deserialize};
use serde_json::{Result as SerdeResult};

static FPROBE_COMMAND: (&'static str, [&'static str; 6]) = ("ffprobe", ["-v", "quiet", "-print_format", "json", "-show_format", "-show_streams"]);
static VERSION_COMMAND: (&'static str, [&'static str; 1]) = ("ffprobe", ["-version"]);

pub struct Ffmpeg {}

pub fn empty_vec<T>() -> Vec<T> { vec![] }

pub fn cmd<S, I>(program: &'static str, args: I) -> Option<String>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
{
    let process = Command::new(program).args(args).output().expect(format!("Failed to run {}", program).as_str());
    let output = String::from_utf8_lossy(&process.stdout);
    Some(String::from(output))
}

impl Ffmpeg {
    pub fn version() -> Option<String> {
        let (base, args) = VERSION_COMMAND;
        let out = cmd(base, &args)?;
        let version_num = out.split_whitespace().skip(2).next()?;
        Some(String::from(version_num))
    }

    pub fn media_file_metadata_raw<T: AsRef<Path>>(file: T) -> Option<FfmpegFileData> {
        let (base, args) = FPROBE_COMMAND;
        let mut args = args.to_vec();
        args.push(file.as_ref().to_str()?);
        match cmd(base, args) {
            Some(json) => {
                Some(json.try_into().unwrap())
            },
            None => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RawStream {
    pub index: u32,
    pub codec_name: Option<String>,
    pub codec_long_name: Option<String>,
    pub codec_type: Option<String>,
    pub codec_time_base: Option<String>,
    pub codec_tag_string: Option<String>,
    pub codec_tag: Option<String>,
    pub sample_fmt: Option<String>,
    pub sample_rate: Option<String>, // "44100"
    pub channels: Option<u16>,
    pub bits_per_sample: Option<u16>,
    pub r_frame_rate: Option<String>,
    pub avg_frame_rate: Option<String>,
    pub time_base: Option<String>,
    pub duration_ts: Option<u64>,
    pub duration: Option<String>, // "32.219138"
    pub bit_rate: Option<String>, // "2116800"
    pub bits_per_raw_sample: Option<String>, // "24"
    pub disposition: HashMap<String, u8>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RawFormat {
    pub filename:Option<String>,
    pub nb_streams: Option<u16>,
    pub nb_programs: Option<u16>,
    pub format_name:Option<String>,
    pub format_long_name:Option<String>,
    pub duration:Option<String>,
    pub size:Option<String>,
    pub bit_rate:Option<String>,
    pub probe_score: Option<u32>,
    pub tags: Option<RawFormatTags>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RawFormatTags {
    pub creation_time: Option<String>,
    pub date: Option<String>,
    pub minor_version: Option<String>,
    #[serde(rename = "location_eng")]
    pub location_eng: Option<String>,
    #[serde(rename = "com.android.version")]
    pub com_android_version: Option<String>,
    pub compatible_brands: Option<String>,
    pub location: Option<String>,
    #[serde(rename = "com.android.capture.fps")]
    pub com_android_capture_fps: Option<String>,
    pub major_brand: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FfmpegFileData {
    #[serde(default = "empty_vec")]
    pub streams: Vec<RawStream>,
    pub format: Option<RawFormat>,
}

impl TryFrom<String> for FfmpegFileData {
    type Error = serde_json::Error;
    fn try_from(value: String) -> SerdeResult<Self> {
        serde_json::from_str(value.as_str())
    }
}
