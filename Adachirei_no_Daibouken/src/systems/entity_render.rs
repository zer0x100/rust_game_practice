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
    let (player_entity, player_component, player_fov) = <(Entity, &Player, &FieldOfVeiw)>::query()
        .iter(ecs)
        .find_map(|(entity, component, fov)| Some((*entity, *component, fov.clone())))
        .unwrap();

    <(Entity, &Point, &mut Render)>::query()
        .iter_mut(ecs)
        .filter(|(_, pos, _)| player_fov.visible_tiles.contains(pos))
        .for_each(|(entity, pos, render)| {
            //Playerだけ方向付きのRendering
            if *entity == player_entity {
                let glyph = match player_component.direction {
                    Direction::Left => player_component.left_glyph,
                    Direction::Right => player_component.right_glyph,
                    Direction::Up => player_component.up_glyph,
                    Direction::Down => player_component.down_glyph,
                };
                draw_batch.set(*pos - offset, render.color, glyph);
            } else {
                draw_batch.set(*pos - offset, render.color, render.anime_frames[render.current_frame]);

                render.elasped_time_from_last_frame += elasped_time;
                if render.elasped_time_from_last_frame > ANIME_FRAME_DURATION {
                    render.elasped_time_from_last_frame = 0.0;
                    render.current_frame += 1;
                    render.current_frame %= render.anime_frames.len();
                    println!("{:?}", render);
                }
            }
        });
    draw_batch.submit(5000).expect("Batch error");
}
