#[macro_use]
extern crate flu;

use flu::ffi;

#[test]
fn ffi_load() {
    unsafe {
        let lua = ffi::luaL_newstate();

        ffi::luaL_loadstring(lua, c_str!("a = 5; b = 10"));
        ffi::lua_pcall(lua, 0, ffi::LUA_MULTRET, 0);

        ffi::lua_getglobal(lua, c_str!("a"));
        ffi::lua_getglobal(lua, c_str!("b"));

        let a = ffi::lua_tonumber(lua, -2);
        assert_eq!(a, 5f64);
        
        let b = ffi::lua_tonumber(lua, -1);
        assert_eq!(b, 10f64);

        ffi::lua_close(lua);
    }
}
