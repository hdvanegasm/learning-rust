// The Option enum represents when data can 
// be something or nothing. Rust does not have
// a NULL value as other programming langauges.
// If you try to use a null value, you will get
// an error of some kind. The Option enum is
// used when you have a value that can have a
// NULL or not-NULL.

#[allow(unused)]
fn main() {
    let some_number = Some(5);
    let some_string = Some("Hello");
    let absent_number: Option<i32> = None;
}
