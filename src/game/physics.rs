use crate::util::{Coord, DimReal};

use super::{
    entity::{Entity, MovementMode},
    terrain::Terrain,
    Game,
};

pub const TIMESTEP: DimReal = 1.0 / 60.0;

impl Game {
    pub fn update_physics_bodies(&mut self) {
        let gravity_accel = self.gravity * TIMESTEP;

        for entity in &mut self.entities {
            match entity.movement_mode {
                MovementMode::Static => continue,
                MovementMode::Dynamic => (),
                MovementMode::Walking {
                    walking_speed,
                    jump_impulse,
                } => {
                    let walk_direction = entity.movement_controls.horizontal_direction();

                    entity.velocity.col = {
                        if walk_direction == 0.0 {
                            0.0
                        } else {
                            let vel = entity.velocity.col + walking_speed * walk_direction;
                            vel.clamp(-walking_speed, walking_speed)
                        }
                    };

                    if entity.movement_controls.jump && entity.on_ground {
                        entity.velocity.row += jump_impulse;
                    }
                }
            }

            entity.velocity += gravity_accel * entity.gravity_scale;
            entity.position += entity.velocity * TIMESTEP;
        }
    }

    pub fn solve_collisions(&mut self) {
        let mut entities = self.entities.clone();

        for entity in &mut entities {
            entity.on_ground = false;
            entity.collide(&self.terrain);
        }

        self.entities = entities;
    }
}

impl Entity {
    fn collide(&mut self, terrain: &Terrain) {
        let tile_exists = |entity: &Entity, d_row, d_col| -> bool {
            let point = entity.tile_pos()
                + Coord {
                    row: d_row,
                    col: d_col,
                };

            let tile = terrain[point];
            tile.map(|tile| tile.is_impassable()).unwrap_or(false)
        };

        let reset_row = |entity: &mut Entity| {
            entity.velocity.row = 0.0;
            entity.position.row = entity.tile_pos().to_real().row;
        };

        let reset_col = |entity: &mut Entity| {
            entity.velocity.col = 0.0;
            entity.position.col = entity.tile_pos().to_real().col;
        };

        if self.velocity.row < 0.0 && tile_exists(self, -1, 0) {
            self.on_ground = true;
            reset_row(self);
        } else if self.velocity.row > 0.0 && tile_exists(self, 1, 0) {
            reset_row(self);
        }

        if self.velocity.col < 0.0 && tile_exists(self, 0, -1) {
            reset_col(self);
        } else if self.velocity.col > 0.0 && tile_exists(self, 0, 1) {
            reset_col(self);
        }
    }
}
