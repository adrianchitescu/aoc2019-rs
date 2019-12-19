extern crate int_computer;

use std::env;
use int_computer::computer::*;
use itertools::Itertools;
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;

#[derive(Copy, Debug, Clone, FromPrimitive)]
enum Orientation {
    NORTH = 0,
    EAST  = 1,
    SOUTH = 2,
    WEST  = 3
}

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
            .filter(|line: &Vec<u8>| line.len() == line_length)
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

fn is_intersection(view: &Vec<Vec<u8>>, position: (i32, i32)) -> bool {
    get_neighbours(&(position.0 as i32, position.1 as i32))
        .into_iter()
        .filter(|p| {
            is_scaffold(view, *p, (0, 0))
        })
        .count() > 2
}
fn get_intersections(view: &Vec<Vec<u8>>) -> usize {
    let mut intersections:Vec<usize> = Vec::new();
    let width = view[0].len();
    let height = view.len();
    for i in 0..height {
        for j in 0..width {
            if view[i][j] == '#' as u8 {
                if is_intersection(view, (j as i32, i as i32)) {
                    intersections.push(i *  j);
                }
            }
        }
    }

    println!("{:?}", intersections);
    intersections.into_iter().sum()
}

fn get_start_position(view: &Vec<Vec<u8>>) -> Option<(Orientation, (i32, i32))> {
    for (i, line) in view.iter().enumerate() {
        for (j, el) in line.iter().enumerate() {
            if *el == '#' as u8 || *el == '.' as u8 {
                continue;
            } else {
                let o = match *el as char {
                    '^' => Orientation::NORTH,
                    'V' => Orientation::SOUTH,
                    '<' => Orientation::EAST,
                    '>' => Orientation::WEST,
                    _   => unreachable!()
                };
                return Some((o, (j as i32, i as i32)));
            }
        }
    }
    None
}

fn is_scaffold(view: &Vec<Vec<u8>>, (x, y): (i32, i32), (delta_x, delta_y): (i32, i32)) -> bool {
    // print!("\t is_scaff {:?} {:?}", position, delta);
    if let Some(l) = view.get((y + delta_y) as usize) {
        if let Some(v) = l.get((x + delta_x) as usize) {
            if *v == '#' as u8 {
                // println!(" = true");
            } else {
                // println!("= FALSE");
            }
            *v == '#' as u8
        } else {
            // println!("= FALSE");
            false
        }
    } else {
        // println!("= FALSE");
        false
    }
}

fn print_v(view: &Vec<Vec<u8>>) {
    for line in view {
        for j in line {
            print!("{}", *j as char);
        }
        println!();
    }
}

struct Robot {
    map: Vec<Vec<u8>>,
    x: i32,
    y: i32,
    orientation: Orientation
}

impl Robot {
    fn new (view: &Vec<Vec<u8>>) -> Robot {
        let (orientation, position) =  get_start_position(view).unwrap();
        Robot {
            map : view.clone(),
            x : position.0,
            y : position.1,
            orientation : orientation
        }
    }
    fn get(&mut self, (x, y): (i32, i32)) -> Option<&mut u8> {
        if let Some(l) = self.map.get_mut(y as usize) {
            l.get_mut(x as usize)
        } else {
            None
        }
    }

    fn move_ahead(&mut self) -> bool {
        let (mut new_x, mut new_y) = (self.x, self.y);
        println!();
        print!("{:?}", self.orientation);
        match self.orientation {
            Orientation::NORTH => { new_y -= 1; },
            Orientation::EAST  => { new_x += 1; },
            Orientation::SOUTH => { new_y += 1; },
            Orientation::WEST  => { new_x -= 1; }
        };
        let ahead = self.get((new_x, new_y));
        if let Some(p) =  ahead {
            print!(" {:?}   ", *p as char);
            if *p == '#' as u8 {
                println!(" ---- Moved to {},{}", new_x, new_y);
                self.x = new_x;
                self.y = new_x;
                *p = '.' as u8;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn rotate(&mut self) -> Option<u8> {
        let (x, y) = (self.x, self.y);
        let (left, right) = match self.orientation {
            Orientation::NORTH => (self.get((x - 1, y)), self.get((x + 1, y))),
            Orientation::EAST  => (self.get((x, y -1)),  self.get((x, y + 1))),
            Orientation::SOUTH => (self.get((x + 1, y)), self.get((x - 1, y))),
            Orientation::WEST  => (self.get((x, y + 1)), self.get((x, y + 1)))
        };

        if *left.unwrap_or(&mut 0) == '#' as u8 {
            print!("L,");
            self.orientation = Orientation::from_i32((self.orientation as i32- 1) % 4).unwrap();
            Some('L' as u8)
        } else if *right.unwrap_or(&mut 0) == '#' as u8 {
            print!("R,");
            self.orientation = Orientation::from_i32((self.orientation as i32 + 1) % 4).unwrap();
            Some('R' as u8)
        } else {
            None
        }
    }
}

impl Iterator for Robot {
    type Item = u8;
    fn next(&mut self) -> Option<u8> {
        let mut length = 0;
        while self.move_ahead() {
            length += 1;
        }
        if length > 0 {
            print!("{},", length);
            Some(length)
        } else {
            self.rotate()
        }
    }
}

fn get_move_routine(view: &Vec<Vec<u8>>) -> Vec<u8> {
    let mut routine: Vec<u8> = Vec::new();
    let mut robot = Robot::new(&view);

    let mut i = 0;

    let v:Vec<u8> = robot
        .into_iter()
        .take(100)
        .collect();

    // loop {
    //     i += 1;
    //     if i == 50 {
    //         break;
    //     }
    //     let delta_next = dv[orientation as usize];
    //     if is_scaffold(&mview, position, delta_next) {
    //         if !is_intersection(&mview, position) {
    //             mview[position.1 as usize][position.0 as usize] = '.' as u8;
    //         }
    //         current_length += 1;
    //         position = (position.0 + delta_next.0, position.1 + delta_next.1);
    //     } else {
    //         print!("{}, {:?}", current_length, orientation);
    //         current_length = 0;
    //         if let Some(scaff_position) = dv.iter().position(|delta| is_scaffold(&mview, position, *delta)) {
    //             println!("{}", scaff_position);
    //             if (scaff_position  + 1) % 4 == orientation as usize {
    //                 orientation = Orientation::from_i32((orientation as i32 - 1) % 4).unwrap();
    //                 print!("L,");
    //             } else {
    //                 orientation = Orientation::from_i32((orientation as i32 + 1) % 4).unwrap();
    //                 print!("R,");
    //             }
    //             let delta_next = dv[scaff_position];
    //             // position = (position.0 + delta_next.0, position.1 + delta_next.1);
    //         } else {
    //             break;
    //         }
    //     }
    //     // print_v(&mview);

    // }

    routine
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut computer = Computer::new_from_file(&args[1]);
    let mut view = get_view(&mut computer);
    println!("{:?}", get_intersections(&view));
    get_move_routine(&view);
}
