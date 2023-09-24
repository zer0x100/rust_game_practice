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
            },
            Direction::Down,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                left_frames: smallvec![64, 64, 64, 17, 17, 17],
                right_frames: smallvec![64, 64, 64, 16, 16, 16],
                up_frames: smallvec![64, 64, 64, 30, 30, 30],
                down_frames: smallvec![64, 64, 64, 31, 31, 31],
                current_frame: 0,
                elasped_time_from_last_frame: 0.0,
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
        )
    );

    //can't push more than 9 components at once?
    let mut cb = CommandBuffer::new(&ecs);
    cb.add_component(entity, AttackFrames{
        left: smallvec![48, 49, 50, 51, 52, 53],
        right: smallvec![48, 49, 50, 51, 52, 53],
        up: smallvec![1, 2, 3, 4, 5, 6],
        down: smallvec![1, 2, 3, 4, 5, 6],
    });
    cb.add_component(entity, DamageFrames{
        left: smallvec![19, 19, 173, 173, 19, 19],
        right: smallvec![19, 19, 173, 173, 19, 19],
        up: smallvec![19, 19, 173, 173, 19, 19],
        down: smallvec![19, 19, 173, 173, 19, 19],
    });
    cb.add_component(entity, Damage(1));
    cb.add_component(entity, Defense(0));
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
    let special_template = templates
        .entities
        .iter()
        .filter(|template| template.special_tag == Some(tag.clone()))
        .nth(0)
        .expect("Templates::load Error, No special template exists");
    let mut commands = CommandBuffer::new(ecs);
    templates.spawn_entity(&pos, special_template, &mut commands);
    commands.flush(ecs);
}
