use std::default;

use chrono::{DateTime, Local, Timelike};
use iced::{
    application, color,
    font::Weight,
    time,
    widget::{button, column, container, pick_list, row, text, Column},
    Alignment::Center,
    Application, Element, Font, Subscription, Theme,
};
#[derive(Debug, Clone, Default)]
struct Timer {
    time: DateTime<Local>,
    theme: Theme,
}
#[derive(Debug, Clone)]
enum Message {
    UpdateTime(DateTime<Local>),
    ChangTheme(iced::Theme),
}
impl Timer {
    // fn new() -> Self {
    //     Timer {
    //         time: Local::now(),
    //         theme: Theme::Dark,
    //     }
    // }
    fn update(&mut self, message: Message) {
        match message {
            Message::UpdateTime(time) => self.time = time,
            Message::ChangTheme(theme) => self.theme = theme,
        }
    }

    fn view(&self) -> Element<Message> {
        let choose_theme = row![
            text("Theme:").size(20),
            pick_list(Theme::ALL, Some(&self.theme), Message::ChangTheme,)
        ];

        let interface = container(
            text(format!(
                "{:02}:{:02}:{:02}",
                self.time.hour(),
                self.time.minute(),
                self.time.second()
            ))
            .size(100)
            .color(color!(0x880808)),
        )
        .center(iced::Length::Fill);
        column![interface, choose_theme].padding(10).into()
    }
    fn subscription(&self) -> Subscription<Message> {
        time::every(time::Duration::from_millis(500)).map(|_| Message::UpdateTime(Local::now()))
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

fn main() -> iced::Result {
    iced::application("Pythonic Timer !!", Timer::update, Timer::view)
        .subscription(Timer::subscription)
        .theme(Timer::theme)
        .run()
}
