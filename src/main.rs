extern crate strum;
#[macro_use] extern crate strum_macros;
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
use engine::lua::LuaExt;
use engine::punch::EnemyDefinition;


fn main() {
    let mut c = conf::Conf::default();
    c.window_setup.title = "Bones Quest".to_string();
    let ctx = &mut Context::load_from_conf("bones-quest", "distal-designs", c).unwrap();

    let lua = rlua::Lua::new_with_path();
    let mut script = String::new();
    ctx.filesystem
        .open("/enemies/example.lua")
        .unwrap()
        .read_to_string(&mut script)
        .unwrap();
    let script = "return require 'resources.enemies.example'";
    println!("{:#?}", lua.eval::<EnemyDefinition>(&script, None).unwrap());

    let state = &mut MainState::new();
    let dialog = Command::load(&mut ctx.filesystem, "/dialogs/blood.toml").unwrap();
    let vn = VisualNovel::new(dialog);
    state.scenes.push(Box::new(vn));
    event::run(ctx, state).unwrap();
}
