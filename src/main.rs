use std::thread;
use std::fs::{File, write};

fn main() {
    
    let data_path: &str = "./data/data.json";
    let text = std::fs::read_to_string(data_path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&text).expect("Json file error");
    println!("data :{}",json)
}
