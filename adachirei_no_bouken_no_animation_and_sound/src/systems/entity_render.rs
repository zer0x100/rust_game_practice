use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(FieldOfVeiw)]
#[read_component(Player)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    let offset = Point::new(camera.left_x, camera.top_y);
    let player_fov = <&FieldOfVeiw>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .nth(0)
        .unwrap();

    <(Entity, &Point, &Render)>::query()
        .iter(ecs)
        .filter(|(_, pos, _)| player_fov.visible_tiles.contains(pos))
        .for_each(|(entity, pos, render)| {
            //Playerだけ方向付きのRendering
            if let Ok(player) = ecs.entry_ref(*entity).unwrap().get_component::<Player>() {
                let glyph = match player.direction {
                    Direction::Left => player.left_glyph,
                    Direction::Right => player.right_glyph,
                    Direction::Up => player.up_glyph,
                    Direction::Down => player.down_glyph,
                };
                draw_batch.set(*pos - offset, render.color, glyph);
            } else {
                draw_batch.set(*pos - offset, render.color, render.glyph);
            }
        });
    draw_batch.submit(5000).expect("Batch error");
}
