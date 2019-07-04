use ggez::graphics;
use ggez::{Context, GameResult};
use cgmath::Point2;
use cgmath::InnerSpace;

use crate::Explosion;

pub struct Missile {
    speed: f32,
    pub position: Point2<f32>,
    origin: Point2<f32>,
    target: Point2<f32>,
    pub is_invincible: bool,
    pub is_alive: bool
}

impl Missile {
    pub fn new(speed: f32, origin: Point2<f32>, target: Point2<f32>, is_invincible: bool) -> Missile {
        let direction = (target - origin).normalize();
        let position = origin + direction;

        Missile { speed, position, origin, target, is_invincible, is_alive: true }
    }

    pub fn update(&mut self, elapsed: f32) {
        let direction = (self.target - self.origin).normalize();
        self.position += direction * self.speed * elapsed;

        if self.did_hit_target() {
            self.is_alive = false;
        }
    }

    pub fn did_hit_target(&self) -> bool {
        let distance_to_target = (self.target - self.position).magnitude();
        distance_to_target < 5.0
    }

    pub fn explode(&self, explosions: &mut Vec<Explosion>) {
        explosions.push(Explosion::new(self.position));
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let line = graphics::Mesh::new_line(ctx, &[self.origin, self.position], 2.0, graphics::WHITE)?;
        graphics::draw(ctx, &line, (Point2::new(0.0, 0.0),))?;
        Ok(())
    }
}
