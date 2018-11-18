use std::collections::HashMap;

use rlua::{self, FromLua, Lua, Table, Value};


#[derive(Clone, Debug)]
pub struct EnemyStateVulnerability {
    pub left: Vulnerability,
    pub right: Vulnerability,
    pub parry: bool,
}

#[derive(Clone, Debug)]
pub struct EnemyHitzones {
    pub left: bool,
    pub right: bool,
    pub duck: bool,
    pub stand: bool,
}

#[derive(Clone, Debug)]
pub struct EnemyStateDefinition<'lua> {
    pub frames: u8,
    pub vulnerability: EnemyStateVulnerability,
    pub hitzones: EnemyHitzones,
    pub after_hitting_player: EnemyStateTransition<'lua>,
    pub on_getting_hit: EnemyStateTransition<'lua>,
    pub on_block: EnemyStateTransition<'lua>,
    pub on_parry: EnemyStateTransition<'lua>,
    pub on_end: EnemyStateTransition<'lua>
}

#[derive(Clone, Debug)]
pub struct EnemyDefinition<'lua> {
    pub name: String,
    pub id: String,
    pub fights: MainCharacter,
    pub initial_state: String,
    pub states: HashMap<String, EnemyStateDefinition<'lua>>,
}


#[derive(Clone, Debug)]
pub enum EnemyStateTransition<'lua> {
    Static(String),
    Dynamic(rlua::Function<'lua>),
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

#[derive(Clone, Debug, EnumString)]
pub enum MainCharacter {
    #[strum(serialize="BONES")]
    Bones,
    #[strum(serialize="BEAT")]
    Beat,
    #[strum(serialize="CATTLEBONES")]
    Cattlebones,
}


impl<'lua> EnemyDefinition<'lua> {
    pub fn load(lua: &'lua Lua, enemy_id: &str) -> EnemyDefinition<'lua> {
        let loader = format!("return require 'src.game.enemies.{}'", enemy_id);
        lua.exec(&loader, Some("Loading enemy definition")).unwrap()
    }

    pub fn state(&self, name: &str) -> &EnemyStateDefinition<'lua> {
        self.states.get(name).unwrap_or_else(|| panic!(
            "No state in enemy definition called '{}'",
            name
        ))
    }
}


impl<'lua> FromLua<'lua> for EnemyStateVulnerability {
    fn from_lua(value: Value, lua: &Lua) -> rlua::Result<Self> {
        let t: Table = lua.unpack(value)?;
        Ok(Self {
            left: t.get("left")?,
            right: t.get("right")?,
            parry: t.get("parry")?,
        })
    }
}

impl<'lua> FromLua<'lua> for MainCharacter {
    fn from_lua(value: Value, lua: &Lua) -> rlua::Result<Self> {
        let s: String = lua.unpack(value)?;
        s.parse::<Self>()
            .map_err(|_| rlua::Error::FromLuaConversionError {
                from: "String",
                to: "MainCharacter",
                message: None,
            })
    }
}

impl<'lua> FromLua<'lua> for Vulnerability {
    fn from_lua(value: Value, lua: &Lua) -> rlua::Result<Self> {
        let s: String = lua.unpack(value)?;
        s.parse::<Self>()
            .map_err(|_| rlua::Error::FromLuaConversionError {
                from: "String",
                to: "Vulnerability",
                message: None,
            })
    }
}

impl<'lua> FromLua<'lua> for EnemyHitzones {
    fn from_lua(value: Value, lua: &Lua) -> rlua::Result<Self> {
        let t: Table = lua.unpack(value)?;
        Ok(Self {
            left: t.get("left")?,
            right: t.get("right")?,
            duck: t.get("duck")?,
            stand: t.get("stand")?,
        })
    }
}

impl<'lua> FromLua<'lua> for EnemyStateTransition<'lua> {
    fn from_lua(value: Value<'lua>, _lua: &'lua Lua) -> rlua::Result<Self> {
        match value {
            rlua::Value::Function(func) => Ok(EnemyStateTransition::Dynamic(func)),
            rlua::Value::String(s) =>
                Ok(EnemyStateTransition::Static(s.to_str()?.to_owned())),
            _ => Err(rlua::Error::FromLuaConversionError {
                from: "Function or String",
                to: "EnemyStateTransition",
                message: None,
            }),
        }
    }
}

impl<'lua> FromLua<'lua> for EnemyStateDefinition<'lua> {
    fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> rlua::Result<Self> {
        let state: Table = lua.unpack(value)?;
        Ok(EnemyStateDefinition {
            frames: state.get("frames")?,
            vulnerability: state.get("vulnerability")?,
            hitzones: state.get("hitzones")?,
            after_hitting_player: state.get("after_hitting_player")?,
            on_getting_hit: state.get("on_getting_hit")?,
            on_block: state.get("on_block")?,
            on_parry: state.get("on_parry")?,
            on_end: state.get("on_end")?,
        })
    }
}

impl<'lua> FromLua<'lua> for EnemyDefinition<'lua> {
    fn from_lua(value: Value<'lua>, lua: &'lua Lua) -> rlua::Result<Self> {
        let root: Table = lua.unpack(value)?;
        Ok(EnemyDefinition {
            name: root.get("name")?,
            id: root.get("id")?,
            initial_state: root.get("initial_state")?,
            fights: root.get("fights")?,
            states: root.get("states")?,
        })
    }
}
