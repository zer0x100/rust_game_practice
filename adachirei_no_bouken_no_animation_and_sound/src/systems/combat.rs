use crate::prelude::*;

#[system]
#[read_component(WantsToAttack)]
#[read_component(Player)]
#[write_component(Health)]
#[read_component(Carried)]
#[read_component(Damage)]
#[read_component(Defense)]
pub fn combat(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttack)>::query();

    let victims: Vec<(Entity, Entity, Entity)> = attackers // (1)
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.attacker, attack.victim)) // (2)
        .collect(); // (3)

    victims.iter().for_each(|(message, attacker, victim)| {
        let is_player = ecs
            .entry_ref(*victim)
            .unwrap()
            .get_component::<Player>()
            .is_ok();
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
            if health.current < 1 && !is_player {
                commands.remove(*victim);
            }
        }
        commands.remove(*message);
    });
}