use ggez::{Context, GameResult, event};
use ggez::graphics;
use ggez::event::{Keycode, Mod};
use ggez::graphics::{Text, Font, Drawable, Point2};

use visual_novel::command::Command;
use flags::Flags;
use input::Input;
use scene_stack::SceneStack;

pub struct MainState {
    flags: Flags,
    input: Input,
    scenes: Box<SceneStack>,
}


impl MainState {
    pub fn new() -> MainState {
       MainState {
           flags: Flags {},
           input: Input {},
           scenes: Box::new(Vec::new())
       }
    }
}


impl event::EventHandler for MainState {
    fn update(&mut self, _: &mut Context) -> GameResult<()> {
        self.scenes.update(&self.input, &mut self.flags);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.scenes.draw(&self.flags, ctx);
        Ok(())
    }
}
