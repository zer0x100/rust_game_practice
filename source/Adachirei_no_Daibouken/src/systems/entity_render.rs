use crate::prelude::*;

#[system]
#[read_component(Point)]
#[write_component(Render)]
#[read_component(Direction)]
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

    <(&Point, &mut Render, &Direction)>::query()
        .iter_mut(ecs)
        .filter(|(pos, _, _)| player_fov.visible_tiles.contains(pos))
        .for_each(|(pos, render, direction)| {
            render.elasped_time_from_last_frame += elasped_time;
            if render.elasped_time_from_last_frame > ANIME_FRAME_DURATION {
                render.elasped_time_from_last_frame = 0.0;
                render.current_frame += 1;
            }

            let glyph = match direction {
                Direction::Left => {
                    render.current_frame %= render.left_frames.len();
                    render.left_frames[render.current_frame]
                },
                Direction::Right => {
                    render.current_frame %= render.right_frames.len();
                    render.right_frames[render.current_frame]
                },
                Direction::Up => {
                    render.current_frame %= render.up_frames.len();
                    render.up_frames[render.current_frame]
                },
                Direction::Down => {
                    render.current_frame %= render.down_frames.len();
                    render.down_frames[render.current_frame]
                },
            };

            draw_batch.set(
                *pos - offset,
                render.color,
                glyph,
            );
        });
    draw_batch.submit(5000).expect("Batch error");
}
