use ini::Ini;

use crate::config::Config;

use crate::error::Error;

use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct GameSettings {
    pub game: PathBuf,
    pub settings: PathBuf,
}

impl GameSettings {
    pub fn from_path(folder_path: &Path) -> Result<GameSettings, Error> {
        let game = folder_path.join("game.cfg");
        let settings = folder_path.join("PersistedSettings.json");
        if game.exists() && settings.exists() {
            return Ok(GameSettings { game, settings });
        }
        Err(Error::WrongPath)
    }

    pub fn from_config(config: &Config) -> Result<GameSettings, Error> {
        if let Some(c) = &config.path() {
            let c = c.join("Config");
            return Self::from_path(&c);
        }
        Err(Error::MissingPath)
    }

    pub fn from_string(dir: &Path, content: String) -> Result<Self, Error> {
        let mut parts = content.split(Self::seperator());
        let game_str = parts.next();
        let settings_str = parts.next();

        if game_str.is_none() || settings_str.is_none() {
            return Err(Error::Import);
        }
        let game_str = game_str.unwrap();
        let settings_str = settings_str.unwrap();

        let game = dir.join("game.cfg");
        let settings = dir.join("PersistedSettings.json");

        let mut game_file = File::create(game).expect("Unable to open file");
        game_file
            .write_all(game_str.as_bytes())
            .expect("Unable to write data");

        let mut settings_file = File::create(settings).expect("Unable to open file");
        settings_file
            .write_all(settings_str.as_bytes())
            .expect("Unable to write data");
        Self::from_path(dir)
    }

    pub fn update_paths(&mut self, dir: &Path) {
        self.game = dir.join("game.cfg");
        self.settings = dir.join("PersistedSettings.json");
    }

    pub fn readonly(&self) -> bool {
        let f = File::open(&self.game).expect("File should always exist");
        f.metadata().unwrap().permissions().readonly()
    }

    fn set_readonly_path(p: &PathBuf, value: bool) {
        let f = File::open(p).expect("File should always exist");
        let mut perms = f.metadata().expect("Error reading meatadata").permissions();
        perms.set_readonly(value);
        std::fs::set_permissions(p, perms).unwrap();
    }

    pub fn set_readonly(&self, value: bool) {
        Self::set_readonly_path(&self.game, value);
        Self::set_readonly_path(&self.settings, value);
    }

    fn seperator() -> &'static str {
        "<lcm-seperator>"
    }

    pub fn to_paste_string(&self) -> String {
        let mut f = File::open(&self.game).expect("File should always exist");
        let mut res: String = "".to_string();
        f.read_to_string(&mut res).expect("Should always be valid");
        res.push_str(Self::seperator());
        let mut f = File::open(&self.settings).expect("File should always exist");
        f.read_to_string(&mut res).expect("Should always be valid");

        res
    }

    fn load_ini(&self) -> Result<Ini, ini::Error> {
        Ini::load_from_file(&self.game)
    }

    fn save_ini(&self, ini: Ini) -> Result<(), ini::Error> {
        ini.write_to_file(&self.game)?;
        Ok(())
    }

    pub fn reset_resolution(&self) -> Result<(), ini::Error> {
        let readonly = self.readonly();
        self.set_readonly(false);
        let mut ini = self.load_ini()?;

        ini.delete_from(Some("General"), "Width");
        ini.delete_from(Some("General"), "Height");

        self.save_ini(ini)?;

        self.set_readonly(readonly);

        Ok(())
    }
}
