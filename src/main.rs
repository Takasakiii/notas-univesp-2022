use iced::{window, Application, Settings};

mod application;

fn main() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
            size: (500, 250),
            resizable: false,
            ..Default::default()
        },
        ..Default::default()
    };

    application::Application::run(settings)
}
