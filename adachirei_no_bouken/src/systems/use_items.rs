use crate::prelude::*;

#[system]
#[read_component(ActiveItem)]
#[read_component(ProvidesHealing)]
#[read_component(ProvidesDungeonMap)]
#[write_component(Health)]
pub fn use_items(ecs: &mut SubWorld, #[resource] map: &mut Map, commands: &mut CommandBuffer) {
    let mut healing_to_apply = Vec::<(Entity, i32)>::new();

    <(Entity, &ActiveItem)>::query()
        .iter(ecs)
        .for_each(|(entity, activate)| {
            let item = ecs.entry_ref(activate.item);
            if let Ok(item) = item {
                if let Ok(healing) = item.get_component::<ProvidesHealing>() {
                    healing_to_apply.push((activate.used_by, healing.amount));
                }
                if let Ok(_mapper) = item.get_component::<ProvidesDungeonMap>() {
                    map.revealed_tiles.iter_mut().for_each(|t| *t = true);
                }
            }

            commands.remove(activate.item);
            commands.remove(*entity);
        });

    //Apply Healing Events
    for heal in healing_to_apply.iter() {
        if let Ok(mut target) = ecs.entry_mut(heal.0) {
            if let Ok(health) = target.get_component_mut::<Health>() {
                health.current = i32::min(health.max, health.current + heal.1)
            }
        }
    }
}
