use crate::pattern::{Pattern, checker};

mod pattern;

fn main() {
    println!("{:?}", checker::CheckerPattern {}.generate(4, 4));
}
