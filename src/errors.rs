error_chain! {
    types {
        Error, ErrorKind, ResultExt, Result;
    }

    errors {
        RuntimeError(t: String) {
            description("Lua runtime error")
            display("Lua runtime error: {}", t)
        }
        SyntaxError(t: String) {
            description("Lua syntax error")
            display("Lua syntax error: {}", t)
        }
        TypeError(t: String, g: String) {
            description("invalid type")
            display("invalid type: expected '{}', got '{}'", t, g)
        }
        MemoryError {
            description("memory allocation error")
            display("memory allocation error")
        }
        ErrorHandler {
            description("error while running the error handler function")
            display("error while running the error handler function")
        }
        ArgCount(start: i32, end: i32, sz: i32) {
            description("wrong argument count")
            display("wrong argument count ({} to {} expected, got {})", start, end, sz)
        }
        ArgTypeError(narg: i32, tname: String) {
            description("type error")
            display("type error #{} ({})", narg, tname)
        }
    }
}
