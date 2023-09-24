use crate::prelude::*;

#[system]
#[read_component(Enemy)]
#[read_component(Health)]
pub fn remove_enemies(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    <(Entity, &Health)>::query().filter(component::<Enemy>())
        .iter(ecs)
        .filter(|(_, health)| health.current < 1)
        .for_each(|(entity, _)| commands.remove(*entity));
}