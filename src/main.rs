#[macro_use]
extern crate serde_derive;
extern crate ggez;

use ggez::{conf, Context, GameResult, event};
use ggez::graphics;
use ggez::event::{Keycode, Mod};
use ggez::graphics::{Text, Font, Drawable, Point};
use std::time::Duration;

use visual_novel::Command;

mod visual_novel;


struct MainState {
    dialog: Vec<Command>,
    font: Font,
    dialog_index: usize,
}


impl MainState {
    fn new(_: &mut Context, dialog: Vec<Command>) -> GameResult<MainState> {
        Ok(MainState {
            dialog,
            font:  Font::default_font().unwrap(),
            dialog_index: 0,
        })
    }
}


impl event::EventHandler for MainState {
    fn update(&mut self, _: &mut Context, _: Duration) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        let (_, lines) = self.font.get_wrap(&self.dialog[self.dialog_index].text, 700);

        for (index, line) in lines.iter().enumerate() {
            Text::new(ctx, &line, &self.font)
                .unwrap()
                .draw(ctx, Point { x: 400.0, y: index as f32 * 25.0 + 100.0 }, 0.0)
                .unwrap();
        }

        graphics::present(ctx);
        Ok(())
    }

    fn key_up_event(&mut self, keycode: Keycode, _: Mod, _: bool) {
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


fn main() {
    let mut c = conf::Conf::default();
    c.window_title = "Bones Quest".to_string();
    let ctx = &mut Context::load_from_conf("bones-quest", "distal-designs", c).unwrap();
    let dialog = visual_novel::Command::load("blood").unwrap();
    let state = &mut MainState::new(ctx, dialog).unwrap();
    event::run(ctx, state).unwrap();
}
