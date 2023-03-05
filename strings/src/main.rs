fn main() {
    // Creating a new empty String
    let mut _s = String::new();

    // to_string() can be used by any type that implements the Display trait
    let data = "The data of the string is this";
    let _s = data.to_string();

    // Updating an String 
    let append = "to the end";
    let mut s = String::from("This is a string that appends data ");
    s.push_str(append);
    println!("Content: {}", s);

    // Appending characters
    let mut s = String::from("foo");
    s.push('b');
    println!("Content: {}", s);

    // + operator
    let s1 = String::from("Hello ");
    let s2 = String::from("Hernán.");
    let s3 = s1 + &s2;
    println!("Content: {}", s3);

    let s4 = String::from("Hola a todos");
    let s5 = s2 + &s3 + &s4;
    println!("Content: {}", s5);

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    let join = format!("{}-{}-{}", tic, tac, toe);
    println!("Content: {}", join);

    // Indexing Strings.
    let hello = "Здра";
    // The following len will be 24 because each letter in the string takes
    // 2 bytes.
    println!("Len of hello: {}", hello.len());
    let s = &hello[0..4];
    println!("Slice: {}", s);

    for charact in hello.chars() {
        println!("{}", charact);
    }

    println!("Check if valid {}", hello);

    for byte in hello.bytes() {
        println!("{}", byte);
    }
}
