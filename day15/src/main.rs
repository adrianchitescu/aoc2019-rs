use std::env;
extern crate int_computer;
use int_computer::computer::*;
use itertools::Itertools;
use std::collections::{HashMap, VecDeque};

#[derive(Copy, Clone)]
enum Direction {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

#[derive(PartialEq)]
enum State {
    Processed,
    Seen,
}

fn get_neighbours(current_position: &(i32, i32)) -> Vec<(i32, i32)> {
    vec![
        (current_position.0, current_position.1 + 1),
        (current_position.0, current_position.1 - 1),
        (current_position.0 + 1, current_position.1),
        (current_position.0 - 1, current_position.1),
    ]
}

fn get_commands(current_position: &(i32, i32)) -> Vec<(Direction, (i32, i32))> {
    vec![
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]
    .into_iter()
    .zip(get_neighbours(current_position).into_iter())
    .collect()
}

fn discover(
    computer: &mut Computer,
    map: &mut HashMap<(i32, i32), i32>,
    position: (i32, i32),
    parent_direction: Option<Direction>,
) {
    for (direction, new_position) in get_commands(&position) {
        if !map.contains_key(&new_position) {
            computer.add_input(direction as i32);
            computer.run();
            let out = computer.get_output().unwrap();
            map.insert(new_position, out as i32);
            if out != 0 {
                discover(computer, map, new_position, Some(direction));
            }
        }
    }

    if let Some(back_dir) = match parent_direction {
        None => None,
        Some(Direction::North) => Some(Direction::South),
        Some(Direction::South) => Some(Direction::North),
        Some(Direction::West) => Some(Direction::East),
        Some(Direction::East) => Some(Direction::West),
    } {
        computer.add_input(back_dir as i32);
        computer.run();
        let _ = computer.get_output();
    }
}

fn map_2_matrix(map: &HashMap<(i32, i32), i32>, start_position: &mut (i32, i32)) -> Vec<Vec<i32>> {
    let ((minx, _), (maxx, _)) = map
        .keys()
        .into_iter()
        .minmax_by_key(|(x, _)| x)
        .into_option()
        .unwrap();
    let ((_, miny), (_, maxy)) = map
        .keys()
        .into_iter()
        .minmax_by_key(|(_, y)| y)
        .into_option()
        .unwrap();
    let width = (maxx - minx + 1) as usize;
    let heigth = (maxy - miny + 1) as usize;
    let mut matrix: Vec<Vec<i32>> = Vec::new();
    for _ in 0..width {
        matrix.push(vec![0; heigth]);
    }

    for (k, v) in map {
        let x = (k.0 - minx) as usize;
        let y = (k.1 - miny) as usize;
        matrix[x][y] = *v;
    }

    start_position.0 -= *minx;
    start_position.1 -= *miny;

    matrix
}

fn search_oxygen(map: Vec<Vec<i32>>, start_position: (i32, i32)) -> (i32, i32) {
    let mut q: VecDeque<((i32, i32), usize)> = VecDeque::new();
    q.push_front((start_position, 0));
    let mut visited: HashMap<(i32, i32), State> = HashMap::new();
    visited.insert(start_position, State::Seen);
    let mut oxygen_position = (-1, -1);

    while !q.is_empty() {
        let (p, dist) = q.pop_front().unwrap();
        let s = visited.get_mut(&p).unwrap();
        if *s == State::Processed {
            continue;
        }
        if map[p.0 as usize][p.1 as usize] == 2 {
            println!("Found it, at distance {}", dist);
            oxygen_position = p;
            break;
        }
        *s = State::Processed;
        let valid_pos: Vec<(i32, i32)> = get_neighbours(&p)
            .into_iter()
            .filter(|new_p| {
                !visited.contains_key(new_p) && map[new_p.0 as usize][new_p.1 as usize] != 0
            })
            .collect();
        for new_p in valid_pos {
            visited.insert(new_p, State::Seen);
            q.push_back((new_p, dist + 1));
        }
    }

    oxygen_position
}

fn fill_map(map: &mut Vec<Vec<i32>>, start_position: (i32, i32)) {
    let mut q: VecDeque<((i32, i32), usize)> = VecDeque::new();
    q.push_front((start_position, 0));
    let mut max = 0;

    while !q.is_empty() {
        let (p, dist) = q.pop_front().unwrap();
        if dist > max {
            max = dist;
        }
        let valid_pos: Vec<(i32, i32)> = get_neighbours(&p)
            .into_iter()
            .filter(|new_p| map[new_p.0 as usize][new_p.1 as usize] == 1)
            .collect();
        for new_p in valid_pos {
            map[p.0 as usize][p.1 as usize] = 2;
            q.push_back((new_p, dist + 1));
        }
    }

    println!("Minutes : {}", max);
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut computer = Computer::new_from_file(&args[1]);
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();
    let mut start_position = (25, 25);
    discover(&mut computer, &mut map, start_position, None);

    let mut mmap = map_2_matrix(&map, &mut start_position);
    let ox_position = search_oxygen(mmap.clone(), start_position);
    fill_map(&mut mmap, ox_position);
}
