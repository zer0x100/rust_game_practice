use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(FieldOfVeiw)]
pub fn map_render(ecs: &SubWorld, #[resource] map: &Map, #[resource] camera: &Camera) {
    let player_fov = <&FieldOfVeiw>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .nth(0)
        .unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);
    for y in camera.top_y ..= camera.bottom_y {
        for x in camera.left_x .. camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            if map.in_bounds(pt) && (player_fov.visible_tiles.contains(&pt)
                || map.revealed_tiles[map_idx(x, y)])
            {
                let idx = map_idx(x, y);
                let glyph = match map.tiles[idx] {
                    TileType::Floor => to_cp437('.'),
                    TileType::Wall => to_cp437('#'),
                };

                let color = if player_fov.visible_tiles.contains(&pt) {
                    WHITE
                } else {
                    DARKGRAY
                };
                draw_batch.set(// (1)
                    pt - offset,
                    ColorPair::new(
                        color,
                        BLACK
                    ),
                    glyph
                );
            }
        }
    }
    draw_batch.submit(0).expect("Batch error");// (2)
}

