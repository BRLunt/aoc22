use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_max_elve_yumyum_calories(path: &str) -> anyhow::Result<u32> {
    return if let Some(max_elve) = read_calories_list(path)
        .into_iter()
        .max_by_key(|(_, calories)| calories.into_iter().sum::<u32>())
    {
        Ok(max_elve.1.into_iter().sum())
    } else {
        anyhow::bail!("No elves found")
    };
}

/// Read a file into a vector of lines.
/// then parse the lines into a HashMap per elve
fn read_calories_list(path: &str) -> HashMap<u8, Vec<u32>> {
    let mut elve_list: HashMap<u8, Vec<u32>> = HashMap::new();

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut elve_idx = 0;
    let mut calories_vec: Vec<u32> = Vec::new();
    for line in reader.lines() {
        if let Ok(calories_string) = line {
            if calories_string.trim().is_empty() {
                elve_idx += 1;
                calories_vec.clear();
                continue;
            }

            let calories = calories_string.trim().parse::<u32>().unwrap();
            calories_vec.push(calories);
        }

        elve_list.insert(elve_idx as u8, calories_vec.clone());
    };

    elve_list
}

mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let path = "./src/one/calories.txt";
        let result = get_max_elve_yumyum_calories(path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 24000);
    }
}