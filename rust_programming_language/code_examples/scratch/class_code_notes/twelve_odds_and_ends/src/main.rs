extern crate rand;
extern crate twelve_phrases;

use twelve_phrases::greetings::{english,french};

fn main() {
    println!("English: {}, {}",
        english::hello(),
        english::goodbye(),
    );

    println!("French: {}, {}",
        french::hello(),
        french::goodbye(),
    );
}
