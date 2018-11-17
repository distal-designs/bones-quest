use ggez::event::Keycode::{Left, Right, Up, A, D, S};

use super::scripting::{Hitzone, PlayerAttack};
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
