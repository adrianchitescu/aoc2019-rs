use std::env;
use std::fs;
use std::fmt;

enum Direction {
    Right,
    Left,
    Up,
    Down
}
#[derive(Copy,Clone)]
struct Point {
    x: i32,
    y: i32
}

struct Segment {
    a: Point,
    b: Point,
    horizontal : bool,
    original_order: bool
}
impl Segment {
    fn new(p1: Point, p2: Point) -> Segment {
        if p1.y == p2.y {
            if p1.x < p2.x {
                Segment { a: p1, b: p2, horizontal: true, original_order: true}
            } else {
                Segment { a: p2, b: p1, horizontal: true, original_order: false}
            }
        } else {
            if p1.y < p2.y {
                Segment { a: p1, b: p2, horizontal: false, original_order: true}
            }else {
                Segment { a: p2, b: p1, horizontal: false, original_order: false}
            }
        }
    }

    fn on_x(&self, p: &Point) -> bool {
        self.horizontal && self.a.x <= p.x && p.x <= self.b.x
    }
    fn on_y(&self, p: &Point) -> bool {
        (!self.horizontal) && self.a.y <= p.y && p.y <= self.b.y
    }

    fn intersect(&self, s: &Segment) -> Option<Point> {
        if self.horizontal == s.horizontal {
            None
        } else {
            if self.horizontal {
                if self.on_x(&s.a) && s.on_y(&self.a) {
                    Some(Point {x : s.a.x, y: self.a.y})
                } else {
                    None
                }
            } else {
                if self.on_y(&s.a) && s.on_x(&self.b) {
                    Some(Point { x: self.a.x, y: s.a.y })
                } else {
                    None
                }
            }
        }
    }
    fn length(&self) -> i32 {
        (self.a.x - self.b.x).abs() + (self.a.y - self.b.y).abs()
    }

    fn dist_to(&self, p: &Point) -> Option<i32> {
        if self.on_x(p) {
            if self.a.y == p.y {
                if self.original_order {
                    Some(p.x - self.a.x)
                } else {
                    Some(self.b.x - p.x)
                }
            } else {
                None
            }
        } else {
            if self.on_y(p) {
                if self.a.x == p.x {
                    if self.original_order {
                        Some(p.y - self.a.y)
                    } else {
                        Some(self.b.y - p.y)
                    }
                } else {
                    None
                }
            } else {
                None
            }
        }
    }
}

struct Wire {
    direction : Direction,
    length : i32
}

fn man_dist(p: &Point) -> i32 {
    p.x.abs() + p.y.abs()
}

fn dist_to_intersection(vec: &Vec<Segment>, p : &Point) -> i32 {
    let mut total_dist = 0;
    for s in vec.iter() {
        if let Some(d) = s.dist_to(p) {
            total_dist += d;
            break;
        } else {
            total_dist += s.length();
        }
    }
    total_dist
}

fn parse_input(input: &str) -> Vec<Vec<Wire>> {
    input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|s| Wire {
                    direction : match s.chars().next().unwrap() {
                        'U' => Direction::Up,
                        'R' => Direction::Right,
                        'D' => Direction::Down,
                        'L' => Direction::Left,
                        _   =>  {
                            eprintln!("Invalid input");
                            unreachable!()
                        }
                    },
                    length : s[1..].parse().unwrap()
                })
                .collect()
        })
        .collect()
}

fn generate_segments(wire : &Vec<Wire>) -> Vec<Segment> {
    let mut p: Point = Point { x: 0, y: 0 };
    let mut points = vec![];
    points.push(p);
    for w in wire {
        match w.direction {
            Direction::Left => points.push(Point { x: p.x - w.length, y: p.y}),
            Direction::Right => points.push(Point { x: p.x + w.length, y: p.y}),
            Direction::Up => points.push( Point { x: p.x, y: p.y + w.length}),
            Direction::Down => points.push(Point { x: p.x, y: p.y - w.length})
        }
        p.x = points.last().unwrap().x;
        p.y = points.last().unwrap().y;
    }
    let mut segments = vec![];
    for (p1, p2) in points.iter().zip(points[1..].iter()) {
        segments.push(Segment::new((p1.clone()), p2.clone()));
    }

    segments
}

fn part1(wire1: &Vec<Wire>, wire2: &Vec<Wire>) -> (i32, i32) {
    let seg1 = generate_segments(wire1);
    let seg2 = generate_segments(wire2);

    let mut intersections = vec![];
    let mut dist_inter = vec![];
    for s1 in &seg1 {
        for s2 in &seg2 {
            if let Some(p) = s1.intersect(&s2) {
                if p.x != 0 && p.y != 0 {
                    intersections.push(man_dist(&p));
                    dist_inter.push(
                        dist_to_intersection(&seg1, &p) + dist_to_intersection(&seg2, &p));
                }
            }
        }
    }

    let p1 = *intersections.iter()
        .min_by(|x,y| x.cmp(y)).unwrap_or(&0);
    let p2 = *dist_inter.iter()
        .min_by(|x,y| x.cmp(y)).unwrap_or(&0);

    (p1, p2)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = args[1].clone();

    let file_contents = fs::read_to_string(&input_filename)
        .unwrap_or_else(|err| {
            eprintln!("Error : {}", err);
            eprintln!("Cannot read from file {}", input_filename);
            std::process::exit(1);
        });

    let vec = parse_input(&file_contents);

    println!("The min distance is {:?}", part1(&vec[0], &vec[1]));
}