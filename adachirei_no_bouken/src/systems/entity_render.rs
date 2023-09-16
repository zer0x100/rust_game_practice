use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &mut SubWorld, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);//set target console

    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(pos, render)| {
            draw_batch.set(
                *pos - Point::new(camera.left_x, camera.top_y),
                render.color,
                render.glyph,
            );
        }
    );

    draw_batch.submit(NUM_TILES + 1000).expect("Batch error");
}