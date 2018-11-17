use ggez::graphics::{Drawable, Point2, Text};
use ggez::{self, GameResult};
use rlua::Lua;

use super::Player;
use super::Enemy;
use super::scripting::EnemyDefinition;
use engine;
use engine::input::Input;
use engine::lua::LuaExt;


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
            enemy: Enemy { state, frame: 0 },
        }
    }
}


impl<F> engine::scene::Scene<Input, F> for Scene {
    fn update(&mut self, input: &Input, _: &mut F) -> GameResult<()> {
        self.player.update(input);
        let enemy_definition = EnemyDefinition::load(&self.lua, &self.enemy_id);
        let state_definition = enemy_definition.state(&self.enemy.state);
        self.enemy.update(&state_definition, &self.player);
        self.player.handle_collisions(&state_definition.hitzones);
        Ok(())
    }

    fn draw(&self, _: &F, ctx: &mut ggez::Context) -> GameResult<()> {
        let font = ctx.default_font.clone();
        let text = format!(
            "Player: {:?} {:?} {:?}- Enemy: {:?}",
            self.player.hitzone, self.player.attack, self.player.stun_status, self.enemy.state
        );
        Text::new(ctx, &text, &font)?.draw(ctx, Point2::new(100.0, 100.0), 0.0)?;
        Ok(())
    }
}
