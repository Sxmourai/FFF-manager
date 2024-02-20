use std::{env::current_dir, fs::DirEntry, path::PathBuf};

use iced::{
    widget::{button, container, row, text}, *
};
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Message {
    ChangeDir(PathBuf),
    OpenFile(PathBuf),
}

pub struct App {
    current_dir: PathBuf,
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self {
            current_dir: current_dir().unwrap(),
        }
    }

    fn title(&self) -> String {
        "A Freaking Fast File manager".into()
    }

    fn update(&mut self, message: Self::Message) {
        #[allow(unreachable_patterns)]
        match message {
            Self::Message::ChangeDir(dir) => self.current_dir = self.current_dir.join(dir),
            Self::Message::OpenFile(dir) => {
                println!("Opening file {dir:?}")
            }
            _ => {
                todo!()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let files = files(&self.current_dir)
            .unwrap()
            .enumerate()
            .map(|(_i, entry)| {
                let _name = entry.file_name(); // Binding
                let name = _name.to_string_lossy();
                let button = button(text(&name)).width(200);
                let button = if entry.file_type().unwrap().is_dir() {
                    button.on_press(Message::ChangeDir(name.to_string().into()))
                } else {
                    button.on_press(Message::OpenFile(name.to_string().into()))
                };
                container(button).padding(2.5).into()
            });
        widget::column!(self.navbar(), row!(widget::column(files).padding(20))).into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }

    fn style(&self) -> iced::theme::Application {
        iced::theme::Application::default()
    }
}
impl App {
    fn navbar<'a>(&self) -> Element<'a, <Self as Sandbox>::Message, Theme, Renderer> {
        text(self.current_dir.to_string_lossy()).into()
    }
}
fn files(path: &PathBuf) -> Option<impl Iterator<Item = DirEntry>> {
    Some(std::fs::read_dir(path).unwrap().filter_map(|f| f.ok()))
}
// 0,
// 40 * i as i32,
// surface.width(),
// 40,
// file.file_name().to_string_lossy().to_string().as_str(),
// );
