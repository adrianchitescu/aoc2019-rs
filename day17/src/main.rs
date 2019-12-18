extern crate int_computer;

use std::env;
use int_computer::computer::*;
use itertools::Itertools;


fn get_view(computer: &mut Computer) -> Vec<Vec<u8>> {
    let mut view: Vec<Vec<u8>> = Vec::new();
    computer.run();
    let mut output: Vec<i128> = computer.get_all_output();
    let endline_pos = output.iter().find_position(|o| **o as i32 == 10);
    if let Some((line_length, _)) = endline_pos  {
        view = output
            .iter_mut()
            .chunks(line_length + 1)
            .into_iter()
            .map(|line| line.into_iter().map(|x| *x as u8).take(line_length).collect())
            .collect();
    } else {
        println!("Failed to parse");
    }

    for line in view.clone() {
        for j in line {
            print!("{}", j as char);
        }
        println!();
    }
    println!("view :\n {:?}", view);
    view
}

fn get_neighbours(current_position: &(i32, i32)) -> Vec<(i32, i32)> {
    vec![
        (current_position.0, current_position.1 + 1),
        (current_position.0, current_position.1 - 1),
        (current_position.0 + 1, current_position.1),
        (current_position.0 - 1, current_position.1),
    ]
}

fn get_intersections(view: &Vec<Vec<u8>>) -> usize {
    let mut intersections:Vec<usize> = Vec::new();
    let width = view[0].len();
    let height = view.len() -1 ;
    println!("{}, {}", width, height);
    for i in 0..height {
        for j in 0..width {
            if view[i][j] == '#' as u8 {
                let inters = get_neighbours(&(i as i32, j as i32))
                    .into_iter()
                    .filter(|(ii, jj)| {
                        if let Some(l) = view.get(*ii as usize) {
                            if let Some(v) = l.get(*jj as usize) {
                                *v == '#' as u8
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    })
                    .count();
                if inters > 2 {
                    intersections.push(i *  j);
                }
            }
        }
    }

    println!("{:?}", intersections);
    intersections.into_iter().sum()
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut computer = Computer::new_from_file(&args[1]);
    let mut view = get_view(&mut computer);
    println!("{:?}", get_intersections(&view));
}
