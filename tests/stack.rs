#[macro_use]
extern crate flu;

use flu::LuaContext;
use flu::LuaValue;
use flu::LuaRef;

use flu::Table;

use flu::ffi;

#[test]
fn flu_stack_size() {
    let cxt = LuaContext::new();

    push!(&cxt, true, false, true, false);

    assert_eq!(cxt.size(), 4);
}

#[test]
fn flu_stack_read_int() {
    let cxt = LuaContext::new();

    push!(&cxt, 42i8);
    assert_eq!(cxt.pop::<i8>(), 42i8);

    push!(&cxt, 16i16);
    assert_eq!(cxt.pop::<i16>(), 16i16);

    push!(&cxt, 101i32);
    assert_eq!(cxt.pop::<i32>(), 101i32);
}

#[test]
fn flu_stack_read_num() {
    let cxt = LuaContext::new();

    push!(&cxt, 42f64);
    assert_eq!(cxt.pop::<f64>(), 42f64);

    push!(&cxt, 16f32);
    assert_eq!(cxt.pop::<f32>(), 16f32);

    push!(&cxt, 101f32);
    assert_eq!(cxt.pop::<f64>(), 101f64);

    push!(&cxt, 99f64);
    assert_eq!(cxt.pop::<f32>(), 99f32);
}

#[test]
fn flu_stack_read_string() {
    let cxt = LuaContext::new();

    push!(&cxt, "Hello world!", "Hello rust!".to_string());

    assert_eq!(cxt.pop::<String>(), "Hello rust!");
    assert_eq!(cxt.pop::<&str>(), "Hello world!");
}

#[test]
fn flu_stack_read_optional() {
    let cxt = LuaContext::new();

    push!(&cxt, "Hello world!", flu::nil);

    assert_eq!(cxt.pop::<Option<String>>(), None);
    assert_eq!(cxt.pop::<Option<&str>>(), Some("Hello world!"));

    /*push!(&cxt, flu::nil, 5f64, flu::nil);
    assert_eq!(cxt.pop::<(Option<f64>, Option<f64>, Option<f64>)>(), (None, Some(5f64), None));*/
}

#[test]
fn flu_stack_read_value() {
    let cxt = LuaContext::new();

    push!(&cxt, flu::nil, 45f32, "Hello world!");

    assert_eq!(cxt.remove::<LuaValue>(1), LuaValue::Nil);
    assert_eq!(cxt.remove::<LuaValue>(1), LuaValue::Number(45f64));
    assert_eq!(cxt.remove::<LuaValue>(1), LuaValue::String("Hello world!"));
}

#[test]
fn flu_stack_read_table() {
    let cxt = LuaContext::new();

    let table = Table::new(&cxt);

    table.set("alongkeyinatable", flu::nil);
    table.set(0, 5f64);
    table.set("akey", "flim-flam");

    assert_eq!(table.get::<Option<i32>, _>("alongkeyinatable"), None);
    assert_eq!(table.get::<f64, _>(0), 5f64);
    assert_eq!(table.get::<&str, _>("akey"), "flim-flam");
}

#[test]
fn flu_stack_read_ref() {
    let cxt = LuaContext::new();

    push!(&cxt, "Hello world!");

    {
        assert_eq!(cxt.size(), 1);

        let r = cxt.pop::<LuaRef>();

        assert_eq!(cxt.size(), 0);

        cxt.push(r);

        assert_eq!(cxt.pop::<&str>(), "Hello world!");
    }
}

/*#[test]
fn flu_stack_read_tuple() {
    let cxt = LuaContext::new();

    push!(&cxt, 1f64, 2f64);
    assert_eq!(cxt.pop::<(f64, f64)>(), (1f64, 2f64));

    push!(&cxt, "lululua", flu::nil);
    assert_eq!(cxt.pop::<(String, Option<&str>)>(), ("lululua".to_string(), None));

    push!(&cxt, true, 303f32, 604f32);
    assert_eq!(cxt.pop::<(bool, (f32, f32))>(), (true, (303f32, 604f32)));
}*/

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
