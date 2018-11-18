use std::env;

use rlua::{Lua, Value};

pub trait Ext {
    fn new_with_path() -> Lua;
}

impl Ext for Lua {
    fn new_with_path() -> Lua {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let script = format!("package.path = '{}/?.lua;' .. package.path", manifest_dir);
        let lua = Self::new();
        lua.eval::<Value>(&script, Some(&"package.path Initialization")).unwrap();
        lua.eval::<Value>(&"math.randomseed(os.time())", None).unwrap();
        lua
    }
}
