extern crate flu;

use flu::Value;
use flu::ffi;
use flu::Function;
use flu::ToLua;
use flu::errors::*;

#[inline(never)]
fn example() -> Result<()> {
    let mut state = flu::State::new();

    fn add(stack: flu::FunctionStack) -> Result<i32> {
        let sz = stack.check_size(1..10)?;

        let mut sum = 0f64;
        for i in 1..(sz + 1) {
            sum += stack.arg::<f64>(i)?;
        }
        stack.push(sum);

        Ok(1)
    }
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
test = test + test
print(test) -- prints 8

function add(a, b)
    local q = 5
    table.insert(q, b)
    return a + b
end

function addtwo(a, b)
    local m = function(r) end
    return add(a, b) + add(a, b)
end

--local u = addtwo(3, "oops")
    "#)?;

    Ok(())
}

struct DropTest;
impl Drop for DropTest {
    fn drop(&mut self) {
        println!("{}", "drop!!");
    }
}

extern "C" fn test(stack: flu::UncheckedFunctionStack) -> i32 {
    let t = DropTest;
    let sz = stack.check_size(1..3);

    let a: i32 = stack.arg(1);
    let b: i32 = stack.arg(2);
    let c: String = stack.arg(3);

    println!("{:?}, {:?}, {:?}", a, b, c);

    //let a = flu::arg::<i32>(1)?;
    0
}

fn test2(stack: flu::FunctionStack) -> Result<i32> {
    let t = DropTest;
    let sz = stack.check_size(1..3)?;

    let a: i32 = stack.arg(1)?;
    let b: i32 = stack.arg(2)?;
    let c: String = stack.arg(3)?;

    println!("{:?}, {:?}, {:?}", a, b, c);

    Ok(0)
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
