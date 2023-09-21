use crate::prelude::*;

pub struct ResearchInstitueTheme {}

impl MapTheme for ResearchInstitueTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437('.'),
            TileType::Wall => to_cp437('#'),
            TileType::Exit => to_cp437('>'),
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
            TileType::Floor => to_cp437(';'),
            TileType::Wall => to_cp437('"'),
            TileType::Exit => to_cp437('>'),
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
            TileType::Floor => 176,
            TileType::Wall => 178,
            TileType::Exit => to_cp437('>'),
        }
    }
}

impl StockRoomTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self{})
    }
}