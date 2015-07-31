extern crate flu;

use flu::ffi;

#[test]
fn stack_size() {
    unsafe {
        let lua = ffi::luaL_newstate();

        ffi::lua_pushboolean(lua, 1);
        ffi::lua_pushboolean(lua, 0);
        ffi::lua_pushboolean(lua, 1);
        ffi::lua_pushboolean(lua, 0);

        assert_eq!(ffi::lua_checkstack(lua, 4), 1);

        ffi::lua_close(lua);
    }
}

#[test]
fn stack_string() {
    unsafe {
        let lua = ffi::luaL_newstate();
        
        ffi::lua_pushliteral(lua, "hello stack!");
        assert_eq!(ffi::lua_strlen(lua, 1), 12);

        ffi::lua_pushliteral(lua, "hello \0zero!");
        assert_eq!(ffi::lua_strlen(lua, 2), 12);

        ffi::lua_close(lua);
    }
}
