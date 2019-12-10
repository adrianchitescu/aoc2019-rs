use std::fs;
use std::env;
use itertools::Itertools;
use std::collections::HashMap;

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| (x, y))
        })
        .collect()
}

fn get_best_position(asteroids : &Vec<(usize, usize)>) -> (usize, usize, (usize, usize)) {
    let mut max =  0;
    let mut max_index = 0;
    for i in 0..asteroids.len() {
        let can_see = asteroids
            .iter()
            .filter(|a| **a!= asteroids[i])
            .map(|(x,y)| {
                (*y as f64 - asteroids[i].1 as f64).atan2(*x as f64 - asteroids[i].0 as f64)
            })
            .unique_by(|a|a.to_bits())
            .count();

        if max < can_see {
            max = can_see;
            max_index = i;
        }
    }

    (max, max_index, asteroids[max_index])
}

fn vaporize(ast: &Vec<(usize, usize)>, position: usize, nth: usize) -> (usize, usize){
    let mut asteroids = ast.clone();
    let center = asteroids[position];
    asteroids.remove(position);
    let mut map: HashMap<i32, Vec<((usize, usize), i32)>> =
        asteroids
            .iter()
            .map(|(x,y)| {
                (
                    (((*y as f64 - center.1 as f64).atan2(*x as f64 - center.0 as f64) + std::f64::consts::PI/2.0 ) * 1000.0) as i32,
                    (x,y)
                )
    //                +PI/2 to change reference angle to oY instead of oX
            })
            .fold(HashMap::new(), |mut m, p| {
                let dist_to_center =
                      (*(p.1).0 as i32 - center.0 as i32).abs()
                    + (*(p.1).1 as i32 - center.1 as i32).abs() ;
                m.entry(p.0).or_default().push(((*(p.1).0, *(p.1).1), dist_to_center));
                m
            });
    map
        .iter_mut()
        .for_each(|(_, v)| {
            v.sort_by_key(|(_, dist)| -dist)
        });


    let the_one = map
        .keys()
        .cloned()
        .sorted()
        .cycle()
        .skip_while(|angle| *angle < 0)
        .filter_map(|ref angle| {
            if let Some(a) = map.get_mut(angle) {
                a.pop()
            } else {
                None
            }
        } )
        .take(nth);


    the_one.last().unwrap().0
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

    let best_pos = get_best_position(&map);

    println!("best pos = {:?}", best_pos);
    println!("The 200th asteroid to be vaporized is {:?}",vaporize(&map, best_pos.1, 200));
}
