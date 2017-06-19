mod testbed;
use testbed::*;

#[test]
fn metatable() {
    testbed::run(|state| {
        let m = Table::reference(&state, |cxt| {
            fn __add(stack: FunctionStack) -> Result<i32> {
                let a = stack.with_arg::<Table, _, _>(1, |cxt| {
                    cxt.get::<f64>("inner")
                })?;

                let b = stack.with_arg::<Table, _, _>(2, |cxt| {
                    cxt.get::<f64>("inner")
                })?;

                stack.push(a + b);

                Ok(1)
            }

            cxt.set("__add", __add);
        });

        state.set("testA", Table::new(|cxt| {
            cxt.set("inner", 5);
            cxt.set_meta(&m);
        }));

        state.set("testB", Table::new(|cxt| {
            cxt.set("inner", 10);
            cxt.set_meta(&m);
        }));

        assert_eq!(state.eval::<f64>("return testA + testB")?, 15f64);

        Ok(())
    })
}