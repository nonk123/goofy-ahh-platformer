use std::collections::HashSet;

use crate::util::{Coord, CoordReal, Dim, DimReal};

use super::{
    anim::Animation,
    renderer::{Camera, Pixel, Screen},
};

fn entity_pixel_overwrite(output: &mut Pixel, replacement: &Pixel) {
    let empty = Pixel::EMPTY;

    if output.character == empty.character {
        *output = *replacement;
    } else if output.bg_color.is_none() {
        output.bg_color = Some(replacement.fg_color);
    }
}

#[derive(Clone)]
pub struct Entity {
    pub position: CoordReal,
    pub velocity: CoordReal,
    pub gravity_scale: DimReal,
    animations: Vec<Animation>,
    pub current_animation: usize,
    // TODO: use bitflags instead.
    pub flags: HashSet<EntityFlag>,
    pub movement_mode: MovementMode,
    pub movement_controls: MovementControls,
    pub deletion_flag: bool,
    // TODO: put into `EntityFlag`, perhaps?
    pub on_ground: bool,
}

impl Entity {
    pub fn new(animations: Vec<Animation>) -> Self {
        Self {
            position: CoordReal::ZERO,
            velocity: CoordReal::ZERO,
            gravity_scale: 1.0,
            animations,
            current_animation: 0,
            flags: HashSet::new(),
            movement_mode: MovementMode::Static,
            movement_controls: MovementControls::EMPTY,
            deletion_flag: false,
            on_ground: false,
        }
    }

    pub fn tile_pos(&self) -> Coord {
        Coord {
            row: self.position.row.round() as Dim,
            col: self.position.col.round() as Dim,
        }
    }

    pub fn display(&mut self, camera: &Camera, screen: &mut Screen) {
        let tile_pos = self.tile_pos();
        let frame = self.animations[self.current_animation].next_frame();
        frame.blit_custom(tile_pos, camera, screen, entity_pixel_overwrite);
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityFlag {
    Player,
}

#[derive(Clone)]
pub enum MovementMode {
    /// Completely static and not affected by gravity. No control.
    Static,
    /// Affected by gravity and physics interactions. Cannot be controlled.
    Dynamic,
    /// Walking and/or jumping. Customizable and controllable.
    Walking {
        walking_speed: DimReal,
        jump_impulse: DimReal,
    },
}

#[derive(Debug, Clone, Copy)]
pub struct MovementControls {
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub jump: bool,
}

impl MovementControls {
    pub const EMPTY: Self = Self {
        left: false,
        right: false,
        up: false,
        down: false,
        jump: false,
    };

    pub fn horizontal_direction(&self) -> DimReal {
        if self.left && self.right {
            0.0
        } else if self.left {
            -1.0
        } else if self.right {
            1.0
        } else {
            0.0
        }
    }
}
