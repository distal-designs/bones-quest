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
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        let (_, lines) = self.font.get_wrap(&self.dialog[self.dialog_index].text, 700);

        for (index, line) in lines.iter().enumerate() {
            Text::new(ctx, &line, &self.font)
                .unwrap()
                .draw(ctx, Point2::new(0.0, index as f32 * 25.0 + 100.0 ), 0.0)
                .unwrap();
        }

        graphics::present(ctx);
        Ok(())
    }
}
