fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point
    println!("{} and {}", r1, r2);
    println!("{} and {}", r1, r2);

    let r3 = &mut s; // no problem
    println!("{}", r3);
}