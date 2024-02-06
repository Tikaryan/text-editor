use crate::Terminal;
use std::io::{stdout, Error};
use termion::event::Key;
use termion::raw::IntoRawMode;

const VERSION: &str = env!("CARGO_PKG_VERSION");
pub struct Editor {
    should_quit: bool,
    terminal: Terminal,
    cursor_pos: Position,
}

// represents the position of the cursor in the terminal
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Editor{
    pub fn run (&mut self) {
        let _stdout = stdout().into_raw_mode().unwrap();

        loop {
            if let Err(error) = self.refresh_screen() {
                die(error);
            }

            if self.should_quit {
                break;
            }

            if let Err(error) = self.process_keypress() {
                die(error);
            }

        }
    }
    pub fn default() -> Self {
       Self {
           should_quit: false,
           terminal: Terminal::default().expect("Failed to initialize terminal"),
           cursor_pos: Position { x: 0, y: 0},
       }
    }

    fn refresh_screen(&self) -> Result<(), Error>{
        Terminal::cursor_hide();
        Terminal::clear_screen();
        Terminal::cursor_position(&Position { x: 0, y: 0});

        // print goodbye message if the user wants to quit
        if self.should_quit {
            Terminal::clear_current_line();
            println!("Goodbye.\r");
        } else {
            self.draw_rows();
           Terminal::cursor_position(&Position { x: 0, y: 0});
        }

        Terminal::cursor_show();
        Terminal::flush()
    }

    fn process_keypress(&mut self) -> Result<(), Error>{
        let pressed_key = Terminal::read_key()?;
        match pressed_key {
            Key::Ctrl('q') => self.should_quit = true,
            Key::PageUp | Key::PageDown | Key::Home | Key::End => self.move_cursor(pressed_key),
            _ => (),
        }

        Ok(())
    }

    pub fn draw_welcome_message(&self) {
        let mut welcome_message = format!("akaladarshi editor -- version {}", VERSION);
        let width = self.terminal.size().width as usize;
        let len = welcome_message.len();
        let padding = width.saturating_sub(len) / 2;
        let spaces = " ".repeat(padding.saturating_sub(1));
        welcome_message = format!("~{}{}", spaces, welcome_message);
        println!("{}\r", welcome_message)
    }
    fn draw_rows(&self) {
        let height = self.terminal.size().height;
        for row in 0..self.terminal.size().height -1 {
            Terminal::clear_current_line();
            if row == height / 3 {
                self.draw_welcome_message();
            } else {
                println!("~\r");
            }
        }
    }

    fn move_cursor(&mut self, key: Key) {
        let Position{mut x, mut y }  = self.cursor_pos;
        let height = self.terminal.size().height.saturating_sub(1) as usize;
        let width = self.terminal.size().width.saturating_sub(1) as usize;
        match key {
            Key::Up => y =  y.saturating_sub(1),
            Key::Down => {
                if y < height {
                    y = y.saturating_add(1);
                }
            },
            Key::Left => x = x.saturating_sub(1),
            Key::Right => {
                if x < width {
                    x = x.saturating_add(1);
                }
            },
            _ => (),
        }
        self.cursor_pos = Position { x, y };
    }
}




fn die(e: Error) {
    panic!("{}", e);
}