extern crate strum;
#[macro_use] extern crate strum_macros;
extern crate ggez;
extern crate rlua;
extern crate queues;
extern crate regex;
#[macro_use]
extern crate serde_derive;


mod engine;


use ggez::{conf, event, Context};

use engine::main_state::MainState;
use engine::punch::Scene;


fn main() {
    let mut c = conf::Conf::default();
    c.window_setup.title = "Bones Quest".to_string();
    let ctx = &mut Context::load_from_conf("bones-quest", "distal-designs", c).unwrap();

    let state = &mut MainState::new();
    let scene = Scene::new("example");
    state.scenes.push(Box::new(scene));
    event::run(ctx, state).unwrap();
}
