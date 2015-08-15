use LuaContext;
use LuaRef;
use ffi;
use nil;

use read::Read;
use push::Push;
use size::Size;

use std::mem;
use std::hash::Hash;
use std::collections::HashMap;

pub trait LuaIndex {
    fn get(&self, cxt: &LuaContext, idx: i32);
    fn set(&self, cxt: &LuaContext, idx: i32);
}

macro_rules! integer_index {
    ($ty:ident) => (
        impl LuaIndex for $ty {
            fn get(&self, cxt: &LuaContext, idx: i32) {
                unsafe { ffi::lua_rawgeti(cxt.handle, idx, 1 + *self as i32) }
            }

            fn set(&self, cxt: &LuaContext, idx: i32) {
                unsafe { ffi::lua_rawseti(cxt.handle, idx, 1 + *self as i32) }
            }
        }
    )
}

integer_index!(i32);
integer_index!(usize);

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
        unsafe { ffi::lua_newtable(cxt.handle); }

        Table {
            cxt: cxt,
            ptr: LuaRef::read(cxt, -1)
        }
    }

    pub fn from_map<K, V>(cxt: &'a LuaContext, map: &HashMap<K, V>) -> Self
                          where K: LuaIndex + Eq + Hash,
                                V: Push + Size {
        unsafe { ffi::lua_newtable(cxt.handle); }

        for (k, v) in map.iter() {
            v.push(cxt);
            k.set(cxt, cxt.size() - v.size());
        }

        Table {
            cxt: cxt,
            ptr: LuaRef::read(cxt, -1)
        }
    }

    pub fn from_vec<V>(cxt: &'a LuaContext, vec: &Vec<V>) -> Self
                       where V: Push + Size {
        unsafe { ffi::lua_newtable(cxt.handle); }

        for (k, v) in vec.iter().enumerate() {
            v.push(cxt);
            k.set(cxt, cxt.size() - v.size());
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
                  where T: Push + Size,
                        K: LuaIndex {
        self.ptr.push(self.cxt);
        self.cxt.push(val);

        idx.set(self.cxt, -2);

        self.cxt.pop_discard(2);
    }

    pub fn len(&self) -> usize {
        self.ptr.push(self.cxt);
        let len = unsafe { ffi::lua_objlen(self.cxt.handle, -1) as usize };
        self.cxt.pop_discard(1);
        len
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

#[test]
fn len() {
    let cxt = LuaContext::new();

    let table = Table::new(&cxt);

    table.set(0, 100);
    table.set(1, 200);
    table.set(2, 300);

    assert_eq!(table.len(), 3);
}

#[test]
fn access() {
    let cxt = LuaContext::new();

    let table = Table::new(&cxt);

    table.set("alongkeyinatable", nil);
    table.set(0, 5f64);
    table.set("akey", "flim-flam");

    assert_eq!(table.get::<Option<i32>, _>("alongkeyinatable"), None);
    assert_eq!(table.get::<f64, _>(0), 5f64);
    assert_eq!(table.get::<&str, _>("akey"), "flim-flam");
}

#[test]
fn from_map() {
    let cxt = LuaContext::new();

    let mut map = HashMap::new();
    map.insert("foo", 5);
    map.insert("bar", 10);
    let table = Table::from_map(&cxt, &map);

    assert_eq!(cxt.size(), 0);
    
    // TODO: separate tables and contigious integral arrays as `objlen`
    // (#-operator) doesn't take into account non-contigious keys
    
    // assert_eq!(table.len(), 2);

    assert_eq!(table.get::<i32, _>("foo"), 5);
    assert_eq!(table.get::<i32, _>("bar"), 10);
}

#[test]
fn from_vec() {
    let cxt = LuaContext::new();

    let vec = vec![2, 4, 6, 8];
    let table = Table::from_vec(&cxt, &vec);

    assert_eq!(cxt.size(), 0);
    assert_eq!(table.len(), 4);

    assert_eq!(table.get::<i32, _>(0), 2);
    assert_eq!(table.get::<i32, _>(1), 4);
    assert_eq!(table.get::<i32, _>(2), 6);
    assert_eq!(table.get::<i32, _>(3), 8);
}
