use super::*;
use super::super::*;

#[no_mangle]
#[inline(never)]
pub unsafe extern "C" fn flu_traceback(L: *mut lua_State) {
    let Lj: *mut luajit_State = L as _;

    let mut level = 0;

    let top = (*Lj).base.offset_to((*Lj).top).unwrap();
    let mut lim = 12;

    let mut ar: lua_Debug = ::std::mem::uninitialized();

    while lua_getstack(L, level, &mut ar) != 0 {
        level = level + 1;

        let mut func: *mut GCfunc = ::std::mem::uninitialized();
        if level > lim {
            if lua_getstack(L, level + 10, &mut ar) != 0 {
                level = level - 1;
            } else {
                lua_pushliteral(L, "\n\t...");
                lua_getstack(L, -10, &mut ar);
                level = ar.i_ci - 10;
            }
            lim = 2147483647;
            continue;
        }

        lua_getinfo(L, c_str!("Snlf"), &mut ar);
        func = funcV((*Lj).top.offset(-1));
        (*Lj).top = (*Lj).top.offset(-1);

        if isffunc(func) && (*ar.namewhat) == 0 {
            lua_pushfstring(L, c_str!("\n\t[builtin#%d]:"), (*func).c.ffid as libc::c_int);
        } else {
            lua_pushfstring(L, c_str!("\n\t%s:"), ::std::mem::transmute::<_, *const libc::c_char>(&ar.short_src));
        }

        if ar.currentline > 0 {
            lua_pushfstring(L, c_str!("%d:"), ar.currentline);
        }

        if (*ar.namewhat) != 0 {
            lua_pushfstring(L, c_str!(" in function %s"), ar.name);
        } else {
            if *ar.what == 'm' as _ {
                lua_pushliteral(L, " in main chunk");
            } else if *ar.what == 'C' as _ {
                lua_pushfstring(L, c_str!(" at %p"), (*func).c.f);
            } else {
                lua_pushfstring(L, c_str!(" in function <%s:%d>"),
                                ar.short_src, ar.linedefined);
            }
        }

        let mut name: *const libc::c_char = ptr::null();
        for l in 1.. {
            name = lua_getlocal(L, &mut ar, l);
            if name == ptr::null() {
                break;
            }

            if (*name) == '(' as _ {
                lua_pop(L, 1);
                break;
            }

            let ty = lua_type(L, -1);
            lua_pushfstring(L, c_str!("\n\t\tlocal %s = "), name);

            match ty {
                LUA_TNONE => {
                    lua_pushliteral(L, "none");
                },
                LUA_TNIL => {
                    lua_pushliteral(L, "nil");
                },
                LUA_TBOOLEAN => {
                    if lua_toboolean(L, -2) != 0 {
                        lua_pushliteral(L, "true");
                    } else {
                        lua_pushliteral(L, "false");
                    }
                },
                LUA_TLIGHTUSERDATA => {
                    let p = lua_touserdata(L, -2);
                    lua_pushfstring(L, c_str!("%p"), p);
                },
                LUA_TNUMBER => {
                    let n = lua_tonumber(L, -2);
                    lua_pushfstring(L, c_str!("%f"), n);
                },
                LUA_TSTRING => {
                    let s = lua_tostring(L, -2);
                    lua_pushfstring(L, c_str!("'%s'"), s);
                },
                LUA_TTABLE => {
                    lua_pushliteral(L, "table");
                },
                LUA_TFUNCTION => {
                    lua_pushliteral(L, "function");
                },
                LUA_TUSERDATA => {
                    lua_pushliteral(L, "nil");
                },
                LUA_TTHREAD => {
                    lua_pushliteral(L, "thread");
                }
                _ => {}
            }

            lua_remove(L, -3);
        }

        let diff = (*Lj).base.offset_to((*Lj).top).unwrap();
        if diff - top >= 17 {
            lua_concat(L, (diff - top) as _);
        }
    }

    lua_concat(L, ((*Lj).base.offset_to((*Lj).top).unwrap() - top + 1) as _);
}