use std::fs::read_to_string;

fn help_the_elves(path: &str) -> u32 {
    let rucksacks = look_through_rucksacks(path);

    let res = split_compartment_content(rucksacks)
        .into_iter().map(|r| get_duplicate_items_in(r.0, r.1))
        .map(|d| get_priority_of(d) as u32)
        .sum();


    return res
}


fn get_priority_of(item: String) -> u8 {
    let binding = item.into_bytes();
    let byte_representation = binding.get(0).unwrap();

    if byte_representation >= &b'A' && byte_representation <= &b'Z' {
        return byte_representation - 64 + 26;
    } else if byte_representation >= &b'a' && byte_representation <= &b'z' {
        return byte_representation - 96;
    } else {
        panic!("Invalid item");
    }
}

fn get_duplicate_items_in(comp_1: String, comp_2: String) -> String {
    let mut duplicate_item: Option<String> = None;
    for c in comp_1.chars() {
        if comp_2.contains(c) {
            duplicate_item = Some(c.to_string());
            break;
        }
    };
    match duplicate_item {
        Some(d) => d,
        None => panic!("No duplicate item found"),
    }
}

fn look_through_rucksacks(path: &str) -> Vec<String> {
    let mut rucksacks = Vec::new();
    let rucksacks_file = read_to_string(path).unwrap();
    let mut rucksacks_lines = rucksacks_file.lines();
    while let Some(rucksack) = rucksacks_lines.next() {
        rucksacks.push(rucksack.to_string());
    }
    rucksacks
}

fn split_compartment_content(rucksacks: Vec<String>) -> Vec<(String, String)> {
    let mut rucksacks_split = Vec::new();
    for rucksack in rucksacks {
        let (first, last) = rucksack.split_at(rucksack.len()/2);
        rucksacks_split.push((first.to_string(), last.to_string()));
    }

    rucksacks_split
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn test_priority_a() {
        let item = "a".to_string();
        let priority = get_priority_of(item);
        assert_eq!(priority, 1);
    }

    #[test]
    fn test_priority_z() {
        let item = "z".to_string();
        let priority = get_priority_of(item);
        assert_eq!(priority, 26);
    }

    #[test]
    fn test_get_priority_A() {
        let item = "A".to_string();
        let priority = get_priority_of(item);
        assert_eq!(27, priority);
    }

    #[test]
    fn test_get_priority_Z() {
        let item = "Z".to_string();
        let priority = get_priority_of(item);
        assert_eq!(52, priority);
    }
    #[test]
    #[should_panic]
    fn priority_panics_on_invalid_item() {
        let item = "1".to_string();
        let _priority = get_priority_of(item);
    }

    #[test]
    #[should_panic]
    fn test_get_duplicate_items_in_panics() {
        let comp_1 = "abc".to_string();
        let comp_2 = "def".to_string();
        let _duplicate_item = get_duplicate_items_in(comp_1, comp_2);
    }

    #[test]
    fn test_can_find_duplicate_item() {
        let comp_1 = "abc".to_string();
        let comp_2 = "defc".to_string();
        let duplicate_item = get_duplicate_items_in(comp_1, comp_2);
        assert_eq!(duplicate_item, "c");
    }


    #[test]
    fn test_can_split_compartment_content() {
        let rucksacks = vec!["abcdef".to_string(), "ghijkl".to_string()];
        let rucksacks_split = split_compartment_content(rucksacks);
        assert_eq!(rucksacks_split, vec![("abc".to_string(), "def".to_string()), ("ghi".to_string(), "jkl".to_string())]);
    }

    #[test]
    fn can_help_the_poor_elves() {
        let path = "./src/three/rucksack.txt";
        let result = help_the_elves(path);
        assert_eq!(result, 157);
    }
}