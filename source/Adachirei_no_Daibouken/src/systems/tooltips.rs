use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Name)]
#[read_component(Health)]
#[read_component(FieldOfVeiw)]
#[read_component(Player)]
pub fn tooltips(
    ecs: &SubWorld,
    #[resource] mouse_pos: &Point, // (1)
    #[resource] camera: &Camera,   // (2)
) {
    let mut positions = <(Entity, &Point, &Name)>::query(); // (3)
    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_pos + offset; // (4)
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(3);

    let player_fov = <&FieldOfVeiw>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .nth(0)
        .unwrap();
    positions
        .iter(ecs) // (5)
        .filter(|(_, pos, _)| **pos == map_pos && player_fov.visible_tiles.contains(*pos)) // (6)
        .for_each(|(entity, _, name)| {
            let screen_pos = *mouse_pos * 3; // (7)
            let display = if let Ok(health) = ecs
                .entry_ref(*entity) // (8)
                .unwrap()
                .get_component::<Health>()
            {
                format!("{} : {} hp", &name.0, health.current) // (9)
            } else {
                // (10)
                name.0.clone()
            };
            draw_batch.print(screen_pos, &display);
        });
    draw_batch.submit(10100).expect("Batch error");
}
