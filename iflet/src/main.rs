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


fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => ()
    };

    // This can be translated into the following code
    // The sintax is
    //      if let <pattern> = <expression>
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // if let - else example
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alabama);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }
    println!("The count is: {}", count);
}
