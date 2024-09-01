use iced::Sandbox;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
enum Message {}

struct Window;

impl Sandbox for Window {
    type Message = Message;

    fn new() -> Self {
        todo!()
    }

    fn title(&self) -> String {
        todo!()
    }

    fn update(&mut self, message: Self::Message) {
        todo!()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        todo!()
    }
}
