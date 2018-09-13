use std::env;

use rlua::{Lua, Value};


pub trait LuaExt {
    fn new_with_path() -> Lua;
}


impl LuaExt for Lua {
    fn new_with_path() -> Lua {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let script = format!("package.path = '{}/?.lua;' .. package.path", manifest_dir);
        let lua = Lua::new();
        lua.eval::<Value>(&script, Some(&"package.path Initialization")).unwrap();
        return lua;
    }
}
