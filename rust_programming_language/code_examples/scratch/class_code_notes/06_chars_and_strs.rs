// Strings
fn strings() {
    // Two different string types
    let s:&'static str = "Hello there!"; // &str = string slice
    //s="abcd" would not work

    for c in s.chars() {
        println!("{}", c);
    }

    println!("{:?}", s.chars().rev().collect::<String>());

    // String
    /*
    - Valid UTF8 sequence 
    - Heap construct 
    - If you want to build a string or something, that's where we'd want to use this
    */
    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8) {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }
    println!("letters = {:?}", &letters);

    // &str <> String conversions 
    let u:&str = &letters;

    // Concatenation
    // let z = letters + &letters.copy();

    let mut abc = String::from("hello world");
}

// String formatting 
fn formatting() {
    let name = "John";
    let greeting = format!("hi, I'm {}, nice to meet you", name);
    println!("{}", greeting);

    let hello = "hello";
    let rust = "rust";
    let hello_rust = format!("{} {}!", hello, rust);

    let run = "run";
    let forest = "forest";
    let rfr = format!("{0}, {1}, {0}", run, forest);
    println!("rfr = {}", rfr);

    let info = format!(
        "the name's {last}. {first} {last}",
        first = "james",
        last = "bond"
    );
    println!("{}", info);

    let mixed = format!(
        "{1} {} {0} {} {data}",
        "alpha",
        "beta",
        //"gamma", // unused, so this won't compile
        data="delta"
    );
    println!("{}", mixed);
}

// Number Guessing Game
// use rand::Rng;
use std::io::stdin;
fn number_guessing_game() {
    // Need to import this when I have wifi apparently 
    // let number = rand::thread_rng().gen_range(1, 101);
    let number = 76;
    loop {
        println!("Enter your guess: ");
        let mut buffer = String::new();
        match stdin().read_line(&mut buffer) {
            Ok(_) => {
                let parsed = buffer.trim_end().parse::<i64>();
                match parsed {
                    Ok(guess) => {
                        println!("Guessed: {}", guess);
                        if guess < 1 || guess > 100 {
                            println!("Your guess is out of range");
                        } else if guess < number {
                            println!("Your guess is too low");
                        } else if guess > number {
                            println!("Your guess is too high");
                        } else {
                            println!("Correct!");
                            break;
                        }
                    }
                    Err(e) => {
                        println!("Could not read your input. {}. Try again!", e);
                    }
                }

            },
            Err(_) => continue,
        }
    }
}


fn main() {
    strings();
    formatting();
    number_guessing_game();
}