use std::fs;
use std::env;
use itertools::Itertools;
use std::ops::IndexMut;
use std::borrow::BorrowMut;

#[derive(Copy, Clone, Debug)]
struct Moon {
    x: i32,
    y: i32,
    z: i32,
    vx: i32,
    vy: i32,
    vz: i32
}
impl Moon {
    fn new(coordinates: &str) -> Moon {
        let c:Vec<i32> = coordinates
            .split_terminator(',')
            .into_iter()
            .map(|n| {
                if let Ok(nn) = n.parse::<i32>() {
                    nn
                } else {
                    0
                }
            })
            .collect();

        Moon { x: c[0], y: c[1], z : c[2],
            vx: 0, vy: 0, vz: 0 }
    }

    fn potential_energy(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    fn kinetic_energy(&self) -> i32 {
        self.vx.abs() + self.vy.abs() + self.vz.abs()
    }

    fn total_energy(&self) -> i32 {
        self.potential_energy() * self.kinetic_energy()
    }

    fn apply_gravity(&mut self, m: &mut Moon) {
        let  axis_gravity = |&c1, &c2, v1: &mut i32, v2: &mut i32| {
            if c1 < c2 {
                *v1 += 1;
                *v2 -= 1;
            } else if  c1 > c2 {
                *v2 += 1;
                *v1 -= 1;
            }
        };

        axis_gravity(&self.x, &m.x, &mut self.vx, &mut m.vx);
        axis_gravity(&self.y, &m.y, &mut self.vy, &mut m.vy);
        axis_gravity(&self.z, &m.z, &mut self.vz, &mut m.vz);
    }

    fn apply_velocity(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
        self.z += self.vz;
    }

}
fn parse_input(input: &str) -> Vec<Moon> {
    input
        .lines()
        .map(Moon::new)
        .collect()
}

fn get_energy(m: &Vec<Moon>, steps: usize) -> i32{
    let mut moons = m.clone();
    let combinations= (0..moons.len()).combinations(2);
    println!("{:?}", combinations);
    for _ in 0..steps {
//        println!("{:?}", moons);

        for c in (0..moons.len()).combinations(2) {
            let (a,b) = moons.split_at_mut(c[1]);
            a[c[0]].apply_gravity(&mut b[0]);

        }
        moons.iter_mut().for_each(|m: &mut Moon| m.apply_velocity());

    }

    moons.iter().map(|m| m.total_energy()).sum()

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = args[1].clone();

    let file_contents = fs::read_to_string(&input_filename).unwrap_or_else(|err| {
        eprintln!("Error : {}", err);
        eprintln!("Cannot read from file {}", input_filename);
        std::process::exit(1);
    });
    let moons = parse_input(&file_contents);
    println!("total energy after 100 = {}", get_energy(&moons, 1000));
}
