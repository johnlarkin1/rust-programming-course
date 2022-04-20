

// Ownership
/*
Key point is memory safety 
So that means Rust needs tight memory ownership
*/

fn ownership() {
    let v = vec![1,2,3,4];
    // v OWNS what it is bound to 
    // it owns that memory
    let v2 = v; 
    // This is COPYing a pointer 
    // only one thing is bound to the resource at that time
    // the below would throw a value borrowed here after move
    // println!("v={:?}", v);

    /*
    Also seen this with closures
    */
    let foo = |v:Vec<i32>| ();
    // as soon as you call foo(v) you won't be able to use v anymore 

    let u = 1;
    let u2 = u;
    println!("u = {}", u);
    // u is a primitive 
    // for i32 it's very cheap to perform a copy 
    // Box does not implement the copy trait 
    let z = Box::new(1);
    let z2 = z;
    // println!("z = {}", *z); // ERROR

    let print_vector = | x | -> Vec<i32> {
        println!("x = {:?}", x);
        x
    };

    let vv = print_vector(v2);
}

// Borrowing 
fn borrowing() {
    let print_vector = | x:& Vec<i32> | {
        println!("x = {:?}", x);
    };
    let v = vec![3,2,1];
    // & means BORROW the vector for a bit 
    //
    print_vector(&v);
    print_vector(&v);

    let mut a = 40;
    /*
    How does the borrowing actually work?
    b is a mutable reference to a 
    Follow the reference (the asterik)
    */
    let b = &mut a;
    *b += 2;
    println!("a = {:?}", a);
    // The borrow has to match the mutability of what you're borrowing 

    let mut z = vec![3,2,1];
    // iterate over reference 
    for i in &z {
        println!("i = {}", i);
        // can't do this 
        // z.push(5);
    }
}

// Lifetimes

struct Person {
    name: String
}

impl Person {
    fn get_ref_name(&self) -> &String {
        &self.name
    }

    /*
    The compiler is automagically doing some 'lifetime ellision' here 
    but the code above is basically giving 

    fn get_ref_name<'a>(&'a self) -> &'a String {
        &self.name
    }
    */
}

struct Company<'z> {
    name: String,
    ceo: &'z Person
}

fn lifetimes() {
    /*
    Static definitions 
    &'static str 

    static is a lifetime 
    means it's going to live as long as the program lives 

    let's look at an example where you for sure need a lifetime
    */

    let boss = Person { name: String::from("Elon Musk") };
    let tesla = Company { name: String::from("Tesla"), ceo: &boss};

    // This won't work because the boss could become invalid and it would therefore invalidate the company

    // need to specify that the LIFETIME of the ceo and the company are going to be the same

    // Now that I've added the 'z and <'z> it means that we know the company and the 
    // ceo are going to have the same lifetime 
}

// Lifetime in Structure Implementation 
struct MyPerson<'a> {
    name: &'a str
}

impl<'a> MyPerson<'a> {
    fn talk(&self) {
        println!("Hi, my name is {}", self.name);
    }
}

fn lifetime_in_structure_impl() {
    let person = MyPerson { name: "John" };
    // doesn't compile because we need lifetime parameter
    person.talk();
}

// Reference-Counted Variables (Rc)
use std::rc::Rc;

struct NewPerson {
    name: Rc<String>
}

impl NewPerson {
    fn new(name: Rc<String>) -> NewPerson {
        NewPerson { name: name} 
    }
    fn greet(&self) {
        println!("Hi, my name is {}", self.name);
    }
}

fn reference_counted_variables() {
    let name = Rc::new("John".to_string());
    println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));
    {
        let person = NewPerson::new(name.clone());
        println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));
        person.greet();
    }
    println!("Name = {}, name has {} strong pointers", name, Rc::strong_count(&name));

    // One option is to make a reference counted variable 
    // My understanding is this is basically a smart pointer 
    // Keeps a count of references and as it turns zero , it gets dropped

}


// Atomic Reference-Counted Variables (Arc)
// RC is limited to a single thread
use std::thread;
use std::sync::Arc;

struct ArcPerson {
    name: Arc<String>
}

impl ArcPerson {
    fn new(name: Arc<String>) -> ArcPerson {
        ArcPerson { name: name }
    }
    fn greet(&self) {
        println!("Hi, my name is {}", self.name);
    }
}

fn atomic_reference_counted_variables() {
    let name = Rc::new("John".to_string());
    let person = NewPerson::new(name);
    // let t = thread::spawn(move || {
    //     person.greet();
    // });
    // println!("Name = {}", name);
    // t.join().unwrap();  

    // error because 
    /*
      `Rc<String>` cannot be sent between threads safely
      RC is not thread safe 
      They would have to implement the Send trait
    */

    let arcname = Arc::new("John".to_string());
    let arcperson = ArcPerson::new(arcname.clone());
    let t = thread::spawn(move || {
        arcperson.greet();
    });
    println!("Name = {}", arcname);
    t.join().unwrap();  
}

// Using a Mutex for Thread Safe Mutability
use std::sync::Mutex;

struct MutPerson {
    name: Arc<String>,
    state: Arc<Mutex<String>>
}

impl MutPerson {
    fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> MutPerson {
        MutPerson { name: name, state: state}
    }
    
    fn greet(&self) {
        let mut state = self.state.lock().unwrap();
        state.clear();
        state.push_str("excited");
        println!("Hello, my name is {} and I'm {}", self.name, state.as_str());
        self.state = state.as_str();
    }
}

fn using_a_mutex_for_thread_safe_mutability() {
    let name = Arc::new("John".to_string());
    let state = Arc::new(Mutex::new("bored".to_string()));
    let person = MutPerson::new(name.clone(), state.clone());
    let t = thread::spawn(move || {
        person.greet();
    });
    println!("Name = {}, state = {}", name, state.lock().unwrap().as_str());
    t.join().unwrap();  

    //mutex short for mutually exclusive
    // can't both modify the state field at the same time 
}


fn main() {
    ownership();
    borrowing();
    lifetimes();
    lifetime_in_structure_impl();
    reference_counted_variables();
    atomic_reference_counted_variables();
    using_a_mutex_for_thread_safe_mutability();
}