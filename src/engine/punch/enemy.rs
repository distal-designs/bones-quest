use rlua::Value::Nil;

use super::Player;
use super::scripting::EnemyStateTransition::*;
use super::scripting::{
    EnemyDefinition, EnemyStateDefinition, EnemyStateTransition, Hitzone, PlayerAttack,
    Vulnerability,
};


#[derive(Debug)]
pub struct Enemy {
    pub frame: u8,
    pub state: String,
}


impl Enemy {
    pub fn update(&mut self, enemy_definition: &EnemyDefinition, player: &Player) {
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
