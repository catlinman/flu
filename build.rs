extern crate pkg_config;
extern crate gcc;

fn main() {
    match pkg_config::find_library("lua5.1") {
        Ok(_) => return,
        Err(..) => {}
    };

    gcc::Config::new()
        .file("dist/lua-5.1.5/src/lapi.c")
        .file("dist/lua-5.1.5/src/lauxlib.c")
        .file("dist/lua-5.1.5/src/lbaselib.c")
        .file("dist/lua-5.1.5/src/lcode.c")
        .file("dist/lua-5.1.5/src/ldblib.c")
        .file("dist/lua-5.1.5/src/ldebug.c")
        .file("dist/lua-5.1.5/src/ldo.c")
        .file("dist/lua-5.1.5/src/ldump.c")
        .file("dist/lua-5.1.5/src/lfunc.c")
        .file("dist/lua-5.1.5/src/lgc.c")
        .file("dist/lua-5.1.5/src/linit.c")
        .file("dist/lua-5.1.5/src/liolib.c")
        .file("dist/lua-5.1.5/src/llex.c")
        .file("dist/lua-5.1.5/src/lmathlib.c")
        .file("dist/lua-5.1.5/src/lmem.c")
        .file("dist/lua-5.1.5/src/loadlib.c")
        .file("dist/lua-5.1.5/src/lobject.c")
        .file("dist/lua-5.1.5/src/lopcodes.c")
        .file("dist/lua-5.1.5/src/loslib.c")
        .file("dist/lua-5.1.5/src/lparser.c")
        .file("dist/lua-5.1.5/src/lstate.c")
        .file("dist/lua-5.1.5/src/lstring.c")
        .file("dist/lua-5.1.5/src/lstrlib.c")
        .file("dist/lua-5.1.5/src/ltable.c")
        .file("dist/lua-5.1.5/src/ltablib.c")
        .file("dist/lua-5.1.5/src/ltm.c")
        .file("dist/lua-5.1.5/src/lundump.c")
        .file("dist/lua-5.1.5/src/lvm.c")
        .file("dist/lua-5.1.5/src/lzio.c")
        .file("dist/lua-5.1.5/src/print.c")
        .define("LUA_COMPAT_ALL", None)
        .include("dist/lua-5.1.5/src")
        .compile("liblua.a");
}
