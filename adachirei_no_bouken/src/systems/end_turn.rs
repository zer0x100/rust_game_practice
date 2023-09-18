use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Health)]
#[read_component(Point)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState) {
    let mut players = <(&Health, &Point)>::query().filter(component::<Player>());
    let mut amulet = <&Point>::query().filter(component::<AmuletOfYala>());
    let amulet_pos = amulet.iter(ecs).nth(0).unwrap();

    let currnet_state = turn_state.clone();
    let mut new_state = match turn_state {
        TurnState::AwaitingInput => return,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => currnet_state,
    };

    //check game over
    players.iter(ecs).for_each(|(health, pos)| {
        if health.current < 1 {
            new_state = TurnState::GameOver;
        }
        if *pos == *amulet_pos {
            new_state = TurnState::Victory;
        }
    });

    *turn_state = new_state;
}
