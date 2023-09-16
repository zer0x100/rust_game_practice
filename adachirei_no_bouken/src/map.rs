use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TileType {
    Floor,
    Wall,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn can_enter_tile(&self, position: Point) -> bool {
        in_bounds(position) && self.tiles[map_idx(position.x, position.y)] == TileType::Floor
    }
}

pub fn map_idx(x: i32, y: i32) -> usize {
    (y * SCREEN_WIDTH + x) as usize
}

pub fn in_bounds(point: Point) -> bool {
    point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
}
