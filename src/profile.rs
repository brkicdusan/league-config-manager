use std::{fs, path::PathBuf};

use iced::widget::{button, horizontal_space, row, text};

use crate::{cfg::Cfg, config::get_config_dir, message::Message};

#[derive(Debug)]
pub struct Profile {
    name: String,
}

impl Profile {
    fn gen_name() -> Option<String> {
        let profiles = Self::get_profiles();
        for i in 0..15 {
            let name: String = format!("profile_{i}");
            let mut flag = true;
            for p in &profiles {
                if p.get_name().eq(&name) {
                    flag = false;
                    break;
                }
            }
            if flag {
                return Some(name);
            }
        }
        None
    }

    pub fn new(cfg: &Cfg) -> Self {
        let name = Profile::gen_name().unwrap();
        let mut dir = get_config_dir();
        dir.push(&name);
        fs::create_dir_all(&dir).unwrap();
        fs::copy(&cfg.game, dir.join(cfg.game.file_name().unwrap())).unwrap();
        fs::copy(&cfg.settings, dir.join(cfg.settings.file_name().unwrap())).unwrap();
        Self { name }
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
                profiles.push(Self { name })
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

    pub fn get_item(&self) -> iced::widget::Row<'_, Message> {
        let del_btn = button(text("Remove"))
            .style(iced::theme::Button::Destructive)
            .on_press(Message::RemoveProfile(self.name.clone()));
        row![text(&self.name), horizontal_space(), del_btn]
    }
}
