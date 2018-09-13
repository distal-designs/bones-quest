use std::collections::HashMap;

use rlua;


#[derive(Clone, Debug)]
pub enum MainCharacter {
    Bones,
    Beat,
    Cattlebones,
}


#[derive(Clone, Debug)]
pub enum Vulnerability {
    Block,
    Hit,
    Whiff,
}


#[derive(Clone, Debug)]
pub struct EnemyStateVulnerability {
    left: Vulnerability,
    right: Vulnerability,
    parry: bool,
}


#[derive(Clone, Debug)]
pub enum Hitzone {
    Left,
    Right,
    Duck,
    Stand,
}


#[derive(Clone, Debug)]
pub struct EnemyHitzones {
    left: Hitzone,
    right: Hitzone,
    duck: Hitzone,
    stand: Hitzone,
}


#[derive(Clone, Debug)]
pub enum EnemyStateTransition<'lua> {
    Static(String),
    Dynamic(rlua::Function<'lua>),
}


#[derive(Clone, Debug)]
pub struct EnemyStateDefinition<'lua> {
    frames: u8,
    vulnerability: EnemyStateVulnerability,
    hitzones: EnemyHitzones,
    on_hitting_player: EnemyStateTransition<'lua>,
    on_getting_hit: EnemyStateTransition<'lua>,
    on_block: EnemyStateTransition<'lua>,
    on_end: EnemyStateTransition<'lua>
}


#[derive(Clone, Debug)]
pub struct EnemyDefinition<'lua> {
    name: String,
    id: String,
    fights: MainCharacter,
    default_state: String,
    states: HashMap<String, EnemyStateDefinition<'lua>>,
}