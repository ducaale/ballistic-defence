use ggez::graphics;
use ggez::{Context, GameResult};

pub struct Assets {
    pub cursor_image: graphics::Image
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let cursor_image = graphics::Image::new(ctx, "/crosshair.png")?;
        let assets = Assets { cursor_image };
        Ok(assets)
    }
}