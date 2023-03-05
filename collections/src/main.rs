enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // Creates an empty vector
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);
    println!("{:?}", v);

    // Creates a vector with initial elements
    let v2 = vec![1, 2, 3, 4];
    println!("{:?}", v2);

    // Referencing elements in a vector
    let third: &i32 = &v[3];
    println!("Third element of v: {}", third);

    // Referencin element using get
    let third: Option<&i32> = v.get(3);
    match third {
        Some(element) => println!("The third element is {}", element),
        None => println!("Error, there is no third element"),
    }

    let v3 = vec![1, 2, 3, 4, 5];
    // let does_not_exist = &v3[100];
    let does_not_exist = v3.get(100);

    let mut v4 = vec![1, 2, 3, 4, 5];
    let first = &v4[0];
    println!("First element: {}", first);
    v4.push(6);

    // Iterating vectors
    let v = vec![1, 2, 3, 4, 5];
    for i in &v {
        println!("Element: {}", i);
    }

    // Iterating mutable references
    let mut v = vec![10, 100, 40];
    for i in &mut v {
        // Follows the reference and modify the value also called
        // "derreference". We use * to change the value of a mutable reference
        *i += 50;
    }

    for i in &v {
        println!("Element modified: {}", i);
    }

    // Using a vector of enums
    let row = vec![
        SpreadsheetCell::Int(2),
        SpreadsheetCell::Text(String::from("Hi")),
        SpreadsheetCell::Float(12.4),
    ];

    for element in &row {
        match element {
            SpreadsheetCell::Int(content) => println!("Int with content {}", content),
            SpreadsheetCell::Float(content) => println!("Float with content {}", content),
            SpreadsheetCell::Text(content) => println!("Text with content {}", content),
        };
    }

    // When a vector is dropped all of his contents are also dropped. The
    // borrow checker ensures that all the references to contents of the vector
    // are only used while the vector is valid.
}
