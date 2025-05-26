use iced::widget::{Row, Space, button, column, row, svg, text};
use iced::{Length, Theme, alignment};

#[derive(Default)]
struct QrGenerator {
    exist_qr: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}

impl QrGenerator {
    pub fn view(&self) -> Row<Message> {
        // We use a column: a simple vertical layout
        row![
            row![
                column![
                    button("+").on_press(Message::Increment),
                    text(self.exist_qr).size(50),
                    button("-").on_press(Message::Decrement),
                ]
                .width(Length::Fill)
                .height(Length::Fill),
            ]
            .width(Length::Fill)
            .height(Length::Fill)
            .align_y(alignment::Vertical::Center),
            Space::with_width(20),
            if self.exist_qr {
                row![svg("output.svg").width(Length::Fill).height(Length::Fill)]
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_y(alignment::Vertical::Center)
            } else {
                row![text("No QR code generated yet").size(20)]
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_y(alignment::Vertical::Center)
            }
        ]
        .padding(20)
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.exist_qr = true;
            }
            Message::Decrement => {
                self.exist_qr = false;
            }
        }
    }

    fn theme(&self) -> Theme {
        Theme::CatppuccinMocha
    }
}

pub fn handle() {
    let _ = iced::application("A cool counter", QrGenerator::update, QrGenerator::view)
        .theme(QrGenerator::theme)
        .run();
}
