use crate::private::Int;

mod private;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub struct Pow2 {
    exponent: u8,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct NotPow2;

impl Pow2 {
    pub fn from_exponent(exponent: u8) -> Pow2 {
        Pow2 { exponent }
    }

    pub fn exponent(&self) -> u8 {
        self.exponent
    }
}

macro_rules! impl_as {
    ($name:ident, $t:ty) => {
        impl Pow2 {
            pub fn $name(self) -> $t {
                1 << self.exponent
            }
        }
    };
}

impl_as!(as_u8, u8);
impl_as!(as_u16, u16);
impl_as!(as_u32, u32);
impl_as!(as_u64, u64);
impl_as!(as_u128, u128);
impl_as!(as_usize, usize);

impl_as!(as_i8, i8);
impl_as!(as_i16, i16);
impl_as!(as_i32, i32);
impl_as!(as_i64, i64);
impl_as!(as_i128, i128);
impl_as!(as_isize, isize);

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
        assert_eq!(Pow2::try_from(32_isize), Ok(Pow2::from_exponent(5)));
        assert_eq!(Pow2::try_from(32_usize), Ok(Pow2::from_exponent(5)));
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

    #[test]
    fn as_int_all_types() {
        let p = Pow2::from_exponent(6);
        assert_eq!(p.as_i8(), 64);
        assert_eq!(p.as_u8(), 64);
        assert_eq!(p.as_i16(), 64);
        assert_eq!(p.as_u16(), 64);
        assert_eq!(p.as_i32(), 64);
        assert_eq!(p.as_u32(), 64);
        assert_eq!(p.as_i64(), 64);
        assert_eq!(p.as_u64(), 64);
        assert_eq!(p.as_i128(), 64);
        assert_eq!(p.as_u128(), 64);
        assert_eq!(p.as_isize(), 64);
        assert_eq!(p.as_usize(), 64);
    }

    #[test]
    fn as_int_max_fitting() {
        assert_eq!(Pow2::from_exponent(6).as_i8(), 1 << 6);
        assert_eq!(Pow2::from_exponent(7).as_u8(), 1 << 7);
        assert_eq!(Pow2::from_exponent(14).as_i16(), 1 << 14);
        assert_eq!(Pow2::from_exponent(15).as_u16(), 1 << 15);
        assert_eq!(Pow2::from_exponent(30).as_i32(), 1 << 30);
        assert_eq!(Pow2::from_exponent(31).as_u32(), 1 << 31);
        assert_eq!(Pow2::from_exponent(62).as_i64(), 1 << 62);
        assert_eq!(Pow2::from_exponent(63).as_u64(), 1 << 63);
        assert_eq!(Pow2::from_exponent(126).as_i128(), 1 << 126);
        assert_eq!(Pow2::from_exponent(127).as_u128(), 1 << 127);
    }
}
