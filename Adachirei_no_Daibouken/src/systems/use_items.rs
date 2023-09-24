use crate::prelude::*;

#[system]
#[read_component(ActiveItem)]
#[read_component(ProvidesHealing)]
#[read_component(ProvidesWiderView)]
#[read_component(ProvidesSurroundingAttack)]
#[read_component(ProvidesLinerAttack)]
#[read_component(Point)]
#[read_component(Direction)]
#[write_component(Health)]
#[write_component(FieldOfVeiw)]
pub fn use_items(ecs: &mut SubWorld, commands: &mut CommandBuffer, #[resource] map: &Map) {
    let mut healing_to_apply = Vec::<(Entity, i32)>::new();
    let mut wider_view_to_apply = Vec::<(Entity, i32)>::new();

    let mut positions = <(Entity, &Point)>::query().filter(component::<Health>());

    <(Entity, &ActiveItem)>::query()
        .iter(ecs)
        .for_each(|(entity, activate)| {
            let item = ecs.entry_ref(activate.item);
            if let Ok(item) = item {
                if let Ok(user) = ecs.entry_ref(activate.used_by) {
                    if let Ok(user_pos) = user.get_component::<Point>() {
                        if let Ok(healing) = item.get_component::<ProvidesHealing>() {
                            healing_to_apply.push((activate.used_by, healing.amount));

                            //Heal Animation
                            send_heal_effect(user_pos, commands);
                        }
                        if let Ok(wider_view) = item.get_component::<ProvidesWiderView>() {
                            wider_view_to_apply.push((activate.used_by, wider_view.amount));

                            //Light Animation
                            send_light_effect(user_pos, commands);
                        }
                        if let Ok(attack) = item.get_component::<ProvidesSurroundingAttack>() {
                            positions.iter(ecs)
                                .filter(|(victim, pos)| DistanceAlg::Pythagoras
                                    .distance2d(*user_pos, **pos) < 1.7 && **victim != activate.used_by
                                )
                                .for_each(|(victim, _)| healing_to_apply.push((*victim, -attack.amount)));
        
                            //Shock Wave Animation
                            send_shock_effects(user_pos, commands);
                        }
                        if let Ok(attack) = item.get_component::<ProvidesLinerAttack>() {
                            if let Ok(user_direction) = user.get_component::<Direction>() {
                                positions.iter(ecs)
                                    .filter(|(_, pos)| {
                                        let mut target_pos = *user_pos + user_direction.unit_vector();
                                        let mut count = 1;
                                        while map.can_enter_tile(target_pos) && count <= 8{
                                            if **pos == target_pos { return true; }
                                            target_pos += user_direction.unit_vector();
                                            count += 1;
                                        }
                                        return false;
                                    })
                                    .for_each(|(victim, _)| { healing_to_apply.push((*victim, -attack.amount))});
                                
                                //Rocket Punch Animation
                                send_rocket_punch_effect(user_pos, user_direction, map, commands);
                            }
                        }
                    }
                }
            }

            commands.remove(activate.item);
            commands.remove(*entity);
        });

    //Apply Healing Events
    //You can apply damage by negative healing.
    for heal in healing_to_apply.iter() {
        if let Ok(mut target) = ecs.entry_mut(heal.0) {
            if let Ok(health) = target.get_component_mut::<Health>() {
                health.current = i32::min(health.max, health.current + heal.1)
            }
        }
    }

    //Apply Wider Views
    for wider_view in wider_view_to_apply.iter() {
        if let Ok(mut target) = ecs.entry_mut(wider_view.0) {
            if let Ok(field_of_view) = target.get_component_mut::<FieldOfVeiw>() {
                field_of_view.radius = field_of_view.radius + wider_view.1;
                field_of_view.is_dirty = true;
            }
        }
    }
}

fn send_heal_effect(pos: &Point, commands: &mut CommandBuffer) {
    commands.push(
        ((), EffectMotion {
            position: *pos,
            console: 4,
            anime_frames: smallvec![106, 106, 107, 107, 108, 108],
            current_frame: 0,
            elasped_time_from_last_frame: 0.0,
        })
    );
}

fn send_light_effect(pos: &Point, commands: &mut CommandBuffer) {
    commands.push(
        ((), EffectMotion {
            position: *pos,
            console: 4,
            anime_frames: smallvec![105, 104, 103, 105, 104, 103],
            current_frame: 0,
            elasped_time_from_last_frame: 0.0,
        })
    );
}

fn send_shock_effects(pos: &Point, commands: &mut CommandBuffer) {
    for iy in -1..=1 {
        for ix in -1..=1 {
            commands.push(
                ((), EffectMotion {
                    position: *pos + Point::new(ix, iy),
                    console: 4,
                    anime_frames: smallvec![7, 7, 9, 9, 8, 8, 10, 10],
                    current_frame: 0,
                    elasped_time_from_last_frame: 0.0,
                })
            );
        }
    }
}

fn send_rocket_punch_effect(pos: &Point, direction: &Direction, map: &Map, commands: &mut CommandBuffer) {
    let fist_glyph = match *direction {
        Direction::Left => 63,
        Direction::Right => 79,
        Direction::Up => 31,
        Direction::Down => 47,
    };

    let mut target_pos = *pos + direction.unit_vector();
    let mut offset = 0;
    while map.can_enter_tile(target_pos) && offset < 7 {
        let mut frames = smallvec![0; 8];
        frames[offset] = fist_glyph;
        commands.push(
            ((), EffectMotion {
                position: target_pos,
                console: 4,
                anime_frames: frames,
                current_frame: 0,
                elasped_time_from_last_frame: 0.0,
            })
        );

        offset += 1;
        target_pos += direction.unit_vector();
    }
}