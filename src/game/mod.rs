use std::collections::HashSet;

use crate::util::{Coord, CoordReal};

use self::{
    entity::{Entity, EntityFlag},
    input::Key,
    renderer::{Camera, Screen},
    terrain::Terrain,
};

pub mod anim;
pub mod chunk;
pub mod display;
pub mod entity;
pub mod input;
pub mod physics;
pub mod player;
pub mod renderer;
pub mod terrain;
pub mod update;

pub struct Game {
    exit_requested: bool,
    camera: Camera,
    entities: Vec<Entity>,
    terrain: Terrain,
    gravity: CoordReal,
    held_keys: HashSet<Key>,
}

impl Game {
    pub fn new() -> Self {
        Self {
            exit_requested: false,
            camera: Camera {
                position: Coord::ZERO,
            },
            entities: vec![],
            terrain: Terrain::new(),
            gravity: CoordReal {
                row: -10.0,
                col: 0.0,
            },
            held_keys: HashSet::new(),
        }
    }

    pub fn exit_requested(&self) -> bool {
        self.exit_requested
    }

    pub fn spawn(&mut self, entity: Entity) {
        let is_player = entity.flags.contains(&EntityFlag::Player);

        self.entities.push(entity);

        if is_player {
            self.snap_camera_to_player();
            self.load_chunks_around_camera();
        }
    }

    pub fn tick(&mut self, screen: &mut Screen) {
        self.process_player_input();
        self.update_physics_bodies();
        self.solve_collisions();
        self.update_entities_state();
        self.snap_camera_to_player();
        self.load_chunks_around_camera();
        self.delete_marked_entities();
        self.display_terrain(screen);
        self.display_entities(screen);
    }

    pub fn delete_marked_entities(&mut self) {
        let mut i = 0;

        while i < self.entities.len() {
            if self.entities[i].deletion_flag {
                self.entities.remove(i);
            } else {
                i += 1;
            }
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
