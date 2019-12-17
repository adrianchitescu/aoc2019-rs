use std::env;
extern crate int_computer;
use int_computer::computer::*;
use std::collections::HashMap;
use itertools::Itertools;
use itertools::MinMaxResult::{NoElements, OneElement, MinMax};

#[derive(Copy, Clone)]
enum Direction {
    North = 1,
    South = 2,
    West  = 3,
    East  = 4
}

// struct Matrix<T> {
//     vec : Vec<T>
// }
// impl Index<Matrix> {
//     fn index(&self, index: Side) -> &Self::Output {
//         println!("Accessing {:?}-side of balance immutably", index);
//         match index {
//             Side::Left => &self.left,
//             Side::Right => &self.right,
//         }
//     }
// }

fn get_commands(current_position: &(i32, i32)) ->  Vec<(Direction, (i32, i32))> {
    vec![
        (Direction::North, (current_position.0, current_position.1 + 1)),
        (Direction::South, (current_position.0, current_position.1 - 1)),
        (Direction::West,  (current_position.0 + 1, current_position.1)),
        (Direction::East,  (current_position.0 - 1, current_position.1))]
}

fn discover(computer: &mut Computer, map: &mut HashMap<(i32, i32), i32>, position: (i32, i32), parent_direction: Option<Direction>) {
    for (direction, new_position) in get_commands(&position) {
        if !map.contains_key(&new_position) {
            computer.add_input(direction as i32);
            computer.run();
            let out = computer.get_output().unwrap();
            map.insert(new_position, out as i32);
            if out != 0 {
                discover(computer,map, new_position, Some(direction));
            }
        }
    }


    if let Some(back_dir) = match parent_direction {
        None => None,
        Some(Direction::North) => Some(Direction::South),
        Some(Direction::South) => Some(Direction::North),
        Some(Direction::West) =>  Some(Direction::East),
        Some(Direction::East) =>  Some(Direction::West)
    } {
        computer.add_input(back_dir as i32);
        computer.run();
        let _ = computer.get_output();
    }
}

fn map_2_matrix(map: &HashMap<(i32, i32), i32>)  {
    let ((minx, _), (maxx, _)) = map.keys().into_iter().minmax_by_key(|(x,y)| x).into_option().unwrap();
    let ((_, miny), (_, maxy)) = map.keys().into_iter().minmax_by_key(|(x,y)| y).into_option().unwrap();
    let width  = (maxx - minx + 1) as usize;
    let heigth = (maxy - miny + 1) as usize;
    let mut matrix:Vec<Vec<i32>> = Vec::new();
    for _ in 0..width {
        matrix.push(vec![0; heigth]);
    }

    for (k,v) in map {
        let x = (k.0 - minx) as usize;
        let y = (k.1 - miny) as usize;
        matrix[x][y] = *v;
    }

    // matrix[19][19] = 3;
    // for l in matrix {
    //     for x in l {
    //         print!("{}", x);
    //     }
    //     println!();
    // }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut computer = Computer::new_from_file(&args[1]);
    let mut map: HashMap<(i32, i32), i32> = HashMap::new();
    discover(&mut computer,&mut map, (25,25), None);

    println!("{:?}", map);
    map_2_matrix(&map);
}
