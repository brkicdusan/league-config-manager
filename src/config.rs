use std::{
    fs::{self, OpenOptions},
    io::BufReader,
    path::PathBuf,
};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

fn get_config_path() -> PathBuf {
    let proj_dir =
        ProjectDirs::from("", "", "league_config_manager").expect("This should always resolve");
    let dir = proj_dir.data_dir().to_path_buf();
    fs::create_dir_all(&dir).expect("Should always create these dirs");
    dir.join("config.json")
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    cfg_path: Option<PathBuf>,
}

impl Config {
    pub fn new() -> Self {
        let config_path = get_config_path();
        // TODO:config gct ccommentar
        let config_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(config_path)
            .expect("Can't open config file");
        let reader = BufReader::new(config_file);
        let config: Config = serde_json::from_reader(reader).unwrap_or_else(|_| {
            let config = Config::default();
            config.set_config();
            config
        });
        config
    }

    pub fn get_cfg_path(&self) -> &Option<PathBuf> {
        &self.cfg_path
    }

    pub fn path_to_str(&self) -> &str {
        match &self.cfg_path {
            Some(path) => path.to_str().expect("Should always work"),
            _ => "",
        }
    }

    fn default() -> Config {
        Config { cfg_path: None }
    }

    pub fn set_path(&mut self, cfg_path: Option<PathBuf>) {
        self.cfg_path = cfg_path;
        self.update();
    }

    pub fn set_config(&self) {
        let config_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(get_config_path())
            .expect("Can't open config file");
        let writer = std::io::BufWriter::new(config_file);
        serde_json::to_writer(writer, self).expect("Couldn't write the config");
    }

    pub fn update(&self) {
        self.set_config();
    }
}
