use std::cell::RefCell;

use ggez::{Context, GameResult};
use ggez::graphics::{self, Color, DrawMode, DrawParam, Drawable, Font, Point2, Rect, Text};

use super::draw_cache::TryIntoDrawable;

pub struct Dialog {
    pub text: String,
}

impl TryIntoDrawable<DialogCache> for Dialog {
    fn try_into_drawable(&self, ctx: &mut Context) -> GameResult<DialogCache> {
        let font = ctx.default_font.clone();
        let texts = font.get_wrap(&self.text, Message::bounds(ctx).w as usize)
            .1
            .iter()
            .map(|line| Text::new(ctx, &line, &font).unwrap())
            .collect();
        Ok(DialogCache {
            font_cache: font,
            text_cache: texts,
        })
    }
}

pub struct DialogCache {
    font_cache: Font,
    text_cache: Vec<Text>,
}

impl Drawable for DialogCache {
    fn draw_ex(&self, ctx: &mut Context, _param: DrawParam) -> GameResult<()> {
        let bounds = Message::bounds(ctx);

        graphics::Mesh::new_polygon(
            ctx,
            DrawMode::Fill,
            &[
                Point2::new(0.0, 0.0),
                Point2::new(0.0, bounds.h),
                Point2::new(bounds.w, bounds.h),
                Point2::new(bounds.w, 0.0),
            ],
        )?.draw_ex(
            ctx,
            DrawParam {
                dest: Point2::new(bounds.x, bounds.y),
                color: Some(Color::from_rgb(0, 0, 0)),
                ..Default::default()
            },
        )?;

        for (index, text) in self.text_cache.iter().enumerate() {
            let x = bounds.x;
            let y = bounds.y as usize + (index * text.height() as usize);
            text.draw(ctx, Point2::new(x, y as f32), 0.0)?
        }
        Ok(())
    }

    fn set_blend_mode(&mut self, mode: Option<graphics::BlendMode>) {
        for text in self.text_cache.iter_mut() {
            text.set_blend_mode(mode);
        }
    }

    fn get_blend_mode(&self) -> Option<graphics::BlendMode> {
        for text in self.text_cache.iter() {
            return text.get_blend_mode();
        }
        None
    }
}

pub struct Message {
    text: String,
    font_cache: RefCell<Option<Font>>,
    text_cache: RefCell<Option<Vec<Text>>>,
}

impl Drawable for Message {
    fn draw_ex(&self, ctx: &mut Context, _param: DrawParam) -> GameResult<()> {
        let bounds = Message::bounds(ctx);

        let mut font_cache = self.font_cache.borrow_mut();
        let font = font_cache.get_or_insert_with(|| ctx.default_font.clone());

        let mut text_cache = self.text_cache.borrow_mut();
        let texts = text_cache.get_or_insert_with(|| {
            font.get_wrap(&self.text, bounds.w as usize)
                .1
                .iter()
                .map(|line| Text::new(ctx, &line, &font).unwrap())
                .collect()
        });

        graphics::Mesh::new_polygon(
            ctx,
            DrawMode::Fill,
            &[
                Point2::new(0.0, 0.0),
                Point2::new(0.0, bounds.h),
                Point2::new(bounds.w, bounds.h),
                Point2::new(bounds.w, 0.0),
            ],
        )?.draw_ex(
            ctx,
            DrawParam {
                dest: Point2::new(bounds.x, bounds.y),
                color: Some(Color::from_rgb(0, 0, 0)),
                ..Default::default()
            },
        )?;

        for (index, text) in texts.iter().enumerate() {
            let x = bounds.x;
            let y = bounds.y as usize + (index * text.height() as usize);
            text.draw(ctx, Point2::new(x, y as f32), 0.0)?
        }
        Ok(())
    }

    fn set_blend_mode(&mut self, mode: Option<graphics::BlendMode>) {
        if let &mut Some(ref mut texts) = self.text_cache.get_mut() {
            for text in texts.iter_mut() {
                text.set_blend_mode(mode);
            }
        }
    }

    fn get_blend_mode(&self) -> Option<graphics::BlendMode> {
        if let Some(texts) = self.text_cache.borrow().as_ref() {
            for text in texts.iter() {
                return text.get_blend_mode();
            }
        }
        None
    }
}

impl Message {
    pub fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            font_cache: RefCell::new(None),
            text_cache: RefCell::new(None),
        }
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
