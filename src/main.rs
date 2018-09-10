extern crate ggez;
extern crate rlua;
extern crate queues;
extern crate regex;
#[macro_use]
extern crate serde_derive;


mod engine;


use std::io::Read;

use ggez::{conf, event, Context};

use engine::main_state::MainState;
use engine::scenes::visual_novel::VisualNovel;
use engine::visual_novel::command::Command;


fn main() {
    let mut c = conf::Conf::default();
    c.window_setup.title = "Bones Quest".to_string();
    let ctx = &mut Context::load_from_conf("bones-quest", "distal-designs", c).unwrap();

    let lua = rlua::Lua::new();
    let mut script = String::new();
    ctx.filesystem
        .open("/enemies/example.lua")
        .unwrap()
        .read_to_string(&mut script)
        .unwrap();
    let enemy_table: rlua::Table = lua.exec(&script, None).unwrap();
    println!("{:?}", enemy_table);

    let state = &mut MainState::new();
    let dialog = Command::load(&mut ctx.filesystem, "/dialogs/blood.toml").unwrap();
    let vn = VisualNovel::new(dialog);
    state.scenes.push(Box::new(vn));
    event::run(ctx, state).unwrap();
}
