#![warn(clippy::pedantic)]

use bracket_lib::prelude::*;

const SCREEN_WIDTH: i32 = 25;
const SCREEN_HEIGHT: i32 = 25;
const KARAAGE_DURATION: f32 = 1500.0;
const PLAYER_SIZE: i32 = 3;//the size of collision detection
const PLAYER_Y: i32 = SCREEN_HEIGHT - 2;

enum GameMode {
    Menu,
    Playing,
    End,
}

struct Player {
    x: i32,
    right: bool,
}

impl Player {
    fn new(x: i32) -> Self {
        Self{ x, right: true }
    }
    
    fn render(&self, ctx: &mut BTerm) {
        ctx.set_active_console(1);
        ctx.cls();
        let glyph = if self.right { 1 } else { 2 };
        ctx.set_fancy(
            PointF::new(self.x as f32, PLAYER_Y as f32),
            1,
            Degrees::new(0.0),
            PointF::new(2.0, 2.0),
            WHITE,
            BLACK,
            glyph,
        );
        ctx.set_active_console(0);
    }

    fn move_right(&mut self, v: i32) {
        if v > 0 { self. right = true; }
        else if v < 0 { self.right = false; }

        self.x += v;
        if self.x < 0 {
            self.x = 0;     
        }
        if self.x >= SCREEN_WIDTH {
            self.x = SCREEN_WIDTH - 1;
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Karaage {
    x: f32,
    y: f32,
    velocity_x: f32,
    velocity_y: f32,
}

impl Karaage {
    fn new(x: f32, y: f32, velocity_x: f32, velocity_y: f32) -> Self {
        Self {
            x,
            y,
            velocity_x,
            velocity_y,
        }
    }

    fn fall(&mut self) {
        self.x += self.velocity_x;
        self.y += self.velocity_y;
    }

    fn hit_karaage(&self, player: &Player) -> bool {
        let half_size = PLAYER_SIZE / 2;
        let x_match = (player.x - self.x as i32).abs() <= half_size;
        let y_match = self.y as i32 == PLAYER_Y;
        x_match && y_match
    }

    fn render(&self, ctx: &mut BTerm) {
        ctx.set(
            self.x as i32,
            self.y as i32,
            WHITE,
            BLACK,
            3,
        )
    }
}

struct State{
    mode: GameMode,
    player: Player,
    karaages: Vec<Karaage>,
    score: i32,
    frame_time: f32,
}

impl State {
    fn new() -> Self {
        Self {
            mode: GameMode::Menu,
            player: Player::new(SCREEN_WIDTH/2),
            karaages: Vec::new(),
            score: 0,
            frame_time: 0.0,
        }
    }

    fn restart(&mut self) {
        self.player = Player::new(SCREEN_WIDTH/2);
        self.mode = GameMode::Playing;
        self.karaages = Vec::new();
        self.score = 0;
        self.frame_time = 0.0;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_color_centered(5, ORANGE, WHITE, "Karaage Aaaaaaa");
        ctx.print_color_centered(8, CYAN, BLACK, "(P) Play Game");
        ctx.print_color_centered(9, CYAN, BLACK, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        //slide player
        if let Some(key) = ctx.key {
            self.player.move_right(
                match key {
                    VirtualKeyCode::Right => 1,
                    VirtualKeyCode::Left => -1,
                    _ => 0,
                }
            )
        }

        //fall karaages
        for karaage in self.karaages.iter_mut() {
            karaage.fall();
            //x axis collision check
            if (karaage.x as i32) < 0 {
                karaage.x = 0.0;
                karaage.velocity_x = -karaage.velocity_x;
            }
            if karaage.x as i32 > SCREEN_WIDTH-1 {
                karaage.x = (SCREEN_WIDTH - 1) as f32;
                karaage.velocity_x = -karaage.velocity_x;
            }
        }

        //pop out karaage
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > KARAAGE_DURATION {
            self.frame_time = 0.0;
            self.karaage_pop_out();
        }

        //rendering
        ctx.cls();
        self.render_karaages_and_ground(ctx);
        self.player.render(ctx);
        ctx.print(0, 0, "Cursor keys to slide");
        ctx.print_color(0, 1, ORANGE, BLACK, &format!("{} karaagees", self.score));

        //check score
        self.karaages
            .iter()
            .for_each(|karaage| {
                if karaage.hit_karaage(&self.player) {
                    self.score += 1;
                }
            }
        );

        //remove catched karaages
        self.karaages = self.karaages
            .iter()
            .filter(|karaage| !karaage.hit_karaage(&self.player))
            .map(|karaage| *karaage)
            .collect();


        //check gameover
        for karaage in self.karaages.iter() {
            if karaage.y as i32 > SCREEN_HEIGHT {
                self.mode = GameMode::End;
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_color_centered(5, ORANGERED, WHITE, "Karaage fell to the floor");
        ctx.print_color_centered(6, YELLOW, BLACK, &format!("You earned {} karaagees", self.score));
        ctx.print_color_centered(8, CYAN, BLACK, "(P) Play Game");
        ctx.print_color_centered(9, CYAN, BLACK, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn karaage_pop_out(&mut self) {
        let mut rng = RandomNumberGenerator::new();
        self.karaages.push(
            Karaage::new(
                rng.range(0, SCREEN_WIDTH) as f32,
                0.0,
                rng.range(-0.2, 0.2),
                0.2,
            )
        );
    }

    fn render_karaages_and_ground(&self, ctx: &mut BTerm) {
        //Draw the ground
        for x in 0..SCREEN_WIDTH {
            ctx.set(
                x,
                SCREEN_HEIGHT-1,
                WHITE,
                BLACK,
                35,
            )
        }

        //Draw karaages
        for karaage in self.karaages.iter() {
            karaage.render(ctx);
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::End => self.dead(ctx),
        }
    }
}

fn main() -> BError {
    std::env::set_var("RUST_BACKTRACE", "1");

    let context = BTermBuilder::new()
        .with_title("Karaage Aaaaaaa")
        .with_font("../resources/karaage_font.png", 32, 32)
        .with_simple_console(SCREEN_WIDTH, SCREEN_HEIGHT, "../resources/karaage_font.png")
        .with_fancy_console(SCREEN_WIDTH, SCREEN_HEIGHT, "../resources/karaage_font.png")
        .with_tile_dimensions(16, 16)
        .with_dimensions(SCREEN_WIDTH*2, SCREEN_HEIGHT*2)
        .build()?;

    main_loop(context, State::new())
}