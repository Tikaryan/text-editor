use std::io::{self, Error, stdout, Write};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use crate::Position;

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: RawTerminal<io::Stdout>,
}

impl Terminal {
    pub fn default() -> Result<Self, Error> {
        let size = termion::terminal_size()?;
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1,
            },
            _stdout: stdout().into_raw_mode()?,
        })
    }

    pub fn size(&self) -> &Size{
        &self.size
    }

    // clear the terminal
    pub fn clear_screen() {
        print!("{}", termion::clear::All);
    }

    pub fn cursor_show() {
        print!("{}", termion::cursor::Show);
    }
    pub fn cursor_hide() {
        print!("{}", termion::cursor::Hide);
    }

    // reposition the cursor at the top-left corner
    #[allow(clippy::cast_possible_truncation)]
    pub fn cursor_position(position: &Position) {
        let Position { x, y } = position;
        let x = x.saturating_add(1);
        let y = y.saturating_add(1);
        print!("{}", termion::cursor::Goto(x as u16, y as u16));
    }

    // flush the output buffer to ensure the message is printed
    pub fn flush() -> Result<(), Error> {
        stdout().flush()
    }

    pub fn read_key() -> Result<Key, Error> {
        loop {
            if let Some(key) = io::stdin().keys().next() {
                return key;
            }
        }
    }

    pub fn clear_current_line() {
        print!("{}", termion::clear::CurrentLine);
    }
}