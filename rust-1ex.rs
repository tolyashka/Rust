fn fib(n: u32) -> u32 {
    if n < 2 {
        return n;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

fn main() {
    println!("Числа Фибоначчи");
    let n: u32 = 20;
    println!("fib({}) = {}", n, fib(n));
}