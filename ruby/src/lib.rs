#[no_mangle]
pub extern "C" fn fib(n: u64) -> u64 {
    tco_fib(n, 0, 1)
}

fn tco_fib(target: u64, previous: u64, next: u64) -> u64 {
    if target <= 1 {
        previous + next
    } else {
        tco_fib(target -1, next, previous + next)
    }
}