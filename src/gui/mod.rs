use iced::{Theme, Element, Sandbox, Settings, Length, window};
use iced::widget::{container, text_input, text, column, scrollable, Column, horizontal_space, vertical_space, row, button};
use crate::db::get_clipboard_content;
// Documentation https://www.youtube.com/watch?v=gcBJ7cPSALo&t=664s
// https://github.com/iced-rs/iced/blob/master/examples/todos/src/main.rs

struct GUI {
    search_content: String,
}

#[derive(Debug, Clone)]
enum Message {
    ContentChanged(String),
    ButtonPressed,
}

impl Sandbox for GUI {
    type Message = Message;

    fn new() -> Self {
        Self {
            search_content: String::new()
        }
    }

    fn title(&self) -> String {
        String::from("Clipboard Manager")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ContentChanged(content) => {
                self.search_content = content;
            }
            Message::ButtonPressed => {
                println!("Button pressed");
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let input_search = text_input("Search", &self.search_content)
            .on_input(Message::ContentChanged);

        let clipboard_items = self.get_clipboard_items();

        let content = column![
            input_search,
            horizontal_space(),
            clipboard_items
        ].spacing(10);

        container(content)
            .padding(20)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::default()
    }
}

impl GUI {
    fn get_clipboard_items(&self) -> Element<'_, Message> {
        let clipboard_items = match get_clipboard_content() {
            Ok(contents) => {
                contents.iter().map(|content| {

                    // Mostrar solo 20 caracteres del contenido
                    let content = if content.len() > 30 {
                        format!("{}...", &content[..30])
                    } else {
                        content.clone()
                    };

                    row![
                        text(content),
                        button("Press me!").on_press(Message::ButtonPressed)
                    ]
                        .height(Length::Shrink)
                        .into()
                }).collect()
            }
            Err(e) => {
                eprintln!("Error retrieving clipboard content: {}", e);
                vec![text("Error retrieving clipboard content").into()]
            }
        };

        scrollable(column![
            Column::with_children(clipboard_items)
        ].spacing(20)
        ).into()
    }
}

pub fn start() {
    let settings = Settings {
        window: window::Settings {
            size: iced::Size::new(450.0, 650.0),
            resizable: false,
            decorations: false,
            ..Default::default()
        },
        ..Default::default()
    };

    match GUI::run(settings) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error running GUI: {}", e);
        }
    }
}
