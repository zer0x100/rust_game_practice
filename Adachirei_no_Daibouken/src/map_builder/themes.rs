use crate::prelude::*;

pub struct ResearchInstitueTheme {}

impl MapTheme for ResearchInstitueTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => 9,
            TileType::Wall => 10,
            TileType::Exit => 15,
        }
    }
}

impl ResearchInstitueTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

pub struct ForestTheme {}

impl MapTheme for ForestTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => 2,
            TileType::Wall => 1,
            TileType::Exit => 16,
        }
    }
}

impl ForestTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

pub struct StockRoomTheme {}

impl MapTheme for StockRoomTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => 11,
            TileType::Wall => 12,
            TileType::Exit => 4,
        }
    }
}

impl StockRoomTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self{})
    }
}