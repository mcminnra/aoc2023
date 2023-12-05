use std::collections::HashMap;
use std::fs;
use std::ops::Range;
use std::cmp;

#[derive(Debug)]
struct AlmanacItem {
    source: String,
    source_num: usize,
    dest: String,
    dest_num: usize,
    range: usize,
}

fn is_overlap(a: Range<usize>, b: Range<usize>) -> bool {
    a.start < b.end && b.start < a.end
}

fn get_overlap(a: Range<usize>, b: Range<usize>) -> Range<usize> {
    let lower = cmp::max(a.start, b.start) as usize;
    let upper = cmp::min(a.end, b.end) as usize;

    lower..upper
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
    let mut almanac: HashMap<(String, Range<usize>), (String, Range<usize>)> = HashMap::new();
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
            almanac.insert(
                (source.to_owned(), source_num..source_num + range),
                (dest.to_owned(), dest_num..dest_num + range),
            );
        }
    }
    println!("Creating almanac HashMap...DONE!");

    let sources_chain = vec![
        "seed",
        "soil",
        "fertilizer",
        "water",
        "light",
        "temperature",
        "humidity",
    ];

    let mut source_pairs: Vec<(String, Range<usize>)> = seeds
        .windows(2)
        .map(|pair| {
            (pair.get(0).unwrap().clone(), pair.get(1).unwrap().clone())
        })
        .map(|(start, range)| {
            ("seed".to_string(), (start..start+range))
        })
        .collect();

    let mut locations: Vec<usize> = vec![];
    while !source_pairs.is_empty() {
        let (current_source, current_range) = source_pairs.pop().unwrap();
        //dbg!((&current_source, &current_range));

        let keys: Vec<_> = almanac
            .keys()
            .filter(|(key_source, key_source_range)| {
                *key_source == current_source && is_overlap(current_range.clone(), key_source_range.clone())
            })
            .collect();

        if keys.is_empty() {
            let source_idx = sources_chain.iter().position(|&s| s == current_source).unwrap();
            if source_idx != sources_chain.len()-1 {
                source_pairs.push((sources_chain.get(source_idx+1).unwrap().to_string(), current_range.clone()));
            }
        }

        for key in keys {
            let (key_source, key_source_range) = key;
            let (key_dest, key_dest_range) = almanac.get(&key).unwrap();

            let source_overlap = get_overlap(current_range.clone(), key_source_range.clone());
            
            // Put prefixes and suffixes back in queue
            if current_range.start != source_overlap.start {
                let source_prefix = current_range.start..source_overlap.start;
                let source_pair = (key_source.to_owned(), source_prefix);
                if !source_pairs.contains(&source_pair){
                    source_pairs.push(source_pair);
                }
            }
            if source_overlap.end != current_range.end {
                let source_suffix = source_overlap.end..current_range.end;
                let source_pair = (key_source.to_owned(), source_suffix);
                if !source_pairs.contains(&source_pair){
                    source_pairs.push(source_pair);
                }
            }
            
            // Convert to dest
            let start_delta = source_overlap.start-key_source_range.start;
            let end_delta = key_source_range.end-source_overlap.end;
            let dest_overlap = (key_dest_range.start+start_delta)..(key_dest_range.end-end_delta);

            // Processes overlap
            if key_dest == "location" {
                locations.push(dest_overlap.start);
                //dbg!(source_pairs.len());
                //dbg!(locations.iter().min().unwrap());
            } else {
                source_pairs.push((key_dest.to_owned(), dest_overlap));
            }
        }
    }
    println!("Lowest Location from Seed Ranges: {}", locations.iter().min().unwrap());

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
