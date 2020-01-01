#![feature(box_syntax)]

mod media_file;
mod ffmpeg_wrapper;
mod devices;
mod select;

use std::path::PathBuf;
use std::error::Error;
use structopt::StructOpt;
use crate::media_file::{media_files, MediaFile};
use crate::ffmpeg_wrapper::Ffmpeg;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
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
    for entry in entries.into_iter() {
        let entry: MediaFile = entry.clone();
        let length = entry.duration_pretty().expect("cannot find duration");
        println!(
            "{}[{}] ~~~~ {} - {}",
            entry.filename(),
            length,
            entry.start().expect("cannot decode creation time"),
            entry.end().unwrap(),
        );
    }
    Ok(())
}
