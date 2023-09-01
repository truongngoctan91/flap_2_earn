use bracket_lib::prelude::*;
use rand::Rng;
use std::{thread, time};
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const FRAME_DURATION: f32 = 75.0;
const ENEMY_NO: i32 = 1; // level game depends on this
const DRAGON_FRAMES: [u16; 6] = [64, 1, 2, 3, 2, 1];
const GIFT_NO: u8 = 5;

fn dist(x1: i32, y1: f32, x2: i32, y2: i32) -> f32 {
    let a = x1 - x2;
    let b = y1 as i32 - y2;
    let d2: f32 = (a * a + b * b) as f32;

    d2.sqrt()
}

fn get_rnd_x() -> i32 {
    rand::thread_rng().gen_range(SCREEN_WIDTH, SCREEN_WIDTH + 20)
}
fn get_rnd_y() -> i32 {
    rand::thread_rng().gen_range(1, SCREEN_HEIGHT - 1)
}

enum GameMode {
    Menu,
    Playing,
    End,
    Question,
}

struct Player {
    x: i32,
    y: f32,
    velocity: f32,
    frame: usize,
}

struct Enemy {
    x: i32,
    y: i32,
    active: bool,
}

struct Gift {
    x: i32,
    y: i32,
    active: bool,
}

impl Enemy {
    fn new(x: i32, y: i32) -> Self {
        Enemy { x, y, active: true }
    }
    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(self.x, self.y, WHITE, NAVY, 179);
    }

    fn move_(&mut self) {
        self.x -= 1;
    }

    fn hit(&mut self, player: &Player) -> bool {
        if dist(player.x, player.y, self.x, self.y) < 2.0 {
            return true;
        }
        return false;
    }
}

impl Gift {
    fn new(x: i32, y: i32) -> Self {
        Gift { x, y, active: true }
    }
    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set(self.x, self.y, WHITE, NAVY, 35);
    }

    fn move_(&mut self) {
        self.x -= 1;
    }

    fn hit(&mut self, player: &Player) -> bool {
        if dist(player.x, player.y, self.x, self.y) < 1.5 {
            return true;
        }
        return false;
    }
}

impl Player {
    fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y: y as f32,
            velocity: 0.0,
            frame: 0,
        }
    }
    fn render(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_fancy(
            PointF::new(0.0, self.y),
            1,
            Degrees::new(0.0),
            PointF::new(2.0, 2.0),
            WHITE,
            NAVY,
            DRAGON_FRAMES[self.frame],
        );
        ctx.set_active_console(0);
    }

    fn gravity_and_move(&mut self) {
        if self.velocity < 2.0 {
            self.velocity += 0.2;
        }
        self.y += self.velocity;
        /*
        self.x += 1;
        if self.x > SCREEN_WIDTH {
            self.x = 0;
        }
        */
        if self.y < 0.0 {
            self.y = 0.0;
        }
        self.frame += 1;
        self.frame = self.frame % 6; // % is modulus - remainder
    }
    fn flap(&mut self) {
        if self.velocity > 0.0 {
            self.velocity -= 2.0;
        }
    }
}

struct State {
    mode: GameMode,
    player: Player,
    enemy_vec: Vec<Enemy>,
    frame_time: f32,
    active_enemies: i32,
    gifts: Vec<Gift>,
    active_gift: i32,
    score: i32,
    life: u32,
    level: u32,
    nb_die: usize,
    rank: u32,
}

impl State {
    fn new() -> Self {
        State {
            frame_time: 0.0,
            mode: GameMode::Menu,
            player: Player::new(5, 25),
            enemy_vec: Vec::with_capacity(ENEMY_NO as usize),
            active_enemies: 0,
            gifts: Vec::with_capacity(GIFT_NO as usize),
            active_gift: 0,
            score: 0,
            life: 3,
            level: 1,
            nb_die: 0,
            rank: 1,
        }
    }

    fn hit_enemy(&mut self) -> bool {
        for enemy in self.enemy_vec.iter_mut() {
            if enemy.active && enemy.hit(&self.player) {
                return true;
            }
        }

        return false;
    }

    //     fn hit_gift(&mut self) -> bool {

    //     return false;
    // }
    fn play(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
            for enemy in self.enemy_vec.iter_mut() {
                if enemy.active {
                    enemy.move_();
                    if enemy.x <= 0 {
                        enemy.active = false;
                        self.active_enemies -= 1;
                    }
                }
            }
            for gift in self.gifts.iter_mut() {
                if gift.active {
                    gift.move_();
                    if gift.x <= 0 {
                        gift.active = false;
                        self.active_gift -= 1;
                    }
                }
            }
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        let mut x = 0;
        let y = SCREEN_HEIGHT - 1;
        while x < SCREEN_WIDTH {
            let mut symbol1 = '|';
            let mut symbol2 = 'X';

            if self.frame_time as i32 % 2 == 0 {
                symbol1 = 'X';
                symbol2 = '|';
            }

            if x % 2 == 0 {
                ctx.set(x, y, GREEN, BLACK, to_cp437(symbol1));
            } else {
                ctx.set(x, y, GREEN, BLACK, to_cp437(symbol2));
            }
            x += 1;
        }
        self.player.render(ctx);
        if self.active_enemies == 0 {
            self.enemy_vec.clear();
            let active_enemies = rand::thread_rng().gen_range(ENEMY_NO / 2, ENEMY_NO);
            for n in 0..active_enemies {
                self.enemy_vec.push(Enemy::new(get_rnd_x(), get_rnd_y()));
                self.active_enemies += 1;
            }
        }

        for enemy in self.enemy_vec.iter_mut() {
            if enemy.active {
                enemy.render(ctx);
            }
        }
        if self.active_gift == 0 {
            self.gifts.clear();
            let active_gift = rand::thread_rng().gen_range(GIFT_NO / 2, GIFT_NO);
            for n in 0..active_gift {
                self.gifts.push(Gift::new(get_rnd_x(), get_rnd_y()));
                self.active_gift += 1;
            }
        }
        for gift in self.gifts.iter_mut() {
            if gift.active {
                gift.render(ctx);
            }
        }
        ctx.print(0, 0, "Press SPACE to flap.");
        ctx.print(0, 1, &format!("Score: {}", self.score));
        ctx.print(0, 2, &format!("Life: {}", self.life));
        ctx.print(0, 3, &format!("Level: {}", self.level));
        ctx.print(
            SCREEN_WIDTH - 50,
            0,
            &format!("Altitude: {}", SCREEN_HEIGHT - self.player.y as i32),
        );
        if self.player.y > SCREEN_HEIGHT as f32 || self.hit_enemy() {
            let pause_sec = time::Duration::from_millis(2000);
            thread::sleep(pause_sec);
            self.life = self.life - 1;
            self.mode = GameMode::Question;
            self.nb_die +=1;
        }

        for gift in self.gifts.iter_mut() {
            if gift.active && gift.hit(&self.player) {
                self.score += 1;
            }
        }
    }

    fn question(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.cls_bg(NAVY);
        let question: Vec<[&str; 6]> = vec![
            [
                "When was the completed Pokadot mainet launched",
                "A: 2020",
                "B: 2021",
                "C: 2022",
                "D: 2023",
                "B",
            ],
            [
                "Who is a founder of Pokadot",
                "A:  Elon Musk",
                "B: Donal Trump",
                "C: Vitalik Buterin",
                "D: Gavin James Wood",
                "D",
            ],
            [
                "What is the capital of France?",
                "A: London",
                "B: Paris",
                "C: Berlin",
                "D: Hanoi",
                "B",
            ],
            [
                "What is the capital of Vietnam?",
                "A: London",
                "B: Paris",
                "C: Berlin",
                "D: Hanoi",
                "D",
            ],
        ];
        let index_question = self.nb_die -1;
        let length_of_questions = question.len();
        if self.nb_die > length_of_questions {
               if self.life == 0 {
                    self.mode = GameMode::End
                } else {
                    self.playing_continue()
                }

        }
        else {

             ctx.print(20,5, question[index_question][0]);
        ctx.print(30,8, question[index_question][1]);
        ctx.print(30,9, question[index_question][2]);
        ctx.print(30,10, question[index_question][3]);
        ctx.print(30,11, question[index_question][4]);
        let expected = question[index_question][5];
        let mut answered: String = String::from("5");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::A => {
                    answered = String::from("A");
                }
                VirtualKeyCode::B => {
                    answered = String::from("B");
                }
                VirtualKeyCode::C => {
                    answered = String::from("C");
                }
                VirtualKeyCode::D => {
                    answered = String::from("D");
                }
                _ => {
                    answered = String::from("null");
                }
            }

            if expected == answered {
                self.life += 1;
                self.playing_continue()
            } else {
                if self.life == 0 {
                    self.mode = GameMode::End
                } else {
                    self.playing_continue()
                }
            }
        }

        }



    }
    fn restart(&mut self, _ctx: &mut BTerm) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.active_enemies = 0;
        self.enemy_vec.clear();
        self.mode = GameMode::Playing;
        self.score = 0;
        self.life = 3;
        self.level = 1;
    }

    fn playing_continue(&mut self) {
        self.player = Player::new(5, 25);
        self.frame_time = 0.0;
        self.active_enemies = 0;
        self.enemy_vec.clear();
        self.mode = GameMode::Playing;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_color_centered(5, YELLOW, BLACK, "Welcome to Flappy Dragon");
        ctx.print_color_centered(8, CYAN, BLACK, "(P) Play Game");
        ctx.print_color_centered(9, CYAN, BLACK, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(ctx),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
    fn dead(&mut self, ctx: &mut BTerm) {
        let mut rewards = 0;
        if self.rank == 1 {
            rewards = 1;
        }

        ctx.cls();
        ctx.print_centered(5, "You are dead!");

         ctx.print_centered(8, &format!("Score: {}", self.score));
         ctx.print_centered(10, &format!("You will receive : {} $DOT", rewards));
        ctx.print_centered(12, " (P) Play Again");
        ctx.print_centered(13, "(Q) Quit Game");

        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(ctx),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::Menu => self.main_menu(ctx),
            GameMode::End => self.dead(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::Question => self.question(ctx),
        }
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_font("../resources/flappy32.png", 32, 32)
        .with_simple_console(SCREEN_WIDTH, SCREEN_HEIGHT, "../resources/flappy32.png")
        .with_fancy_console(SCREEN_WIDTH, SCREEN_HEIGHT, "../resources/flappy32.png")
        .with_title("Flap_2_earn")
        .with_tile_dimensions(16, 16)
        .build()?;

    main_loop(context, State::new())
}
