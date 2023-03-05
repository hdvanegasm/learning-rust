fn main() {
    let mut counter = 0;

    // Useful in threads
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("==================================");

    println!("The result value is {}", result);

    let mut count = 0;
    'counting_up: loop {
        println!("Count: {}", count);
        let mut remaining = 10;
        loop {
            println!("Remaining: {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count: {}", count);

    println!("==================================");

    let mut number = 3;
    while number != 0 {
        println!("Number: {}", number);
        number -= 1;
    }
    println!("Final");

    println!("==================================");

    // Using a while/if for loop the collection is
    // slower and unsafer that this.
    let array = [10, 20, 30, 40, 50];
    for element in array {
        println!("Element: {}", element);
    }

    println!("==================================");
    
    for number in (1..4).rev() {
        println!("Number: {}", number);
    }

    println!("==================================");
    
    // Computation of n-th Fibonacci number
    let n = 12;
    let mut f_prev = 0;
    let mut f_curr = 1;
    for _ in 2..=n {
        let f_curr_tmp = f_curr;
        f_curr = f_curr + f_prev;
        f_prev = f_curr_tmp;
    }
    println!("N-th Fibonacci: {}", f_curr);
}
