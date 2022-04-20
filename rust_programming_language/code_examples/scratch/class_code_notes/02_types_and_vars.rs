
use std::mem;

// THis does not have an address space
// Means there's no real memory address
// Basically inlining
const MEANING_OF_LIFE: u8 = 42; // no fixed address

static Z: u8 = 64;

// Could also have this be mutable... which could be really not great. Rust fights against global static variables 

static mut OH_NO: u8 = 64;



fn main() {
    /******************************
     * Core Data Types
     *******************************/
    let a: u8 = 123;
    println!("a = {}", a);

    // u = unsigned, 0 to 2^N-1
    // i = signed, -2^(N-1) .. 2^(N-1) -1
    let mut b: i8 = 0;
    println!("b = {} before", b);
    b = 42;
    println!("b = {} after", b);

    let c = 123456789; // i32 = 32 bits = 4 bytes
    println!("c = {}, takes up {} bytes", c, mem::size_of_val(&c));

    // u8, u16, u32, u64, i8, i16, ...

    // usize, isize 
    let z: isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!("z={}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z*8);

    let d: char = 'x';
    println!("{} is a char, size = {} bytes", d, mem::size_of_val(&d));

    let e: f32 = 2.5;
    println!("{}, size = {} bytes", e, mem::size_of_val(&e));

    let f: bool = false;
    println!("{}, size = {} bytes", f, mem::size_of_val(&f));

    /******************************
     * Operators
     *******************************/

    let a = 17;
    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3); // powi means the power is integer
    let b_to_pi = f64::powf(b, std::f64::consts::PI); // powf means 
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    // bitwise
    let c = 1 | 2;

    /******************************
     * Scope and Shadowing
     *******************************/
     scope_and_shadowing();

    /******************************
     * Declaring and Using Constants
     *******************************/
    // See the const at the top

    unsafe {
        println!("OH NO = {}", OH_NO);
    }

    /******************************
     * Stack and Heap
     *******************************/    
    //  * THe Box is a reference or a pointer 
    //  * The Box itself is still on the stack
    //  * but it's pointing to something on the Heap
    //  * It just has an address that points to something on the heap
    //  * It means that to access, you have to follow 
    //  * the memory address and then dereference
}

fn scope_and_shadowing(){
    let a = 123;
    // let a = 777; // this would work because it's a later declaration
    { // Creating a scope
        let b = 456;
        println!("inside, b={}", b);
        // Didn't change the value 
        println!("inside, a={}", a);

        // But we can also shadow this
        let a = 777; // different a 
        println!("inside, a={}", a);
    }
    println!("outside, a={}", a);
}