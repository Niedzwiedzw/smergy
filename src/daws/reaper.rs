pub use crate::daw_support::{DAWProjectFile, Track};
use crate::media_file::{MediaFile, MediaType};

pub struct Reaper {
    tracks: Vec<ReaperTrack>,
    project_id: String,
}

impl Reaper {
    pub fn new(tracks: Vec<ReaperTrack>, project_id: String) -> Self {
        Self { tracks, project_id }
    }

    fn project_id(&self) -> String {
        self.project_id.clone()
    }

    fn track_strings(&self) -> Vec<String> {
        self.tracks.iter().map(|t| t.as_string()).collect()
    }
}

impl DAWProjectFile for Reaper {
    fn project_file(&self) -> String {
        reaper_project_file(self.track_strings(), self.project_id())
    }

    fn filename(&self) -> String {
        let base_name = match self.tracks.first() {
            Some(t) => t.track_name.clone(),
            None => String::from("new-project"),
        };

        let base_name = base_name.replace(".", "");
        format!("{}.rpp", base_name)
    }
}

pub struct ReaperTrack {
    track_id: String,
    track_name: String,
    track_file_name: String,
    track_path: String,
    track_length: String,
    another_id: String,
    item_id: String,
    media_type: MediaType,
}

impl From<MediaFile> for ReaperTrack {
    fn from(media_file: MediaFile) -> Self {
        Self::new(
            media_file.filename(),
            media_file.filename(),
            media_file.filename(),
            String::from(media_file.full_path()),
            format!("{}", media_file.duration().unwrap().num_seconds() + 1),
            String::from(media_file.filename()),
            String::from(media_file.filename()),
            media_file.media_type().expect("Reaper requires media type specifying"),
        )
    }
}

impl ReaperTrack {
    pub fn new(
        track_id: String,
        track_name: String,
        track_file_name: String,
        track_path: String,
        track_length: String,
        another_id: String,
        item_id: String,
        media_type: MediaType,
    ) -> Self {
        Self {
//            track_id: String::from("45571899-9E84-5151-B211-B16E459C356D"),
//            track_name: String::from("BE FOUND SOON_m1.1"),
//            track_file_name: String::from("BE FOUND SOON_m1.1.wav"),
//            track_path: String::from("/home/niedzwiedz/Downloads/BE FOUND SOON_m1.1.wav"),
//            track_length: String::from("206.69117913832199"),
//            another_id: String::from("007E36C0-EC22-CB19-1344-D87A1BAB7798"),
//            item_id: String::from("88564052-9129-3D60-867F-7840E1CD613E"),

            track_id,
            track_name,
            track_file_name,
            track_path,
            track_length,
            another_id,
            item_id,
            media_type,
        }
    }

    fn media_type_str(&self) -> String {
        match self.media_type {
            MediaType::Audio => String::from("WAVE"),
            MediaType::Video => String::from("VIDEO"),
        }
    }
}

impl Track for ReaperTrack {
    fn as_string(&self) -> String {
        reaper_track(
//            String::from("45571899-9E84-5151-B211-B16E459C356D"),
//            String::from("BE FOUND SOON_m1.1"),
//            String::from("BE FOUND SOON_m1.1.wav"),
//            String::from("/home/niedzwiedz/Downloads/BE FOUND SOON_m1.1.wav"),
//            String::from("206.69117913832199"),
//            String::from("007E36C0-EC22-CB19-1344-D87A1BAB7798"),
//            String::from("88564052-9129-3D60-867F-7840E1CD613E"),
            self.track_id.clone(),
            self.track_name.clone(),
            self.track_file_name.clone(),
            self.track_path.clone(),
            self.track_length.clone(),
            self.another_id.clone(),
            self.item_id.clone(),
            self.media_type_str()
        )
    }
}

pub(crate) const REAPER_EXAMPLE: &'static str = r#"\
<REAPER_PROJECT 0.1 "5.983/linux64" 1578133917
  RIPPLE 0
  GROUPOVERRIDE 0 0 0
  AUTOXFADE 1
  ENVATTACH 1
  POOLEDENVATTACH 0
  MIXERUIFLAGS 11 48
  PEAKGAIN 1
  FEEDBACK 0
  PANLAW 1
  PROJOFFS 0 0 0
  MAXPROJLEN 0 600
  GRID 3199 8 1 8 1 0 0 0
  TIMEMODE 1 5 -1 30 0 0 -1
  VIDEO_CONFIG 0 0 256
  PANMODE 3
  CURSOR 56
  ZOOM 1.1747815778626 0 0
  VZOOMEX 6
  USE_REC_CFG 0
  RECMODE 1
  SMPTESYNC 0 30 100 40 1000 300 0 0 1 0 0
  LOOP 0
  LOOPGRAN 0 4
  RECORD_PATH "" ""
  <RECORD_CFG
  >
  <APPLYFX_CFG
  >
  RENDER_FILE ""
  RENDER_PATTERN ""
  RENDER_FMT 0 2 0
  RENDER_1X 0
  RENDER_RANGE 1 0 0 18 1000
  RENDER_RESAMPLE 3 0 1
  RENDER_ADDTOPROJ 0
  RENDER_STEMS 0
  RENDER_DITHER 0
  TIMELOCKMODE 1
  TEMPOENVLOCKMODE 1
  ITEMMIX 0
  DEFPITCHMODE 589824 0
  TAKELANE 1
  SAMPLERATE 44100 0 0
  <RENDER_CFG
  >
  LOCK 1
  <METRONOME 6 2
    VOL 0.25 0.125
    FREQ 800 1600 1
    BEATLEN 4
    SAMPLES "" ""
    PATTERN 2863311530 2863311529
  >
  GLOBAL_AUTO -1
  TEMPO 120 4 4
  PLAYRATE 1 0 0.25 4
  SELECTION 0 0
  SELECTION2 0 0
  MASTERAUTOMODE 0
  MASTERTRACKHEIGHT 0 0
  MASTERPEAKCOL 16576
  MASTERMUTESOLO 0
  MASTERTRACKVIEW 0 0.6667 0.5 0.5 0 0 0
  MASTERHWOUT 0 0 1 0 0 0 0 -1
  MASTER_NCH 2 2
  MASTER_VOLUME 1 0 -1 -1 1
  MASTER_FX 1
  MASTER_SEL 0
  <MASTERPLAYSPEEDENV
    ACT 0 -1
    VIS 0 1 1
    LANEHEIGHT 0 0
    ARM 0
    DEFSHAPE 0 -1 -1
  >
  <TEMPOENVEX
    ACT 0 -1
    VIS 1 0 1
    LANEHEIGHT 0 0
    ARM 0
    DEFSHAPE 1 -1 -1
  >
  <PROJBAY
  >
  <TRACK {45571899-9E84-5151-B211-B16E459C356D}
    NAME "BE FOUND SOON_m1.1"
    PEAKCOL 16576
    BEAT -1
    AUTOMODE 0
    VOLPAN 1 0 -1 -1 1
    MUTESOLO 0 0 0
    IPHASE 0
    ISBUS 0 0
    BUSCOMP 0 0
    SHOWINMIX 1 0.6667 0.5 1 0.5 0 0 0
    FREEMODE 0
    SEL 0
    REC 0 0 0 0 0 0 0
    VU 2
    TRACKHEIGHT 0 0 0
    INQ 0 0 0 0.5 100 0 0 100
    NCHAN 2
    FX 1
    TRACKID {45571899-9E84-5151-B211-B16E459C356D}
    PERF 0
    MIDIOUT -1
    MAINSEND 1 0
    <ITEM
      POSITION 0
      SNAPOFFS 0
      LENGTH 206.69117913832199
      LOOP 1
      ALLTAKES 0
      FADEIN 1 0.01 0 1 0 0
      FADEOUT 1 0.01 0 1 0 0
      MUTE 0
      SEL 0
      IGUID {88564052-9129-3D60-867F-7840E1CD613E}
      IID 1
      NAME "BE FOUND SOON_m1.1.wav"
      VOLPAN 1 0 1 -1
      SOFFS 0
      PLAYRATE 1 1 0 -1 0 0.0025
      CHANMODE 0
      GUID {007E36C0-EC22-CB19-1344-D87A1BAB7798}
      <SOURCE WAVE
        FILE "/home/niedzwiedz/Downloads/BE FOUND SOON_m1.1.wav"
      >
    >
  >
>
"#;

pub(crate) fn reaper_project_file(tracks: Vec<String>, project_id: String) -> String {
    format!(r#"\
<REAPER_PROJECT 0.1 "5.983/linux64" {project_id}
  RIPPLE 0
  GROUPOVERRIDE 0 0 0
  AUTOXFADE 1
  ENVATTACH 1
  POOLEDENVATTACH 0
  MIXERUIFLAGS 11 48
  PEAKGAIN 1
  FEEDBACK 0
  PANLAW 1
  PROJOFFS 0 0 0
  MAXPROJLEN 0 600
  GRID 3199 8 1 8 1 0 0 0
  TIMEMODE 1 5 -1 30 0 0 -1
  VIDEO_CONFIG 0 0 256
  PANMODE 3
  CURSOR 56
  ZOOM 1.1747815778626 0 0
  VZOOMEX 6
  USE_REC_CFG 0
  RECMODE 1
  SMPTESYNC 0 30 100 40 1000 300 0 0 1 0 0
  LOOP 0
  LOOPGRAN 0 4
  RECORD_PATH "" ""
  <RECORD_CFG
  >
  <APPLYFX_CFG
  >
  RENDER_FILE ""
  RENDER_PATTERN ""
  RENDER_FMT 0 2 0
  RENDER_1X 0
  RENDER_RANGE 1 0 0 18 1000
  RENDER_RESAMPLE 3 0 1
  RENDER_ADDTOPROJ 0
  RENDER_STEMS 0
  RENDER_DITHER 0
  TIMELOCKMODE 1
  TEMPOENVLOCKMODE 1
  ITEMMIX 0
  DEFPITCHMODE 589824 0
  TAKELANE 1
  SAMPLERATE 44100 0 0
  <RENDER_CFG
  >
  LOCK 1
  <METRONOME 6 2
    VOL 0.25 0.125
    FREQ 800 1600 1
    BEATLEN 4
    SAMPLES "" ""
    PATTERN 2863311530 2863311529
  >
  GLOBAL_AUTO -1
  TEMPO 120 4 4
  PLAYRATE 1 0 0.25 4
  SELECTION 0 0
  SELECTION2 0 0
  MASTERAUTOMODE 0
  MASTERTRACKHEIGHT 0 0
  MASTERPEAKCOL 16576
  MASTERMUTESOLO 0
  MASTERTRACKVIEW 0 0.6667 0.5 0.5 0 0 0
  MASTERHWOUT 0 0 1 0 0 0 0 -1
  MASTER_NCH 2 2
  MASTER_VOLUME 1 0 -1 -1 1
  MASTER_FX 1
  MASTER_SEL 0
  <MASTERPLAYSPEEDENV
    ACT 0 -1
    VIS 0 1 1
    LANEHEIGHT 0 0
    ARM 0
    DEFSHAPE 0 -1 -1
  >
  <TEMPOENVEX
    ACT 0 -1
    VIS 1 0 1
    LANEHEIGHT 0 0
    ARM 0
    DEFSHAPE 1 -1 -1
  >
  <PROJBAY
  >{tracks}
>
"#, tracks = tracks.join("\n"), project_id = project_id).to_string()
}

pub(crate) const REAPER_TRACK_EXAMPLE: &'static str = r#"
  <TRACK {45571899-9E84-5151-B211-B16E459C356D}
    NAME "BE FOUND SOON_m1.1"
    PEAKCOL 16576
    BEAT -1
    AUTOMODE 0
    VOLPAN 1 0 -1 -1 1
    MUTESOLO 0 0 0
    IPHASE 0
    ISBUS 0 0
    BUSCOMP 0 0
    SHOWINMIX 1 0.6667 0.5 1 0.5 0 0 0
    FREEMODE 0
    SEL 0
    REC 0 0 0 0 0 0 0
    VU 2
    TRACKHEIGHT 0 0 0
    INQ 0 0 0 0.5 100 0 0 100
    NCHAN 2
    FX 1
    TRACKID {45571899-9E84-5151-B211-B16E459C356D}
    PERF 0
    MIDIOUT -1
    MAINSEND 1 0
    <ITEM
      POSITION 0
      SNAPOFFS 0
      LENGTH 206.69117913832199
      LOOP 1
      ALLTAKES 0
      FADEIN 1 0.01 0 1 0 0
      FADEOUT 1 0.01 0 1 0 0
      MUTE 0
      SEL 0
      IGUID {88564052-9129-3D60-867F-7840E1CD613E}
      IID 1
      NAME "BE FOUND SOON_m1.1.wav"
      VOLPAN 1 0 1 -1
      SOFFS 0
      PLAYRATE 1 1 0 -1 0 0.0025
      CHANMODE 0
      GUID {007E36C0-EC22-CB19-1344-D87A1BAB7798}
      <SOURCE WAVE
        FILE "/home/niedzwiedz/Downloads/BE FOUND SOON_m1.1.wav"
      >
    >
  >"#;

pub(crate) fn reaper_track(
    track_id: String,
    track_name: String,
    track_file_name: String,
    track_path: String,
    track_length: String,
    another_id: String,
    item_id: String,
    media_type: String,
) -> String {
    format!(r#"
  <TRACK {{{track_id}}}
    NAME "{track_name}"
    PEAKCOL 16576
    BEAT -1
    AUTOMODE 0
    VOLPAN 1 0 -1 -1 1
    MUTESOLO 0 0 0
    IPHASE 0
    ISBUS 0 0
    BUSCOMP 0 0
    SHOWINMIX 1 0.6667 0.5 1 0.5 0 0 0
    FREEMODE 0
    SEL 0
    REC 0 0 0 0 0 0 0
    VU 2
    TRACKHEIGHT 0 0 0
    INQ 0 0 0 0.5 100 0 0 100
    NCHAN 2
    FX 1
    TRACKID {{{track_id}}}
    PERF 0
    MIDIOUT -1
    MAINSEND 1 0
    <ITEM
      POSITION 0
      SNAPOFFS 0
      LENGTH {track_length}
      LOOP 1
      ALLTAKES 0
      FADEIN 1 0.01 0 1 0 0
      FADEOUT 1 0.01 0 1 0 0
      MUTE 0
      SEL 0
      IGUID {{{item_id}}}
      IID 1
      NAME "{track_file_name}"
      VOLPAN 1 0 1 -1
      SOFFS 0
      PLAYRATE 1 1 0 -1 0 0.0025
      CHANMODE 0
      GUID {{{another_id}}}
      <SOURCE {media_type}
        FILE "{track_path}"
      >
    >
  >"#,
            track_path = track_path,
            track_name = track_name,
            track_file_name = track_file_name,
            item_id = item_id,
            track_length = track_length,
            another_id = another_id,
            track_id = track_id,
            media_type = media_type,
    )
}