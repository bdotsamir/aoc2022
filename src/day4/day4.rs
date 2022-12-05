use std::fs::File;
use std::io::prelude::*;

struct Range(u16, u16);
struct ElfPair(Range, Range);

pub fn main() {
  let mut file = File::open("./src/day4/groups.txt").expect("Unable to open file");
  let mut ranges_file = String::new();
  file
    .read_to_string(&mut ranges_file)
    .expect("Error reading the file");

  let range_lines: Vec<&str> = ranges_file.lines().collect();
  let mut pairs: Vec<ElfPair> = Vec::new();

  let mut fully_contains: u16 = 0;

  // Format everything
  for range_line in range_lines {
    let split = range_line.split(",").collect::<Vec<&str>>();
    println!("split: {split:?}");

    let first_elf = split.get(0).expect("Missing range 1");
    let second_elf = split.get(1).expect("Missing range 2");

    let first_elf_split = first_elf.split("-").collect::<Vec<&str>>();
    let second_elf_split = second_elf.split("-").collect::<Vec<&str>>();

    let elf_1_range_start = first_elf_split.get(0).unwrap().parse::<u16>().unwrap();
    let elf_1_range_end = first_elf_split.get(1).unwrap().parse::<u16>().unwrap();

    let elf_2_range_start = second_elf_split.get(0).unwrap().parse::<u16>().unwrap();
    let elf_2_range_end = second_elf_split.get(1).unwrap().parse::<u16>().unwrap();

    let elf_1_range = Range(elf_1_range_start, elf_1_range_end);
    let elf_2_range = Range(elf_2_range_start, elf_2_range_end);

    let pair = ElfPair(elf_1_range, elf_2_range);

    pairs.push(pair);
  }

  for pair in pairs {
    if overlaps(&pair) {
      fully_contains += 1;
    }
  }

  println!("{fully_contains}")

}

// I remember reading this Stack Overflow question years ago in high school CS class.
// lucky i remembered it.
// https://stackoverflow.com/questions/325933/determine-whether-two-date-ranges-overlap
fn overlaps(elf_pair: &ElfPair) -> bool {
  let range_1 = &elf_pair.0;
  let range_2 = &elf_pair.1;

  let start_a = range_1.0;
  let end_a = range_1.1;
  let start_b = range_2.0;
  let end_b = range_2.1;

  return (start_a <= end_b) && (end_a >= start_b);
}

fn _contains(elf_pair: &ElfPair) -> bool {
  let range_1 = &elf_pair.0;
  let range_2 = &elf_pair.1;

  let start_a = range_1.0;
  let end_a = range_1.1;
  let start_b = range_2.0;
  let end_b = range_2.1;

  return ((start_a >= start_b) && (end_a <= end_b)) || ((start_b >= start_a) && (end_b <= end_a));
}
