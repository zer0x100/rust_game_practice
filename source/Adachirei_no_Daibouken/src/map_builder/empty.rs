use super::MapArchitect;
use crate::prelude::*;

pub struct EmptyArchitect {}

impl MapArchitect for EmptyArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
            theme: super::themes::ResearchInstitueTheme::new(),
        };

        mb.fill(TileType::Floor);
        mb.player_start = Point::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
        mb.amulet_start = mb.find_most_distant();
        mb.monster_spawns = mb.spawn_monsters(&mb.player_start, &[mb.amulet_start], rng);

        mb
    }
}
