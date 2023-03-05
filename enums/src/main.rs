// An enum value can have only one value at a time.
// Also an enum value can have arguments and in that
// case, the variants of an enum become a function
// that constructs an instance of the enum with the
// given arguments.
#[derive(Debug)]
enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String)
}

  // This variant of the enum has named fields
#[derive(Debug)]
enum Message {
    Quit,
    Move {
        x: i32,
        y: i32
    },
    Write(String),
    ChangeColor(i32, i32, i32)
}

// We can define methods on enums!
impl Message {
    fn call(&self) {
        println!("Call to enum: {:?}", self);
    }
}

fn main() {
    // The variants of enum are namespaced under its
    // identifier.

    let home = IpAddress::V4(127, 0, 0, 1);
    let loopback = IpAddress::V6(String::from("::1"));

    println!("Home addr = {:?}", home);
    println!("Loopback addr = {:?}", loopback);

    let m = Message::Write(String::from("Hello world!"));
    m.call()
}
