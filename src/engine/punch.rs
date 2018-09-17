use std::collections::HashMap;

use rlua::{self, FromLua, Lua, Table, Value};

#[derive(Clone, Debug, EnumString)]
pub enum MainCharacter {
    #[strum(serialize="BONES")]
    Bones,
    #[strum(serialize="BEAT")]
    Beat,
    #[strum(serialize="CATTLEBONES")]
    Cattlebones,
}

impl<'lua> FromLua<'lua> for MainCharacter {
    fn from_lua(value: Value, lua: &Lua) -> rlua::Result<Self> {
        let s: String = lua.unpack(value)?;
        s.parse::<MainCharacter>()
            .map_err(|_| rlua::Error::FromLuaConversionError {
                from: "String",
                to: "MainCharacter",
                message: None,
            })
    }
}


#[derive(Clone, Debug, EnumString)]
pub enum Vulnerability {
    #[strum(serialize="BLOCK")]
    Block,
    #[strum(serialize="HIT")]
    Hit,
    #[strum(serialize="WHIFF")]
    Whiff,
}

impl<'lua> FromLua<'lua> for Vulnerability {
    fn from_lua(value: Value, lua: &Lua) -> rlua::Result<Self> {
        let s: String = lua.unpack(value)?;
        s.parse::<Vulnerability>()
            .map_err(|_| rlua::Error::FromLuaConversionError {
                from: "String",
                to: "Vulnerability",
                message: None,
            })
    }
}


#[derive(Clone, Debug)]
pub struct EnemyStateVulnerability {
    left: Vulnerability,
    right: Vulnerability,
    parry: bool,
}

impl<'lua> FromLua<'lua> for EnemyStateVulnerability {
    fn from_lua(value: Value, lua: &Lua) -> rlua::Result<Self> {
        let t: Table = lua.unpack(value)?;
        Ok(EnemyStateVulnerability {
            left: t.get("left")?,
            right: t.get("right")?,
            parry: t.get("parry")?,
        })
    }
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
    left: bool,
    right: bool,
    duck: bool,
    stand: bool,
}

impl<'lua> FromLua<'lua> for EnemyHitzones {
    fn from_lua(value: Value, lua: &Lua) -> rlua::Result<Self> {
        let t: Table = lua.unpack(value)?;
        Ok(EnemyHitzones {
            left: t.get("left")?,
            right: t.get("right")?,
            duck: t.get("duck")?,
            stand: t.get("stand")?,
        })
    }
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

impl<'lua> FromLua<'lua> for EnemyDefinition<'lua> {
    fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> rlua::Result<Self> {
        let root: Table = lua.unpack(value)?;
        Ok(EnemyDefinition {
            name: root.get("name")?,
            id: root.get("id")?,
            default_state: root.get("default_state")?,
            fights: root.get("fights")?,
            states: HashMap::new(),
        })
    }
}
