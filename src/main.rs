use serde::{Deserialize, Serialize};
use serde_json::{self};
use std::fs::File;
use std::io::BufReader;
use std::error::Error;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    gender: String,
}

fn main() {
    let data_path: &str = "./data/data.json";

    println!("{:?}",read_person_from_file(data_path));

}

fn read_person_from_file<P: AsRef<Path>>(path: P) -> Result<Person, Box<dyn Error>> {

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let u = serde_json::from_reader(reader)?;

    Ok(u)
}
