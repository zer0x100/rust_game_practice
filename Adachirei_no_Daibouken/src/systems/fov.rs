use crate::prelude::*;

#[system]
#[write_component(FieldOfVeiw)]
#[read_component(Point)]
pub fn fov(ecs: &mut SubWorld, #[resource] map: &Map) {
    <(&mut FieldOfVeiw, &Point)>::query()
        .iter_mut(ecs)
        .filter(|(fov, _)| fov.is_dirty)
        .for_each(|(fov, pos)| {
            fov.visible_tiles = field_of_view_set(*pos, fov.radius, map);
            fov.is_dirty = false;
        });
}
