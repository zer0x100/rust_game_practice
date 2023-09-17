use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Point)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn: &mut TurnState,
    commands: &mut CommandBuffer,
) {
    if let Some(key) = *key {
        let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::zero(),
        };

        players
            .iter(ecs)
            .for_each(|(entity, pos)| {
                let destination = *pos + delta;
                commands.push(((), WantsToMove{ entity: *entity, destination}));
            }
        );

        *turn = TurnState::PlayerTurn;
    }
}