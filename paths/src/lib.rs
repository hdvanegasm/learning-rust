mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// This is the idiomatic way to bring a function into
// scope. We should specify the parent module.
use crate::front_of_house::hosting;

fn eat_at_restaurant() {
    hosting::add_to_waitlist()
}

// This is the idiomatic way to bring an enum/structs
// into scope. We should specify the full path.
use std::collections::HashMap;

fn use_hash_map() {
    let mut hash_map = HashMap::new();
    hash_map.insert(1, 2);
}

// There is an exception where we use a partial path to
// import some struct/enum which is when two elements
// share the same name. For example
use std::fmt;
use std::io;

fn function1() -> fmt::Result {

}

fn function2() -> io::Result<()> {
    
}