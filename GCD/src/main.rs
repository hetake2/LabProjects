use std::io;
use std::time::SystemTime;

// Easy input function.
fn get_input<T>(text: T) -> i32
where
    T: ToString,
{
    println!("{}", text.to_string());
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    return buf.trim().parse::<i32>().unwrap();
}

// Recursive GCD.
fn gcd_rec(mut a: i32, mut b: i32) -> i32 {
    if b == 0 {
        return a;
    }
    return gcd_rec(b, a % b);
}

// Normal GCD.
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = a % b;
        a = b;
        b = temp;
    }
    return a;
}

fn main() {
    let a = get_input("Write down first variable:");
    let b = get_input("Write down second variable:");
	// Comparing performance time.
    let mut start = SystemTime::now();
    println!("GCD({}, {}) = {}", a, b, gcd(a, b));
    let mut end = SystemTime::now();
    println!("{:?}", end.duration_since(start));
    start = SystemTime::now();
    println!("GCD_REC({}, {}) = {}", a, b, gcd_rec(a, b));
    end = SystemTime::now();
    println!("{:?}", end.duration_since(start));
}
