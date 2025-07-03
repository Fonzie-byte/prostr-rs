use rand::prelude::IndexedRandom;
use std::env;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

#[rustfmt::skip]
const CONSONANTS: [char; 20] = [
    'b', 'c', 'd', 'f', 'g',
    'h', 'j', 'k', 'l', 'm',
    'n', 'p', 'q', 'r', 's',
    't', 'v', 'w', 'x', 'z',
];

fn main() {
    let mut rng = rand::rng();
    let length: u32 = env::args()
        .nth(1)
        .unwrap_or("8".to_string())
        .parse()
        .unwrap_or(8);

    for i in 0..length {
        if i % 2 == 0 {
            print!("{}", CONSONANTS.choose(&mut rng).unwrap());
        } else {
            print!("{}", VOWELS.choose(&mut rng).unwrap());
        };
    }
    println!();
}
