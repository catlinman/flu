use libc;

type MSize = u32;

#[repr(C)]
#[derive(Copy, Clone)]
struct MRef {
    ptr32: u32
}

#[repr(C)]
#[derive(Copy, Clone)]
struct GCRef {
    gcptr32: u32
}

#[repr(C)]
struct GChead {
    nextgc: GCRef,
    marked: u8,
    gct: u8,
    unused1: u8,
    unused2: u8,
    env: GCRef,
    gclist: GCRef,
    metatable: GCRef
}

#[repr(C)]
struct GCstr {
    nextgc: GCRef,
    marked: u8,
    gct: u8,
    reserved: u8,
    unused: u8,
    hash: MSize,
    len: MSize
}

#[repr(C)]
struct GCtab {
    nextgc: GCRef,
    marked: u8,
    gct: u8,

    nomm: u8,
    colo: i8,
    array: MRef,
    gclist: GCRef,
    metatable: GCRef,
    node: MRef,
    asize: u32,
    hmask: u32
}

#[repr(C)]
union GCobj {
    gch: GChead,
    str: GCstr,
    th: luajit_State,
    tab: GCtab,
}

#[repr(C)]
struct Node {
    val: TValue,
    key: TValue,
    next: MRef,
    freetop: MRef
}

#[repr(C)]
struct SBuf {
    buf: *mut libc::c_char,
    n: MSize,
    sz: MSize
}

#[repr(C, align(8))]
#[derive(Copy, Clone)]
struct TValue(TValueUnion);

#[repr(C)]
#[derive(Copy, Clone)]
struct InnerTValueStruct {
    _union: InnerTValueStructUnion,
    it: u32
}

#[repr(C)]
#[derive(Copy, Clone)]
union InnerTValueStructUnion {
    gcr: GCRef,
    i: i32
}

#[repr(C)]
#[derive(Copy, Clone)]
union TValueUnion {
    _u64: u64,
    _inner: InnerTValueStruct
}

#[repr(C)]
struct luajit_State {
    nextgc: GCRef,
    marked: u8,
    gct: u8,
    dummy_ffid: u8,
    status: u8,
    glref: MRef,
    gclist: GCRef,
    base: *mut TValue,
    top: *mut TValue,
    maxstack: MRef,
    stack: MRef,
    openupval: GCRef,
    env: GCRef,
    cframe: *mut libc::c_void,
    stacksize: MSize
}

#[repr(C)]
struct GCState {
    total: MSize,
    threshold: MSize,
    currentwhite: u8,
    state: u8,
    nocdatafin: u8,
    unused2: u8,
    sweepstr: MSize,
    root: GCRef,
    sweep: MRef,
    gray: GCRef,
    grayagain: GCRef,
    weak: GCRef,
    mmudata: GCRef,
    stepmul: MSize,
    debt: MSize,
    estimate: MSize,
    pause: MSize
}

#[repr(C)]
struct _global_State {
    strhash: *mut GCRef,
    strmask: MSize,
    strnum: MSize,
    allocf: super::lua_Alloc,
    allocd: *mut libc::c_void,
    gc: GCState,
    tmpbuf: SBuf,
    nilnode: Node,
    strempty: GCstr,
    stremptyz: u8,
    hookmask: u8,
    dispatchmode: u8,
    vmevmask: u8,
    mainthref: GCRef,
    registrytv: TValue,
    tmptv: TValue,
    tmptv2: TValue,
    // TODO: ...
}

const LJ_TNIL: libc::c_uint = !0;
const LJ_TFALSE: libc::c_uint = !1;
const LJ_TTRUE: libc::c_uint = !2;
const LJ_TLIGHTUD: libc::c_uint = !3;
const LJ_TSTR: libc::c_uint = !4;
const LJ_TUPVAL: libc::c_uint = !5;
const LJ_TTHREAD: libc::c_uint = !6;
const LJ_TPROTO: libc::c_uint = !7;
const LJ_TFUNC: libc::c_uint = !8;
const LJ_TTRACE: libc::c_uint = !9;
const LJ_TCDATA: libc::c_uint = !10;
const LJ_TTAB: libc::c_uint = !11;
const LJ_TUDATA: libc::c_uint = !12;
const LJ_TNUMX: libc::c_uint = !13;

#[inline(always)]
unsafe extern "C" fn obj2gco<T>(v: *mut T) -> *mut GCobj {
    ::std::mem::transmute(v)
}

#[inline(always)]
unsafe extern "C" fn setgcref(r: &mut GCRef, gc: *mut GCobj) {
    r.gcptr32 = &(*gc).gch as *const _ as usize as u32;
}

#[inline(always)]
unsafe extern "C" fn setitype(o: *mut TValue, itype: u32) {
    ((*o).0)._inner.it = itype;
}

#[inline(always)]
unsafe extern "C" fn copyTV(L: *mut luajit_State, o1: *mut TValue, o2: *mut TValue) {
    *o1 = *o2;
    tvchecklive(L, o1);
}

#[inline(always)]
unsafe extern "C" fn tvchecklive(L: *mut luajit_State, o: *mut TValue) {

}

#[inline(always)]
unsafe extern "C" fn setgcV(L: *mut luajit_State, o: *mut TValue, v: *mut GCobj, itype: u32) {
    setgcref(&mut ((*o).0)._inner._union.gcr, v);
    setitype(o, itype);
    tvchecklive(L, o);
}

#[inline(always)]
unsafe extern "C" fn setstrV(L: *mut luajit_State, o: *mut TValue, v: *mut GCstr) {
    setgcV(L, o, obj2gco(v), LJ_TSTR);
}

#[inline(always)]
unsafe extern "C" fn settabV(L: *mut luajit_State, o: *mut TValue, v: *mut GCtab) {
    setgcV(L, o, obj2gco(v), LJ_TTAB);
}


#[inline(always)]
unsafe extern "C" fn incr_top(L: *mut luajit_State) -> bool {
    //TODO
    (*L).top = (*L).top.offset(1);
    true
}

#[inline(always)]
unsafe extern "C" fn mref<T>(r: MRef) -> *mut T {
    ((r.ptr32 as libc::uintptr_t) as *mut libc::c_void) as *mut T
}
#[inline(always)]
unsafe extern "C" fn gcref(r: GCRef) -> *mut GCobj {
    (r.gcptr32 as libc::uintptr_t) as *mut GCobj
}
#[inline(always)]
unsafe extern "C" fn tabref(r: GCRef) -> *mut GCtab {
    &mut (*gcref(r)).tab
}

extern "C" {
    fn lj_str_new(L: *mut luajit_State, str: *const libc::c_char, len: libc::size_t) -> *mut GCstr;
    fn lj_meta_tset(L: *mut luajit_State, str: *mut TValue, len: *mut TValue) -> *mut TValue;
    fn lj_meta_tget(L: *mut luajit_State, str: *mut TValue, len: *mut TValue) -> *mut TValue;
    fn lj_vm_call(L: *mut luajit_State, base: *mut TValue, nres1: libc::c_int);
}

#[no_mangle]
pub unsafe extern "C" fn flu_setlfield(L: *mut super::lua_State, idx: libc::c_int, k: *const libc::c_char, len: libc::size_t) {
    let L: *mut luajit_State = L as _;
    let mut o: *mut TValue = ::std::mem::uninitialized();
    let mut key: TValue = ::std::mem::uninitialized();
    let t = index2adr(L, idx);
    setstrV(L, &mut key, lj_str_new(L, k, len));
    o = lj_meta_tset(L, t, &mut key);
    if !o.is_null() {
        (*L).top = (*L).top.offset(-1);
        copyTV(L, o, (*L).top);
    } else {
        (*L).top = (*L).top.offset(3);
        copyTV(L, (*L).top.offset(-1), (*L).top.offset(-6));
        lj_vm_call(L, (*L).top.offset(-3), 1);
        (*L).top = (*L).top.offset(-2);
    }
}

#[no_mangle]
pub unsafe extern "C" fn flu_getlfield(L: *mut super::lua_State, idx: libc::c_int, k: *const libc::c_char, len: libc::size_t) {
    let L: *mut luajit_State = L as _;

    let mut v: *mut TValue = ::std::mem::uninitialized();
    let t = index2adr(L, idx);
    let mut key: TValue = ::std::mem::uninitialized();

    setstrV(L, &mut key, lj_str_new(L, k, len));
    v = lj_meta_tget(L, t, &mut key);
    if v.is_null() {
        (*L).top = (*L).top.offset(2);
        lj_vm_call(L, (*L).top.offset(-2), 2);
        (*L).top = (*L).top.offset(-2);
        v = (*L).top.offset(1);
    }
    copyTV(L, (*L).top, v);
    incr_top(L);
}

#[inline(always)]
unsafe extern "C" fn tvisnil(o: *mut TValue) -> bool {
    ((*o).0)._inner.it == LJ_TNIL
}

#[inline(always)]
unsafe extern "C" fn G(L: *mut luajit_State) -> *mut _global_State {
    mref::<_global_State>((*L).glref)
}

#[inline(always)]
unsafe extern "C" fn niltv(L: *mut luajit_State) -> *mut TValue {
    &mut (*G(L)).nilnode.val
}


unsafe extern "C" fn index2adr(L: *mut luajit_State, idx: libc::c_int) -> *mut TValue {
    if idx > 0 {
        let o = (*L).base.offset((idx - 1) as _);
        if o < (*L).top {
            o
        } else {
            niltv(L)
        }
    } else if idx > super::LUA_REGISTRYINDEX {
        (*L).top.offset(idx as _)
    } else if idx == super::LUA_GLOBALSINDEX {
        let mut o = &mut (*G(L)).tmptv;
        settabV(L, o, tabref((*L).env));
        o
    } else if idx == super::LUA_REGISTRYINDEX {
        unimplemented!();
    } else {
        if idx == super::LUA_ENVIRONINDEX {
            unimplemented!();
        } else {
            unimplemented!();
        }
    }
}