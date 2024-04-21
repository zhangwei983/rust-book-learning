use std::fmt;

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

// Implement Summary trait.
impl Summary for NewArticle {
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

// Implement Summary trait.
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Implement Display trait.
impl fmt::Display for Tweet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Display: ({}, {})", self.username, self.content)
    }
}

// Trait as parameter.
pub fn notify(item: &impl Summary) {
    println!("Summary: {}", item.summarize());
}

// Multiple traits.
// pub fn notify_multi_traits<T: Summary + Display>(item: &T) { // Also valid syntax.
pub fn notify_multi_traits(item: &(impl Summary + fmt::Display)) {
    println!("Summary: {}", item.summarize());
    println!("{}", item);
}

// Clause `where`.
pub fn notify_where<T>(item: &T) 
where
    T: Summary + fmt::Display
{
    println!("Breaking news! {}", item.summarize());
}

// Traits as returning types.
fn _returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
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

    // One trait as parameter.
    notify(&tweet);

    let _news = String::from("Breaking new!");
    // notify(&news); // This won't compile, as String doesn't implement the `Summary` trait.

    // Multiple traits as parameter.
    notify_multi_traits(&tweet);
}
