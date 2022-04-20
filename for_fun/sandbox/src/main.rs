// Question 1 
fn q1_mults_of_3_and_5() -> u32 {
    let mut running_sum = 0;
    for i in 1..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            running_sum += i;
        }
    }
    return running_sum;
}

// Question 2 - Even Fib Numbers
use std::iter::Iterator;

// boring old struct
struct Fibonacci {
    a: u64,
    b: u64
}

// function on struct
impl Fibonacci {
    fn new() -> Self {
        Fibonacci {
            a: 1,
            b: 0
        }
    }
}

// implementing a trait for our struct
impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let next_term_to_return = self.b;
        let next_val_to_generate = self.b + self.a;
        self.b = self.a;
        self.a = next_val_to_generate;
        Some(next_term_to_return)
    }
}

fn q2_even_fib_numbers() -> u64 {
    let mut sum_of_even_terms = 0;
    let limit = 4*1000*1000;
    let mut fib = Fibonacci::new();
    let mut fib_term = fib.next().unwrap();
    while fib_term < limit {
        if fib_term % 2 == 0 {
            sum_of_even_terms += fib_term;
        }
        fib_term = fib.next().unwrap();
    }
    return sum_of_even_terms;
}

// Question 3 - Largest Prime Factor 
fn q3_largest_prime_factor() -> u64 {
    let target_number: u64 = 600851475143;
    let mut mut_tgt_number = target_number;
    let mut max_prime = 0;
    let sqrt_tgt_num = (target_number as f64).sqrt() as i64 + 1;

    for i in (3..sqrt_tgt_num).step_by(2) {
        let potential_prime = i as u64;

        while mut_tgt_number % potential_prime == 0 {
            max_prime = potential_prime;
            mut_tgt_number /= potential_prime;
        }
    }
    return max_prime;
}

// Question 4 - Largest Palindrome Product
fn is_number_palindrome(number: u64) -> bool {
    let str_form = number.to_string();
    str_form == str_form.chars().rev().collect::<String>()
}
fn q4_largest_palindrome_product() -> u64 {
    // Find the largest palindrome made from the product of two 3-digit numbers
    // Are two for loops that costly?
    let mut largest_palindrome = 0;
    for i in (100..1000).rev() {
        if largest_palindrome >= i * 999 {
            // We know here that we're not going to be able to
            // multiple any other two numbers together 
            // and get a larger one
            break;
        }
        for j in (100..1000).rev() {
            let candidate = i * j;
            if candidate > largest_palindrome && is_number_palindrome(candidate) {
                largest_palindrome = candidate;
            }
        }
    }
    return largest_palindrome;
}

// Question 5 - Smallest Multiple
fn is_multiple(n: u64, divisors: &Vec<u32>) -> bool{
    for divisor in divisors {
        if n % (*divisor as u64) != 0 {
            return false;
        }
    }
    return true;
}
fn q5_smallest_multiple() -> u64 {
    // Smallest positive number that is evently divisible by all of the numbers from 1 to 20
    // 2520 is the smallest mult of all numbers 1 through 10 
    // So any mult of 2520 is also divisbly by 1 through 10
    // Any non multiple can be ignored 
    let mut n = 2520;
    let divisors = (11..21).rev().collect::<Vec<u32>>();
    while !is_multiple(n, &divisors) {
        n += 2520;
    }
    return n
}

fn main() {
    println!("Rust Project Euler Solving...");
    println!("Q1 Ans: {}", q1_mults_of_3_and_5());
    println!("Q2 Ans: {}", q2_even_fib_numbers());
    println!("Q3 Ans: {}", q3_largest_prime_factor());
    println!("Q4 Ans: {}", q4_largest_palindrome_product());
    println!("Q5 Ans: {}", q5_smallest_multiple());
}
