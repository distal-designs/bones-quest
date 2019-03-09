use ggez::event::{Keycode, Mod};
use ggez::graphics::{self, Drawable, Point2, Text};
use ggez::timer;
use ggez::{event, Context, GameResult};

use super::flags::Flags;
use super::input::Input;
use crate::engine::scene_stack::SceneStack;


pub struct MainState {
    flags: Flags,
    input: Input,
    pub scenes: Box<SceneStack<Input, Flags>>,
}


impl MainState {
    pub fn new() -> Self {
        Self {
            flags: Flags {},
            input: Input::new(),
            scenes: Box::new(Vec::new()),
        }
    }
}


impl event::EventHandler for MainState {
    fn update(&mut self, _: &mut Context) -> GameResult<()> {
        self.input.finalize();
        self.scenes.update(&self.input, &mut self.flags)?;
        Ok(())
    }


    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        self.scenes.draw(&self.flags, ctx)?;
        let font = ctx.default_font.clone();
        let fps = timer::get_fps(ctx) as u8;
        let text = Text::new(ctx, &fps.to_string(), &font)?;
        text.draw(ctx, Point2::new(0.0, 0.0), 0.0)?;
        graphics::present(ctx);
        Ok(())
    }


    fn key_down_event(&mut self, _: &mut Context, keycode: Keycode, _: Mod, _: bool) {
        self.input.add_input(keycode);
    }


    fn key_up_event(&mut self, _: &mut Context, keycode: Keycode, _: Mod, _: bool) {
        self.input.remove_input(keycode);
    }
}
