/*
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
*/

// implements the same as above but Trait bound syntax is prefered bc details below
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

/*
this will always allow different Summary trait types
pub fn notify(item1: &impl Summary, item2: &impl Summary) {

this allows us to specify explicitly
pub fn notify<T: Summary>(item1: &T, item2: &T) {
pub fn notify<T: Summary>(item1: &T, item2: &U) {

multiple traits
pub fn notify(item: &(impl Summary + Display)) {
pub fn notify<T: Summary + Display>(item: &T) {
*/

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// impl Summary for NewsArticle {} -> Default behavior
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
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
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn main() {
    {
        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                hockey team in the NHL.",
            ),
        };

        notify(&article);
        //println!("New article available! {}", article.summarize());
    }
    {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        };
        notify(&tweet);
        //println!("1 new tweet: {}", tweet.summarize());
    }
}
