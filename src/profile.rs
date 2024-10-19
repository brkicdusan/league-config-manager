use std::{
    error::Error,
    fs::{self, File, OpenOptions},
    io::{BufReader, Read, Write},
    path::{Path, PathBuf},
};

use iced::{
    widget::{column, container, pick_list, row, text, text_input, tooltip},
    Element, Length,
};
use serde::{Deserialize, Serialize};
use zip::{write::SimpleFileOptions, CompressionMethod, ZipArchive, ZipWriter};

use crate::{
    cfg::Cfg,
    champion::{get_champion_id_from_name, get_champion_name_from_id, get_champion_name_list},
    colors,
    config::get_config_dir,
    error,
    message::Message,
    theme::{self, Theme},
    widget::{cancel_icon, confirm_icon, edit_icon, icon_btn, share_icon, trash_icon, use_icon},
};

#[derive(Debug, Clone)]
pub struct Profile {
    name: String,
    editing: bool,
    edit_name: String,
    champion: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Settings {
    champion: Option<u32>,
}

impl Settings {
    pub fn from_path(path: &Path) -> Settings {
        let settings_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)
            .expect("Can't open config file");
        let reader = BufReader::new(settings_file);
        let settings: Settings =
            serde_json::from_reader(reader).unwrap_or_else(|_| Settings { champion: None });
        settings
    }

    pub fn export(&self, path: &Path) {
        let settings_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)
            .expect("Can't open config file");
        let writer = std::io::BufWriter::new(settings_file);
        serde_json::to_writer(writer, self).expect("Couldn't write the config");
    }
}

impl Profile {
    fn gen_name() -> Option<String> {
        let profiles = Self::get_profiles();
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
            if p.get_name().eq(name) {
                return false;
            }
        }
        true
    }

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
        if !Self::check_name(&name, &Self::get_profiles()) {
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

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_profiles() -> Vec<Profile> {
        let config_dir = get_config_dir();
        let mut profiles: Vec<Self> = vec![];
        for entry in fs::read_dir(config_dir).unwrap() {
            let entry = entry.unwrap().path();
            if entry.is_dir() {
                let name = entry.file_name().unwrap().to_str().unwrap().to_string();

                let settings_path = entry.join("settings.json");

                let settings = Settings::from_path(&settings_path);

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

    fn get_path(&self) -> PathBuf {
        let dir = get_config_dir();
        dir.join(&self.name)
    }

    pub fn delete(&self) {
        let dir = self.get_path();
        fs::remove_dir_all(dir).unwrap();
    }

    pub fn copy_files(&self, cfg: &Cfg) {
        cfg.set_readonly(false);

        fs::copy(
            self.get_path().join(cfg.game.file_name().unwrap()),
            &cfg.game,
        )
        .unwrap();
        fs::copy(
            self.get_path().join(cfg.settings.file_name().unwrap()),
            &cfg.settings,
        )
        .unwrap();
    }

    pub fn edit_start(&mut self) {
        self.edit_name.clone_from(&self.name);
        self.editing = true;
    }

    pub fn edit_confirm(&mut self) -> Result<(), error::Error> {
        for p in Profile::get_profiles() {
            if p.get_name().eq(&self.edit_name) {
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

    pub fn edit_change(&mut self, new_name: String) {
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

    pub fn get_champion(&self) -> Option<u32> {
        self.champion
    }

    pub fn options_list() -> Vec<&'static str> {
        let mut options = vec!["Disabled", "Default"];
        options.append(&mut get_champion_name_list());
        options
    }

    fn set_settings(&self) {
        let settings = Settings {
            champion: self.champion,
        };

        let path = self.get_path().join("settings.json");

        settings.export(&path);
    }

    pub fn get_selected(&self) -> &'static str {
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

        self.set_settings();
    }

    fn champion_row(&self) -> Element<Message, Theme> {
        let txt = text("Swap when this champion is selected:").width(Length::Fill);
        let options = Self::options_list();
        let pl = pick_list(options, Some(self.get_selected()), |s| {
            Message::PickListChange(self.name.clone(), s)
        });
        row![txt, pl].into()
    }

    pub fn get_item(&self, cfg: &Option<Cfg>) -> Element<Message, Theme> {
        let del_btn = tooltip(
            icon_btn(
                trash_icon(),
                Message::RemoveProfile(self.name.clone()).into(),
                colors::RED,
            ),
            "Delete profile",
            tooltip::Position::Bottom,
        )
        .class(theme::Container::Tooltip);

        let mut use_msg = None;
        if cfg.is_some() {
            use_msg = Some(Message::UseProfile(self.clone()));
        }
        let use_btn = tooltip(
            icon_btn(use_icon(), use_msg, colors::BLUE),
            "Use this profile",
            tooltip::Position::Bottom,
        )
        .class(theme::Container::Tooltip);

        let export_btn = tooltip(
            icon_btn(
                share_icon(),
                Message::Export(self.clone()).into(),
                colors::BLUE,
            ),
            "Export settings to .zip",
            tooltip::Position::Bottom,
        )
        .class(theme::Container::Tooltip);

        let edit_btn = if !self.editing {
            container(
                tooltip(
                    icon_btn(
                        edit_icon(),
                        Message::Edit(self.name.clone()).into(),
                        colors::BLUE,
                    ),
                    "Edit name",
                    tooltip::Position::Bottom,
                )
                .class(theme::Container::Tooltip),
            )
        } else {
            container(
                row![
                    icon_btn(
                        confirm_icon(),
                        Message::Confirm(self.name.clone()).into(),
                        colors::GREEN
                    ),
                    icon_btn(
                        cancel_icon(),
                        Message::Reset(self.name.clone()).into(),
                        colors::RED
                    )
                ]
                .spacing(10),
            )
        };

        let mut profile_row = row![].spacing(10).align_y(iced::Alignment::Center);
        if !self.editing {
            profile_row = profile_row.push(text(&self.name).width(iced::Length::Fill));
        }

        if self.editing {
            profile_row = profile_row.push(
                text_input("", &self.edit_name)
                    .padding(10)
                    .on_input(|s| Message::OnChange(self.name.clone(), s))
                    .width(iced::Length::Fill),
            );
        }

        profile_row = profile_row.push(edit_btn);
        profile_row = profile_row.push(use_btn);
        profile_row = profile_row.push(export_btn);
        profile_row = profile_row.push(del_btn);

        column![profile_row, self.champion_row()].into()
    }
}
