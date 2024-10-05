use mongodb::{
    options::{ClientOptions, ResolverConfig},
    Client,
};
use std::env;
use std::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client_uri =
        env::var("MONGODB_URI").expect("Envirnoment variable not set. Set MONGODB_URI env variable");
    let options = ClientOptions::parse(&client_uri).await?;
    let client = Client::with_options(options)?;

    println!("Databases: ");
    for name in client.list_database_names().await? {
        println!("- {}", name);
    }

    Ok(())
}
