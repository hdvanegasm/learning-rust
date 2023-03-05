fn another_function(x: i32) {
    println!("The value of x is {x}");
}

fn labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    println!("Hello, world!");
    another_function(43);
    labeled_measurement(12, 'c');

    let y = {
        let x = 4;
        x + 20
    };

    println!("The value of y is: {}", y);

    let num = 4;
    let sum = plus_one(num);
    println!("Sum one to {} results in {}", num, sum);
}
