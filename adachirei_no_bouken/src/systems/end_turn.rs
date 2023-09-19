use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Health)]
#[read_component(Point)]
#[read_component(Carried)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState, #[resource] map: &Map) {
    let mut players = <(Entity, &Health, &Point)>::query().filter(component::<Player>());
    let mut amulet = <&Carried>::query().filter(component::<AmuletOfYala>());
    let amulet_carrier = amulet.iter(ecs)
        .find_map(|carried| Some(carried.0));

    let currnet_state = turn_state.clone();
    let mut new_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => currnet_state,
    };

    //check game over and victory, next-level
    players.iter(ecs).for_each(|(player_entity, health, pos)| {
        if health.current < 1 {
            new_state = TurnState::GameOver;
        }
        let idx = map.point2d_to_index(*pos);
        if map.tiles[idx] == TileType::Exit {
            new_state = TurnState::NextLevel;
        }
        if let Some(amulet_carrier) = amulet_carrier {
            if amulet_carrier == *player_entity {
                new_state = TurnState::Victory;
            }
        }
    });

    *turn_state = new_state;
}
