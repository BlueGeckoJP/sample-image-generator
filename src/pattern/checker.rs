use crate::pattern::Pattern;

pub struct CheckerPattern {}

impl Pattern for CheckerPattern {
    fn generate(&self, width: u32, height: u32) -> Vec<u8> {
        (0..(width * height)).map(|i| (i % 2) as u8).collect()
    }
}
