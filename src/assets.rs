use ggez::graphics;
use ggez::{Context, GameResult};

pub struct Assets {
    pub cursor_image: graphics::Image,
    pub font: graphics::Font
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let cursor_image = graphics::Image::new(ctx, "/crosshair.png")?;
        let font= graphics::Font::new(ctx, "/DejaVuSerif.ttf")?;
        let assets = Assets { cursor_image, font };
        Ok(assets)
    }
}