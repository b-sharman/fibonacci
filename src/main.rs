fn recursion(n: i32) -> i32 {
    match n {
        0 | 1 => 1,
        _ => recursion(n-1) + recursion(n-2)
    }
}

fn main() {
    println!("recursion:");
    for i in 0..10 {
        println!("{}: {}", i, recursion(i));
    }
}
