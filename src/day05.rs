use std::collections::HashMap;
use std::fs;
use std::iter::zip;

#[derive(Debug)]
struct AlmanacItem {
    source: String,
    source_num: usize,
    dest: String,
    dest_num: usize,
    range: usize,
}

fn main() {
    let input = fs::read_to_string("inputs/day05.txt").expect("Error reading input");

    let mut input_lines = input.split("\n").collect::<Vec<_>>();

    // Get seeds
    let seed_str: String = input_lines.remove(0).to_owned();
    let seed_str = seed_str
        .split_once(":")
        .expect("Unable to split seed_str")
        .1;
    let mut seeds: Vec<usize> = vec![];
    for item in seed_str.split(" ") {
        if !item.is_empty() {
            seeds.push(item.parse::<usize>().expect("Unable to parse item"));
        }
    }

    // Create almanac mapping
    let mut almanac: Vec<AlmanacItem> = vec![];
    let mut current_source = String::new();
    let mut current_dest = String::new();
    for line in input_lines.iter() {
        if line.is_empty() {
            continue;
        } else if line.contains("map") {
            let map_str = line.split_once(" ").expect("Unable to split").0.to_owned();
            current_source = map_str
                .split_once("-")
                .expect("Unable to split")
                .0
                .to_owned();
            current_dest = map_str
                .rsplit_once("-")
                .expect("Unable to split")
                .1
                .to_owned();
        } else {
            let mappings = line
                .splitn(3, " ")
                .map(|item| item.parse::<usize>().expect("Unable to parse item"))
                .collect::<Vec<usize>>();
            let item = AlmanacItem {
                source: current_source.clone(),
                source_num: mappings.get(1).unwrap().to_owned(),
                dest: current_dest.clone(),
                dest_num: mappings.get(0).unwrap().to_owned(), // NOTE: dest first
                range: mappings.get(2).unwrap().to_owned(),
            };
            almanac.push(item);
        }
    }

    // Find lowest location seed
    let sources_chain = vec![
        "seed",
        "soil",
        "fertilizer",
        "water",
        "light",
        "temperature",
        "humidity",
    ];
    let mut locations: Vec<usize> = vec![];
    for seed_num in &seeds {
        let mut num = seed_num.to_owned();
        for source in &sources_chain {
            let map: Option<&AlmanacItem> = almanac
                .iter()
                .filter(|item| {
                    item.source == source.to_owned()
                        && (item.source_num..item.source_num + item.range).contains(&num)
                })
                .next();
            match map {
                Some(map) => {
                    num = (num - map.source_num) + map.dest_num;
                }
                None => {
                    num = num; // if no mapping, then source == dest
                }
            }
        }
        locations.push(num);
    }
    println!("Lowest Location: {}", locations.iter().min().unwrap());

    // Part 2
    // Create almanac mapping
    // TODO: research data structs
    let mut almanac: HashMap<(String, usize), (String, usize)> = HashMap::new();
    let mut current_source = String::new();
    let mut current_dest = String::new();
    println!("Creating almanac HashMap...");
    for line in input_lines.iter() {
        if line.is_empty() {
            continue;
        } else if line.contains("map") {
            let map_str = line.split_once(" ").expect("Unable to split").0.to_owned();
            current_source = map_str
                .split_once("-")
                .expect("Unable to split")
                .0
                .to_owned();
            current_dest = map_str
                .rsplit_once("-")
                .expect("Unable to split")
                .1
                .to_owned();
        } else {
            let mappings = line
                .splitn(3, " ")
                .map(|item| item.parse::<usize>().expect("Unable to parse item"))
                .collect::<Vec<usize>>();

            let source = current_source.clone();
            let source_num = mappings.get(1).unwrap().to_owned();
            let dest = current_dest.clone();
            let dest_num = mappings.get(0).unwrap().to_owned(); // NOTE: dest first
            let range = mappings.get(2).unwrap().to_owned();

            for (s, d) in zip(source_num..source_num+range, dest_num..dest_num+range) {
                almanac.insert((source.to_owned(), s), (dest.to_owned(), d));
            }
        }
    }
    println!("Creating almanac HashMap...DONE!");

    let mut new_seeds: Vec<usize> = vec![];
    for pair in seeds.windows(2) {
        println!("Checking {:?}", pair);
        // Find overlaping seed nums for seed ranges and seed-to-soil maps
        let (seed_num, seed_range) = (pair.get(0).unwrap().clone(), pair.get(1).unwrap().clone());
        for num in seed_num..seed_num+seed_range {
            if almanac.contains_key(&("seed".to_string(), num)) {
                new_seeds.push(num);
            }
        }
    }
    dbg!(new_seeds.len());



    // == Old Brute Force
    // // // Maybe Cheating - Just get all seed-to-soil nums to start, no need to check all ranges if we don't have a map for it.
    // let seed_maps: Vec<&AlmanacItem> = almanac
    //     .iter()
    //     .filter(|item| {
    //         item.source == "seed"
    //     })
    //     .collect();

    // let mut new_seeds: Vec<usize> = vec![];
    // let mut locations: Vec<usize> = vec![];
    // for pair in seeds.windows(2) {
    //     println!("Checking {:?}", pair);
    //     // Find overlaping seed nums for seed ranges and seed-to-soil maps
    //     let (seed, seed_range) = (pair.get(0).unwrap(), pair.get(1).unwrap());
    //     for seed_map in seed_maps.clone() {
    //         let (x1, x2) = (seed, seed+seed_range);
    //         let (y1, y2) = (seed_map.source_num, seed_map.source_num+seed_map.range);

    //         if x1 <= &y2 && y1 <= x2 {
    //             let lower = *cmp::max(x1, &y1) as usize;
    //             let upper = cmp::min(x2, y2) as usize;

    //             for seed_num in lower..upper+1 {
    //                 new_seeds.push(seed_num.clone());
    //             }
    //         }

    //     }

    //     // Check seeds from that pair
    //     while !new_seeds.is_empty() {
    //         println!("Processing {} seed nums", new_seeds.len());
    //         let mut num = new_seeds.pop().unwrap();
    //         for source in &sources_chain {
    //             let map: Option<&AlmanacItem> = almanac
    //                 .iter()
    //                 .find(|item| {
    //                     item.source == source.to_owned()
    //                         && (item.source_num..item.source_num + item.range).contains(&num)
    //                 });
    //             match map {
    //                 Some(map) => {
    //                     num = (num - map.source_num) + map.dest_num;
    //                 },
    //                 None => {
    //                     num = num;  // if no mapping, then source == dest
    //                 }
    //             }
    //         }
    //         if !locations.contains(&num) {
    //             locations.push(num);
    //         }
    //     }
    // }
    // println!("Lowest Location from Seed Ranges: {}", locations.iter().min().unwrap());
}