use LuaContext;
use LuaValue;
use LuaRef;
use ffi;
use nil;

use stack::Read;
use stack::Push;
use stack::Size;

use std::marker::PhantomData;
use std::mem;
use std::hash::Hash;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
pub struct Array<'a> {
    cxt: &'a LuaContext,
    ptr: LuaRef<'a>,
}

impl<'a> Array<'a> {
    pub fn new(cxt: &'a LuaContext) -> Self {
        unsafe {
            ffi::lua_newtable(cxt.handle);
        }

        Array { cxt: cxt, ptr: LuaRef::read(cxt, -1) }
    }

    pub fn from_vec<V>(cxt: &'a LuaContext, vec: &Vec<V>) -> Self
        where V: Push + Size
    {
        unsafe {
            ffi::lua_newtable(cxt.handle);
        }

        for (k, v) in vec.iter().enumerate() {
            v.push(cxt);

            unsafe {
                ffi::lua_rawseti(cxt.handle, cxt.size() - v.size(), 1 + k as i32);
            }
        }

        Array { cxt: cxt, ptr: LuaRef::read(cxt, -1) }
    }

    pub fn get<T>(&'a self, idx: i32) -> T
        where T: Read<'a> + Size
    {
        self.ptr.push(self.cxt);

        unsafe {
            ffi::lua_rawgeti(self.cxt.handle, -1, 1 + idx);
        }

        let ret = self.cxt.pop::<T>();
        self.cxt.pop_discard(1);
        ret
    }

    pub fn set<T>(&'a self, idx: i32, val: T)
        where T: Push + Size
    {
        self.ptr.push(self.cxt);
        self.cxt.push(val);

        unsafe {
            ffi::lua_rawseti(self.cxt.handle, -2, 1 + idx);
        }

        self.cxt.pop_discard(1);
    }

    pub fn iter<T>(&self) -> ArrayIterator<T>
        where T: Read<'a> + Size
    {
        self.ptr.push(self.cxt);

        unsafe {
            ffi::lua_pushnil(self.cxt.handle);
        }

        // TODO: make LuaContext immutable during iter borrow
        ArrayIterator {
            cxt: self.cxt,
            _pd: PhantomData
        }
    }

    pub fn len(&self) -> usize {
        self.ptr.push(self.cxt);
        let len = unsafe {
            ffi::lua_objlen(self.cxt.handle, -1) as usize
        };
        self.cxt.pop_discard(1);
        len
    }
}

impl<'a> Read<'a> for Array<'a> {
    fn read(cxt: &'a LuaContext, idx: i32) -> Self {
        Array { cxt: cxt, ptr: LuaRef::read(cxt, idx) }
    }

    fn check(cxt: &'a LuaContext, idx: i32) -> bool {
        unsafe {
            ffi::lua_istable(cxt.handle, idx)
        }
    }
}

impl<'a> Push for Array<'a> {
    fn push(&self, cxt: &LuaContext) {
        self.ptr.push(cxt)
    }
}

impl<'a> Size for Array<'a> {
    fn size(&self) -> i32 {
        self.ptr.size()
    }
}

pub struct ArrayIterator<'a, T> {
    cxt: &'a LuaContext,
    _pd: PhantomData<T>
}

impl<'a, T> Iterator for ArrayIterator<'a, T>
        where T: Read<'a> + Size
    {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            match ffi::lua_next(self.cxt.handle, -2) > 0 {
                true => {
                    ffi::lua_pushvalue(self.cxt.handle, -2);
                    let v = self.cxt.peek::<T>(-2);
                    self.cxt.pop_discard(2);

                    Some(v)
                }
                false => {
                    self.cxt.pop_discard(1);
                    None
                }
            }
        }
    }
}

#[test]
fn len() {
    let cxt = LuaContext::new();

    let table = Array::new(&cxt);

    table.set(0, 100);
    table.set(1, 200);
    table.set(2, 300);

    assert_eq!(table.len(), 3);
}

#[test]
fn access() {
    let cxt = LuaContext::new();

    let table = Array::new(&cxt);

    table.set(0, nil);
    table.set(1, 5f64);
    table.set(2, "flim-flam");

    assert_eq!(table.get::<Option<i32>>(0), None);
    assert_eq!(table.get::<f64>(1), 5f64);
    assert_eq!(table.get::<&str>(2), "flim-flam");
}

#[test]
fn iter() {
    let cxt = LuaContext::new();

    let table = Array::new(&cxt);

    table.set(0, 5);
    table.set(1, 10);
    table.set(2, 15);

    assert_eq!(table.iter::<i32>().fold(0, |acc, item| { acc + item}), 30);
    assert_eq!(cxt.size(), 0);
}

#[test]
fn from_vec() {
    let cxt = LuaContext::new();

    let vec = vec![2, 4, 6, 8];
    let table = Array::from_vec(&cxt, &vec);

    assert_eq!(cxt.size(), 0);
    assert_eq!(table.len(), 4);

    assert_eq!(table.get::<i32>(0), 2);
    assert_eq!(table.get::<i32>(1), 4);
    assert_eq!(table.get::<i32>(2), 6);
    assert_eq!(table.get::<i32>(3), 8);
}
