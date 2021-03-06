use ggez::graphics::{Drawable, Point2, Text};
use ggez::{self, GameResult};
use rlua::Lua;

use super::Player;
use super::Enemy;
use super::scripting::EnemyDefinition;
use crate::engine;
use crate::engine::input::Input;
use crate::engine::lua::Ext;

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
        let state = EnemyDefinition::load(&lua, &enemy_id).initial_state;
        Self {
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
        let player_text = format!("Player: {:?}", self.player);
        Text::new(ctx, &player_text, &font)?.draw(ctx, Point2::new(0.0, 100.0), 0.0)?;
        let enemy_text = format!("Enemy: {:?}", self.enemy.state);
        Text::new(ctx, &enemy_text, &font)?.draw(ctx, Point2::new(0.0, 150.0), 0.0)?;
        Ok(())
    }
}
