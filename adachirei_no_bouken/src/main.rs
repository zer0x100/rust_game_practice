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
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);
        //spawn entities
        spawn_player(&mut ecs, map_builder.player_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;
        let unique_item_start = rng.random_slice_entry(&map_builder.monster_spawns).unwrap();
        spawn_special_tagged(&mut ecs, *unique_item_start, SpecialTag::UniqueEye);
        spawn_level(&mut ecs, &mut rng, 0, &map_builder.monster_spawns);

        //add resources
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::AwaitingInput);
        resources.insert(map_builder.theme);

        Self {
            ecs,
            resources,
            input_systems: build_input_scheduler(),
            player_systems: build_player_scheduler(),
            monster_systems: build_monster_scheduler(),
        }
    }

    fn reset_game(&mut self) {
        self.ecs = World::default();
        self.resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut self.ecs, map_builder.player_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;
        let unique_item_start = rng.random_slice_entry(&map_builder.monster_spawns).unwrap();
        spawn_special_tagged(&mut self.ecs, *unique_item_start, SpecialTag::UniqueEye);
        spawn_level(&mut self.ecs, &mut rng, 0, &map_builder.monster_spawns);
        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.theme);
    }

    fn gameover(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, RED, BLACK, "Your quest has ended.");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "Slain by a monster, your hero's journey has come to a premature end.",
        );
        ctx.print_color_centered(
            5,
            WHITE,
            BLACK,
            "The Amulet of Yala remains unclaimed, and your home town is not saved.",
        );
        ctx.print_color_centered(
            8,
            YELLOW,
            BLACK,
            "Don't worry, you can always try again with a new hero.",
        );
        ctx.print_color_centered(9, GREEN, BLACK, "Press 1 to play again.");

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game();
        }
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(2, GREEN, BLACK, "You have won!");
        ctx.print_color_centered(
            4,
            WHITE,
            BLACK,
            "You put on the Amulet of Yala and feel its power course through \
        your veins.",
        );
        ctx.print_color_centered(
            5,
            WHITE,
            BLACK,
            "Your town is saved, and you can return to your normal life.",
        );
        ctx.print_color_centered(
            7,
            GREEN,
            BLACK,
            "Press 1 to play again.",
        );

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game();
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
                    if !map.revealed_tiles[idx] { continue; }
                    let glyph = map_theme.tile_to_render(map.tiles[idx]);
                    ctx.set(x, y, WHITE, BLACK, glyph);
                }
            }
            <&Point>::query().filter(component::<Player>())
                .iter(&self.ecs)
                .for_each(|pos| {
                    ctx.set(
                        pos.x,
                        pos.y,
                        WHITE,
                        BLACK,
                        to_cp437('@'),
                    );
                }
            );
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

        //Create a new map
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);

        //Place the player in a new map.
        let mut map_level = 0;
        <(&mut Player, &mut Point)>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|(player, pos)| {
                player.map_level += 1;
                map_level = player.map_level;
                *pos = map_builder.player_start;
            });

        //add other entities
        let unique_item_start = rng.random_slice_entry(&map_builder.monster_spawns).unwrap();
        if map_level == 2 {
            spawn_special_tagged(&mut self.ecs, map_builder.amulet_start, SpecialTag::Boss);
            spawn_special_tagged(&mut self.ecs, *unique_item_start, SpecialTag::UniqueWeapon);
        } else {
            let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
            map_builder.map.tiles[exit_idx] = TileType::Exit;

            spawn_special_tagged(&mut self.ecs, *unique_item_start, SpecialTag::UniqueArmor);
        }
        spawn_level(
            &mut self.ecs,
            &mut rng,
            map_level as usize,
            &map_builder.monster_spawns,
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

        self.resources.insert(ctx.key);
        ctx.set_active_console(0);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));
        let current_state = self.resources.get::<TurnState>().unwrap().clone();
        match current_state {
            TurnState::AwaitingInput => self
                .input_systems
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => {
                self.player_systems
                    .execute(&mut self.ecs, &mut self.resources);
            }
            TurnState::MonsterTurn => self
                .monster_systems
                .execute(&mut self.ecs, &mut self.resources),
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

    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(55, 55)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(SCREEN_WIDTH, SCREEN_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH * 3, DISPLAY_HEIGHT * 3, "terminal8x8.png")
        .build()?;

    main_loop(context, State::new())
}
