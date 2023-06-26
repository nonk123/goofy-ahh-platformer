use std::ops::{Index, IndexMut};

use crate::util::{Coord, Dim};

use super::renderer::{Camera, Pixel, Screen};

#[derive(Debug)]
pub enum TextArtError {
    BufferLen {
        expected_len: Dim,
        actual_len: usize,
    },
}

#[derive(Clone)]
pub struct TextArt {
    buffer: Vec<Option<Pixel>>,
    rows: Dim,
    cols: Dim,
}

pub type OverwriteFn = fn(&mut Pixel, Pixel);

pub fn overwrite(output: &mut Pixel, replacement: Pixel) {
    *output = replacement;
}

impl TextArt {
    pub fn try_new(buffer: Vec<Option<Pixel>>, rows: Dim, cols: Dim) -> Result<Self, TextArtError> {
        let instance = Self { buffer, rows, cols };

        let expected_len = instance.len();
        let actual_len = instance.buffer.len();

        if actual_len == expected_len as usize {
            Ok(instance)
        } else {
            Err(TextArtError::BufferLen {
                expected_len,
                actual_len,
            })
        }
    }

    pub fn len(&self) -> Dim {
        self.rows * self.cols
    }

    fn flat_index(&self, index: Coord) -> usize {
        let index = index.row * self.cols() + index.col;
        assert!(index >= 0);
        index as usize
    }

    pub fn rows(&self) -> Dim {
        self.rows
    }

    pub fn cols(&self) -> Dim {
        self.cols
    }

    pub fn blit(&self, offset: Coord, camera: &Camera, screen: &mut Screen) {
        self.blit_custom(offset, camera, screen, overwrite);
    }

    pub fn blit_custom(
        &self,
        offset: Coord,
        camera: &Camera,
        screen: &mut Screen,
        overwrite_fn: OverwriteFn,
    ) {
        for row in 0..self.rows() {
            for col in 0..self.cols() {
                let pixel_coord = Coord { row, col };
                let screen_coord = camera.project(offset + pixel_coord, screen);

                // Check AABB visibility on the first iteration.
                if row == 0 && col == 0 {
                    let bottom_left = screen_coord;

                    let top_right = Coord {
                        row: bottom_left.row + self.rows(),
                        col: bottom_left.col + self.cols(),
                    };

                    if bottom_left.row >= screen.rows()
                        || top_right.row < 0
                        || bottom_left.col >= screen.cols()
                        || top_right.col < 0
                    {
                        return;
                    }
                }

                if screen_coord.row < 0 || screen_coord.row >= screen.rows() {
                    continue;
                }

                if screen_coord.col < 0 || screen_coord.col >= screen.cols() {
                    continue;
                }

                let pixel = self[pixel_coord];

                if let Some(pixel) = pixel {
                    overwrite_fn(&mut screen[screen_coord], pixel);
                }
            }
        }
    }
}

impl Index<Coord> for TextArt {
    type Output = Option<Pixel>;

    fn index(&self, index: Coord) -> &Self::Output {
        let index = self.flat_index(index);
        &self.buffer[index as usize]
    }
}

impl IndexMut<Coord> for TextArt {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        let index = self.flat_index(index);
        &mut self.buffer[index as usize]
    }
}
