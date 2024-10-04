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

fn read_json_data(raw_json: &str) -> Article {
    let parsed: Article = serde_json::from_str(raw_json).unwrap();
    return parsed;
}

fn main() {
    let json = r#"
    {
        "article" : "How to work with json in rust",
        "author" : "Kunal",
        "paragraph" : [
            {
                "name" : "This is a test String" 
            },
            {
                "name" : "This is another test string"
            },
            {
                "name" : "Yet another String"
            }
        ]
    }"#;

    let parsed: Article = read_json_data(json);

    println!(
        "\n\n The name of the first paragraph is: {}",
        parsed.paragraph[0].name
    );
}
