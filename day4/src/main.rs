use std::{fs::File, io::Read};

fn read_input() -> Result<Vec<String>, std::io::Error> {
    let mut file = File::open("prod.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    Ok(lines)
}
#[derive(Debug)]
struct ElfPair {
    left_start: u32,
    left_end: u32,
    right_start: u32,
    right_end: u32,
}

fn is_overlapping(x: (u32, u32), y: (u32, u32)) -> bool {
    (x.0 <= y.0 && y.0 <= x.1) || (x.0 <= y.1 && y.1 <= x.1)
}

impl ElfPair {
    fn new(string: &String) -> Result<ElfPair, std::fmt::Error> {
        let (left, right) = string.split_once(",").unwrap();
        let (left_start, left_end) = left.split_once("-").unwrap();
        let (right_start, right_end) = right.split_once("-").unwrap();
        Ok(ElfPair {
            left_start: left_start.to_string().parse::<u32>().unwrap(),
            left_end: left_end.to_string().parse::<u32>().unwrap(),
            right_start: right_start.to_string().parse::<u32>().unwrap(),
            right_end: right_end.to_string().parse::<u32>().unwrap(),
        })
    }
    // part 1
    // fn is_contained(self: Self) -> bool {
    //     (self.left_start <= self.right_start && self.right_end <= self.left_end)
    //         || (self.right_start <= self.left_start && self.left_end <= self.right_end)
    // }
    fn is_overlapping(self: Self) -> bool {
        is_overlapping(
            (self.left_start, self.left_end),
            (self.right_start, self.right_end),
        ) || is_overlapping(
            (self.right_start, self.right_end),
            (self.left_start, self.left_end),
        )
    }
}
fn main() {
    let lines = read_input().unwrap();
    let num_containments = lines
        .iter()
        .filter(|s| ElfPair::new(s).unwrap().is_overlapping())
        .count();
    println!("{:?}", num_containments);
}
