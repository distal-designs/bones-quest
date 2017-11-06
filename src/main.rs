#[macro_use]
extern crate serde_derive;
extern crate ggez;

use ggez::{conf, Context, GameResult, event};

mod visual_novel;


struct MainState {
}


impl MainState {
    fn new(_: &mut Context) -> GameResult<MainState>  {
        Ok(MainState {})
    }
}


fn main() {
    println!("{:?}", visual_novel::Command::load("blood"));
}
