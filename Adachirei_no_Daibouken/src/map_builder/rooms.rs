use super::MapArchitect;
use crate::prelude::*;
const NUM_ROOMS: usize = 20;

pub struct RoomsArchitect {}

impl RoomsArchitect {}

impl MapArchitect for RoomsArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
            theme: super::themes::ResearchInstitueTheme::new(),
        };

        mb.fill(TileType::Wall);
        let rooms = self.build_random_rooms(rng, &mut mb.map);
        self.build_corridors(rng, &mut mb, &rooms);
        mb.player_start = rooms[0].center();
        mb.amulet_start = mb.find_most_distant();
        mb.monster_spawns = mb.spawn_monsters(&mb.player_start, rng);

        mb
    }
}

impl RoomsArchitect {
    fn build_random_rooms(&self, rng: &mut RandomNumberGenerator, map: &mut Map) -> Vec<Rect> {
        let mut rooms = Vec::<Rect>::new();

        while rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );
            let mut overlap = false;
            for r in rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }
            if !overlap {
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
                        let idx = map_idx(p.x, p.y);
                        map.tiles[idx] = TileType::Floor;
                    }
                });

                rooms.push(room);
            }
        }

        rooms
    }

    fn build_corridors(&self, rng: &mut RandomNumberGenerator, map_builder: &mut MapBuilder, rooms: &Vec<Rect>) {
        let mut rooms_sub = rooms.clone();
        rooms_sub.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms_sub.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            if rng.range(0, 2) == 1 {
                map_builder.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                map_builder.apply_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                map_builder.apply_vertical_tunnel(prev.y, new.y, prev.x);
                map_builder.apply_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }

}
