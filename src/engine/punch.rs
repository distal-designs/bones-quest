use std::collections::HashMap;

use rlua;


pub enum MainCharacter {
    Bones,
    Beat,
    Cattlebones,
}


pub enum Vulnerability {
    Block,
    Hit,
    Whiff,
}


pub struct EnemyStateVulnerability {
    left: Vulnerability,
    right: Vulnerability,
    parry: bool,
}


pub enum Hitzone {
    Left,
    Right,
    Duck,
    Stand,
}


pub struct EnemyHitzones {
    left: Hitzone,
    right: Hitzone,
    duck: Hitzone,
    stand: Hitzone,
}


pub enum EnemyStateTransition<'lua> {
    Static(String),
    Dynamic(rlua::Function<'lua>),
}


pub struct EnemyStateDefinition<'lua> {
    frames: u8,
    vulnerability: EnemyStateVulnerability,
    hitzones: EnemyHitzones,
    on_hitting_player: EnemyStateTransition<'lua>,
    on_getting_hit: EnemyStateTransition<'lua>,
    on_block: EnemyStateTransition<'lua>,
    on_end: EnemyStateTransition<'lua>
}


pub struct EnemyDefinition<'lua> {
    name: String,
    id: String,
    fights: MainCharacter,
    default_state: String,
    states: HashMap<String, EnemyStateDefinition<'lua>>,
}
