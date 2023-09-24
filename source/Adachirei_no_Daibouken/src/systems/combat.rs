use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[write_component(Health)]
#[read_component(Carried)]
#[read_component(Damage)]
#[read_component(Defense)]
#[read_component(Point)]
#[read_component(AttackFrames)]
#[read_component(DamageFrames)]
#[read_component(Direction)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();

    let victims: Vec<(Entity, Entity, Entity)> = attackers // (1)
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.attacker, attack.victim)) // (2)
        .collect(); // (3)

    //If an attacker has AttackMotion, send EffectMotion Message
    victims.iter().for_each(|(message, attacker, victim)| {
        //EffectMotion Message
        if let Ok(attacker) = ecs.entry_ref(*attacker){
            if let Ok(victim) = ecs.entry_ref(*victim) {
                if let Ok(attacker_pos) = attacker.get_component::<Point>() {
                    if let Ok(victim_pos) = victim.get_component::<Point>() {
                        if let Ok(attack_motion) = attacker.get_component::<AttackFrames>() {
                            let direction = *victim_pos - *attacker_pos;
                            let anime_frames = match (direction.x, direction.y) {
                                (-1, 0) => attack_motion.left.clone(),
                                (1, 0) => attack_motion.right.clone(),
                                (0, -1) => attack_motion.up.clone(),
                                _ => attack_motion.down.clone(),
                            };

                            commands.push(
                                ((), EffectMotion{
                                    position: *attacker_pos,
                                    console: 2,
                                    anime_frames,
                                    current_frame: 0,
                                    elasped_time_from_last_frame: 0.0,
                                })
                            );
                        }
                        if let Ok(damage_motion) = victim.get_component::<DamageFrames>() {
                            if let Ok(direction) = victim.get_component::<Direction>() {
                                let anime_frames = match direction {
                                    Direction::Left => damage_motion.left.clone(),
                                    Direction::Right => damage_motion.right.clone(),
                                    Direction::Up => damage_motion.up.clone(),
                                    Direction::Down => damage_motion.down.clone(),
                                };

                                commands.push(
                                    ((), EffectMotion{
                                        position: *victim_pos,
                                        console: 2,
                                        anime_frames,
                                        current_frame: 0,
                                        elasped_time_from_last_frame: 0.0,
                                    })
                                );
                            }
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
