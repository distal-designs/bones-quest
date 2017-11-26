#[macro_use]
extern crate serde_derive;
extern crate ggez;

mod visual_novel;
mod scene_stack;
mod scene;
mod main_state;

use ggez::{conf, Context, event};

use visual_novel::command::Command;
use main_state::MainState;

fn main() {
    let mut c = conf::Conf::default();
    c.window_title = "Bones Quest".to_string();
    let ctx = &mut Context::load_from_conf("bones-quest", "distal-designs", c).unwrap();
    let dialog = Command::load("blood").unwrap();
    let state = &mut MainState::new(ctx, dialog).unwrap();
    event::run(ctx, state).unwrap();
}
