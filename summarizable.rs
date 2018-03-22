
trait Summarizable {
    fn summary(&self) -> String;
}

struct NewsArticle
{
    headline : String,
    location : String,
    author   : String,
    content  : String, 
}

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

struct Tweet
{
    username : String,
    content  : String,
    reply    : bool,
    retweet  : bool,
}

impl Summarizable for Tweet {
    fn summary(&self) -> String {
        let res = format!("{}: {}", &self.username, &self.content);
        res
    }
}

fn notify<T : Summarizable>(item : &T) {
    println!("Notification: {}", item.summary());
}

fn main() {
    let tweet = Tweet {
        username : String::from("arunmu"),
        content  : String::from("Rust is cool!"),
        reply    : false,
        retweet  : false,
    };

    println!("Summary is : {}", tweet.summary());
    notify(&tweet);
}
