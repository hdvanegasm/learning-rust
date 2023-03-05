/* 
Here we used String insead of &str (slices).
With this definition, we want to say that each
field is valid as long as the whole structure 
is valid. So each instance owns all of his data.
*/
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32);

/*
Unit-like struct. This is usefull when you need
to implement a trait over some struct, but that
struct does not have any fields in it.
*/
struct AllwaysEqual;

fn build_user(email: String, username: String) -> User {
    User {
        active: true, 
        username,
        email, 
        sign_in_count: 1
    }
}

fn main() {
    let mut user1 = User {
        email: String::from("hdvanegasm@unal.edu.co"),
        username: String::from("hdvanegasm"),
        active: true,
        sign_in_count: 1
    };

    println!("Username = {}", user1.username);

    // Update field
    user1.username = String::from("hdvanegasm121");

    println!("New username = {}", user1.username);

    // You can use mut here
    let mut user2 = build_user(
        String::from("vvalenciah@gmail.com"), 
        String::from("vvalencia")
    );

    println!("Username 2 = {}", user2.username);

    // Changing user 2
    user2.username = String::from("verovalenciah");
    println!("Show change user 2 = {}", user2.username);

    // Creating an struct from another. The another
    // elements must come last.
    let user_vero2 = User {
        username: String::from("vvalenciaSecondAccount"),
        ..user2
    };

    println!(
        "UserVero2 username = {}, UserVero2 email = {}",
        user_vero2.username,
        user_vero2.email
    );

    // Here I can't use user2.email because the creation of user_vero2
    // implies a movement of username.

    let origin = Point(0, 0);
    let black = Color(0, 0, 0);

    println!("Origin = {}, Color = {}", origin.0, black.0);

    let subject = AllwaysEqual;
}
