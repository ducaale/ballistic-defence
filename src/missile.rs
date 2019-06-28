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

    pub fn update(&mut self, elapsed: f32, explosions: &mut Vec<Explosion>) {
        let direction = (self.target - self.origin).normalize();
        self.position += direction * self.speed * elapsed;

        let distance_to_target = (self.target - self.position).magnitude();
        if distance_to_target < 5.0 {
            self.is_alive = false;
            self.explode(explosions);
        }
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
