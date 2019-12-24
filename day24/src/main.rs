use std::env;
use std::fs;
use std::collections::HashSet;

#[derive(Copy, Clone, PartialEq)]
enum Tile {
    Empty = '.' as isize,
    Bug = '#' as isize
}
struct BugPlanet {
    map: Vec<Vec<Tile>>,
    rating: i128,
    hash: HashSet<i128>
}

fn get_neighbours(current_position: (i32, i32)) -> Vec<(i32, i32)> {
    vec![
        (current_position.0, current_position.1 + 1),
        (current_position.0, current_position.1 - 1),
        (current_position.0 + 1, current_position.1),
        (current_position.0 - 1, current_position.1),
    ]
}

impl BugPlanet {
    fn new(input_filename: &String) -> BugPlanet {
        let file_contents = fs::read_to_string(input_filename).unwrap_or_else(|err| {
            eprintln!("Error : {}", err);
            eprintln!("Cannot read from file {}", input_filename);
            std::process::exit(1);
        });
    
        BugPlanet { 
            map : parse_bugs(&file_contents),
            rating: 0,
            hash: HashSet::new()
        }
    }

    fn simulate(&mut self) -> bool {
        let old_map = self.map.clone();
        let mut p: i128 = 1;
        self.rating = 0;
        
        for j in 0..5 {
            for i in 0..5 {
                let bugs = get_neighbours((i,j))
                    .into_iter()
                    .filter(|(x,y)| *x >= 0 && *x < 5 && *y >= 0 && *y < 5)
                    .filter(|(x,y)| old_map[*y as usize][*x as usize] == Tile::Bug)
                    .count();
                if old_map[j as usize][i as usize] == Tile::Bug && bugs != 1 {
                    self.map[j as usize][i as usize] = Tile::Empty;
                }
                if old_map[j as usize][i as usize] == Tile::Empty && (bugs == 1 || bugs == 2) {
                    self.map[j as usize][i as usize] = Tile::Bug;
                }
                if self.map[j as usize][i as usize] == Tile::Bug {
                    self.rating += p;
                }
                p *= 2;
            }
        }
        self.print();

        if self.hash.contains(&self.rating) {
            println!("Found first repeating rating {}", self.rating);
            return false;
        } else {
            self.hash.insert(self.rating);
            return true;
        }
    }

    fn print(&self) {
        for line in &self.map {
            for i in line {
                print!("{}", *i as u8 as char);
            }
            println!();
        }
        println!();
    }

}

fn parse_bugs(file_contents: &String) -> Vec<Vec<Tile>> {
    let mut bugs:Vec<Vec<Tile>> = Vec::new();
    for l in file_contents.lines() {
        bugs.push( l.chars().fold(Vec::new(), |mut acc, c| {
            match c {
                '#' => acc.push(Tile::Bug),
                '.' => acc.push(Tile::Empty),
                _   => unreachable!()
            };
            acc
        }))
    }


    bugs
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut eris = BugPlanet::new(&args[1]);

    while eris.simulate() {
        
    }
    

}
