#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub struct Pow2 {
    exponent: u8,
}

impl Pow2 {
    pub fn from_exponent(exponent: u8) -> Pow2 {
        Pow2 { exponent }
    }

    pub fn exponent(&self) -> u8 {
        self.exponent
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pow2_constructible_from_any_u8_exponent() {
        for e in 0_u8..=u8::MAX {
            assert_eq!(Pow2::from_exponent(e).exponent(), e);
        }
    }
}
