use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[read_component(Carried)]
#[read_component(Item)]
#[read_component(Weapon)]
#[read_component(Armor)]
pub fn player_input(
    ecs: &SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
    if let Some(key) = *key {
        let mut new_turn = TurnState::PlayerTurn;

        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            VirtualKeyCode::G => {
                let (player_entity, player_pos) = players
                    .iter(ecs)
                    .find_map(|(entity, pos)| Some((*entity, *pos)))
                    .unwrap();
                let mut items = <(Entity, &Point)>::query().filter(component::<Item>());
                items.iter(ecs)
                    .filter(|(_, pos)| **pos == player_pos)
                    .for_each(|(item_entity, _)| {
                        commands.remove_component::<Point>(*item_entity);
                        commands.add_component(*item_entity, Carried(player_entity));

                        //You can only carry one weapon and one armor.
                        if ecs.entry_ref(*item_entity).unwrap().get_component::<Weapon>().is_ok() {
                            <(Entity, &Carried)>::query().filter(component::<Weapon>())
                                .iter(ecs)
                                .filter(|(_, carried)| carried.0 == player_entity)
                                .for_each(|(wepons_entity, _) | {
                                    commands.remove(*wepons_entity);
                                }
                            );
                        }
                        if ecs.entry_ref(*item_entity).unwrap().get_component::<Armor>().is_ok() {
                            <(Entity, &Carried)>::query().filter(component::<Armor>())
                                .iter(ecs)
                                .filter(|(_, carried)| carried.0 == player_entity)
                                .for_each(|(wepons_entity, _) | {
                                    commands.remove(*wepons_entity);
                                }
                            );
                        }
                    }
                );
                //grabing items doesn't skip player's turn.
                new_turn = TurnState::AwaitingInput;

                Point::zero()
            }
            VirtualKeyCode::M => {
                new_turn = TurnState::WorldMap;
                Point::zero()
            },
            VirtualKeyCode::Key1 => use_item(0, ecs, commands),
            VirtualKeyCode::Key2 => use_item(1, ecs, commands),
            VirtualKeyCode::Key3 => use_item(2, ecs, commands),
            VirtualKeyCode::Key4 => use_item(3, ecs, commands),
            VirtualKeyCode::Key5 => use_item(4, ecs, commands),
            VirtualKeyCode::Key6 => use_item(5, ecs, commands),
            VirtualKeyCode::Key7 => use_item(6, ecs, commands),
            VirtualKeyCode::Key8 => use_item(7, ecs, commands),
            VirtualKeyCode::Key9 => use_item(8, ecs, commands),
            _ => Point::new(0, 0),
        };

        let (player_entity, destination) = players
            .iter(ecs)
            .find_map(|(entity, pos)| Some((*entity, *pos + delta)))
            .unwrap();

        let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());
        if delta.x != 0 || delta.y != 0 {
            let mut hit_something = false;
            enemies
                .iter(ecs)
                .filter(|(_, pos)| **pos == destination)
                .for_each(|(entity, _)| {
                    hit_something = true;

                    commands.push((
                        (),
                        WantsToAttack {
                            attacker: player_entity,
                            victim: *entity,
                        },
                    ));
                });

            if !hit_something {
                commands.push((
                    (),
                    WantsToMove {
                        entity: player_entity,
                        destination,
                    },
                ));
            }
        }
        *turn_state = new_turn;
    }
}

fn use_item(n: usize, ecs: &SubWorld, commands: &mut CommandBuffer) -> Point {
    let player_entity = <Entity>::query()
        .filter(component::<Player>())
        .iter(ecs)
        .find_map(|entity| Some(*entity))
        .unwrap();

    let item_entity = <(Entity, &Carried)>::query()
        .filter(component::<Item>())
        .iter(ecs)
        .filter(|(_, carried)| carried.0 == player_entity)
        .enumerate()
        .filter(|(item_count, (_, _))| *item_count == n)
        .find_map(|(_, (item_entity, _))| Some(*item_entity));

    if let Some(item_entity) = item_entity {
        commands.push((
            (),
            ActiveItem {
                used_by: player_entity,
                item: item_entity,
            },
        ));
    }

    Point::zero()
}
