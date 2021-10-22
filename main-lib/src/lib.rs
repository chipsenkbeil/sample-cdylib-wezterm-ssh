use mlua::prelude::*;

#[mlua::lua_module]
fn test_lib(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;

    helper_lib::connect_to_server();
    println!("Connected!");

    Ok(exports)
}

#[test]
fn test_me() {
    println!("test!");
}
