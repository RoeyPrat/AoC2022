use std::{collections::HashSet, fs::File, io::Read};
fn read_input() -> Result<Vec<String>, std::io::Error> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    Ok(lines)
}
#[derive(Debug)]
struct RuckSack {
    compartment1: Vec<char>,
    compartment2: Vec<char>,
}
impl RuckSack {
    fn new(string: &str) -> Result<RuckSack, &'static str> {
        let length = string.len();
        if length % 2 == 1 {
            Err("RuckSack must have an even number of items")
        } else {
            let compartment_size = length / 2;
            Ok(RuckSack {
                compartment1: string.chars().take(compartment_size).collect(),
                compartment2: string
                    .chars()
                    .skip(compartment_size)
                    .take(compartment_size)
                    .collect(),
            })
        }
    }
    fn in_both_compartments(self: Self) -> Vec<char> {
        let compartment_set1: HashSet<_> = HashSet::from_iter(self.compartment1.iter().cloned());
        let compartment_set2: HashSet<_> = HashSet::from_iter(self.compartment2.iter().cloned());
        let intersection: HashSet<_> = compartment_set1.intersection(&compartment_set2).collect();
        intersection.into_iter().cloned().collect()
    }
}
trait Item {
    fn priority(&self) -> Result<u32, &'static str>;
}

impl Item for char {
    fn priority(&self) -> Result<u32, &'static str> {
        if self.is_lowercase() {
            Ok(*self as u32 - 'a' as u32 + 1)
        } else if self.is_uppercase() {
            Ok(*self as u32 - 'A' as u32 + 27)
        } else {
            Err("item must be alphabetic ascii")
        }
    }
}
fn main() {
    let lines = read_input().unwrap();
    let rucksacks: Vec<RuckSack> = lines.iter().map(|l| RuckSack::new(l).unwrap()).collect();

    let x: u32 = rucksacks
        .into_iter()
        .map(|r| {
            r.in_both_compartments()
                .iter()
                .map(|i| i.priority().unwrap())
                .sum::<u32>()
        })
        .sum();
    println!("{:?}", x);
}
