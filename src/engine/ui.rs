use ggez::{Context, GameResult};
use ggez::graphics::{self, DrawMode, DrawParam, Drawable, Mesh, Point2, Rect, Text};

use super::draw_cache::TryIntoDrawable;

pub struct Dialog {
    pub text: String,
    pub portrait: Option<Portrait>,
}

pub struct DialogCache {
    text_cache: Vec<Text>,
    dialog_box: Mesh,
}

#[derive(Clone)]
pub struct Portrait {
    pub character: String,
    pub style: String,
}

impl TryIntoDrawable<DialogCache> for Dialog {
    fn try_into_drawable(&self, ctx: &mut Context) -> GameResult<DialogCache> {
        let font = ctx.default_font.clone();
        let text_cache = font.get_wrap(&self.text, Dialog::bounds(ctx).w as usize)
            .1
            .iter()
            .map(|line| Text::new(ctx, &line, &font).unwrap())
            .collect();

        let bounds = Dialog::bounds(ctx);
        let dialog_box = graphics::Mesh::new_polygon(
            ctx,
            DrawMode::Fill,
            &[
                Point2::new(0.0, 0.0),
                Point2::new(0.0, bounds.h),
                Point2::new(bounds.w, bounds.h),
                Point2::new(bounds.w, 0.0),
            ],
        )?;
        Ok(DialogCache {
            text_cache,
            dialog_box,
        })
    }
}

impl Drawable for DialogCache {
    fn draw_ex(&self, ctx: &mut Context, _param: DrawParam) -> GameResult<()> {
        let bounds = Dialog::bounds(ctx);

        self.dialog_box.draw_ex(
            ctx,
            DrawParam {
                dest: Point2::new(bounds.x, bounds.y),
                color: Some(graphics::BLACK),
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

impl Dialog {
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
