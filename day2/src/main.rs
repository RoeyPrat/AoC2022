use std::{fs::File, io::Read};
enum GameResult {
    Win,
    Draw,
    Lose,
}

impl GameResult {
    fn new(string: &str) -> Result<GameResult, &'static str> {
        match string {
            "X" => Ok(GameResult::Lose),
            "Y" => Ok(GameResult::Draw),
            "Z" => Ok(GameResult::Win),
            _ => Err("unrecognized game result"),
        }
    }
}
#[derive(Debug, PartialEq, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}
impl Hand {
    fn new(string: &str) -> Result<Hand, &'static str> {
        match string {
            "A" => Ok(Hand::Rock),
            "B" => Ok(Hand::Paper),
            "C" => Ok(Hand::Scissors),
            _ => Err("unrecognizable hand"),
        }
    }
    fn value(self: Self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
    fn fight(self: Self, opponent_hand: Hand) -> GameResult {
        match (self, opponent_hand) {
            (Hand::Rock, Hand::Scissors) => GameResult::Win,
            (Hand::Rock, Hand::Paper) => GameResult::Lose,
            (Hand::Paper, Hand::Rock) => GameResult::Win,
            (Hand::Paper, Hand::Scissors) => GameResult::Lose,
            (Hand::Scissors, Hand::Paper) => GameResult::Win,
            (Hand::Scissors, Hand::Rock) => GameResult::Lose,
            _ => GameResult::Draw,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Game {
    player_hand: Hand,
    opponent_hand: Hand,
}

fn calculate_hand(game_result: GameResult, opponent_hand: Hand) -> Hand {
    match (game_result, opponent_hand) {
        (GameResult::Win, Hand::Scissors) => Hand::Rock,
        (GameResult::Lose, Hand::Paper) => Hand::Rock,
        (GameResult::Win, Hand::Rock) => Hand::Paper,
        (GameResult::Lose, Hand::Scissors) => Hand::Paper,
        (GameResult::Win, Hand::Paper) => Hand::Scissors,
        (GameResult::Lose, Hand::Rock) => Hand::Scissors,
        _ => opponent_hand,
    }
}

impl Game {
    fn new(string: &str) -> Result<Game, &'static str> {
        let v: Vec<&str> = string.split(" ").take(2).collect();
        let game_result = GameResult::new(v[1])?;
        let opponent_hand = Hand::new(v[0])?;
        let player_hand = calculate_hand(game_result, opponent_hand);
        Ok(Game {
            player_hand,
            opponent_hand,
        })
    }
    fn score(self: Self) -> u32 {
        let game_value = match self.player_hand.fight(self.opponent_hand) {
            GameResult::Win => 6,
            GameResult::Draw => 3,
            GameResult::Lose => 0,
        };
        self.player_hand.value() + game_value
    }
}

fn read_input() -> Result<Vec<String>, std::io::Error> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let lines: Vec<String> = contents.split("\n").map(|s| s.to_string()).collect();
    Ok(lines)
}

fn main() {
    let lines = read_input().unwrap();
    let games: Vec<Game> = lines.into_iter().map(|g| Game::new(&g).unwrap()).collect();
    let scores: Vec<(Game, u32)> = games
        .to_owned()
        .into_iter()
        .map(|g| (g, g.score()))
        .collect();
    let score: u32 = games.into_iter().map(|g| g.score()).sum();

    println!("{:?}  {:?}", scores, score);
}
