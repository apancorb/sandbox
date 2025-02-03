fn main() {
    fib(3);
}

fn fib(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    fib(n - 1) + fib(n - 2)
}
