use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
#[read_component(FieldOfVeiw)]
#[read_component(Point)]
#[read_component(Item)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &mut Map,
    #[resource] camera: &mut Camera,
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
) {
    if map.can_enter_tile(want_move.destination) {
        commands.add_component(want_move.entity, want_move.destination);
        //特に、FieldOfViewを持ってるなら更新
        if let Ok(entry) = ecs.entry_ref(want_move.entity) {
            if let Ok(fov) = entry.get_component::<FieldOfVeiw>() {
                commands.add_component(want_move.entity, fov.clone_dirty());

                //If player moves, then move camera and update visible tiles.
                //especially, if there exits an item. pick up it.
                if entry.get_component::<Player>().is_ok() {
                    camera.on_player_move(want_move.destination);
                    fov.visible_tiles.iter().for_each(|pos| {
                        map.revealed_tiles[map_idx(pos.x, pos.y)] = true;
                    });

                    let mut items = <(Entity, &Point)>::query().filter(component::<Item>());
                    items.iter(ecs)
                        .filter(|(_, pos)| **pos == want_move.destination)
                        .for_each(|(item_entity, _)| {
                            commands.add_component(*item_entity, Carried(want_move.entity));
                            commands.remove_component::<Point>(*item_entity);
                        }
                    );
                }
            }
        }
    }
    commands.remove(*entity);
}
