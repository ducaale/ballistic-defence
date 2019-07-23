use ggez::event::{self, MouseButton};
use ggez::graphics;
use ggez::{Context, GameResult};
use ggez::timer;
use cgmath::Point2;
use cgmath::InnerSpace;
use rand::Rng;
use std::env;
use std::path;

mod explosion;
mod missile;
mod assets;

use explosion::Explosion;
use missile::Missile;
use assets::Assets;

struct MainState {
    frames: usize,
    assets: Assets,
    cursor: Point2<f32>,
    missiles: Vec<Missile>,
    explosions: Vec<Explosion>,
    missiles_intercepted: u32,
    missiles_missed: u32
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState {
            frames: 0,
            assets: Assets::new(ctx)?,
            cursor: Point2::new(0.0, 0.0),
            missiles: vec![],
            explosions: vec![],
            missiles_intercepted: 0,
            missiles_missed: 0
        };
        Ok(s)
    }

    fn draw_cursor(&self, ctx: &mut Context) -> GameResult {
        ggez::input::mouse::set_cursor_hidden(ctx, true);
        let image = &self.assets.cursor_image;
        let drawparams = graphics::DrawParam::new()
            .dest(self.cursor)
            .offset(Point2::new(0.5, 0.5));
        graphics::draw(ctx, image, drawparams)?;

        Ok(())
    }

    fn draw_score(&self, ctx: &mut Context) -> GameResult {
        let intercepted_dest = Point2::new(10.0, 10.0);
        let missed_dest = Point2::new(600.0, 10.0);

        let intercepted_str = format!("INTERCEPTED: {}", self.missiles_intercepted);
        let missed_str = format!("MISSED: {}", self.missiles_missed);

        let intercepted_display = graphics::Text::new((intercepted_str, self.assets.font, 32.0));
        let missed_display = graphics::Text::new((missed_str, self.assets.font, 32.0));

        graphics::draw(ctx, &intercepted_display, (intercepted_dest, 0.0, graphics::WHITE))?;
        graphics::draw(ctx, &missed_display, (missed_dest, 0.0, graphics::WHITE))?;

        Ok(())
    }

    fn handle_collisions(&mut self) {
        for explosion in self.explosions.iter_mut() {
            for missile in self.missiles.iter_mut() {
                if (explosion.position - missile.position).magnitude() < explosion.radius
                    && !missile.is_invincible
                {
                    missile.is_alive = false;
                }
            }
        }
    }

    fn explode_dead_missiles(&mut self) {
        for missile in self.missiles.iter() {
            if !missile.is_alive {
                missile.explode(&mut self.explosions)
            }
        }
    }

    fn update_score(&mut self) {
        for missile in self.missiles.iter() {
            if !missile.is_alive && !missile.is_invincible {
                if missile.did_hit_target() {
                    self.missiles_missed += 1;
                }
                else {
                    self.missiles_intercepted += 1;
                }
            }
        }
    }

    fn remove_dead_entites(&mut self) {
        self.missiles.retain(|missile| missile.is_alive);
        self.explosions.retain(|explosion| explosion.is_alive);
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        const DESIRED_FPS: u32 = 60;
        while timer::check_update_time(ctx, DESIRED_FPS) {
            let elapsed = 1.0 / (DESIRED_FPS as f32);

            self.missiles.iter_mut().for_each(|missile| missile.update(elapsed));
            self.explosions.iter_mut().for_each(|explosion| explosion.update(elapsed));

            self.handle_collisions();
            self.explode_dead_missiles();
            self.update_score();
            self.remove_dead_entites();

            if self.frames % 100 == 0 {
                let mut rng = rand::thread_rng();
                let origin_x = rng.gen_range(0.0, 800.0);
                let target_x = rng.gen_range(0.0, 800.0);
                self.missiles.push(Missile::new(
                    100.0,
                    Point2::new(origin_x, 0.0),
                    Point2::new(target_x, 600.0),
                    false,
                ));
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());

        for missile in self.missiles.iter() {
            missile.draw(ctx)?;
        }
        for explosion in self.explosions.iter() {
            if (self.frames % 4) != 0 {
                explosion.draw(ctx)?;
            }
        }
        self.draw_cursor(ctx)?;
        self.draw_score(ctx)?;
        graphics::present(ctx)?;

        self.frames += 1;
        if (self.frames % 100) == 0 {
            println!("FPS: {}", ggez::timer::fps(ctx));
        }

        Ok(())
    }

    fn mouse_motion_event(&mut self, _ctx: &mut Context, x: f32, y: f32, _dx: f32, _dy: f32) {
        self.cursor.x = x;
        self.cursor.y = y;
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        self.missiles.push(Missile::new(
            500.0,
            Point2::new(400.0, 550.0),
            self.cursor,
            true,
        ))
    }
}

fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let (ctx, event_loop) = &mut ggez::ContextBuilder::new("missle_command", "ggez")
        .add_resource_path(resource_dir)
        .build()?;

    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}
