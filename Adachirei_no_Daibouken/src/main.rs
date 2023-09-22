#![warn(clippy::pedantic)]

mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod turn_state;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 3;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 3;
    pub const ANIME_FRAME_DURATION: f32 = 75.0;
    pub const MAX_NUM_FRAMES: usize = 10;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    input_systems: Schedule,
    player_systems: Schedule,
    monster_systems: Schedule,
    effect_anime_systems: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new_level(&mut rng, 0);
        //spawn entities
        spawn_player(&mut ecs, map_builder.player_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;
        let unique_item_start = map_builder.monster_spawns[0];
        spawn_special_tagged(&mut ecs, unique_item_start, SpecialTag::UniqueEye);
        spawn_level(&mut ecs, &mut rng, 0, &map_builder.monster_spawns[1..]); // make sure that unique item does't overlap other entities

        //add resources
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::MainMenue);
        resources.insert(map_builder.theme);
        resources.insert(0.0 as f32);

        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
            effect_anime_systems: build_effect_anime_scheduler(),
        }
    }

    fn main_menue(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(3);

        //Story
        ctx.print_centered(1, "Story");
        ctx.print_color_centered(3, ORANGE, BLACK, "Humanoid Character Interface 3 Prototype 0,");
        ctx.print_color_centered(4, ORANGE, BLACK, "codenamed Adachi Rei is starting up...");
        ctx.print_centered(6, "She woke up receiving an emergency signal during a maintenance.");
        ctx.print_centered(8, "It seems that a large number of robots are attacking the laboratory,");
        ctx.print_centered(10, "and the doctor is about to be taken away by a giant robot.");
        ctx.print_centered(12, "'War Robot Prototype' was written on his shoulder.");
        ctx.print_centered(14, "Let's hurry and go to rescue him");

        ctx.print_color_centered(25, YELLOW, BLACK, "Display an illust");

        //TODO: Display an illust

        ctx.print_color_centered(32, GREEN, BLACK, "(P) Play Game");

        if Some(VirtualKeyCode::P) == ctx.key {
            self.reset_game();
        }
    }

    fn reset_game(&mut self) {
        self.ecs = World::default();
        self.resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new_level(&mut rng, 0);
        spawn_player(&mut self.ecs, map_builder.player_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;
        let unique_item_start = map_builder.monster_spawns[0];
        spawn_special_tagged(&mut self.ecs, unique_item_start, SpecialTag::UniqueEye);
        spawn_level(&mut self.ecs, &mut rng, 0, &map_builder.monster_spawns[1..]); // make sure that unique item does't overlap other entities
        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);
        self.resources.insert(0.0 as f32);
    }

    fn gameover(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(3);
        ctx.print_color_centered(2, RED, BLACK, "Your quest has ended.");
        ctx.print_centered(4, "Slain by a robot");
        ctx.print_centered(8, "The doctor was taken away, and her battery was broken.");
        ctx.print_centered(12, "After a while, her consciousness completely ceased.");

        ctx.print_color_centered(20, YELLOW, BLACK, "Display an illust");

        //TODO: Display an illust

        ctx.print_color_centered(32, GREEN, BLACK, "(M) Go Back Main Menue");

        if let Some(VirtualKeyCode::M) = ctx.key {
            self.resources.insert(TurnState::MainMenue);
        }
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(3);
        ctx.print_color_centered(2, BLUE, BLACK, "You have won!");
        ctx.print_centered(8, "After a fierce battle,");
        ctx.print_centered(12, "she defeated the giant robot and rescued the doctor.");

        ctx.print_color_centered(20, YELLOW, BLACK, "Display an illust");

        //TODO: Display an illust

        ctx.print_color_centered(32, GREEN, BLACK, "(M) Go Back Main Menue");

        if let Some(VirtualKeyCode::M) = ctx.key {
            self.resources.insert(TurnState::MainMenue);
        }
    }

    fn world_map(&mut self, ctx: &mut BTerm) {
        //Display world map, and player
        {
            ctx.set_active_console(1);
            let map = self.resources.get::<Map>().unwrap();
            let map_theme = self.resources.get::<Box<dyn MapTheme>>().unwrap();
            for y in 0..SCREEN_HEIGHT {
                for x in 0..SCREEN_WIDTH {
                    let idx = map_idx(x, y);
                    if !map.revealed_tiles[idx] {
                        continue;
                    }
                    let glyph = map_theme.tile_to_render(map.tiles[idx]);
                    ctx.set(x, y, WHITE, BLACK, glyph);
                }
            }
            <&Point>::query()
                .filter(component::<Player>())
                .iter(&self.ecs)
                .for_each(|pos| {
                    ctx.set(pos.x, pos.y, WHITE, BLACK, to_cp437('@'));
                });
        }

        ctx.set_active_console(3);
        ctx.print_centered(1, "(M) Quit Display The WorldMap");

        if Some(VirtualKeyCode::M) == ctx.key {
            self.resources.insert(TurnState::AwaitingInput);
        }
    }

    fn advance_level(&mut self) {
        let player_entity = *<Entity>::query()
            .filter(component::<Player>())
            .iter(&self.ecs)
            .nth(0)
            .unwrap();

        //entities to keep
        use std::collections::HashSet;
        let mut entities_to_keep = HashSet::new();
        entities_to_keep.insert(player_entity);
        <(Entity, &Carried)>::query()
            .iter(&self.ecs)
            .filter(|(_, carried)| carried.0 == player_entity)
            .for_each(|(e, _)| {
                entities_to_keep.insert(*e);
            });

        //Remove the other entities
        let mut cb = CommandBuffer::new(&mut self.ecs);
        for e in Entity::query().iter(&self.ecs) {
            if !entities_to_keep.contains(e) {
                cb.remove(*e);
            }
        }
        //When you use a CommandBuffer outside of a system, you have to apply it to the world by calling flush()
        cb.flush(&mut self.ecs);

        //Set the Field of View to Dirty
        <&mut FieldOfVeiw>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|fov| fov.is_dirty = true);

        //Create a new map, and place the player in a new map.
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);
        let mut map_level = 0;
        <(&mut Player, &mut Point)>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|(player, pos)| {
                player.map_level += 1;
                map_level = player.map_level;
                map_builder = MapBuilder::new_level(&mut rng, map_level as usize);
                *pos = map_builder.player_start;
            });

        //add other entities
        let unique_item_start = map_builder.monster_spawns[0];
        if map_level == 2 {
            spawn_special_tagged(&mut self.ecs, map_builder.amulet_start, SpecialTag::Boss);
            spawn_special_tagged(&mut self.ecs, unique_item_start, SpecialTag::UniqueWeapon);
        } else {
            let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
            map_builder.map.tiles[exit_idx] = TileType::Exit;

            spawn_special_tagged(&mut self.ecs, unique_item_start, SpecialTag::UniqueArmor);
        }
        spawn_level(
            &mut self.ecs,
            &mut rng,
            map_level as usize,
            &map_builder.monster_spawns[1..],
        );

        //add resources
        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();
        ctx.set_active_console(3);
        ctx.cls();
        ctx.set_active_console(4);
        ctx.cls();

        //take input and elasped time from last tick
        self.resources.insert(ctx.frame_time_ms);
        self.resources.insert(ctx.key);
        ctx.set_active_console(0);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));
        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        match current_state {
            TurnState::AwaitingInput => {
                self.input_systems
                    .execute(&mut self.ecs, &mut self.resources);
            },
            TurnState::PlayerTurn => {
                self.player_systems
                    .execute(&mut self.ecs, &mut self.resources);
            },
            TurnState::MonsterTurn => {
                self.monster_systems
                    .execute(&mut self.ecs, &mut self.resources);
            },
            TurnState::EffectAnime => {
                self.effect_anime_systems
                    .execute(&mut self.ecs, &mut self.resources);
            },
            TurnState::MainMenue => self.main_menue(ctx),
            TurnState::GameOver => self.gameover(ctx),
            TurnState::Victory => self.victory(ctx),
            TurnState::NextLevel => self.advance_level(),
            TurnState::WorldMap => self.world_map(ctx),
        }
        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    std::env::set_var("RUST_BACKTRACE", "1");

    //starting sound
    use std::fs::File;
    use std::io::BufReader;
    use rodio::{Decoder, OutputStream, Sink};
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open("resources/pc_calculation.wav").unwrap());
    let source = Decoder::new(file).unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(source);
    sink.sleep_until_end();

    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(60, 60)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(SCREEN_WIDTH, SCREEN_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH * 3, DISPLAY_HEIGHT * 3, "terminal8x8.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .build()?;

    main_loop(context, State::new())
}
