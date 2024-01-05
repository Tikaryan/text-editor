use std::io::{self, stdout, Error};
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
pub struct Editor {}

impl Editor{
    pub fn run (&self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        loop {
            if let Err(error) = self.process_keypress() {
                die(&error);
            }
        }
    }
    pub fn new() -> Self {
       Self {}
    }
    fn process_keypress(&self) -> Result<(), Error>{
        let pressed_key = read_key()?;
        match pressed_key {
            Key::Ctrl('q') => panic!("Quit"),
            _ => (),
        }

        Ok(())
    }
}

fn read_key() -> Result<Key, Error> {
    loop {
        if let Some(key) = io::stdin().keys().next() {
            return key;
        }
    }
}


fn die(e: &Error) {
    panic!("{}", e);
}