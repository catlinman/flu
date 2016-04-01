use Context;
use LuaValue;
use LuaRef;
use ffi;
use nil;

use super::LuaIndex;

use stack::Read;
use stack::Push;
use stack::Size;

use std::marker::PhantomData;
use std::mem;
use std::hash::Hash;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq)]
pub struct Table<'a> {
    pub cxt: &'a Context,
    pub ptr: LuaRef<'a>,
}

impl<'a> Table<'a> {
    pub fn new(cxt: &'a Context) -> Self {
        unsafe {
            ffi::lua_newtable(cxt.handle);
        }

        Table { cxt: cxt, ptr: LuaRef::read(cxt, -1) }
    }

    pub fn from_map<K, V>(cxt: &'a Context, map: &HashMap<K, V>) -> Self
        where K: LuaIndex + Eq + Hash,
              V: Push + Size
    {
        unsafe {
            ffi::lua_newtable(cxt.handle);
        }

        for (k, v) in map.iter() {
            v.push(cxt);
            k.set(cxt, cxt.size() - V::size());
        }

        Table { cxt: cxt, ptr: LuaRef::read(cxt, -1) }
    }

    pub fn from_vec<V>(cxt: &'a Context, vec: &Vec<V>) -> Self
        where V: Push + Size
    {
        unsafe {
            ffi::lua_newtable(cxt.handle);
        }

        for (k, v) in vec.iter().enumerate() {
            v.push(cxt);
            (k + 1).set(cxt, cxt.size() - V::size());
        }

        Table { cxt: cxt, ptr: LuaRef::read(cxt, -1) }
    }

    pub fn get<T, K>(&self, idx: K) -> T
        where T: Read<'a> + Size,
              K: LuaIndex
    {
        self.ptr.push(self.cxt);

        idx.get(self.cxt, -1);

        let ret = self.cxt.pop::<T>();
        self.cxt.pop_discard(1);
        ret
    }

    pub fn set<T, K>(&'a self, idx: K, val: T)
        where T: Push + Size,
              K: LuaIndex
    {
        self.ptr.push(self.cxt);
        self.cxt.push(val);

        idx.set(self.cxt, -2);

        self.cxt.pop_discard(1);
    }

    pub fn iter<T>(&self) -> TableIterator<T>
        where T: Read<'a> + Size
    {
        self.ptr.push(self.cxt);

        unsafe {
            ffi::lua_pushnil(self.cxt.handle);
        }

        // TODO: make Context immutable during iter borrow
        TableIterator {
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

impl<'a> Read<'a> for Table<'a> {
    fn read(cxt: &'a Context, idx: i32) -> Self {
        Table { cxt: cxt, ptr: LuaRef::read(cxt, idx) }
    }

    fn check(cxt: &'a Context, idx: i32) -> bool {
        unsafe {
            ffi::lua_istable(cxt.handle, idx)
        }
    }
}

impl<'a> Push for Table<'a> {
    fn push(&self, cxt: &Context) {
        self.ptr.push(cxt)
    }
}

impl<'a> Size for Table<'a> {
    fn size() -> i32 {
        1
    }
}

pub struct TableIterator<'a, T> {
    cxt: &'a Context,
    _pd: PhantomData<T>
}

impl<'a, T> Iterator for TableIterator<'a, T>
        where T: Read<'a> + Size
    {
    type Item = (LuaValue<'a>, T);

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            match ffi::lua_next(self.cxt.handle, -2) > 0 {
                true => {
                    ffi::lua_pushvalue(self.cxt.handle, -2);
                    let k = self.cxt.peek::<LuaValue>(-1);
                    let v = self.cxt.peek::<T>(-2);
                    self.cxt.pop_discard(2);

                    Some((k, v))
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
    let cxt = Context::new();

    let table = Table::new(&cxt);

    table.set(1, 100);
    table.set(2, 200);
    table.set(3, 300);

    assert_eq!(table.len(), 3);
}

#[test]
fn access() {
    let cxt = Context::new();

    let table = Table::new(&cxt);

    table.set("alongkeyinatable", nil);
    table.set(1, 5f64);
    table.set("akey", "flim-flam");

    assert_eq!(table.get::<Option<i32>, _>("alongkeyinatable"), None);
    assert_eq!(table.get::<f64, _>(1), 5f64);
    assert_eq!(table.get::<&str, _>("akey"), "flim-flam");
}

#[test]
fn iter() {
    let cxt = Context::new();

    let table = Table::new(&cxt);

    table.set(1, 5);
    table.set(2, 15);
    table.set("woop", false);

    assert_eq!(table.iter::<LuaValue>().collect::<Vec<(LuaValue, LuaValue)>>(), vec![
        (LuaValue::Number(1f64), LuaValue::Number(5f64)),
        (LuaValue::Number(2f64), LuaValue::Number(15f64)),
        (LuaValue::String("woop"), LuaValue::Bool(false)),
    ]);
    assert_eq!(cxt.size(), 0);
}

#[test]
fn from_map() {
    let cxt = Context::new();

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
    let cxt = Context::new();

    let vec = vec![2, 4, 6, 8];
    let table = Table::from_vec(&cxt, &vec);

    assert_eq!(cxt.size(), 0);
    assert_eq!(table.len(), 4);

    assert_eq!(table.get::<i32, _>(1), 2);
    assert_eq!(table.get::<i32, _>(2), 4);
    assert_eq!(table.get::<i32, _>(3), 6);
    assert_eq!(table.get::<i32, _>(4), 8);
}
