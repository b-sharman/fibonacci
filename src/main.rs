use std::collections::HashMap;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_recursion() {
        assert_eq!(1, recursion(0));
        assert_eq!(1, recursion(1));
        assert_eq!(8, recursion(5));
    }

    #[test]
    fn test_dp() {
        let mut dp = DP::new();
        assert_eq!(1, dp.nth_fib(0));
        assert_eq!(1, dp.nth_fib(1));
        assert_eq!(8, dp.nth_fib(5));
    }
}

fn recursion(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        _ => recursion(n - 1) + recursion(n - 2),
    }
}

struct DP {
    cache: HashMap<u32, u32>,
}

impl DP {
    fn new() -> Self {
        Self {
            cache: HashMap::from([(0, 1), (1, 1)]),
        }
    }

    fn nth_fib(&mut self, n: u32) -> u32 {
        match self.cache.get(&n) {
            Some(n) => *n,
            None => {
                let retval = self.nth_fib(n - 1) + self.nth_fib(n - 2);
                self.cache.insert(n, retval);
                retval
            }
        }
    }
}

fn main() {
    println!("recursion:");
    for i in 0..40 {
        println!("{}: {}", i, recursion(i));
    }

    let mut dp = DP::new();
    println!("\ndp:");
    for i in 0..40 {
        println!("{}: {}", i, dp.nth_fib(i));
    }
}
