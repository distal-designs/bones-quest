use ggez::event::Keycode::{Left, Right, W, A, D, S};

use super::scripting::{EnemyHitzones};
use engine::input::Input;

type Frames = u8;

#[derive(Debug)]
pub struct Player {
    state: PlayerState,
}

#[derive(Debug)]
pub enum PlayerState {
    Stunned(Frames),
    Dodge(Frames, DodgeDirection),
    Stand(PlayerAction),
}

#[derive(Clone, Copy, Debug)]
pub enum DodgeDirection {
    Left,
    Right,
    Duck,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Hitzone {
    Left,
    Right,
    Duck,
    Stand,
}

#[derive(Clone, Copy, Debug)]
pub enum AttackDirection {
    Left,
    Right,
}

#[derive(Debug)]
pub enum PlayerAction {
    Neutral,
    Attack(Frames, AttackDirection),
    Parry(Frames),
}

impl Player {
    pub fn update(&mut self, input: &Input) {
        use self::PlayerState::*;
        use self::PlayerAction::*;
        let recently_pressed = input.pressed();
        self.state = match self.state {
            Stunned(1) | Dodge(1, _) | Stand(Attack(1, _)) | Stand(Parry(1))
                => Stand(Neutral),
            Stunned(x)
                => Stunned(x - 1),
            Dodge(x, d)
                => Dodge(x - 1, d),
            Stand(Attack(x, a))
                => Stand(Attack(x - 1, a)),
            Stand(Parry(x))
                => Stand(Parry(x - 1)),
            Stand(Neutral)
                => vec![
                    (&W, Stand(Parry(7))),
                    (&A, Dodge(30, DodgeDirection::Left)),
                    (&D, Dodge(30, DodgeDirection::Right)),
                    (&S, Dodge(20, DodgeDirection::Duck)),
                    (&Left, Stand(Attack(14, AttackDirection::Left))),
                    (&Right, Stand(Attack(14, AttackDirection::Right)))
                   ].into_iter()
                    .find(|(key, _)| recently_pressed.contains(key))
                    .map_or(Stand(Neutral), |(_, new_state)| new_state)
        };
    }

    pub fn handle_collisions(&mut self, hitzones: &EnemyHitzones) {
        match self.state {
            PlayerState::Dodge(_, DodgeDirection::Left) if hitzones.left => self.get_hit(),
            PlayerState::Dodge(_, DodgeDirection::Right) if hitzones.right => self.get_hit(),
            PlayerState::Dodge(_, DodgeDirection::Duck) if hitzones.duck => self.get_hit(),
            PlayerState::Stand(_) if hitzones.stand => self.get_hit(),
            _ => {}
        }
    }

    pub fn is_parrying(&self) -> bool {
        if let PlayerState::Stand(PlayerAction::Parry(_)) = self.state {
            true
        } else {
            false
        }
    }

    pub fn attack_direction(&self) -> Option<AttackDirection> {
        match self.state {
            PlayerState::Stand(PlayerAction::Attack(_, ad)) => Some(ad),
            _ => None,
        }
    }

    pub fn hitzone(&self) -> Option<Hitzone> {
        match self.state {
            PlayerState::Stand(_) => Some(Hitzone::Stand),
            PlayerState::Dodge(_, DodgeDirection::Left) => Some(Hitzone::Left),
            PlayerState::Dodge(_, DodgeDirection::Right) => Some(Hitzone::Right),
            PlayerState::Dodge(_, DodgeDirection::Duck) => Some(Hitzone::Duck),
            _ => None,
        }
    }

    fn get_hit(&mut self) {
        self.state = PlayerState::Stunned(30);
    }
}


impl Default for Player {
    fn default() -> Self {
        Player {
            state: PlayerState::Stand(PlayerAction::Neutral),
        }
    }
}
