use std::collections::HashMap;
use std::fs;

fn main() {
    //=== Puzzle 1
    let p1_input = fs::read_to_string("inputs/day01.txt").expect("Error reading input");

    let p1_values = p1_input.split("\n").collect::<Vec<_>>();

    let mut p1_numbers = vec![];

    for value in p1_values {
        let mut first = None;
        let mut last = None;

        // First digit
        for c in value.chars() {
            if c.is_digit(10) {
                first = Some(c);
                break;
            }
        }

        // Last digit
        for c in value.chars().rev() {
            if c.is_digit(10) {
                last = Some(c);
                break;
            }
        }

        if first.is_some() && last.is_some() {
            let number = format!("{}{}", first.unwrap(), last.unwrap())
                .parse::<u32>()
                .unwrap();
            p1_numbers.push(number);
        } else {
            panic!("first or last is null")
        }
    }

    let p1_sum: u32 = p1_numbers.iter().sum();
    println!("Puzzle 1 Sum: {}", p1_sum);

    //=== Puzzle 2
    let p2_input = fs::read_to_string("inputs/day01.txt").expect("Error reading input");

    let p2_values = p2_input.split("\n").collect::<Vec<_>>();

    let digit_map = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut p2_numbers = vec![];

    for value in p2_values {
        let mut first = None;
        let mut last = None;

        // First digit
        'idx_loop: for idx in 0..value.len() {
            // Check for numeric
            let c: char = value[idx..idx + 1].chars().next().unwrap();
            if c.is_digit(10) {
                first = Some(c);
                break 'idx_loop;
            }

            // Check for word
            for word in digit_map.keys() {
                if idx + word.len() <= value.len() {
                    let substring = value[idx..idx + word.len()].to_string();
                    if **word == substring {
                        first = (*digit_map.get(word).unwrap() as u32)
                            .to_string()
                            .chars()
                            .next();
                        break 'idx_loop;
                    }
                }
            }
        }

        // Last digit
        'idx_loop: for idx in (0..value.len()).rev() {
            // Check for numeric
            let c: char = value[idx..idx + 1].chars().next().unwrap();
            if c.is_digit(10) {
                last = Some(c);
                break 'idx_loop;
            }

            // Check for word
            for word in digit_map.keys() {
                if idx + word.len() <= value.len() {
                    let substring = value[idx..idx + word.len()].to_string();
                    if **word == substring {
                        last = (*digit_map.get(word).unwrap() as u32)
                            .to_string()
                            .chars()
                            .next();
                        break 'idx_loop;
                    }
                }
            }
        }

        if first.is_some() && last.is_some() {
            let number = format!("{}{}", first.unwrap(), last.unwrap())
                .parse::<u32>()
                .unwrap();
            p2_numbers.push(number);
        } else {
            panic!("first or last is null")
        }
    }

    let p2_sum: u32 = p2_numbers.iter().sum();
    println!("Puzzle 2 Sum: {}", p2_sum);
}
