extern crate ggez;
extern crate queues;
extern crate regex;
#[macro_use]
extern crate serde_derive;

mod engine;

use std::env;
use std::path;

use ggez::{conf, event, Context};

use engine::main_state::MainState;
use engine::scenes::visual_novel::VisualNovel;
use engine::visual_novel::command::Command;

fn main() {
    let mut c = conf::Conf::default();
    c.window_setup.title = "Bones Quest".to_string();
    let ctx = &mut Context::load_from_conf("bones-quest", "distal-designs", c).unwrap();

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.filesystem.mount(&path, true);
    }

    let state = &mut MainState::new();
    let dialog = Command::load(&mut ctx.filesystem, "/dialogs/blood.toml").unwrap();
    let vn = VisualNovel::new(dialog);
    state.scenes.push(Box::new(vn));
    event::run(ctx, state).unwrap();
}
