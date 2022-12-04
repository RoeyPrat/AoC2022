use std::{fs::File, io::Read};

fn read_input() -> Result<Vec<String>, std::io::Error> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    Ok(lines)
}

fn main() -> std::io::Result<()> {
    let lines = read_input()?;

    let elves: Vec<Vec<u32>> = lines
        .iter()
        .fold(Vec::new(), |mut accumulator, x| {
            if x == "" || accumulator.is_empty() {
                accumulator.push(Vec::new());
            } else {
                accumulator.last_mut().unwrap().push(x.parse().unwrap());
            }
            accumulator
        })
        .into_iter()
        .filter(|nums| !nums.is_empty())
        .collect();
    let mut calories_per_elf: Vec<u32> = elves
        .into_iter()
        .map(|calorie_seq| calorie_seq.iter().sum())
        .collect();
    calories_per_elf.sort_unstable();
    let sum_top_elves: u32 = calories_per_elf.iter().rev().copied().take(3).sum();
    print!("{:?}", sum_top_elves);
    Ok(())
}
