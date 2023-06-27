use crate::util::{Coord, DimReal};

use super::{
    chunk::Tile,
    entity::{Entity, MovementMode},
    terrain::Terrain,
    Game,
};

pub const TIMESTEP: DimReal = 1.0 / 60.0;

pub const BREAKING_FORCE: DimReal = 60000.0;

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
            entity.collide(&mut self.terrain);
        }

        self.entities = entities;
    }
}

fn tile_impassable(terrain: &Terrain, pos: Coord) -> bool {
    match &terrain[pos] {
        Some(tile) if tile.is_impassable() => true,
        _ => false,
    }
}

impl Entity {
    fn collide(&mut self, terrain: &mut Terrain) {
        self.process_horizontal_collision(terrain);
        self.process_vertical_collision(terrain);
    }

    fn process_horizontal_collision(&mut self, terrain: &mut Terrain) {
        let eps = 0.01;
        let pos = self.tile_pos();

        if tile_impassable(terrain, pos) && self.velocity.col.abs() > eps {
            self.process_collision(terrain, false);
        }
    }

    fn process_vertical_collision(&mut self, terrain: &mut Terrain) {
        let eps = 0.01;
        let pos = self.tile_pos();

        if !tile_impassable(terrain, pos) {
            return;
        }

        if self.velocity.row < -eps {
            self.on_ground = true;
        }

        if self.velocity.row.abs() > eps {
            self.process_collision(terrain, true);
        }
    }

    fn process_collision(&mut self, terrain: &mut Terrain, vertical: bool) {
        let pos = self.tile_pos();

        let tile_ref = &mut terrain[pos];
        let tile = tile_ref.unwrap();

        let breaking_velocity = self.breaking_velocity_of(&tile);

        let (position_axis, velocity_axis) = {
            if vertical {
                (&mut self.position.row, &mut self.velocity.row)
            } else {
                (&mut self.position.col, &mut self.velocity.col)
            }
        };

        let direction = velocity_axis.signum();

        if velocity_axis.abs() > breaking_velocity {
            *velocity_axis -= direction * breaking_velocity;

            // TODO: add a tile destruction procedure.
            *tile_ref = None;
        } else {
            if direction > 0.0 {
                *position_axis = position_axis.floor();
            } else if direction < 0.0 {
                *position_axis = position_axis.ceil();
            }

            *velocity_axis = 0.0;
        }
    }

    fn breaking_velocity_of(&self, _tile: &Tile) -> DimReal {
        // TODO: add tile durability.
        BREAKING_FORCE * TIMESTEP / self.mass
    }
}
