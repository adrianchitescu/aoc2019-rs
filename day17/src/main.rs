extern crate int_computer;
extern crate regex;

use std::env;
use int_computer::computer::*;
use itertools::Itertools;
use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
use regex::Regex;


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

    view
}

fn display(computer: &mut Computer) {
    computer.run();
    let last = computer.get_exit_value();
    let output: Vec<i128> = computer.get_all_output();

    for c in output {
        print!("{}", (c as u8) as char);
    }
    println!("{:?}", last);
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
    if let Some(l) = view.get((y + delta_y) as usize) {
        if let Some(v) = l.get((x + delta_x) as usize) {
            if *v == '#' as u8 {
            } else {
            }
            *v == '#' as u8
        } else {
            false
        }
    } else {
        false
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
            orientation
        }
    }
    fn get(&self, (x, y): (i32, i32)) -> Option<&u8> {
        if let Some(l) = self.map.get(y as usize) {
            l.get(x as usize)
        } else {
            None
        }
    }

    fn get_mut(&mut self, (x, y): (i32, i32)) -> Option<&mut u8> {
        if let Some(l) = self.map.get_mut(y as usize) {
            l.get_mut(x as usize)
        } else {
            None
        }
    }

    fn move_ahead(&mut self) -> bool {
        let (mut new_x, mut new_y) = (self.x, self.y);
        match self.orientation {
            Orientation::NORTH => { new_y -= 1; },
            Orientation::EAST  => { new_x += 1; },
            Orientation::SOUTH => { new_y += 1; },
            Orientation::WEST  => { new_x -= 1; }
        };
        let ahead = self.get_mut((new_x, new_y));
        if let Some(p) =  ahead {
            if *p == '#' as u8 {
                self.x = new_x;
                self.y = new_y;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn rotate(&mut self) -> Option<String> {
        let (x, y) = (self.x, self.y);
        let (left, right) = match self.orientation {
            Orientation::NORTH => (self.get((x - 1, y)), self.get((x + 1, y))),
            Orientation::EAST  => (self.get((x, y -1)),  self.get((x, y + 1))),
            Orientation::SOUTH => (self.get((x + 1, y)), self.get((x - 1, y))),
            Orientation::WEST  => (self.get((x, y + 1)), self.get((x, y - 1)))
        };
        // println!("{:?},{:?}", left, right);
        if *left.unwrap_or(&mut 0) == '#' as u8 {
            self.orientation = Orientation::from_i32((self.orientation as i32- 1 + 4) % 4).unwrap();
            Some("L".to_string())
        } else if *right.unwrap_or(&mut 0) == '#' as u8 {
            self.orientation = Orientation::from_i32((self.orientation as i32 + 1 + 4) % 4).unwrap();
            Some("R".to_string())
        } else {
            None
        }
    }
}

impl Iterator for Robot {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        let mut length = 0;
        while self.move_ahead() {
            length += 1;
        }
        if length > 0 {
            Some(length.to_string())
        } else {
            self.rotate()
        }
    }
}

fn get_move_routine(view: &Vec<Vec<u8>>) -> String {
    let robot = Robot::new(&view);
    robot
        .into_iter()
        .join(",")
}
fn get_next_start(s: &String) -> usize {
    s
        .chars()
        .position(|ch| "RL01234567890".contains(ch))
        .unwrap_or(0)
}

fn split_routine(routine: String) -> String {
    let mut active_routine:Vec<String> = vec!["".to_string();3];
    let mut a_fn: String;
    let mut b_fn: String;
    let mut c_fn: String;
    let re = Regex::new(r"[^ABC,]").unwrap();

    for i in 1..21 {
        active_routine[0] = routine.clone();
        a_fn = routine[0..i].to_string();
        active_routine[0] = active_routine[0].replace(&a_fn, "A");

        for j in 1..21 {
            let s = get_next_start(&active_routine[0]);
            b_fn = active_routine[0][s..s+j].to_string();
            active_routine[1] = active_routine[0].replace(&b_fn, "B");

            for k in 1..21 {
                let s = get_next_start(&active_routine[1]);
                c_fn = active_routine[1][s..s+k].to_string();
                active_routine[2] = active_routine[1].replace(&c_fn, "C");
                if !re.is_match(&active_routine[2]) {
                    println!("A={:?}", a_fn);
                    println!("B={:?}", b_fn);
                    println!("C={:?}", c_fn);
                    println!("{:?}", active_routine[2]);
                    return format!("{}\n{}\n{}\n{}\nn\n", active_routine[2], a_fn, b_fn, c_fn);
                }
            }
        }
    }
    "".to_string()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut computer = Computer::new_from_file(&args[1]);
    let view = get_view(&mut computer);
    println!("{:?}", get_intersections(&view));

    let routines= split_routine(get_move_routine(&view));
    let mut part2computer = Computer::new_from_file(&args[1]);
    part2computer.memwrite(0, 2);
    for c in routines.chars() {
        part2computer.add_input(c as i32);
    }

    display(&mut part2computer);
}
