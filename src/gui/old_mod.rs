// use iced::{Theme, Sandbox, Settings, Length, window, Element, Font, Subscription, Task};
// use iced::widget::{container, text_input, text, column, scrollable, Column, horizontal_space, row, button, keyed_column, checkbox};
// use serde::{Deserialize, Serialize};
// use crate::db::get_clipboard_content;
// // Documentation https://www.youtube.com/watch?v=gcBJ7cPSALo&t=664s
// // https://github.com/iced-rs/iced/blob/master/examples/todos/src/main.rs
//
// #[derive(Debug)]
// enum GUI {
//     Loading,
//     Loaded(State),
// }
//
// #[derive(Debug, Default)]
// struct State {
//     input_value: String,
//     filter: Filter,
//     items: Vec<Item>,
//     dirty: bool,
//     saving: bool,
// }
//
// #[derive(Debug, Clone)]
// enum Message {
//     ContentChanged(String),
//     ButtonPressed,
//     Loaded(Result<SavedState, LoadError>),
// }
//
// impl Sandbox for GUI {
//     type Message = Message;
//
//     fn new() -> Self {
//         Self {
//             Self::Loading,
//             Task::perform(SavedState::load(), Message::Loaded)
//         }
//     }
//
//     fn title(&self) -> String {
//         String::from("Clipboard Manager")
//     }
//
//     fn update(&mut self, message: Message) {
//         match message {
//             Message::ContentChanged(content) => {
//                 self.search_content = content;
//             }
//             Message::ButtonPressed => {
//                 println!("Button pressed");
//             }
//         }
//     }
//
//     fn view(&self) -> Element<'_, Message> {
//         let input_search = text_input("Search", &self.search_content)
//             .on_input(Message::ContentChanged);
//
//         let clipboard_items = self.get_clipboard_items();
//
//         let content = column![
//             input_search,
//             horizontal_space(),
//             clipboard_items
//         ].spacing(10);
//
//         container(content)
//             .padding(20)
//             .into()
//     }
//
//     fn theme(&self) -> Theme {
//         Theme::default()
//     }
// }
//
// impl GUI {
//     fn get_clipboard_items(&self) -> Element<'_, Message> {
//         let clipboard_items = match get_clipboard_content() {
//             Ok(contents) => {
//                 contents.iter().map(|content| {
//                     // Mostrar solo 20 caracteres del contenido
//                     let content = if content.len() > 30 {
//                         format!("{}...", &content[..30])
//                     } else {
//                         content.clone()
//                     };
//
//                     (
//                         task.id,
//                         task.view(i).map(move |message| {
//                             Message::TaskMessage(i, message)
//                         }),
//                     )
//
//                     // row![
//                     //     text(content),
//                     //     button("Press me!").on_press(Message::ButtonPressed)
//                     // ]
//                     //     .height(Length::Shrink)
//                     //     .into()
//                 }).collect()
//             }
//             Err(e) => {
//                 eprintln!("Error retrieving clipboard content: {}", e);
//                 vec![text("Error retrieving clipboard content").into()]
//             }
//         };
//
//         scrollable(keyed_column(
//             clipboard_items
//         ).spacing(20)
//         ).into()
//     }
// }
//
// #[derive(Debug, Clone, Serialize, Deserialize)]
// struct Item {
//     id: i32,
//     text: String,
//     favorite: bool,
//
//     #[serde(skip)]
//     state: ItemState,
// }
//
// #[derive(Debug, Clone)]
// pub enum ItemState {
//     Idle
// }
//
// impl Default for ItemState {
//     fn default() -> Self {
//         Self::Idle
//     }
// }
//
// #[derive(Debug, Clone)]
// pub enum ItemMessage {
//     FavoriteToggle(bool),
//     Delete,
// }
//
// impl Item {
//     fn update(&mut self, message: ItemMessage) {
//         match message {
//             ItemMessage::FavoriteToggle(favorite) => {
//                 self.favorite = favorite;
//             }
//             ItemMessage::Delete => {}
//         }
//     }
//
//     fn view(&self, i: usize) -> Element<ItemMessage> {
//         match &self.state {
//             ItemState::Idle => {
//                 let checkbox = checkbox(&self.text, false)
//                     .on_toggle(ItemMessage::FavoriteToggle)
//                     .width(Fill)
//                     .size(17)
//                     .text_shaping(text::Shaping::Advanced);
//
//                 row![
//                     checkbox,
//                     // button(edit_icon())
//                     //     .on_press(TaskMessage::Edit)
//                     //     .padding(10)
//                     //     .style(button::text),
//                 ]
//                     .spacing(20)
//                     .align_y(Center)
//                     .into()
//             }
//         }
//     }
// }
//
// // Persistence
// #[derive(Debug, Clone, Serialize, Deserialize)]
// struct SavedState {
//     input_value: String,
//     items: Vec<Item>,
// }
//
// #[derive(Debug, Clone)]
// enum LoadError {
//     DbError,
// }
//
// impl SavedState{
//     async fn load() -> Result<SavedState, LoadError> {
//        match get_clipboard_content(){
//            Ok(contents) => {
//                let items = contents.iter().map(|clipboard_item| {
//                    Item {
//                        id: clipboard_item.id,
//                        text: clipboard_item.content.clone(),
//                        favorite: clipboard_item.favorite
//                    }
//                }).collect();
//                Ok(SavedState {
//                    input_value: String::new(),
//                    items,
//                })
//            }
//            Err(_) => Err(LoadError::DbError),
//        }
//     }
// }
//
// #[derive(
//     Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize,
// )]
// pub enum Filter {
//     #[default]
//     All,
//     Active,
//     Completed,
// }
//
// pub fn start() {
//     let settings = Settings {
//         window: window::Settings {
//             size: iced::Size::new(450.0, 650.0),
//             resizable: false,
//             decorations: false,
//             ..Default::default()
//         },
//         ..Default::default()
//     };
//
//     match GUI::run(settings) {
//         Ok(_) => {}
//         Err(e) => {
//             eprintln!("Error running GUI: {}", e);
//         }
//     }
// }
