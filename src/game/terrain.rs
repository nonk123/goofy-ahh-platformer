use std::ops::{Index, IndexMut};

use crossterm::style::Color;
use noise::{NoiseFn, OpenSimplex};
use serde::{Deserialize, Serialize};

use crate::util::{Coord, Dim};

use super::Game;

pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 256;

pub const TERRAIN_HEIGHT: usize = 128;
pub const TERRAIN_SCALE: f64 = 6.0;

pub const CHUNKS_LOADED_RADIUS: Dim = 16;

#[derive(Serialize, Deserialize)]
pub struct Chunk {
    world_position: Dim,
    tiles: Vec<Option<Tile>>,
}

impl Chunk {
    pub fn new(world_position: Dim) -> Self {
        Self {
            world_position,
            tiles: vec![None; CHUNK_WIDTH * CHUNK_HEIGHT],
        }
    }

    pub fn regenerate(&mut self, noise: &OpenSimplex) {
        for col in 0..CHUNK_WIDTH {
            let world_position = self.world_position() + col as Dim;
            let chunk_position = world_position as f64 / CHUNK_WIDTH as f64;
            let noise_position = chunk_position / TERRAIN_SCALE;

            let noise_value = noise.get([noise_position, 0.0]);
            let height_scale = noise_value * 0.5 + 0.5;

            let height = height_scale * TERRAIN_HEIGHT as f64;
            let height = height as usize;

            let mut layers = [
                (1, Tile::Grass),
                (5, Tile::GrassyDirt),
                (12, Tile::Dirt),
                (CHUNK_HEIGHT, Tile::Stone),
            ];

            let mut layer_idx = 0;

            for row in (0..CHUNK_HEIGHT).rev() {
                let coord = Coord {
                    row: row as Dim,
                    col: world_position,
                };

                self[coord] = {
                    if row <= height {
                        let layer = &mut layers[layer_idx];

                        layer.0 -= 1;

                        if layer.0 == 0 {
                            layer_idx += 1;
                        }

                        Some(layer.1)
                    } else {
                        None
                    }
                };
            }
        }
    }

    fn flat_index(&self, index: Coord) -> Option<usize> {
        if index.row < 0 || index.row >= CHUNK_HEIGHT as Dim {
            return None;
        }

        let col = index.col - self.world_position();

        if col < 0 || col >= CHUNK_WIDTH as Dim {
            return None;
        }

        let index = col * CHUNK_HEIGHT as Dim + index.row;
        Some(index as usize)
    }

    pub fn world_position(&self) -> Dim {
        self.world_position
    }

    pub fn chunk_position(&self) -> Dim {
        self.world_position / CHUNK_WIDTH as Dim
    }
}

/// Uses world coordinates for indexing.
impl Index<Coord> for Chunk {
    type Output = Option<Tile>;

    fn index(&self, index: Coord) -> &Self::Output {
        if let Some(index) = self.flat_index(index) {
            &self.tiles[index]
        } else {
            &None
        }
    }
}

/// Uses world coordinates for indexing.
impl IndexMut<Coord> for Chunk {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        let index = self
            .flat_index(index)
            .expect("Index out of bounds; check it with `contains` before `index_mut`");

        &mut self.tiles[index]
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Tile {
    Dirt,
    Stone,
    GrassyDirt,
    Grass,
    Flower(Color),
}

impl Tile {
    pub fn is_impassable(&self) -> bool {
        use Tile::*;

        match self {
            Grass | Flower(_) => false,
            _ => true,
        }
    }
}

impl Game {
    pub fn load_chunks_around_camera(&mut self) {
        let chunk_pos = |world_col| world_col / CHUNK_WIDTH as Dim;
        let world_pos = |chunk_pos| chunk_pos * CHUNK_WIDTH as Dim;

        let center_chunk = chunk_pos(self.camera.position.col);

        let start = center_chunk - CHUNKS_LOADED_RADIUS;
        let end = center_chunk + CHUNKS_LOADED_RADIUS;

        let range = start..=end;

        let mut exclusions = vec![];
        let mut i = 0;

        while i < self.loaded_chunks.len() {
            let pos = self.loaded_chunks[i].chunk_position();

            if range.contains(&pos) {
                exclusions.push(pos);
                i += 1;
            } else {
                self.loaded_chunks.remove(i);
            }
        }

        for pos in range {
            if exclusions.contains(&pos) {
                continue;
            }

            let mut chunk = Chunk::new(world_pos(pos));
            chunk.regenerate(&self.terrain_noise);

            self.loaded_chunks.push(chunk)
        }
    }
}
