pub mod stack;
pub mod trait_example;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

#[inline]
pub fn fibonacci_slow(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => fibonacci_slow(n-1) + fibonacci_slow(n-2),
    }
}

#[inline]
pub fn fibonacci_fast(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    match n {
        0 => b,
        _ => {
            for _ in 0..n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}


// https://stackoverflow.com/a/19892721/1691005
#[inline]
pub fn fibonacci_realy_fast(n: u64) -> u64 {
    let f = n as f64;
    let i = n as i32;
    let sqrt = f64::sqrt(f);
    let a=(1.0+sqrt)/2.0;
    let b=(1.0-sqrt)/2.0;
    let fib = (f64::powi(a, i) - f64::powi(b, i)) / sqrt;
    fib.ceil() as u64
}
