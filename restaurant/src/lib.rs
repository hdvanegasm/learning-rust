// By default, the modules are private. Because eat_at_restaura&nt
// and front_of_house are siblings in the crate root, the
// eat_at_restaurant function can access to the front_of_house
// module.

// This will load the module front_of_house from the file.
mod front_of_house;

mod back_of_the_house {
    // Structs keep the rule of thumb that every field
    // is private by default unless annotated with pub.
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    // Given that this enum is public, all of his
    // variants are public as well.
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

// Just as if hosting module were defined in the crate root
use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // Absolute path
    hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_the_house::Breakfast::summer("Rye");

    // Change the value of the toast
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // This line does not compile because seasonal_fruit is
    // a private field of Breakfast
    // meal.seasonal_fruit = String::from("banana");

    let order1 = back_of_the_house::Appetizer::Salad;
    let order2 = back_of_the_house::Appetizer::Soup;
}

// The `as` keyword avoids the conflict between the two imports here.
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    Result::Ok(())
}

fn function2() -> IoResult<()> {
    IoResult::Ok(())
}

// When we use the `use` keyword we are bringing the name into the scope in a
// private way. We can use the `pub` keyword to allow to other programs to use
// the name as if we have declared it in the new scope publicly. The `pub use`
// keyword is useful when the people that is using your program perceives the
// structure differently of what you are doing to implement the library itself.
// This allows to organize the program for both programmers that are working
// *in* the library and programmers that are *calling* the library.
pub use crate::front_of_house::entrance;

fn entry() {
    entrance::give_entrance();
}

// We can abbreviate the `use` keyword for paths that share common prefixes
use std::{cmp::Ordering, io};

// We can bring all the public items defined in a path as follows. Be careful
// with this because it reduces the redability.
use std::collections::*;