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
    for i in 0..steps {
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

    println!("sequence after {} step : \n {:?}", steps, lst);
}

// fn get_at_offset(sequence: &Vec<u32>, steps: usize, offset: i32) {
//     let base_pattern: Vec<i32> = vec![0, 1, 0, -1];
//     let cyclon: Vec<Vec<(usize, i32)>> = (1..=sequence.len())
//         .into_iter()
//         .map(|i| {
//             base_pattern.clone()
//                 .into_iter()
//                 .cycle()
//                 .map(|v| vec![v;i])
//                 .flatten()
//                 .enumerate()
//                 .skip(1)
//                 .take(sequence.len())
//                 .into_iter()
//                 .skip__while(|(_, x)| x == 0)
//                 .collect()
//         })
//         .collect();

//     println!("{:?}", cyclon);
//     // let mut lst:Vec<i32> = sequence.iter().map(|x| *x as i32).collect();
//     // for i in 0..steps {
//     //     lst = cyclon[0..]
//     //         .into_iter()
//     //         .map(|(index: usize, extended_pattern:)| {
//     //             let s: i32 = extended_pattern
//     //                 .into_iter()
//     //                 .enumerate()
//     //                 .map(|p| (lst[index] * p ))
//     //                 .sum();
//     //             s.abs() % 10
//     //         })
//     //         .collect();
//     // }

    // println!("sequence after {} step : \n {:?}", steps, lst);
// }

fn part2(sequence: &Vec<i32>, steps: usize, offset: usize) {
    let mut lst:Vec<i32> = sequence.iter().map(|x| *x as i32).collect();
    for s in 0..steps {
        // lst[offset] = &sequence[offset..].into_iter().sum() %10;
        let mut i = sequence.len() - 2;
        while i >= offset-2 {
            lst[i] = (lst[i] + lst[i+1]) % 10;
            i -= 1;
        }
        // println!("{} {}", s, sequence.len() );
    }

    let n = vec![0,1,2,3,4,5,6,7].into_iter().fold(0, |acc, i| acc * 10 + sequence[offset + i]);
    println!("{:?}", n);
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
    // part1(&sequence, 100);
    // println!("{:?}", sequence);
    // get_at_offset(&sequence, 100, 0);
    let mut repetitions = 10000 ;
    let mut long_sequence = Vec::new();
    while repetitions > 0 {
        long_sequence.extend(sequence.clone().iter());
        repetitions -= 1;
    }

    part2(&long_sequence, 100, 5970417);
}
