use std::fmt::{Debug, Display};

pub trait Summarizable {
    fn author_summary(&self) -> String;

    fn summary(&self) -> String {
        format!("(Read more from {}...)", self.author_summary())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarizable for NewsArticle {
    fn author_summary(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarizable for Tweet {
    fn author_summary(&self) -> String {
        format!("@{}", self.username)
    }

    fn summary(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify<T: Summarizable>(item: T) {
    println!("Breaking news! {}", item.summary());
}

pub fn some_function<T, U>(t: T, u: U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
{
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{NewsArticle, notify, Summarizable, Tweet};

    #[test]
    fn it_works() {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };

        println!("1 new tweet: {}", tweet.summary());

        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again the best hockey team in the NHL",
            ),
        };

        println!("New article available! {}", article.summary());

        notify(tweet);
    }
}
