use crate::prelude::*;

pub fn spawn_player(pos: Point, ecs: &mut World) {
    ecs.push(
        (
            Player,
            Render{
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('@'),
            },
            pos,
        )
    );
}

pub fn spawn_monster(pos: Point, ecs: &mut World, rng: &mut RandomNumberGenerator) {
    ecs.push(
        (
            Enemy,
            Render{
                color: ColorPair::new(WHITE, BLACK),
                glyph: match rng.range(0, 4) {
                    0 => to_cp437('E'),
                    1 => to_cp437('O'),
                    2 => to_cp437('o'),
                    _ => to_cp437('g'),
                }
            },
            pos,
            MovingRandomly,
        )
    );
}