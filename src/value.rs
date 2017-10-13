use Context;
use LuaRef;
use Function;
use Table;
use ffi;
use nil;

use stack::Read;
use stack::Size;

#[derive(Debug, PartialEq)]
pub enum LuaValue<'a> {
    Number(f64),
    String(&'a str),
    Bool(bool),
    Table(Table<'a>),
    Function(Function<'a>),
    /*Userdata,
    Thread,*/
    Nil,
    None,
}

impl<'a> Read<'a> for LuaValue<'a> {
    fn read(ctx: &'a Context, idx: i32) -> Self {
        unsafe {
            match ffi::lua_type(ctx.handle, idx) {
                ffi::LUA_TNONE => LuaValue::None,
                ffi::LUA_TNIL => LuaValue::Nil,
                ffi::LUA_TBOOLEAN => LuaValue::Bool(bool::read(ctx, idx)),
                ffi::LUA_TLIGHTUSERDATA => unimplemented!(),
                ffi::LUA_TNUMBER => LuaValue::Number(f64::read(ctx, idx)),
                ffi::LUA_TSTRING => LuaValue::String(<&str>::read(ctx, idx)),
                ffi::LUA_TTABLE => LuaValue::Table(Table { ctx: ctx, ptr: <LuaRef>::read(ctx, idx) }),
                ffi::LUA_TFUNCTION => LuaValue::Function(Function::read(ctx, idx)),
                ffi::LUA_TUSERDATA => unimplemented!(),
                ffi::LUA_TTHREAD => unimplemented!(),
                _ => panic!("yahallo"),
            }
        }
    }

    fn check(ctx: &'a Context, idx: i32) -> bool {
        true
    }
}

impl<'a> Size for LuaValue<'a> {
    fn size() -> i32 {
        1
    }
}

#[test]
fn read_value() {
    let ctx = Context::new();

    ctx.push((nil, 45f32, "Hello world!"));

    assert_eq!(ctx.remove::<LuaValue>(1), LuaValue::Nil);
    assert_eq!(ctx.remove::<LuaValue>(1), LuaValue::Number(45f64));
    assert_eq!(ctx.remove::<LuaValue>(1), LuaValue::String("Hello world!"));
}

