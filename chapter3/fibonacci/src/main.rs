fn main() {
    let n = fibonacci(1);
    println!("fib(1) = {n}");
    let n = fibonacci(2);
    println!("fib(2) = {n}");
    let n = fibonacci(20);
    println!("fib(20) = {n}");
}
fn fibonacci(n: i32) -> i32 {
    if n == 1 || n == 0 {
        return 1;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}
