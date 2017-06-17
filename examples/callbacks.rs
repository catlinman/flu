extern crate flu;

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

    state.set("test", 5);

    state.set("lib", flu::Table::new(|cxt| {
        cxt.set("add", add);
        cxt.set("test2", state.get::<f64>("test").unwrap());
    }));

    /*state.set("testtable", flu::Table::new(|cxt| {
        cxt.set("a", 3);
        cxt.set("emptysubtable", flu::Table::new(|cxt| {}));
        cxt.set("subtable", flu::Table::new(|cxt| {
            cxt.set("foo", "bar");
            cxt.set("bar", 45.23);
            cxt.set("noot", cxt.get::<String>("foo").unwrap());
            cxt.set("test", 3.14);
        }));
    }));*/

    state.eval(r#"
function tprint (tbl, indent)
  if not indent then indent = 0 end
  for k, v in pairs(tbl) do
    formatting = string.rep("  ", indent) .. k .. ": "
    if type(v) == "table" then
      print(formatting)
      tprint(v, indent+1)
    else
      print(formatting .. v)
    end
  end
end

print(lib.add(5))
print(lib.add(5, 10))
print(lib.add(5, 10, 100, 200))

--tprint(testtable)
    "#)?;

    //state.set("test2", test2);
    //state.set("test", test as flu::LuaUncheckedFn);

    //state.eval("test2(1, 2, \"hello\")")?;
    //state.eval("test(1, 2, \"hello\")")?;

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
