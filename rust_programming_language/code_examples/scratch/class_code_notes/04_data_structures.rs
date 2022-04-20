
mod pm;

// Structs

struct Point {
    x: f64,
    y: f64
}

struct Line {
    start: Point,
    end: Point
}

fn structures() {
    let p1 = Point {x: 3.0, y: 4.0};
    println!("Point P is at ({}, {})", p1.x, p1.y);
    let p2 = Point {x: 5.0, y: 10.0};

    let myline = Line { start: p1, end: p2};
}

// Enumerations

// Enums can also have their own values 
enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8),
    CmykColor {
        cyan: u8, 
        magenta: u8, 
        yellow: u8,
        black: u8,
    } // no names because using tuple member 
}

fn enums() {
    // let c:Color = Color::RgbColor(0, 0, 0);
    let c:Color = Color::CmykColor{cyan: 0, magenta: 0, yellow: 0,black: 255};

    match c {
        Color::Red => println!("r"),
        Color::Green => println!("g"),
        Color::Blue => println!("b"),
        Color::RgbColor(0, 0, 0) 
        | Color::CmykColor{cyan: _, magenta: _, yellow: _, black: 255} => println!("black"),
        Color::RgbColor(r, g, b) => println!("rgb({}, {}, {})", r, g, b),
        _ => ()
    }
}

// Unions
/*
Typically used in C/C++
*/
// 32 bits... but you don't know which one is which 
// Rust kinda solves that. 
union IntOrFloat {
    i: i32,
    f: f32
}

fn process_value(iof: IntOrFloat) {
    // Has to happen in unsafe because of the union
    unsafe {
        match iof {
            IntOrFloat {i: 42} => {
                println!("meaning of life value");
            }
            // Eligible for a reinterpret cast
            // in C++ because we're taking an integer bit and interpreting as a float
            IntOrFloat {f} => {
                println!("value = {}", f);
            }
        }
    }
}

fn unions() {
    let mut iof = IntOrFloat { i: 123 };
    iof.i = 234;

    // Have to use an unsafe block 
    let value = unsafe { iof.i };
    // Compiler is basically saying you're going to assume responsibility of this. 

    process_value(IntOrFloat{i:42});
    process_value(IntOrFloat{f:42.0});
    process_value(IntOrFloat{i:5});
}

// Option<F,T>
fn option_ft() {
    let x = 3.0;
    let y = 1.0;
    // Option type -> Some(v) | None
    let result = if (y != 0.0) {
        Some(x/y) 
    } else {
        None
    };

    match result {
        Some(z) => println!("{}/{}={}", x, y, z),
        None => println!("Can't divide by zero")
    }

    if let Some(z) = result {
        println!("result = {}", z);
    }
}

// Arrays
fn arrays() {
    // Most simple data structure for sevbeeral of the same 
    // Fixed length of 5... bit redundant
    let mut a:[i32; 5] = [1,2,3,4,5];
    let mut b = [1,2,3,4,5]; // same data structure 
    println!("a has {} elems, first is {}", a.len(), a[0]);

    a[0] = 4;
    println!("a has {} elems, first is {}", a.len(), a[0]);
    println!("{:?}", a);

    if a != [1,2,3,4,5] {
        println!("Doesn't match");
    }

    // This means we have 10 elements, all with 1
    let c = [1; 10];
    for i in 0..c.len() {
        println!("{}", c[i]);
    }
    // c would take 40 bytes because each int is 4 bytes

    let d = [1u16; 10];
    // now d would only take 20 bytes

    // multidimensional array
    let mtx : [[f32;3];2] = [
        [1.0, 0.0, 0.0],
        [0.0, 2.0, 0.0]
    ];
    println!("{:?}", mtx);

    for i in 0..mtx.len() {
        for j in 0..mtx[i].len() {
            if i == j {
                println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
            }
        }
    }
}

// Slices 
fn use_slice(slice: &[i32]) {
    println!("First elem = {} and len = {}", slice[0], slice.len());
}

fn use_slice_mut(slice: &mut [i32]) {
    println!("First elem = {} and len = {}", slice[0], slice.len());
    slice[0] = 12039480;
}

fn slices() {
    let mut data = [1,2,3,4,5,6];
    use_slice(&data[1..4]); // positions at idx 1,2,3
    println!("{:?}", data);

    use_slice_mut(&mut data[1..4]); // positions at idx 1,2,3
    println!("{:?}", data);

}

// Tuples
fn sum_and_product(x: i32, y:i32) -> (i32, i32) {
    return (x+y, x*y);
}

fn tuples() {
    let x = 3;
    let y = 4;
    let sp = sum_and_product(x, y);
    println!("{:?}", sp);

    // Can also destructure 
    let (a, b) = sp;
    println!("Destructured: {} {}", a, b);
    let sp2 = sum_and_product(12, 23);
    let combined = (sp, sp2);

    let foo = (true, 42.0, -1i8);

    let meaning = (42); // this is actually just an int 
    let meaning_tuple = (42,); // this is now a tuple with a single elemtn
    println!("{:?}", meaning);
    println!("{:?}", meaning_tuple);
}

// Pattern matching 
// -> see pm.rs

// Generics
struct Point<T> {
    x: T,
    y: T
}

fn generics() {
    let a = Point { x: 0, y: 0};
    let b = Point { x: 1.2, y: 3.4};
    let c:Point<u16> = Point { x: 0, y: 0}; // can also explicitly declare it
}

struct Line<T> {
    start: Point<T>,
    end: Point<T>
}

// Overview
/*

*/


fn main() {
    structures();
    enums();
    unions();
    option_ft();
    arrays();
    slices();
    tuples();
    pm::pattern_matching();
    generics();
}