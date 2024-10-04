use csv;
use std::error::Error;

// Result is to handle dynamic errors as there are no exceptions in rust only panics
fn read_csv(path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;

    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(e) = read_csv("./customers.csv") {
        eprintln!("Error encountered: {}", e);
    }
}
