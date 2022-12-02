use std::fs::File;
use std::io::prelude::*;

pub fn main() {
    let mut file = File::open("./src/day1/inventory.txt").expect("Unable to open file");
    let mut inventory = String::new();
    file.read_to_string(&mut inventory)
        .expect("Error reading the file");

    // we good we don't need to print the whole inventory anymore
    // println!("{}", inventory);

    let inventories_as_strings = inventory.split("\n\n");
    let mut each_calories: Vec<i32> = Vec::new();

    let mut maximum: i32 = 0;

    for inventory in inventories_as_strings {
        let mut vec: Vec<i32> = Vec::new();
        let split = inventory.split("\n");
        split.for_each(|f| match f.parse() {
            Ok(val) => {
                vec.push(val);
            }
            Err(error) => {
                println!("Swallowed Error: {:?}", error);
            }
        });

        let total = vec
            .iter()
            .fold(0, |accumulator, next_value| accumulator + next_value);

        println!("{:?}, Total: {}", vec, total);

        if total > maximum {
            maximum = total;
        }

        each_calories.push(total);
    }

    println!("Maximum: {}", maximum);

    each_calories.sort_unstable();
    // see why i used sort_unstable instead of sort here:
    // https://stackoverflow.com/a/67707591/8916706
    each_calories.reverse(); // reverse so it's high to low

    println!(
        "Top three: {:?} {:?} {:?}",
        each_calories.get(0),
        each_calories.get(1),
        each_calories.get(2)
    );
    println!(
        "Top three combined: {:?}",
        each_calories[0] + each_calories[1] + each_calories[2]
    )
}
