use iced::keyboard;
use iced::widget::{
    self, button, center, checkbox, column, container, keyed_column, row,
    scrollable, text, text_input, Text,};
use iced::window;
use iced::{Center, Element, Fill, Font, Subscription, Task as Command};

use serde::{Deserialize, Serialize};
use crate::db::get_clipboard_content;

// Code from https://github.com/iced-rs/iced/blob/master/examples/todos/src/main.rs for change.
pub fn show() -> iced::Result {
    tracing_subscriber::fmt::init();

    iced::application(
        "Ropias",
        MainWindows::update,
        MainWindows::view,
    )
        .subscription(MainWindows::subscription)
        .font(include_bytes!("./fonts/icons.ttf").as_slice())
        .window_size((450.0, 650.0))
        .resizable(false)
        .run_with(MainWindows::new)
}

enum MainWindows {
    Loading,
    Loaded(State),
}

#[derive(Debug, Default)]
struct State {
    input_value: String,
    filter: Filter,
    tasks: Vec<ClipboardItemUI>,
    dirty: bool,
    saving: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Loaded(Result<SavedState, LoadError>),
    Saved(Result<(), SaveError>),
    InputChanged(String),
    CreateTask,
    FilterChanged(Filter),
    TaskMessage(usize, ClipboardItemMessage),
    TabPressed { shift: bool },
    ToggleFullscreen(window::Mode),
}

impl MainWindows {
    fn new() -> (Self, Command<Message>) {
        (
            Self::Loading,
            Command::perform(SavedState::load(), Message::Loaded),
        )
    }
    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            MainWindows::Loading => {
                match message {
                    Message::Loaded(Ok(state)) => {
                        *self = MainWindows::Loaded(State {
                            input_value: state.input_value,
                            filter: state.filter,
                            tasks: state.items,
                            ..State::default()
                        });
                    }
                    Message::Loaded(Err(_)) => {
                        *self = MainWindows::Loaded(State::default());
                    }
                    _ => {}
                }

                text_input::focus("new-task")
            }
            MainWindows::Loaded(state) => {
                let mut saved = false;

                let command = match message {
                    Message::InputChanged(value) => {
                        state.input_value = value;

                        Command::none()
                    }
                    Message::CreateTask => {
                        if !state.input_value.is_empty() {
                            state
                                .tasks
                                .push(ClipboardItemUI::new(state.input_value.clone()));
                            state.input_value.clear();
                        }

                        Command::none()
                    }
                    Message::FilterChanged(filter) => {
                        state.filter = filter;

                        Command::none()
                    }
                    Message::TaskMessage(i, ClipboardItemMessage::Delete) => {
                        state.tasks.remove(i);

                        Command::none()
                    }
                    Message::TaskMessage(i, task_message) => {
                        if let Some(task) = state.tasks.get_mut(i) {
                            let should_focus =
                                matches!(task_message, ClipboardItemMessage::Edit);

                            task.update(task_message);

                            if should_focus {
                                let id = ClipboardItemUI::text_input_id(i);
                                Command::batch(vec![
                                    text_input::focus(id.clone()),
                                    text_input::select_all(id),
                                ])
                            } else {
                                Command::none()
                            }
                        } else {
                            Command::none()
                        }
                    }
                    Message::Saved(_result) => {
                        state.saving = false;
                        saved = true;

                        Command::none()
                    }
                    Message::TabPressed { shift } => {
                        if shift {
                            widget::focus_previous()
                        } else {
                            widget::focus_next()
                        }
                    }
                    Message::ToggleFullscreen(mode) => window::get_latest()
                        .and_then(move |window| {
                            window::change_mode(window, mode)
                        }),
                    Message::Loaded(_) => Command::none(),
                };

                if !saved {
                    state.dirty = true;
                }

                let save = if state.dirty && !state.saving {
                    state.dirty = false;
                    state.saving = true;

                    Command::perform(
                        SavedState {
                            input_value: state.input_value.clone(),
                            filter: state.filter,
                            items: state.tasks.clone(),
                        }
                            .save(),
                        Message::Saved,
                    )
                } else {
                    Command::none()
                };

                Command::batch(vec![command, save])
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match self {
            MainWindows::Loading => loading_message(),
            MainWindows::Loaded(State {
                                    input_value,
                                    filter,
                                    tasks,
                                    ..
                                }) => {
                let input = text_input("What needs to be done?", input_value)
                    .id("new-task")
                    .on_input(Message::InputChanged)
                    .on_submit(Message::CreateTask)
                    .padding(15)
                    .size(30)
                    .align_x(Center);

                let controls = view_controls(tasks, *filter);
                let filtered_tasks =
                    tasks.iter().filter(|task| filter.matches(task));

                let tasks: Element<_> = if filtered_tasks.count() > 0 {
                    keyed_column(
                        tasks
                            .iter()
                            .enumerate()
                            .filter(|(_, task)| filter.matches(task))
                            .map(|(i, task)| {
                                (
                                    task.id,
                                    task.view(i).map(move |message| {
                                        Message::TaskMessage(i, message)
                                    }),
                                )
                            }),
                    )
                        .spacing(10)
                        .into()
                } else {
                    empty_message(match filter {
                        Filter::All => "You have not created a task yet...",
                        Filter::Active => "All your tasks are done! :D",
                        Filter::Completed => {
                            "You have not completed a task yet..."
                        }
                    })
                };

                let content = column![input, controls, tasks]
                    .spacing(20)
                    .max_width(800);

                scrollable(container(content).center_x(Fill).padding(40)).into()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        use keyboard::key;

        keyboard::on_key_press(|key, modifiers| {
            let keyboard::Key::Named(key) = key else {
                return None;
            };

            match (key, modifiers) {
                (key::Named::Tab, _) => Some(Message::TabPressed {
                    shift: modifiers.shift(),
                }),
                (key::Named::ArrowUp, keyboard::Modifiers::SHIFT) => {
                    Some(Message::ToggleFullscreen(window::Mode::Fullscreen))
                }
                (key::Named::ArrowDown, keyboard::Modifiers::SHIFT) => {
                    Some(Message::ToggleFullscreen(window::Mode::Windowed))
                }
                _ => None,
            }
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ClipboardItemUI {
    id: i32,
    content: String,
    completed: bool,
    is_focused: Option<Focus>,
    #[serde(skip)]
    state: ClipboardItemState,
}

#[derive(Debug, Clone)]
pub enum ClipboardItemState {
    Idle,
    Editing,
}

impl Default for ClipboardItemState {
    fn default() -> Self {
        Self::Idle
    }
}

#[derive(Debug, Clone)]
pub enum ClipboardItemMessage {
    Completed(bool),
    Edit,
    DescriptionEdited(String),
    FinishEdition,
    Delete,
}

impl ClipboardItemUI {
    fn text_input_id(i: usize) -> text_input::Id {
        text_input::Id::new(format!("task-{i}"))
    }

    fn new(description: String) -> Self {
        ClipboardItemUI {
            id: 1,
            content: description,
            completed: false,
            state: ClipboardItemState::Idle,
            is_focused: None,
        }
    }

    fn update(&mut self, message: ClipboardItemMessage) {
        match message {
            ClipboardItemMessage::Completed(completed) => {
                self.completed = completed;
            }
            ClipboardItemMessage::Edit => {
                self.state = ClipboardItemState::Editing;
            }
            ClipboardItemMessage::DescriptionEdited(new_description) => {
                self.content = new_description;
            }
            ClipboardItemMessage::FinishEdition => {
                if !self.content.is_empty() {
                    self.state = ClipboardItemState::Idle;
                }
            }
            ClipboardItemMessage::Delete => {}
        }
    }

    fn view(&self, i: usize) -> Element<ClipboardItemMessage> {
        match &self.state {
            ClipboardItemState::Idle => {
                let content = if self.content.len() > 30 {
                    format!("{} ...", &self.content[..30])
                } else {
                    self.content.clone()
                };

                let checkbox = checkbox(content, self.completed)
                    .on_toggle(ClipboardItemMessage::Completed)
                    .width(Fill)
                    .size(17)
                    .text_shaping(text::Shaping::Advanced);

                row![
                    checkbox,
                    button(edit_icon())
                        .on_press(ClipboardItemMessage::Edit)
                        .padding(10)
                        .style(button::text),
                ]
                    .spacing(20)
                    .align_y(Center)
                    .into()
            }
            ClipboardItemState::Editing => {
                let text_input =
                    text_input("Describe your task...", &self.content)
                        .id(Self::text_input_id(i))
                        .on_input(ClipboardItemMessage::DescriptionEdited)
                        .on_submit(ClipboardItemMessage::FinishEdition)
                        .padding(10);

                row![
                    text_input,
                    button(
                        row![delete_icon(), "Delete"]
                            .spacing(10)
                            .align_y(Center)
                    )
                    .on_press(ClipboardItemMessage::Delete)
                    .padding(10)
                    .style(button::danger)
                ]
                    .spacing(20)
                    .align_y(Center)
                    .into()
            }
        }
    }
}

impl operation::Focusable for ClipboardItemState {
    fn is_focused(&self) -> bool {
        ClipboardItemState::is_focused(self)
    }

    fn focus(&mut self) {
        ClipboardItemState::focus(self);
    }

    fn unfocus(&mut self) {
        ClipboardItemState::unfocus(self);
    }
}

// impl Focusable for ClipboardItemUI {
//     fn is_focused(&self) -> bool;
//     fn focus(&mut self);
//     fn unfocus(&mut self);
// }
//
// impl<Highlighter: text::Highlighter> operation::Focusable for State<Highlighter>
// {
//     fn is_focused(&self) -> bool {
//         self.focus.is_some()
//     }
//
//     fn focus(&mut self) {
//         self.focus = Some(Focus::now());
//     }
//
//     fn unfocus(&mut self) {
//         self.focus = None;
//     }
// }

fn view_controls(tasks: &[ClipboardItemUI], current_filter: Filter) -> Element<Message> {
    let tasks_left = tasks.iter().filter(|task| !task.completed).count();

    let filter_button = |label, filter, current_filter| {
        let label = text(label);

        let button = button(label).style(if filter == current_filter {
            button::primary
        } else {
            button::text
        });

        button.on_press(Message::FilterChanged(filter)).padding(8)
    };

    row![
        text!(
            "{tasks_left} {} left",
            if tasks_left == 1 { "task" } else { "tasks" }
        )
        .width(Fill),
        row![
            filter_button("All", Filter::All, current_filter),
            filter_button("Active", Filter::Active, current_filter),
            filter_button("Completed", Filter::Completed, current_filter,),
        ]
        .spacing(10)
    ]
        .spacing(20)
        .align_y(Center)
        .into()
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize,
)]
pub enum Filter {
    #[default]
    All,
    Active,
    Completed,
}

impl Filter {
    fn matches(self, task: &ClipboardItemUI) -> bool {
        match self {
            Filter::All => true,
            Filter::Active => !task.completed,
            Filter::Completed => task.completed,
        }
    }
}

fn loading_message<'a>() -> Element<'a, Message> {
    center(text("Loading...").width(Fill).align_x(Center).size(50)).into()
}

fn empty_message(message: &str) -> Element<'_, Message> {
    center(
        text(message)
            .width(Fill)
            .size(25)
            .align_x(Center)
            .color([0.7, 0.7, 0.7]),
    )
        .height(200)
        .into()
}

// Fonts
const ICONS: Font = Font::with_name("Iced-Todos-Icons");

fn icon(unicode: char) -> Text<'static> {
    text(unicode.to_string())
        .font(ICONS)
        .width(20)
        .align_x(Center)
}

fn edit_icon() -> Text<'static> {
    icon('\u{F303}')
}

fn delete_icon() -> Text<'static> {
    icon('\u{F1F8}')
}

// Persistence
#[derive(Debug, Clone, Serialize, Deserialize)]
struct SavedState {
    input_value: String,
    filter: Filter,
    items: Vec<ClipboardItemUI>,
}

#[derive(Debug, Clone)]
enum LoadError {
    File,
    Format,
}

#[derive(Debug, Clone)]
enum SaveError {
    File,
    Write,
    Format,
}

impl SavedState {
    async fn load() -> Result<SavedState, LoadError> {
        match get_clipboard_content() {
            Ok(items) =>
                Ok(SavedState {
                    input_value: "".to_string(),
                    filter: Filter::All,
                    items: items.into_iter().map(|item| ClipboardItemUI {
                        id: item.id,
                        content: item.content,
                        completed: false,
                        state: ClipboardItemState::Idle,
                        is_focused: None,
                    }).collect(),
                }),
            Err(_) => Err(LoadError::Format),
        }
    }

    async fn save(self) -> Result<(), SaveError> {
        // @todo: Implementar esta parte
        Ok(())
    }
}