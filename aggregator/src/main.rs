use aggregator::{self, Publication, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("This book is amazing! Recommended read..."),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let publication = Publication {
        username: String::from("hdvanegasm"),
        content: String::from("Hola, este es el contenido."),
    };

    aggregator::notify(&tweet);
    aggregator::notify(&publication);
}
