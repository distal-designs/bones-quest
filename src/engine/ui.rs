use ggez::graphics::{
    self, BlendMode, Color, DrawMode, DrawParam, Drawable, Image, Mesh, Point2, Rect, Text,
};
use ggez::{Context, GameResult};

use super::draw_cache::TryIntoDrawable;

pub struct Dialog {
    pub text: String,
    pub portrait: Option<Portrait>,
}

pub struct DialogCache {
    text_cache: Vec<Text>,
    dialog_box: Mesh,
    portrait: Option<Image>,
    character: Option<Text>,
    name_box: Option<Mesh>,
}

#[derive(Clone)]
pub struct Portrait {
    pub character: String,
    pub style: String,
}

pub struct Character {
    pub name: String,
    pub direction: String,
    pub position: i8,
}

impl TryIntoDrawable<Image> for Character {
    fn try_into_drawable(&self, ctx: &mut Context) -> GameResult<Image> {
        Image::solid(ctx, 50, graphics::WHITE)
    }
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

        let portrait = match self.portrait {
            Some(_) => {
                let height = Dialog::bounds(ctx).h as u16;
                let image = Image::solid(ctx, height, Color::from_rgb(0, 255, 0))?;
                Some(image)
            }
            None => None,
        };

        let (character, name_box) = match self.portrait {
            Some(Portrait { ref character, .. }) => {
                let text = Text::new(ctx, &character, &font)?;
                let name_box = graphics::Mesh::new_polygon(
                    ctx,
                    DrawMode::Fill,
                    &[
                        Point2::new(0.0, 0.0),
                        Point2::new(0.0, 20.0),
                        Point2::new(bounds.h, 20.0),
                        Point2::new(bounds.h, 0.0),
                    ],
                )?;
                (Some(text), Some(name_box))
            }
            None => (None, None),
        };

        Ok(DialogCache {
            text_cache,
            dialog_box,
            portrait,
            character,
            name_box,
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
                ..DrawParam::default()
            },
        )?;

        if let Some(ref portrait) = self.portrait {
            portrait.draw_ex(
                ctx,
                DrawParam {
                    dest: Point2::new(bounds.x, bounds.y),
                    ..DrawParam::default()
                },
            )?;
        }

        if let (Some(character), Some(name_box)) = (&self.character, &self.name_box) {
            let dest = Point2::new(bounds.x, bounds.y - 20.0);
            name_box.draw_ex(
                ctx,
                DrawParam {
                    dest,
                    color: Some(graphics::BLACK),
                    ..DrawParam::default()
                },
            )?;
            character.draw(ctx, dest, 0.0)?;
        }

        for (index, text) in self.text_cache.iter().enumerate() {
            let x = bounds.x;
            let y = bounds.y as usize + (index * text.height() as usize);
            text.draw(ctx, Point2::new(x, y as f32), 0.0)?
        }
        Ok(())
    }

    fn set_blend_mode(&mut self, mode: Option<graphics::BlendMode>) {
        for text in &mut self.text_cache {
            text.set_blend_mode(mode);
        }
    }

    fn get_blend_mode(&self) -> Option<graphics::BlendMode> {
        self.text_cache
            .first()
            .and_then(|text| text.get_blend_mode())
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

pub enum Background {
    Color(graphics::Color),
}

impl TryIntoDrawable<BackgroundCache> for Background {
    fn try_into_drawable(&self, ctx: &mut Context) -> GameResult<BackgroundCache> {
        let h = ctx.conf.window_mode.height as f32;
        let w = ctx.conf.window_mode.width as f32;
        let points = [
            Point2::new(0.0, 0.0),
            Point2::new(0.0, h),
            Point2::new(w, h),
            Point2::new(w, 0.0),
        ];
        let mesh = graphics::Mesh::new_polygon(ctx, DrawMode::Fill, &points)?;
        Ok(BackgroundCache::Mesh(mesh))
    }
}

pub enum BackgroundCache {
    Mesh(graphics::Mesh),
}

impl Drawable for BackgroundCache {
    fn draw_ex(&self, ctx: &mut Context, mode: DrawParam) -> GameResult<()> {
        match *self {
            BackgroundCache::Mesh(ref mesh) => mesh.draw_ex(ctx, mode),
        }
    }

    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        match *self {
            BackgroundCache::Mesh(ref mut mesh) => mesh.set_blend_mode(mode),
        }
    }

    fn get_blend_mode(&self) -> Option<BlendMode> {
        match *self {
            BackgroundCache::Mesh(ref mesh) => mesh.get_blend_mode(),
        }
    }
}

pub fn to_window_position(width: u32, position: i8) -> f32 {
    let position = f32::from(position);
    let width = width as f32;

    let half = width / 2.0;
    let shift = half * position / 100.0;
    half + shift
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_window_position() {
        assert_eq!(to_window_position(800, -100), 0.0);
        assert_eq!(to_window_position(800, -50), 200.0);
        assert_eq!(to_window_position(800, 0), 400.0);
        assert_eq!(to_window_position(800, 50), 600.0);
        assert_eq!(to_window_position(800, 100), 800.0);
    }
}
