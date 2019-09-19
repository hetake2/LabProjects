fn main() {
    println!("\n\t***TRUTH TABLE***");
    println!("P:\tQ:\t!(p || q) == (p && !q):");
    // TT
    let mut p: bool = true;
    let mut q: bool = true;
    let mut r = !(p || q) == (p && !q);
    println!("{:?}\t{:?}\t\t{:?}", p, q, r);
    // TF
    q = false;
    r = !(p || q) == (p && !q);
    println!("{:?}\t{:?}\t\t{:?}", p, q, r);
    // FT
    p = false;
    q = true;
    r = !(p || q) == (p && !q);
    println!("{:?}\t{:?}\t\t{:?}", p, q, r);
    // FF
    q = false;
    r = !(p || q) == (p && !q);
    println!("{:?}\t{:?}\t\t{:?}", p, q, r);
}
