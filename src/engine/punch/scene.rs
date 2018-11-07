use ggez::event::Keycode::{Left, Right, Up, A, D, S};
use ggez::graphics::{Drawable, Point2, Text};
use ggez::{self, GameResult};
use rlua::Lua;
use rlua::Value::Nil;

use super::scripting::EnemyStateTransition::*;
use super::scripting::{
    EnemyDefinition, EnemyStateDefinition, EnemyStateTransition, Hitzone, PlayerAttack,
    Vulnerability,
};
use engine;
use engine::input::Input;
use engine::lua::LuaExt;

#[derive(Debug)]
pub struct Enemy {
    frame: u8,
    state: String,
}

#[derive(Debug)]
pub struct Player {
    pub hitzone: Hitzone,
    pub attack: PlayerAttack,
    pub parrying: bool,
}

pub struct Scene {
    lua: Lua,
    enemy_id: String,
    enemy: Enemy,
    player: Player,
}

impl Enemy {
    fn update(&mut self, enemy_definition: &EnemyDefinition, player: &Player) {
        let state = enemy_definition.states.get(&self.state).expect(&format!(
            "No state in enemy definition called '{}'",
            &self.state
        ));

        if Self::was_parried_by_player(state, &player) {
            self.transition(&state.on_parry);
        } else if Self::did_block_player(state, &player.attack) {
            self.transition(&state.on_block);
        } else if Self::was_hit_by_player(state, &player.attack) {
            self.transition(&state.on_getting_hit);
        } else if Self::did_hit_player(state, &player.hitzone) {
            self.transition(&state.after_hitting_player);
        } else if self.frame >= state.frames {
            self.transition(&state.on_end);
        } else {
            self.frame += 1;
        }
    }

    fn transition(&mut self, transition_definition: &EnemyStateTransition) {
        self.frame = 1;
        self.state = match transition_definition {
            Static(ref new_state) => new_state.to_owned(),
            Dynamic(ref transition_fn) => transition_fn.call(Nil).unwrap(),
        };
    }

    fn was_parried_by_player(state: &EnemyStateDefinition, player: &Player) -> bool {
        state.vulnerability.parry && player.parrying
    }

    fn was_hit_by_player(state: &EnemyStateDefinition, attack: &PlayerAttack) -> bool {
        match (
            &state.vulnerability.left,
            &state.vulnerability.right,
            attack,
        ) {
            (Vulnerability::Hit, _, PlayerAttack::Left) => true,
            (_, Vulnerability::Hit, PlayerAttack::Right) => true,
            (_, _, _) => false,
        }
    }

    fn did_hit_player(state: &EnemyStateDefinition, player_zone: &Hitzone) -> bool {
        match player_zone {
            Hitzone::Left => state.hitzones.left,
            Hitzone::Right => state.hitzones.right,
            Hitzone::Duck => state.hitzones.duck,
            Hitzone::Stand => state.hitzones.stand,
        }
    }

    fn did_block_player(state: &EnemyStateDefinition, attack: &PlayerAttack) -> bool {
        match (
            &state.vulnerability.left,
            &state.vulnerability.right,
            attack,
        ) {
            (Vulnerability::Block, _, PlayerAttack::Left) => true,
            (_, Vulnerability::Block, PlayerAttack::Right) => true,
            (_, _, _) => false,
        }
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
        };

        self.attack = if self.hitzone != Hitzone::Stand {
            PlayerAttack::None
        } else if input.current_input.contains(&Left) {
            PlayerAttack::Left
        } else if input.current_input.contains(&Right) {
            PlayerAttack::Right
        } else {
            PlayerAttack::None
        };

        self.parrying = self.hitzone == Hitzone::Stand
            && self.attack == PlayerAttack::None
            && input.current_input.contains(&Up);
    }
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

impl Default for Player {
    fn default() -> Self {
        Player {
            hitzone: Hitzone::Stand,
            attack: PlayerAttack::None,
            parrying: true,
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

    fn draw(&self, _: &F, ctx: &mut ggez::Context) -> GameResult<()> {
        let font = ctx.default_font.clone();
        let text = format!(
            "Player: {:?} {:?} - Enemy: {:?}",
            self.player.hitzone, self.player.attack, self.enemy.state
        );
        Text::new(ctx, &text, &font)?.draw(ctx, Point2::new(100.0, 100.0), 0.0)?;
        Ok(())
    }
}
