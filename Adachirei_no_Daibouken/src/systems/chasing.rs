use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(ChasingPlayer)]
#[read_component(Point)]
#[read_component(Health)]
#[read_component(FieldOfVeiw)]
#[read_component(HeatSeeking)]
pub fn chasing(ecs: &SubWorld, #[resource] map: &Map, commands: &mut CommandBuffer) {
    let mut movers = <(Entity, &Point, &FieldOfVeiw)>::query().filter(component::<ChasingPlayer>());
    let mut positions = <(Entity, &Point)>::query().filter(component::<Health>());
    let mut player = <&Point>::query().filter(component::<Player>());
    let player_pos = player.iter(ecs).nth(0).unwrap();
    let player_idx = map.point2d_to_index(*player_pos);

    let search_targets = vec![player_idx];
    let dijkstra_map = DijkstraMap::new(SCREEN_WIDTH, SCREEN_HEIGHT, &search_targets, map, 1024.0);

    movers.iter(ecs).for_each(|(entity, pos, fov)| {
        //check heat seeking
        let is_heat_seeking = if let Ok(entry) = ecs.entry_ref(*entity) {
            if let Ok(heat_seeking) = entry.get_component::<HeatSeeking>() {
                heat_seeking.saw_player
            } else {
                false
            }
        } else {
            false
        };
        if !fov.visible_tiles.contains(player_pos) {
            if !is_heat_seeking {
                return;
            }
        } else {
            if let Ok(entry) = ecs.entry_ref(*entity) {
                if entry.get_component::<HeatSeeking>().is_ok() {
                    commands.add_component(*entity, HeatSeeking{ saw_player: true});
                }
            }
        }

        let idx = map.point2d_to_index(*pos);
        if let Some(destination) = DijkstraMap::find_lowest_exit(&dijkstra_map, idx, map) {
            let distance = DistanceAlg::Pythagoras.distance2d(*player_pos, *pos);
            let destination = if distance < 1.2 {
                *player_pos
            } else {
                map.index_to_point2d(destination)
            };

            let mut hit_something = false;
            positions
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(victim, _)| {
                    hit_something = true;

                    if ecs
                        .entry_ref(*victim)
                        .unwrap()
                        .get_component::<Player>()
                        .is_ok()
                    {
                        commands.push((
                            (),
                            WantsToAttack {
                                attacker: *entity,
                                victim: *victim,
                            },
                        ));
                    }
                });
            if !hit_something {
                commands.push((
                    (),
                    WantsToMove {
                        entity: *entity,
                        destination,
                    },
                ));
            }
        }
    });
}
