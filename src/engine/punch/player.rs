use ggez::event::Keycode::{Left, Right, W, A, D, S};

use super::scripting::{EnemyHitzones};
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
        use self::PlayerState::*;
        use self::PlayerAction::*;
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
                    (&S, Dodge(30, DodgeDirection::Right)),
                    (&D, Dodge(30, DodgeDirection::Duck)),
                    (&Left, Stand(Attack(4, AttackDirection::Left))),
                    (&Right, Stand(Attack(4, AttackDirection::Right)))
                   ].into_iter()
                    .find(|(key, _)| input.current_input.contains(key))
                    .map_or(Stand(Neutral), |(_, new_state)| new_state)
        };
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
