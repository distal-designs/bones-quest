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
    fn new(_: &mut Context) -> GameResult<MainState> {
        Ok(MainState {})
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
    println!("{:?}", visual_novel::Command::load("blood"));

    let mut c = conf::Conf::default();
    c.window_title = "Bones Quest".to_string();
    let ctx = &mut Context::load_from_conf("bones-quest", "distal-designs", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
