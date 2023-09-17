use crate::prelude::*;

#[system]
pub fn end_turn(#[resource] turn: &mut TurnState) {
    let new_state = match turn {
        TurnState::AwaitingInput => TurnState::AwaitingInput,
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        TurnState::MonsterTurn => TurnState::AwaitingInput,
    };
    *turn = new_state;
}