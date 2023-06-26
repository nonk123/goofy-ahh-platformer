use crossterm::style::Color;

use crate::util::{Coord, Dim};

use super::{
    renderer::{Pixel, Screen},
    terrain::{Tile, CHUNK_HEIGHT, CHUNK_WIDTH},
    Game,
};

impl Game {
    pub fn display_entities(&mut self, screen: &mut Screen) {
        for entity in &mut self.entities {
            entity.display(&self.camera, screen);
        }
    }

    pub fn display_terrain(&mut self, screen: &mut Screen) {
        for chunk in &self.loaded_chunks {
            for row in 0..CHUNK_HEIGHT {
                for col in 0..CHUNK_WIDTH {
                    let chunk_coord = Coord {
                        row: row as Dim,
                        col: chunk.world_position() + col as Dim,
                    };

                    let screen_coord = self.camera.project(chunk_coord, screen);

                    if screen_coord.row < 0 || screen_coord.row >= screen.rows() {
                        continue;
                    }

                    if screen_coord.col < 0 || screen_coord.col >= screen.cols() {
                        continue;
                    }

                    let pixel = chunk[chunk_coord].map(|x| x.display());

                    if let Some(pixel) = pixel {
                        screen[screen_coord] = pixel;
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
