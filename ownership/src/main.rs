fn takes_ownership(some_str: String) {
    println!("Str val = {}", some_str);
}

fn compute(x: i32) {
    println!("Int val = {}", x);
}

fn gives_ownership() -> String {
    let s = String::from("Giving ownership str");
    s
}

fn return_str_len(some_str: String) -> (String, usize) {
    let length = some_str.len();
    (some_str, length)
}

fn takes_gives_back_ownership(some_str: String) -> String {
    some_str
}

fn main() {
    // String literal. String literals are imutable.
    // They are hardcoded in the source code. Here
    // the content is known at compile time. String
    // literals are fast and efficient.
    let _s = "Hello";

    // String types can be mutable and with unknown
    // length. Here, the content is not known at
    // compile time.
    let mut s = String::from("Hello");
    s.push_str(" World");
    println!("{}", s);

    // Move. 
    
    // At this piece of code, we are copying
    // the value of x (5) into the y variable because
    // both are in the stack. They are known values in
    // compile time and they are pushed in the stack.
    // The stack has two 5 values. 
    let x = 5;
    let _y = x;

    // The pointers are in the stack. But the contents
    // of the String is in the heap. So wen we do this
    // we are copying only the information from the pointer.
    // Here s1 is not longer valid because s2 has the
    // ownership.
    let s1 = String::from("Str");
    let mut s2 = s1.clone();
    s2.push_str("Hola");
    println!("s1 = {}, s2 = {}", s1, s2);

    // Ownership in functions. Passing variables to a
    // function will move or Copy as in assignment.
    let s = String::from("Hola");
    takes_ownership(s);

    // This works because i32 is Copy
    let x = 39;
    compute(x);
    println!("{}", x);

    // Ownership in functions
    let s = gives_ownership();
    println!("Give own => {}", s);

    let s2 = takes_gives_back_ownership(s);
    println!("Takes and give back => {}", s2);

    // Functions with tuples
    let (string, length) = return_str_len(s2);
    println!("String = {}, length = {}", string, length);
}
