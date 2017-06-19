extern crate flu;

pub use self::flu::*;

pub fn run<T, F: Fn(flu::State) -> Result<T>>(func: F) {
    let state = flu::State::new();

    if let Err(ref e) = (func)(state) {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        panic!("{}", e);
    }
}