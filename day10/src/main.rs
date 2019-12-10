use std::fs;
use std::env;
use std::mem::swap;

#[derive(Debug)]
struct Map {
    data: Vec<char>,
    rows: i32,
    columns: i32
}
impl  Map {
    fn is_asteroid(&self, pos: i32) -> bool {
        self.data[pos as usize] == '#'
    }
}

fn get_line(map: &Map, a1: i32, a2: i32) -> (f32, f32){
    let ast1:(i32, i32) = (a1 % map.columns, a1 / map.columns);
    let ast2:(i32, i32) = (a2 % map.columns, a2 / map.columns);

    let a:f32;
    let b:f32;
    if ast2.0 == ast1.0 {
        a = 0.0;
        b = ast1.1 as f32;
    } else {
        a = (ast2.1 - ast1.1) as f32 / (ast2.0 - ast1.0) as f32;
        b = ast1.1 as f32 - (ast1.0 as f32 ) *  a;
    }

    (a, b)
}
fn parse_input(input: &str) -> Map {
    let mut map:Map = Map {
        data: Vec::new(),
        rows: 0,
        columns: 0
    };
    let lines = input.lines().into_iter();
    for l in lines {
        map.rows += 1;
        map.data.extend(l.chars().into_iter())
    }
    map.columns = map.data.len() as i32 / map.rows;

    map
}

fn check_clear_line(map: &Map, p1: i32, p2: i32) -> bool {
    let mut clear = true;
    let mut i = p1;
    let mut j = p2;
    if i > j {
        swap(&mut i, &mut j);
    }

    let x1 = i % map.columns;
    let x2 = j % map.columns;
    let line_eq = get_line(map, i, j);
    if line_eq.0 == 0.0 {
        for y in (i/map.columns+1..j/map.columns) {
            if map.is_asteroid(y * map.columns + x1) {
                clear = false;
                break;
            }
        }
    }
//    println!("{} {} -> {} {} {:?}", p1, p2, x1, x2, line_eq);
    for x in (x1+1..x2) {
        let y = (line_eq.0 * (x as f32) + line_eq.1);
//        println!("\t {} {} {}", x, y, y.fract());
        if y.fract() == 0.0 {
            if map.is_asteroid(y as i32 * map.columns + x) {
                clear = false;
                break;
            }
        }
    }

//    println!("{} {} {}", p1, p2, clear);
    clear
}
fn get_best_position(map: &Map) {
//    let can_see: Vec<usize> = Vec::new();
    let mut max : usize = 0;
    for i in (0..map.rows*map.columns) {
        if !map.is_asteroid(i) {
            continue;
        }
        let mut count = 0;
        for j in (0..map.rows*map.columns) {
            if i == j || !map.is_asteroid(j){
                continue;
            }
            if check_clear_line(map, i, j) {
                count += 1;
            }

        }
        if count > max {
            max = count;
            println!("new max {} at {}, {}", max, i%map.columns, i / map.columns);
        }
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
    let map = parse_input(&file_contents);
    get_best_position(&map);
}
