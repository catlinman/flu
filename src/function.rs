use ::{
    ffi, nil, typename,

    FunctionStack, Ref, State, UncheckedFunctionStack, WeakState
};
use errors::*;
use transfer::{
    FromLua, LuaSize, ToLua
};

use libc;

use std::marker::PhantomData;
use std::mem;
use std::ptr;

pub type LuaUncheckedFn = extern "C" fn(UncheckedFunctionStack) -> i32;
pub type LuaFn = Fn(FunctionStack) -> Result<i32>;

#[derive(Debug, PartialEq)]
pub struct Function<'a> {
    ptr: Ref<'a>,
}


impl<'a, 'b> Function<'a> {
    pub fn call<A, R>(&self, state: &'b State, args: A) -> Result<R>
    where
        A: ToLua + LuaSize,
        R: FromLua<'b> + LuaSize,
    {
        self.ptr.write(state);
        args.write(state);

        unsafe {
            let ret = ffi::lua_pcall(state.L, A::size(), R::size(), -A::size()-1);

            match ret {
                0 => Ok(R::read(state, -1)?),
                ffi::LUA_ERRRUN | ffi::LUA_ERRMEM | ffi::LUA_ERRERR => Err(
                    ErrorKind::RuntimeError(
                        String::read(&state, -1)?,
                    ).into(),
                ),
                _ => unreachable!(),
            }
        }
    }
}

impl<'a> FromLua<'a> for Function<'a> {
    fn read(state: &'a WeakState, idx: i32) -> Result<Self> {
        unsafe {
            let ty = ffi::lua_type(state.L, idx);
            if ty == ffi::LUA_TFUNCTION {
                let func = Function { ptr: Ref::read(state, idx).unwrap() };

                Ok(func)
            } else {
                Err(
                    ErrorKind::TypeError("function".into(), typename(state, idx)).into(),
                )
            }
        }
    }
}

impl<'a> ::FromLuaFunctionStack<'a> for Function<'a> {
    fn read_unchecked_arg(state: &'a WeakState, idx: i32) -> Self {
        ::arg_unchecked_typeck(state, idx, ffi::LUA_TFUNCTION);

        let func = Function { ptr: Ref::read(state, idx).unwrap() };

        func
    }

    fn read_arg(state: &'a WeakState, idx: i32) -> Result<Self> {
        ::arg_typeck(state, idx, ffi::LUA_TFUNCTION)?;

        let func = Function { ptr: Ref::read(state, idx)? };

        Ok(func)
    }
}

impl<'a> ToLua for Function<'a> {
    fn write(&self, state: &WeakState) {
        self.ptr.write(state);
    }
}

impl ToLua for LuaUncheckedFn {
    fn write(&self, state: &WeakState) {
        unsafe {
            ffi::lua_pushcfunction(state.L, mem::transmute(*self));
        }
    }
}

impl<F: Fn(FunctionStack) -> Result<i32>> ToLua for F {
    fn write(&self, state: &WeakState) {
        unsafe {
            let wrapper = fn_wrapper::<F>;
            let func: &mut F = mem::transmute(ffi::lua_newuserdata(
                state.L,
                mem::size_of::<F>() as libc::size_t,
            ));
            ptr::copy(&self as &F, func, 1);

            ffi::lua_pushcclosure(state.L, wrapper, 1);
        }
    }
}

unsafe extern "C" fn fn_wrapper<F: Fn(FunctionStack) -> Result<i32>>(
    L: *mut ffi::lua_State,
) -> libc::c_int {
    let mut cxt = FunctionStack { state: WeakState::from_state(L) };
    let func: &mut F = mem::transmute(ffi::lua_touserdata(L, ffi::lua_upvalueindex(1)));

    match func(cxt) {
        Ok(v) => v,
        Err(e) => {
            // we longjmp after wrapped function errors and push the error
            // message to the error handler
            format!("{}", e).write(&WeakState::from_state(L));
            ffi::lua_error(L);

            -1
        }
    }
}

mod bench {
    use super::*;
    use test::Bencher;

    #[bench]
    fn checked(b: &mut Bencher) {
        fn test(stack: FunctionStack) -> Result<i32> {
            let sz = stack.check_size(1..3)?;

            let a: i32 = stack.arg(1)?;
            let b: i32 = stack.arg(2)?;
            let c: String = stack.arg(3)?;

            //println!("{:?}, {:?}, {:?}", a, b, c);

            Ok(0)
        }

        let mut state = State::new();
        state.set("test", test);

        b.iter(|| state.eval::<()>(r#"
for i=1,256 do
    test(1, 2, "hello")
end
        "#).unwrap());
    }

    #[bench]
    fn unchecked(b: &mut Bencher) {
        extern "C" fn test(stack: UncheckedFunctionStack) -> i32 {
            let sz = stack.check_size(1..3);

            let a: i32 = stack.arg(1);
            let b: i32 = stack.arg(2);
            let c: String = stack.arg(3);

            //println!("{:?}, {:?}, {:?}", a, b, c);

            //let a = flu::arg::<i32>(1)?;
            0
        }

        let mut state = State::new();
        state.set("test", test as LuaUncheckedFn);

        b.iter(|| state.eval::<()>(r#"
for i=1,256 do
    test(1, 2, "hello")
end
        "#).unwrap());
    }
}