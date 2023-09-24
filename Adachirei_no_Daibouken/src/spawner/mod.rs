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
                left_frames: smallvec![20, 20, 21, 21, 22, 22, 21, 21],
                right_frames: smallvec![23, 23, 24, 24, 25, 25, 24, 24],
                up_frames: smallvec![26, 26, 27, 27, 28, 28, 27, 27],
                down_frames: smallvec![17, 17, 18, 18, 19, 19, 18, 18],
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
        left: smallvec![20, 21, 22, 21, 20, 21],
        right: smallvec![23, 24,25, 24, 23, 24],
        up: smallvec![26, 27, 28, 27, 26, 27],
        down: smallvec![17, 18, 19, 18, 17, 18],
    });
    cb.add_component(entity, DamageFrames{
        left: smallvec![20, 0, 21, 0, 22, 0],
        right: smallvec![23, 0, 24, 0, 25, 0],
        up: smallvec![26, 0, 27, 0, 28, 0],
        down: smallvec![17, 0, 18, 0, 19, 0],
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
