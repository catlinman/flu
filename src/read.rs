use ffi;
use LuaContext;

use std::ffi::CString;

pub trait Read {
    fn read(self, cxt: &mut LuaContext) -> Self;
}
