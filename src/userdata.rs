use ::{
    ffi, State, WeakState, Ref
};
use errors::*;
use transfer::{FromLuaFunctionStack, FromLua, ToLua, LuaSize};

use std::ffi::CString;
use std::mem;
use std::slice;



struct TypedLightUserdata<T> {
    marker: usize,
    data: T
}

impl<T> From<T> for TypedLightUserdata<T> {
    fn from(v: T) -> Self {
        TypedLightUserdata {
            marker: 0,
            data: v
        }
    }
}
