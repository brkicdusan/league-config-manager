use crate::config::Config;

use crate::error::Error;

use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

pub struct Cfg {
    pub game: PathBuf,
    pub settings: PathBuf,
}

impl Cfg {
    pub fn new(folder_path: &Path) -> Result<Cfg, Error> {
        let folder_path = folder_path.join("Config");
        let game = folder_path.join("game.cfg");
        let settings = folder_path.join("PersistedSettings.json");
        if game.exists() && settings.exists() {
            return Ok(Cfg { game, settings });
        }
        Err(Error::WrongPath)
    }

    pub fn from_config(config: &Config) -> Result<Cfg, Error> {
        if let Some(c) = &config.get_cfg_path() {
            return Self::new(c);
        }
        Err(Error::WrongPath)
        // TODO: error handling
    }

    pub fn get_readonly(&self) -> bool {
        let f = File::open(&self.game).expect("File should always exist");
        f.metadata().unwrap().permissions().readonly()
    }

    fn set_readonly_path(&self, p: &PathBuf, value: bool) {
        let f = File::open(p).expect("File should always exist");
        let mut perms = f.metadata().unwrap().permissions();
        perms.set_readonly(value);
        f.set_permissions(perms)
            .expect("Problem setting premissions");
    }

    pub fn set_readonly(&self, value: bool) {
        self.set_readonly_path(&self.game, value);
        self.set_readonly_path(&self.settings, value);
    }
}