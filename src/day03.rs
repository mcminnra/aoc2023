use std::cmp;
use std::fs;

#[derive(Debug, Clone, Copy)]
struct Number {
    line_idx: usize,
    start_idx: usize,
    end_idx: usize
}

fn get_char_at(i: usize, j: usize, lines: &Vec<&str>) -> char {
    lines.get(i).expect("Unable to find i")[j..j + 1]
        .chars()
        .next()
        .expect("unable to get char at j")
}

fn main() {
    let input = fs::read_to_string("inputs/day03.txt").expect("Error reading input");
    let lines = input.split("\n").collect::<Vec<_>>();

    let line_length = lines.get(0).expect("Unable to get line").len();

    // Find digit indexes
    let mut digit_indices: Vec<Vec<usize>> = vec![];
    for i in 0..lines.len() {
        digit_indices.push(
            lines
                .get(i)
                .expect("Unable to get line")
                .match_indices(char::is_numeric)
                .map(|(i, _)| i)
                .collect(),
        );
    }

    // Convert to numbers
    let mut numbers: Vec<Number> = vec![];
    for i in 0..digit_indices.len() {
        let digit_indices_line: Vec<usize> =
            digit_indices.get(i).expect("Unable to get line").to_vec();
        let mut start: usize = *digit_indices_line.get(0).expect("Unable to init start");
        let mut end: usize = start;

        for j in 1..digit_indices_line.len() {
            let digit_idx = *digit_indices_line.get(j).expect("Unable to get digit_idx");
            if digit_idx == end + 1 {
                end = digit_idx;
            }
            if digit_idx > end + 1 || digit_idx == line_length - 1 || j == digit_indices_line.len()-1 {
                numbers.push(Number {
                    line_idx: i,
                    start_idx: start,
                    end_idx: end
                });
                (start, end) = (digit_idx, digit_idx);
            }
        }
    }

    // Check for symbols around numbers
    let mut part_num_total = 0;
    'number_loop: for number in &numbers {
        for p in number.start_idx..number.end_idx + 1 {
            let lower: usize = cmp::max((number.line_idx as i32)-1, 0) as usize;
            let upper: usize = cmp::min((number.line_idx as i32)+2, line_length as i32) as usize;
            for x in lower..upper {
                let lower: usize = cmp::max((p as i32)-1, 0) as usize;
                let upper: usize = cmp::min((p as i32)+2, line_length as i32) as usize;
                for y in lower..upper {
                    let check_char = get_char_at(x, y, &lines);
                    if !check_char.is_digit(10) && !(check_char == '.') {
                        let number = lines.get(number.line_idx).expect("Unable to get lines")
                            [number.start_idx..number.end_idx + 1]
                            .parse::<u32>()
                            .expect("Unable to parse u32");
                        part_num_total += number;
                        continue 'number_loop;
                    }
                }
            }
        }
    }
    println!("part_num_total: {part_num_total}");

    // Part Two
    // Find gears
    let mut gears_x_y: Vec<(usize, usize)> = vec![];
    for i in 0..lines.len() {
        gears_x_y.extend(lines
            .get(i)
            .expect("Unable to get line")
            .match_indices("*")
            .map(|(gear_idx, _)| (i, gear_idx))
            .collect::<Vec<(usize, usize)>>());

    }

    let mut gear_ratio_total = 0;
    for (gear_x, gear_y) in gears_x_y {
        let near_numbers = numbers
            .clone()
            .into_iter()
            .filter(|number| (gear_x as i32-number.line_idx as i32).abs() <=1)
            .filter(|number| (gear_y as i32-number.start_idx as i32).abs() <=1 || (gear_y as i32-number.end_idx as i32).abs() <=1)
            .collect::<Vec<Number>>();

        if near_numbers.len() == 2 {
            let first_number = {
                let number = near_numbers.first().expect("Unable to get first_number");
                lines
                    .get(number.line_idx)
                    .expect("Unable to get lines")
                    [number.start_idx..number.end_idx + 1]
                    .parse::<u32>()
                    .expect("Unable to parse u32")
            };
            let last_number = {
                let number = near_numbers.last().expect("Unable to get last_number");
                lines
                    .get(number.line_idx)
                    .expect("Unable to get lines")
                    [number.start_idx..number.end_idx + 1]
                    .parse::<u32>()
                    .expect("Unable to parse u32")
            };
            gear_ratio_total += first_number*last_number;
        }
    }
    println!("gear_ratio_total: {}", gear_ratio_total);
}
