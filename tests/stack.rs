#[macro_use]
extern crate flu;

use flu::ffi;

#[test]
fn flu_stack_size() {
    let mut cxt = flu::LuaContext::new();

    push!(&mut cxt, true, false, true, false);

    assert_eq!(cxt.size(), 4);
}

#[test]
fn flu_stack_read_num() {
    let mut cxt = flu::LuaContext::new();

    push!(&mut cxt, 42f64);

    assert_eq!(cxt.read::<f64>(-1), 42f64);
}

#[test]
fn flu_stack_read_string() {
    let mut cxt = flu::LuaContext::new();

    push!(&mut cxt, "Hello world!", "Hello rust!".to_string());

    assert_eq!(cxt.read::<&str>(1), "Hello world!");
    assert_eq!(cxt.read::<String>(2), "Hello rust!");
}

#[test]
fn ffi_stack_size() {
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
fn ffi_stack_string() {
    unsafe {
        let lua = ffi::luaL_newstate();
        
        ffi::lua_pushliteral(lua, "hello stack!");
        assert_eq!(ffi::lua_strlen(lua, 1), 12);

        ffi::lua_pushliteral(lua, "hello \0zero!");
        assert_eq!(ffi::lua_strlen(lua, 2), 12);

        ffi::lua_close(lua);
    }
}
