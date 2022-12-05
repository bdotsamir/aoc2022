// PART 1
// 1. split by newlines to get the rucksacks
// 2. split the lines in half to get the two compartments
// 3. find the common item between compartments
// 4. find the common item's priority
// 5. sum all common item priorities

use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

pub fn main() {
  let mut file = File::open("./src/day3/backpacks.txt").expect("Unable to open file");
  let mut backpacks_file = String::new();
  file
    .read_to_string(&mut backpacks_file)
    .expect("Error reading the file");

  let backpacks: Vec<&str> = backpacks_file.split("\n").collect();

  let mut sum: u16 = 0;

  for backpack in backpacks {
    // cant use backpack.len() here because len() returns the bytes, not the
    // characters. so instead, we use .chars().count();
    // https://stackoverflow.com/a/46290728/8916706
    let length = backpack.chars().count();
    let first_compartment_strs = &backpack[0..length / 2];
    let second_compartment_strs = &backpack[length / 2..length];

    // println!(
    //     "Backpack: {} (len {length})\nFirst compartment: {}, Second compartment: {}",
    //     backpack, first_compartment_strs, second_compartment_strs
    // );

    let mut first_compartment = HashSet::new();
    first_compartment_strs.split("").for_each(|s| {
      if s != "" {
        let c = s.chars().last().unwrap();
        first_compartment.insert(c);
      }
    });

    let mut second_compartment = HashSet::new();
    second_compartment_strs.split("").for_each(|s| {
      if s != "" {
        let c = s.chars().last().unwrap();
        second_compartment.insert(c);
      }
    });

    let intersection = first_compartment.intersection(&second_compartment);

    println!(
      "{:?}\n{:?}\n{:?}\n{:?}",
      backpack, first_compartment, second_compartment, intersection
    );

    let int_as_char_num = parse_char(intersection.last().unwrap().to_owned());

    println!("{int_as_char_num:?}");

    sum += int_as_char_num;
  }

  println!("Sum: {sum}");

}

// A = 65 -> 27
// a = 97 -> 1
fn parse_char(character: char) -> u16 {
  let mut char_as_16 = character as u16; // should return the unicode *number*

  if char_as_16 >= 97 {
    // character is equal to 'a'
    char_as_16 -= 96;
  } else {
    char_as_16 -= 38;
  }

  return char_as_16;
}

// A = 27
// B = 28
// C = 29
// D = 30
