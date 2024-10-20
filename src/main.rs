use std::time::Instant;

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

fn main() {
    let n = 40;
    let start_time = Instant::now();

    let result = fibonacci(n);

    let duration = start_time.elapsed();
    println!("Fibonacci({}) = {}", n, result);
    println!("耗时: {} 毫秒", duration.as_millis());
}
