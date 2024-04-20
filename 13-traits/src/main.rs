pub trait Summary {
    fn summarize(&self) -> String;

    fn default_summarize(&self) -> String { // With default implementation.
        String::from("(Read more...)")
    }
}

pub struct NewArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewArticle { // Implement Summary trait for NewArticle.
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet { // Implement Summary trait for Tweet.
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Trait as parameter.
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("{}", tweet.summarize());
    println!("{}", tweet.default_summarize()); // Use default implementation.

    // Traits as parameter.
    notify(&tweet);

    let _news = String::from("Breaking new!");
    // notify(&news); // This won't compile, as String doesn't implement the `Summary` trait.
}
