fn add(a: u32, b: u32) -> u32 {
    let carry = (a & b) << 1;
    let result = a ^ b;
    if carry == 0 {
        result
    } else {
        add(carry, result)
    }
}

fn main() {
    let a = 12;
    let b = 11;
    let res = add(a, b);
    println!("Result: {}", res);
}
