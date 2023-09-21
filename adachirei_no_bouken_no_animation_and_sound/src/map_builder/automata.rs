use super::MapArchitect;
use crate::prelude::*;

const NUM_ITERATION: usize = 10;

pub struct CellularAutomataArchitect {}

impl MapArchitect for CellularAutomataArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
            theme: super::themes::ResearchInstitueTheme::new(),
        };

        //set initial map
        self.random_noise_map(rng, &mut mb.map);
        //cellular automata iteration
        for _ in 0..NUM_ITERATION {
            self.iteration(&mut mb.map);
        }
        mb.player_start = self.find_start(&mb.map);
        mb.amulet_start = mb.find_most_distant();
        mb.monster_spawns = mb.spawn_monsters(&mb.player_start, rng);

        mb
    }
}

impl CellularAutomataArchitect {
    fn random_noise_map(&self, rng: &mut RandomNumberGenerator, map: &mut Map) {
        map.tiles.iter_mut().for_each(|t| {
            let roll = rng.range(0, 100);
            *t = if roll > 55 {
                TileType::Floor
            } else {
                TileType::Wall
            }
        });
    }

    fn count_neighbor(&self, x: i32, y: i32, map: &Map) -> usize {
        let mut count = 0;
        for y_delta in -1..=1 {
            for x_delta in -1..=1 {
                if (x_delta == 0 || y_delta == 0)
                    && map.tiles[map_idx(x + x_delta, y + y_delta)] == TileType::Wall
                {
                    count += 1;
                }
            }
        }
        count
    }

    fn iteration(&self, map: &mut Map) {
        let mut new_tiles = map.tiles.clone();
        for y in 1..SCREEN_HEIGHT - 1 {
            for x in 1..SCREEN_WIDTH - 1 {
                let neighbors = self.count_neighbor(x, y, map);
                let idx = map_idx(x, y);
                new_tiles[idx] = if neighbors > 4 || neighbors == 0 {
                    TileType::Wall
                } else {
                    TileType::Floor
                };
            }
        }
        map.tiles = new_tiles;
    }

    fn find_start(&self, map: &Map) -> Point {
        let center = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        let closest_point = map
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| **t == TileType::Floor)
            .map(|(i, _)| {
                (
                    i,
                    DistanceAlg::Pythagoras.distance2d(map.index_to_point2d(i), center),
                )
            })
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
            .0;

        map.index_to_point2d(closest_point)
    }
}
