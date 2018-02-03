use ggez::{Context, GameResult, event};
use ggez::graphics;
use ggez::event::{Keycode, Mod};
use ggez::graphics::{Text, Font, Drawable, Point2};

use visual_novel::command::Command;

pub struct MainState {
    dialog: Vec<Command>,
    font: Font,
    dialog_index: usize,
}


impl MainState {
    pub fn new(_: &mut Context, dialog: Vec<Command>) -> GameResult<MainState> {
        Ok(MainState {
            dialog,
            font:  Font::default_font().unwrap(),
            dialog_index: 0,
        })
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

    fn key_up_event(&mut self, _: &mut Context, keycode: Keycode, _: Mod, _: bool) {
        match keycode {
            Keycode::Left => self.dialog_index = match self.dialog_index {
                0 => 0,
                x => x - 1,
            },
            Keycode::Right => self.dialog_index = match self.dialog_index {
                x if x == self.dialog.len() - 1 => x,
                x => x + 1,
            },
            _ => {},
        };
    }
}
