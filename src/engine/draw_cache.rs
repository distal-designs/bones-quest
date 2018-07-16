use std::cell::RefCell;
use std::fmt;

use ggez::{
    graphics::{BlendMode, DrawParam, Drawable},
    Context, GameResult,
};


pub trait TryIntoDrawable<T>
where
    T: Drawable,
{
    fn try_into_drawable(&self, ctx: &mut Context) -> GameResult<T>;
}


pub struct DrawCache<T, U>
where
    T: TryIntoDrawable<U>,
    U: Drawable,
{
    data: T,
    cache: RefCell<Option<U>>,
}


impl<T, U> DrawCache<T, U>
where
    T: TryIntoDrawable<U>,
    U: Drawable,
{
    pub fn new(data: T) -> Self {
        Self {
            data,
            cache: RefCell::new(None),
        }
    }
}


impl<T, U> AsRef<T> for DrawCache<T, U>
where
    T: TryIntoDrawable<U>,
    U: Drawable,
{
    fn as_ref(&self) -> &T {
        &self.data
    }
}


impl<T, U> fmt::Debug for DrawCache<T, U>
where
    T: TryIntoDrawable<U> + fmt::Debug,
    U: Drawable,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.data)
    }
}


impl<T, U> Drawable for DrawCache<T, U>
where
    T: TryIntoDrawable<U>,
    U: Drawable,
{
    fn draw_ex(&self, ctx: &mut Context, param: DrawParam) -> GameResult<()> {
        let is_cached = self.cache.borrow().is_some();
        if !is_cached {
            let drawable = self.data.try_into_drawable(ctx)?;
            self.cache.replace(Some(drawable));
        }
        self.cache
            .borrow_mut()
            .as_mut()
            .unwrap()
            .draw_ex(ctx, param)
    }


    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        if let Some(ref mut drawable) = *self.cache.get_mut() {
            drawable.set_blend_mode(mode);
        }
    }


    fn get_blend_mode(&self) -> Option<BlendMode> {
        match *self.cache.borrow() {
            None => None,
            Some(ref drawable) => drawable.get_blend_mode(),
        }
    }
}
