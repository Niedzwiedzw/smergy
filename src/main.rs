mod media_file;
mod ffmpeg_wrapper;
mod devices;
mod select;
mod daw_support;
pub mod daws;

mod tests;
use daw_support::DAWProjectFile;
use std::path::PathBuf;
use std::error::Error;
use structopt::StructOpt;
use crate::media_file::{media_files};
use crate::ffmpeg_wrapper::Ffmpeg;
use crate::select::Select;
use crate::daws::reaper::{ReaperTrack, Reaper};

#[derive(StructOpt, Debug)]
#[structopt(name = "smergy")]
struct Cli {
    #[structopt(short, long, parse(from_os_str))]
    pub directories: Vec<PathBuf>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli: Cli = Cli::from_args();
    println!("FFMPEG version found: {}\n", Ffmpeg::version().unwrap());
    for (video, audios) in Select::candidates(&cli.directories) {
        let video = video.make_copy().unwrap();
        let audios: Vec<_> = audios.into_iter().map(|a| a.make_copy().unwrap()).collect();
        println!("\n# {}", video);
        let command = String::from("reaper");
        println!("{} \"{}\"", command, video.cli_friendly_path());
//        for audio in audios {
//            println!("{} \"{}\"", command, audio.cli_friendly_path());
//        }

        let tracks: Vec<ReaperTrack> = vec![video].into_iter().chain(audios.into_iter()).map(|i| i.into()).collect();
        let project_file = Reaper::new(tracks, String::from("1578133999"));
        project_file.save();
    }
    Ok(())
}
