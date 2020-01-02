#![feature(box_syntax)]

mod media_file;
mod ffmpeg_wrapper;
mod devices;
mod select;

use std::path::PathBuf;
use std::fmt::Write;
use std::error::Error;
use structopt::StructOpt;
use crate::media_file::{media_files, MediaFile};
use crate::ffmpeg_wrapper::Ffmpeg;
use crate::select::Select;

#[derive(StructOpt, Debug)]
#[structopt(name = "smergy")]
struct Cli {
    #[structopt(short, long, parse(from_os_str))]
    pub directories: Vec<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = Cli::from_args();
    let entries = cli.directories
        .iter()
        .map(media_files)
        .flatten();
    println!("FFMPEG version found: {}\n", Ffmpeg::version().unwrap());
    for (video, audios) in Select::candidates(&cli.directories) {
        let video = video.make_copy().unwrap();
        println!("\n# {}", video);
        let command = String::from("reaper");
        println!("{} \"{}\"", command, video.cli_friendly_path());
        for audio in audios {
            let audio = audio.make_copy().unwrap();
            println!("{} \"{}\"", command, audio.cli_friendly_path());
        }
    }
    Ok(())
}
