use crossterm::style::Color;

use crate::util::{Coord, Dim};

use super::{
    chunk::{Tile, CHUNK_HEIGHT, CHUNK_WIDTH},
    renderer::{Pixel, Screen},
    Game,
};

impl Game {
    pub fn display_entities(&mut self, screen: &mut Screen) {
        for entity in &mut self.entities {
            entity.display(&self.camera, screen);
        }
    }

    pub fn display_terrain(&mut self, screen: &mut Screen) {
        for chunk in self.terrain.loaded_chunks() {
            for row in 0..CHUNK_HEIGHT {
                for col in 0..CHUNK_WIDTH {
                    let chunk_coord = Coord {
                        row: row as Dim,
                        col: chunk.world_position() + col as Dim,
                    };

                    let pixel = chunk[chunk_coord].map(|x| x.display());

                    if let Some(pixel) = pixel {
                        pixel.blit(chunk_coord, &self.camera, screen);
                    }
                }
            }
        }
    }
}

impl Tile {
    pub fn display(&self) -> Pixel {
        let solid = |color| Pixel {
            character: ' ',
            fg_color: Color::White,
            bg_color: Some(color),
        };

        let object = |character, color| Pixel {
            character,
            fg_color: color,
            bg_color: None,
        };

        use Tile::*;

        match self {
            Dirt => solid(Color::Yellow),
            Stone => solid(Color::DarkGrey),
            GrassyDirt => solid(Color::DarkGreen),
            Grass => object('\"', Color::DarkGreen),
            Flower(color) => object('*', *color),
        }
    }
}
