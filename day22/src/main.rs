use std::env;
use std::fs;
use std::mem::swap;
#[macro_use] extern crate scan_fmt;

struct Deck {
    deck: Vec<usize>,
    temp_deck: Vec<usize>
}
impl Deck {
    fn new(s: Vec<usize>) -> Deck {
        Deck {
            deck: s.clone(),
            temp_deck: Vec::with_capacity(s.len())
        }
    }

    fn deal_into_new_stack(&mut self){
        let mid = self.deck.len() / 2;
        let ( first,  second) = self.deck.split_at_mut(mid);
        let slen = second.len() - 1;
        for i in 0..mid {
            swap(&mut first[i], &mut second[slen - i]);
        }
    }
    fn cut(&mut self, n : i32) {
        let mut to_skip = n;
        if n < 0 {
            to_skip += self.deck.len() as i32;
        }

        self.temp_deck = self.deck
            .iter()
            .cycle()
            .skip(to_skip as usize)
            .map(|x| *x)
            .take(self.deck.len())
            .collect();

        self.deck = self.temp_deck.drain(..).collect();
    }

    fn deal_with_increment(&mut self, n: i32) {
        self.temp_deck = self.deck.iter().cloned().collect();
        let mut pos: usize = 0;
        for (_, v) in self.temp_deck.iter().enumerate() {
            self.deck[pos as usize] = *v;
            pos += n as usize;
            pos %= self.deck.len();
        }
    }

    fn get_position_for(&self, card: usize) -> usize{
        self.deck.iter().position(|x| *x == card).unwrap()
    }
}


fn main() {
    let mut deck = Deck::new((0..10007).into_iter().collect());
    let args: Vec<String> = env::args().collect();
    let input_filename = args[1].clone();
    let file_contents = fs::read_to_string(&input_filename).unwrap_or_else(|err| {
        eprintln!("Error : {}", err);
        eprintln!("Cannot read from file {}", input_filename);
        std::process::exit(1);
    });

    file_contents
        .lines()
        .for_each(|line| {
            if line.starts_with("deal into new stack") {
                deck.deal_into_new_stack();
            } else if line.starts_with("cut") {
                if let Ok(n) = scan_fmt!(line, "cut {d}", i32) {
                    deck.cut(n);
                }
            } else if line.starts_with("deal with") {
                if let Ok(n) = scan_fmt!(line, "deal with increment {d}", i32) {
                    deck.deal_with_increment(n);
                }
            } else {
                println!("invalid input");
            }
        });

    println!("{:?}", deck.get_position_for(2019));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_deal_into_new_even() {
        let mut deck = Deck::new((0..10).into_iter().collect());
        deck.deal_into_new_stack();
        assert_eq!(deck.deck, [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn test_deal_into_new_odd() {
        let mut deck = Deck::new((0..9).into_iter().collect());
        deck.deal_into_new_stack();
        assert_eq!(deck.deck, [8, 7, 6, 5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn test_cut_positive() {
        let mut deck = Deck::new((0..10).into_iter().collect());
        deck.cut(3);
        // ;
        assert_eq!(deck.deck, [3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
    }

    #[test]
    fn test_cut_negative() {
        let mut deck = Deck::new((0..10).into_iter().collect());
        deck.cut(-4);
        assert_eq!(deck.deck, [6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_deal_with_increment() {
        let mut deck = Deck::new((0..10).into_iter().collect());
        deck.deal_with_increment(3);
        assert_eq!(deck.deck, [0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
    }
}