use ggez::event::Keycode::{Left, Right, W, A, D, S};

use super::scripting::{EnemyHitzones, Hitzone, PlayerAttack};
use engine::input::Input;

type Frames = u8;

#[derive(Debug)]
pub enum PlayerState {
    Stunned(Frames),
    Dodge(Frames, DodgeDirection),
    Stand(PlayerAction),
}

#[derive(Debug)]
pub enum DodgeDirection {
    Left,
    Right,
    Duck,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Player {
    state: PlayerState,
}

#[derive(Debug)]
pub enum StunStatus {
    Normal,
    Stunned(u8),
}


impl Player {
    pub fn update(&mut self, input: &Input) {
        match self.stun_status {
            StunStatus::Stunned(0) => {
                self.stun_status = StunStatus::Normal;
            }
            StunStatus::Stunned(x) => {
                self.stun_status = StunStatus::Stunned(x - 1);
                return;
            }
            StunStatus::Normal => {}
        };

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
            && input.current_input.contains(&W);
    }

    pub fn handle_collisions(&mut self, hitzones: &EnemyHitzones) {
        if let StunStatus::Stunned(_) = self.stun_status { return };

        match self.hitzone {
            Hitzone::Left if hitzones.left => self.get_hit(),
            Hitzone::Right if hitzones.right => self.get_hit(),
            Hitzone::Stand if hitzones.stand => self.get_hit(),
            Hitzone::Duck if hitzones.duck => self.get_hit(),
            _ => {}
        };
    }

    fn get_hit(&mut self) {
        const STUN_DURATION: u8 = 30;
        self.stun_status = StunStatus::Stunned(STUN_DURATION);
        self.attack = PlayerAttack::None;
        self.parrying = false;
        self.hitzone = Hitzone::Stand;
    }
}


impl Default for Player {
    fn default() -> Self {
        Player {
            hitzone: Hitzone::Stand,
            attack: PlayerAttack::None,
            parrying: true,
            stun_status: StunStatus::Normal,
        }
    }
}
