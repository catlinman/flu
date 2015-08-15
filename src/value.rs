use LuaContext;
use Table;
use ffi;

use read::Read;
use size::Size;

#[derive(Debug, PartialEq)]
pub enum LuaValue<'a> {
    Number(f64),
    String(&'a str),
    Bool(bool),
    Table(Table<'a>),
    /*Function(LuaFunction),
    Userdata,
    Thread,*/
    Nil
}

impl<'a> Read<'a> for LuaValue<'a> {
    fn read(cxt: &'a LuaContext, idx: i32) -> Self {
        unsafe {
            match ffi::lua_type(cxt.handle, idx) {
                -1 => panic!("woops!"),                         /* TNONE */
                 0 => LuaValue::Nil,                            /* TNIL */
                 1 => LuaValue::Bool(bool::read(cxt, idx)),     /* TBOOLEAN */
                 2 => unimplemented!(),                         /* TLIGHTUSERDATA */
                 3 => LuaValue::Number(f64::read(cxt, idx)),    /* TNUMBER */
                 4 => LuaValue::String(<&str>::read(cxt, idx)), /* TSTRING */
                 5 => unimplemented!(),                         /* TTABLE */
                 6 => unimplemented!(),                         /* TFUNCTION */
                 7 => unimplemented!(),                         /* TUSERDATA */
                 8 => unimplemented!(),                         /* TTHREAD */
                 _ => panic!("yahallo")
            }
        }
    }

    fn check(cxt: &'a LuaContext, idx: i32) -> bool {
        true
    }
}

impl<'a> Size for LuaValue<'a> {
    fn size(&self) -> i32 {
        match self {
            _ => 1,
        }
    }
}
