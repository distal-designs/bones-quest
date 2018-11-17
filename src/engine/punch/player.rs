use ggez::event::Keycode::{Left, Right, Up, A, D, S};

use super::scripting::{EnemyHitzones, Hitzone, PlayerAttack};
use engine::input::Input;


#[derive(Debug)]
pub struct Player {
    pub hitzone: Hitzone,
    pub attack: PlayerAttack,
    pub parrying: bool,
    pub stun_status: StunStatus,
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
            && input.current_input.contains(&Up);
    }

    pub fn handle_collisions(&mut self, hitzones: &EnemyHitzones) {
        const STUN_DURATION: u8 = 10;
        if let StunStatus::Stunned(_) = self.stun_status { return };

        self.stun_status = match self.hitzone {
            Hitzone::Left if hitzones.left => StunStatus::Stunned(STUN_DURATION),
            Hitzone::Right if hitzones.right => StunStatus::Stunned(STUN_DURATION),
            Hitzone::Stand if hitzones.stand => StunStatus::Stunned(STUN_DURATION),
            Hitzone::Duck if hitzones.duck => StunStatus::Stunned(STUN_DURATION),
            _ => StunStatus::Normal,
        }
    }

    fn get_hit(&mut self) {
        const STUN_DURATION: u8 = 10;
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
