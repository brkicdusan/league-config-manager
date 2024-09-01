#![windows_subsystem = "windows"]

use std::path::PathBuf;

use iced::{
    executor,
    widget::{button, column, container, row, text, text_input},
    Application, Command, Sandbox, Settings, Theme,
};

fn main() -> Result<(), iced::Error> {
    Window::run(Settings::default())
}

#[derive(Debug, Clone)]
enum Message {
    FindLocation,
    SetLocation(Result<PathBuf, Error>),
}

struct Window {
    config_path: Option<PathBuf>,
}

impl Window {
    fn config_path(&self) -> &str {
        match &self.config_path {
            Some(path) => path.to_str().expect("Should always work"),
            _ => "",
        }
    }
}

impl Application for Window {
    type Message = Message;
    type Executor = executor::Default;

    type Theme = Theme;

    type Flags = ();

    fn new(_flags: Self::Flags) -> (Window, iced::Command<Message>) {
        (Window { config_path: None }, Command::none())
    }

    fn title(&self) -> String {
        String::from("League Config Manager")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::FindLocation => Command::perform(find_config(), Message::SetLocation),
            Message::SetLocation(Ok(location)) => {
                self.config_path = Some(location);
                Command::none()
            }
            Message::SetLocation(Err(error)) => {
                println!("{:?}", error);
                //TODO: error handling
                Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let config_path = text_input("Config not found", self.config_path()).padding(10);
        let location_btn = button(text("Find config"))
            .padding(10)
            .on_press(Message::FindLocation);
        let location = row![config_path, location_btn]
            .align_items(iced::Alignment::Center)
            .spacing(10);

        container(column![location])
            .padding(10)
            .center_x()
            .center_y()
            .into()
    }
}

#[derive(Debug, Clone)]
enum Error {
    DialogClosed,
}

/// Opens dialog to locate config.
///
/// # Errors
/// When dialog closes return Error::DialogClosed
async fn find_config() -> Result<std::path::PathBuf, Error> {
    let handle = rfd::AsyncFileDialog::new()
        .set_title("Find settings location")
        // .pick_folder()
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;
    Ok(handle.path().to_owned())
}
