use std::collections::HashMap;
use std::cell::RefCell;

use ggez::{Context, GameResult};
use ggez::graphics::{rectangle, Color, DrawMode, Drawable, Font, Point2, Rect, Text};

use super::color::with_color;

pub struct Message {
    text: String,
    font_cache: RefCell<Option<Font>>,
    line_cache: RefCell<HashMap<String, Text>>,
}

impl Message {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            font_cache: RefCell::new(None),
            line_cache: RefCell::new(HashMap::new()),
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let bounds = Message::bounds(ctx);
        with_color(ctx, &Color::from_rgb(0, 0, 0), |ctx| {
            rectangle(ctx, DrawMode::Fill, bounds)
        })?;

        let mut font_cache = self.font_cache.borrow_mut();
        let font = font_cache.get_or_insert_with(|| ctx.default_font.clone());

        let (_, lines) = font.get_wrap(&self.text, bounds.w as usize);
        let mut line_cache = self.line_cache.borrow_mut();
        for (index, line) in lines.iter().enumerate() {
            let text = line_cache
                .entry(line.to_string())
                .or_insert_with(|| Text::new(ctx, &line, &font).unwrap());
            let x = bounds.x;
            let y = bounds.y as usize + (index * text.height() as usize);
            text.draw(ctx, Point2::new(x, y as f32), 0.0)?
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
