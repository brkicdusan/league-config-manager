mod settings;
mod view;

use std::{
    error::Error,
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};

use zip::{write::SimpleFileOptions, CompressionMethod, ZipArchive, ZipWriter};

use crate::{
    cfg::Cfg,
    champion::{get_champion_id_from_name, get_champion_name_from_id, get_champion_name_list},
    config::get_config_dir,
    error,
};

#[derive(Debug, Clone)]
pub struct Profile {
    name: String,
    editing: bool,
    edit_name: String,
    champion: Option<u32>,
}

//name gen
impl Profile {
    fn gen_name() -> Option<String> {
        let profiles = Self::profiles();
        for i in 0..15 {
            let name: String = format!("profile_{i}");
            if Self::check_name(&name, &profiles) {
                return Some(name);
            }
        }
        None
    }

    fn check_name(name: &String, profiles: &Vec<Profile>) -> bool {
        for p in profiles {
            if p.name().eq(name) {
                return false;
            }
        }
        true
    }
}

//constructors
impl Profile {
    pub fn new(cfg: &Cfg) -> Self {
        let name = Profile::gen_name().unwrap();
        let mut dir = get_config_dir();
        dir.push(&name);
        fs::create_dir_all(&dir).unwrap();
        fs::copy(&cfg.game, dir.join(cfg.game.file_name().unwrap())).unwrap();
        fs::copy(&cfg.settings, dir.join(cfg.settings.file_name().unwrap())).unwrap();
        Self {
            name,
            editing: false,
            edit_name: String::from(""),
            champion: None,
        }
    }

    pub fn from_zip(zip_file_path: &PathBuf) -> Result<Self, Box<dyn Error>> {
        let zip_file = File::open(zip_file_path)?;
        let mut name = zip_file_path
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        if !Self::check_name(&name, &Self::profiles()) {
            name = Self::gen_name().unwrap();
        }

        let mut archive = ZipArchive::new(zip_file)?;
        let extraction_dir = get_config_dir().join(&name);
        std::fs::create_dir(&extraction_dir)?;

        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            let file_name = file.name().to_owned();

            // Create the path to the extracted file in the destination directory.
            let target_path = extraction_dir.join(file_name);

            // Create the destination directory if it does not exist.
            if let Some(parent_dir) = target_path.parent() {
                std::fs::create_dir_all(parent_dir)?;
            }

            let mut output_file = File::create(&target_path)?;

            // Read the contents of the file from the ZIP archive and write them to the destination file.
            std::io::copy(&mut file, &mut output_file)?;
        }

        Ok(Self {
            name,
            editing: false,
            edit_name: String::from(""),
            champion: None,
        })
    }
}

impl Profile {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn profiles() -> Vec<Profile> {
        let config_dir = get_config_dir();
        let mut profiles: Vec<Self> = vec![];
        for entry in fs::read_dir(config_dir).unwrap() {
            let entry = entry.unwrap().path();
            if entry.is_dir() {
                let name = entry.file_name().unwrap().to_str().unwrap().to_string();

                let settings_path = entry.join("settings.json");

                let settings = settings::Settings::from_path(&settings_path);

                profiles.push(Self {
                    name,
                    editing: false,
                    edit_name: String::from(""),
                    champion: settings.champion,
                })
            }
        }
        profiles
    }

    fn path(&self) -> PathBuf {
        let dir = get_config_dir();
        dir.join(&self.name)
    }

    pub fn delete(&self) {
        let dir = self.path();
        fs::remove_dir_all(dir).unwrap();
    }

    pub fn copy_files(&self, cfg: &Cfg) {
        cfg.set_readonly(false);

        fs::copy(self.path().join(cfg.game.file_name().unwrap()), &cfg.game).unwrap();
        fs::copy(
            self.path().join(cfg.settings.file_name().unwrap()),
            &cfg.settings,
        )
        .unwrap();
    }

    pub fn edit_start(&mut self) {
        self.edit_name.clone_from(&self.name);
        self.editing = true;
    }

    pub fn edit_confirm(&mut self) -> Result<(), error::Error> {
        for p in Profile::profiles() {
            if p.name().eq(&self.edit_name) {
                return Err(error::Error::NameTaken);
            }
        }
        self.editing = false;
        let dir = get_config_dir();
        fs::rename(dir.join(&self.name), dir.join(&self.edit_name)).unwrap();
        self.name.clone_from(&self.edit_name);
        Ok(())
    }

    pub fn edit_reset(&mut self) {
        self.editing = false;
    }

    pub fn set_edit_name(&mut self, new_name: String) {
        self.edit_name = new_name;
    }

    pub fn zip(&self, export_dir: PathBuf) -> Result<PathBuf, Box<dyn Error>> {
        let zip_file_path = export_dir.join(format!("{}.zip", &self.name));
        let zip_file = File::create(&zip_file_path)?;

        let mut zip_writer = ZipWriter::new(zip_file);

        let dir = get_config_dir().join(&self.name);

        let files_to_compress: Vec<PathBuf> =
            vec![dir.join("game.cfg"), dir.join("PersistedSettings.json")];

        let options = SimpleFileOptions::default().compression_method(CompressionMethod::DEFLATE);

        for file_path in &files_to_compress {
            let mut file = File::open(file_path)?;
            let file_name = file_path.file_name().unwrap().to_str().unwrap();

            // Adding the file to the ZIP archive.
            zip_writer.start_file(file_name, options)?;

            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;

            zip_writer.write_all(&buffer)?;
        }

        Ok(zip_file_path)
    }

    pub fn champion(&self) -> &Option<u32> {
        &self.champion
    }

    pub fn options_list() -> Vec<&'static str> {
        let mut options = vec!["Disabled", "Default"];
        options.append(&mut get_champion_name_list());
        options
    }

    fn save_settings(&self) {
        let settings = settings::Settings {
            champion: self.champion,
        };

        let path = self.path().join("settings.json");

        settings.export(&path);
    }

    pub fn selected(&self) -> &'static str {
        if self.champion.is_none() {
            return "Disabled";
        }
        let champ = self.champion.unwrap();
        if champ == 0 {
            return "Default";
        }
        get_champion_name_from_id(champ).unwrap()
    }

    pub fn set_selected(&mut self, option: &str) {
        let champion = match option {
            "Disabled" => None,
            "Default" => Some(0),
            _ => get_champion_id_from_name(option),
        };

        self.champion = champion;

        self.save_settings();
    }
}
