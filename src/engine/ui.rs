use ggez::{Context, GameResult};
use ggez::graphics::{Drawable, Point2, Text};

pub struct Message {
    text: String,
}

impl Message {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let font = ctx.default_font.clone();

        let (_, lines) = ctx.default_font.get_wrap(&self.text, 700);

        for (index, line) in lines.iter().enumerate() {
            let text = Text::new(ctx, &line, &font)?;
            text.draw(ctx, Point2::new(400.0, index as f32 * 25.0 + 100.0), 0.0)?
        }

        Ok(())
    }
}
