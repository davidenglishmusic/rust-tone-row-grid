extern crate rand;
use rand::{thread_rng, Rng};

fn main() {
    let default_row: [(u8, String); 12] = [
        (0, String::from("c")),
        (1, String::from("c#")),
        (2, String::from("d")),
        (3, String::from("eb")),
        (4, String::from("e")),
        (5, String::from("f")),
        (6, String::from("f#")),
        (7, String::from("g")),
        (8, String::from("ab")),
        (9, String::from("a")),
        (10, String::from("bb")),
        (11, String::from("b"))
    ];

    let new_row: [(u8, String); 12] = generate_random_row(default_row);

    for (pitch_number, pitch_letter) in &new_row {
        println!("{}:{}", pitch_number, pitch_letter);
    }
}

fn generate_random_row(default_row: [(u8, String); 12]) -> [(u8, String); 12] {
    let mut new_row = default_row;
    thread_rng().shuffle(&mut new_row);
    return new_row;
}
