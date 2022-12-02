use std::fs::File;
use std::io::prelude::*;

pub fn main() {
  let mut file = File::open("./src/day1/inventory.txt")
    .expect("Unable to open file");
  let mut inventory = String::new();
  file.read_to_string(&mut inventory)
    .expect("Error reading the file");
  
  // we good we don't need to print the whole inventory anymore
  // println!("{}", inventory);

  let inventories_as_strings = inventory.split("\n\n");

  let mut maximum: i32 = 0;

  for inventory in inventories_as_strings {
    let mut vec: Vec<i32> = Vec::new();
    let split = inventory.split("\n");
    split.for_each(|f| { match f.parse() {
      Ok(val) => { vec.push(val); }
      Err(error) => { println!("Swallowed Error: {:?}", error); }  
    } });

    let total = vec.iter().fold(0, |accumulator, next_value| accumulator + next_value);

    println!("{:?}, Total: {}", vec, total);

    if total > maximum {
      maximum = total;
    }
  }

  println!("Maximum: {}", maximum);

  // for inventory in inventories {
  //   println!("Inventory: {}", inventory);
  // }

}