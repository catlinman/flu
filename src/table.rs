use ::{
    ffi, State, WeakState
};
use errors::*;
use transfer::{FromLua, ToLua};

use std::ffi::CString;
use std::mem;
use std::slice;

pub struct Table<'a> {
    state: &'a State,

}

pub struct TableContext {
    state: WeakState,
    root: i32
}

impl TableContext {
    pub fn get<'a, V>(&'a self, idx: &str) -> Result<V>
        where
            V: FromLua<'a>,
    {
        unsafe {
            ffi::patch::flu_getlfield(
                self.state.L,
                self.root,
                idx.as_ptr() as _,
                idx.len()
            );

            /*ffi::lua_getfield(
                self.state.L,
                self.root,
                CString::new(idx).unwrap().as_ptr() as _,
            );*/
        }

        V::read(&self.state, -1)
    }

    pub fn set<'a, V>(&'a self, idx: &'a str, val: V)
        where
            V: ToLua,
    {
        val.write(&self.state);

        unsafe {
            ffi::patch::flu_setlfield(
                self.state.L,
                self.root,
                idx.as_ptr() as _,
                idx.len()
            );

            /*ffi::lua_setfield(
                self.state.L,
                self.root,
                CString::new(idx).unwrap().as_ptr() as _,
            );*/
        }
    }
}


pub struct TableInit<F: Fn(TableContext)> {
    func: F
}

impl<'a> Table<'a> {
    pub fn new<F>(func: F) -> TableInit<F>
            where F: Fn(TableContext)
    {
        TableInit {
            func: func
        }
    }
}

impl<F> ToLua for TableInit<F>
        where F: Fn(TableContext) {
    fn write(&self, state: &WeakState) {
        unsafe {
            ffi::lua_newtable(state.L);

            (self.func)(TableContext { state: WeakState::from_state(state.L), root: ffi::lua_gettop(state.L) });
        }
    }
}


mod bench {
    use super::*;
    use test::Bencher;

    #[bench]
    fn table_get_set(b: &mut Bencher) {
        let mut state = State::new();

        b.iter(|| {
            state.set("one", Table::new(|root| {
                root.set("foobartwobar", 4);
                root.set("two", Table::new(|cxt| {
                    cxt.set("three", Table::new(|cxt| {
                        cxt.set("four", Table::new(|cxt| {
                            //cxt.set("onebar", root.get::<i32>("foobartwobar").unwrap());
                        }));
                    }));
                }));
            }));
        });
    }
}