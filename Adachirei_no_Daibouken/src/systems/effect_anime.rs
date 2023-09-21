use crate::prelude::*;

#[system]
#[write_component(EffectMotion)]
#[read_component(TurnBeforeEffects)]
pub fn effect_anime(
        ecs: &mut SubWorld,
        #[resource] elasped_time: &f32,
        #[resource] turn: &mut TurnState,
        #[resource] camera: &Camera,
        commands: &mut CommandBuffer,
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    let offset = Point::new(camera.left_x, camera.top_y);
    let mut no_effects = true;
    <(Entity, &mut EffectMotion)>::query()
        .iter_mut(ecs)
        .for_each(|(message_entity, effect_motion)| {
            no_effects= false;

            draw_batch.set(
                effect_motion.position - offset,
                ColorPair::new(WHITE, BLACK),
                effect_motion.anime_frames[effect_motion.current_frame]
            );
            //adjust current frame
            effect_motion.elasped_time_from_last_frame += elasped_time;
            if effect_motion.elasped_time_from_last_frame > ANIME_FRAME_DURATION {
                effect_motion.current_frame += 1;
                if effect_motion.current_frame >= effect_motion.anime_frames.len() {
                    commands.remove(*message_entity);
                }
                effect_motion.elasped_time_from_last_frame = 0.0;
            }
        }
    );
    //EffectMotionが無かったら、TurnStateをBasice Game Loopに戻す
    if no_effects {
        let mut tur_before_effects = TurnState::MainMenue;
        <(Entity, &TurnBeforeEffects)>::query()
            .iter(ecs)
            .for_each(|(entity, turn)| {
                tur_before_effects = turn.0;
                commands.remove(*entity);
            }
        );

        *turn = match tur_before_effects {
            TurnState::AwaitingInput => TurnState::PlayerTurn,
            TurnState::PlayerTurn => TurnState::MonsterTurn,
            TurnState::MonsterTurn => TurnState::AwaitingInput,
            _ => tur_before_effects
        };
    }

    draw_batch.submit(10100).expect("Batch Error");
}