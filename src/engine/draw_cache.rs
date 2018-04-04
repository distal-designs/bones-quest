use std::cell::RefCell;

use try_from::TryFrom;
use ggez::{
    Context,
    GameError,
    GameResult,
    graphics::BlendMode,
    graphics::Drawable,
    graphics::DrawParam,
};

pub struct DrawCache<T, U> {
    data: T,
    cache: RefCell<Option<U>>,
}

impl<T, U> DrawCache<T, U>
where
    U: Drawable + TryFrom<T, Err=GameError>,
{
    pub fn new(data: T) -> Self {
        Self {
            data,
            cache: RefCell::new(None),
        }
    }
}

impl<T, U> Drawable for DrawCache<T, U>
where
    T: Clone,
    U: Drawable + TryFrom<T, Err=GameError>,
{
    fn draw_ex(&self, ctx: &mut Context, param: DrawParam) -> GameResult<()> {
        static ERROR_MESSAGE: &'static str = "DrawCache Error";
        if let None = *self.cache.borrow() {
            let drawable = U::try_from(self.data.clone())?;
            self.cache.replace(Some(drawable));
        }
        if let Some(ref cache) = *self.cache.borrow() {
            cache.draw_ex(ctx, param)
        } else {
            Err(GameError::RenderError(ERROR_MESSAGE.to_string()))
        }
    }

    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        if let &mut Some(ref mut drawable) = self.cache.get_mut() {
            drawable.set_blend_mode(mode);
        }
    }

    fn get_blend_mode(&self) -> Option<BlendMode> {
        match *self.cache.borrow() {
            None => None,
            Some(ref drawable) => drawable.get_blend_mode()
        }
    }
}
