use postgres::{Client, Error, NoTls};
use std::collections::HashMap;

struct Author {
    _id: i32,
    name: String,
    country: String,
}

fn main() -> Result<(), Error> {
    let mut client = Client::connect("postgres://postgres:postgres@localhost:5432/library", NoTls)?;
    let mut authors = HashMap::new();
    authors.insert(String::from("Chinua Achebe"), "Nigeria");
    authors.insert(String::from("Rabindarnath tagore"), "India");
    authors.insert(String::from("Anita Nair"), "India");

    for (key, value) in &authors {
        let author = Author {
            _id: 0,
            name: key.to_string(),
            country: value.to_string(),
        };

        client.execute(
            "INSERT INTO author (name, country) VALUES ($1, $2)",
            &[&author.name, &author.country],
        )?;
    }
    for row in client.query("SELECT id, name, country FROM author", &[])? {
        let author = Author {
            _id: row.get(0),
            name: row.get(1),
            country: row.get(2),
        };
        println!("Author {} coming from {}", author.name, author.country);
    }
    Ok(())
}
