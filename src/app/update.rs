use error::Error;

use dialog::export_zip_path;

use profile::Profile;

use iced::Task;

use message::Message;

use crate::*;

use super::App;

impl App {
    pub(crate) fn update(&mut self, message: Message) -> Task<Message> {
        self.success = None;
        // TODO: fix errors disappearing
        self.error = None;

        match message {
            Message::FindLocation => {
                Task::perform(dialog::find_config_dialog(), Message::SetLocation)
            }
            Message::SetLocation(Ok(location)) => {
                if self.set_cfg(&location).is_none() {
                    self.config.set_path(Some(location));
                }
                Task::none()
            }
            Message::SetLocation(Err(error)) => {
                self.error = Some(error);
                Task::none()
            }
            Message::SetReadonly(readonly) => {
                self.readonly = readonly;
                if let Some(c) = &self.cfg {
                    c.set_readonly(readonly);
                }
                Task::none()
            }
            Message::AddProfile => {
                if let Some(cfg) = &self.cfg {
                    let new_profile = Profile::new(cfg);
                    self.profiles.push(new_profile);
                }
                Task::none()
            }
            Message::RemoveProfile(s) => {
                for (i, p) in self.profiles.iter().enumerate() {
                    if p.get_name() == &s {
                        p.delete();
                        self.profiles.remove(i);
                        break;
                    }
                }
                Task::none()
            }
            Message::UseProfile(prof) => {
                if let Some(cfg) = &self.cfg {
                    prof.copy_files(cfg);
                    cfg.set_readonly(self.readonly);
                    self.success = Some(format!("Using \"{}\"", prof.get_name()))
                }
                Task::none()
            }
            Message::Edit(name) => {
                let prof = self.get_profile_from_name(&name).unwrap();
                prof.edit_start();
                Task::none()
            }
            Message::Confirm(name) => {
                let prof = self.get_profile_from_name(&name).unwrap();

                if let Err(e) = prof.edit_confirm() {
                    self.error = Some(e);
                } else {
                    self.success = Some(format!("Changed name to {}", &prof.get_name()));
                    self.error = None;
                }
                Task::none()
            }
            Message::Reset(name) => {
                let prof = self.get_profile_from_name(&name).unwrap();
                prof.edit_reset();
                Task::none()
            }
            Message::OnChange(name, new_name) => {
                let prof = self.get_profile_from_name(&name).unwrap();
                prof.edit_change(new_name);
                Task::none()
            }
            Message::Export(profile) => Task::perform(export_zip_path(profile), Message::SetExport),
            Message::SetExport(Ok((export_path, profile))) => {
                match profile.zip(export_path) {
                    Ok(_) => self.success = Some("Exported profile".to_string()),
                    Err(_) => self.error = Some(Error::ZipExport),
                };
                Task::none()
            }
            Message::SetExport(Err(e)) => {
                self.error = Some(e);
                Task::none()
            }
            Message::SetImport(Ok(import_path)) => {
                match Profile::from_zip(&import_path) {
                    Ok(profile) => self.profiles.push(profile),
                    Err(_) => self.error = Some(Error::ZipImport),
                }
                Task::none()
            }
            Message::SetImport(Err(e)) => {
                self.error = Some(e);
                Task::none()
            }
            Message::Import => Task::perform(dialog::import_zip_path(), Message::SetImport),
            Message::WebsocketEvent(event) => {
                match event {
                    websocket::Event::Selected(x) => {
                        self.champion_id = None;

                        if x > 0 {
                            self.champion_id = Some(x);

                            let mut profile =
                                self.profiles.iter().find(|p| p.get_champion() == Some(x));
                            if profile.is_none() {
                                profile =
                                    self.profiles.iter().find(|p| p.get_champion() == Some(0));
                            }

                            if let Some(prof) = profile {
                                if let Some(cfg) = &self.cfg {
                                    prof.copy_files(cfg);
                                }
                            }
                        }
                    }
                    websocket::Event::Connected => {
                        self.connected = true;
                        self.retry_in = None;
                    }
                    websocket::Event::Disconnected => {
                        self.connected = false;
                        self.retry_in = None;
                    }
                    websocket::Event::Retrying(t) => self.retry_in = Some(t),
                }
                Task::none()
            }
            Message::PickListChange(profile_name, option) => {
                let profiles = &self.profiles;

                for profile in profiles {
                    if profile.get_selected() == option
                        && profile.get_name() != &profile_name
                        && option != "Disabled"
                    {
                        self.error = Error::ChampionTaken.into();
                        return Task::none();
                    }
                }

                let profile = self
                    .profiles
                    .iter_mut()
                    .find(|p| p.get_name() == &profile_name)
                    .unwrap();

                profile.set_selected(option);

                Task::none()
            }
        }
    }
}
