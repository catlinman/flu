extern crate libc;

pub mod ffi;
pub mod push;
pub mod read;

use push::Push;
use read::Read;

pub struct LuaTable;
pub struct LuaFunction;

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

    /*pub fn read<T>(&mut self) -> T
                   where T: Read {

    }*/

    pub fn push<T>(&mut self, val: T)
                   where T: Push {
        val.push(self);
    }

    pub fn pop(&mut self, size: i32) {
        unsafe { ffi::lua_pop(self.handle, size) }
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
