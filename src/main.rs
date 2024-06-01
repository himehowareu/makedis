use makedis::*;


fn main() {
    println!("Hello, world!");
}



use regex::Regex;

let re = Regex::new(r"[0-9]{4}-[0-9]{2}-[0-9]{2}").unwrap();
let hay = "What do 1865-04-14, 1881-07-02, 1901-09-06 and 1963-11-22 have in common?";
// 'm' is a 'Match', and 'as_str()' returns the matching part of the haystack.
let dates: Vec<&str> = re.find_iter(hay).map(|m| m.as_str()).collect();


let parts = "some string 123 content".split("123");

for part in parts {
    println!("{}", part)
}

let collection = parts.collect::<Vec<&str>>();
dbg!(collection);

let collection: Vec<&str> = parts.collect();
dbg!(collection);

s.lines()

fn main() {
    let name = "Jake".to_string();
    let mut name_in_binary = "".to_string();

    // Call into_bytes() which returns a Vec<u8>, and iterate accordingly
    // I only called clone() because this for loop takes ownership
    for character in name.clone().into_bytes() {
        name_in_binary += &format!("0{:b} ", character);
    }
    println!("\"{}\" in binary is {}", name, name_in_binary);
}


let bits: String = txt.into_bytes()
        .iter()
        .map(|&c| format!("{c:08b}"))
        .collect();




// https://lib.rs/crates/to-binary


use to_binary::{BinaryString,BinaryError};

fn main(){
    // Hexadecimal
    let hex = BinaryString::from_hex("2879E653864EA6047FEBBBD9AE6DA8DA").unwrap();
    
    // String
    let x = BinaryString::from(String::from("Test"));
    assert_eq!(bin_string_2,"01010100011001010111001101110100")
  
  	// str
  	let y = BinaryString::from("Hello World");
  
  	// Byte
  	let byte = BinaryString::from(118u8);
  	
  	// Vector
  	let vector = vec![36,57,123,38,2];
  	let bin_vector = BinaryString::from(vector);
}
