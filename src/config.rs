use std::{
    fs::{self, OpenOptions},
    io::BufReader,
    path::{Path, PathBuf},
};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::game_settings::GameSettings;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    path: Option<PathBuf>,
}

//static methods
impl Config {
    pub fn get_config_dir() -> PathBuf {
        let proj_dir =
            ProjectDirs::from("", "", "league_config_manager").expect("This should always resolve");
        let dir = proj_dir.data_dir().to_path_buf();
        fs::create_dir_all(&dir).expect("Should always create these dirs");
        dir
    }

    pub fn get_config_path() -> PathBuf {
        let dir = Self::get_config_dir();
        dir.join("config.json")
    }
}

impl Config {
    pub fn new() -> Self {
        let config_path = Self::get_config_path();
        let config_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(config_path)
            .expect("Can't open config file");
        let reader = BufReader::new(config_file);
        let mut config: Config = serde_json::from_reader(reader).unwrap_or_else(|_| {
            let cfg_path = Self::try_cfg();
            let config = Config { path: cfg_path };
            config.write_config();
            config
        });
        // TODO: ovo nije dobro uopste uradi kad budes odspavao
        if let Some(path) = &config.path {
            if GameSettings::from_path(&Self::game_settings_path(path)).is_err() {
                config.path = Self::try_cfg();
            }
        }
        config
    }

    pub fn path(&self) -> &Option<PathBuf> {
        &self.path
    }

    pub fn path_to_str(&self) -> &str {
        match &self.path {
            Some(path) => path.to_str().expect("Should always work"),
            _ => "",
        }
    }

    pub fn set_path(&mut self, cfg_path: Option<PathBuf>) {
        self.path = cfg_path;
        self.update();
    }

    /// Write config file
    pub fn write_config(&self) {
        let config_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(Self::get_config_path())
            .expect("Can't open config file");
        let writer = std::io::BufWriter::new(config_file);
        serde_json::to_writer(writer, self).expect("Couldn't write the config");
    }

    pub fn update(&self) {
        self.write_config();
    }

    fn try_cfg() -> Option<PathBuf> {
        let folders = ["C:\\Riot Games\\League of Legends"].map(PathBuf::from);
        for folder in folders {
            let folder_new = folder.join("Config");
            let cfg = GameSettings::from_path(&folder_new);
            if cfg.is_ok() {
                return Some(folder);
            }
        }
        None
    }

    pub fn game_settings_path(path: &Path) -> PathBuf {
        path.join("Config")
    }
}
