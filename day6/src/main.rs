use std::collections::HashMap;
use std::env;
use std::fs;
use std::hash::Hash;

fn parse(input: &str) -> HashMap<&str, &str> {
    let mut orbits: HashMap<&str, &str> = HashMap::new();
    for line in input.lines() {
        let planets: Vec<&str> = line.split_terminator(')').collect();
        orbits.insert(planets[1], planets[0]);
    }

    orbits
}

fn count_orbits(orbit_map: &HashMap<&str, &str>) -> u32 {
    let mut count: u32 = 0;

    for (planet, orbit) in orbit_map {
        let mut cnt: u32 = 1;
        let mut current_planet = orbit;
        while *current_planet != "COM" {
            cnt += 1;
            current_planet = orbit_map.get(current_planet).unwrap();
        }

        count += cnt;
    }

    count
}

fn count_tranfers(orbit_map: &HashMap<&str, &str>, src: &str, dest: &str) -> u32 {
    let mut transfers: u32 = 0;
    let mut src_dist_map: HashMap<&str, u32> = HashMap::new();
    let mut current_planet = src;
    let mut distance = 0;
    src_dist_map.insert(current_planet, distance);

    while current_planet != "COM" {
        current_planet = orbit_map.get(current_planet).unwrap();
        src_dist_map.insert(current_planet, distance);
        distance += 1;
    }

    current_planet = orbit_map.get(dest).unwrap();
    distance = 0;
    loop {
        if let Some(d) = src_dist_map.get(current_planet) {
            transfers = distance + d;
            break;
        } else {
            distance += 1;
            current_planet = orbit_map.get(current_planet).unwrap();
        }
    }

    transfers
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = args[1].clone();
    let file_contents = fs::read_to_string(&input_filename).unwrap_or_else(|err| {
        eprintln!("Error : {}", err);
        eprintln!("Cannot read from file {}", input_filename);
        std::process::exit(1);
    });

    let orbits = parse(&file_contents);
    let count = count_orbits(&orbits);

    println!("Total orbits {}", count);
    println!("Total transfers {}", count_tranfers(&orbits, "YOU", "SAN"));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let orbits = parse(
            "COM)B\n\
             B)C\n\
             C)D\n\
             D)E\n\
             E)F\n\
             B)G\n\
             G)H\n\
             D)I\n\
             E)J\n\
             J)K\n\
             K)L\n\
             ",
        );
        assert_eq!(orbits.get("B").unwrap_or(&""), &"COM");
        assert_eq!(orbits.get("C").unwrap_or(&""), &"B");
        assert_eq!(orbits.get("G").unwrap_or(&""), &"B");
    }
    #[test]
    fn test_count() {
        let orbits = parse(
            "COM)B\n\
             B)C\n\
             C)D\n\
             D)E\n\
             E)F\n\
             B)G\n\
             G)H\n\
             D)I\n\
             E)J\n\
             J)K\n\
             K)L\n\
             ",
        );
        assert_eq!(count_orbits(&orbits), 42);
    }

    #[test]
    fn test_transfers() {
        let orbits = parse(
            "COM)B\n\
             B)C\n\
             C)D\n\
             D)E\n\
             E)F\n\
             B)G\n\
             G)H\n\
             D)I\n\
             E)J\n\
             J)K\n\
             K)L\n\
             K)YOU\n\
             I)SAN",
        );
        assert_eq!(count_tranfers(&orbits, "YOU", "SAN"), 4);
    }
}
