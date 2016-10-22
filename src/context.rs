
use LuaValue;
use Table;
use ffi;

use stack::Read;
use stack::Push;
use stack::Size;

use std::ffi::CString;

#[derive(Debug, PartialEq, Eq)]
pub struct Context {
    pub handle: *mut ffi::lua_State,
    owner: bool,
}

impl Context {
    pub fn new() -> Self {
        Context {
            handle: unsafe {
                ffi::luaL_newstate()
            },
            owner: true,
        }
    }

    pub fn from_state(state: *mut ffi::lua_State) -> Self {
        Context { handle: state, owner: true }
    }

    pub fn from_state_weak(state: *mut ffi::lua_State) -> Self {
        Context { handle: state, owner: false }
    }

    /*pub fn load(&mut self, path: std::path::Path) -> Result<(), IoError> {
        let mut f = try!(File::open(path));
        let mut s = String::new();
        try!(f.read_to_string(&mut s));

        unsafe {

        };

        Ok(())
    }*/

    /*pub fn eval_file<T>(&mut self, path: std::path::Path) -> Result<Result<T, ()>, IoError> {
        unimplemented!()
    }*/

    pub fn eval(&self, code: &str) -> Result<(), &str> {
        unsafe {
            let ret = ffi::luaL_dostring(self.handle, CString::new(code).unwrap().as_ptr());

            match ret {
                false => Ok(()),
                true => Err(self.pop::<&str>())
            }
        }
    }

    pub fn get<'a, T>(&'a self, idx: &str) -> T
        where T: Read<'a> + Size
    {
        unsafe {
            ffi::lua_getfield(self.handle, ffi::LUA_GLOBALSINDEX, unsafe { CString::new(idx).unwrap().as_ptr() as _ });
        }

        self.pop::<T>()
    }

    pub fn set<T>(&self, idx: &str, val: T)
        where T: Push
    {
        val.push(self);

        unsafe {
            ffi::lua_setfield(self.handle, ffi::LUA_GLOBALSINDEX, unsafe { CString::new(idx).unwrap().as_ptr() as _ });
        }
    }

    pub fn peek<'a, T>(&'a self, idx: i32) -> T
        where T: Read<'a>
    {
        T::read(self, idx)
    }

    pub fn push<T>(&self, val: T)
        where T: Push
    {
        val.push(self);
    }

    pub fn pop<'a, T>(&'a self) -> T
        where T: Read<'a> + Size
    {
        let ret = T::read(self, -1);
        if T::size() > 0 {
            self.pop_discard(1);
        }
        ret
    }

    pub fn pop_discard(&self, idx: i32) {
        unsafe {
            ffi::lua_pop(self.handle, idx)
        };
    }

    pub fn remove<'a, T>(&'a self, idx: i32) -> T
        where T: Read<'a> + Size
    {
        let ret = T::read(self, idx);
        if T::size() > 0 {
            self.remove_discard(idx);
        }
        ret
    }

    pub fn remove_discard(&self, idx: i32) {
        unsafe {
            ffi::lua_remove(self.handle, idx)
        };
    }

    pub fn size(&self) -> i32 {
        unsafe {
            ffi::lua_gettop(self.handle)
        }
    }

    pub fn dump(&self) {
        let size = self.size();

        print!("[ ({}) ", size);
        unsafe {
            for i in 1..(size + 1) {
                print!("{}: {:?} ", i,  match ffi::lua_type(self.handle, i) {
                    ffi::LUA_TNONE => "none",
                    ffi::LUA_TNIL => "nil",
                    ffi::LUA_TBOOLEAN => "bool",
                    ffi::LUA_TLIGHTUSERDATA => "lightuserdata",
                    ffi::LUA_TNUMBER => "number",
                    ffi::LUA_TSTRING => "string",
                    ffi::LUA_TTABLE => "table",
                    ffi::LUA_TFUNCTION => "function",
                    ffi::LUA_TUSERDATA => "userdata",
                    ffi::LUA_TTHREAD => "thread",
                    _ => panic!("unknown type"),
                });
            }
        }
        println!(" ]");
    }
    // TODO: more stuff

}

impl Drop for Context {
    fn drop(&mut self) {
        if self.owner {
            unsafe {
                ffi::lua_close(self.handle)
            }
        }
    }
}

#[test]
fn get_globals() {
    let cxt = Context::new();

    cxt.set("foo", 1.32f32);
    cxt.set("bar", "quux");

    assert_eq!(cxt.size(), 0);

    assert_eq!(cxt.get::<f32>("foo"), 1.32f32);
    assert_eq!(cxt.get::<String>("bar"), "quux");

    assert_eq!(cxt.size(), 0);
}

#[test]
fn stack_size() {
    let cxt = Context::new();

    cxt.push((true, false, true, false));

    assert_eq!(cxt.size(), 4);
}

