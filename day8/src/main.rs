use std::env;
use std::fs;
use itertools::Itertools;

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input.chars()
        .into_iter()
        .chunks(25*6)
        .into_iter()
        .map(|layer|
            layer
                .into_iter()
                .map(|pixel| pixel.to_digit(10).unwrap())
                .collect()
        )
        .collect()
}
fn check_image(img: &Vec<Vec<u32>>) -> u32 {
    let min_layer = img
        .into_iter()
        .enumerate()
        .map(|(index, layer)| {
            let c = layer.into_iter().filter(|x| *x == &0).count();
            (index, c)
        })
        .min_by(|x, y| x.1.cmp(&y.1))
        .unwrap();

    let ones = img[min_layer.0].clone()
        .into_iter()
        .filter(|x| *x == 1)
        .count();
    let twos = img[min_layer.0].clone()
        .into_iter()
        .filter(|x| *x == 2)
        .count();

    (ones * twos) as u32
}
fn extract_pixels(img: &Vec<Vec<u32>>, pos: usize) -> Vec<&u32> {
    img
        .into_iter()
        .map(|layer| &layer[pos])
        .collect()
}
fn print_layer(layer: &Vec<u32>, width: usize) -> (){
    let mut i = 0;
    for p in layer {
        print!("{}", p);
        i += 1;
        if i % width == 0 {
            println!();
        }
    }
}

fn decode(img: &Vec<Vec<u32>>) {
    let new_img: Vec<u32> = (0..(25*6))
        .map(|x|extract_pixels(&img, x))
        .map(|pixels : Vec<&u32>| {
            let mut winning_pixel: u32 = 2;
            for p in pixels {
                if *p < 2 {
                    winning_pixel = *p;
                    break;
                }
            }
            winning_pixel
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
