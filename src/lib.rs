use crate::private::Int;

mod private;

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

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct NotPow2;

macro_rules! impl_try_from {
    ($t:ty) => {
        impl TryFrom<$t> for Pow2 {
            type Error = NotPow2;

            fn try_from(value: $t) -> Result<Self, Self::Error> {
                if !value.is_power_of_two() {
                    Err(NotPow2)
                } else {
                    Ok(Pow2::from_exponent(value.ilog2() as u8))
                }
            }
        }
    };
}

impl_try_from!(u8);
impl_try_from!(u16);
impl_try_from!(u32);
impl_try_from!(u64);
impl_try_from!(u128);
impl_try_from!(usize);

impl_try_from!(i8);
impl_try_from!(i16);
impl_try_from!(i32);
impl_try_from!(i64);
impl_try_from!(i128);
impl_try_from!(isize);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pow2_constructible_from_any_u8_exponent() {
        for e in 0_u8..=u8::MAX {
            assert_eq!(Pow2::from_exponent(e).exponent(), e);
        }
    }

    #[test]
    fn try_from_power_of_two_int() {
        assert_eq!(Pow2::try_from(32_i8), Ok(Pow2::from_exponent(5)));
        assert_eq!(Pow2::try_from(32_u8), Ok(Pow2::from_exponent(5)));
        assert_eq!(Pow2::try_from(32_i16), Ok(Pow2::from_exponent(5)));
        assert_eq!(Pow2::try_from(32_u16), Ok(Pow2::from_exponent(5)));
        assert_eq!(Pow2::try_from(32_i32), Ok(Pow2::from_exponent(5)));
        assert_eq!(Pow2::try_from(32_u32), Ok(Pow2::from_exponent(5)));
        assert_eq!(Pow2::try_from(32_i64), Ok(Pow2::from_exponent(5)));
        assert_eq!(Pow2::try_from(32_u64), Ok(Pow2::from_exponent(5)));
        assert_eq!(Pow2::try_from(32_i128), Ok(Pow2::from_exponent(5)));
        assert_eq!(Pow2::try_from(32_u128), Ok(Pow2::from_exponent(5)));
    }

    #[test]
    fn try_from_near_power_of_two() {
        assert_eq!(Pow2::try_from(31_i8), Err(NotPow2));
        assert_eq!(Pow2::try_from(33_i8), Err(NotPow2));
    }

    #[test]
    fn try_from_zero() {
        assert_eq!(Pow2::try_from(0_i8), Err(NotPow2));
    }

    #[test]
    fn try_from_negative() {
        assert_eq!(Pow2::try_from(-1_i8), Err(NotPow2));
    }

    #[test]
    fn try_from_min() {
        assert_eq!(Pow2::try_from(i64::MIN), Err(NotPow2));
    }

    #[test]
    fn try_from_max() {
        assert_eq!(Pow2::try_from(i64::MAX), Err(NotPow2));
    }
}
