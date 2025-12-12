pub mod checker;

pub trait Pattern {
    fn generate(&self, width: u32, height: u32) -> Vec<u8>;
}
