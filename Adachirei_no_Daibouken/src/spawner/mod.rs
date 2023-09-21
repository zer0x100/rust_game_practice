mod template;

use std::collections::HashSet;

use crate::prelude::*;
use template::Templates;
//available from outside
pub use template::SpecialTag;

pub fn spawn_player(ecs: &mut World, pos: Point) {
    let entity = ecs.push(
        (
            Name("Adachi Rei".to_string()),
            Player {
                map_level: 0,
                direction: Direction::Left,
                left_glyph: 17,
                right_glyph: 16,
                up_glyph: 30,
                down_glyph: 31, 
            },
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('@'),
            },
            Health {
                current: 15,
                max: 15,
            },
            FieldOfVeiw {
                visible_tiles: HashSet::new(),
                radius: 4,
                is_dirty: true,
            },
            Damage(1),
            Defense(0),
        )
    );

    //can't push more than 9 components at once?
    let mut cb = CommandBuffer::new(&ecs);
    cb.add_component(entity, AttackFrames{
        left: [48, 49, 50, 51, 52, 53],
        right: [48, 49, 50, 51, 52, 53],
        up: [1, 2, 3, 4, 5, 6],
        down: [1, 2, 3, 4, 5, 6],
    });
    cb.flush(ecs);
}

//Levelに合わせてモンスターとアイテムをランダム発生
pub fn spawn_level(
    ecs: &mut World,
    rng: &mut RandomNumberGenerator,
    level: usize,
    spawn_points: &[Point],
) {
    let templates = Templates::load();
    templates.spawn_entities(ecs, rng, level, spawn_points);
}

//ランダム発生以外の、固有敵・武器などSpecialTagが付けられたものを発生させる。
pub fn spawn_special_tagged(ecs: &mut World, pos: Point, tag: template::SpecialTag) {
    let templates = Templates::load();
    let boss_template = templates
        .entities
        .iter()
        .filter(|template| template.special_tag == Some(tag.clone()))
        .nth(0)
        .expect("Templates::load Error, No Boss exists");
    let mut commands = CommandBuffer::new(ecs);
    templates.spawn_entity(&pos, boss_template, &mut commands);
    commands.flush(ecs);
}
