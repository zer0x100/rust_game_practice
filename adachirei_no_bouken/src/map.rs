use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

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

    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        ctx.set_active_console(0);
        for y in camera.top_y..=camera.bottom_y {
            for x in camera.left_x..=camera.right_x {
                if in_bounds(Point::new(x, y)) {
                    match self.tiles[map_idx(x, y)] {
                        TileType::Floor => {
                            ctx.set(x - camera.left_x, y - camera.top_y, WHITE, BLACK, to_cp437('.'));
                        }
                        TileType::Wall => {
                            ctx.set(x - camera.left_x, y - camera.top_y, WHITE, BLACK, to_cp437('#'));
                        }
                    }
                }
            }
        }
    }
}

pub fn map_idx(x: i32, y: i32) -> usize {
    (y * SCREEN_WIDTH + x) as usize
}

pub fn in_bounds(point: Point) -> bool {
    point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
}
