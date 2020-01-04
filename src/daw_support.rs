use std::fs::File;
use std::io::Write;

pub trait DAWProjectFile {
    fn project_file(&self) -> String;
    fn filename(&self) -> String;
    fn save(&self) -> std::io::Result<()> {
        let mut file = File::create(self.filename())?;
        file.write_all(self.project_file().as_bytes())
    }
}

pub trait Track {
    fn as_string(&self) -> String;
}
