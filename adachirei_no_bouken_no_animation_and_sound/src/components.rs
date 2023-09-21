pub use crate::prelude::*;
use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player {
    pub map_level: u32,
    pub direction: Direction,
    pub left_glyph: FontCharType,
    pub right_glyph: FontCharType,
    pub up_glyph: FontCharType,
    pub down_glyph: FontCharType,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Point,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToAttack {
    pub attacker: Entity,
    pub victim: Entity,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Clone, PartialEq)]
pub struct Name(pub String);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChasingPlayer;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Item;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Boss;

#[derive(Clone, Debug, PartialEq)]
pub struct FieldOfVeiw {
    pub visible_tiles: HashSet<Point>,
    pub radius: i32,
    pub is_dirty: bool,
}

impl FieldOfVeiw {
    pub fn new(radius: i32) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius,
            is_dirty: true,
        }
    }

    pub fn clone_dirty(&self) -> Self {
        Self {
            visible_tiles: HashSet::new(),
            radius: self.radius,
            is_dirty: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProvidesHealing {
    pub amount: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProvidesWiderView {
    pub amount: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Carried(pub Entity);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ActiveItem {
    pub used_by: Entity,
    pub item: Entity,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Damage(pub i32);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Weapon;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Defense(pub i32);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Armor;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeatSeeking {
    pub saw_player: bool,
}