// Spawning and Joining Threads

use std::thread;
use std::time;
use std::io;
use std::io::Write;

fn main() {
    let handle = thread::spawn(|| {
        for _ in 1..10 {
            print!("+");
            io::stdout().flush().unwrap();
            thread::sleep(time::Duration::from_millis(500));
        }
    });

    for _ in 1..10 {
        print!("-");
        io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(300));
    }
    
    // If we don't do this, we wouldn't have let all of it finish
    handle.join();
}