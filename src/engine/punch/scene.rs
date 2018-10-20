use ggez::{self, GameResult};
use ggez::event::Keycode::{A, S, D};
use rlua::Lua;
use rlua::Value::Nil;

use super::scripting::EnemyDefinition;
use super::scripting::EnemyStateTransition::*;
use super::scripting::Hitzone;
use engine;
use engine::lua::LuaExt;
use engine::input::Input;


#[derive(Debug)]
pub struct Enemy {
    frame: u8,
    state: String,
}

impl Enemy {
    fn update(&mut self, enemy_definition: &EnemyDefinition, player: &Player) {
        let state = enemy_definition.states.get(&self.state).unwrap();
        if state.hitzones.did_hit_player(&player.hitzone) {
            self.frame = 1;
            self.state = match state.on_hitting_player {
                Static(ref new_state) => new_state.to_owned(),
                Dynamic(ref transition_fn) => transition_fn.call(Nil).unwrap(),
            }
        }
        else if self.frame >= state.frames {
            self.frame = 1;
            self.state = match state.on_end {
                Static(ref new_state) => new_state.to_owned(),
                Dynamic(ref transition_fn) => transition_fn.call(Nil).unwrap(),
            }
        } else {
            self.frame += 1;
        }
    }
}


#[derive(Debug)]
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
    enemy: Enemy,
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
            enemy: Enemy { state, frame: 0 }
        }
    }
}

impl<F> engine::scene::Scene<Input, F> for Scene {
    fn update(&mut self, input: &Input, _: &mut F) -> GameResult<()> {
        self.player.update(input);
        let enemy_definition = EnemyDefinition::load(&self.lua, &self.enemy_id);
        self.enemy.update(&enemy_definition, &self.player);
        Ok(())
    }


    fn draw(&self, _: &F, _ctx: &mut ggez::Context) -> GameResult<()> {
        Ok(())
    }
}
