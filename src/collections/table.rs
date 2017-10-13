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
    pub ctx: &'a Context,
    pub ptr: LuaRef<'a>,
}

impl<'a> Table<'a> {
    pub fn new(ctx: &'a Context) -> Self {
        unsafe {
            ffi::lua_newtable(ctx.handle);
        }

        Table { ctx: ctx, ptr: LuaRef::read(ctx, -1) }
    }

    pub fn from_map<K, V>(ctx: &'a Context, map: &HashMap<K, V>) -> Self
        where K: LuaIndex + Eq + Hash,
              V: Push + Size
    {
        unsafe {
            ffi::lua_newtable(ctx.handle);
        }

        for (k, v) in map.iter() {
            v.push(ctx);
            k.set(ctx, ctx.size() - V::size());
        }

        Table { ctx: ctx, ptr: LuaRef::read(ctx, -1) }
    }

    pub fn from_vec<V>(ctx: &'a Context, vec: &Vec<V>) -> Self
        where V: Push + Size
    {
        unsafe {
            ffi::lua_newtable(ctx.handle);
        }

        for (k, v) in vec.iter().enumerate() {
            v.push(ctx);
            (k + 1).set(ctx, ctx.size() - V::size());
        }

        Table { ctx: ctx, ptr: LuaRef::read(ctx, -1) }
    }

    pub fn get<T, K>(&self, idx: K) -> T
        where T: Read<'a> + Size,
              K: LuaIndex
    {
        self.ptr.push(self.ctx);

        idx.get(self.ctx, -1);

        let ret = self.ctx.pop::<T>();
        self.ctx.pop_discard(1);
        ret
    }

    pub fn set<T, K>(&'a self, idx: K, val: T)
        where T: Push,
              K: LuaIndex
    {
        self.ptr.push(self.ctx);
        self.ctx.push(val);

        idx.set(self.ctx, -2);
        self.ctx.dump();

        self.ctx.pop_discard(1);
    }

    pub fn iter<T>(&self) -> TableIterator<T>
        where T: Read<'a> + Size
    {
        self.ptr.push(self.ctx);

        unsafe {
            ffi::lua_pushnil(self.ctx.handle);
        }

        // TODO: make Context immutable during iter borrow
        TableIterator {
            ctx: self.ctx,
            _pd: PhantomData
        }
    }

    pub fn len(&self) -> usize {
        self.ptr.push(self.ctx);
        let len = unsafe {
            ffi::lua_objlen(self.ctx.handle, -1) as usize
        };
        self.ctx.pop_discard(1);
        len
    }
}

impl<'a> Read<'a> for Table<'a> {
    fn read(ctx: &'a Context, idx: i32) -> Self {
        Table { ctx: ctx, ptr: LuaRef::read(ctx, idx) }
    }

    fn check(ctx: &'a Context, idx: i32) -> bool {
        unsafe {
            ffi::lua_istable(ctx.handle, idx)
        }
    }
}

impl<'a> Push for Table<'a> {
    fn push(&self, ctx: &Context) {
        self.ptr.push(ctx)
    }
}

impl<'a> Size for Table<'a> {
    fn size() -> i32 {
        1
    }
}

pub struct TableIterator<'a, T> {
    ctx: &'a Context,
    _pd: PhantomData<T>
}

impl<'a, T> Iterator for TableIterator<'a, T>
        where T: Read<'a> + Size
    {
    type Item = (LuaValue<'a>, T);

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            match ffi::lua_next(self.ctx.handle, -2) > 0 {
                true => {
                    ffi::lua_pushvalue(self.ctx.handle, -2);
                    let k = self.ctx.peek::<LuaValue>(-1);
                    let v = self.ctx.peek::<T>(-2);
                    self.ctx.pop_discard(2);

                    Some((k, v))
                }
                false => {
                    self.ctx.pop_discard(1);
                    None
                }
            }
        }
    }
}

#[test]
fn len() {
    let ctx = Context::new();

    let table = Table::new(&ctx);

    table.set(1, 100);
    table.set(2, 200);
    table.set(3, 300);

    assert_eq!(table.len(), 3);
}

#[test]
fn access() {
    let ctx = Context::new();

    let table = Table::new(&ctx);

    table.set("alongkeyinatable", nil);
    table.set(1, 5f64);
    table.set("akey", "flim-flam");

    assert_eq!(table.get::<Option<i32>, _>("alongkeyinatable"), None);
    assert_eq!(table.get::<f64, _>(1), 5f64);
    assert_eq!(table.get::<&str, _>("akey"), "flim-flam");
}

#[test]
fn iter() {
    let ctx = Context::new();

    let table = Table::new(&ctx);

    table.set(1, 5);
    table.set(2, 15);
    table.set("woop", false);

    assert_eq!(table.iter::<LuaValue>().collect::<Vec<(LuaValue, LuaValue)>>(), vec![
        (LuaValue::Number(1f64), LuaValue::Number(5f64)),
        (LuaValue::Number(2f64), LuaValue::Number(15f64)),
        (LuaValue::String("woop"), LuaValue::Bool(false)),
    ]);
    assert_eq!(ctx.size(), 0);
}

#[test]
fn from_map() {
    let ctx = Context::new();

    let mut map = HashMap::new();
    map.insert("foo", 5);
    map.insert("bar", 10);
    let table = Table::from_map(&ctx, &map);

    assert_eq!(ctx.size(), 0);

    // assert_eq!(table.len(), 2);

    assert_eq!(table.get::<i32, _>("foo"), 5);
    assert_eq!(table.get::<i32, _>("bar"), 10);
}

#[test]
fn from_vec() {
    let ctx = Context::new();

    let vec = vec![2, 4, 6, 8];
    let table = Table::from_vec(&ctx, &vec);

    assert_eq!(ctx.size(), 0);
    assert_eq!(table.len(), 4);

    assert_eq!(table.get::<i32, _>(1), 2);
    assert_eq!(table.get::<i32, _>(2), 4);
    assert_eq!(table.get::<i32, _>(3), 6);
    assert_eq!(table.get::<i32, _>(4), 8);
}
