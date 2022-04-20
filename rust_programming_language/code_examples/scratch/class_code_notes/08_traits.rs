
trait Animal {
    // static functino
    fn create(name: &'static str) -> Self where Self:Sized;
    fn name(&self) -> &'static str;
    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

struct Human {
    name: &'static str
}

struct Cat {
    name: &'static str
}

impl Animal for Human {
    fn create(name:&'static str) -> Human {
        Human {name: name}
    }
    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says hello", self.name());
    }
}

impl Animal for Cat {
    fn create(name:&'static str) -> Cat {
        Cat {name: name}
    }
    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says meow", self.name());
    }
}

trait Summable<T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut result:i32=0;
        for x in self {
            result += x;
        }
        return result;
    }
}

fn traits() {
    // let h = Human { name: "John"};
    let h = Human::create("John");
    h.talk();

    let c = Cat { name: "April" };
    c.talk();

    let a = vec![1,2,3,4];
    println!("sum of {:?} = {}", a, a.sum());
}

// Trait Parameters
use std::fmt::Debug;

#[derive(Debug)]
struct Circle {
    radius: f64,
}

#[derive(Debug)]
struct Square {
    side: f64,
}

trait Shape {
    fn area(&self) -> f64;
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        self.radius * self.radius * std::f64::consts::PI
    }
}

// This means that the function implements that trait
// fn print_info(shape: impl Shape) {
//     println!("The area is {}", shape.area());
// }

fn print_info(ref shape: &(impl Shape + Debug)) {
    println!("{:?}", shape);
    println!("The area is {}", shape.area());
}
// below is easier to use - TraitBound syntax
fn print_info2<T: Shape + Debug>(ref shape: &T) {
    println!("{:?}", shape);
    println!("The area is {}", shape.area());
}

fn print_info3<T>(ref shape: &T) 
    where T: Shape + Debug {
    println!("{:?}", shape);
    println!("The area is {}", shape.area());
}

fn trait_parameters() {
    let c = Circle { radius: 2.0 };
    print_info(&c);
    print_info2(&c);
    print_info3(&c);
}

// Into 
struct Person {
    name: String
}

impl Person {
    // fn new(name: &str) -> Person {
    //     Person { name: name.to_string()}
    // }

    // whatever S is it has to be convertible into a string
    // using the Into trait
    // fn new<S: Into<String>>(name: S) -> Person {
    //     Person { name: name.into()}
    // }

    // another way to do it... with a where clause
    fn new<S>(name: S) -> Person 
        where S: Into<String> {
            Person { name: name.into()}
        }
}

fn into_example() {
    let john = Person::new("John");
    let name: String = "Jane".to_string();
    // let jane = Person::new(name.as_ref());
    let jane = Person::new(name);
    // we want two way initialization
}

// Drop
struct Creature {
    name: String
}

impl Creature {
    fn new(name: &str) -> Creature {
        println!("{} enterrs the game", name);
        Creature { name: name.into()}
    }
}

impl Drop for Creature {
    fn drop(&mut self) {
        println!("{} is dead", self.name);
    }
}

fn drop_example() {
    let mut clever: Creature;
    {
        let goblin = Creature::new("Jeff");
        println!("Game proceeds");
        // Global function
        // basically just moves the values.
        // not able to use it post that.
        clever = goblin;
        println!("End of scope");
    }
    println!("Scope about to end");
}

// Operator Overloading
use std::ops::{Add,AddAssign,Neg,Mul};

#[derive(Debug)]
// #[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
struct Complex<T> {
    real: T,
    imaginary: T
}

impl<T> Complex<T> {
    fn new(real: T, imaginary: T) -> Complex<T> {
        Complex::<T> { real, imaginary}
    }
}

impl<T> Add for Complex<T> 
    where T: Add<Output = T> // this means where T is some type T that implements the Add trait 
{
    type Output = Complex<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary
        }
    }
}

impl<T> AddAssign for Complex<T>
    where T: AddAssign<T>
{
    fn add_assign(&mut self, rhs: Self) {
        self.real += rhs.real;
        self.imaginary += rhs.imaginary;
    } 
}

impl<T> Mul for Complex<T> 
    where T:Mul<Output = T>
{
    type Output = Complex<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Complex {
            real: self.real * rhs.real,
            imaginary: self.imaginary * rhs.imaginary
        }
    }
}

// impl<T> Mul<i32> for Complex<T>
//     where T:Mul<Output = T>
// {
//     type Output = Complex<T>;

//     fn mul(self, rhs: i32) -> Self::Output {
//         Complex {
//             real: self.real * rhs,
//             imaginary: self.imaginary * rhs
//         }
//     }
// }


impl<T> Neg for Complex<T>
    where T: Neg<Output=T>
{
    type Output = Complex<T>;

    fn neg(self) -> Self::Output {
        Complex {
            real: -self.real,
            imaginary: -self.imaginary
        }
    }   
}

impl<T> PartialEq for Complex<T>
    where T:PartialEq
{
    fn eq(&self, rhs: &Self) -> bool {
        self.real == rhs.real && self.imaginary == rhs.imaginary
    }

    // fn ne(&self, other: &Self) -> basically negated off the other oine
}

/*
There's also parital eq
and full eq: x = x
NAN = not a number 0/0 inf/inf
NaNs infect thing... kinda viral
NaN == NaN -> THIS WOULD GIVE YOU FALSE 
Which is super annoying 
*/

impl<T: Eq> Eq for Complex<T> where T:Eq {} // automatically inferred from partialeq

// YOU CAN ALSO DERIVE THEM 
// THIS WILL USE PARTIAL COMPARISON

fn operator_overloading() {
    let mut a = Complex::new(1,2);
    let mut b = Complex::new(3,4);
    println!("a = {:?}", a);

    // This wouldn't compile yet 
    // it's a new trait
    a += b;
    println!("a = {:?}", a);

    // What about negation?
    // println!("-a = {:?}", a*1);

    // What about comparison?
    println!("a == a ={:?}", a==a);
}

// Static Dispatch
/*
Dispatching when invoking functions
Dispatch -> how does the compiler figure out what to call
Two ways: static and dynamic (not predetermined)
    Static is much more efficient 
*/

trait Printable {
    fn format(&self) -> String;
}

impl Printable for i32 {
    fn format(&self) -> String {
        // No semicolon because this is what we're returnign
        format!("i32: {}", *self)
    }
}

impl Printable for String {
    fn format(&self) -> String {
        // No semicolon because this is what we're returnign
        format!("string: {}", *self)
    }
}

fn print_it<T: Printable>(z: T) {
    // This is called monomorphisation
    println!("{}", z.format());
}

/*
At compile time, you're basically getting a function like this

fn print_it(z: String) {}
and also getting
fn print_it(z: i32) {}

Kinda like a substitution in C++

This is called static dispatch BECAUSE the decision actually happens at compile time 

*/

fn static_dispatch() {
    let a = 123;
    let b = "hello".to_string();
    // println!("{}", a.format());
    // println!("{}", b.format());

    print_it(a);
    print_it(b);
}

// Dynamic Dispatch
/*
Kinda overcomplicated things with the generic param
*/

fn print_it_too(z: &Printable) {
    println!("{}", z.format());
}

fn dynamic_dispatch() {
    let a = 123;
    let b = "hello".to_string();

    print_it_too(&a);
    print_it_too(&b);
}

/*
This *is* slightly different 

We lose the underlying object and we just have the underlying trait implementation 

Before we knew that we were calling a very specific implementation 

Dynamic dispatch is hapepning here 

This function has to look at the type of z and then know which type of .format() to call

It has to say like ok I'm a int or ok I'm a string and which format are we going to call 

It's a MORE expensive call because you're leaning on the compiler even more with dynamic
*/

// Why Dynamic Dispatch?

/*
Sometimes it's absolutely necessary
*/

fn why_dynamic_dispatch() {
    let shapes:[&Shape; 4] = [
        &Circle { radius : 1.0 },
        &Square { side :   3.0 },
        &Circle { radius : 2.0 },
        &Square { side :   13.0 },
    ];

    for (i, shape) in shapes.iter().enumerate() {
        // This is where the dynamic dispatch is happening
        println!("Shape #{} has area {}", i, shape.area());
    }
}

// Vectors of Different Types
enum VecCreature {
    Human(Human), // second arg in the paren is basically how we're storing the human enum
    Cat(Cat)
}


fn vectors_of_diff_types() {
    let mut creatures = Vec::new();
    // creatures.push(Human { name: "John" });
    // creatures.push(Cat { name: "Fluffy" });

    // Two approaches
    // 1 -> introduce an enum
    creatures.push(VecCreature::Human(
        Human { name: "John"}
    ));
    creatures.push(VecCreature::Cat(
        Cat { name: "Fluffy"}
    ));

    for c in creatures {
        // CANNOT do this 
        // c.talk();
        // because we just have a creature
        match c {
            VecCreature::Human(h) => h.talk(),
            VecCreature::Cat(c) => c.talk()
        }
    }

    // the below code would give: 
    // ^^^ doesn't have a size known at compile-time
    // let mut animals:Vec<Animal> = Vec::new();
    // animals.push(Human { name: "John" });
    // animals.push(Cat { name: "Fluffy" });

    // So we can use Box
    let mut animals:Vec<Box<dyn Animal>> = Vec::new();
    animals.push(Box::new(Human { name: "John" }));
    animals.push(Box::new(Cat { name: "Fluffy" }));

    for a in animals.iter() {
        a.talk();
    }
}

fn main() {
    traits();
    trait_parameters();
    into_example();
    drop_example();
    operator_overloading();
    static_dispatch();
    dynamic_dispatch();
    why_dynamic_dispatch();
    vectors_of_diff_types();
}
