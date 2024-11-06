fn fibonacci_recursive(n: u32) -> u64 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
    }
}

fn main() {
    let n = 10; // Change this value to calculate a different Fibonacci number
    println!("The {}th Fibonacci number is: {}", n, fibonacci_recursive(n));
}
