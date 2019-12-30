mod media_file;
mod ffmpeg_wrapper;

use std::path::PathBuf;
use std::error::Error;
use structopt::StructOpt;
use crate::media_file::{media_files, MediaFile};
use walkdir::DirEntry;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Cli {
    #[structopt(short, long, parse(from_os_str))]
    pub directories: Vec<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");
    let cli = Cli::from_args();
    let entries: Vec<_> = cli.directories
        .iter()
        .map(media_files)
        .flatten()
        .collect();

    println!("{:?}", entries);
    Ok(())
}
