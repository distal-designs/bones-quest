#[macro_use]
extern crate serde_derive;
extern crate ggez;

use ggez::{conf, Context, GameResult, event};
use ggez::graphics;
use std::time::Duration;

mod visual_novel;


struct MainState {
}


impl MainState {
    fn new(_: &mut Context) -> GameResult<MainState>  {
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
}
