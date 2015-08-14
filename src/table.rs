use LuaContext;
use LuaRef;
use ffi;

use read::Read;
use push::Push;
use size::Size;

use std::mem;

pub trait LuaIndex {
    fn get(&self, cxt: &LuaContext, idx: i32);
    fn set(&self, cxt: &LuaContext, idx: i32);
}

impl LuaIndex for i32 {
    fn get(&self, cxt: &LuaContext, idx: i32) {
        unsafe { ffi::lua_rawgeti(cxt.handle, idx, *self) }
    }

    fn set(&self, cxt: &LuaContext, idx: i32) {
        unsafe { ffi::lua_rawseti(cxt.handle, idx, *self) }
    }
}

impl<'a, 'b> LuaIndex for &'b str {
    fn get(&self, cxt: &LuaContext, idx: i32) {
        unsafe { ffi::lua_getfield(cxt.handle, idx, self.as_ptr() as *const i8) }
    }

    fn set(&self, cxt: &LuaContext, idx: i32) {
        unsafe { ffi::lua_setfield(cxt.handle, idx, self.as_ptr() as *const i8) }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Table<'a> {
    cxt: &'a LuaContext,
    ptr: LuaRef<'a>
}

impl<'a> Table<'a> {
    pub fn new(cxt: &'a LuaContext) -> Self {
        unsafe {
            ffi::lua_newtable(cxt.handle);
        }

        Table {
            cxt: cxt,
            ptr: LuaRef::read(cxt, -1)
        }
    }

    pub fn get<T, K>(&'a self, idx: K) -> T
                  where T: Read<'a> + Size,
                        K: LuaIndex {
        self.ptr.push(self.cxt);

        idx.get(self.cxt, -1);

        let ret = self.cxt.pop::<T>();
        self.cxt.pop_discard(1);
        ret
    }

    pub fn set<T, K>(&'a self, idx: K, val: T)
                  where T: Push + Read<'a> + Size,
                        K: LuaIndex {
        self.ptr.push(self.cxt);
        self.cxt.push(val);

        idx.set(self.cxt, -2);

        let ret = self.cxt.pop::<T>();
        self.cxt.pop_discard(1);
    }
}

impl<'a> Read<'a> for Table<'a> {
    fn read(cxt: &'a LuaContext, idx: i32) -> Self {
        Table {
            cxt: cxt,
            ptr: LuaRef::read(cxt, idx)
        }
    }

    fn check(cxt: &'a LuaContext, idx: i32) -> bool {
        unsafe { ffi::lua_istable(cxt.handle, idx) }
    }
}

impl<'a> Push for Table<'a> {
    fn push(&self, cxt: &LuaContext) {
        self.ptr.push(cxt)
    }
}

impl<'a> Size for Table<'a> {
    fn size(&self) -> i32 {
        self.ptr.size()
    }
}
