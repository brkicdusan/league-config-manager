use super::error::Error;

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
}
