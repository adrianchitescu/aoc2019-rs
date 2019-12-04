fn parse_input(input: &str) -> (i32, i32) {
    let ranges: Vec<i32>= input
        .split("-")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    (ranges[0], ranges[1])
}

fn is_valid(pass:i32) -> (i8, i8) {
    let pass_str = pass.to_string();
    let digits: Vec<u32>= pass_str
        .chars()
        .map(|d|d.to_digit(10).unwrap())
        .collect();
    let mut part1 = false;
    let mut part2 = false;
    for (d1, d2) in digits.iter().zip(digits[1..].iter()) {
        if d1 == d2 {
            part2 |= pass_str.matches(&d1.to_string()).count() == 2;
            part1 = true;
        } else {
            if d1 > d2 {
                return (0, 0)
            }
        }
    }

    (part1 as i8, part2 as i8)
}

fn main() {
    let (start, end) = parse_input(("387638-919123"));

    let result = (start .. end).fold((0,0), |acc:(i32, i32), pass| {
        let valid = is_valid(pass);
        (acc.0 + valid.0 as i32, acc.1 + valid.1 as i32)
    });

    println!("Total number of passwords {:?}", result);
}