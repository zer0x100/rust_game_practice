use crate::prelude::*;

#[system]
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);//set target console

    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..=camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);

            if in_bounds(pt) {
                let glyph = match map.tiles[map_idx(x, y)] {
                    TileType::Floor => to_cp437('.'),
                    TileType::Wall => to_cp437('#'),
                };

                draw_batch.set(
                    pt - offset,
                    ColorPair::new(WHITE, BLACK),
                    glyph,
                );
            }
        }
    }
    draw_batch.submit(0).expect("Batch error");
}