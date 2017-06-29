mod testbed;
use testbed::*;

#[test]
fn values() {
    testbed::run(|state| {
        fn swap_any_type_func(stack: FunctionStack) -> Result<i32> {
            let a = stack.arg::<Value>(1)?;
            let b = stack.arg::<Value>(2)?;

            assert_eq!(a, Value::Bool(true));
            assert_eq!(b, Value::String(String::from("testabc")));

            Ok(0)
        }

        state.set("swap_any_type_func", swap_any_type_func);

        state.eval::<()>(
r#"
swap_any_type_func(true, "testabc")
"#
        )?;

        Ok(())
    })
}