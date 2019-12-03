use std::env;
use std::fs;


enum Direction {
    Right,
    Left,
    Up,
    Down
}
struct Point(i32, i32);
struct Wire {
    direction : Direction,
    length : i32
}

fn parse_input(input: &str) {
    input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|s| Wire {
                    direction : match s.char().next().unwrap() {
                        'U' => Direction::Up,
                        'R' => Direction::Right,
                        'D' => Direction::Down,
                        'L' => Direction::Left,
                        _   =>  {
                            eprintln!("Invalid input");
                            unreachable!()
                        }
                    },
                    length : s[1..].parse().unwrap()
                })
                .collect()
        })
}



fn part2(vec: &Vec<i32>) -> () {
    let mut noun = 0;
    let mut verb = 0;

    let target = 19690720;
    while part1(vec, noun, verb) < target {
        noun += 1;
    }

    noun -= 1;
    while part1(vec, noun, verb) < target {
        verb += 1;
    }

    println!("Values : {} and {}", noun, verb);
}


fn part1(v: &Vec<Wire>) -> i32 {

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = args[1].clone();

    let file_contents = fs::read_to_string(&input_filename)
        .unwrap_or_else(|err| {
            eprintln!("Error : {}", err);
            eprintln!("Cannot read from file {}", input_filename);
            std::process::exit(1);
        });

    let vec = parse_input(&file_contents);

//    println!("The total fuel needed is {}", part1(&vec,72, 94));
//    println!("The total fuelify  is {}", part2(&vec));
    part2(&vec);
}