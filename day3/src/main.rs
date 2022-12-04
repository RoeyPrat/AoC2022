use std::{collections::HashSet, fs::File, io::Read};
fn read_input() -> Result<Vec<String>, std::io::Error> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    Ok(lines)
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

fn triplets(strings: Vec<String>) -> Vec<(String, String, String)> {
    let num_triplets = strings.len() / 3;
    (0..num_triplets)
        .into_iter()
        .map(|i| {
            (
                strings[i * 3].to_owned(),
                strings[i * 3 + 1].to_owned(),
                strings[i * 3 + 2].to_owned(),
            )
        })
        .collect()
}
fn badge(group: &(String, String, String)) -> char {
    let (a, b, c) = group;
    let set: HashSet<char> = a.chars().collect();
    let set2: HashSet<char> = b.chars().filter(|ch| set.contains(ch)).collect();
    let set3: HashSet<char> = c.chars().filter(|ch| set2.contains(ch)).collect();
    set3.iter().next().cloned().unwrap()
}
fn main() {
    let lines = read_input().unwrap();
    let elf_groups = triplets(lines);
    let badges: Vec<char> = elf_groups.iter().map(|group| badge(group)).collect();
    let badge_priority_sum: u32 = badges.iter().map(|b| b.priority().unwrap()).sum();
    println!("{:?}", badge_priority_sum);
}
