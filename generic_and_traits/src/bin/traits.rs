// fn main() {
//     println!("Hello, World!");
// }

// A trait defines functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way. We can use trait bounds to specify that a generic type can be any type that has certain behavior.

// Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.

pub trait Summary {
    fn summarize(&self) -> String;
}
// first we declare trait using `trait` keyword,then the trait's name, we declared it as pub so that other crates depending on this crate can use it too.
// INside curly brackets, we declare method signatures describing behaviors of the types that implement this trait.
// NOTE: WE PROVIDE ; AFTER DECLARATION
// REASON: Each type implementing this trait must provide its own custom behavior for the body of the method.
// Compiler enforces any type having Summary trait to have exact same `summarize` method.

// IMPLEMENTING A TRAIT ON A TYPE
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
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

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Now that the library has implemented the Summary trait on NewsArticle and Tweet, users of the crate can call the trait methods on instances of NewsArticle and Tweet in the same way we call regular methods. The only difference is that the user must bring the trait into scope as well as the types.

// use aggregator::{self, Summary, Tweet};

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize())
}

// TRAIT BOUND SYNTAX
pub fn notify<T: Summary>(item: &T) {
    println!("Breakin news! {}", item.summarize())
}

// SPECIFYING MULTIPLE TRAIT BOUNDS WITH THE + SYNTAX
pub fn notify(item: &(impl Summary + Display))
// The + syntax is also valid with trait bounds on generic types:
pub fn notify<T: Summary+Display>(item: &T)

// USING THE WHERE CLAUSE FOR CLEAR TRAIT BOUNDS
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    unimplemented!()
}
