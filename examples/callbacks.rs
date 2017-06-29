extern crate flu;

use flu::Value;
use flu::ffi;
use flu::Function;
use flu::ToLua;
use flu::errors::*;

#[inline(never)]
fn example() -> Result<()> {
    let mut state = flu::State::new();

/*
    state.set("lib", flu::Table::new(|cxt| {
        cxt.set("add", add);
        cxt.set("test2", state.get::<f64>("test").unwrap());
    }));*/


    /*state.with("test", |cxt| {
        cxt.set_meta()
    });*/

    /*let m = flu::Table::reference(&state, |cxt| {
        fn __add(stack: flu::FunctionStack) -> Result<i32> {
            let a = stack.with_arg::<flu::Table, _, _>(1, |cxt| {
                cxt.get::<f64>("a")
            })?;

            let b = stack.with_arg::<flu::Table, _, _>(2, |cxt| {
                cxt.get::<f64>("a")
            })?;

            stack.push(a + b);

            Ok(1)
        }

        cxt.set("__index", cxt.as_table());
        cxt.set("__add", __add);
    });*/

    let b = 4;
    /*state.set("test", flu::Table::new(|cxt| {
        cxt.set("b", b);
        cxt.set_meta(&m);
    }));*/

    fn foo(stack: flu::FunctionStack) -> Result<i32> {
        stack.push(flu::Table::new(|cxt| {
            cxt.set("first", stack.value::<Value>(1).unwrap());
            cxt.set("second", stack.value::<Value>(2).unwrap());
            cxt.set("third", stack.value::<Value>(3).unwrap());
        }));

        Ok(1)
    }

    state.set("foo", foo);

    state.eval(r#"
    t = foo(14, 5, 6)
    print(t.first)
    print(t.second)
    print(t.third)
"#)?;

    Ok(())
}

fn main() {
    if let Err(ref e) = example() {
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
    }
}
