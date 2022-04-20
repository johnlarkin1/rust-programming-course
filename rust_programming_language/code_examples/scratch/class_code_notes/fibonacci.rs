

fn fibonacci(n: i32) {
    let mut vec = vec![0, 1];
    for i in 2..n {
        vec.append(vec[i as usize -1] + vec[i as usize -2]);
    }
    println!("Final fib val: {}", vec[n-1]);
    return vec[n-1];
}

fn main() {
    let n = 3;
    let ans = fibonacci(n);
}