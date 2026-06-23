use crate::private::Int;
use std::ops::{Div, DivAssign, Mul, MulAssign};

#[macro_use]
mod private;

#[repr(transparent)]
#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub struct Pow2 {
    exponent: u8,
}

const _: () = assert!(size_of::<Pow2>() == size_of::<u8>());
const _: () = assert!(align_of::<Pow2>() == align_of::<u8>());

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct NotPow2;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Pow2OutOfRange;

impl Pow2 {
    pub const VAL_1: Pow2 = Pow2::from_exponent(0);
    pub const VAL_2: Pow2 = Pow2::from_exponent(1);
    pub const VAL_4: Pow2 = Pow2::from_exponent(2);
    pub const VAL_8: Pow2 = Pow2::from_exponent(3);
    pub const VAL_16: Pow2 = Pow2::from_exponent(4);
    pub const VAL_32: Pow2 = Pow2::from_exponent(5);
    pub const VAL_64: Pow2 = Pow2::from_exponent(6);
    pub const VAL_128: Pow2 = Pow2::from_exponent(7);
    pub const VAL_256: Pow2 = Pow2::from_exponent(8);
    pub const VAL_512: Pow2 = Pow2::from_exponent(9);

    pub const KIBI: Pow2 = Pow2::from_exponent(10);
    pub const MEBI: Pow2 = Pow2::from_exponent(20);
    pub const GIBI: Pow2 = Pow2::from_exponent(30);
    pub const TEBI: Pow2 = Pow2::from_exponent(40);
    pub const PEBI: Pow2 = Pow2::from_exponent(50);
    pub const EXBI: Pow2 = Pow2::from_exponent(60);
    pub const ZEBI: Pow2 = Pow2::from_exponent(70);
    pub const YOBI: Pow2 = Pow2::from_exponent(80);
    pub const ROBI: Pow2 = Pow2::from_exponent(90);
    pub const QUEBI: Pow2 = Pow2::from_exponent(100);

    #[inline(always)]
    pub const fn from_exponent(exponent: u8) -> Pow2 {
        Pow2 { exponent }
    }

    #[inline(always)]
    pub fn align_of<T>() -> Pow2 {
        Pow2 {
            exponent: align_of::<T>().ilog2() as u8,
        }
    }

    #[inline(always)]
    pub fn align_of_val<T: ?Sized>(val: &T) -> Pow2 {
        Pow2 {
            exponent: align_of_val(val).ilog2() as u8,
        }
    }

    #[inline(always)]
    pub fn exponent(self) -> u8 {
        self.exponent
    }

    #[inline(always)]
    pub fn is_safe<T: Int>(self) -> bool {
        self.exponent as u32 <= T::safe_shift_bits()
    }

    #[inline(always)]
    pub fn checked_mul(self, other: Pow2) -> Option<Pow2> {
        Some(Pow2::from_exponent(
            self.exponent.checked_add(other.exponent)?,
        ))
    }

    #[inline(always)]
    pub fn saturating_mul(self, other: Pow2) -> Pow2 {
        Pow2::from_exponent(self.exponent.saturating_add(other.exponent))
    }

    #[inline(always)]
    pub fn checked_div(self, other: Pow2) -> Option<Pow2> {
        Some(Pow2::from_exponent(
            self.exponent.checked_sub(other.exponent)?,
        ))
    }

    #[inline(always)]
    pub fn saturating_div(self, other: Pow2) -> Pow2 {
        Pow2::from_exponent(self.exponent.saturating_sub(other.exponent))
    }
}

macro_rules! impl_as {
    ($name:ident, $t:ty) => {
        impl Pow2 {
            #[inline(always)]
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

            #[inline(always)]
            fn try_from(value: $t) -> Result<Self, Self::Error> {
                if !value.is_power_of_two() {
                    Err(NotPow2)
                } else {
                    Ok(Pow2::from_exponent(value.ilog2() as u8))
                }
            }
        }

        impl TryFrom<Pow2> for $t {
            type Error = Pow2OutOfRange;

            #[inline(always)]
            fn try_from(value: Pow2) -> Result<Self, Self::Error> {
                if value.is_safe::<$t>() {
                    Ok(1 << value.exponent)
                } else {
                    Err(Pow2OutOfRange)
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

impl Mul for Pow2 {
    type Output = Pow2;

    #[allow(clippy::suspicious_arithmetic_impl)]
    #[inline(always)]
    fn mul(self, other: Pow2) -> Pow2 {
        Pow2::from_exponent(self.exponent + other.exponent)
    }
}

impl MulAssign for Pow2 {
    #[allow(clippy::suspicious_arithmetic_impl)]
    #[inline(always)]
    fn mul_assign(&mut self, other: Pow2) {
        self.exponent += other.exponent;
    }
}

impl Div for Pow2 {
    type Output = Pow2;
    #[inline(always)]
    fn div(self, other: Pow2) -> Pow2 {
        Pow2::from_exponent(self.exponent - other.exponent)
    }
}

impl DivAssign for Pow2 {
    #[allow(clippy::suspicious_arithmetic_impl)]
    #[inline(always)]
    fn div_assign(&mut self, other: Pow2) {
        self.exponent -= other.exponent;
    }
}

macro_rules! impl_div_signed {
    ($t:ty) => {
        impl Div<Pow2> for $t {
            type Output = $t;

            #[inline(always)]
            fn div(self, other: Pow2) -> $t {
                debug_assert!(other.is_safe::<<$t as Int>::Unsigned>());
                if self >= 0 {
                    self >> other.exponent
                } else {
                    let mask = <$t as Int>::mask(other.exponent as u32);
                    (self + mask) >> other.exponent
                }
            }
        }
    };
}

macro_rules! impl_div_unsigned {
    ($t:ty) => {
        impl Div<Pow2> for $t {
            type Output = $t;

            #[inline(always)]
            fn div(self, other: Pow2) -> $t {
                debug_assert!(other.is_safe::<<$t as Int>::Unsigned>());
                self >> other.exponent
            }
        }
    };
}

impl_for_signed!(impl_div_signed);
impl_for_unsigned!(impl_div_unsigned);

macro_rules! impl_div_assign {
    ($t:ty) => {
        impl DivAssign<Pow2> for $t {
            #[inline(always)]
            fn div_assign(&mut self, other: Pow2) {
                debug_assert!(other.is_safe::<<$t as Int>::Unsigned>());
                *self = *self / other;
            }
        }
    };
}

impl_for_signed!(impl_div_assign);
impl_for_unsigned!(impl_div_assign);

macro_rules! impl_mul {
    ($t:ty) => {
        impl Mul<Pow2> for $t {
            type Output = $t;

            #[inline(always)]
            fn mul(self, other: Pow2) -> $t {
                debug_assert!(other.is_safe::<<$t as Int>::Unsigned>());
                // Check for overflow.
                debug_assert!(self == self << other.exponent >> other.exponent);
                self << other.exponent
            }
        }

        impl MulAssign<Pow2> for $t {
            #[inline(always)]
            fn mul_assign(&mut self, other: Pow2) {
                *self = *self * other;
            }
        }
    };
}

impl_for_signed!(impl_mul);
impl_for_unsigned!(impl_mul);

#[inline(always)]
pub fn checked_mul<T: Int>(lhs: T, rhs: Pow2) -> Option<T> {
    let result = lhs.checked_shl(rhs.exponent as u32)?;
    if lhs == result >> rhs.exponent {
        Some(result)
    } else {
        None
    }
}

#[inline(always)]
pub fn checked_div<T>(lhs: T, rhs: Pow2) -> Option<T>
where
    T: Int + Div<Pow2, Output = T>,
{
    rhs.is_safe::<T::Unsigned>().then(|| lhs / rhs)
}

#[inline(always)]
pub fn unbounded_div<T>(lhs: T, rhs: Pow2) -> T
where
    T: Int + Div<Pow2, Output = T>,
{
    if rhs.is_safe::<T::Unsigned>() {
        lhs / rhs
    } else {
        T::zero()
    }
}

#[inline(always)]
pub fn div_floor<T: Int>(lhs: T, rhs: Pow2) -> T {
    debug_assert!(rhs.is_safe::<T::Unsigned>());
    lhs >> rhs.exponent
}

#[inline(always)]
pub fn checked_div_floor<T: Int>(lhs: T, rhs: Pow2) -> Option<T> {
    rhs.is_safe::<T::Unsigned>().then(|| div_floor(lhs, rhs))
}

#[inline(always)]
pub fn unbounded_div_floor<T: Int>(lhs: T, rhs: Pow2) -> T {
    if rhs.is_safe::<T::Unsigned>() {
        div_floor(lhs, rhs)
    } else if T::is_unsigned() || lhs >= T::zero() {
        T::zero()
    } else {
        T::minus_one()
    }
}

#[inline(always)]
pub fn mod_floor<T: Int>(lhs: T, rhs: Pow2) -> T {
    debug_assert!(rhs.is_safe::<T::Unsigned>());
    lhs - floor_to_multiple(lhs, rhs)
}

#[inline(always)]
pub fn checked_mod_floor<T: Int>(lhs: T, rhs: Pow2) -> Option<T> {
    rhs.is_safe::<T::Unsigned>().then(|| mod_floor(lhs, rhs))
}

#[inline(always)]
pub fn div_ceil<T: Int>(lhs: T, rhs: Pow2) -> T {
    debug_assert!(rhs.is_safe::<T::Unsigned>());
    // Can't use a faster implementation with a mask due to possible overflow
    // of the intermediate `a + mask`
    let floored = div_floor(lhs, rhs);
    let rem = lhs - (floored << rhs.exponent);
    floored + T::from_bool(rem.is_not_zero())
}

#[inline(always)]
pub fn checked_div_ceil<T: Int>(lhs: T, rhs: Pow2) -> Option<T> {
    rhs.is_safe::<T::Unsigned>().then(|| div_ceil(lhs, rhs))
}

#[inline(always)]
pub fn unbounded_div_ceil<T: Int>(lhs: T, rhs: Pow2) -> T {
    if rhs.is_safe::<T::Unsigned>() {
        div_ceil(lhs, rhs)
    } else if lhs <= T::zero() {
        T::zero()
    } else {
        T::one()
    }
}

#[inline(always)]
pub fn is_multiple_of<T: Int>(lhs: T, rhs: Pow2) -> bool {
    debug_assert!(rhs.is_safe::<T::Unsigned>());
    floor_to_multiple(lhs, rhs) == lhs
}

#[inline(always)]
pub fn unbounded_is_multiple_of<T: Int>(lhs: T, rhs: Pow2) -> bool {
    if !rhs.is_safe::<T::Unsigned>() {
        lhs.is_zero()
    } else {
        // This actually works for signed integers being shifted by `BITS-1` too.
        // It's important, because we need to check for `Int::MIN`
        lhs >> rhs.exponent << rhs.exponent == lhs
    }
}

#[inline(always)]
pub fn floor_to_multiple<T: Int>(lhs: T, rhs: Pow2) -> T {
    debug_assert!(rhs.is_safe::<T::Unsigned>());
    lhs >> rhs.exponent << rhs.exponent
}

#[inline(always)]
pub fn checked_floor_to_multiple<T: Int>(lhs: T, rhs: Pow2) -> Option<T> {
    rhs.is_safe::<T::Unsigned>()
        .then(|| floor_to_multiple(lhs, rhs))
}

impl_signed_unsigned_trait!(
    IntForUnboundedFloorToMultiple,
    trait_body {
        type ResultType;

        fn unbounded_floor_to_multiple(lhs: Self, rhs: Pow2) -> Self::ResultType;
    },
    signed_body {
        type ResultType = Option<Self>;

        fn unbounded_floor_to_multiple(lhs: Self, rhs: Pow2) -> Self::ResultType {
            debug_assert!(Self::is_signed());
            if rhs.is_safe::<Self::Unsigned>() {
                Some(floor_to_multiple(lhs, rhs))
            } else if lhs >= Self::zero() {
                Some(Self::zero())
            } else {
                None
            }
        }
    },
    unsigned_body {
        type ResultType = Self;

        fn unbounded_floor_to_multiple(lhs: Self, rhs: Pow2) -> Self::ResultType {
            debug_assert!(Self::is_unsigned());
            if rhs.is_safe::<Self>() {
                floor_to_multiple(lhs, rhs)
            } else {
                Self::zero()
            }
        }
    }
);

#[inline(always)]
#[allow(private_interfaces)]
#[allow(private_bounds)]
pub fn unbounded_floor_to_multiple<T: IntForUnboundedFloorToMultiple>(
    lhs: T,
    rhs: Pow2,
) -> T::ResultType {
    T::unbounded_floor_to_multiple(lhs, rhs)
}

#[inline(always)]
pub fn ceil_to_multiple<T: Int>(lhs: T, rhs: Pow2) -> T {
    debug_assert!(rhs.is_safe::<T::Unsigned>());
    // We can actually use the mask method here because if the intermediate `a + mask` overflows
    // then the actual result would overflow too.
    let mask = T::mask(rhs.exponent as u32);
    (lhs + mask) & !mask
}

#[inline(always)]
pub fn checked_ceil_to_multiple<T: Int>(lhs: T, rhs: Pow2) -> Option<T> {
    if rhs.is_safe::<T::Unsigned>() {
        let mask = T::mask(rhs.exponent as u32);
        Some(lhs.checked_add(mask)? & !mask)
    } else {
        None
    }
}

#[inline(always)]
pub fn unbounded_ceil_to_multiple<T: Int>(lhs: T, rhs: Pow2) -> Option<T> {
    if rhs.is_safe::<T::Unsigned>() {
        checked_ceil_to_multiple(lhs, rhs)
    } else if lhs <= T::zero() {
        Some(T::zero())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Write;
    use std::hash::{DefaultHasher, Hash, Hasher};

    #[test]
    fn pow2_constructible_from_any_u8_exponent() {
        for e in 0_u8..=u8::MAX {
            assert_eq!(Pow2::from_exponent(e).exponent(), e);
        }
    }

    #[test]
    fn pow2_constructible_from_align_of() {
        assert_eq!(Pow2::align_of::<()>(), Pow2::from_exponent(0));
        assert_eq!(Pow2::align_of::<u8>(), Pow2::from_exponent(0));
        assert_eq!(Pow2::align_of::<u32>(), Pow2::from_exponent(2));
        assert_eq!(Pow2::align_of::<u128>(), Pow2::from_exponent(4));
    }

    #[test]
    fn pow2_constructible_from_align_of_val() {
        assert_eq!(Pow2::align_of_val(&()), Pow2::from_exponent(0));
        assert_eq!(Pow2::align_of_val(&0_u8), Pow2::from_exponent(0));
        assert_eq!(Pow2::align_of_val(&0_u32), Pow2::from_exponent(2));
        assert_eq!(Pow2::align_of_val(&0_u128), Pow2::from_exponent(4));
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

    #[test]
    fn pow2_to_int() {
        assert_eq!(i8::try_from(Pow2::from_exponent(6)), Ok(1 << 6));
        assert_eq!(i8::try_from(Pow2::from_exponent(7)), Err(Pow2OutOfRange));

        assert_eq!(u8::try_from(Pow2::from_exponent(7)), Ok(1 << 7));
        assert_eq!(u8::try_from(Pow2::from_exponent(8)), Err(Pow2OutOfRange));

        assert_eq!(i16::try_from(Pow2::from_exponent(14)), Ok(1 << 14));
        assert_eq!(i16::try_from(Pow2::from_exponent(15)), Err(Pow2OutOfRange));

        assert_eq!(u16::try_from(Pow2::from_exponent(15)), Ok(1 << 15));
        assert_eq!(u16::try_from(Pow2::from_exponent(16)), Err(Pow2OutOfRange));

        assert_eq!(i32::try_from(Pow2::from_exponent(30)), Ok(1 << 30));
        assert_eq!(i32::try_from(Pow2::from_exponent(31)), Err(Pow2OutOfRange));

        assert_eq!(u32::try_from(Pow2::from_exponent(31)), Ok(1 << 31));
        assert_eq!(u32::try_from(Pow2::from_exponent(32)), Err(Pow2OutOfRange));

        assert_eq!(i64::try_from(Pow2::from_exponent(62)), Ok(1 << 62));
        assert_eq!(i64::try_from(Pow2::from_exponent(63)), Err(Pow2OutOfRange));

        assert_eq!(u64::try_from(Pow2::from_exponent(63)), Ok(1 << 63));
        assert_eq!(u64::try_from(Pow2::from_exponent(64)), Err(Pow2OutOfRange));

        assert_eq!(i128::try_from(Pow2::from_exponent(126)), Ok(1 << 126));
        assert_eq!(
            i128::try_from(Pow2::from_exponent(127)),
            Err(Pow2OutOfRange)
        );

        assert_eq!(u128::try_from(Pow2::from_exponent(127)), Ok(1 << 127));
        assert_eq!(
            u128::try_from(Pow2::from_exponent(128)),
            Err(Pow2OutOfRange)
        );
    }

    #[test]
    fn pow2_mul() {
        let lhs = Pow2::from_exponent(6);
        let rhs = Pow2::from_exponent(7);
        assert_eq!(lhs * rhs, Pow2::from_exponent(13));
        assert_eq!(rhs * lhs, Pow2::from_exponent(13));
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn pow2_mul_overflow() {
        let lhs = Pow2::from_exponent(128);
        let rhs = Pow2::from_exponent(128);
        let _ = lhs * rhs;
    }

    #[test]
    fn pow2_mul_assign() {
        let lhs = Pow2::from_exponent(6);
        let rhs = Pow2::from_exponent(7);
        let mut new_lhs = lhs;
        new_lhs *= rhs;
        assert_eq!(new_lhs, Pow2::from_exponent(13));

        let mut new_rhs = rhs;
        new_rhs *= lhs;
        assert_eq!(new_rhs, Pow2::from_exponent(13));
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn pow2_mul_assign_overflow() {
        let mut lhs = Pow2::from_exponent(128);
        let rhs = Pow2::from_exponent(128);
        lhs *= rhs;
    }

    #[test]
    fn pow2_mul_zero() {
        let lhs = Pow2::from_exponent(65);
        let rhs = Pow2::from_exponent(0);
        assert_eq!(lhs * rhs, lhs);
        assert_eq!(rhs * lhs, lhs);
    }

    #[test]
    fn pow2_mul_assign_zero() {
        let lhs = Pow2::from_exponent(65);
        let rhs = Pow2::from_exponent(0);
        let mut new_lhs = lhs;
        new_lhs *= rhs;
        assert_eq!(new_lhs, lhs);

        let mut new_rhs = rhs;
        new_rhs *= lhs;
        assert_eq!(new_rhs, lhs);
    }

    #[test]
    fn pow2_checked_mul() {
        assert_eq!(
            Pow2::from_exponent(12).checked_mul(Pow2::from_exponent(13)),
            Some(Pow2::from_exponent(25))
        );
        assert_eq!(
            Pow2::from_exponent(12).checked_mul(Pow2::from_exponent(0)),
            Some(Pow2::from_exponent(12))
        );
        assert_eq!(
            Pow2::from_exponent(0).checked_mul(Pow2::from_exponent(13)),
            Some(Pow2::from_exponent(13))
        );
    }

    #[test]
    fn pow2_checked_mul_boundary() {
        assert_eq!(
            Pow2::from_exponent(254).checked_mul(Pow2::from_exponent(1)),
            Some(Pow2::from_exponent(255))
        );
        assert_eq!(
            Pow2::from_exponent(254).checked_mul(Pow2::from_exponent(2)),
            None
        );
    }

    #[test]
    fn pow2_saturating_mul() {
        assert_eq!(
            Pow2::from_exponent(13).saturating_mul(Pow2::from_exponent(14)),
            Pow2::from_exponent(27)
        );
        assert_eq!(
            Pow2::from_exponent(13).saturating_mul(Pow2::from_exponent(0)),
            Pow2::from_exponent(13)
        );
        assert_eq!(
            Pow2::from_exponent(0).saturating_mul(Pow2::from_exponent(14)),
            Pow2::from_exponent(14)
        );
    }

    #[test]
    fn pow2_saturating_mul_boundary() {
        assert_eq!(
            Pow2::from_exponent(254).saturating_mul(Pow2::from_exponent(1)),
            Pow2::from_exponent(255)
        );
        assert_eq!(
            Pow2::from_exponent(254).saturating_mul(Pow2::from_exponent(2)),
            Pow2::from_exponent(255)
        );
    }

    #[test]
    fn pow2_div_positive() {
        let lhs = Pow2::from_exponent(6);
        let rhs = Pow2::from_exponent(3);
        assert_eq!(lhs / rhs, Pow2::from_exponent(3));
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn pow2_div_overflow() {
        let lhs = Pow2::from_exponent(127);
        let rhs = Pow2::from_exponent(128);
        let _ = lhs / rhs;
    }

    #[test]
    fn pow2_div_assign_positive() {
        let mut lhs = Pow2::from_exponent(6);
        let rhs = Pow2::from_exponent(3);
        lhs /= rhs;
        assert_eq!(lhs, Pow2::from_exponent(3));
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn pow2_div_assign_overflow() {
        let mut lhs = Pow2::from_exponent(127);
        let rhs = Pow2::from_exponent(128);
        lhs /= rhs;
    }

    #[test]
    fn pow2_checked_div_overflow() {
        let lhs = Pow2::from_exponent(3);
        let rhs = Pow2::from_exponent(6);
        assert_eq!(lhs.checked_div(rhs), None);
        assert_eq!(rhs.checked_div(lhs), Some(Pow2::from_exponent(3)));
    }

    #[test]
    fn pow2_saturating_div_overflow() {
        let lhs = Pow2::from_exponent(3);
        let rhs = Pow2::from_exponent(6);
        assert_eq!(lhs.saturating_div(rhs), Pow2::from_exponent(0));
        assert_eq!(rhs.saturating_div(lhs), Pow2::from_exponent(3));
    }

    #[test]
    fn pow2_div_one() {
        let lhs = Pow2::from_exponent(3);
        let rhs = Pow2::from_exponent(0);
        assert_eq!(lhs / rhs, lhs);
    }

    #[test]
    fn pow2_div_assign_one() {
        let lhs = Pow2::from_exponent(3);
        let rhs = Pow2::from_exponent(0);
        let mut new_lhs = lhs;
        new_lhs /= rhs;
        assert_eq!(new_lhs, lhs);
    }

    #[test]
    fn pow2_div_self() {
        let v = Pow2::from_exponent(123);
        assert_eq!(v / v, Pow2::from_exponent(0));
    }

    #[test]
    fn pow2_div_assign_self() {
        let mut v = Pow2::from_exponent(123);
        v /= v;
        assert_eq!(v, Pow2::from_exponent(0));
    }

    #[test]
    fn mul_one() {
        assert_eq!(123 * Pow2::from_exponent(0), 123);
        assert_eq!(-123 * Pow2::from_exponent(0), -123);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn mul_overflow() {
        let _ = 123_i32 * Pow2::from_exponent(27);
    }

    #[test]
    fn mul_assign_one() {
        let mut v = 123;
        v *= Pow2::from_exponent(0);
        assert_eq!(v, 123);

        let mut v = -123;
        v *= Pow2::from_exponent(0);
        assert_eq!(v, -123);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn mul_assign_overflow() {
        let mut lhs = 123_i32;
        lhs *= Pow2::from_exponent(27);
    }

    #[test]
    fn mul() {
        assert_eq!(123_i32 * Pow2::from_exponent(15), 123 << 15);
        assert_eq!(-123_i32 * Pow2::from_exponent(15), -123 << 15);
        assert_eq!(123_i64 * Pow2::from_exponent(32), 123 << 32);
        assert_eq!(-123_i64 * Pow2::from_exponent(32), -123 << 32);
        assert_eq!(123_u64 * Pow2::from_exponent(46), 123 << 46);
    }

    #[test]
    fn mul_assign() {
        let mut v = 123_i32;
        v *= Pow2::from_exponent(15);
        assert_eq!(v, 123 << 15);

        let mut v = -123_i32;
        v *= Pow2::from_exponent(15);
        assert_eq!(v, -123 << 15);

        let mut v = 123_i64;
        v *= Pow2::from_exponent(32);
        assert_eq!(v, 123 << 32);

        let mut v = -123_i64;
        v *= Pow2::from_exponent(32);
        assert_eq!(v, -123 << 32);

        let mut v = 123_u64;
        v *= Pow2::from_exponent(46);
        assert_eq!(v, 123 << 46);
    }

    #[test]
    fn checked_mul_boundary() {
        assert_eq!(
            checked_mul(i32::MAX as u32 + 1, Pow2::from_exponent(1)),
            None
        );
        assert_eq!(
            checked_mul(i32::MAX as u32, Pow2::from_exponent(1)),
            Some(u32::MAX - 1)
        );
    }

    #[test]
    fn div_exact() {
        assert_eq!(32u64 / Pow2::from_exponent(5), 1);
        assert_eq!(64u64 / Pow2::from_exponent(3), 8);
        assert_eq!(-64i64 / Pow2::from_exponent(3), -8);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn div_divisor_out_of_range() {
        let _ = 123_i32 / Pow2::from_exponent(32);
    }

    #[test]
    fn div_assign_exact() {
        let mut v = 32u64;
        v /= Pow2::from_exponent(5);
        assert_eq!(v, 1);

        let mut v = 64u64;
        v /= Pow2::from_exponent(3);
        assert_eq!(v, 8);

        let mut v = -64i64;
        v /= Pow2::from_exponent(3);
        assert_eq!(v, -8);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn div_assign_divisor_out_of_range() {
        let _ = 123_i32 / Pow2::from_exponent(32);
    }

    #[test]
    fn div_rounds_towards_zero() {
        assert_eq!(37u64 / Pow2::from_exponent(5), 1);
        assert_eq!(63u64 / Pow2::from_exponent(5), 1);
        assert_eq!(-37i64 / Pow2::from_exponent(5), -1);
        assert_eq!(-1i64 / Pow2::from_exponent(5), 0);
    }

    #[test]
    fn div_assign_rounds_towards_zero() {
        let mut v = 37u64;
        v /= Pow2::from_exponent(5);
        assert_eq!(v, 1);

        let mut v = 63u64;
        v /= Pow2::from_exponent(5);
        assert_eq!(v, 1);

        let mut v = -37i64;
        v /= Pow2::from_exponent(5);
        assert_eq!(v, -1);

        let mut v = -1i64;
        v /= Pow2::from_exponent(5);
        assert_eq!(v, 0);
    }

    #[test]
    fn div_by_one_is_identity() {
        assert_eq!(42i64 / Pow2::from_exponent(0), 42);
        assert_eq!(-42i64 / Pow2::from_exponent(0), -42);
    }

    #[test]
    fn div_assign_by_one_is_identity() {
        let mut v = 42i64;
        v /= Pow2::from_exponent(0);
        assert_eq!(v, 42);

        let mut v = -42i64;
        v /= Pow2::from_exponent(0);
        assert_eq!(v, -42);
    }

    #[test]
    fn div_min() {
        assert_eq!(i32::MIN / Pow2::from_exponent(30), -2);
        assert_eq!(i32::MIN / Pow2::from_exponent(31), -1);
    }

    #[test]
    fn div_assign_min() {
        let mut v = i32::MIN;
        v /= Pow2::from_exponent(30);
        assert_eq!(v, -2);

        let mut v = i32::MIN;
        v /= Pow2::from_exponent(31);
        assert_eq!(v, -1);
    }

    #[test]
    fn div_max() {
        assert_eq!(i32::MAX / Pow2::from_exponent(30), 1);
        assert_eq!(i32::MAX / Pow2::from_exponent(31), 0);
        assert_eq!(u32::MAX / Pow2::from_exponent(31), 1);
    }

    #[test]
    fn div_assign_max() {
        let mut v = i32::MAX;
        v /= Pow2::from_exponent(30);
        assert_eq!(v, 1);

        let mut v = i32::MAX;
        v /= Pow2::from_exponent(31);
        assert_eq!(v, 0);

        let mut v = u32::MAX;
        v /= Pow2::from_exponent(31);
        assert_eq!(v, 1);
    }

    #[test]
    fn checked_div_boundary() {
        assert_eq!(checked_div(i32::MAX, Pow2::from_exponent(30)), Some(1));
        assert_eq!(checked_div(i32::MIN, Pow2::from_exponent(30)), Some(-2));
        assert_eq!(checked_div(123_i32, Pow2::from_exponent(31)), Some(0));
        assert_eq!(checked_div(-123_i32, Pow2::from_exponent(31)), Some(0));
        assert_eq!(checked_div(123_i32, Pow2::from_exponent(32)), None);
        assert_eq!(checked_div(-123_i32, Pow2::from_exponent(32)), None);
    }

    #[test]
    fn unbounded_div_boundary() {
        assert_eq!(unbounded_div(i32::MAX, Pow2::from_exponent(30)), 1);
        assert_eq!(unbounded_div(i32::MIN, Pow2::from_exponent(30)), -2);
        assert_eq!(unbounded_div(123_i32, Pow2::from_exponent(31)), 0);
        assert_eq!(unbounded_div(-123_i32, Pow2::from_exponent(31)), 0);
        assert_eq!(unbounded_div(123_i32, Pow2::from_exponent(32)), 0);
        assert_eq!(unbounded_div(-123_i32, Pow2::from_exponent(32)), 0);
    }

    #[test]
    fn div_floor_u64_exact() {
        assert_eq!(div_floor(32u64, Pow2::from_exponent(5)), 1);
        assert_eq!(div_floor(64u64, Pow2::from_exponent(3)), 8);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn div_floor_divisor_out_of_range() {
        let _ = div_floor(32u64, Pow2::from_exponent(64));
    }

    #[test]
    fn div_floor_u64_rounds_down() {
        assert_eq!(div_floor(37u64, Pow2::from_exponent(5)), 1);
        assert_eq!(div_floor(63u64, Pow2::from_exponent(5)), 1);
    }

    #[test]
    fn div_floor_u64_zero() {
        assert_eq!(div_floor(0u64, Pow2::from_exponent(5)), 0);
    }

    #[test]
    fn div_floor_i64_positive() {
        assert_eq!(div_floor(37i64, Pow2::from_exponent(5)), 1);
    }

    #[test]
    fn div_floor_i64_exact_negative() {
        assert_eq!(div_floor(-32i64, Pow2::from_exponent(5)), -1);
    }

    #[test]
    fn div_floor_i64_negative_rounds_toward_neg_inf() {
        assert_eq!(div_floor(-37i64, Pow2::from_exponent(5)), -2);
        assert_eq!(div_floor(-1i64, Pow2::from_exponent(5)), -1);
    }

    #[test]
    fn div_floor_by_one_is_identity() {
        assert_eq!(div_floor(42i64, Pow2::from_exponent(0)), 42);
        assert_eq!(div_floor(-42i64, Pow2::from_exponent(0)), -42);
    }

    #[test]
    fn div_floor_min() {
        assert_eq!(div_floor(i32::MIN, Pow2::from_exponent(30)), -2);
        assert_eq!(div_floor(i32::MIN, Pow2::from_exponent(31)), -1);
    }

    #[test]
    fn div_floor_max() {
        assert_eq!(div_floor(i32::MAX, Pow2::from_exponent(30)), 1);
        assert_eq!(div_floor(i32::MAX, Pow2::from_exponent(31)), 0);
        assert_eq!(div_floor(u32::MAX, Pow2::from_exponent(31)), 1);
    }

    #[test]
    fn checked_div_floor_boundary() {
        assert_eq!(checked_div_floor(0_i32, Pow2::from_exponent(30)), Some(0));
        assert_eq!(checked_div_floor(0_i32, Pow2::from_exponent(31)), Some(0));
        assert_eq!(checked_div_floor(0_i32, Pow2::from_exponent(32)), None);

        assert_eq!(checked_div_floor(0_u32, Pow2::from_exponent(31)), Some(0));
        assert_eq!(checked_div_floor(0_u32, Pow2::from_exponent(32)), None);
    }

    #[test]
    fn unbounded_div_floor_boundary() {
        assert_eq!(unbounded_div_floor(0_i32, Pow2::from_exponent(30)), 0);
        assert_eq!(unbounded_div_floor(i32::MIN, Pow2::from_exponent(30)), -2);
        assert_eq!(unbounded_div_floor(0_i32, Pow2::from_exponent(31)), 0);
        assert_eq!(unbounded_div_floor(-1_i32, Pow2::from_exponent(31)), -1);
        assert_eq!(unbounded_div_floor(i32::MIN, Pow2::from_exponent(31)), -1);
        assert_eq!(unbounded_div_floor(0_i32, Pow2::from_exponent(32)), 0);
        assert_eq!(unbounded_div_floor(-1_i32, Pow2::from_exponent(32)), -1);
        assert_eq!(unbounded_div_floor(0_i32, Pow2::from_exponent(255)), 0);
        assert_eq!(unbounded_div_floor(i32::MAX, Pow2::from_exponent(255)), 0);
        assert_eq!(unbounded_div_floor(i32::MIN, Pow2::from_exponent(255)), -1);

        assert_eq!(unbounded_div_floor(0_u32, Pow2::from_exponent(31)), 0);
        assert_eq!(unbounded_div_floor(u32::MAX, Pow2::from_exponent(31)), 1);
        assert_eq!(unbounded_div_floor(0_u32, Pow2::from_exponent(32)), 0);
        assert_eq!(unbounded_div_floor(u32::MAX, Pow2::from_exponent(32)), 0);
        assert_eq!(unbounded_div_floor(0_u32, Pow2::from_exponent(255)), 0);
        assert_eq!(unbounded_div_floor(u32::MAX, Pow2::from_exponent(255)), 0);
    }

    #[test]
    fn div_ceil_u64_exact() {
        assert_eq!(div_ceil(32u64, Pow2::from_exponent(5)), 1);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn div_ceil_divisor_out_of_range() {
        let _ = div_ceil(32u64, Pow2::from_exponent(64));
    }

    #[test]
    fn div_ceil_u64_rounds_up() {
        assert_eq!(div_ceil(33u64, Pow2::from_exponent(5)), 2);
        assert_eq!(div_ceil(63u64, Pow2::from_exponent(5)), 2);
    }

    #[test]
    fn div_ceil_u64_zero() {
        assert_eq!(div_ceil(0u64, Pow2::from_exponent(5)), 0);
    }

    #[test]
    fn div_ceil_i64_negative() {
        assert_eq!(div_ceil(-37i64, Pow2::from_exponent(5)), -1);
        assert_eq!(div_ceil(-32i64, Pow2::from_exponent(5)), -1);
        assert_eq!(div_ceil(-1i64, Pow2::from_exponent(5)), 0);
        assert_eq!(div_ceil(-33i64, Pow2::from_exponent(5)), -1);
    }

    #[test]
    fn div_ceil_by_one_is_identity() {
        assert_eq!(div_ceil(42i64, Pow2::from_exponent(0)), 42);
        assert_eq!(div_ceil(-42i64, Pow2::from_exponent(0)), -42);
    }

    #[test]
    fn div_ceil_min() {
        assert_eq!(div_ceil(i32::MIN, Pow2::from_exponent(30)), -2);
        assert_eq!(div_ceil(i32::MIN, Pow2::from_exponent(31)), -1);
    }

    #[test]
    fn div_ceil_max() {
        assert_eq!(div_ceil(i32::MAX, Pow2::from_exponent(30)), 2);
        assert_eq!(div_ceil(i32::MAX, Pow2::from_exponent(31)), 1);
        assert_eq!(div_ceil(u32::MAX, Pow2::from_exponent(31)), 2);
    }

    #[test]
    fn checked_div_ceil_boundary() {
        assert_eq!(checked_div_ceil(0_i32, Pow2::from_exponent(30)), Some(0));
        assert_eq!(checked_div_ceil(0_i32, Pow2::from_exponent(31)), Some(0));
        assert_eq!(checked_div_ceil(0_i32, Pow2::from_exponent(32)), None);

        assert_eq!(checked_div_ceil(0_u32, Pow2::from_exponent(31)), Some(0));
        assert_eq!(checked_div_ceil(0_u32, Pow2::from_exponent(32)), None);
    }

    #[test]
    fn unbounded_div_ceil_boundary() {
        assert_eq!(unbounded_div_ceil(0_i32, Pow2::from_exponent(30)), 0);
        assert_eq!(unbounded_div_ceil(i32::MAX, Pow2::from_exponent(30)), 2);
        assert_eq!(unbounded_div_ceil(0_i32, Pow2::from_exponent(31)), 0);
        assert_eq!(unbounded_div_ceil(i32::MIN, Pow2::from_exponent(31)), -1);
        assert_eq!(unbounded_div_ceil(-1_i32, Pow2::from_exponent(32)), 0);
        assert_eq!(unbounded_div_ceil(0_i32, Pow2::from_exponent(32)), 0);
        assert_eq!(unbounded_div_ceil(1_i32, Pow2::from_exponent(32)), 1);
        assert_eq!(unbounded_div_ceil(i32::MAX, Pow2::from_exponent(32)), 1);
        assert_eq!(unbounded_div_ceil(-1_i32, Pow2::from_exponent(255)), 0);
        assert_eq!(unbounded_div_ceil(i32::MIN, Pow2::from_exponent(255)), 0);
        assert_eq!(unbounded_div_ceil(i32::MAX, Pow2::from_exponent(255)), 1);

        assert_eq!(unbounded_div_ceil(0_u32, Pow2::from_exponent(31)), 0);
        assert_eq!(unbounded_div_ceil(0_u32, Pow2::from_exponent(32)), 0);
        assert_eq!(unbounded_div_ceil(0_u32, Pow2::from_exponent(255)), 0);
        assert_eq!(unbounded_div_ceil(u32::MAX, Pow2::from_exponent(255)), 1);
    }

    #[test]
    fn floor_to_multiple_u64_already_aligned() {
        assert_eq!(floor_to_multiple(64u64, Pow2::from_exponent(6)), 64);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn floor_to_multiple_align_out_of_range() {
        let _ = floor_to_multiple(64u64, Pow2::from_exponent(64));
    }

    #[test]
    fn floor_to_multiple_u64_unaligned() {
        assert_eq!(floor_to_multiple(65u64, Pow2::from_exponent(6)), 64);
        assert_eq!(floor_to_multiple(127u64, Pow2::from_exponent(6)), 64);
    }

    #[test]
    fn floor_to_multiple_i64_positive() {
        assert_eq!(floor_to_multiple(37i64, Pow2::from_exponent(5)), 32);
    }

    #[test]
    fn floor_to_multiple_i64_negative() {
        assert_eq!(floor_to_multiple(-37i64, Pow2::from_exponent(5)), -64);
        assert_eq!(floor_to_multiple(-32i64, Pow2::from_exponent(5)), -32);
    }

    #[test]
    fn floor_to_multiple_by_one_is_identity() {
        assert_eq!(floor_to_multiple(37i64, Pow2::from_exponent(0)), 37);
        assert_eq!(floor_to_multiple(-37i64, Pow2::from_exponent(0)), -37);
    }

    #[test]
    fn floor_to_multiple_min() {
        assert_eq!(
            floor_to_multiple(i32::MIN, Pow2::from_exponent(15)),
            i32::MIN
        );
        assert_eq!(
            floor_to_multiple(i32::MIN, Pow2::from_exponent(30)),
            i32::MIN
        );
        assert_eq!(
            floor_to_multiple(i32::MIN, Pow2::from_exponent(31)),
            i32::MIN
        );
    }

    #[test]
    fn floor_to_multiple_max() {
        assert_eq!(
            floor_to_multiple(i32::MAX, Pow2::from_exponent(30)),
            1 << 30
        );
        assert_eq!(floor_to_multiple(i32::MAX, Pow2::from_exponent(31)), 0);
        assert_eq!(
            floor_to_multiple(u32::MAX, Pow2::from_exponent(31)),
            1 << 31
        );
    }

    #[test]
    fn checked_floor_to_multiple_boundary() {
        assert_eq!(
            checked_floor_to_multiple(0_i32, Pow2::from_exponent(30)),
            Some(0)
        );
        assert_eq!(
            checked_floor_to_multiple(0_i32, Pow2::from_exponent(31)),
            Some(0)
        );
        assert_eq!(
            checked_floor_to_multiple(0_i32, Pow2::from_exponent(32)),
            None
        );

        assert_eq!(
            checked_floor_to_multiple(0_u32, Pow2::from_exponent(31)),
            Some(0)
        );
        assert_eq!(
            checked_floor_to_multiple(0_u32, Pow2::from_exponent(32)),
            None
        );
    }

    #[test]
    fn unbounded_floor_to_multiple_boundary() {
        assert_eq!(
            unbounded_floor_to_multiple(0_i32, Pow2::from_exponent(30)),
            Some(0)
        );
        assert_eq!(
            unbounded_floor_to_multiple(0_i32, Pow2::from_exponent(31)),
            Some(0)
        );
        assert_eq!(
            unbounded_floor_to_multiple(-1_i32, Pow2::from_exponent(31)),
            Some(i32::MIN)
        );
        assert_eq!(
            unbounded_floor_to_multiple(0_i32, Pow2::from_exponent(32)),
            Some(0)
        );
        assert_eq!(
            unbounded_floor_to_multiple(-1_i32, Pow2::from_exponent(32)),
            None
        );
        assert_eq!(
            unbounded_floor_to_multiple(1_i32, Pow2::from_exponent(32)),
            Some(0)
        );
        assert_eq!(
            unbounded_floor_to_multiple(0_i32, Pow2::from_exponent(255)),
            Some(0)
        );

        assert_eq!(
            unbounded_floor_to_multiple(0_u32, Pow2::from_exponent(31)),
            0
        );
        assert_eq!(
            unbounded_floor_to_multiple(0_u32, Pow2::from_exponent(32)),
            0
        );
        assert_eq!(
            unbounded_floor_to_multiple(0_u32, Pow2::from_exponent(255)),
            0
        );
    }

    #[test]
    fn ceil_to_multiple_u64_already_aligned() {
        assert_eq!(ceil_to_multiple(64u64, Pow2::from_exponent(6)), 64);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn ceil_to_multiple_align_out_of_range() {
        let _ = ceil_to_multiple(64u64, Pow2::from_exponent(64));
    }

    #[test]
    fn ceil_to_multiple_u64_unaligned() {
        assert_eq!(ceil_to_multiple(65u64, Pow2::from_exponent(6)), 128);
        assert_eq!(ceil_to_multiple(1u64, Pow2::from_exponent(6)), 64);
    }

    #[test]
    fn ceil_to_multiple_u64_zero() {
        assert_eq!(ceil_to_multiple(0u64, Pow2::from_exponent(5)), 0);
    }

    #[test]
    fn ceil_to_multiple_i64_positive() {
        assert_eq!(ceil_to_multiple(33i64, Pow2::from_exponent(5)), 64);
        assert_eq!(ceil_to_multiple(32i64, Pow2::from_exponent(5)), 32);
    }

    #[test]
    fn ceil_to_multiple_i64_negative() {
        assert_eq!(ceil_to_multiple(-33i64, Pow2::from_exponent(5)), -32);
        assert_eq!(ceil_to_multiple(-32i64, Pow2::from_exponent(5)), -32);
        assert_eq!(ceil_to_multiple(-16i64, Pow2::from_exponent(5)), 0);
    }

    #[test]
    fn ceil_to_multiple_min() {
        assert_eq!(ceil_to_multiple(i32::MIN, Pow2::from_exponent(2)), i32::MIN);
        assert_eq!(
            ceil_to_multiple(i32::MIN, Pow2::from_exponent(30)),
            i32::MIN
        );
    }

    #[test]
    fn ceil_to_multiple_max() {
        assert_eq!(ceil_to_multiple(i32::MAX, Pow2::from_exponent(0)), i32::MAX);
        assert_eq!(
            ceil_to_multiple(i32::MAX - 1, Pow2::from_exponent(1)),
            i32::MAX - 1
        );

        assert_eq!(ceil_to_multiple(u32::MAX, Pow2::from_exponent(0)), u32::MAX);
        assert_eq!(
            ceil_to_multiple(u32::MAX - 1, Pow2::from_exponent(1)),
            u32::MAX - 1
        );
    }

    #[test]
    fn checked_ceil_to_multiple_boundary() {
        assert_eq!(
            checked_ceil_to_multiple(0_i32, Pow2::from_exponent(30)),
            Some(0)
        );
        assert_eq!(
            checked_ceil_to_multiple(0_i32, Pow2::from_exponent(31)),
            Some(0)
        );
        assert_eq!(
            checked_ceil_to_multiple(i32::MAX, Pow2::from_exponent(0)),
            Some(i32::MAX)
        );
        assert_eq!(
            checked_ceil_to_multiple(i32::MAX, Pow2::from_exponent(1)),
            None
        );

        assert_eq!(
            checked_ceil_to_multiple(0_u32, Pow2::from_exponent(31)),
            Some(0)
        );
        assert_eq!(
            checked_ceil_to_multiple(i32::MAX as u32 + 2, Pow2::from_exponent(31)),
            None
        );
        assert_eq!(
            checked_ceil_to_multiple(0_u32, Pow2::from_exponent(32)),
            None
        );
    }

    #[test]
    fn unbounded_ceil_to_multiple_boundary() {
        assert_eq!(
            unbounded_ceil_to_multiple(0_i32, Pow2::from_exponent(30)),
            Some(0)
        );
        assert_eq!(
            unbounded_ceil_to_multiple(-1_i32, Pow2::from_exponent(31)),
            Some(0)
        );
        assert_eq!(
            unbounded_ceil_to_multiple(i32::MIN, Pow2::from_exponent(31)),
            Some(i32::MIN)
        );
        assert_eq!(
            unbounded_ceil_to_multiple(0_i32, Pow2::from_exponent(31)),
            Some(0)
        );
        assert_eq!(
            unbounded_ceil_to_multiple(1_i32, Pow2::from_exponent(31)),
            None
        );
        assert_eq!(
            unbounded_ceil_to_multiple(-1_i32, Pow2::from_exponent(255)),
            Some(0)
        );
        assert_eq!(
            unbounded_ceil_to_multiple(0_i32, Pow2::from_exponent(255)),
            Some(0)
        );
        assert_eq!(
            unbounded_ceil_to_multiple(1_i32, Pow2::from_exponent(255)),
            None
        );
        assert_eq!(
            unbounded_ceil_to_multiple(i32::MIN, Pow2::from_exponent(255)),
            Some(0)
        );
        assert_eq!(
            unbounded_ceil_to_multiple(i32::MAX, Pow2::from_exponent(0)),
            Some(i32::MAX)
        );
        assert_eq!(
            unbounded_ceil_to_multiple(i32::MAX, Pow2::from_exponent(1)),
            None
        );

        assert_eq!(
            unbounded_ceil_to_multiple(0_u32, Pow2::from_exponent(31)),
            Some(0)
        );
        assert_eq!(
            unbounded_ceil_to_multiple(1_u32, Pow2::from_exponent(31)),
            Some(1 << 31)
        );
        assert_eq!(
            unbounded_ceil_to_multiple(0_u32, Pow2::from_exponent(32)),
            Some(0)
        );
        assert_eq!(
            unbounded_ceil_to_multiple(1_u32, Pow2::from_exponent(32)),
            None
        );
        assert_eq!(
            unbounded_ceil_to_multiple(0_u32, Pow2::from_exponent(255)),
            Some(0)
        );
        assert_eq!(
            unbounded_ceil_to_multiple(1_u32, Pow2::from_exponent(255)),
            None
        );
        assert_eq!(
            unbounded_ceil_to_multiple(i32::MAX as u32 + 2, Pow2::from_exponent(31)),
            None
        );
    }

    #[test]
    fn mod_floor_u64_exact() {
        assert_eq!(mod_floor(32u64, Pow2::from_exponent(5)), 0);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn mod_floor_divisor_out_of_range() {
        let _ = mod_floor(32u64, Pow2::from_exponent(64));
    }

    #[test]
    fn mod_floor_u64_remainder() {
        assert_eq!(mod_floor(37u64, Pow2::from_exponent(5)), 5);
        assert_eq!(mod_floor(63u64, Pow2::from_exponent(5)), 31);
    }

    #[test]
    fn mod_floor_i64_positive() {
        assert_eq!(mod_floor(37i64, Pow2::from_exponent(5)), 5);
    }

    #[test]
    fn mod_floor_i64_negative() {
        assert_eq!(mod_floor(-1i64, Pow2::from_exponent(5)), 31);
    }

    #[test]
    fn mod_floor_min() {
        assert_eq!(mod_floor(i32::MIN, Pow2::from_exponent(2)), 0);
        assert_eq!(mod_floor(i32::MIN, Pow2::from_exponent(31)), 0);
    }

    #[test]
    fn mod_floor_max() {
        assert_eq!(mod_floor(i32::MAX, Pow2::from_exponent(2)), 3);
        assert_eq!(mod_floor(i32::MAX, Pow2::from_exponent(31)), i32::MAX);
        assert_eq!(mod_floor(u32::MAX, Pow2::from_exponent(2)), 3);
        assert_eq!(
            mod_floor(u32::MAX, Pow2::from_exponent(31)),
            i32::MAX as u32
        );
    }

    #[test]
    fn mod_floor_i64_negative_is_always_nonnegative() {
        // div_floor(-37, 32) = -64, so mod = -37 - (-64) = 27
        assert_eq!(mod_floor(-37i64, Pow2::from_exponent(5)), 27);
        // div_floor(-32, 32) = -32, so mod = 0
        assert_eq!(mod_floor(-32i64, Pow2::from_exponent(5)), 0);
        // div_floor(-1, 32) = -32, so mod = 31
        assert_eq!(mod_floor(-1i64, Pow2::from_exponent(5)), 31);
    }

    #[test]
    fn checked_mod_floor_boundary() {
        assert_eq!(checked_mod_floor(0_i32, Pow2::from_exponent(30)), Some(0));
        assert_eq!(checked_mod_floor(0_i32, Pow2::from_exponent(31)), Some(0));
        assert_eq!(checked_mod_floor(0_i32, Pow2::from_exponent(32)), None);

        assert_eq!(checked_mod_floor(0_u32, Pow2::from_exponent(31)), Some(0));
        assert_eq!(checked_mod_floor(0_u32, Pow2::from_exponent(32)), None);
    }

    #[test]
    fn is_multiple_of_u64_true() {
        assert!(is_multiple_of(0u64, Pow2::from_exponent(5)));
        assert!(is_multiple_of(32u64, Pow2::from_exponent(5)));
        assert!(is_multiple_of(64u64, Pow2::from_exponent(5)));
    }

    #[test]
    fn is_multiple_of_u64_false() {
        assert!(!is_multiple_of(31u64, Pow2::from_exponent(5)));
        assert!(!is_multiple_of(33u64, Pow2::from_exponent(5)));
    }

    #[test]
    fn is_multiple_of_i64_negative() {
        assert!(is_multiple_of(-32i64, Pow2::from_exponent(5)));
        assert!(is_multiple_of(i32::MIN, Pow2::from_exponent(31)));
        assert!(!is_multiple_of(-31i64, Pow2::from_exponent(5)));
    }

    #[test]
    fn unbounded_is_multiple_of_boundary() {
        assert!(unbounded_is_multiple_of(0_i32, Pow2::from_exponent(30)));
        assert!(unbounded_is_multiple_of(i32::MIN, Pow2::from_exponent(31)));
        assert!(unbounded_is_multiple_of(0_i32, Pow2::from_exponent(31)));
        assert!(unbounded_is_multiple_of(0_i32, Pow2::from_exponent(32)));
        assert!(unbounded_is_multiple_of(0_i32, Pow2::from_exponent(33)));
        assert!(unbounded_is_multiple_of(0_i32, Pow2::from_exponent(255)));
        assert!(!unbounded_is_multiple_of(
            i32::MIN,
            Pow2::from_exponent(255)
        ));
    }

    #[test]
    fn div_floor_consistent_with_floor_to_multiple() {
        let b = Pow2::from_exponent(5); // 32
        for a in (-200i64..=200).step_by(7) {
            // floor_to_multiple gives the floored dividend times the divisor,
            // so recover the quotient via div_floor itself rather than `/`
            // (plain `/` truncates toward zero, not toward -∞).
            let q = div_floor(a, b);
            assert_eq!(floor_to_multiple(a, b), q * 32, "mismatch at a={a}");
        }
    }

    #[test]
    fn floor_and_ceil_to_multiple_bracket_the_input() {
        let b = Pow2::from_exponent(4); // 16
        for a in 0u64..=200 {
            let lo = floor_to_multiple(a, b);
            let hi = ceil_to_multiple(a, b);
            assert!(lo <= a, "floor {lo} > {a}");
            assert!(hi >= a, "ceil {hi} < {a}");
            assert!(
                hi - lo == 0 || hi - lo == 16,
                "gap between floor and ceil should be 0 or 16, got {} for a={a}",
                hi - lo
            );
        }
    }

    #[test]
    fn mod_floor_plus_floor_to_multiple_equals_input() {
        let b = Pow2::from_exponent(5); // 32
        for a in (-300i64..=300).step_by(11) {
            let reconstructed = floor_to_multiple(a, b) + mod_floor(a, b);
            assert_eq!(reconstructed, a, "reconstruction failed at a={a}");
        }
    }

    #[test]
    fn constants() {
        assert_eq!(Pow2::VAL_1, Pow2::from_exponent(0));
        assert_eq!(Pow2::VAL_2, Pow2::from_exponent(1));
        assert_eq!(Pow2::VAL_4, Pow2::from_exponent(2));
        assert_eq!(Pow2::VAL_8, Pow2::from_exponent(3));
        assert_eq!(Pow2::VAL_16, Pow2::from_exponent(4));
        assert_eq!(Pow2::VAL_32, Pow2::from_exponent(5));
        assert_eq!(Pow2::VAL_64, Pow2::from_exponent(6));
        assert_eq!(Pow2::VAL_128, Pow2::from_exponent(7));
        assert_eq!(Pow2::VAL_256, Pow2::from_exponent(8));
        assert_eq!(Pow2::VAL_512, Pow2::from_exponent(9));

        assert_eq!(Pow2::KIBI, Pow2::from_exponent(10));
        assert_eq!(Pow2::MEBI, Pow2::from_exponent(20));
        assert_eq!(Pow2::GIBI, Pow2::from_exponent(30));
        assert_eq!(Pow2::TEBI, Pow2::from_exponent(40));
        assert_eq!(Pow2::PEBI, Pow2::from_exponent(50));
        assert_eq!(Pow2::EXBI, Pow2::from_exponent(60));
        assert_eq!(Pow2::ZEBI, Pow2::from_exponent(70));
        assert_eq!(Pow2::YOBI, Pow2::from_exponent(80));
        assert_eq!(Pow2::ROBI, Pow2::from_exponent(90));
        assert_eq!(Pow2::QUEBI, Pow2::from_exponent(100));
    }

    #[test]
    fn pow2_has_debug() {
        let v = Pow2::from_exponent(0);
        let mut s = String::new();
        write!(&mut s, "{:?}", v).unwrap();
        assert_eq!(s, "Pow2 { exponent: 0 }");
    }

    #[test]
    fn pow2_has_hash() {
        let v = Pow2::from_exponent(123);
        let mut s0 = DefaultHasher::new();
        v.hash(&mut s0);
        let mut s1 = DefaultHasher::new();
        123u8.hash(&mut s1);
        assert_eq!(s0.finish(), s1.finish());
    }

    #[test]
    fn pow2_has_eq() {
        assert_eq!(Pow2::VAL_1, Pow2::VAL_1);
        assert_eq!(Pow2::KIBI, Pow2::KIBI);
        assert_ne!(Pow2::VAL_1, Pow2::VAL_2);
    }

    #[test]
    fn pow2_has_ord() {
        assert!(Pow2::VAL_1 < Pow2::VAL_2);
        assert!(Pow2::VAL_2 <= Pow2::VAL_2);
        assert!(Pow2::VAL_2 >= Pow2::VAL_2);
        assert!(Pow2::VAL_4 > Pow2::VAL_2);
    }
}
