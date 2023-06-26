use std::collections::HashSet;

use crate::util::{Coord, CoordReal, Dim, DimReal};

use super::{
    anim::Animation,
    physics::AABB,
    renderer::{Camera, Pixel, Screen},
};

fn entity_pixel_overwrite(output: &mut Pixel, replacement: Pixel) {
    let empty = Pixel::EMPTY;

    if output.character == empty.character {
        *output = replacement;
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
    pub movement_hitbox: Option<AABB>,
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
            movement_hitbox: None,
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

    pub fn width(&self) -> Dim {
        match &self.movement_hitbox {
            Some(hitbox) => hitbox.width,
            None => 0,
        }
    }

    pub fn half_width(&self) -> Dim {
        self.width() / 2
    }

    pub fn height(&self) -> Dim {
        match &self.movement_hitbox {
            Some(hitbox) => hitbox.height,
            None => 0,
        }
    }

    pub fn left_edge(&self) -> Dim {
        self.tile_pos().col - self.half_width()
    }

    pub fn set_left_edge(&mut self, col: Dim) {
        self.position.col = (col + self.half_width()) as DimReal;
    }

    pub fn right_edge(&self) -> Dim {
        self.tile_pos().col + self.half_width()
    }

    pub fn set_right_edge(&mut self, col: Dim) {
        self.position.col = (col - self.half_width()) as DimReal;
    }

    pub fn top_edge(&self) -> Dim {
        self.bottom_edge() + self.height() - 1
    }

    pub fn set_top_edge(&mut self, row: Dim) {
        self.position.row = (row - self.height() - 1) as DimReal;
    }

    pub fn bottom_edge(&self) -> Dim {
        self.tile_pos().row
    }

    pub fn set_bottom_edge(&mut self, row: Dim) {
        self.position.row = row as DimReal;
    }

    pub fn center(&self) -> CoordReal {
        CoordReal {
            row: self.position.row + self.height() as DimReal / 2.0,
            col: self.position.col + self.width() as DimReal / 2.0,
        }
    }

    pub fn display(&mut self, camera: &Camera, screen: &mut Screen) {
        let tile_pos = self.tile_pos();

        let frame = self.animations[self.current_animation].next_frame();

        let offset = Coord {
            row: 0,
            col: frame.cols() / 2,
        };

        frame.blit_custom(tile_pos + offset, camera, screen, entity_pixel_overwrite);
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
