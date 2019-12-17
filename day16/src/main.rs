use std::env;
use std::fs;

fn parse_input(input: &String) -> Vec<i32> {
    input.chars().into_iter().map(|c| c.to_digit(10).unwrap() as i32).collect()
}

fn part1(sequence: &Vec<i32>, steps: usize) {
    let base_pattern: Vec<i32> = vec![0, 1, 0, -1];
    let cyclon: Vec<Vec<i32>> = (1..=sequence.len())
        .into_iter()
        .map(|i| {
            base_pattern.clone()
                .into_iter()
                .cycle()
                .map(|v| vec![v;i])
                .flatten()
                .skip(1)
                .take(sequence.len())
                .collect()
        })
        .collect();

    let mut lst:Vec<i32> = sequence.iter().map(|x| *x as i32).collect();
    for _ in 0..steps {
        lst = cyclon[0..]
            .into_iter()
            .map(|extended_pattern| {
                let s: i32 = extended_pattern
                    .into_iter()
                    .enumerate()
                    .map(|(index, p)| (lst[index] * p ))
                    .sum();
                s.abs() % 10
            })
            .collect();
    }

    
    println!("Part1 : {}", get_number(&lst[0..], 8));
}

fn part2(sequence: &mut Vec<i32>, steps: usize) {
    for _ in 0..steps {
        for i in (0..sequence.len()-1).rev(){
            sequence[i] = (sequence[i] + sequence[i+1]) % 10;
        }
    }

    let n = get_number(&sequence[0..], 8);
    println!("Real signal {:?}", n);
}

fn get_number(v: &[i32], digits: usize) -> i32 {
    let mut n = 0;
    for i in 0..digits {
        n = n * 10 + v[i];
    }

    n
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = args[1].clone();

    let file_contents = fs::read_to_string(&input_filename).unwrap_or_else(|err| {
        eprintln!("Error : {}", err);
        eprintln!("Cannot read from file {}", input_filename);
        std::process::exit(1);
    });

    let sequence = parse_input(&file_contents);
    part1(&sequence, 100);

    let offset = get_number(&sequence[0..], 7);
    let length = sequence.len();
    let mut long_sequence:Vec<i32> = sequence
        .into_iter()
        .cycle()
        .skip(offset as usize % length as usize)
        .take(length * 10000 - offset as usize)
        .collect();

    part2(&mut long_sequence, 100);
}
