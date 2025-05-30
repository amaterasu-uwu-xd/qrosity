use iced::widget::svg::Handle;
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
                row![svg(Handle::from_memory(
                    "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
                    <!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd\">
                    <svg xmlns=\"http://www.w3.org/2000/svg\" version=\"1.1\" viewBox=\"0 0 29 29\" stroke=\"none\">
                        <rect width=\"100%\" height=\"100%\" fill=\"#FFFFFF\"/>
                        <path d=\"M4,4h1v1h-1z M5,4h1v1h-1z M6,4h1v1h-1z M7,4h1v1h-1z M8,4h1v1h-1z M9,4h1v1h-1z M10,4h1v1h-1z M12,4h1v1h-1z M13,4h1v1h-1z M18,4h1v1h-1z M19,4h1v1h-1z M20,4h1v1h-1z M21,4h1v1h-1z M22,4h1v1h-1z M23,4h1v1h-1z M24,4h1v1h-1z M4,5h1v1h-1z M10,5h1v1h-1z M12,5h1v1h-1z M13,5h1v1h-1z M14,5h1v1h-1z M16,5h1v1h-1z M18,5h1v1h-1z M24,5h1v1h-1z M4,6h1v1h-1z M6,6h1v1h-1z M7,6h1v1h-1z M8,6h1v1h-1z M10,6h1v1h-1z M12,6h1v1h-1z M14,6h1v1h-1z M15,6h1v1h-1z M18,6h1v1h-1z M20,6h1v1h-1z M21,6h1v1h-1z M22,6h1v1h-1z M24,6h1v1h-1z M4,7h1v1h-1z M6,7h1v1h-1z M7,7h1v1h-1z M8,7h1v1h-1z M10,7h1v1h-1z M13,7h1v1h-1z M14,7h1v1h-1z M18,7h1v1h-1z M20,7h1v1h-1z M21,7h1v1h-1z M22,7h1v1h-1z M24,7h1v1h-1z M4,8h1v1h-1z M6,8h1v1h-1z M7,8h1v1h-1z M8,8h1v1h-1z M10,8h1v1h-1z M12,8h1v1h-1z M14,8h1v1h-1z M16,8h1v1h-1z M18,8h1v1h-1z M20,8h1v1h-1z M21,8h1v1h-1z M22,8h1v1h-1z M24,8h1v1h-1z M4,9h1v1h-1z M10,9h1v1h-1z M15,9h1v1h-1z M16,9h1v1h-1z M18,9h1v1h-1z M24,9h1v1h-1z M4,10h1v1h-1z M5,10h1v1h-1z M6,10h1v1h-1z M7,10h1v1h-1z M8,10h1v1h-1z M9,10h1v1h-1z M10,10h1v1h-1z M12,10h1v1h-1z M14,10h1v1h-1z M16,10h1v1h-1z M18,10h1v1h-1z M19,10h1v1h-1z M20,10h1v1h-1z M21,10h1v1h-1z M22,10h1v1h-1z M23,10h1v1h-1z M24,10h1v1h-1z M14,11h1v1h-1z M15,11h1v1h-1z M16,11h1v1h-1z M4,12h1v1h-1z M7,12h1v1h-1z M8,12h1v1h-1z M9,12h1v1h-1z M10,12h1v1h-1z M11,12h1v1h-1z M12,12h1v1h-1z M13,12h1v1h-1z M16,12h1v1h-1z M17,12h1v1h-1z M20,12h1v1h-1z M22,12h1v1h-1z M23,12h1v1h-1z M24,12h1v1h-1z M11,13h1v1h-1z M12,13h1v1h-1z M13,13h1v1h-1z M14,13h1v1h-1z M16,13h1v1h-1z M17,13h1v1h-1z M21,13h1v1h-1z M22,13h1v1h-1z M4,14h1v1h-1z M5,14h1v1h-1z M6,14h1v1h-1z M7,14h1v1h-1z M8,14h1v1h-1z M9,14h1v1h-1z M10,14h1v1h-1z M12,14h1v1h-1z M17,14h1v1h-1z M22,14h1v1h-1z M23,14h1v1h-1z M24,14h1v1h-1z M4,15h1v1h-1z M7,15h1v1h-1z M8,15h1v1h-1z M9,15h1v1h-1z M15,15h1v1h-1z M17,15h1v1h-1z M19,15h1v1h-1z M22,15h1v1h-1z M8,16h1v1h-1z M10,16h1v1h-1z M11,16h1v1h-1z M13,16h1v1h-1z M20,16h1v1h-1z M21,16h1v1h-1z M23,16h1v1h-1z M12,17h1v1h-1z M16,17h1v1h-1z M17,17h1v1h-1z M18,17h1v1h-1z M19,17h1v1h-1z M21,17h1v1h-1z M23,17h1v1h-1z M24,17h1v1h-1z M4,18h1v1h-1z M5,18h1v1h-1z M6,18h1v1h-1z M7,18h1v1h-1z M8,18h1v1h-1z M9,18h1v1h-1z M10,18h1v1h-1z M12,18h1v1h-1z M13,18h1v1h-1z M14,18h1v1h-1z M16,18h1v1h-1z M17,18h1v1h-1z M18,18h1v1h-1z M20,18h1v1h-1z M22,18h1v1h-1z M4,19h1v1h-1z M10,19h1v1h-1z M12,19h1v1h-1z M15,19h1v1h-1z M16,19h1v1h-1z M17,19h1v1h-1z M18,19h1v1h-1z M20,19h1v1h-1z M21,19h1v1h-1z M22,19h1v1h-1z M24,19h1v1h-1z M4,20h1v1h-1z M6,20h1v1h-1z M7,20h1v1h-1z M8,20h1v1h-1z M10,20h1v1h-1z M12,20h1v1h-1z M13,20h1v1h-1z M15,20h1v1h-1z M16,20h1v1h-1z M19,20h1v1h-1z M21,20h1v1h-1z M23,20h1v1h-1z M4,21h1v1h-1z M6,21h1v1h-1z M7,21h1v1h-1z M8,21h1v1h-1z M10,21h1v1h-1z M12,21h1v1h-1z M15,21h1v1h-1z M17,21h1v1h-1z M18,21h1v1h-1z M4,22h1v1h-1z M6,22h1v1h-1z M7,22h1v1h-1z M8,22h1v1h-1z M10,22h1v1h-1z M13,22h1v1h-1z M14,22h1v1h-1z M15,22h1v1h-1z M16,22h1v1h-1z M17,22h1v1h-1z M18,22h1v1h-1z M19,22h1v1h-1z M20,22h1v1h-1z M21,22h1v1h-1z M22,22h1v1h-1z M23,22h1v1h-1z M24,22h1v1h-1z M4,23h1v1h-1z M10,23h1v1h-1z M16,23h1v1h-1z M18,23h1v1h-1z M19,23h1v1h-1z M20,23h1v1h-1z M21,23h1v1h-1z M22,23h1v1h-1z M23,23h1v1h-1z M24,23h1v1h-1z M4,24h1v1h-1z M5,24h1v1h-1z M6,24h1v1h-1z M7,24h1v1h-1z M8,24h1v1h-1z M9,24h1v1h-1z M10,24h1v1h-1z M12,24h1v1h-1z M13,24h1v1h-1z M15,24h1v1h-1z M16,24h1v1h-1z M17,24h1v1h-1z M18,24h1v1h-1z\" fill=\"#000000\"/>
                    </svg>"
                .as_bytes()
                ))
                    .width(Length::Fill).height(Length::Fill)]
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
