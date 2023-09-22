use crate::prelude::*;
use legion::systems::CommandBuffer;
use ron::de::from_reader;
use serde::Deserialize;
use std::collections::HashSet;
use std::fs::File;

#[derive(Clone, Deserialize, Debug)]
pub struct Template {
    pub entity_type: EntityType,
    pub levels: HashSet<usize>,
    pub frequency: i32,
    pub name: String,
    pub left_frames: Vec<FontCharType>,
    pub right_frames: Vec<FontCharType>,
    pub up_frames: Vec<FontCharType>,
    pub down_frames: Vec<FontCharType>,
    pub provides: Option<Vec<(String, i32)>>,
    pub hp: Option<i32>,
    pub base_damage: Option<i32>,
    pub base_defense: Option<i32>,
    pub special_tag: Option<SpecialTag>,
    pub field_of_view_radius: Option<i32>,
    pub heat_seeking: bool,
    pub attack_left_frames: Option<Vec<FontCharType>>,
    pub attack_right_frames: Option<Vec<FontCharType>>,
    pub attack_up_frames: Option<Vec<FontCharType>>,
    pub attack_down_frames: Option<Vec<FontCharType>>,
}

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub enum EntityType {
    Enemy,
    Item,
}

//固有の敵やアイテムに付けるラベル
#[derive(Clone, Deserialize, Debug, PartialEq)]
pub enum SpecialTag {
    Boss,
    UniqueWeapon,
    UniqueArmor,
    UniqueEye,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Templates {
    pub entities: Vec<Template>,
}

impl Templates {
    pub fn load() -> Self {
        let file = File::open("resources/template.ron").expect("Failed opening file");
        from_reader(file).expect("Unable to load templates")
    }

    pub fn spawn_entities(
        &self,
        ecs: &mut World,
        rng: &mut RandomNumberGenerator,
        level: usize,
        spawn_points: &[Point],
    ) {
        let mut available_entities = Vec::new();
        self.entities
            .iter()
            .filter(|e| e.levels.contains(&level))
            .for_each(|t| {
                for _ in 0..t.frequency {
                    available_entities.push(t);
                }
            });
        let mut commands = CommandBuffer::new(ecs);
        spawn_points.iter().for_each(|pt| {
            if let Some(entity) = rng.random_slice_entry(&available_entities) {
                self.spawn_entity(pt, entity, &mut commands);
            }
        });
        commands.flush(ecs);
    }

    pub fn spawn_entity(
        &self,
        pt: &Point,
        template: &Template,
        commands: &mut legion::systems::CommandBuffer,
    ) {
        let entity = commands.push((
            pt.clone(),
            Render {
                color: ColorPair::new(WHITE, BLACK),
                left_frames: SmallVec::from_vec(template.left_frames.clone()),
                right_frames: SmallVec::from_vec(template.right_frames.clone()),
                up_frames: SmallVec::from_vec(template.up_frames.clone()),
                down_frames: SmallVec::from_vec(template.down_frames.clone()),
                current_frame: 0,
                elasped_time_from_last_frame: 0.0,
            },
            Direction::Down,
            Name(template.name.clone()),
        ));
        match template.entity_type {
            EntityType::Item => commands.add_component(entity, Item {}),
            EntityType::Enemy => {
                commands.add_component(entity, Enemy);
                commands.add_component(entity, ChasingPlayer {});
            }
        }

        if let Some(health) = &template.hp {
            commands.add_component(
                entity,
                Health {
                    max: *health,
                    current: *health,
                },
            );
        }
        if let Some(radius) = &template.field_of_view_radius {
            commands.add_component(entity, FieldOfVeiw::new(*radius));
        }
        if let Some(effects) = &template.provides {
            effects
                .iter()
                .for_each(|(provides, n)| match provides.as_str() {
                    "Healing" => commands.add_component(entity, ProvidesHealing { amount: *n }),
                    "MagicEye" => commands.add_component(entity, ProvidesWiderView { amount: *n }),
                    "ShockWave" => commands.add_component(entity, ProvidesSurroundingAttack{ amount: *n}),
                    "RocketPunch" => commands.add_component(entity, ProvidesLinerAttack{ amount: *n }),
                    _ => {
                        println!("Warning: we don't know how to provide {}", provides);
                    }
                });
        }
        if let Some(damage) = &template.base_damage {
            commands.add_component(entity, Damage(*damage));
            if template.entity_type == EntityType::Item {
                commands.add_component(entity, Weapon {});
            }
        }
        if let Some(defense) = &template.base_defense {
            commands.add_component(entity, Defense(*defense));
            if template.entity_type == EntityType::Item {
                commands.add_component(entity, Armor);
            }
        }
        if template.heat_seeking {
            commands.add_component(entity, HeatSeeking{ saw_player: false });
        }
        if let Some(attack_left_frames) = &template.attack_left_frames{
            if let Some(attack_right_frames) = &template.attack_right_frames {
                if let Some(attack_up_frames) = &template.attack_up_frames {
                    if let Some(attack_down_frames) = &template.attack_down_frames {
                        commands.add_component(entity, AttackFrames {
                            left: SmallVec::from_vec(attack_left_frames.clone()),
                            right: SmallVec::from_vec(attack_right_frames.clone()),
                            up: SmallVec::from_vec(attack_up_frames.clone()),
                            down: SmallVec::from_vec(attack_down_frames.clone()),
                        });
                    }
                }
            }
        }

        //check special tag
        if template.special_tag == Some(SpecialTag::Boss) {
            commands.add_component(entity, Boss);
        }
    }
}
