extern crate libc;

pub mod ffi;
pub mod push;
pub mod read;

use push::Push;
use read::Read;

pub struct LuaTable;
pub struct LuaFunction;

pub struct nil;

pub enum LuaType {
    Number,
    String,
    Bool,
    Table,
    Function,
    Userdata,
    Thread,
    Nil
}

pub struct LuaValue {

    ty: LuaType
    /*
    Number(f64),
    String(&'a str),
    Bool(bool),
    Table(LuaTable),
    Function(LuaFunction),
    Userdata,
    Thread,
    Nil*/
}

pub struct LuaContext {
    handle: *mut ffi::lua_State,
    owner: bool
}

impl LuaContext {
    pub fn new() -> Self {
        LuaContext {
            handle: unsafe { ffi::luaL_newstate() },
            owner: true,
        }
    }

    pub fn from_state(state: *mut ffi::lua_State) -> Self {
        LuaContext {
            handle: state,
            owner: true,
        }
    }

    pub fn from_state_weak(state: *mut ffi::lua_State) -> Self {
        LuaContext {
            handle: state,
            owner: false,
        }
    }

    /*pub fn load(&mut self, path: std::path::Path) -> Result<(), IoError> {
        let mut f = try!(File::open(path));
        let mut s = String::new();
        try!(f.read_to_string(&mut s));

        unsafe {

        };

        Ok(())
    }*/ 

    pub fn read<T>(&mut self, idx: i32) -> T
                   where T: Read {
        T::read(self, idx)
    }

    pub fn push<T>(&mut self, val: T)
                   where T: Push {
        val.push(self);
    }

    pub fn pop(&mut self, size: i32) {
        unsafe { ffi::lua_pop(self.handle, size) }
    }

    pub fn size(&self) -> i32 {
        unsafe { ffi::lua_gettop(self.handle) }
    }

    // TODO: more stuff

}

impl Drop for LuaContext {
    fn drop(&mut self) {
        if self.owner {
            unsafe { ffi::lua_close(self.handle) }
        }
    }
}

// read! creates Pop(s)
// see: http://www.jeremyong.com/blog/2014/01/10/interfacing-lua-with-templates-in-c-plus-plus-11/

#[macro_export]
macro_rules! push {
    ($cxt:expr, $($arg:expr),*) => (
        $(
            $cxt.push($arg);
        )*
    )
}
