use std::{
    fs::{self},
    path::PathBuf,
};

use iced::widget::{button, container, horizontal_space, row, text, text_input, Row};

use crate::{cfg::Cfg, config::get_config_dir, error, message::Message};

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
        Self {
            name,
            editing: false,
            edit_name: String::from(""),
        }
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

    pub fn get_item(&self, cfg: &Option<Cfg>) -> Row<Message> {
        let del_btn = button(text("Remove"))
            .style(iced::theme::Button::Destructive)
            .on_press(Message::RemoveProfile(self.name.clone()));

        let mut use_btn = button(text("Use"));
        if cfg.is_some() {
            use_btn = use_btn.on_press(Message::UseProfile(self.clone()));
        }

        let edit_btn = if !self.editing {
            container(button("Edit").on_press(Message::Edit(self.name.clone())))
        } else {
            container(
                row![
                    button("Confirm")
                        .style(iced::theme::Button::Positive)
                        .on_press(Message::Confirm(self.name.clone())),
                    button("Reset")
                        .style(iced::theme::Button::Destructive)
                        .on_press(Message::Reset(self.name.clone())),
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
                    .on_input(|s| Message::OnChange(self.name.clone(), s)),
            );
        }

        profile_row = profile_row.push(horizontal_space());
        profile_row = profile_row.push(edit_btn);
        profile_row = profile_row.push(del_btn);
        profile_row = profile_row.push(use_btn);

        profile_row
    }
}
