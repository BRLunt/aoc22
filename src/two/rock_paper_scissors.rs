use std::fs::File;
use std::io::Read;

enum HANDS {
    ROCK,
    PAPER,
    SCISSORS,
}

impl From<String> for HANDS {
    fn from(s: String) -> Self {
        match s.as_str() {
            "A" | "X" => HANDS::ROCK,
            "B" | "Y" => HANDS::PAPER,
            "C" | "Z" => HANDS::SCISSORS,
            _ => panic!("Invalid hand"),
        }
    }
}

fn calc_points(opponent_choice: HANDS, my_choice: HANDS) -> u8 {
    return outcome_points(&my_choice, &opponent_choice)
        + hand_points(&my_choice);
}


fn outcome_points(my_choice: &HANDS, opponents_choice: &HANDS) -> u8 {
    return match (my_choice, opponents_choice) {
        (HANDS::PAPER, HANDS::ROCK) => 6,
        (HANDS::PAPER, HANDS::SCISSORS) => 0,
        (HANDS::PAPER, HANDS::PAPER) => 3,
        (HANDS::ROCK, HANDS::PAPER) => 0,
        (HANDS::ROCK, HANDS::SCISSORS) => 6,
        (HANDS::ROCK, HANDS::ROCK) => 3,
        (HANDS::SCISSORS, HANDS::PAPER) => 6,
        (HANDS::SCISSORS, HANDS::ROCK) => 0,
        (HANDS::SCISSORS, HANDS::SCISSORS) => 3,
    };
}

fn hand_points(my_choice: &HANDS) -> u8 {
    match my_choice {
        HANDS::ROCK => 1,
        HANDS::PAPER => 2,
        HANDS::SCISSORS => 3,
    }
}

fn read_strat_file(path: &str) -> Vec<(HANDS, HANDS)> {
    let mut strat_file = File::open(path).unwrap();
    let mut strat = String::new();
    strat_file.read_to_string(&mut strat).unwrap();

    let mut strat_vec = Vec::new();

    for line in strat.lines() {
        let mut strat_line = line.split_whitespace();
        let opponent_choice = HANDS::from(strat_line.next().unwrap().to_string());
        let my_choice = HANDS::from(strat_line.next().unwrap().to_string());
        strat_vec.push((opponent_choice, my_choice));
    }

    strat_vec
}

fn calculate_points(path: &str) -> u32 {
    let strat = read_strat_file(&path);
    let mut points = 0;
    for (opponent_choice, my_choice) in strat {
        points += calc_points(opponent_choice, my_choice) as u32;
    }
    points
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_points() {
        let path = "./src/two/strat_one.txt";
        let expected_points = 15;

        let points = calculate_points(path);
        assert_eq!(points, expected_points);
    }
}