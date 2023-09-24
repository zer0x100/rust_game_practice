#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
    MainMenue,
    GameOver,
    Victory,
    NextLevel,
    WorldMap,
    EffectAnime,
}