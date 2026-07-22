trait Summary {
    fn name(&self) -> &str;
    fn summarize(&self) -> String {
        let to_print = format!("Read more from {}...", self.name());
        return to_print;
    }
}

// any type that want to be summary has to provide a summarize function implementation

struct NewsArticle {
    author: String,
    headline: String,
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for NewsArticle {
    fn name(&self) -> &str {
        &self.author
    }

    fn summarize(&self) -> String {
        let to_print = format!("{} , {}", self.headline, self.author);
        return to_print;
    }
}

impl Summary for Tweet {
    fn name(&self) -> &str {
        &self.username
    }

    fn summarize(&self) -> String {
        let to_print = format!("{} , {}", self.username, self.content);
        return to_print;
    }
}

fn main() {
    let article = NewsArticle {
        author: String::from("John Doe"),
        headline: String::from("Breaking News!"),
    };

    let tweet = Tweet {
        username: String::from("johndoe"),
        content: String::from("Hello, world!"),
    };

    aggregator(&article);
    aggregator(&tweet);

    println!("Article Summary: {}", article.summarize());
}

fn aggregator<T: Summary>(source: &T) {
    println!("Summary: {} and name {}", source.summarize(), source.name());
}
