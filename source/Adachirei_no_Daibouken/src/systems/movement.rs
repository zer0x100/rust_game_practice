use crate::prelude::*;

#[system(for_each)]
#[read_component(Player)]
#[read_component(FieldOfVeiw)]
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

                //If player moves, update camera and revealed_tiles
                if entry.get_component::<Player>().is_ok() {
                    camera.on_player_move(want_move.destination);
                    fov.visible_tiles.iter().for_each(|pos| {
                        map.revealed_tiles[map_idx(pos.x, pos.y)] = true;
                    });
                }
            }
        }
    }
    commands.remove(*entity);
}
