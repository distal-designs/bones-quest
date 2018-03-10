extern crate ggez;
#[macro_use]
extern crate serde_derive;

mod visual_novel;
mod scene_stack;
mod scene;
mod main_state;
mod input;
mod flags;

use ggez::{conf, event, Context};

use visual_novel::command::Command;
use main_state::MainState;

fn main() {
    let mut c = conf::Conf::default();
    c.window_setup.title = "Bones Quest".to_string();
    let ctx = &mut Context::load_from_conf("bones-quest", "distal-designs", c).unwrap();
    let state = &mut MainState::new();
    let dialog = Command::load("blood").unwrap();
    let vn = visual_novel::scene::Scene::new(dialog);
    state.scenes.push(Box::new(vn));
    event::run(ctx, state).unwrap();
}
