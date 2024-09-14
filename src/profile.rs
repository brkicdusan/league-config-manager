use std::{
    error::Error,
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};

use iced::{
    widget::{button, container, horizontal_space, row, text, text_input, Row},
    Length,
};
use zip::{write::SimpleFileOptions, CompressionMethod, ZipArchive, ZipWriter};

use crate::{
    cfg::Cfg,
    colors,
    config::get_config_dir,
    error,
    message::Message,
    theme::Theme,
    widget::{cancel_icon, confirm_icon, edit_icon, icon_btn, share_icon, trash_icon, use_icon},
};

#[derive(Debug, Clone)]
pub struct Profile {
    name: String,
    editing: bool,
    edit_name: String,
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
                profiles.push(Self {
                    name,
                    editing: false,
                    edit_name: String::from(""),
                })
            }
        }
        profiles
    }

    fn path_to(&self) -> PathBuf {
        let dir = get_config_dir();
        dir.join(&self.name)
    }

    pub fn delete(&self) {
        let dir = self.path_to();
        fs::remove_dir_all(dir).unwrap();
    }

    pub fn copy_files(&self, cfg: &Cfg) {
        cfg.set_readonly(false);

        fs::copy(
            self.path_to().join(cfg.game.file_name().unwrap()),
            &cfg.game,
        )
        .unwrap();
        fs::copy(
            self.path_to().join(cfg.settings.file_name().unwrap()),
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
        // TODO:check if new name is valid name for dir
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

    pub fn get_item(&self, cfg: &Option<Cfg>) -> Row<Message, Theme> {
        let del_btn = icon_btn(
            trash_icon(),
            Message::RemoveProfile(self.name.clone()).into(),
            colors::RED,
        );

        let mut use_msg = None;
        if cfg.is_some() {
            use_msg = Some(Message::UseProfile(self.clone()));
        }
        let use_btn = icon_btn(use_icon(), use_msg, colors::BLUE);

        let export_btn = icon_btn(
            share_icon(),
            Message::Export(self.clone()).into(),
            colors::BLUE,
        );

        let edit_btn = if !self.editing {
            container(icon_btn(
                edit_icon(),
                Message::Edit(self.name.clone()).into(),
                colors::BLUE,
            ))
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

        let mut profile_row = row![].spacing(10).align_items(iced::Alignment::Center);
        if !self.editing {
            profile_row = profile_row.push(text(&self.name));
        }

        if self.editing {
            profile_row = profile_row.push(
                text_input("", &self.edit_name)
                    .padding(10)
                    .on_input(|s| Message::OnChange(self.name.clone(), s)),
            );
        }

        profile_row = profile_row.push(horizontal_space());
        profile_row = profile_row.push(edit_btn);
        profile_row = profile_row.push(use_btn);
        profile_row = profile_row.push(export_btn);
        profile_row = profile_row.push(del_btn);

        profile_row
    }
}
