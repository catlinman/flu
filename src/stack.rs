use ffi;
use WeakState;
use transfer::{ToLua, FromLua, FromLuaFunctionStack, LuaSize};
use std::mem;
use errors::*;

#[repr(C)]
pub struct UncheckedFunctionStack {
    pub state: WeakState,
}

#[repr(C)]
pub struct FunctionStack {
    pub state: WeakState,
}

impl UncheckedFunctionStack {
    pub fn check_size(&self, range: ::std::ops::Range<i32>) {
        unsafe {
            let sz = ffi::lua_gettop(self.state.L);

            if sz < range.start || sz > range.end {
                let mut ar: ffi::lua_Debug = mem::uninitialized();
                if ffi::lua_getstack(self.state.L, 0, &mut ar) != 0 {
                    ffi::luaL_error(
                        self.state.L,
                        c_str!("wrong argument count (%d to %d expected, got %d)"),
                        range.start,
                        range.end,
                        sz,
                    );
                }

                ffi::lua_getinfo(self.state.L, c_str!("n"), &mut ar);
                if ar.name.is_null() {
                    ar.name = c_str!("?");
                }

                ffi::luaL_error(
                    self.state.L,
                    c_str!("wrong argument count for %s (%d to %d expected, got %d)"),
                    ar.name,
                    range.start,
                    range.end,
                    sz,
                );
            }
        }
    }

    pub fn arg<'b, T>(&'b self, idx: i32) -> T
    where
        T: FromLuaFunctionStack<'b>,
    {
        T::read_unchecked_arg(&self.state, idx)
    }

    pub fn push<T>(&self, val: T)
    where
        T: ToLua,
    {
        val.write(&self.state);
    }
}

impl FunctionStack {
    pub fn check_size(&self, range: ::std::ops::Range<i32>) -> Result<i32> {
        unsafe {
            let sz = ffi::lua_gettop(self.state.L);

            if sz < range.start || sz > range.end {
                let mut ar: ffi::lua_Debug = mem::uninitialized();
                if ffi::lua_getstack(self.state.L, 0, &mut ar) != 0 {
                    return Err(ErrorKind::ArgCount(range.start, range.end, sz).into());
                }

                ffi::lua_getinfo(self.state.L, c_str!("n"), &mut ar);
                if ar.name.is_null() {
                    ar.name = c_str!("?");
                }

                return Err(ErrorKind::ArgCount(range.start, range.end, sz).into());

                /*ffi::luaL_error(
                    self.state.L,
                    c_str!("wrong argument count for %s (%d to %d expected, got %d)"),
                    ar.name,
                    range.start,
                    range.end,
                    sz,
                );*/
            }

            Ok(sz)
        }
    }

    pub fn arg<'b, T>(&'b self, idx: i32) -> Result<T>
        where
            T: FromLuaFunctionStack<'b>,
    {
        T::read_arg(&self.state, idx)
    }

    pub fn push<T>(&self, val: T)
        where
            T: ToLua,
    {
        val.write(&self.state);
    }

    pub fn with_arg<'b, A, T, F>(&'b self, idx: i32, func: F) -> Result<T>
        where A: FromLuaFunctionStack<'b>, F: Fn(A::WithContext) -> Result<T>
    {
        A::with_arg::<F, T>(&self.state, idx, func)
    }
}