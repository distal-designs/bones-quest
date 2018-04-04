extern crate ggez;
#[macro_use]
extern crate serde_derive;
extern crate regex;

mod engine;
mod game;

use ggez::{conf, event, Context};

use engine::visual_novel::command::Command;
use game::scenes::VisualNovel;
use game::main_state::MainState;

fn main() {
    let mut c = conf::Conf::default();
    c.window_setup.title = "Bones Quest".to_string();
    let ctx = &mut Context::load_from_conf("bones-quest", "distal-designs", c).unwrap();
    let state = &mut MainState::new();
    let dialog = Command::load("blood").unwrap();
    let vn = VisualNovel::new(dialog);
    state.scenes.push(Box::new(vn));
    event::run(ctx, state).unwrap();
}
