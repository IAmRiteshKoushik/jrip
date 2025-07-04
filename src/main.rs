use std::{fs, path::PathBuf, process::Command};

use iced::{
    Border, Element,
    Length::Fill,
    Shadow, Task,
    widget::{button, column, horizontal_rule, row, text},
    window,
};

#[derive(Debug)]
struct AppState {
    current_dir: PathBuf,
    current_files: Vec<(String, bool)>,
    popup: Option<String>,
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
            popup: None,
        }
    }
}

// Elm Architecture
#[derive(Debug, Clone)]
enum Message {
    Exit,
    CD(PathBuf),
    JRIP(PathBuf),
    ClosePopup,
}

fn update(state: &mut AppState, message: Message) -> Task<Message> {
    match message {
        Message::Exit => window::get_latest().and_then(window::close),
        Message::CD(path_buf) => {
            state.current_dir = path_buf;
            state.current_files = get_files(&state.current_dir);
            Task::none()
        }
        Message::JRIP(path_buf) => {
            if let Some(parent) = path_buf.parent() {
                let mut new_file = parent.to_path_buf();
                new_file.push("output.mp3");

                if let Ok(output) = Command::new("ffmpeg")
                    .args([
                        "-i",
                        path_buf.to_str().unwrap_or("/home"),
                        "-vn",
                        "-acodec",
                        "libmp3lame",
                        "-q:a",
                        "4",
                        new_file.to_str().unwrap_or("/home"),
                    ])
                    .status()
                {
                    if output.success() {
                        state.popup =
                            Some(String::from("audio has been ripped"))
                    } else {
                        state.popup = Some(String::from("failed to RIP audio"))
                    }
                }
            }
            Task::none()
        }
        Message::ClosePopup => {
            state.popup = None;
            Task::none()
        }
    }
}

fn view(state: &AppState) -> Element<Message> {
    let mut content = column![
        row![
            text(state.current_dir.to_str().unwrap_or("unknown directory"))
                .size(24)
                .width(Fill),
            button(text("Up").size(24)).on_press(Message::CD(
                state
                    .current_dir
                    .parent()
                    .unwrap_or(&state.current_dir)
                    .to_path_buf()
            )),
            button(text("Exit").size(24)).on_press(Message::Exit)
        ]
        .spacing(8)
    ]
    .spacing(2)
    .padding(4);

    content = content.push(horizontal_rule(2));

    if let Some(pat) = &state.popup {
        content = content.push(row![
            text(pat).width(Fill),
            button("close").on_press(Message::ClosePopup)
        ]);
    }

    for file in &state.current_files {
        let file_name = text(&file.0);
        let mut file_path = state.current_dir.clone();
        file_path.push(&file.0);

        // If the file is a directory, push a button in the content list
        if file.1 {
            content = content.push(
                button(file_name)
                    .style(dir_button_style())
                    .on_press(Message::CD(file_path)),
            );
        // If it is not a directory, then push the file_name in the content list
        } else {
            content = content.push(row![
                file_name.width(Fill),
                button(text("Jrip")).on_press(Message::JRIP(file_path)),
            ]);
        }
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
                        if name.ends_with("mkv") {
                            files.push((name.to_string(), false));
                        }
                    }
                }
            }
        }
    }

    dirs.append(&mut files);
    dirs
}

fn dir_button_style() -> impl Fn(&iced::Theme, button::Status) -> button::Style
{
    |_t, _e| button::Style {
        background: None,
        text_color: iced::Color::from_rgb(
            3.0 / 255.0,
            161.0 / 255.0,
            252.0 / 255.0,
        ),
        border: Border::default(),
        shadow: Shadow::default(),
    }
}
