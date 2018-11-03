mod outline;
pub mod outline_template;
pub mod template_based;

extern crate integral_geometry;
extern crate itertools;
extern crate land2d;

pub struct LandGenerationParameters<T> {
    zero: T,
    basic: T,
    distance_divisor: u32,
}

impl<T: Copy + PartialEq> LandGenerationParameters<T> {
    pub fn new(zero: T, basic: T) -> Self {
        Self {
            zero,
            basic,
            distance_divisor: 1,
        }
    }
}

pub trait LandGenerator {
    fn generate_land<T: Copy + PartialEq, I: Iterator<Item = u32>>(
        &self,
        parameters: LandGenerationParameters<T>,
        random_numbers: &mut I,
    ) -> land2d::Land2D<T>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}