use crate::prelude::*;

#[system]
#[read_component(Player)]
#[write_component(Point)]
pub fn player_input(ecs: &mut SubWorld,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
) {
    if let Some(key) = *key {
        let mut players = <&mut Point>::query().filter(component::<Player>());
        let player_pos = players
            .iter_mut(ecs)
            .nth(0)
            .unwrap();
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::zero(),
        };
        let new_position = *player_pos + delta;

        if map.can_enter_tile(new_position) {
            *player_pos = new_position;
            camera.on_player_move(new_position);
        }
    }
}