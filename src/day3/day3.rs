// PART 1
// 1. split by newlines to get the rucksacks
// 2. split the lines in half to get the two compartments
// 3. find the common item between compartments
// 4. find the common item's priority
// 5. sum all common item priorities

// PART 2
// 1. split by groups of three
// 2. split each string slice by "" then collect into vecs
// 3. iterate through second vec to .has() first vec for intersection
// 4. use intersection to iterate through third vec for .has()

use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

pub fn main() {
  let mut file = File::open("./src/day3/backpacks.txt").expect("Unable to open file");
  let mut backpacks_file = String::new();
  file
    .read_to_string(&mut backpacks_file)
    .expect("Error reading the file");

  let backpacks: Vec<String> = backpacks_file.split("\n").map(String::from).collect();

  let groups_of_three = backpacks.chunks(3);

  let mut common_items: Vec<char> = Vec::new();

  for group in groups_of_three {
    let group: Vec<Vec<&str>> = group.iter().map(|g| g.split("").collect()).collect();

    let backpack_one_items = group.get(0).unwrap();
    let backpack_two_items = group.get(1).unwrap();
    let backpack_three_items = group.get(2).unwrap();

    let mut backpack_one = HashSet::new();
    backpack_one_items.iter().for_each(|i| {
      if i != &"" {
        backpack_one.insert(i);
      }
    });
    let mut backpack_two = HashSet::new();
    backpack_two_items.iter().for_each(|i| {
      if i != &"" {
        backpack_two.insert(i);
      }
    });
    let mut backpack_three = HashSet::new();
    backpack_three_items.iter().for_each(|i| {
      if i != &"" {
        backpack_three.insert(i);
      }
    });
        
    let intersection = backpack_one.intersection(&backpack_two);

    println!("BP1: {backpack_one:?}\nBP2: {backpack_two:?}\nBP3:{backpack_three:?}");
    println!("intersection of bp1 & 2: {:?}", intersection);

    let mut common_item = "";

    for item in intersection {
      if backpack_three.contains(item) {
        common_item = item;
        println!("Found common item between all three backpacks: {item:?}");
        break;
      }
    }

    if common_item == "" {
      panic!("Did not find a common item!");
    }

    common_items.push(common_item.chars().last().expect("Couldn't find char."));
    
  }

  let sum = common_items.iter().map(|i| {
    parse_char(i.to_owned())
  }).fold(0, |acc, val| acc + val);

  println!("Sum of all priorities: {sum}");

  // println!(
  //   "First group: {:?}\nLast group: {:?}",
  //   groups_of_three.get(0).expect("missing first group"),
  //   groups_of_three.last().expect("missing last group")
  // )
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
