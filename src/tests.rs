#[cfg(test)]
mod tests {
    use crate::daws::reaper::*;
    use crate::media_file::MediaType;

    #[test]
    fn test_test() {
        assert_eq!(1, 1);
    }
    #[test]
    fn test_reaper_track() {
        assert_eq!(
            reaper_track(
                String::from("45571899-9E84-5151-B211-B16E459C356D"),
                String::from("BE FOUND SOON_m1.1"),
                String::from("BE FOUND SOON_m1.1.wav"),
                String::from("/home/niedzwiedz/Downloads/BE FOUND SOON_m1.1.wav"),
                String::from("206.69117913832199"),
                String::from("007E36C0-EC22-CB19-1344-D87A1BAB7798"),
                String::from("88564052-9129-3D60-867F-7840E1CD613E"),
                String::from("WAVE"),
            ),
            String::from(REAPER_TRACK_EXAMPLE),
        );
    }
    #[test]
    fn test_reaper_project_render() {
        assert_eq!(
            reaper_project_file(vec![String::from(REAPER_TRACK_EXAMPLE)], String::from("1578133917")),
            String::from(REAPER_EXAMPLE),
        );
    }
    #[test]
    fn test_reaper_project_object() {
        assert_eq!(Reaper::new(
            vec![ReaperTrack::new(
                String::from("45571899-9E84-5151-B211-B16E459C356D"),
                String::from("BE FOUND SOON_m1.1"),
                String::from("BE FOUND SOON_m1.1.wav"),
                String::from("/home/niedzwiedz/Downloads/BE FOUND SOON_m1.1.wav"),
                String::from("206.69117913832199"),
                String::from("007E36C0-EC22-CB19-1344-D87A1BAB7798"),
                String::from("88564052-9129-3D60-867F-7840E1CD613E"),
                MediaType::Audio,
            )],
            String::from("1578133917"),
        ).project_file(), REAPER_EXAMPLE)
    }
}
