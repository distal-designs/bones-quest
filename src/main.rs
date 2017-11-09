#[macro_use]
extern crate serde_derive;
extern crate ggez;

use ggez::{conf, Context, GameResult, event};
use ggez::graphics;
use ggez::graphics::{Text, Font, Drawable, Point};
use std::time::Duration;

use visual_novel::Command;

mod visual_novel;


struct MainState {
    dialog: Vec<Command>,
    font: Font,
}


impl MainState {
    fn new(_: &mut Context, dialog: Vec<Command>) -> GameResult<MainState> {
        Ok(MainState {
            dialog,
            font:  Font::default_font().unwrap(),
        })
    }
}


impl event::EventHandler for MainState {
    fn update(&mut self, _: &mut Context, _: Duration) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::present(ctx);
        Ok(())
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
