use std::{
    io::Write,
    ops::{Index, IndexMut},
};

use crossterm::{
    cursor,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal, QueueableCommand,
};

use crate::util::{Coord, Dim};

pub struct Camera {
    pub position: Coord,
}

impl Camera {
    pub fn project(&self, world_point: Coord, screen: &Screen) -> Coord {
        Coord {
            row: screen.rows() / 2 + world_point.row - self.position.row,
            col: screen.cols() / 2 + world_point.col - self.position.col,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    pub character: char,
    pub fg_color: Color,
    /// If `None`, use the default background color instead.
    pub bg_color: Option<Color>,
}

impl Pixel {
    pub const EMPTY: Self = Self {
        character: ' ',
        fg_color: Color::Grey,
        bg_color: None,
    };
}

impl Default for Pixel {
    fn default() -> Self {
        Self::EMPTY
    }
}

pub struct Screen {
    buffer: Vec<Pixel>,
    rows: Dim,
    cols: Dim,
}

impl Screen {
    pub fn new() -> Self {
        Self {
            buffer: vec![],
            rows: 0,
            cols: 0,
        }
    }

    pub fn len(&self) -> Dim {
        self.rows * self.cols
    }

    fn flat_index(&self, index: Coord) -> usize {
        let index = index.row * self.cols() + index.col;
        index as usize
    }

    pub fn resize(&mut self, rows: Dim, cols: Dim) {
        if rows < 0 || cols < 0 {
            panic!("`rows` and `cols` must be zero or positive");
        }

        let old_len = self.len();

        self.rows = rows;
        self.cols = cols;

        let new_len = self.len();

        if old_len < new_len {
            self.clear();
        } else if old_len > new_len {
            self.buffer = self.buffer.iter().take(new_len as usize).cloned().collect();
        }
    }

    pub fn clear(&mut self) {
        self.buffer = vec![Default::default(); self.len() as usize];
    }

    pub fn rows(&self) -> Dim {
        self.rows
    }

    pub fn cols(&self) -> Dim {
        self.cols
    }

    pub fn print_string(&mut self, position: Coord, text: &str) {
        for (col, character) in text.chars().enumerate() {
            let coord = Coord {
                row: position.row,
                col: position.col + col as Dim,
            };

            if coord.col < 0 {
                continue;
            }

            if coord.row < 0 || coord.row >= self.rows() || coord.col >= self.cols() {
                return;
            }

            self[coord] = Pixel {
                character,
                fg_color: Color::Grey,
                bg_color: Some(Color::Black),
            };
        }
    }

    pub fn begin_frame(&mut self) {
        let size = match terminal::size() {
            // Rows and columns are returned swapped.
            Ok(size) => (size.1 as Dim, size.0 as Dim),
            Err(_) => {
                // TODO: handle error.
                self.resize(0, 0);
                return;
            }
        };

        self.resize(size.0, size.1);
        self.clear();
    }

    pub fn end_frame(&mut self) -> crossterm::Result<()> {
        let mut stdout = std::io::stdout().lock();

        stdout.queue(cursor::Hide)?;

        if self.len() == 0 {
            stdout
                .queue(cursor::MoveTo(0, 0))?
                .queue(ResetColor)?
                .queue(Print("Cannot get terminal size"))?;

            return Ok(());
        }

        let mut last_fg_color = Color::Grey;
        let mut last_bg_color = Color::Black;

        stdout
            .queue(SetForegroundColor(last_fg_color))?
            .queue(SetBackgroundColor(last_bg_color))?;

        for row in 0..self.rows() {
            stdout.queue(cursor::MoveTo(0, row as u16))?;

            // Visually flip the screen to make the Y axis go up.
            let row = self.rows() - 1 - row;

            for col in 0..self.cols() {
                let pixel = &self[Coord { row, col }];

                if pixel.fg_color != last_fg_color {
                    stdout.queue(SetForegroundColor(pixel.fg_color))?;
                    last_fg_color = pixel.fg_color;
                }

                // TODO: use a dynamic default background color.
                let bg_color = pixel.bg_color.unwrap_or(Color::DarkCyan);

                if bg_color != last_bg_color {
                    stdout.queue(SetBackgroundColor(bg_color))?;
                    last_bg_color = bg_color;
                }

                stdout.queue(Print(pixel.character))?;
            }
        }

        stdout.flush()?;

        Ok(())
    }
}

impl Index<Coord> for Screen {
    type Output = Pixel;

    fn index(&self, index: Coord) -> &Self::Output {
        let index = self.flat_index(index);
        &self.buffer[index as usize]
    }
}

impl IndexMut<Coord> for Screen {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        let index = self.flat_index(index);
        &mut self.buffer[index as usize]
    }
}
