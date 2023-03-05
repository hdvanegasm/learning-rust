use std::fmt::Display;

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'b> ImportantExcerpt<'b> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// Including traits, type parameters, generics together
fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T
) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}



// The 'static lifetime lives for the entire duration of the program
fn main() {
    let string2 = String::from("xyz");
    let string1 = String::from("abcd");
    {
        let result;
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    let novel = String::from("Call me Ismael. Some years ago...");
    let first_sentence = novel.split(".")
                                    .next()
                                    .expect("Could not find a '.'");
    
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// The returned reference will be valid as long as both parameters are valid.
// The reference returned by the funcion has a lifetime at long as the smaller
// of the lifetimes of the valued refered in the arguments. Lifetime annotations
// are part of the function signature.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Generating a dangling reference. Dangling references are not allowed in Rust.
// However, we can return ownership to acces to a resource created within the
// function.
// fn longest2<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("A longest one");
//     result.as_str()
// }