use serde::{Deserialize, Serialize};

use std::io::BufReader;

use std::fs::OpenOptions;

use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Settings {
    pub(crate) champion: Option<u32>,
    pub(crate) last_link: String,
}

impl Settings {
    pub fn from_path(path: &Path) -> Settings {
        let settings_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)
            .expect("Can't open profile settings file");
        let reader = BufReader::new(settings_file);
        let settings: Settings = serde_json::from_reader(reader).unwrap_or(Settings {
            champion: None,
            last_link: "".to_string(),
        });
        settings
    }

    pub fn export(&self, path: &Path) {
        let settings_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .expect("Can't open profile settings file");
        let writer = std::io::BufWriter::new(settings_file);
        serde_json::to_writer(writer, self).expect("Couldn't write the profile settings");
    }
}
