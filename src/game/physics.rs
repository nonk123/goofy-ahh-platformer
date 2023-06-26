use crate::util::{Coord, Dim, DimReal};

use super::{
    entity::{Entity, MovementMode},
    Game,
};

pub const TIMESTEP: DimReal = 1.0 / 60.0;

#[derive(Clone)]
pub struct AABB {
    pub width: Dim,
    pub height: Dim,
}

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
            self.solve_collision(entity);
        }

        self.entities = entities;
    }

    fn solve_collision(&self, entity: &mut Entity) {
        entity.on_ground = false;

        let left = entity.left_edge();
        let right = entity.right_edge();
        let bottom = entity.bottom_edge();
        let top = entity.top_edge();

        let tile_exists = |point| {
            let mut result = false;

            for chunk in &self.loaded_chunks {
                match chunk[point] {
                    Some(tile) => {
                        result = tile.is_impassable();
                        break;
                    }
                    None => (),
                };
            }

            result
        };

        for col in left..=right {
            let point = Coord {
                row: bottom - 1,
                col,
            };

            if tile_exists(point) && entity.collide_below(point) {
                break;
            }

            let point = Coord { row: top + 1, col };

            if tile_exists(point) && entity.collide_above(point) {
                break;
            }
        }

        for row in bottom..=top {
            let point = Coord { row, col: left - 1 };

            if tile_exists(point) && entity.collide_left(point) {
                break;
            }

            let point = Coord {
                row,
                col: right + 1,
            };

            if tile_exists(point) && entity.collide_right(point) {
                break;
            }
        }
    }
}

impl Entity {
    fn collide_below(&mut self, point: Coord) -> bool {
        if self.velocity.row < 0.0 {
            self.set_bottom_edge(point.row + 1);
            self.on_ground = true;
            self.velocity.row = 0.0;

            true
        } else {
            false
        }
    }

    fn collide_above(&mut self, point: Coord) -> bool {
        if self.velocity.row > 0.0 {
            self.set_top_edge(point.row - 1);
            self.velocity.row = 0.0;

            true
        } else {
            false
        }
    }

    fn collide_left(&mut self, point: Coord) -> bool {
        if self.velocity.col < 0.0 {
            self.set_left_edge(point.col + 1);
            self.velocity.col = 0.0;

            true
        } else {
            false
        }
    }

    fn collide_right(&mut self, point: Coord) -> bool {
        if self.velocity.col > 0.0 {
            self.set_right_edge(point.col - 1);
            self.velocity.col = 0.0;

            true
        } else {
            false
        }
    }
}
