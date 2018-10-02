use ggez::{self, GameResult};
use rlua::Lua;

use super::scripting::EnemyDefinition;
use engine;
use engine::lua::LuaExt;


pub struct Scene {
    lua: Lua,
    enemy_id: String,
}


impl Scene {
    pub fn new(enemy_id: &str) -> Self {
        Scene {
            lua: Lua::new_with_path(),
            enemy_id: enemy_id.to_owned(),
        }
    }


    fn enemy_definition(&self) -> EnemyDefinition {
        let loader = format!("return require 'resources.enemies.{}'", self.enemy_id);
        self.lua.exec(&loader, Some("Loading enemy definition")).unwrap()
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
