#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

// Managing Option enum with binding
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1)
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one_no_return(x: Option<i32>) {
    println!("Enter to plus_one_no_return: {:?}", x);
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}

fn main() {
    let coin = Coin::Quarter(UsState::Alabama);
    let cents = value_in_cents(coin);
    println!("The value of the coin in cents is {}", cents);

    // Match with Option<T> enum
    let some_num = Some(50);
    let none_num: Option<i32> = None;
    println!("Some num: {:?}", plus_one(some_num));
    println!("None num: {:?}", plus_one(none_num));

    // Options does not move ownership.
    let x = Some(3);
    plus_one_no_return(x);
    println!("X: {:?}", x);

    // Match with default value using binding.
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other)
    };

    // Match with default value without matching.
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll()
    };
}