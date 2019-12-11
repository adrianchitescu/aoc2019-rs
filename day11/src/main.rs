use std::fs;
use std::env;
use std::collections::HashMap;

extern crate int_computer;
use int_computer::computer::*;

enum Panel {
    White,
    Black
}
#[derive(Debug, Copy, Clone)]
enum Orientation {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

fn rotate_robot(current_orientation: &mut Orientation, position: (i32, i32), direction: i32) -> (i32, i32){
    match *current_orientation {
        Orientation::UP =>  {
            if direction == 0 {
                *current_orientation = Orientation::LEFT;
                (position.0 - 1, position.1)
            } else {
                *current_orientation = Orientation::RIGHT;
                (position.0 + 1, position.1)
            }
        }
        Orientation::DOWN =>  {
            if direction == 0 {
                *current_orientation = Orientation::RIGHT;
                (position.0 + 1, position.1)
            } else {
                *current_orientation = Orientation::LEFT;
                (position.0 - 1, position.1)
            }
        }
        Orientation::RIGHT=> {
            if direction == 0 {
                *current_orientation = Orientation::UP;
                (position.0, position.1 + 1)
            } else {
                *current_orientation = Orientation::DOWN;
                (position.0, position.1 - 1)
            }
        }
        Orientation::LEFT => {
            if direction == 0 {
                *current_orientation = Orientation::DOWN;
                (position.0, position.1 - 1)
            } else {
                *current_orientation = Orientation::UP;
                (position.0, position.1 + 1)
            }
        }
    }
}

fn paint(computer: &mut Computer, hull: &mut HashMap<(i32, i32), i32>, start: (i32, i32)) -> usize {
    let mut pos = start;
    let mut robot_orientation = Orientation::UP;
    loop {
        let panel_color = hull.get(&pos).unwrap_or(&1);
        computer.add_input(*panel_color);
        let state = computer.run();
        let mut new_color = computer.get_output().unwrap();
        let mut rotation = computer.get_output().unwrap();
        hull.insert(pos, new_color as i32);
        pos = rotate_robot(&mut robot_orientation, pos, rotation as i32);
        if state == State::Done {
            break;
        }
    }

    hull.keys().len()
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = args[1].clone();

    let file_contents = fs::read_to_string(&input_filename).unwrap_or_else(|err| {
        eprintln!("Error : {}", err);
        eprintln!("Cannot read from file {}", input_filename);
        std::process::exit(1);
    });

    let instructions = int_computer::computer::read_instructions(&file_contents);
    let mut computer = Computer::new(&instructions);

    let mut hull:HashMap<(i32, i32), i32> = HashMap::new();

    println!("Painted {} panels", paint(&mut computer, &mut hull, (0,0)));

    for j in (0..6).rev() {
        for i in 0..40 {
            let c = hull.get(&(i, j - 5)).unwrap_or(&0);
            if *c == 1 {
                print!("XX");
            } else {
                print!("  ");
            }
        }
        println!()
    }
}
