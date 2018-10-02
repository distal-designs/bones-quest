use ggez::{self, GameResult};
use rlua::Lua;

use engine;
use engine::lua::LuaExt;


pub struct Scene {
    lua: Lua,
    enemy_id: String,
}


impl Scene {
    fn new(enemy_id: &str) -> Self {
        Scene {
            lua: Lua::new_with_path(),
            enemy_id: enemy_id.to_owned(),
        }
    }


impl<I, F> engine::scene::Scene<I, F> for Scene {
    fn update(&mut self, _: &I, _: &mut F) -> GameResult<()> {
        unimplemented!();
    }


    fn draw(&self, _: &F, _ctx: &mut ggez::Context) -> GameResult<()> {
        unimplemented!();
    }
}
