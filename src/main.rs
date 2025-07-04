use std::{fs, path::PathBuf};

use iced::{
    Element,
    Length::Fill,
    Task,
    widget::{button, column, row, text},
    window,
};

#[derive(Debug)]
struct AppState {
    current_dir: PathBuf,
    current_files: Vec<(String, bool)>,
}

impl Default for AppState {
    fn default() -> Self {
        let current_dir = std::env::current_dir().unwrap();
        let current_files = get_files(&current_dir);

        // Transferring ownership of the current directory variable does not
        // allow it to be read by the current files directory so instead,
        // reading it before hand and performing the computation through a
        // borrow makes it easier and then we pass the ownership of both the
        // variables to the struct AppState{}
        AppState {
            current_dir,
            current_files,
        }
    }
}

// Elm Architecture
#[derive(Debug, Clone)]
enum Message {
    Exit,
}

fn update(_state: &mut AppState, message: Message) -> Task<Message> {
    match message {
        Message::Exit => window::get_latest().and_then(window::close),
    }
}

fn view(state: &AppState) -> Element<Message> {
    let mut content = column![
        row![
            text(state.current_dir.to_str().unwrap_or("unknown directory"))
                .size(24)
                .width(Fill),
            button(text("Up").size(24)).on_press(Message::Exit),
            button(text("Exit").size(24)).on_press(Message::Exit)
        ]
        .spacing(8)
    ];
    for file in &state.current_files {
        content = content.push(text(&file.0));
    }
    content.into()
}

fn main() -> iced::Result {
    iced::application("Jrip", update, view)
        .theme(|_s| iced::Theme::KanagawaDragon)
        .run()
}

fn get_files(path: &PathBuf) -> Vec<(String, bool)> {
    let mut dirs = Vec::default();
    let mut files = Vec::default();

    if let Ok(read_dir) = fs::read_dir(path) {
        for read in read_dir {
            if let Ok(dir_entry) = read {
                if let Some(name) = dir_entry.file_name().to_str() {
                    if dir_entry.path().is_dir() {
                        dirs.push((name.to_string(), true));
                    } else {
                        files.push((name.to_string(), false));
                    }
                }
            }
        }
    }

    dirs.append(&mut files);
    dirs
}
