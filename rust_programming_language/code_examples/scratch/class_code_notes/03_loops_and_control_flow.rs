
use std::io::stdin;

fn while_and_loop() {
    let mut x= 1;
    while x < 1000 {
        x *= 2;
        // Skips the rest of the loop
        if x == 64 { continue; }
        println!("x = {}", x);
    }

    let mut y = 1;
    loop { // while true 
        y *= 2;
        println!("y = {}", y);
        if y == 1<<10 {break;}
    }
}

fn for_loop() {
    // [1, 10] 
    // 11 is the upper bound
    for x in 1..11 {
        println!("x = {}", x);
    }

    for (idx, val) in (30..41).enumerate() {
        println!(" At idx {}, val={}", idx, val);
    }
}

// Match statement is exhaustive 
// Can test a range, so you should test that range
fn match_statement() {
    let country_code = 44;
    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7 => "Russia",
        1..=1000 => "Unknown",
        _ => "Invalid"
    };
    println!("The country with code {} is {}", country_code, country);
}

// Combination_lock

enum State {
    Locked,
    Failed, 
    Unlocked,
}

fn combination_lock() {
    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new();
    
    loop {
        match state {
            State::Locked => {
                // User enters a digit
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => {
                        entry.push_str(&input.trim_end());

                    }
                    Err(_) => continue
                };

                if entry == code {
                    state = State::Unlocked;
                    continue;
                }
                
                if !code.starts_with(&entry) {
                    state = State::Failed;
                }
            }
            State::Failed => {
                println!("FAILED");
                entry.clear();
                state = State::Locked;
                continue;
            }
            State::Unlocked => {
                println!("UNLOCKED");
                return;
            }
        }
    }
}

fn main() {
    while_and_loop();
    for_loop();
    match_statement();
    combination_lock();
}