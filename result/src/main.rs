use std::fs::{self, File};
use std::io::{self, Read, ErrorKind};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// The operator ? placed after a result works as follows: if the Result is Ok, the
// value inside Ok(_) will be returned from this expression, and the program
// continues; if the value is Err, the Err will be returned from the whole function
// as if we had used return, so the error value gets propagated.

// The ? operator can be used only in function whose return type matches with the
// return type where the ? operator is used. The return type must be a Return.
fn read_username_short() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_ultra_short() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_final_boss() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn last_char_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(created_file) => created_file,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file {:?}", other_error);
            }
        },
    };

    // We can open the file as follows
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = greeting_file_result.unwrap();

    // Using expect you can specify the panic message
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = greeting_file_result.expect("Error opening the file");
}
