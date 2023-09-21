use crate::prelude::*;

#[system]
#[write_component(EffectMotion)]
pub fn effect_anime(
        ecs: &mut SubWorld,
        #[resource] current_time: &f32,
        #[resource] turn: &mut TurnState,
        #[resource] camera: &Camera,
        commands: &mut CommandBuffer,
) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    let offset = Point::new(camera.left_x, camera.top_y);
    let mut no_effects = true;
    let mut prev_turn = TurnState::PlayerTurn;
    <(Entity, &mut EffectMotion)>::query()
        .iter_mut(ecs)
        .for_each(|(message_entity, effect_motion)| {
            no_effects= false;
            prev_turn = effect_motion.prev_turn;

            draw_batch.set(
                effect_motion.position - offset,
                ColorPair::new(WHITE, BLACK),
                effect_motion.anime_frames[effect_motion.current_frame]
            );
            //adjust current frame
            if *current_time >= effect_motion.last_frame_time + ANIME_FRAME_DURATION {
                effect_motion.current_frame += 1;
                if effect_motion.current_frame >= effect_motion.anime_frames.len() {
                    commands.remove(*message_entity);
                }
                effect_motion.last_frame_time = *current_time;
            }
        }
    );
    //EffectMotionが無かったら、TurnStateをBasice Game Loopに戻す
    if no_effects {
        *turn = match prev_turn {
            TurnState::AwaitingInput => TurnState::PlayerTurn,
            TurnState::PlayerTurn => TurnState::MonsterTurn,
            TurnState::MonsterTurn => TurnState::AwaitingInput,
            _ => prev_turn,
        };
    }

    draw_batch.submit(10100).expect("Batch Error");
}