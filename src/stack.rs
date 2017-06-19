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

pub struct StackValue<'a, T: 'a> {
    idx: i32,
    _phantom: ::std::marker::PhantomData<&'a T>
}

impl<'a, T> ToLua for StackValue<'a, T>
    where T: FromLuaFunctionStack<'a>
{
    #[inline(always)]
    fn write(&self, state: &WeakState) {
        unsafe {
            //if ffi::lua_gettop(state.L) != self.idx {
                ffi::lua_pushvalue(state.L, self.idx);
            //}
        }
    }
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


impl ::std::ops::Deref for FunctionStack {
    type Target = WeakState;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.state
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

    #[inline(always)]
    pub fn value<'b, T>(&'b self, idx: i32) -> Result<StackValue<T>>
    where T: FromLuaFunctionStack<'b>
    {
        let idx = ::abs_idx(self.state.L, idx);

        if T::valid(&self.state, idx) {
            Ok(StackValue {
                idx: idx,
                _phantom: ::std::marker::PhantomData
            })
        } else {
            Err(
                ErrorKind::TypeError("todo".into(), ::typename(&self.state, idx)).into(),
            )
        }
    }

    pub fn with_arg<'b, A, T, F>(&'b self, idx: i32, func: F) -> Result<T>
        where A: FromLuaFunctionStack<'b>, F: Fn(A::WithContext) -> Result<T>
    {
        A::with_arg::<F, T>(&self.state, idx, func)
    }
}