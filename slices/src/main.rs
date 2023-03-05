// There is no way to retrieve a part of a String. So we
// return the index of the end of the first word. But this
// is not good, because if the content of the changes,
// then the information returned by the string is invalid.
// This teaches us that if you want to make something safer
// you can use mechanisms of references to avoid problems 
// of multiple reference modifications. Using a &str param
// allows to the function to be more general than just
// defining the function only with &String.
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    // When we get a reference from we pepend an &
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    
    &s[..]
}

fn main() {
    let str_hello = String::from("Hello world");
    let first = first_word(&str_hello);
    // Clear changes the string, so this function takes a mutable
    // reference of str_hello. So you cannot take a mutable
    // reference from this variable and an inmutable reference
    // as well.
    println!("First word: {}", first);

    let hello = &str_hello[0..5];
    let world = &str_hello[6..];

    println!("First slice: {}", hello);
    println!("Second slice: {}", world);

    // String literals are slices. For this reason, they are
    // references so they are inmutable.
    let _s = "Hello world!";

    // Array slice
    let arr = [1, 2, 3, 4, 5, 6];
    let slice_arr = &arr[1..3];
    assert_eq!(slice_arr, &[2, 3]);
}
