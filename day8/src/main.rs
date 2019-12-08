use std::env;
use std::fs;
use itertools::Itertools;

#[derive(Eq, PartialEq, Clone, Copy)]
enum Color {
    Black,
    White,
    Transparent
}

impl From<u8> for Color {
    fn from(item: u8) -> Self {
        match item {
            0 => Color::Black,
            1 => Color::White,
            2 => Color::Transparent,
            _ => unreachable!()
        }
    }
}

fn parse_input(input: &str) -> Vec<Vec<Color>> {
    input.chars()
        .into_iter()
        .chunks(25*6)
        .into_iter()
        .map(|layer|
            layer
                .into_iter()
                .map(|pixel| Color::from(pixel.to_digit(10).unwrap() as u8))
                .collect()
        )
        .collect()
}
fn check_image(img: &Vec<Vec<Color>>) -> u32 {
    let min_layer = img
        .into_iter()
        .enumerate()
        .map(|(index, layer)| {
            let c = layer.into_iter().filter(|x| **x == Color::Black).count();
            (index, c)
        })
        .min_by(|x, y| x.1.cmp(&y.1))
        .unwrap();

    let ones = img[min_layer.0].clone()
        .into_iter()
        .filter(|x| *x == Color::White)
        .count();
    let twos = img[min_layer.0].clone()
        .into_iter()
        .filter(|x| *x == Color::Transparent)
        .count();

    (ones * twos) as u32
}
fn extract_pixels(img: &Vec<Vec<Color>>, pos: usize) -> Vec<&Color> {
    img
        .into_iter()
        .map(|layer| &layer[pos])
        .collect()
}

fn print_layer(layer: &Vec<Color>, width: usize) -> (){
    let mut i = 0;
    for p in layer {
        if *p == Color::White {
            print!("XX");
        } else {
            print!("  ");
        }
        i += 1;
        if i % width == 0 {
            println!();
        }
    }
}

fn decode(img: &Vec<Vec<Color>>) {
    let new_img: Vec<Color> = (0..(25*6))
        .map(|x|extract_pixels(&img, x))
        .map(|pixels : Vec<&Color>| {
            if let Some(&pixel) = pixels
                .into_iter()
                .skip_while(|&x| *x == Color::Transparent)
                .next()
            {
                pixel
            } else {
                Color::Transparent
            }
        })
        .collect();

    print_layer(&new_img, 25);
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input_filename = args[1].clone();

    let file_contents = fs::read_to_string(&input_filename).unwrap_or_else(|err| {
        eprintln!("Error : {}", err);
        eprintln!("Cannot read from file {}", input_filename);
        std::process::exit(1);
    });

    let layers = parse_input(&file_contents);

    println!("Min layer {}", check_image(&layers));

    decode(&layers);
}
