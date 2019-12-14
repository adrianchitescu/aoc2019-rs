extern crate regex;
use std::env;
use std::fs;
use std::collections::HashMap;
use std::collections::VecDeque;
use regex::Regex;

type Reaction = (i64, Vec<(String, i64)>);

fn parse_reactions(input: &String) -> HashMap<String, Reaction> {
    let re = Regex::new(r"(\d+) ([A-Z]+)").unwrap();
    let mut reactions = HashMap::new();
    for l in input.lines() {
        let mut captures = re.captures_iter(l).peekable();
        let mut ins = Vec::new();
        while let Some(c) = captures.next() {
            let q = c[1].parse::<i64>().unwrap();
            if captures.peek().is_some() {
                ins.push((c[2].to_string(), q));
            } else {
                reactions.insert(c[2].to_string(), (q, ins));
                break;
            }
        }
    }

    reactions
}

fn get_ore(reactions: &HashMap<String, Reaction>, total_fuel: i64) -> i64 {
    let mut queue = VecDeque::new();
    let mut extra: HashMap<String, i64> = HashMap::new();
    let mut total_ore = 0;
    let fuel_r: &Reaction = reactions.get("FUEL").unwrap();
    queue.extend(fuel_r.1.clone().into_iter().map(|(s,c)|{
        (s, c as i64 * total_fuel)
    }));

    while queue.len() > 0 {
        let (substance, mut quantity) = queue.pop_front().unwrap();
        if substance == "ORE" {
            total_ore += quantity;
            continue;
        }
        if let Some(extra_substance) = extra.get_mut(&substance) {
            if (quantity as i64) < *extra_substance {
                *extra_substance -= quantity as i64;
                continue;
            } else {
                quantity -= *extra_substance;
                extra.remove(&substance);
            }
        }
        // we still need @quantity of substance
        let (oq, rvec) = reactions.get(&substance).unwrap();
        let mut multiplier = quantity/ oq;
        if quantity % oq != 0 {
            multiplier += 1;
        }
        let produced = multiplier * oq;
        let extra_q = produced - quantity;
        if extra_q > 0 {
            extra.insert(substance, extra_q as i64);
        }
        queue.extend(rvec.clone().into_iter().map(|(s,c)|{
            (s, c * multiplier)
        }))     
    }

    total_ore
}

fn get_max_fuel_for(total_ore: i64, reactions: &HashMap<String, Reaction>) -> i64 {
    let ore_per_fuel = get_ore(&reactions, 1) as i64;
    let mut range = (ore_per_fuel, ore_per_fuel * 2);

    loop {
        if range.1 - range.0 <2 {
            break;
        }
        let mid = (range.1 + range.0) / 2;
        let ore = get_ore(&reactions, mid);
        if ore < total_ore {
            range = (mid + 1, range.1);
        } else {
            range = (range.0, mid -1);
        }
    }

    range.0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = args[1].clone();

    let file_contents = fs::read_to_string(&input_filename).unwrap_or_else(|err| {
        eprintln!("Error : {}", err);
        eprintln!("Cannot read from file {}", input_filename);
        std::process::exit(1);
    });

    let reactions = parse_reactions(&file_contents);
    
    let ore_per_fuel = get_ore(&reactions, 1) as i64;
    println!("1 fuel = {} ore", ore_per_fuel);
    println!("max fuel = {:?}", get_max_fuel_for(1000000000000 as i64, &reactions));
}
