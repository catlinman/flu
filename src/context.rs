
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

    pub fn eval(&mut self, code: &str) -> Result<(), &str> {
        unsafe {
            let ret = ffi::luaL_dostring(self.handle, CString::new(code).unwrap().as_ptr());

            match ret {
                0 => Ok(()),
                _ => Err(self.pop::<&str>())
            }
        }
    }

    pub fn get<'a, T>(&'a self, idx: &str) -> T
        where T: Read<'a> + Size
    {
        unsafe {
            ffi::lua_getfield(self.handle, ffi::LUA_GLOBALSINDEX, idx.as_ptr() as *const i8);
        }

        self.pop::<T>()
    }

    pub fn set<T>(&self, idx: &str, val: T)
        where T: Push
    {
        idx.push(self);
        val.push(self);

        unsafe {
            ffi::lua_setfield(self.handle, ffi::LUA_GLOBALSINDEX, idx.as_ptr() as *const i8);
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
        for i in 1..(size + 1) {
            print!("{}: {:?} ", i, self.peek::<LuaValue>(i));
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

    cxt.set("quxxy_macro_wizard", Table::new(&cxt));
    assert_enum!(cxt.get::<LuaValue>("quxxy_macro_wizard"), LuaValue::Table);

    assert_eq!(cxt.size(), 0);
}

#[test]
fn stack_size() {
    let cxt = Context::new();

    cxt.push((true, false, true, false));

    assert_eq!(cxt.size(), 4);
}

