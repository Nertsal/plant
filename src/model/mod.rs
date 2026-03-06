mod logic;

use crate::prelude::*;

pub type ICoord = i32;

pub struct Model {
    pub grid_visual: GridVisual,
    pub camera: Camera2d,
}

pub struct GridVisual {
    /// Position of the (0, 0) point in the world.
    pub center: vec2<f32>,
    pub tile_size: vec2<f32>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            camera: Camera2d {
                center: vec2::ZERO,
                rotation: Angle::ZERO,
                fov: Camera2dFov::Vertical(10.0),
            },
            grid_visual: GridVisual {
                center: vec2::ZERO,
                tile_size: vec2(1.0, 1.0),
            },
        }
    }
}
