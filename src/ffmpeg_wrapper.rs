use std::path::{Path, PathBuf};
use std::process::Command;
use std::ffi::OsStr;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};

use serde::{Serialize, Deserialize};
use serde_json::{Result as SerdeResult};

static FPROBE_COMMAND: (&'static str, [&'static str; 6]) = ("ffprobe", ["-v", "quiet", "-print_format", "json", "-show_format", "-show_streams"]);
static VERSION_COMMAND: (&'static str, [&'static str; 1]) = ("ffprobe", ["-version"]);

pub struct Ffmpeg {}

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

    pub fn media_file_metadata_raw<T: AsRef<Path>>(file: T) -> Option<RawFfmpegFileData> {
        let (base, args) = FPROBE_COMMAND;
        let name = file.as_ref().to_str()?.clone().to_owned();
        let mut args = args.to_vec();
        args.push(file.as_ref().to_str()?);
        match cmd(base, args) {
            Some(json) => json.try_into().ok(),
            None => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RawStream {
    pub index: u32,
    pub codec_name:String,
    pub codec_long_name:String,
    pub codec_type:String,
    pub codec_time_base:String,
    pub codec_tag_string:String,
    pub codec_tag:String,
    pub sample_fmt:String,
    pub sample_rate:String,
    pub channels: u16,
    pub bits_per_sample: u16,
    pub r_frame_rate:String,
    pub avg_frame_rate:String,
    pub time_base:String,
    pub duration_ts: u64,
    pub duration: f64,
    pub bit_rate: u64,
    pub bits_per_raw_sample: u16,
    pub disposition: HashMap<String, u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RawFormat {
    pub filename:String,
    pub nb_streams: u16,
    pub nb_programs: u16,
    pub format_name:String,
    pub format_long_name:String,
    pub duration:String,
    pub size:String,
    pub bit_rate:String,
    pub probe_score: u32,
    pub tags: RawFormatTags,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RawFormatTags {
    pub encoded_by:String,
    pub originator_reference:String,
    pub date:String,
    pub creation_time:String,
    pub time_reference:String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RawFfmpegFileData {
    pub streams: Vec<RawStream>,
    pub format: RawFormat,
}

impl TryFrom<String> for RawFfmpegFileData {
    type Error = serde_json::Error;
    fn try_from(value: String) -> SerdeResult<Self> {
        serde_json::from_str(value.as_str())
    }
}
