use std::fs;

fn aggregate(acc: i32, rps_game: &str) -> i32 {
  match rps_game {
    "A X" => acc + 4,
    "A Y" => acc + 8,
    "A Z" => acc + 3,
    "B X" => acc + 1,
    "B Y" => acc + 5,
    "B Z" => acc + 9,
    "C X" => acc + 7,
    "C Y" => acc + 2,
    "C Z" => acc + 6,
    _     => acc
  }
}

fn parse_input(rps_game: String) -> i32 {
  rps_game
    .lines()
    .fold(0, aggregate)
}

fn main() {
  let result = fs:: read_to_string("input.txt")
    .map_err(|_| "Ran into an error tryign to read from input file: input.txt".to_string())
    .map(parse_input);

  match result {
    Ok(total) => println!("Total Score: {total}"),
    Err(msg)  => println!("{msg}")
  }
}
