use crate::Position;
use std::{io::{ stdout, Error, Stdout, Write}};
use crossterm::{event::*, style::Color};
use crossterm::cursor;
use crossterm::terminal::*;
use crossterm::event::KeyEvent;
// use termion::input::TermRead;
// use termion::raw::{IntoRawMode, RawTerminal};
// use termion::terminal_size;
// use termion::{clear, color, cursor};

pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size,
    _stdout: Box<dyn term::Terminal<Output=Stdout>>,
}

impl Terminal {
    /// # Errors
    ///
    /// Will return `Error` if it fails to get terminal size  
    /// or if it fails to switch to raw mode
    pub fn new() -> Result<Self, Error> {
        let size : (u16,u16)= crossterm::terminal::size().ok().unwrap();

        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1.saturating_sub(2),
            },
            _stdout: term::stdout().unwrap(),
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn clear_screen() {
        print!("{}", Clear(ClearType::All)); // Uses the temios features of vt900
    }

    #[allow(clippy::cast_possible_truncation)]
    pub fn cursor_position(position: &Position) {
        let Position { mut x, mut y } = position;

        x = x.saturating_add(1);
        y = y.saturating_add(1);

        let x = x as u16;
        let y = y as u16;

        print!("{}", cursor::MoveTo(x, y));
    }

    /// # Errors
    ///
    /// Will return an error if not
    /// all bytes could be written due to I/O errors
    /// or EOF being reached.
    pub fn flush() -> Result<(), Error> {
        stdout().flush()
    }

    /// # Errors
    ///
    /// Will return an error if it fails to read key
    pub fn read_key() -> Result<KeyEvent, Error> {
        loop {
            let key = crossterm::event::read()?;
            match key  {
                Event::Key(opt)=> {
                    return Ok(opt);
                },
                _=>{}
            }
        }
    }

    pub fn cursor_hide() {
        print!("{}", cursor::Hide);
    }

    pub fn cursor_show() {
        print!("{}", cursor::Show);
    }

    pub fn clear_current_line() {
        print!("{}", Clear(ClearType::CurrentLine));
    }

    pub fn set_bg_color(color: Color) {
        print!("{}", crossterm::style::SetBackgroundColor(color));
    }

    pub fn reset_bg_color() {
        print!("{}", crossterm::style::SetBackgroundColor(Color::Reset));
    }

    pub fn set_fg_color(color: Color) {
        print!("{}", crossterm::style::SetForegroundColor(color));
    }

    pub fn reset_fg_color() {
        print!("{}", crossterm::style::SetForegroundColor(Color::Reset));
    }
}
