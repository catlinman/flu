use Context;
use LuaRef;
use Table;
use ffi;
use nil;

use stack::Read;
use stack::Push;
use stack::Size;

use libc;

use std::ptr;
use std::mem;
use std::marker::PhantomData;

#[derive(Debug, Eq, PartialEq)]
pub struct Function<'a> {
    cxt: &'a Context,
    ptr: LuaRef<'a>
}

impl<'a> Function<'a> {
    pub fn call<T: Push + Size, R: Read<'a> + Size>(&self, args: T) -> Result<R, &'a str> {
        self.ptr.push(self.cxt);
        self.cxt.push(args);

        unsafe {
            let ret = ffi::lua_pcall(self.cxt.handle, T::size(), R::size(), 0);

            match ret {
                0 => Ok(R::read(self.cxt, -1)),
                ffi::LUA_ERRRUN |
                ffi::LUA_ERRMEM |
                ffi::LUA_ERRERR => Err(self.cxt.pop::<&str>()),
                _ => unreachable!()
            }
        }
    }
}

/*impl<'a, T> Push for T where T: Fn(&'a Context) {

}*/

impl<'a> Read<'a> for Function<'a> {
    fn read(cxt: &'a Context, idx: i32) -> Self {
        unsafe {
            let func: LuaRef<'a> = cxt.remove(idx);

            Function {
                cxt: cxt,
                ptr: func
            }
        }
    }

    fn check(cxt: &'a Context, idx: i32) -> bool {
        unsafe {
            ffi::lua_type(cxt.handle, idx) == ffi::LUA_TFUNCTION
        }
    }
}

impl<'a> Size for Function<'a> {
    fn size() -> i32 {
        LuaRef::size()
    }
}

pub fn function<F, A, R>(func: F) -> RustFunction<F, A, R>
        where F: Fn(A) -> R, A: for<'b> Read<'b> + Size, R: Push {
    RustFunction {
        func: func,
        marker: PhantomData
    }
}

pub struct RustFunction<F, A, R> {
    func: F,
    marker: PhantomData<(A, R)>
}

impl<'a, F, A, R> Push for RustFunction<F, A, R>
        where F: Fn(A) -> R, A: for<'b> Read<'b> + Size, R: Push {
    fn push(&self, cxt: &Context) {
        unsafe {
            let wrapped = wrapper::<R, A, F>;

            let func: &mut F = mem::transmute(ffi::lua_newuserdata(cxt.handle, mem::size_of::<F>() as libc::size_t));
            ptr::copy(&self.func, func, 1);

            ffi::lua_pushcclosure(cxt.handle, wrapped, 1);
        }
    }
}

unsafe extern fn wrapper<P, R, F>(L: *mut ffi::lua_State) -> libc::c_int
        where P: Push, R: for<'a> Read<'a> + Size, F: Fn(R) -> P {
    let cxt = Context::from_state_weak(L);
    let func: &mut F = unsafe { mem::transmute(ffi::lua_touserdata(L, ffi::lua_upvalueindex(1))) };

    let args = cxt.pop::<R>();
    cxt.push(func(args));

    R::size()
}

#[test]
fn simple() {
    let mut cxt = Context::new();

    let func = {
        cxt.eval("return function(a) return a * a end").ok();
        cxt.pop::<Function>()
    };

    assert_eq!(func.call::<i32, i32>(5).unwrap(), 25);
}

#[test]
fn multiple_args() {
    let mut cxt = Context::new();

    let func = {
        cxt.eval("return function(a, b, c) return (a + b) * c end").ok();
        cxt.pop::<Function>()
    };

    assert_eq!(func.call::<(i32, i32, f64), f64>((5, 10, 0.1)).unwrap(), 1.5);
}

#[test]
fn custom_types() {
    let mut cxt = Context::new();

    let func = {
        cxt.eval("return function (a, b) return { a, b } end").ok();
        cxt.pop::<Function>()
    };

    let table: Table = func.call((5, 10)).unwrap();

    assert_eq!(table.get::<i32, _>(1), 5);
    assert_eq!(table.get::<i32, _>(2), 10);
}

#[test]
fn rust_fn() {
    let mut cxt = Context::new();

    cxt.set("foo", function(|a: i32| a + a));
    let func = cxt.get::<Function>("foo");

    assert_eq!(func.call::<i32, i32>(10).unwrap(), 20);
}

