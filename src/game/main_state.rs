use ggez::{event, Context, GameResult};
use ggez::timer;
use ggez::graphics::{self, Drawable, Point2, Text};

use super::flags::Flags;
use super::input::Input;
use engine::scene_stack::SceneStack;

pub struct MainState {
    flags: Flags,
    input: Input,
    pub scenes: Box<SceneStack<Input, Flags>>,
}

impl MainState {
    pub fn new() -> MainState {
        MainState {
            flags: Flags {},
            input: Input {},
            scenes: Box::new(Vec::new()),
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _: &mut Context) -> GameResult<()> {
        self.scenes.update(&self.input, &mut self.flags);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        self.scenes.draw(&self.flags, ctx);
        let font = ctx.default_font.clone();
        let fps = timer::get_fps(ctx) as u8;
        let text = Text::new(ctx, &fps.to_string(), &font)?;
        text.draw(ctx, Point2::new(0.0, 0.0), 0.0)?;
        graphics::present(ctx);
        Ok(())
    }
}
