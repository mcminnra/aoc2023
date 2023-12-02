use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day02.txt").expect("Error reading input");
    let games = input.split("\n").collect::<Vec<_>>();

    // Puzzle 1
    let mut id_sum = 0;
    'game_loop: for game_record in games.to_owned() {
        let id_game_pair = game_record.split(":").collect::<Vec<_>>();
        let id = id_game_pair
            .get(0)
            .unwrap()
            .replace("Game ", "")
            .parse::<u32>()
            .unwrap();
        let game = id_game_pair
            .get(1)
            .unwrap();


        let (red_max, green_max, blue_max) = (12, 13, 14);
        for handful in game.trim().split(";") {
            for color in handful.split(",") {
                let color = color.trim();
                if color.contains("red") {
                    let amount = color.split(" ").next().unwrap().parse::<u32>().unwrap();
                    if amount > red_max {
                        continue 'game_loop;
                    }
                } else if color.contains("green") {
                    let amount = color.split(" ").next().unwrap().parse::<u32>().unwrap();
                    if amount > green_max {
                        continue 'game_loop;
                    }
                } else if color.contains("blue") {
                    let amount = color.split(" ").next().unwrap().parse::<u32>().unwrap();
                    if amount > blue_max {
                        continue 'game_loop;
                    }
                } else {
                    panic!("Couldn't find any color in handful grab.");
                }
            }
        }
        id_sum += id;
    }
    println!("Part 1 - ID sums: {}", id_sum);

    // Puzzle 2
    let mut power_set_sum = 0;
    for game_record in games.to_owned() {
        let id_game_pair = game_record.split(":").collect::<Vec<_>>();
        let game = id_game_pair
            .get(1)
            .unwrap();

        let (mut red_max, mut green_max, mut blue_max) = (0, 0, 0);
        for handful in game.trim().split(";") {
            for color in handful.split(",") {
                let color = color.trim();
                if color.contains("red") {
                    let amount = color.split(" ").next().unwrap().parse::<u32>().unwrap();
                    if amount > red_max {
                        red_max = amount;
                    }
                } else if color.contains("green") {
                    let amount = color.split(" ").next().unwrap().parse::<u32>().unwrap();
                    if amount > green_max {
                        green_max = amount;
                    }
                } else if color.contains("blue") {
                    let amount = color.split(" ").next().unwrap().parse::<u32>().unwrap();
                    if amount > blue_max {
                        blue_max = amount;
                    }
                } else {
                    panic!("Couldn't find any color in handful grab.");
                }
            }
        }
        power_set_sum += red_max*green_max*blue_max;
    }
    println!("Part 2 - power_set_sum: {}", power_set_sum);
}