use ggez::{self, GameResult};
use rlua::Lua;

use engine;
use super::scripting::EnemyDefinition;


pub struct Scene {
    lua: Lua,
    enemy_id: String,
}


impl<I, F> engine::scene::Scene<I, F> for Scene {
    fn update(&mut self, _: &I, _: &mut F) -> GameResult<()> {
        unimplemented!();
    }


    fn draw(&self, _: &F, _ctx: &mut ggez::Context) -> GameResult<()> {
        unimplemented!();
    }
}
