use permutator::Permutation;

use std::env;
use std::fs;

extern crate int_computer;
use int_computer::computer::*;

fn parse_input(input: &str) -> Vec<i32> {
    let mut vec = Vec::new();
    for n in input.split_terminator(',') {
        if let Ok(f) = n.parse::<i32>() {
            vec.push(f);
        } else {
            eprintln!("invalid value in the provided input");
            break;
        }
    }
    vec
}

fn run_amplifiers(phase_seq: &Vec<i32>, program: &Vec<i32>) -> i32 {
    let mut amplifiers: Vec<Computer> = phase_seq
        .into_iter()
        .map(|phase| {
            let mut c = Computer::new(&program);
            c.add_input(*phase);
            c
        })
        .collect();

    let mut last_out = Some(0);
    let mut index = 0;
    loop {
        if let Some(o) = last_out {
            amplifiers[index].add_input(o);
        }
        let s = amplifiers[index].run();
        if s == State::Done && index == amplifiers.len() - 1 {
            last_out = amplifiers[index].get_exit_value();
            break;
        }

        last_out = amplifiers[index].get_output();
        index = (index + 1) % amplifiers.len();
    }

    last_out.unwrap()
}

fn get_max_signal(program: &Vec<i32>, phase: &[i32]) -> i32 {
    if let Some(max_signal) = phase.to_vec()
        .permutation()
        .into_iter()
        .map(|p| run_amplifiers(&p, &program.clone()))
        .max_by(|x, y| x.cmp(y))
    {
        println!("Max signal is {}", max_signal);
        max_signal
    } else {
        println!("Max signal is missing");
        -1
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = args[1].clone();

    let file_contents = fs::read_to_string(&input_filename).unwrap_or_else(|err| {
        eprintln!("Error : {}", err);
        eprintln!("Cannot read from file {}", input_filename);
        std::process::exit(1);
    });

    let vec = parse_input(&file_contents);
    get_max_signal(&vec.clone(), &[0, 1, 2, 3, 4]);
    get_max_signal(&vec.clone(), &[5, 6, 7, 8, 9]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amp() {
        let vec = parse_input("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
        assert_eq!(get_max_signal(&vec.clone(), &[0, 1, 2, 3, 4]), 43210);
    }

    #[test]
    fn test_amp2() {
        let vec =
            parse_input("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0");
        assert_eq!(get_max_signal(&vec.clone(), &[0, 1, 2, 3, 4]), 54312);
    }

    #[test]
    fn test_amp3() {
        let vec = parse_input(
            "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0",
        );
        assert_eq!(get_max_signal(&vec.clone(), &[0, 1, 2, 3, 4]), 65210);
    }

    #[test]
    fn test_loop_amp() {
        let vec = parse_input(
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
        );
        assert_eq!(get_max_signal(&vec.clone(), &[5, 6, 7, 8, 9]), 139629729);
    }

    #[test]
    fn test_loop_amp2() {
        let vec = parse_input(
            "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10",
        );
        assert_eq!(get_max_signal(&vec.clone(), &[5, 6, 7, 8, 9]), 18216);
    }
}
