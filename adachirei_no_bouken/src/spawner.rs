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