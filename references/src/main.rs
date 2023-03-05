fn calculate_lenght(s: &String) -> usize {
    s.len()
}

// References are inmutable by default.
fn change_str(s: &mut String) {
    s.push_str(" + Modification");
}

fn no_dangle() -> String {
    let s = String::from("No dangling string");
    s
}

fn main() {
    let some_str = String::from("Hello world!");

    // The data in the reference is owed by another variable.
    // The ownership of some_str is not moved to the function.
    let len = calculate_lenght(&some_str);
    println!("The lenght of '{}' is {}", some_str, len);

    // This allows to modify the string in the method without
    // give the ownership. You can't have multiple mutable
    // references of a value. You can't have multiple references
    // to where one is mutable and another is inmutable from the
    // same variable. You can have all the inmutable references
    // from a variable as you want. 
    let mut mod_string = String::from("Modifiable str");
    change_str(&mut mod_string);
    println!("The value of mod_string is '{}'", mod_string);

    let ref1 = &mod_string;
    let ref2 = &mod_string;

    println!("References: {} {}", ref1, ref2);

    let ref3_mut = &mod_string;
    println!("Mut reference: {}", ref3_mut);
    println!("Trying scope of ref1: {}", ref1);

    // Dangling references. You cannot point to a reference
    // that is already invalidated. Here, the ownership of
    // the String is given to the outer function and nothing
    // is dealocated.
    let no_dangle_str = no_dangle();
    println!("Value no dangle: {}", no_dangle_str);
}
