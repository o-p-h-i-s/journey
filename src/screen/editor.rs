use std::{fs, path::PathBuf};

use iced::{
    futures::io,
    widget::{button, column, container, horizontal_space, row, text, text_editor},
    Element, Length,
};

#[derive(Debug, Clone)]
pub enum Message {
    Edit(text_editor::Action),
    New,
    Open,
    Save,
}

pub struct Editor {
    path: Option<PathBuf>,
    contents: text_editor::Content,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            path: None,
            contents: text_editor::Content::new(),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Edit(action) => self.contents.perform(action),
            Message::New => {
                self.path = None;
                self.contents = text_editor::Content::new();
            }
            Message::Open => {
                if let Ok((path, contents)) = open_file() {
                    self.path = Some(path);
                    self.contents = text_editor::Content::with_text(&contents);
                }
            }
            Message::Save => {
                if let Ok(path) = save_file(self.path.clone(), self.contents.text()) {
                    self.path = Some(path);
                }
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let new = button("New").on_press(Message::New);
        let open = button("Open").on_press(Message::Open);
        let save = button("Save").on_press(Message::Save);

        let controls = row![horizontal_space(), new, open, save].spacing(5);

        let editor = text_editor(&self.contents)
            .on_action(Message::Edit)
            .height(Length::Fill);

        let cursol_position = {
            let file_type = if let Some(ext) = self
                .path
                .as_ref()
                .and_then(|path| path.extension())
                .and_then(|ext| ext.to_str())
            {
                ext
            } else {
                "None"
            };
            let (line, column) = self.contents.cursor_position();
            text!("{}/{}:{}", file_type, line, column)
        };
        let status_bar = row![horizontal_space(), cursol_position];

        let main = column![controls, editor, status_bar].spacing(5);

        container(main).padding(5).into()
    }
}

#[derive(Debug)]
enum Error {
    FailedPickFile,
    FailedLoadFile(io::Error),
    FailedSaveFile(io::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::FailedPickFile => write!(f, "Failed to pick file"),
            Error::FailedLoadFile(err) => write!(f, "Failed to load file: {:?}", err),
            Error::FailedSaveFile(err) => write!(f, "Failed to save file: {:?}", err),
        }
    }
}

impl std::error::Error for Error {}

fn open_file() -> Result<(PathBuf, String), Error> {
    let path = rfd::FileDialog::new()
        .set_directory("./")
        .pick_file()
        .ok_or(Error::FailedPickFile)?;
    let contents = fs::read_to_string(&path).map_err(|err| Error::FailedLoadFile(err))?;

    Ok((path, contents))
}

fn save_file(path: Option<PathBuf>, contents: String) -> Result<PathBuf, Error> {
    let path = if let Some(path) = path {
        path
    } else {
        rfd::FileDialog::new()
            .set_directory("./")
            .save_file()
            .ok_or(Error::FailedPickFile)?
    };
    fs::write(&path, contents).map_err(|err| Error::FailedSaveFile(err))?;

    Ok(path)
}
