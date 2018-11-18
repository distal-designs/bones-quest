use rlua::Value::Nil;

use super::Player;
use super::player::{AttackDirection, Hitzone};
use super::scripting::{EnemyStateDefinition, EnemyStateTransition, Vulnerability};


#[derive(Debug)]
pub struct Enemy {
    pub frame: u8,
    pub state: String,
}


impl Enemy {
    pub fn update(&mut self, state: &EnemyStateDefinition, player: &Player) {
        if Self::was_parried_by_player(state, &player) {
            self.transition(&state.on_parry);
        } else if Self::did_block_player(state, &player) {
            self.transition(&state.on_block);
        } else if Self::was_hit_by_player(state, &player) {
            self.transition(&state.on_getting_hit);
        } else if Self::did_hit_player(state, &player) {
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
            EnemyStateTransition::Static(ref new_state) => new_state.to_owned(),
            EnemyStateTransition::Dynamic(ref transition_fn) => transition_fn.call(Nil).unwrap(),
        };
    }

    fn was_parried_by_player(state: &EnemyStateDefinition, player: &Player) -> bool {
        state.vulnerability.parry && player.is_parrying()
    }

    fn was_hit_by_player(state: &EnemyStateDefinition, player: &Player) -> bool {
        match (
            &state.vulnerability.left,
            &state.vulnerability.right,
            player.attack_direction(),
        ) {
            (Vulnerability::Hit, _, Some(AttackDirection::Left)) => true,
            (_, Vulnerability::Hit, Some(AttackDirection::Right)) => true,
            _ => false,
        }
    }

    fn did_hit_player(state: &EnemyStateDefinition, player: &Player) -> bool {
        match player.hitzone() {
            Some(Hitzone::Left) => state.hitzones.left,
            Some(Hitzone::Right) => state.hitzones.right,
            Some(Hitzone::Duck) => state.hitzones.duck,
            Some(Hitzone::Stand) => state.hitzones.stand,
            None => false,
        }
    }

    fn did_block_player(state: &EnemyStateDefinition, player: &Player) -> bool {
        match (
            &state.vulnerability.left,
            &state.vulnerability.right,
            player.attack_direction(),
        ) {
            (Vulnerability::Block, _, Some(AttackDirection::Left)) => true,
            (_, Vulnerability::Block, Some(AttackDirection::Right)) => true,
            (_, _, _) => false,
        }
    }
}
