use ggez::graphics;
use ggez::{Context, GameResult};
use cgmath::Point2;

pub struct Explosion {
    pub position: Point2<f32>,
    pub radius: f32,
    pub is_alive: bool,
    pub is_radius_increasing: bool
}

impl Explosion {
    pub fn new(position: Point2<f32>) -> Explosion {
        Explosion {
            position,
            radius: 20.0,
            is_alive: true,
            is_radius_increasing: true
        }
    }

    pub fn update(&mut self, elapsed: f32) {
        let detonation_velocity = 85.0;
        let target_radius = 80.0;

        if self.is_radius_increasing {
            self.radius += detonation_velocity * elapsed;
            if self.radius > target_radius {
                self.is_radius_increasing = false;
            }
        }
        else {
            self.radius -= detonation_velocity * elapsed;
        }

        if self.radius < 20.0 {
            self.is_alive = false;
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Point2::new(0.0, 0.0),
            self.radius,
            0.05,
            graphics::WHITE
        )?;
        graphics::draw(ctx, &circle, (self.position,))?;

        Ok(())
    }
}