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

#[test]
fn metatable_class() {
    testbed::run(|state| {
        state.set("Class", Table::new(|meta_cxt| {
            fn new(stack: FunctionStack) -> Result<i32> {
                stack.state.dump();

                let arg = stack.arg::<String>(2).unwrap_or(String::from("no bunny :-("));

                stack.push(Table::new(|cxt| {
                    cxt.set("bunny", arg.clone());
                    cxt.set_meta(stack.get::<Table>("Class").unwrap());

                    /* FIXME: broken
                    stack.with::<Table, _, _>("Class", |meta| {

                        cxt.set_meta(meta.as_table());

                        Ok(())
                    }).unwrap();*/
                }));


                Ok(1)
            }

            fn bunny(stack: FunctionStack) -> Result<i32> {
                let n = stack.with_arg::<Table, _, _>(1, |cxt| {
                    cxt.get::<String>("bunny")
                })?;

                stack.push(n);

                Ok(1)
            }

            meta_cxt.set("__index", meta_cxt.as_table());
            meta_cxt.set("new", new);
            meta_cxt.set("get_bun", bunny);
        }));


        assert_eq!(state.eval::<String>(
            "local b = Class:new('abc') return b:get_bun()"
        )?, "abc");

        Ok(())
    })
}
