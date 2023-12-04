use core::num;
use std::fs;

#[derive(Debug, Clone)]
struct Card {
    id: u32,
    win_nums: Vec<u32>,
    check_nums: Vec<u32>
}

fn main() {
    let input = fs::read_to_string("inputs/day04.txt").expect("Error reading input");
    let card_strs = input.split("\n").collect::<Vec<_>>();

    // Process input
    let mut cards: Vec<Card> = vec![];
    for card_str in card_strs {
        // Process input str
        let (card_prefix_str, num_str) = card_str.split_once(":").expect("Unable to split ':'");
        let (_, id_str) = card_prefix_str.split_once(" ").expect("Unable to get card prefix");
        let (win_nums_str, check_nums_str) = num_str.split_once("|").expect("Unable to get nums");

        // Convert to ints
        let id: u32 = id_str.trim().parse::<u32>().expect("Unable to parse id");
        let win_nums: Vec<u32> = win_nums_str
            .split(" ")
            .filter(|item| !item.is_empty())
            .map(|item| item.trim().parse::<u32>().expect("Unable to parse win_num"))
            .collect();
        let check_nums: Vec<u32> = check_nums_str
            .split(" ")
            .filter(|item| !item.is_empty())
            .map(|item| item.trim().parse::<u32>().expect("Unable to parse check_num"))
            .collect();

        let card: Card = Card{
            id,
            win_nums: win_nums.clone(),
            check_nums: check_nums.clone()
        };

        cards.push(
            card
        );
    }

    let mut points: u32 = 0;
    for card in &cards {
        let mut matches: u32 = 0;
        for win_num in card.win_nums.iter() {
            for check_num in card.check_nums.iter() {
                if win_num == check_num {
                    matches+=1;
                }
            }
        }
        if matches != 0 {
            points+=2_u32.pow(matches-1);
        }
    }
    println!("points: {}", points);

    // Part 2
    let num_of_original_cards: u32 = cards.len() as u32;

    //Create copies
    for card_id in 1..cards.len()+1 {
        println!("Card ID: {}, Cards Len: {}", card_id, cards.len());

        // Find original and copies in deck
        let current_cards: Vec<Card> = cards
            .clone()
            .into_iter()
            .filter(|c| c.id == card_id as u32)
            .collect();
        let num_of_current_cards: u32 = current_cards.len() as u32;
        let current_card: Card = current_cards.first().expect("Unable to get current card").clone();

        // Check for matches of current card_id
        let mut matches: u32 = 0;
        for win_num in current_card.win_nums.iter() {
            for check_num in current_card.check_nums.iter() {
                if win_num == check_num {
                    matches+=1;
                }
            }
        }
        let card_match_ids: Vec<_> = (0..matches).map(|j| (card_id as u32)+1+j).collect();

        // Add new ids to deck
        for id in card_match_ids.into_iter() {
            if id <= num_of_original_cards {
                let match_card: Card = cards
                    .clone()
                    .into_iter()
                    .filter(|c| c.id == id)
                    .next()
                    .expect("Unable to get match card");
                for _ in 0..num_of_current_cards {
                    cards.push(match_card.clone());
                }
            }
        }
    }
    println!("Num. Cards: {}", cards.len());
}