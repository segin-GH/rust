fn main() {
    let age: i32 = 1;

    match age {
        0 => println!("I'm not born yet I guess"),
        1..=12 => println!("I'm a child"),
        13..=19 => println!("I'm a teenager"),
        20..=30 => println!("I'm a young adult"),
        _ => println!("I'm an adult"),
    };

    const VOTING_AGE: i32 = 18;
    match age.cmp(&VOTING_AGE) {
        std::cmp::Ordering::Less => println!("I'm not old enough to vote"),
        std::cmp::Ordering::Equal => println!("I'm old enough to vote"),
        std::cmp::Ordering::Greater => println!("I'm old enough to vote and then some"),
    };
}
