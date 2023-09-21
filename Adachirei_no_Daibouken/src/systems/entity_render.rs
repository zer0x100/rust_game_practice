use crate::prelude::*;

#[system]
#[read_component(Point)]
#[write_component(Render)]
#[read_component(FieldOfVeiw)]
#[read_component(Player)]
pub fn entity_render(ecs: &mut SubWorld, #[resource] camera: &Camera, #[resource] elasped_time: &f32) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    let offset = Point::new(camera.left_x, camera.top_y);
    let player_fov = <&FieldOfVeiw>::query()
        .iter(ecs)
        .find_map(|fov| Some(fov.clone()))
        .unwrap();

    <(&Point, &mut Render)>::query()
        .iter_mut(ecs)
        .filter(|(pos, _)| player_fov.visible_tiles.contains(pos))
        .for_each(|(pos, render)| {
                draw_batch.set(*pos - offset, render.color, render.anime_frames[render.current_frame]);

                render.elasped_time_from_last_frame += elasped_time;
                if render.elasped_time_from_last_frame > ANIME_FRAME_DURATION {
                    render.elasped_time_from_last_frame = 0.0;
                    render.current_frame += 1;
                    render.current_frame %= render.anime_frames.len();
                }
        });
    draw_batch.submit(5000).expect("Batch error");
}
