use ggez::{event, Context, GameResult};
use ggez::graphics;

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
        graphics::present(ctx);
        Ok(())
    }
}
