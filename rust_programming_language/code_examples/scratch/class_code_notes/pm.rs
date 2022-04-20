

fn how_many(x: i32) -> &'static str {
    match x {
        0 => "no",
        1 | 2 => "one or two",
        12 => "a dozen",
        9...11 => "lots of", // inclusive 
        _ if x % 2 == 0 => "some",
        _ => "a few"
    }
}

pub fn pattern_matching() {
    for x in 0..13 {
        println!("{}: I have {} oranges", x, how_many(x));
    }

    let point = (3,4);

    // You can also infer the ref mut for example of some of the matched patterns
    match (point) {
        (0, 0) => println!("Origin"),
        (0, y) => println!("x axis, y = {}", y),
        (x, 0) => println!("y axis, x = {}", x),
        (x, y) => println!("x = {}, y = {}", x, y)
    }
}