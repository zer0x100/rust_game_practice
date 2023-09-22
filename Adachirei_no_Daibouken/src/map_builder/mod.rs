mod automata;
mod drunkard;
mod empty;
mod prefab;
mod rooms;
mod themes;

use crate::prelude::*;
const NUM_MONSTERS: usize = 70;

use self::{
    prefab::apply_prefab,
    themes::{ResearchInstitueTheme, ForestTheme, StockRoomTheme},
};
use automata::CellularAutomataArchitect;
use drunkard::DrunkardArchitect;
use rooms::RoomsArchitect;

pub trait MapArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder;
}

pub trait MapTheme: Sync + Send {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType;
}

pub struct MapBuilder {
    pub map: Map,
    pub monster_spawns: Vec<Point>,
    pub player_start: Point,
    pub amulet_start: Point,
    pub theme: Box<dyn MapTheme>,
}

impl MapBuilder {
    pub fn with_architect_and_theme(rng: &mut RandomNumberGenerator, mut architect: Box<dyn MapArchitect>, theme: Box<dyn MapTheme>) -> Self {
        let mut mb = architect.new(rng);
        apply_prefab(&mut mb, rng);
        mb.theme = theme;

        mb
    }

    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let map_architect: Box<dyn MapArchitect> = match rng.range(0, 3) {
            0 => Box::new(RoomsArchitect {}),
            1 => Box::new(DrunkardArchitect {}),
            _ => Box::new(CellularAutomataArchitect {}),
        };
        let theme = match rng.range(0, 3) {
            0 => ResearchInstitueTheme::new(),
            1 => ForestTheme::new(),
            _ => StockRoomTheme::new(),
        };

        MapBuilder::with_architect_and_theme(rng, map_architect, theme)
    }

    pub fn new_level(rng: &mut RandomNumberGenerator, level: usize) -> Self {
        match level {
            0 => MapBuilder::with_architect_and_theme(rng, Box::new(DrunkardArchitect{}), StockRoomTheme::new()),
            1 => MapBuilder::with_architect_and_theme(rng, Box::new(RoomsArchitect{}), ResearchInstitueTheme::new()),
            _ => MapBuilder::with_architect_and_theme(rng, Box::new(CellularAutomataArchitect{}), ForestTheme::new()),
        }
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn find_most_distant(&self) -> Point {
        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &vec![self.map.point2d_to_index(self.player_start)],
            &self.map,
            1024.0,
        );

        const UNREACHABLE: &f32 = &f32::MAX;
        self.map.index_to_point2d(
            dijkstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, v)| *v < UNREACHABLE)
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .unwrap()
                .0,
        )
    }

    fn spawn_monsters(&self, player_start: &Point, rng: &mut RandomNumberGenerator) -> Vec<Point> {

        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &vec![map_idx(player_start.x, player_start.y)],
            &self.map,
            1024.0
        );

        const UNREACHABLE: &f32 = &f32::MAX;
        let spawnable_tiles: Vec<Point> = dijkstra_map.map
            .iter()
            .enumerate()
            .filter(|(idx, v)| {
                *v < UNREACHABLE && DistanceAlg::Pythagoras
                    .distance2d(*player_start, self.map.index_to_point2d(*idx)) > 10.0
            })
            .map(|(idx, _)| self.map.index_to_point2d(idx))
            .collect();

        let mut spawns = Vec::new();
        use std::collections::HashSet;
        let mut already_exist_idx = HashSet::new();
        while spawns.len() < NUM_MONSTERS {
            let target_idx = rng.random_slice_index(&spawnable_tiles).unwrap();
            if !already_exist_idx.contains(&target_idx) {
                spawns.push(spawnable_tiles[target_idx]);
                already_exist_idx.insert(target_idx);
            }
        }
        spawns
    }
}
