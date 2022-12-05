use std::{fs::File, io::Read};
fn read_input() -> Result<(Vec<String>, Vec<String>), std::io::Error> {
    let mut file = File::open("prod.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let (stack_contents, moves_contents) = contents.split_once("\n\n").unwrap();
    let mut stack_lines: Vec<String> = stack_contents.split("\n").map(|s| s.to_string()).collect();
    let moves_lines: Vec<String> = moves_contents.split("\n").map(|s| s.to_string()).collect();
    stack_lines.pop();
    stack_lines.reverse();
    Ok((stack_lines, moves_lines))
}

fn main() {
    let (stack_lines, moves_lines) = read_input().unwrap();
    let mut stacks = stack_lines
        .iter()
        .fold(Vec::<Vec<char>>::new(), |mut _stacks, line| {
            let mut i = 0;
            while i < line.len() {
                if _stacks.len() < (i / 4) + 1 {
                    _stacks.push(Vec::<char>::new());
                }
                if line.chars().nth(i).unwrap() == '[' {
                    _stacks[i / 4].push(line.chars().nth(i + 1).unwrap());
                }
                i += 4;
            }
            _stacks
        });
    for line in moves_lines {
        let (a, b) = line.split_once(" from ").unwrap();
        let num_moves: usize = a.strip_prefix("move ").unwrap().parse().unwrap();
        let from_stack: usize = b.split_once(" to ").unwrap().0.parse().unwrap();
        let to_stack: usize = b.split_once(" to ").unwrap().1.parse().unwrap();
        // first part:
        // for _ in 0..num_moves {
        //     let x = stacks[from_stack - 1].pop().unwrap();
        //     stacks[to_stack - 1].push(x)
        // }
        let x: Vec<char> = (0..num_moves)
            .into_iter()
            .map(|_| stacks[from_stack - 1].pop().unwrap())
            .collect();
        x.iter().rev().map(|i| stacks[to_stack-1].push(*i)).collect()
        // let idx = stacks[from_stack-1].len()- num_moves;
        // let x = stacks[from_stack-1].split_at(idx).1;
        // stacks[to_stack-1].extend_from_slice(x);
    }
    for mut stack in stacks {
        print!("{}", stack.pop().unwrap())
    }
}
