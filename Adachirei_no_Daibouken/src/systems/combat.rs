use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[write_component(Health)]
#[read_component(Carried)]
#[read_component(Damage)]
#[read_component(Defense)]
#[read_component(Point)]
#[read_component(AttackFrames)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer, #[resource] turn: &mut TurnState, #[resource] current_time: &f32) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();

    let victims: Vec<(Entity, Entity, Entity)> = attackers // (1)
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.attacker, attack.victim)) // (2)
        .collect(); // (3)

    victims.iter().for_each(|(message, attacker, victim)| {
        //If the attacker has AttackMotion, display it.
        //send EffectMotion Message, and chage TurnState to EffectAnime
        if let Ok(attacker) = ecs.entry_ref(*attacker) {
            if let Ok(attack_motion) = attacker.get_component::<AttackFrames>() {
                if let Ok(attacker_pos) = attacker.get_component::<Point>() {
                    if let Ok(victim) = ecs.entry_ref(*victim) {
                        if let Ok(victim_pos) = victim.get_component::<Point>() {
                            let direction = *victim_pos - *attacker_pos;
                            let anime_frames = match (direction.x, direction.y) {
                                (-1, 0) => attack_motion.left,
                                (1, 0) => attack_motion.right,
                                (0, -1) => attack_motion.up,
                                _ => attack_motion.down
                            };

                            commands.push(
                                ((), EffectMotion{
                                    position: *attacker_pos,
                                    anime_frames,
                                    current_frame: 0,
                                    last_frame_time: *current_time,
                                    prev_turn: *turn,
                                })
                            );
                            
                            *turn = TurnState::EffectAnime;
                        }
                    }
                }
            }
        }

        //calculate the damage
        let base_damage = if let Ok(v) = ecs.entry_ref(*attacker) {
            if let Ok(dmg) = v.get_component::<Damage>() {
                dmg.0
            } else {
                0
            }
        } else {
            0
        };
        let weapon_damage: i32 = <(&Carried, &Damage)>::query()
            .iter(ecs)
            .filter(|(carried, _)| carried.0 == *attacker)
            .map(|(_, dmg)| dmg.0)
            .sum();
        let base_defense = if let Ok(v) = ecs.entry_ref(*victim) {
            if let Ok(defense) = v.get_component::<Defense>() {
                defense.0
            } else {
                0
            }
        } else {
            0
        };
        let armor_defense: i32 = <(&Carried, &Defense)>::query()
            .iter(ecs)
            .filter(|(carried, _)| carried.0 == *victim)
            .map(|(_, defense)| defense.0)
            .sum();
        let final_damage = std::cmp::max(
            0,
            base_damage + weapon_damage - (base_defense + armor_defense),
        );

        if let Ok(mut health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            health.current -= final_damage;
        }
        commands.remove(*message);
    });
}
