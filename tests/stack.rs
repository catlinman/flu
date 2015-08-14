#[macro_use]
extern crate flu;

use flu::ffi;

#[test]
fn flu_stack_size() {
    let cxt = flu::LuaContext::new();

    push!(&cxt, true, false, true, false);

    assert_eq!(cxt.size(), 4);
}

#[test]
fn flu_stack_read_int() {
    let cxt = flu::LuaContext::new();

    push!(&cxt, 42i8);
    assert_eq!(cxt.pop_front::<i8>(), 42i8);

    push!(&cxt, 16i16);
    assert_eq!(cxt.pop_front::<i16>(), 16i16);

    push!(&cxt, 101i32);
    assert_eq!(cxt.pop_front::<i32>(), 101i32);
}

#[test]
fn flu_stack_read_num() {
    let cxt = flu::LuaContext::new();

    push!(&cxt, 42f64);
    assert_eq!(cxt.pop_front::<f64>(), 42f64);

    push!(&cxt, 16f32);
    assert_eq!(cxt.pop_front::<f32>(), 16f32);

    push!(&cxt, 101f32);
    assert_eq!(cxt.pop_front::<f64>(), 101f64);

    push!(&cxt, 99f64);
    assert_eq!(cxt.pop_front::<f32>(), 99f32);
}

#[test]
fn flu_stack_read_string() {
    let cxt = flu::LuaContext::new();

    push!(&cxt, "Hello world!", "Hello rust!".to_string());

    assert_eq!(cxt.pop_front::<String>(), "Hello rust!");
    assert_eq!(cxt.pop_front::<&str>(), "Hello world!");
}

#[test]
fn flu_stack_read_optional() {
    let cxt = flu::LuaContext::new();

    push!(&cxt, "Hello world!", flu::nil);

    assert_eq!(cxt.pop_front::<Option<String>>(), None);
    assert_eq!(cxt.pop_front::<Option<&str>>(), Some("Hello world!"));

    push!(&cxt, flu::nil, 5f64, flu::nil);
    assert_eq!(cxt.pop_front::<(Option<f64>, Option<f64>, Option<f64>)>(), (None, Some(5f64), None));
}

#[test]
fn flu_stack_read_ref() {
    let cxt = flu::LuaContext::new();

    push!(&cxt, "Hello world!");

    {
        let r = cxt.pop_front::<flu::LuaRef<&str>>();
        cxt.push(r);

        assert_eq!(cxt.pop_front::<&str>(), "Hello world!");
    }
}

#[test]
fn flu_stack_read_tuple() {
    let cxt = flu::LuaContext::new();

    push!(&cxt, 1f64, 2f64);
    assert_eq!(cxt.pop_front::<(f64, f64)>(), (1f64, 2f64));

    push!(&cxt, "lululua", flu::nil);
    assert_eq!(cxt.pop_front::<(String, Option<&str>)>(), ("lululua".to_string(), None));

    push!(&cxt, true, 303f32, 604f32);
    assert_eq!(cxt.pop_front::<(bool, (f32, f32))>(), (true, (303f32, 604f32)));
}

#[test]
fn ffi_stack_read_int() {
    unsafe {
        let lua = ffi::luaL_newstate();

        ffi::lua_pushinteger(lua, 1);
        ffi::lua_pushinteger(lua, 2);
        ffi::lua_pushinteger(lua, 3);

        assert_eq!(ffi::lua_tointeger(lua, -1), 3);
        assert_eq!(ffi::lua_tointeger(lua, -2), 2);
        assert_eq!(ffi::lua_tointeger(lua, -3), 1);

        ffi::lua_close(lua);
    }
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
