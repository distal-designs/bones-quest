use ggez::{self, GameResult};
use rlua::Lua;

use super::scripting::EnemyDefinition;
use engine;
use engine::lua::LuaExt;


pub struct EnemyState {
    frame: u8,
    state: String,
}


pub struct Scene {
    lua: Lua,
    enemy_id: String,
    enemy_state: EnemyState,
}


impl Scene {
    pub fn new(enemy_id: &str) -> Self {
        let lua = Lua::new_with_path();
        let enemy_id = enemy_id.to_owned();
        let state = Self::enemy_definition(&lua, &enemy_id).default_state;
        Scene {
            lua: Lua::new_with_path(),
            enemy_id: enemy_id.to_owned(),
            enemy_state: EnemyState { state, frame: 0 }
        }
    }


    fn enemy_definition<'lua>(lua: &'lua Lua, enemy_id: &str) -> EnemyDefinition<'lua> {
        let loader = format!("return require 'resources.enemies.{}'", enemy_id);
        lua.exec(&loader, Some("Loading enemy definition")).unwrap()
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
