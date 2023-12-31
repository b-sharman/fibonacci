use std::collections::HashMap;

fn recursion(n: u32) -> u32 {
    match n {
        0 | 1 => 1,
        _ => recursion(n-1) + recursion(n-2)
    }
}

fn dp(cache: &mut HashMap<u32, u32>, n: u32) -> u32 {
    match cache.get(&n) {
        Some(x) => *x,
        None => {
            let retval = dp(cache, n-1) + dp(cache, n-2);
            cache.insert(n, retval);
            retval
        }
    }
}

fn main() {
    println!("recursion:");
    for i in 0..40 {
        println!("{}: {}", i, recursion(i));
    }

    let mut cache: HashMap<u32, u32> = HashMap::new();
    cache.insert(0, 1);
    cache.insert(1, 1);
    println!("\ndp:");
    for i in 0..40 {
        println!("{}: {}", i, dp(&mut cache, i));
    }
}
