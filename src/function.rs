use LuaContext;
use LuaRef;
use Table;
use ffi;
use nil;

use stack::Read;
use stack::Push;
use stack::Size;

#[derive(Debug, Eq, PartialEq)]
pub struct Function<'a> {
    cxt: &'a LuaContext,
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

impl<'a> Read<'a> for Function<'a> {
    fn read(cxt: &'a LuaContext, idx: i32) -> Self {
        unsafe {
            let func: LuaRef<'a> = cxt.remove(idx);

            Function {
                cxt: cxt,
                ptr: func
            }
        }
    }

    fn check(cxt: &'a LuaContext, idx: i32) -> bool {
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

#[test]
fn simple() {
    let mut cxt = LuaContext::new();

    let func = {
        cxt.eval("func = function(a) return a + a end").ok();
        cxt.get::<Function>("func")
    };

    assert_eq!(func.call::<i32, i32>(5).unwrap(), 10);
}

#[test]
fn multiple_args() {
    let mut cxt = LuaContext::new();

    let func = {
        cxt.eval("func = function(a, b, c) return (a + b) * c end").ok();
        cxt.get::<Function>("func")
    };

    assert_eq!(func.call::<(i32, i32, f64), f64>((5, 10, 0.1)).unwrap(), 1.5);
}

#[test]
fn custom_types() {
    let mut cxt = LuaContext::new();

    let func = {
        cxt.eval("func = function(a, b) return { a, b } end");
        cxt.get::<Function>("func")
    };

    let table: Table = func.call((5, 10)).unwrap();

    assert_eq!(table.get::<i32, _>(1), 5);
    assert_eq!(table.get::<i32, _>(2), 10);
}

