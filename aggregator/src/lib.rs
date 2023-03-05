use std::fmt::{Debug, Display};

pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// This implementation block can be called only if Pair has a inner type that
// implements PartialOrd and Display.
impl<T: PartialOrd + Display> Pair<T>{
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x = {}", self.x)
        } else {
            println!("The largest number is y = {}", self.y)
        }
    }
}

// We also can do the following. Here we are implementing the ToString trait on
// any type that implements the Display trait.
// impl<T: Display> ToString for T { --code-- }

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

pub struct Publication {
    pub username: String,
    pub content: String,
}

pub trait Summary {
    // Each type iplementing this trait must provide its own custom behavior of the types
    // implementing this trait.
    fn summarize(&self) -> String {
        format!("(Read more...) {}", self.summarize_author())
    }

    fn summarize_author(&self) -> String;
}

impl Summary for Publication {
    fn summarize_author(&self) -> String {
        format!("Published by {}", self.username)
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("Author: {}", self.author)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("from @{}", self.username)
    }
}

pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn some_function<T, U>(t: T, u: U)
where
    T: Display + Clone,
    U: Clone + Debug,
{
}

fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        Tweet {
            username: String::from("vvalenciah"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        }
    }
}

#[cfg(test)]
mod tests {}
