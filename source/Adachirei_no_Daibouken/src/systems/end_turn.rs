use crate::prelude::*;

#[system]
#[read_component(Player)]
#[read_component(Health)]
#[read_component(Point)]
#[read_component(Carried)]
#[read_component(Boss)]
#[read_component(EffectMotion)]
pub fn end_turn(ecs: &SubWorld, #[resource] turn_state: &mut TurnState, #[resource] map: &Map, commands: &mut CommandBuffer) {
    let mut players = <(&Health, &Point)>::query().filter(component::<Player>());

    let boss_health_default = 1;
    let mut boss = <&Health>::query().filter(component::<Boss>());
    let boss_health = boss
        .iter(ecs)
        .find_map(|health| Some(health.current))
        .unwrap_or(boss_health_default);

    let currnet_state = turn_state.clone();
    let mut new_state = match turn_state {
        TurnState::AwaitingInput => TurnState::PlayerTurn,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
        _ => currnet_state,
    };

    //check effects animation message
    //if effectmotion message exists, change turn and send TurnBeforeEffects
    if <&EffectMotion>::query().iter(ecs).count() > 0 {
        commands.push(((), TurnBeforeEffects(currnet_state)));
        new_state = TurnState::EffectAnime;
    }

    //check game over and victory, next-level
    players.iter(ecs).for_each(|(health, pos)| {
        if health.current < 1 {
            new_state = TurnState::GameOver;
        }
        let idx = map.point2d_to_index(*pos);
        if map.tiles[idx] == TileType::Exit {
            new_state = TurnState::NextLevel;
        }
    });
    if boss_health < 1 {
        new_state = TurnState::Victory;
    }

    *turn_state = new_state;
}
