use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let article: Article = Article {
        article: String::from("Working with json in rust"),
        author: String::from("Kunal"),
        paragraph: vec![
            Paragraph {
                name: String::from("This is a string"),
            },
            Paragraph {
                name: String::from("This is another test string"),
            },
            Paragraph {
                name: String::from("Yet another string"),
            },
        ],
    };

    let json = serde_json::to_string(&article).unwrap();
    println!("The json is :- {}", json);
}
