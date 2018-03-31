use ggez::{event, Context, GameResult};
use ggez::event::{Keycode, Mod};
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

    // input object will be a set of all key downs
    // clone set of previous frame
    // add any new key downs, remove any key ups
    // ask ggez what frame it is, input handler should store the most recent frame index it has
    // seen
    fn key_up_event(&mut self, keycode: Keycode, _: Mod, _: bool) {
        
        /*match keycode {
            Keycode::Left => self.dialog_index = match self.dialog_index {
                0 => 0,
                x => x - 1,
            },
            Keycode::Right => self.dialog_index = match self.dialog_index {
                x if x == self.dialog.len() - 1 => x,
                x => x + 1,
            },
            _ => {},
        };*/
    }
}
