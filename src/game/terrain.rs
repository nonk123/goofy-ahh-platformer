use std::ops::{Index, IndexMut};

use noise::OpenSimplex;

use crate::util::{Coord, Dim};

use super::{
    chunk::{Chunk, Tile, CHUNK_WIDTH},
    Game,
};

pub const CHUNKS_LOADED_RADIUS: Dim = 16;

pub struct Terrain {
    noise: OpenSimplex,
    loaded_chunks: Vec<Chunk>,
}

impl Terrain {
    pub fn new() -> Self {
        // TODO: randomize the seed value.
        let seed = 0;

        Self {
            noise: OpenSimplex::new(seed),
            loaded_chunks: vec![],
        }
    }

    pub fn loaded_chunks(&self) -> &[Chunk] {
        &self.loaded_chunks
    }
}

impl Terrain {
    fn load_chunks_around(&mut self, center_chunk: Dim) {
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
                let unloaded_chunk = self.loaded_chunks.remove(i);

                // TODO: save chunk.
                let _ = unloaded_chunk;
            }
        }

        for pos in range {
            if exclusions.contains(&pos) {
                continue;
            }

            let mut new_chunk = Chunk::new(pos * CHUNK_WIDTH as Dim);

            // TODO: load chunk if available.
            new_chunk.regenerate(&self.noise);

            self.loaded_chunks.push(new_chunk)
        }
    }
}

impl Index<Coord> for Terrain {
    type Output = Option<Tile>;

    fn index(&self, index: Coord) -> &Self::Output {
        let mut chunk = None;

        for loaded_chunk in &self.loaded_chunks {
            let pos = loaded_chunk.world_position();
            let range = pos..(pos + CHUNK_WIDTH as Dim);

            if range.contains(&index.col) {
                chunk = Some(loaded_chunk);
                break;
            }
        }

        if let Some(chunk) = chunk {
            &chunk[index]
        } else {
            &None
        }
    }
}

impl IndexMut<Coord> for Terrain {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        let mut chunk = None;

        for loaded_chunk in &mut self.loaded_chunks {
            let pos = loaded_chunk.world_position();
            let range = pos..(pos + CHUNK_WIDTH as Dim);

            if range.contains(&index.col) {
                chunk = Some(loaded_chunk);
                break;
            }
        }

        // TODO: write an error message for `expect`.
        chunk.map(|chunk| &mut chunk[index]).unwrap()
    }
}

impl Game {
    pub fn load_chunks_around_camera(&mut self) {
        let center_chunk = self.camera.position.col / CHUNK_WIDTH as Dim;
        self.terrain.load_chunks_around(center_chunk);
    }
}
