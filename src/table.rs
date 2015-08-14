// TODO: shim over reference (lua_ref) to table on the stack. indexing goes
// through the lua api as well
/*
use LuaRef;

struct Table<'a> {
    ptr: LuaRef<'a>
}

impl<'a> Table<'a> {
    pub fn from_ref(ptr: LuaRef) -> Table {
        Table {
            ptr: ptr
        }
    }
}
*/
