use eas::mutation::{Bitflip, Mutation};
use eas::search_space::Bitstring;

fn main() {
    for _ in 0..10000 {
        let bitstring = Bitstring::from_bitstring("00000000000000000000").unwrap();
        let mutated = Bitflip.apply(&bitstring, &mut rand::rng());
        let ones = mutated
            .bits()
            .iter()
            .fold(0, |acc, &c| if c { acc + 1 } else { acc });
        println!("{}", ones);
    }
}
