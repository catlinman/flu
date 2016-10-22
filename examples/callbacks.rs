extern crate flu;

fn main() {
    let cxt = flu::Context::new();
    let (mut x, mut y) = (25f32, 25f32);

    println!("x: {}, y: {}", x, y);

    cxt.set("move_x", |cxt: &mut flu::Context| {
        let speed = cxt.pop::<f32>();
        x += speed;
        0
    });

    cxt.set("move_y", |cxt: &mut flu::Context| {
        let speed = cxt.pop::<f32>();
        y += speed;
        0
    });

    cxt.eval("for i=1,10 do move_x(1.0) move_y(2.0) end").unwrap();

    println!("x: {}, y: {}", x, y);
}
