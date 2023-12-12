use std::{
    fs::OpenOptions,
    io::{Read, Write},
    path::PathBuf,
};

use crate::ui::{init_main_view::MainView, window::Window};

#[derive(Debug)]
pub struct Buffer {
    buffer_id: usize,
    file: PathBuf,
    contents: String,
}

pub struct Editor {
    buffer: Buffer,
    buffers: Vec<Buffer>,
}

impl Buffer {
    pub fn new(file: impl Into<PathBuf>, buffer_id: usize) -> anyhow::Result<Buffer> {
        let file = file.into();
        if file.exists() {
            let mut buffer = Self {
                buffer_id,
                file,
                contents: String::new(),
            };
            OpenOptions::new()
                .read(true)
                .open(&buffer.file)?
                .read_to_string(&mut buffer.contents)?;
            Ok(buffer)
        } else {
            Ok(Self {
                buffer_id,
                file,
                contents: String::new(),
            })
        }
    }

    fn save(&self) -> anyhow::Result<()> {
        let mut open_file = OpenOptions::new().write(true).open(&self.file)?;
        open_file.write(self.contents.as_bytes())?;
        Ok(())
    }
}

impl Editor {
    pub fn new(from_path: impl Into<PathBuf>) -> anyhow::Result<Self> {
        Ok(Self {
            buffer: Buffer::new(from_path, 0)?,
            buffers: Vec::new(),
        })
    }

    pub fn open_file(&mut self, file: PathBuf) -> anyhow::Result<()> {
        let buffer = Buffer::new(file, self.buffers.len())?;
        self.buffer = buffer;
        Ok(())
    }

    pub fn run(self) {
        let window = Window::new();
        loop {}
    }
}
