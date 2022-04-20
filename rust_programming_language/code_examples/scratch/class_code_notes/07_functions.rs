

// Functions and Function Arguments
fn print_value(x: i32) {
    println!("value = {}", x);
}

fn increase(x: &mut i32) {
    *x+=1;
}

fn functions() {
    print_value(33);
    let mut z = 1;
    increase(&mut z);
    print_value(z);
}

// Methods
struct Point {
    x: f64,
    y: f64
}
struct Line {
    start: Point,
    end: Point
}

impl Line {
    fn len(&self) -> f64 {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        (dx*dx + dy*dy).sqrt()
    }
}

fn methods() {
    let p = Point { x: 3.0, y: 4.0};
    let p2 = Point { x: 5.0, y: 10.0};
    let my_line = Line {start: p, end: p2};
    println!("Length = {}", my_line.len());
}

// Closures 
fn say_hello() { println!("hello"); }
fn closures() {
    // Can store functions in variable
    let sh = say_hello; 
    sh();

    // Only going to exist in this function
    let plus_one = |x:i32| -> i32 { x + 1};
    let a = 6;
    println!("{} + 1 = {}", a, plus_one(a));

    let plus_two = |x| {
        let mut z = x;
        z += 2; 
        z 
    };
    println!("{} + 2 = {}", 3, plus_two(3));
}

// Higher order functions
fn is_even(x: u32) -> bool {
    x % 2 == 0
}

fn greater_than(limit : u32) -> impl Fn(u32) -> bool {
    // move keyword extends lifetime
    move |y| y > limit
}

fn higher_order_functions() {
    // functions that take functions 
    // f(g) -> let x = g();

    // functions that return functions
    // generators
    // f() -> g

    // sum of all even squares < 500
    let limit = 500;
    let mut sum = 0;

    let above_limit = greater_than(limit);
    for i in 0.. {
        let isq = i*i;
        if above_limit(isq) {
            break;
        } else if is_even(isq) {
            sum += isq;
        }
    }
    println!("loop sum = {}", sum);

    let sum2 = (0..)
        .map(|x| x*x)
        .take_while(|&x| x < limit)
        .filter(|&x| is_even(x))
        .fold(0, |sum, x| sum + x);
    println!("hof sum = {}", sum2);
}

fn main() {
    functions();
    methods();
    closures();
    higher_order_functions();
}