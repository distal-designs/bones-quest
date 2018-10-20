use ggez::{self, GameResult};
use rlua::Lua;
use rlua::Value::Nil;

use super::scripting::EnemyDefinition;
use super::scripting::EnemyStateTransition::*;
use super::scripting::Hitzone;
use engine;
use engine::lua::LuaExt;


#[derive(Debug)]
pub struct EnemyState {
    frame: u8,
    state: String,
}


pub struct Player {
    pub hitzone: Hitzone,
}

impl Default for Player {
    fn default() -> Self {
        Player { hitzone: Hitzone::Stand }
    }
}

impl Player {
    fn update(&mut self, input: &Input) {
        self.hitzone = if input.current_input.contains(&S) {
            Hitzone::Duck
        } else if input.current_input.contains(&A) {
            Hitzone::Left
        } else if input.current_input.contains(&D) {
            Hitzone::Right
        } else {
            Hitzone::Stand
        }
    }
}


pub struct Scene {
    lua: Lua,
    enemy_id: String,
    enemy_state: EnemyState,
    player: Player,
}


impl Scene {
    pub fn new(enemy_id: &str) -> Self {
        let lua = Lua::new_with_path();
        let enemy_id = enemy_id.to_owned();
        let state = EnemyDefinition::load(&lua, &enemy_id).default_state;
        Scene {
            player: Player::default(),
            lua: Lua::new_with_path(),
            enemy_id: enemy_id.to_owned(),
            enemy_state: EnemyState { state, frame: 0 }
        }
    }
}


impl<I, F> engine::scene::Scene<I, F> for Scene {
    fn update(&mut self, _: &I, _: &mut F) -> GameResult<()> {
        let enemy_definition = EnemyDefinition::load(&self.lua, &self.enemy_id);
        let state_definition = enemy_definition.states.get(&self.enemy_state.state).unwrap();
        if self.enemy_state.frame >= state_definition.frames {
            self.enemy_state.frame = 1;
            self.enemy_state.state = match state_definition.on_end {
                Static(ref new_state) => new_state.to_owned(),
                Dynamic(ref transition_fn) => transition_fn.call(Nil).unwrap(),
            }
        } else {
            self.enemy_state.frame += 1;
        }
        Ok(())
    }


    fn draw(&self, _: &F, _ctx: &mut ggez::Context) -> GameResult<()> {
        println!("{:#?}", self.enemy_state);
        Ok(())
    }
}
