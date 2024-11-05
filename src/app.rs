mod update;
mod view;

use crate::*;

use websocket::connect;

use iced::Subscription;

use message::Message;

use iced::Task;

use std::path::Path;

use profile::Profile;

use error::Error;

use cfg::Cfg;

use config::Config;

pub(crate) struct App {
    config: Config,
    cfg: Option<Cfg>,
    readonly: bool,
    error: Option<Error>,
    profiles: Vec<Profile>,
    success: Option<String>,
    champion_id: Option<u32>,
    connected: bool,
    retry_in: Option<u32>,
}

impl App {
    fn set_cfg(&mut self, location: &Path) -> Option<Error> {
        match Cfg::from_path(location) {
            Ok(cfg) => {
                self.readonly = cfg.readonly();
                self.cfg = Some(cfg);
                self.error = None;
            }
            Err(e) => {
                self.cfg = None;
                self.error = Some(e);
            }
        }
        self.error
    }

    fn get_profile_from_name(&mut self, name: &String) -> Option<&mut Profile> {
        self.profiles.iter_mut().find(|p| p.name().eq(name))
    }
}

impl App {
    pub(crate) fn new() -> (App, iced::Task<Message>) {
        let conf = Config::new();
        let mut cfg = None;
        let mut readonly = false;
        let mut err = None;
        let profiles = Profile::profiles();
        match Cfg::from_config(&conf) {
            Ok(c) => {
                readonly = c.readonly();
                cfg = Some(c);
            }
            Err(e) => err = Some(e),
        }
        (
            App {
                config: conf,
                cfg,
                readonly,
                error: err,
                profiles,
                success: None,
                champion_id: None,
                connected: false,
                retry_in: None,
            },
            Task::none(),
        )
    }

    pub(crate) fn subscription(&self) -> Subscription<Message> {
        Subscription::run(connect).map(Message::WebsocketEvent)
    }
}
