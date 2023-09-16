use crate::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Player;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Render {
    pub color: ColorPair,
    pub glyph: FontCharType,
}