use crossterm::style::Color;

use crate::{
    game::{anim::Animation, renderer::Pixel},
    util::DimReal,
};

use super::{
    chunk::TERRAIN_HEIGHT,
    entity::{Entity, EntityFlag, MovementMode},
    Game,
};

pub fn new_player() -> Entity {
    let animations = {
        let basic = Animation::new(vec![Pixel {
            character: '@',
            fg_color: Color::Grey,
            bg_color: None,
        }]);

        vec![basic]
    };

    let mut player = Entity::new(animations);

    player.position.row = (TERRAIN_HEIGHT + 1) as DimReal;

    player.mass = 50.0;

    player.movement_mode = MovementMode::Walking {
        walking_speed: 8.0,
        jump_impulse: 10.0,
    };

    player.flags.insert(EntityFlag::Player);

    player
}

impl Game {
    pub fn find_player(&self) -> Option<usize> {
        for (idx, entity) in self.entities.iter().enumerate() {
            if entity.flags.contains(&EntityFlag::Player) {
                return Some(idx);
            }
        }

        None
    }

    pub fn get_player(&self) -> Option<&Entity> {
        let idx = self.find_player()?;
        Some(&self.entities[idx])
    }

    pub fn get_player_mut(&mut self) -> Option<&mut Entity> {
        let idx = self.find_player()?;
        Some(&mut self.entities[idx])
    }

    pub fn snap_camera_to_player(&mut self) {
        if let Some(player) = self.get_player() {
            self.camera.position = player.tile_pos();
        }
    }
}
