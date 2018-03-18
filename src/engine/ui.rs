use ggez::{Context, GameResult};
use ggez::graphics::{rectangle, set_color, Color, DrawMode, Drawable, Point2, Rect, Text};

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
        let bounds = Message::bounds(ctx);
        set_color(ctx, Color::new(0.0, 0.0, 0.0, 1.0))?;
        rectangle(ctx, DrawMode::Fill, bounds)?;
        set_color(ctx, Color::new(1.0, 1.0, 1.0, 1.0))?;

        let font = ctx.default_font.clone();

        let (_, lines) = ctx.default_font.get_wrap(&self.text, 700);

        for (index, line) in lines.iter().enumerate() {
            let text = Text::new(ctx, &line, &font)?;
            text.draw(ctx, Point2::new(400.0, index as f32 * 25.0 + 100.0), 0.0)?
        }

        Ok(())
    }

    fn bounds(ctx: &Context) -> Rect {
        let width = ctx.conf.window_mode.width;
        let height = ctx.conf.window_mode.height;
        Rect {
            w: width as f32 * 0.8,
            h: height as f32 * 0.2,
            x: width as f32 * 0.1,
            y: height as f32 * 0.7,
        }
    }
}
