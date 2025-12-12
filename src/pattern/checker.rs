use crate::pattern::Pattern;

pub struct CheckerPattern {}

impl Pattern for CheckerPattern {
    fn generate(&self, width: u32, height: u32) -> Vec<u8> {
        (0..(width * height))
            .map(|i| {
                let bit = (i % 2) as u8;
                let invert = ((i / width) & 1) == 0;
                if invert { 1 - bit } else { bit }
            })
            .collect()
    }
}
