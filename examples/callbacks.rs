extern crate flu;

fn main() {
    let ctx = flu::Context::new();
    let (mut x, mut y) = (25f32, 25f32);

    println!("x: {}, y: {}", x, y);

    ctx.set("move_x", |ctx: &mut flu::Context| {
        let speed = ctx.pop::<f32>();
        x += speed;
        0
    });

    ctx.set("move_y", |ctx: &mut flu::Context| {
        let speed = ctx.pop::<f32>();
        y += speed;
        0
    });

    ctx.eval("for i=1,10 do move_x(1.0) move_y(2.0) end").unwrap();

    println!("x: {}, y: {}", x, y);
}
