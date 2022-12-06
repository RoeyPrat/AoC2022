use std::{fs::File, io::Read};

fn read_input() -> Result<Vec<String>, std::io::Error> {
    let mut file = File::open("prod.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    Ok(lines)
}
fn after_unique(string: &str, num_unique_chars: usize) -> Result<usize, &'static str> {
    let mut i = num_unique_chars;
    while i < string.len() {
        let mut temp: Vec<char> = string[i - num_unique_chars..i]
            .chars()
            .into_iter()
            .collect();
        temp.sort();
        temp.dedup();
        if temp.len() == num_unique_chars {
            return Ok(i);
        }
        i += 1;
    }
    Err("no suitable unique sequence found")
}
fn main() {
    let lines = read_input().unwrap();
    println!("{:?}", after_unique(&lines[0], 14).unwrap());
}

#[cfg(test)]
mod tests {
    use crate::after_unique;

    #[test]
    fn start_of_packet() {
        assert_eq!(
            after_unique("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4).unwrap(),
            7
        );
        assert_eq!(after_unique("bvwbjplbgvbhsrlpgdmjqwftvncz", 4).unwrap(), 5);
        assert_eq!(after_unique("nppdvjthqldpwncqszvftbrmjlhg", 4).unwrap(), 6);
        assert_eq!(
            after_unique("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4).unwrap(),
            10
        );
        assert_eq!(
            after_unique("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4).unwrap(),
            11
        );
    }
    #[test]
    fn start_of_message() {
        assert_eq!(
            after_unique("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14).unwrap(),
            19
        );
        assert_eq!(
            after_unique("bvwbjplbgvbhsrlpgdmjqwftvncz", 14).unwrap(),
            23
        );
        assert_eq!(
            after_unique("nppdvjthqldpwncqszvftbrmjlhg", 14).unwrap(),
            23
        );
        assert_eq!(
            after_unique("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14).unwrap(),
            29
        );
        assert_eq!(
            after_unique("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14).unwrap(),
            26
        );
    }
}
