use std::error::Error;
use std::ops::{Div, DivAssign, Mul, MulAssign};
use std::{cmp, marker};

#[macro_use]
mod private;

// Expose the private Int trait as API to allow users to specify generic bounds.
pub use crate::private::{Int, IntAtLeastAsWide, SignedInt, UnsignedInt};

#[repr(transparent)]
#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub struct UnboundedPow2 {
    exponent: u8,
}

const _: () = assert!(size_of::<UnboundedPow2>() == size_of::<u8>());
const _: () = assert!(align_of::<UnboundedPow2>() == align_of::<u8>());

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct NotPow2;

impl Error for NotPow2 {}
impl std::fmt::Display for NotPow2 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Not a power of two")
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Pow2OutOfRange;

impl Error for Pow2OutOfRange {}
impl std::fmt::Display for Pow2OutOfRange {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Out of range")
    }
}

#[rustfmt::skip]
impl UnboundedPow2 {
    pub const VAL_1: Self = Self::from_exponent(0);
    pub const VAL_2: Self = Self::from_exponent(1);
    pub const VAL_4: Self = Self::from_exponent(2);
    pub const VAL_8: Self = Self::from_exponent(3);
    pub const VAL_16: Self = Self::from_exponent(4);
    pub const VAL_32: Self = Self::from_exponent(5);
    pub const VAL_64: Self = Self::from_exponent(6);
    pub const VAL_128: Self = Self::from_exponent(7);
    pub const VAL_256: Self = Self::from_exponent(8);
    pub const VAL_512: Self = Self::from_exponent(9);

    pub const KIBI: Self = Self::from_exponent(10);
    pub const MEBI: Self = Self::from_exponent(20);
    pub const GIBI: Self = Self::from_exponent(30);
    pub const TEBI: Self = Self::from_exponent(40);
    pub const PEBI: Self = Self::from_exponent(50);
    pub const EXBI: Self = Self::from_exponent(60);
    pub const ZEBI: Self = Self::from_exponent(70);
    pub const YOBI: Self = Self::from_exponent(80);
    pub const ROBI: Self = Self::from_exponent(90);
    pub const QUEBI: Self = Self::from_exponent(100);
}

impl UnboundedPow2 {
    #[must_use]
    #[inline(always)]
    pub const fn from_exponent(exponent: u8) -> UnboundedPow2 {
        UnboundedPow2 { exponent }
    }

    #[must_use]
    #[inline(always)]
    pub const fn align_of<T>() -> UnboundedPow2 {
        UnboundedPow2 {
            exponent: align_of::<T>().ilog2() as u8,
        }
    }

    #[must_use]
    #[inline(always)]
    pub const fn align_of_val<T: ?Sized>(val: &T) -> UnboundedPow2 {
        UnboundedPow2 {
            exponent: align_of_val(val).ilog2() as u8,
        }
    }

    #[must_use]
    #[inline(always)]
    pub const fn exponent(self) -> u8 {
        self.exponent
    }

    #[must_use]
    #[inline(always)]
    pub const fn is_safe<T: Int>(self) -> bool {
        self.exponent as u32 <= T::SAFE_SHIFT
    }

    #[must_use]
    #[inline(always)]
    pub fn checked_mul(self, other: UnboundedPow2) -> Option<UnboundedPow2> {
        Some(UnboundedPow2::from_exponent(
            self.exponent.checked_add(other.exponent)?,
        ))
    }

    #[must_use]
    #[inline(always)]
    pub const fn saturating_mul(self, other: UnboundedPow2) -> UnboundedPow2 {
        UnboundedPow2::from_exponent(self.exponent.saturating_add(other.exponent))
    }

    #[must_use]
    #[inline(always)]
    pub fn checked_div(self, other: UnboundedPow2) -> Option<UnboundedPow2> {
        Some(UnboundedPow2::from_exponent(
            self.exponent.checked_sub(other.exponent)?,
        ))
    }

    #[must_use]
    #[inline(always)]
    pub const fn saturating_div(self, other: UnboundedPow2) -> UnboundedPow2 {
        UnboundedPow2::from_exponent(self.exponent.saturating_sub(other.exponent))
    }
}

macro_rules! impl_as {
    ($name:ident, $t:ty) => {
        impl UnboundedPow2 {
            #[must_use]
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
        impl TryFrom<$t> for UnboundedPow2 {
            type Error = NotPow2;

            #[inline(always)]
            fn try_from(value: $t) -> Result<Self, Self::Error> {
                if !value.is_power_of_two() {
                    Err(NotPow2)
                } else {
                    Ok(UnboundedPow2::from_exponent(value.ilog2() as u8))
                }
            }
        }

        impl TryFrom<UnboundedPow2> for $t {
            type Error = Pow2OutOfRange;

            #[inline(always)]
            fn try_from(value: UnboundedPow2) -> Result<Self, Self::Error> {
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

impl Mul for UnboundedPow2 {
    type Output = UnboundedPow2;

    #[allow(clippy::suspicious_arithmetic_impl)]
    #[inline(always)]
    fn mul(self, other: UnboundedPow2) -> UnboundedPow2 {
        UnboundedPow2::from_exponent(self.exponent + other.exponent)
    }
}

impl MulAssign for UnboundedPow2 {
    #[allow(clippy::suspicious_arithmetic_impl)]
    #[inline(always)]
    fn mul_assign(&mut self, other: UnboundedPow2) {
        self.exponent += other.exponent;
    }
}

impl Div for UnboundedPow2 {
    type Output = UnboundedPow2;

    #[inline(always)]
    fn div(self, other: UnboundedPow2) -> UnboundedPow2 {
        UnboundedPow2::from_exponent(self.exponent - other.exponent)
    }
}

impl DivAssign for UnboundedPow2 {
    #[allow(clippy::suspicious_arithmetic_impl)]
    #[inline(always)]
    fn div_assign(&mut self, other: UnboundedPow2) {
        self.exponent -= other.exponent;
    }
}

#[repr(transparent)]
#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub struct Pow2<T> {
    exponent: u8,
    _marker: marker::PhantomData<T>,
}

const _: () = assert!(size_of::<Pow2<u128>>() == size_of::<u8>());
const _: () = assert!(align_of::<Pow2<u128>>() == align_of::<u8>());

impl<T> Pow2<T>
where
    T: UnsignedInt,
{
    #[inline(always)]
    pub const fn from_exponent(exponent: u8) -> Result<Self, Pow2OutOfRange> {
        if exponent as u32 >= T::BITS {
            Err(Pow2OutOfRange)
        } else {
            Ok(Self {
                exponent,
                _marker: marker::PhantomData,
            })
        }
    }

    #[inline(always)]
    pub const fn align_of<U>() -> Result<Self, Pow2OutOfRange> {
        Self::from_exponent(align_of::<U>().ilog2() as u8)
    }

    #[inline(always)]
    pub const fn align_of_val<U: ?Sized>(val: &U) -> Result<Self, Pow2OutOfRange> {
        Self::from_exponent(align_of_val(val).ilog2() as u8)
    }

    #[must_use]
    #[inline(always)]
    pub const fn exponent(self) -> u8 {
        self.exponent
    }

    #[must_use]
    #[inline(always)]
    pub fn value(self) -> T {
        // SAFETY: SafePow2 guarantees a valid shift
        unsafe { T::ONE.unchecked_shl(self.exponent as u32) }
    }

    #[must_use]
    #[inline(always)]
    pub fn mask(self) -> T {
        // SAFETY: SafePow2 guarantees a valid shift
        unsafe { T::unchecked_mask(self.exponent as u32) }
    }

    #[must_use]
    #[inline(always)]
    pub fn checked_mul(self, other: Self) -> Option<Self> {
        // The addition does not overflow because it's at most 127+127<=255 for u128
        Self::from_exponent(self.exponent + other.exponent).ok()
    }

    #[must_use]
    #[inline(always)]
    pub fn saturating_mul(self, other: Self) -> Self {
        Self {
            exponent: cmp::min(self.exponent + other.exponent, (T::BITS - 1) as u8),
            _marker: marker::PhantomData,
        }
    }

    #[must_use]
    #[inline(always)]
    pub fn checked_div(self, other: Self) -> Option<Self> {
        Some(Self {
            exponent: self.exponent.checked_sub(other.exponent)?,
            _marker: marker::PhantomData,
        })
    }

    #[must_use]
    #[inline(always)]
    pub const fn saturating_div(self, other: Self) -> Self {
        Self {
            exponent: self.exponent.saturating_sub(other.exponent),
            _marker: marker::PhantomData,
        }
    }
}

#[rustfmt::skip]
impl Pow2<u8> {
    pub const VAL_1  : Self = Self { exponent: 0,   _marker: marker::PhantomData };
    pub const VAL_2  : Self = Self { exponent: 1,   _marker: marker::PhantomData };
    pub const VAL_4  : Self = Self { exponent: 2,   _marker: marker::PhantomData };
    pub const VAL_8  : Self = Self { exponent: 3,   _marker: marker::PhantomData };
    pub const VAL_16 : Self = Self { exponent: 4,   _marker: marker::PhantomData };
    pub const VAL_32 : Self = Self { exponent: 5,   _marker: marker::PhantomData };
    pub const VAL_64 : Self = Self { exponent: 6,   _marker: marker::PhantomData };
    pub const VAL_128: Self = Self { exponent: 7,   _marker: marker::PhantomData };
}

#[rustfmt::skip] 
impl Pow2<u16> { 
    pub const VAL_1  : Self = Self { exponent: 0,   _marker: marker::PhantomData };
    pub const VAL_2  : Self = Self { exponent: 1,   _marker: marker::PhantomData };
    pub const VAL_4  : Self = Self { exponent: 2,   _marker: marker::PhantomData };
    pub const VAL_8  : Self = Self { exponent: 3,   _marker: marker::PhantomData };
    pub const VAL_16 : Self = Self { exponent: 4,   _marker: marker::PhantomData };
    pub const VAL_32 : Self = Self { exponent: 5,   _marker: marker::PhantomData };
    pub const VAL_64 : Self = Self { exponent: 6,   _marker: marker::PhantomData };
    pub const VAL_128: Self = Self { exponent: 7,   _marker: marker::PhantomData };
    pub const VAL_256: Self = Self { exponent: 8,   _marker: marker::PhantomData };
    pub const VAL_512: Self = Self { exponent: 9,   _marker: marker::PhantomData };
 
    pub const KIBI   : Self = Self { exponent: 10,  _marker: marker::PhantomData };
}

#[rustfmt::skip] 
impl Pow2<u32> { 
    pub const VAL_1  : Self = Self { exponent: 0,   _marker: marker::PhantomData };
    pub const VAL_2  : Self = Self { exponent: 1,   _marker: marker::PhantomData };
    pub const VAL_4  : Self = Self { exponent: 2,   _marker: marker::PhantomData };
    pub const VAL_8  : Self = Self { exponent: 3,   _marker: marker::PhantomData };
    pub const VAL_16 : Self = Self { exponent: 4,   _marker: marker::PhantomData };
    pub const VAL_32 : Self = Self { exponent: 5,   _marker: marker::PhantomData };
    pub const VAL_64 : Self = Self { exponent: 6,   _marker: marker::PhantomData };
    pub const VAL_128: Self = Self { exponent: 7,   _marker: marker::PhantomData };
    pub const VAL_256: Self = Self { exponent: 8,   _marker: marker::PhantomData };
    pub const VAL_512: Self = Self { exponent: 9,   _marker: marker::PhantomData };
 
    pub const KIBI   : Self = Self { exponent: 10,  _marker: marker::PhantomData };
    pub const MEBI   : Self = Self { exponent: 20,  _marker: marker::PhantomData };
    pub const GIBI   : Self = Self { exponent: 30,  _marker: marker::PhantomData };
}

#[rustfmt::skip] 
impl Pow2<u64> { 
    pub const VAL_1  : Self = Self { exponent: 0,   _marker: marker::PhantomData };
    pub const VAL_2  : Self = Self { exponent: 1,   _marker: marker::PhantomData };
    pub const VAL_4  : Self = Self { exponent: 2,   _marker: marker::PhantomData };
    pub const VAL_8  : Self = Self { exponent: 3,   _marker: marker::PhantomData };
    pub const VAL_16 : Self = Self { exponent: 4,   _marker: marker::PhantomData };
    pub const VAL_32 : Self = Self { exponent: 5,   _marker: marker::PhantomData };
    pub const VAL_64 : Self = Self { exponent: 6,   _marker: marker::PhantomData };
    pub const VAL_128: Self = Self { exponent: 7,   _marker: marker::PhantomData };
    pub const VAL_256: Self = Self { exponent: 8,   _marker: marker::PhantomData };
    pub const VAL_512: Self = Self { exponent: 9,   _marker: marker::PhantomData };
 
    pub const KIBI   : Self = Self { exponent: 10,  _marker: marker::PhantomData };
    pub const MEBI   : Self = Self { exponent: 20,  _marker: marker::PhantomData };
    pub const GIBI   : Self = Self { exponent: 30,  _marker: marker::PhantomData };
    pub const TEBI   : Self = Self { exponent: 40,  _marker: marker::PhantomData };
    pub const PEBI   : Self = Self { exponent: 50,  _marker: marker::PhantomData };
    pub const EXBI   : Self = Self { exponent: 60,  _marker: marker::PhantomData };
}

#[rustfmt::skip]
impl Pow2<u128> {
    pub const VAL_1  : Self = Self { exponent: 0,   _marker: marker::PhantomData };
    pub const VAL_2  : Self = Self { exponent: 1,   _marker: marker::PhantomData };
    pub const VAL_4  : Self = Self { exponent: 2,   _marker: marker::PhantomData };
    pub const VAL_8  : Self = Self { exponent: 3,   _marker: marker::PhantomData };
    pub const VAL_16 : Self = Self { exponent: 4,   _marker: marker::PhantomData };
    pub const VAL_32 : Self = Self { exponent: 5,   _marker: marker::PhantomData };
    pub const VAL_64 : Self = Self { exponent: 6,   _marker: marker::PhantomData };
    pub const VAL_128: Self = Self { exponent: 7,   _marker: marker::PhantomData };
    pub const VAL_256: Self = Self { exponent: 8,   _marker: marker::PhantomData };
    pub const VAL_512: Self = Self { exponent: 9,   _marker: marker::PhantomData };

    pub const KIBI   : Self = Self { exponent: 10,  _marker: marker::PhantomData };
    pub const MEBI   : Self = Self { exponent: 20,  _marker: marker::PhantomData };
    pub const GIBI   : Self = Self { exponent: 30,  _marker: marker::PhantomData };
    pub const TEBI   : Self = Self { exponent: 40,  _marker: marker::PhantomData };
    pub const PEBI   : Self = Self { exponent: 50,  _marker: marker::PhantomData };
    pub const EXBI   : Self = Self { exponent: 60,  _marker: marker::PhantomData };
    pub const ZEBI   : Self = Self { exponent: 70,  _marker: marker::PhantomData };
    pub const YOBI   : Self = Self { exponent: 80,  _marker: marker::PhantomData };
    pub const ROBI   : Self = Self { exponent: 90,  _marker: marker::PhantomData };
    pub const QUEBI  : Self = Self { exponent: 100, _marker: marker::PhantomData };
}

#[rustfmt::skip]
#[cfg(target_pointer_width = "16")]
impl Pow2<usize> {
    pub const VAL_1  : Self = Self { exponent: 0,   _marker: marker::PhantomData };
    pub const VAL_2  : Self = Self { exponent: 1,   _marker: marker::PhantomData };
    pub const VAL_4  : Self = Self { exponent: 2,   _marker: marker::PhantomData };
    pub const VAL_8  : Self = Self { exponent: 3,   _marker: marker::PhantomData };
    pub const VAL_16 : Self = Self { exponent: 4,   _marker: marker::PhantomData };
    pub const VAL_32 : Self = Self { exponent: 5,   _marker: marker::PhantomData };
    pub const VAL_64 : Self = Self { exponent: 6,   _marker: marker::PhantomData };
    pub const VAL_128: Self = Self { exponent: 7,   _marker: marker::PhantomData };
    pub const VAL_256: Self = Self { exponent: 8,   _marker: marker::PhantomData };
    pub const VAL_512: Self = Self { exponent: 9,   _marker: marker::PhantomData };
 
    pub const KIBI   : Self = Self { exponent: 10,  _marker: marker::PhantomData };
}

#[rustfmt::skip]
#[cfg(target_pointer_width = "32")]
impl Pow2<usize> {
    pub const VAL_1  : Self = Self { exponent: 0,   _marker: marker::PhantomData };
    pub const VAL_2  : Self = Self { exponent: 1,   _marker: marker::PhantomData };
    pub const VAL_4  : Self = Self { exponent: 2,   _marker: marker::PhantomData };
    pub const VAL_8  : Self = Self { exponent: 3,   _marker: marker::PhantomData };
    pub const VAL_16 : Self = Self { exponent: 4,   _marker: marker::PhantomData };
    pub const VAL_32 : Self = Self { exponent: 5,   _marker: marker::PhantomData };
    pub const VAL_64 : Self = Self { exponent: 6,   _marker: marker::PhantomData };
    pub const VAL_128: Self = Self { exponent: 7,   _marker: marker::PhantomData };
    pub const VAL_256: Self = Self { exponent: 8,   _marker: marker::PhantomData };
    pub const VAL_512: Self = Self { exponent: 9,   _marker: marker::PhantomData };
 
    pub const KIBI   : Self = Self { exponent: 10,  _marker: marker::PhantomData };
    pub const MEBI   : Self = Self { exponent: 20,  _marker: marker::PhantomData };
    pub const GIBI   : Self = Self { exponent: 30,  _marker: marker::PhantomData };
}

#[rustfmt::skip]
#[cfg(target_pointer_width = "64")]
impl Pow2<usize> {
    pub const VAL_1  : Self = Self { exponent: 0,   _marker: marker::PhantomData };
    pub const VAL_2  : Self = Self { exponent: 1,   _marker: marker::PhantomData };
    pub const VAL_4  : Self = Self { exponent: 2,   _marker: marker::PhantomData };
    pub const VAL_8  : Self = Self { exponent: 3,   _marker: marker::PhantomData };
    pub const VAL_16 : Self = Self { exponent: 4,   _marker: marker::PhantomData };
    pub const VAL_32 : Self = Self { exponent: 5,   _marker: marker::PhantomData };
    pub const VAL_64 : Self = Self { exponent: 6,   _marker: marker::PhantomData };
    pub const VAL_128: Self = Self { exponent: 7,   _marker: marker::PhantomData };
    pub const VAL_256: Self = Self { exponent: 8,   _marker: marker::PhantomData };
    pub const VAL_512: Self = Self { exponent: 9,   _marker: marker::PhantomData };
 
    pub const KIBI   : Self = Self { exponent: 10,  _marker: marker::PhantomData };
    pub const MEBI   : Self = Self { exponent: 20,  _marker: marker::PhantomData };
    pub const GIBI   : Self = Self { exponent: 30,  _marker: marker::PhantomData };
    pub const TEBI   : Self = Self { exponent: 40,  _marker: marker::PhantomData };
    pub const PEBI   : Self = Self { exponent: 50,  _marker: marker::PhantomData };
    pub const EXBI   : Self = Self { exponent: 60,  _marker: marker::PhantomData };
}

impl<T> From<Pow2<T>> for UnboundedPow2 {
    #[inline(always)]
    fn from(value: Pow2<T>) -> Self {
        Self {
            exponent: value.exponent,
        }
    }
}

impl<T> TryFrom<UnboundedPow2> for Pow2<T>
where
    T: UnsignedInt,
{
    type Error = Pow2OutOfRange;

    #[inline(always)]
    fn try_from(value: UnboundedPow2) -> Result<Self, Self::Error> {
        Self::from_exponent(value.exponent)
    }
}

macro_rules! impl_pow2_self_conv_from {
    ($t:ty => $($into:ty),*) => {
        $(
            impl From<Pow2<$t>> for Pow2<$into> {
                #[inline(always)]
                fn from(value: Pow2<$t>) -> Self {
                    Self {
                        exponent: value.exponent,
                        _marker: marker::PhantomData,
                    }
                }
            }
        )*
    };
}

impl_pow2_self_conv_from!(u8 => u16, u32, u64, u128, usize);

#[cfg(target_pointer_width = "16")]
impl_pow2_self_conv_from!(u16 => u32, u64, u128);

#[cfg(target_pointer_width = "32")]
impl_pow2_self_conv_from!(u16 => u32, u64, u128, usize);

#[cfg(target_pointer_width = "64")]
impl_pow2_self_conv_from!(u16 => u32, u64, u128, usize);

#[cfg(target_pointer_width = "16")]
impl_pow2_self_conv_from!(u32 => u64, u128);

#[cfg(target_pointer_width = "32")]
impl_pow2_self_conv_from!(u32 => u64, u128);

#[cfg(target_pointer_width = "64")]
impl_pow2_self_conv_from!(u32 => u64, u128, usize);

impl_pow2_self_conv_from!(u64 => u128);

macro_rules! impl_pow2_self_conv_try_from {
    ($t:ty => $($into:ty),*) => {
        $(
            impl TryFrom<Pow2<$t>> for Pow2<$into> {
                type Error = Pow2OutOfRange;

                #[inline(always)]
                fn try_from(value: Pow2<$t>) -> Result<Self, Self::Error> {
                    Pow2::from_exponent(value.exponent)
                }
            }
        )*
    };
}

impl_pow2_self_conv_try_from!(u16 => u8);

#[cfg(target_pointer_width = "16")]
impl_pow2_self_conv_try_from!(u32 => u8, u16, usize);

#[cfg(target_pointer_width = "32")]
impl_pow2_self_conv_try_from!(u32 => u8, u16);

#[cfg(target_pointer_width = "64")]
impl_pow2_self_conv_try_from!(u32 => u8, u16);

#[cfg(target_pointer_width = "16")]
impl_pow2_self_conv_try_from!(u64 => u8, u16, u32, usize);

#[cfg(target_pointer_width = "32")]
impl_pow2_self_conv_try_from!(u64 => u8, u16, u32, usize);

#[cfg(target_pointer_width = "64")]
impl_pow2_self_conv_try_from!(u64 => u8, u16, u32);

impl_pow2_self_conv_try_from!(u128 => u8, u16, u32, u64);

pub type SafePow2u8 = Pow2<u8>;
pub type SafePow2u16 = Pow2<u16>;
pub type SafePow2u32 = Pow2<u32>;
pub type SafePow2u64 = Pow2<u64>;
pub type SafePow2u128 = Pow2<u128>;
pub type SafePow2usize = Pow2<usize>;

impl_trait_signed_unsigned!(
    Div<UnboundedPow2>,
    signed_body {
        type Output = Self;

        #[inline(always)]
        fn div(self, other: UnboundedPow2) -> Self {
            debug_assert!(other.is_safe::<<Self as Int>::Unsigned>());
            if self >= 0 {
                self >> other.exponent
            } else {
                let mask = <Self as Int>::mask(other.exponent as u32);
                (self + mask) >> other.exponent
            }
        }
    },
    unsigned_body {
        type Output = Self;

        #[inline(always)]
        fn div(self, other: UnboundedPow2) -> Self {
            debug_assert!(other.is_safe::<<Self as Int>::Unsigned>());
            self >> other.exponent
        }
    }
);

impl_generic_trait_signed_unsigned!(
    <T> Div<Pow2<T>> where { T: UnsignedInt, Self: IntAtLeastAsWide<T> },
    signed_body {
        type Output = Self;

        #[inline(always)]
        fn div(self, other: Pow2<T>) -> Self {
            if self >= 0 {
                // SAFETY: SafePow2 guarantees a valid shift
                unsafe { self.unchecked_shr(other.exponent as u32) }
            } else {
                // SAFETY: SafePow2 guarantees a valid shift
                unsafe {
                    let mask = <Self as Int>::unchecked_mask(other.exponent as u32);
                    (self + mask).unchecked_shr(other.exponent as u32)
                }
            }
        }
    },
    unsigned_body {
        type Output = Self;

        #[inline(always)]
        fn div(self, other: Pow2<T>) -> Self {
            self >> other.exponent
        }
    }
);

impl_trait_all_ints!(
    DivAssign<UnboundedPow2> => {
        #[inline(always)]
        fn div_assign(&mut self, other: UnboundedPow2) {
            debug_assert!(other.is_safe::<<Self as Int>::Unsigned>());
            *self = *self / other;
        }
    }
);

impl_generic_trait_all_ints!(
    <T> DivAssign<Pow2<T>> where { T: UnsignedInt, Self: IntAtLeastAsWide<T> } => {
        #[inline(always)]
        fn div_assign(&mut self, other: Pow2<T>) {
            *self = *self / other;
        }
    }
);

impl_trait_all_ints!(
    Mul<UnboundedPow2> => {
        type Output = Self;

        #[inline(always)]
        fn mul(self, other: UnboundedPow2) -> Self {
            debug_assert!(other.is_safe::<<Self as Int>::Unsigned>());
            // Check for overflow.
            debug_assert!(self == self << other.exponent >> other.exponent);
            self << other.exponent
        }
    }
);

impl_generic_trait_all_ints!(
    <T> Mul<Pow2<T>> where { T: UnsignedInt } => {
        type Output = Self;

        #[inline(always)]
        fn mul(self, other: Pow2<T>) -> Self {
            // Check for overflow.
            debug_assert!(self == self << other.exponent >> other.exponent);
            // SAFETY: SafePow2 guarantees a valid shift
            unsafe { self.unchecked_shl(other.exponent as u32) }
        }
    }
);

impl_trait_all_ints!(
    MulAssign<UnboundedPow2> => {
        #[inline(always)]
        fn mul_assign(&mut self, other: UnboundedPow2) {
            *self = *self * other;
        }
    }
);

impl_generic_trait_all_ints!(
    <T> MulAssign<Pow2<T>> where { T: UnsignedInt } => {
        #[inline(always)]
        fn mul_assign(&mut self, other: Pow2<T>) {
            *self = *self * other;
        }
    }
);

macro_rules! make_func_trait {
    ($trait_name:ident, $func_name:ident) => {
        #[doc = concat!(
            "Trait implementing `", stringify!($func_name), "` taking `self` and some `rhs` of type `Rhs`.\n\n",
            "Implementations of this trait define the behavior of the free ",
            "function [`", stringify!($func_name), "`] which dispatches to them.\n\n",
            "The [`crate::ipow2`] module may be implementing this trait for all primitive integer ",
            "types as `self` and the following `Rhs` types:\n",
            " - [`Pow2`] - [`docs`](__detached_docs::Pow2::", stringify!($trait_name), ")\n",
            " - [`UnboundedPow2`] - [`docs`](__detached_docs::UnboundedPow2::", stringify!($trait_name), ")\n",
        )]
        pub trait $trait_name<Rhs> {
            type Output;

            #[doc = concat!(
                "See docs for [`", stringify!($trait_name), "`]."
            )]
            #[must_use]
            fn $func_name(self, rhs: Rhs) -> Self::Output;
        }

        #[doc = concat!(
            "Uses the [`", stringify!($trait_name), "`] trait for dispatch.\n\n",
            "Invokes [`", stringify!($trait_name), "::", stringify!($func_name), "`] ",
            "via `lhs.", stringify!($func_name), "(rhs)`.",
        )]
        #[must_use]
        #[inline(always)]
        pub fn $func_name<L, R>(lhs: L, rhs: R) -> L::Output
        where
            L: $trait_name<R>
        {
            lhs.$func_name(rhs)
        }
    };
}

make_func_trait!(CheckedMul, checked_mul);

/// See [docs](__detached_docs::UnboundedPow2::CheckedMul)
impl<T> CheckedMul<UnboundedPow2> for T
where
    T: Int,
{
    type Output = Option<Self>;

    /// See [docs](__detached_docs::UnboundedPow2::CheckedMul)
    #[inline(always)]
    fn checked_mul(self, rhs: UnboundedPow2) -> Self::Output {
        let result = self.checked_shl(rhs.exponent as u32)?;
        if self == result >> rhs.exponent {
            Some(result)
        } else {
            None
        }
    }
}

/// See [docs](__detached_docs::Pow2::CheckedMul)
impl<L, T> CheckedMul<Pow2<T>> for L
where
    L: IntAtLeastAsWide<T>,
    T: UnsignedInt,
{
    type Output = Option<Self>;

    /// See [docs](__detached_docs::Pow2::CheckedMul)
    #[inline(always)]
    fn checked_mul(self, rhs: Pow2<T>) -> Self::Output {
        // SAFETY: SafePow2 guarantees a valid shift
        let result = unsafe { self.unchecked_shl(rhs.exponent as u32) };
        // SAFETY: SafePow2 guarantees a valid shift
        if self == unsafe { result.unchecked_shr(rhs.exponent as u32) } {
            Some(result)
        } else {
            None
        }
    }
}

make_func_trait!(CheckedDiv, checked_div);

impl<T> CheckedDiv<UnboundedPow2> for T
where
    T: Int + Div<UnboundedPow2, Output = T>,
{
    type Output = Option<Self>;

    #[inline(always)]
    fn checked_div(self, rhs: UnboundedPow2) -> Self::Output {
        rhs.is_safe::<T::Unsigned>().then(|| self / rhs)
    }
}

make_func_trait!(UnboundedDiv, unbounded_div);

impl<T> UnboundedDiv<UnboundedPow2> for T
where
    T: Int + Div<UnboundedPow2, Output = T>,
{
    type Output = Self;

    #[inline(always)]
    fn unbounded_div(self, rhs: UnboundedPow2) -> Self::Output {
        if rhs.is_safe::<T::Unsigned>() {
            self / rhs
        } else {
            T::ZERO
        }
    }
}

make_func_trait!(DivFloor, div_floor);

impl<T> DivFloor<UnboundedPow2> for T
where
    T: Int,
{
    type Output = Self;

    #[inline(always)]
    fn div_floor(self, rhs: UnboundedPow2) -> Self::Output {
        debug_assert!(rhs.is_safe::<T::Unsigned>());
        self >> rhs.exponent
    }
}

impl<L, T> DivFloor<Pow2<T>> for L
where
    L: IntAtLeastAsWide<T>,
    T: UnsignedInt,
{
    type Output = Self;

    #[inline(always)]
    fn div_floor(self, rhs: Pow2<T>) -> Self::Output {
        // SAFETY: SafePow2 guarantees a valid shift
        unsafe { self.unchecked_shr(rhs.exponent as u32) }
    }
}

make_func_trait!(CheckedDivFloor, checked_div_floor);

impl<T> CheckedDivFloor<UnboundedPow2> for T
where
    T: Int,
{
    type Output = Option<Self>;

    #[inline(always)]
    fn checked_div_floor(self, rhs: UnboundedPow2) -> Self::Output {
        rhs.is_safe::<T::Unsigned>().then(|| div_floor(self, rhs))
    }
}

make_func_trait!(UnboundedDivFloor, unbounded_div_floor);

impl<T> UnboundedDivFloor<UnboundedPow2> for T
where
    T: Int,
{
    type Output = Self;

    #[inline(always)]
    fn unbounded_div_floor(self, rhs: UnboundedPow2) -> Self::Output {
        if rhs.is_safe::<T::Unsigned>() {
            div_floor(self, rhs)
        } else if T::IS_UNSIGNED || self >= T::ZERO {
            T::ZERO
        } else {
            T::MINUS_ONE
        }
    }
}

make_func_trait!(ModFloor, mod_floor);

impl<T> ModFloor<UnboundedPow2> for T
where
    T: Int,
{
    type Output = Self;

    #[inline(always)]
    fn mod_floor(self, rhs: UnboundedPow2) -> Self::Output {
        debug_assert!(rhs.is_safe::<T::Unsigned>());
        self - floor_to_multiple(self, rhs)
    }
}

impl<L, T> ModFloor<Pow2<T>> for L
where
    L: IntAtLeastAsWide<T>,
    T: UnsignedInt,
{
    type Output = Self;

    #[inline(always)]
    fn mod_floor(self, rhs: Pow2<T>) -> Self::Output {
        self - floor_to_multiple(self, rhs)
    }
}

make_func_trait!(CheckedModFloor, checked_mod_floor);

impl<T> CheckedModFloor<UnboundedPow2> for T
where
    T: Int,
{
    type Output = Option<Self>;

    #[inline(always)]
    fn checked_mod_floor(self, rhs: UnboundedPow2) -> Self::Output {
        rhs.is_safe::<T::Unsigned>().then(|| mod_floor(self, rhs))
    }
}

make_func_trait!(DivCeil, div_ceil);

impl<T> DivCeil<UnboundedPow2> for T
where
    T: Int,
{
    type Output = Self;

    #[inline(always)]
    fn div_ceil(self, rhs: UnboundedPow2) -> Self::Output {
        debug_assert!(rhs.is_safe::<T::Unsigned>());
        // While slightly slower on a single value, this implementation is chosen
        // because it is better when performed with at least 2 values with the same rhs,
        // and also uses the same mask as other operations, and also vectorizable.
        let mask = T::mask(rhs.exponent as u32);
        let floored = div_floor(self, rhs);
        floored + T::from_bool((self & mask).is_not_zero())
    }
}

impl<L, T> DivCeil<Pow2<T>> for L
where
    L: IntAtLeastAsWide<T>,
    T: UnsignedInt,
{
    type Output = Self;

    #[inline(always)]
    fn div_ceil(self, rhs: Pow2<T>) -> Self::Output {
        // SAFETY: SafePow2 guarantees a valid shift
        let mask = unsafe { Self::unchecked_mask(rhs.exponent as u32) };
        let floored = div_floor(self, rhs);
        floored + Self::from_bool((self & mask).is_not_zero())
    }
}

make_func_trait!(CheckedDivCeil, checked_div_ceil);

impl<T> CheckedDivCeil<UnboundedPow2> for T
where
    T: Int,
{
    type Output = Option<Self>;

    #[inline(always)]
    fn checked_div_ceil(self, rhs: UnboundedPow2) -> Self::Output {
        rhs.is_safe::<T::Unsigned>().then(|| div_ceil(self, rhs))
    }
}

make_func_trait!(UnboundedDivCeil, unbounded_div_ceil);

impl<T> UnboundedDivCeil<UnboundedPow2> for T
where
    T: Int,
{
    type Output = Self;

    fn unbounded_div_ceil(self, rhs: UnboundedPow2) -> Self::Output {
        if rhs.is_safe::<T::Unsigned>() {
            div_ceil(self, rhs)
        } else if self <= T::ZERO {
            T::ZERO
        } else {
            T::ONE
        }
    }
}

make_func_trait!(DivRound, div_round);

impl_trait_signed_unsigned!(
    DivRound<UnboundedPow2>,
    signed_body {
        type Output = Self;

        #[inline(always)]
        fn div_round(self, rhs: UnboundedPow2) -> Self::Output {
            debug_assert!(Self::IS_SIGNED);
            debug_assert!(rhs.is_safe::<<Self as Int>::Unsigned>());
            let mask = Self::mask(rhs.exponent as u32);
            let floored = div_floor(self, rhs);
            // Account for signedness of a
            // for a >= 0 we need rem != 0 and rem >= mask_highest_bit
            // for a < 0 we need rem > mask_highest_bit
            // make sure the saturating sub saturates to zero (unsigned)
            let rem = (self & mask)
                .cast_unsigned()
                .saturating_sub(<Self as Int>::Unsigned::from_bool(self < Self::ZERO));
            let rems = (rem << 1) >> rhs.exponent;
            floored + Self::self_from_unsigned(rems & <Self as Int>::Unsigned::ONE)
        }
    },
    unsigned_body {
        type Output = Self;

        #[inline(always)]
        fn div_round(self, rhs: UnboundedPow2) -> Self::Output {
            debug_assert!(Self::IS_UNSIGNED);
            debug_assert!(rhs.is_safe::<<Self as Int>::Unsigned>());
            let bit = Self::highest_mask_bit(rhs.exponent as u32);
            let floored = div_floor(self, rhs);
            // this is only valid for unsigned a
            floored + Self::from_bool((self & bit).is_not_zero())
        }
    }
);

impl_generic_trait_signed_unsigned!(
    <T> DivRound<Pow2<T>> where { T: UnsignedInt, Self: IntAtLeastAsWide<T> },
    signed_body {
        type Output = Self;

        #[inline(always)]
        fn div_round(self, rhs: Pow2<T>) -> Self {
            debug_assert!(Self::IS_SIGNED);
            // SAFETY: SafePow2 guarantees a valid shift
            let mask = unsafe { Self::unchecked_mask(rhs.exponent as u32) };
            let floored = div_floor(self, rhs);
            // Account for signedness of a
            // for a >= 0 we need rem != 0 and rem >= mask_highest_bit
            // for a < 0 we need rem > mask_highest_bit
            // make sure the saturating sub saturates to zero (unsigned)
            let rem = (self & mask)
                .cast_unsigned()
                .saturating_sub(<Self as Int>::Unsigned::from_bool(self < Self::ZERO));
            // SAFETY: SafePow2 guarantees a valid shift
            let rems = unsafe { rem.unchecked_shl(1).unchecked_shr(rhs.exponent as u32) };
            floored + Self::self_from_unsigned(rems & <Self as Int>::Unsigned::ONE)
        }
    },
    unsigned_body {
        type Output = Self;

        #[inline(always)]
        fn div_round(self, rhs: Pow2<T>) -> Self {
            debug_assert!(Self::IS_UNSIGNED);
            // SAFETY: SafePow2 guarantees a valid shift
            let bit = unsafe { Self::unchecked_highest_mask_bit(rhs.exponent as u32) };
            let floored = div_floor(self, rhs);
            // this is only valid for unsigned a
            floored + Self::from_bool((self & bit).is_not_zero())
        }
    }
);

make_func_trait!(CheckedDivRound, checked_div_round);

impl<T> CheckedDivRound<UnboundedPow2> for T
where
    T: Int + DivRound<UnboundedPow2, Output = Self>,
{
    type Output = Option<Self>;

    #[inline(always)]
    fn checked_div_round(self, rhs: UnboundedPow2) -> Self::Output {
        rhs.is_safe::<T::Unsigned>().then(|| div_round(self, rhs))
    }
}

make_func_trait!(UnboundedDivRound, unbounded_div_round);

impl<T> UnboundedDivRound<UnboundedPow2> for T
where
    T: Int + DivRound<UnboundedPow2, Output = Self>,
{
    type Output = Self;

    fn unbounded_div_round(self, rhs: UnboundedPow2) -> Self::Output {
        if rhs.is_safe::<T::Unsigned>() {
            div_round(self, rhs)
        } else if T::IS_SIGNED && self == T::MIN && rhs.exponent as u32 == T::BITS {
            // result would be -0.5, so round to -1
            T::MINUS_ONE
        } else {
            T::ZERO
        }
    }
}

make_func_trait!(IsMultipleOf, is_multiple_of);

impl<T> IsMultipleOf<UnboundedPow2> for T
where
    T: Int,
{
    type Output = bool;

    #[inline(always)]
    fn is_multiple_of(self, rhs: UnboundedPow2) -> Self::Output {
        debug_assert!(rhs.is_safe::<T::Unsigned>());
        self.trailing_zeros() >= rhs.exponent as u32
    }
}

impl<L, T> IsMultipleOf<Pow2<T>> for L
where
    L: IntAtLeastAsWide<T>,
    T: UnsignedInt,
{
    type Output = bool;

    #[inline(always)]
    fn is_multiple_of(self, rhs: Pow2<T>) -> Self::Output {
        self.trailing_zeros() >= rhs.exponent as u32
    }
}

make_func_trait!(UnboundedIsMultipleOf, unbounded_is_multiple_of);

impl<T> UnboundedIsMultipleOf<UnboundedPow2> for T
where
    T: Int,
{
    type Output = bool;

    fn unbounded_is_multiple_of(self, rhs: UnboundedPow2) -> Self::Output {
        self.is_zero() || self.trailing_zeros() >= rhs.exponent as u32
    }
}

make_func_trait!(FloorToMultiple, floor_to_multiple);

impl<T> FloorToMultiple<UnboundedPow2> for T
where
    T: Int,
{
    type Output = Self;

    #[inline(always)]
    fn floor_to_multiple(self, rhs: UnboundedPow2) -> Self::Output {
        debug_assert!(rhs.is_safe::<T::Unsigned>());
        self >> rhs.exponent << rhs.exponent
    }
}

impl<L, T> FloorToMultiple<Pow2<T>> for L
where
    L: IntAtLeastAsWide<T>,
    T: UnsignedInt,
{
    type Output = Self;

    #[inline(always)]
    fn floor_to_multiple(self, rhs: Pow2<T>) -> Self::Output {
        // SAFETY: SafePow2 guarantees a valid shift
        unsafe {
            self.unchecked_shr(rhs.exponent as u32)
                .unchecked_shl(rhs.exponent as u32)
        }
    }
}

make_func_trait!(CheckedFloorToMultiple, checked_floor_to_multiple);

impl<T> CheckedFloorToMultiple<UnboundedPow2> for T
where
    T: Int,
{
    type Output = Option<Self>;

    #[inline(always)]
    fn checked_floor_to_multiple(self, rhs: UnboundedPow2) -> Self::Output {
        rhs.is_safe::<T::Unsigned>()
            .then(|| floor_to_multiple(self, rhs))
    }
}

make_func_trait!(UnboundedFloorToMultiple, unbounded_floor_to_multiple);

impl_trait_signed_unsigned!(
    UnboundedFloorToMultiple<UnboundedPow2>,
    signed_body {
        type Output = Option<Self>;

        fn unbounded_floor_to_multiple(self, rhs: UnboundedPow2) -> Self::Output {
            debug_assert!(Self::IS_SIGNED);
            if rhs.is_safe::<<Self as Int>::Unsigned>() {
                Some(floor_to_multiple(self, rhs))
            } else if self >= Self::ZERO {
                Some(Self::ZERO)
            } else {
                None
            }
        }
    },
    unsigned_body {
        type Output = Self;

        fn unbounded_floor_to_multiple(self, rhs: UnboundedPow2) -> Self::Output {
            debug_assert!(Self::IS_UNSIGNED);
            if rhs.is_safe::<Self>() {
                floor_to_multiple(self, rhs)
            } else {
                Self::ZERO
            }
        }
    }
);

make_func_trait!(CeilToMultiple, ceil_to_multiple);

impl<T> CeilToMultiple<UnboundedPow2> for T
where
    T: Int,
{
    type Output = Self;

    #[inline(always)]
    fn ceil_to_multiple(self, rhs: UnboundedPow2) -> Self::Output {
        debug_assert!(rhs.is_safe::<T::Unsigned>());
        // We can actually use the mask method here because if the intermediate `a + mask` overflows
        // then the actual result would overflow too.
        let mask = T::mask(rhs.exponent as u32);
        (self + mask) & !mask
    }
}

impl<L, T> CeilToMultiple<Pow2<T>> for L
where
    L: IntAtLeastAsWide<T>,
    T: UnsignedInt,
{
    type Output = Self;

    #[inline(always)]
    fn ceil_to_multiple(self, rhs: Pow2<T>) -> Self::Output {
        // We can actually use the mask method here because if the intermediate `a + mask` overflows
        // then the actual result would overflow too.
        // SAFETY: SafePow2 guarantees a valid shift
        let mask = unsafe { Self::unchecked_mask(rhs.exponent as u32) };
        (self + mask) & !mask
    }
}

make_func_trait!(CheckedCeilToMultiple, checked_ceil_to_multiple);

impl<T> CheckedCeilToMultiple<UnboundedPow2> for T
where
    T: Int,
{
    type Output = Option<Self>;

    #[inline(always)]
    fn checked_ceil_to_multiple(self, rhs: UnboundedPow2) -> Self::Output {
        if rhs.is_safe::<T::Unsigned>() {
            let mask = T::mask(rhs.exponent as u32);
            Some(self.checked_add(mask)? & !mask)
        } else {
            None
        }
    }
}

impl<L, T> CheckedCeilToMultiple<Pow2<T>> for L
where
    L: IntAtLeastAsWide<T>,
    T: UnsignedInt,
{
    type Output = Option<Self>;

    #[inline(always)]
    fn checked_ceil_to_multiple(self, rhs: Pow2<T>) -> Self::Output {
        // SAFETY: SafePow2 guarantees a valid shift
        let mask = unsafe { Self::unchecked_mask(rhs.exponent as u32) };
        Some(self.checked_add(mask)? & !mask)
    }
}

make_func_trait!(UnboundedCeilToMultiple, unbounded_ceil_to_multiple);

impl<T> UnboundedCeilToMultiple<UnboundedPow2> for T
where
    T: Int,
{
    type Output = Option<Self>;

    #[inline(always)]
    fn unbounded_ceil_to_multiple(self, rhs: UnboundedPow2) -> Self::Output {
        if rhs.is_safe::<T::Unsigned>() {
            checked_ceil_to_multiple(self, rhs)
        } else if self <= T::ZERO {
            Some(T::ZERO)
        } else {
            None
        }
    }
}

make_func_trait!(RoundToMultiple, round_to_multiple);

impl_trait_signed_unsigned!(
    RoundToMultiple<UnboundedPow2>,
    signed_body {
        type Output = Self;

        #[inline(always)]
        fn round_to_multiple(self, rhs: UnboundedPow2) -> Self::Output {
            debug_assert!(Self::IS_SIGNED);
            debug_assert!(rhs.is_safe::<<Self as Int>::Unsigned>());
            let mask = Self::mask(rhs.exponent as u32);
            let bit = Self::self_from_unsigned(
                // Bias:
                <Self as Int>::Unsigned::highest_mask_bit(rhs.exponent as u32)
                // We bias the bias for negative lhs to get correct rounding away from zero.
                // Saturating sub must be against zero (so unsigned)
                .saturating_sub(
                    <Self as Int>::Unsigned::from_bool(self < Self::ZERO)
                )
            );

            // We can actually use the mask method here because if the intermediate `a + mask` overflows
            // then the actual result would overflow too.
            (self + bit) & !mask
        }
    },
    unsigned_body {
        type Output = Self;

        #[inline(always)]
        fn round_to_multiple(self, rhs: UnboundedPow2) -> Self::Output {
            debug_assert!(Self::IS_UNSIGNED);
            debug_assert!(rhs.is_safe::<<Self as Int>::Unsigned>());
            let mask = Self::mask(rhs.exponent as u32);
            let bit = Self::highest_mask_bit(rhs.exponent as u32);
            // We can actually use the mask method here because if the intermediate `a + mask` overflows
            // then the actual result would overflow too.
            (self + bit) & !mask
        }
    }
);

impl_generic_trait_signed_unsigned!(
    <T> RoundToMultiple<Pow2<T>> where { T: UnsignedInt, Self: IntAtLeastAsWide<T> },
    signed_body {
        type Output = Self;

        #[inline(always)]
        fn round_to_multiple(self, rhs: Pow2<T>) -> Self {
            debug_assert!(Self::IS_SIGNED);
            // SAFETY: SafePow2 guarantees a valid shift
            let mask = unsafe { Self::unchecked_mask(rhs.exponent as u32) };
            let bit = Self::self_from_unsigned(
                // Bias:
                // SAFETY: SafePow2 guarantees a valid shift
                unsafe { <Self as Int>::Unsigned::unchecked_highest_mask_bit(rhs.exponent as u32) }
                // We bias the bias for negative lhs to get correct rounding away from zero.
                // Saturating sub must be against zero (so unsigned)
                .saturating_sub(
                    <Self as Int>::Unsigned::from_bool(self < Self::ZERO)
                )
            );

            // We can actually use the mask method here because if the intermediate `a + mask` overflows
            // then the actual result would overflow too.
            (self + bit) & !mask
        }
    },
    unsigned_body {
        type Output = Self;

        #[inline(always)]
        fn round_to_multiple(self, rhs: Pow2<T>) -> Self::Output {
            debug_assert!(Self::IS_UNSIGNED);
            // SAFETY: SafePow2 guarantees a valid shift
            let mask = unsafe { Self::unchecked_mask(rhs.exponent as u32) };
            // SAFETY: SafePow2 guarantees a valid shift
            let bit = unsafe { Self::unchecked_highest_mask_bit(rhs.exponent as u32) };
            // We can actually use the mask method here because if the intermediate `a + mask` overflows
            // then the actual result would overflow too.
            (self + bit) & !mask
        }
    }
);

make_func_trait!(CheckedRoundToMultiple, checked_round_to_multiple);

impl_trait_signed_unsigned!(
    CheckedRoundToMultiple<UnboundedPow2>,
    signed_body {
        type Output = Option<Self>;

        #[inline(always)]
        fn checked_round_to_multiple(self, rhs: UnboundedPow2) -> Self::Output {
            debug_assert!(Self::IS_SIGNED);
            if !rhs.is_safe::<<Self as Int>::Unsigned>() {
                return None;
            }

            let mask = Self::mask(rhs.exponent as u32);
            let bit = Self::self_from_unsigned(
                // Bias:
                <Self as Int>::Unsigned::highest_mask_bit(rhs.exponent as u32)
                // We bias the bias for negative lhs to get correct rounding away from zero.
                // Saturating sub must be against zero (so unsigned)
                .saturating_sub(
                    <Self as Int>::Unsigned::from_bool(self < Self::ZERO)
                )
            );

            Some(self.checked_add(bit)? & !mask)
        }
    },
    unsigned_body {
        type Output = Option<Self>;

        #[inline(always)]
        fn checked_round_to_multiple(self, rhs: UnboundedPow2) -> Self::Output {
            debug_assert!(Self::IS_UNSIGNED);
            if !rhs.is_safe::<<Self as Int>::Unsigned>() {
                return None;
            }

            let mask = Self::mask(rhs.exponent as u32);
            let bit = Self::highest_mask_bit(rhs.exponent as u32);

            Some(self.checked_add(bit)? & !mask)
        }
    }
);

impl_generic_trait_signed_unsigned!(
    <T> CheckedRoundToMultiple<Pow2<T>> where { T: UnsignedInt, Self: IntAtLeastAsWide<T> },
    signed_body {
        type Output = Option<Self>;

        #[inline(always)]
        fn checked_round_to_multiple(self, rhs: Pow2<T>) -> Self::Output {
            debug_assert!(Self::IS_SIGNED);
            // SAFETY: SafePow2 guarantees a valid shift
            let mask = unsafe { Self::unchecked_mask(rhs.exponent as u32) };
            let bit = Self::self_from_unsigned(
                // Bias:
                // SAFETY: SafePow2 guarantees a valid shift
                unsafe { <Self as Int>::Unsigned::unchecked_highest_mask_bit(rhs.exponent as u32) }
                // We bias the bias for negative lhs to get correct rounding away from zero.
                // Saturating sub must be against zero (so unsigned)
                .saturating_sub(
                    <Self as Int>::Unsigned::from_bool(self < Self::ZERO)
                )
            );

            Some(self.checked_add(bit)? & !mask)
        }
    },
    unsigned_body {
        type Output = Option<Self>;

        #[inline(always)]
        fn checked_round_to_multiple(self, rhs: Pow2<T>) -> Self::Output {
            debug_assert!(Self::IS_UNSIGNED);
            // SAFETY: SafePow2 guarantees a valid shift
            let mask = unsafe { Self::unchecked_mask(rhs.exponent as u32) };
            // SAFETY: SafePow2 guarantees a valid shift
            let bit = unsafe { Self::unchecked_highest_mask_bit(rhs.exponent as u32) };

            Some(self.checked_add(bit)? & !mask)
        }
    }
);

make_func_trait!(UnboundedRoundToMultiple, unbounded_round_to_multiple);

impl<T> UnboundedRoundToMultiple<UnboundedPow2> for T
where
    T: Int + CheckedRoundToMultiple<UnboundedPow2, Output = Option<Self>>,
{
    type Output = Option<Self>;

    #[inline(always)]
    fn unbounded_round_to_multiple(self, rhs: UnboundedPow2) -> Self::Output {
        if rhs.is_safe::<T::Unsigned>() {
            checked_round_to_multiple(self, rhs)
        } else if Self::IS_SIGNED && self == Self::MIN && rhs.exponent as u32 == Self::BITS {
            // would round to twice Self::MIN
            None
        } else if Self::IS_UNSIGNED
            && self >= ((Self::MAX >> 1) + Self::ONE)
            && rhs.exponent as u32 == Self::BITS
        {
            // would round to Self::MAX + 1
            None
        } else {
            Some(T::ZERO)
        }
    }
}

#[allow(nonstandard_style)]
pub mod __detached_docs {
    macro_rules! colored_link {
        ($name:ident, $color:literal) => {
            concat!(
                "[<font color=\"",
                $color,
                "\">",
                stringify!($name),
                "</font>]",
                "(crate::",
                stringify!($name),
                ")"
            )
        };
    }

    macro_rules! trait_link {
        ($trait_name:ident) => {
            colored_link!($trait_name, "#b78cf2")
        };
    }

    macro_rules! struct_link {
        ($struct_name:ident) => {
            colored_link!($struct_name, "#2dbfb8")
        };
    }

    macro_rules! fn_link {
        ($fn_name:ident) => {
            colored_link!($fn_name, "#2bab63")
        };
    }

    #[rustfmt::skip]
    macro_rules! trait_code_header_pow2 {
        ($trait_name:ident, $func_name:ident, [$($additional_trait_bound:ident),*]) => {
            concat!(
                "<span><pre>",
                "impl&lt;L, T&gt; ", trait_link!($trait_name), "&lt;", struct_link!(Pow2), "&lt;T&gt;&gt; for L<br/>",
                "where<br/>",
                "&nbsp;&nbsp;&nbsp;&nbsp;L: ", trait_link!(IntAtLeastAsWide), "&lt;T&gt;", $(" + ", trait_link!($additional_trait_bound), "&lt;", struct_link!(Pow2), "&lt;T&gt;, Output = T&gt;",)* ",<br/>",
                "&nbsp;&nbsp;&nbsp;&nbsp;T: ", trait_link!(UnsignedInt), "<br/>",
                "fn ", fn_link!($func_name), "(self, rhs: ", struct_link!(Pow2), "&lt;T&gt;) -> Self::Output",
                "</pre></span><hr/>"
            )
        };
    }

    #[rustfmt::skip]
    macro_rules! trait_code_header_unb_pow2 {
        ($trait_name:ident, $func_name:ident, [$($additional_trait_bound:ident),*]) => {
            concat!(
                "<span><pre>",
                "impl&lt;T&gt; ", trait_link!($trait_name), "&lt;", struct_link!(UnboundedPow2), "&gt; for L<br/>",
                "where<br/>",
                "&nbsp;&nbsp;&nbsp;&nbsp;L: ", trait_link!(Int), $(" + ", trait_link!($additional_trait_bound), "&lt;", struct_link!(UnboundedPow2), ", Output = T&gt;",)* ",<br/>",
                "fn ", fn_link!($func_name), "(self, rhs: ", struct_link!(UnboundedPow2), ") -> Self::Output",
                "</pre></span><hr/>"
            )
        };
    }

    pub mod Pow2 {
        #[doc = trait_code_header_pow2!(CheckedMul, checked_mul, [])]
        /// Attempts to multiply `self` by `rhs`.
        /// Avoids integer multiplication by using bitwise arithmetic.
        /// 
        /// Returns `None` if the result overflows,
        /// `Some(result)` otherwise.
        pub mod CheckedMul {}
    }

    pub mod UnboundedPow2 {
        #[doc = trait_code_header_unb_pow2!(CheckedMul, checked_mul, [])]
        /// Attempts to multiply `self` by `rhs`.
        /// Avoids integer multiplication by using bitwise arithmetic.
        /// 
        /// Returns `None` if the result overflows or the exponent is too large,
        /// `Some(result)` otherwise.
        pub mod CheckedMul {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Write;
    use std::hash::{DefaultHasher, Hash, Hasher};

    #[test]
    fn unb_pow2_constructible_from_any_u8_exponent() {
        for e in 0_u8..=u8::MAX {
            assert_eq!(UnboundedPow2::from_exponent(e).exponent(), e);
        }
    }

    #[test]
    fn unb_pow2_constructible_from_align_of() {
        assert_eq!(
            UnboundedPow2::align_of::<()>(),
            UnboundedPow2::from_exponent(0)
        );
        assert_eq!(
            UnboundedPow2::align_of::<u8>(),
            UnboundedPow2::from_exponent(0)
        );
        assert_eq!(
            UnboundedPow2::align_of::<u32>(),
            UnboundedPow2::from_exponent(2)
        );
        assert_eq!(
            UnboundedPow2::align_of::<u128>(),
            UnboundedPow2::from_exponent(4)
        );
    }

    #[test]
    fn unb_pow2_constructible_from_align_of_val() {
        assert_eq!(
            UnboundedPow2::align_of_val(&()),
            UnboundedPow2::from_exponent(0)
        );
        assert_eq!(
            UnboundedPow2::align_of_val(&0_u8),
            UnboundedPow2::from_exponent(0)
        );
        assert_eq!(
            UnboundedPow2::align_of_val(&0_u32),
            UnboundedPow2::from_exponent(2)
        );
        assert_eq!(
            UnboundedPow2::align_of_val(&0_u128),
            UnboundedPow2::from_exponent(4)
        );
    }

    #[test]
    fn unb_pow2_try_from_power_of_two_int() {
        assert_eq!(
            UnboundedPow2::try_from(32_i8),
            Ok(UnboundedPow2::from_exponent(5))
        );
        assert_eq!(
            UnboundedPow2::try_from(32_u8),
            Ok(UnboundedPow2::from_exponent(5))
        );
        assert_eq!(
            UnboundedPow2::try_from(32_i16),
            Ok(UnboundedPow2::from_exponent(5))
        );
        assert_eq!(
            UnboundedPow2::try_from(32_u16),
            Ok(UnboundedPow2::from_exponent(5))
        );
        assert_eq!(
            UnboundedPow2::try_from(32_i32),
            Ok(UnboundedPow2::from_exponent(5))
        );
        assert_eq!(
            UnboundedPow2::try_from(32_u32),
            Ok(UnboundedPow2::from_exponent(5))
        );
        assert_eq!(
            UnboundedPow2::try_from(32_i64),
            Ok(UnboundedPow2::from_exponent(5))
        );
        assert_eq!(
            UnboundedPow2::try_from(32_u64),
            Ok(UnboundedPow2::from_exponent(5))
        );
        assert_eq!(
            UnboundedPow2::try_from(32_i128),
            Ok(UnboundedPow2::from_exponent(5))
        );
        assert_eq!(
            UnboundedPow2::try_from(32_u128),
            Ok(UnboundedPow2::from_exponent(5))
        );
        assert_eq!(
            UnboundedPow2::try_from(32_isize),
            Ok(UnboundedPow2::from_exponent(5))
        );
        assert_eq!(
            UnboundedPow2::try_from(32_usize),
            Ok(UnboundedPow2::from_exponent(5))
        );
    }

    #[test]
    fn unb_pow2_try_from_near_power_of_two() {
        assert_eq!(UnboundedPow2::try_from(31_i8), Err(NotPow2));
        assert_eq!(UnboundedPow2::try_from(33_i8), Err(NotPow2));
    }

    #[test]
    fn unb_pow2_try_from_zero() {
        assert_eq!(UnboundedPow2::try_from(0_i8), Err(NotPow2));
    }

    #[test]
    fn unb_pow2_try_from_negative() {
        assert_eq!(UnboundedPow2::try_from(-1_i8), Err(NotPow2));
    }

    #[test]
    fn unb_pow2_try_from_min() {
        assert_eq!(UnboundedPow2::try_from(i64::MIN), Err(NotPow2));
    }

    #[test]
    fn unb_pow2_try_from_max() {
        assert_eq!(UnboundedPow2::try_from(i64::MAX), Err(NotPow2));
    }

    #[test]
    fn unb_pow2_from_pow2() {
        assert_eq!(
            UnboundedPow2::from(Pow2::<u32>::from_exponent(13).unwrap()).exponent(),
            13
        );
        assert_eq!(
            UnboundedPow2::from(Pow2::<u128>::from_exponent(127).unwrap()).exponent(),
            127
        );
    }

    #[test]
    fn pow2_try_from_pow2() {
        assert_eq!(
            Pow2::<u32>::try_from(UnboundedPow2::from_exponent(123)),
            Err(Pow2OutOfRange)
        );
        assert_eq!(
            Pow2::<u64>::try_from(UnboundedPow2::from_exponent(123)),
            Err(Pow2OutOfRange)
        );
        assert_eq!(
            Pow2::<u128>::try_from(UnboundedPow2::from_exponent(123)),
            Ok(Pow2::<u128>::from_exponent(123).unwrap())
        );
    }

    #[test]
    fn unb_pow2_as_int_all_types() {
        let p = UnboundedPow2::from_exponent(6);
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
    fn unb_pow2_as_int_max_fitting() {
        assert_eq!(UnboundedPow2::from_exponent(6).as_i8(), 1 << 6);
        assert_eq!(UnboundedPow2::from_exponent(7).as_u8(), 1 << 7);
        assert_eq!(UnboundedPow2::from_exponent(14).as_i16(), 1 << 14);
        assert_eq!(UnboundedPow2::from_exponent(15).as_u16(), 1 << 15);
        assert_eq!(UnboundedPow2::from_exponent(30).as_i32(), 1 << 30);
        assert_eq!(UnboundedPow2::from_exponent(31).as_u32(), 1 << 31);
        assert_eq!(UnboundedPow2::from_exponent(62).as_i64(), 1 << 62);
        assert_eq!(UnboundedPow2::from_exponent(63).as_u64(), 1 << 63);
        assert_eq!(UnboundedPow2::from_exponent(126).as_i128(), 1 << 126);
        assert_eq!(UnboundedPow2::from_exponent(127).as_u128(), 1 << 127);
    }

    #[test]
    fn unb_pow2_to_int() {
        assert_eq!(i8::try_from(UnboundedPow2::from_exponent(6)), Ok(1 << 6));
        assert_eq!(
            i8::try_from(UnboundedPow2::from_exponent(7)),
            Err(Pow2OutOfRange)
        );

        assert_eq!(u8::try_from(UnboundedPow2::from_exponent(7)), Ok(1 << 7));
        assert_eq!(
            u8::try_from(UnboundedPow2::from_exponent(8)),
            Err(Pow2OutOfRange)
        );

        assert_eq!(i16::try_from(UnboundedPow2::from_exponent(14)), Ok(1 << 14));
        assert_eq!(
            i16::try_from(UnboundedPow2::from_exponent(15)),
            Err(Pow2OutOfRange)
        );

        assert_eq!(u16::try_from(UnboundedPow2::from_exponent(15)), Ok(1 << 15));
        assert_eq!(
            u16::try_from(UnboundedPow2::from_exponent(16)),
            Err(Pow2OutOfRange)
        );

        assert_eq!(i32::try_from(UnboundedPow2::from_exponent(30)), Ok(1 << 30));
        assert_eq!(
            i32::try_from(UnboundedPow2::from_exponent(31)),
            Err(Pow2OutOfRange)
        );

        assert_eq!(u32::try_from(UnboundedPow2::from_exponent(31)), Ok(1 << 31));
        assert_eq!(
            u32::try_from(UnboundedPow2::from_exponent(32)),
            Err(Pow2OutOfRange)
        );

        assert_eq!(i64::try_from(UnboundedPow2::from_exponent(62)), Ok(1 << 62));
        assert_eq!(
            i64::try_from(UnboundedPow2::from_exponent(63)),
            Err(Pow2OutOfRange)
        );

        assert_eq!(u64::try_from(UnboundedPow2::from_exponent(63)), Ok(1 << 63));
        assert_eq!(
            u64::try_from(UnboundedPow2::from_exponent(64)),
            Err(Pow2OutOfRange)
        );

        assert_eq!(
            i128::try_from(UnboundedPow2::from_exponent(126)),
            Ok(1 << 126)
        );
        assert_eq!(
            i128::try_from(UnboundedPow2::from_exponent(127)),
            Err(Pow2OutOfRange)
        );

        assert_eq!(
            u128::try_from(UnboundedPow2::from_exponent(127)),
            Ok(1 << 127)
        );
        assert_eq!(
            u128::try_from(UnboundedPow2::from_exponent(128)),
            Err(Pow2OutOfRange)
        );
    }

    #[test]
    fn pow2_from_exponent() {
        let v = Pow2::<u8>::from_exponent(7);
        assert!(v.is_ok());
        assert_eq!(v.unwrap().exponent(), 7);

        let v = Pow2::<u8>::from_exponent(8);
        assert!(v.is_err());

        let v = Pow2::<u128>::from_exponent(127);
        assert!(v.is_ok());
        assert_eq!(v.unwrap().exponent(), 127);

        let v = Pow2::<u128>::from_exponent(128);
        assert!(v.is_err());
    }

    #[test]
    fn pow2_value() {
        let v = Pow2::<u8>::from_exponent(0);
        assert!(v.is_ok());
        assert_eq!(v.unwrap().value(), 1);

        let v = Pow2::<u8>::from_exponent(7);
        assert!(v.is_ok());
        assert_eq!(v.unwrap().value(), 128);
    }

    #[test]
    fn unb_pow2_mul() {
        let lhs = UnboundedPow2::from_exponent(6);
        let rhs = UnboundedPow2::from_exponent(7);
        assert_eq!(lhs * rhs, UnboundedPow2::from_exponent(13));
        assert_eq!(rhs * lhs, UnboundedPow2::from_exponent(13));
    }

    #[test]
    fn pow2_from_other() {
        assert_eq!(
            Pow2::<u16>::from(Pow2::<u8>::from_exponent(7).unwrap()),
            Pow2::<u16>::from_exponent(7).unwrap()
        );
        assert_eq!(
            Pow2::<u32>::from(Pow2::<u8>::from_exponent(7).unwrap()),
            Pow2::<u32>::from_exponent(7).unwrap()
        );
        assert_eq!(
            Pow2::<u64>::from(Pow2::<u8>::from_exponent(7).unwrap()),
            Pow2::<u64>::from_exponent(7).unwrap()
        );
        assert_eq!(
            Pow2::<u64>::from(Pow2::<u32>::from_exponent(31).unwrap()),
            Pow2::<u64>::from_exponent(31).unwrap()
        );

        /* // should not compile
        assert_eq!(Pow2::<u32>::from(Pow2::<u64>::from_exponent(31).unwrap()), Pow2::<u32>::from_exponent(31).unwrap());
        */
    }

    #[test]
    fn pow2_try_from_other() {
        assert_eq!(
            Pow2::<u8>::try_from(Pow2::<u16>::from_exponent(7).unwrap()),
            Ok(Pow2::<u8>::from_exponent(7).unwrap())
        );
        assert_eq!(
            Pow2::<u8>::try_from(Pow2::<u16>::from_exponent(8).unwrap()),
            Err(Pow2OutOfRange)
        );
        assert_eq!(
            Pow2::<u8>::try_from(Pow2::<u32>::from_exponent(7).unwrap()),
            Ok(Pow2::<u8>::from_exponent(7).unwrap())
        );
        assert_eq!(
            Pow2::<u8>::try_from(Pow2::<u64>::from_exponent(7).unwrap()),
            Ok(Pow2::<u8>::from_exponent(7).unwrap())
        );
        assert_eq!(
            Pow2::<u32>::try_from(Pow2::<u128>::from_exponent(31).unwrap()),
            Ok(Pow2::<u32>::from_exponent(31).unwrap())
        );
        assert_eq!(
            Pow2::<u32>::try_from(Pow2::<u128>::from_exponent(32).unwrap()),
            Err(Pow2OutOfRange)
        );
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn unb_pow2_mul_overflow() {
        let lhs = UnboundedPow2::from_exponent(128);
        let rhs = UnboundedPow2::from_exponent(128);
        let _ = lhs * rhs;
    }

    #[test]
    fn unb_pow2_mul_assign() {
        let lhs = UnboundedPow2::from_exponent(6);
        let rhs = UnboundedPow2::from_exponent(7);
        let mut new_lhs = lhs;
        new_lhs *= rhs;
        assert_eq!(new_lhs, UnboundedPow2::from_exponent(13));

        let mut new_rhs = rhs;
        new_rhs *= lhs;
        assert_eq!(new_rhs, UnboundedPow2::from_exponent(13));
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn unb_pow2_mul_assign_overflow() {
        let mut lhs = UnboundedPow2::from_exponent(128);
        let rhs = UnboundedPow2::from_exponent(128);
        lhs *= rhs;
    }

    #[test]
    fn unb_pow2_mul_zero() {
        let lhs = UnboundedPow2::from_exponent(65);
        let rhs = UnboundedPow2::from_exponent(0);
        assert_eq!(lhs * rhs, lhs);
        assert_eq!(rhs * lhs, lhs);
    }

    #[test]
    fn unb_pow2_mul_assign_zero() {
        let lhs = UnboundedPow2::from_exponent(65);
        let rhs = UnboundedPow2::from_exponent(0);
        let mut new_lhs = lhs;
        new_lhs *= rhs;
        assert_eq!(new_lhs, lhs);

        let mut new_rhs = rhs;
        new_rhs *= lhs;
        assert_eq!(new_rhs, lhs);
    }

    #[test]
    fn unb_pow2_checked_mul() {
        assert_eq!(
            UnboundedPow2::from_exponent(12).checked_mul(UnboundedPow2::from_exponent(13)),
            Some(UnboundedPow2::from_exponent(25))
        );
        assert_eq!(
            UnboundedPow2::from_exponent(12).checked_mul(UnboundedPow2::from_exponent(0)),
            Some(UnboundedPow2::from_exponent(12))
        );
        assert_eq!(
            UnboundedPow2::from_exponent(0).checked_mul(UnboundedPow2::from_exponent(13)),
            Some(UnboundedPow2::from_exponent(13))
        );
    }

    #[test]
    fn unb_pow2_checked_mul_boundary() {
        assert_eq!(
            UnboundedPow2::from_exponent(254).checked_mul(UnboundedPow2::from_exponent(1)),
            Some(UnboundedPow2::from_exponent(255))
        );
        assert_eq!(
            UnboundedPow2::from_exponent(254).checked_mul(UnboundedPow2::from_exponent(2)),
            None
        );
    }

    #[test]
    fn pow2_checked_mul_boundary() {
        assert_eq!(
            Pow2::<u32>::from_exponent(12)
                .unwrap()
                .checked_mul(Pow2::<u32>::from_exponent(19).unwrap()),
            Some(Pow2::<u32>::from_exponent(31).unwrap())
        );
        assert_eq!(
            Pow2::<u32>::from_exponent(12)
                .unwrap()
                .checked_mul(Pow2::<u32>::from_exponent(20).unwrap()),
            None
        );
    }

    #[test]
    fn unb_pow2_saturating_mul() {
        assert_eq!(
            UnboundedPow2::from_exponent(13).saturating_mul(UnboundedPow2::from_exponent(14)),
            UnboundedPow2::from_exponent(27)
        );
        assert_eq!(
            UnboundedPow2::from_exponent(13).saturating_mul(UnboundedPow2::from_exponent(0)),
            UnboundedPow2::from_exponent(13)
        );
        assert_eq!(
            UnboundedPow2::from_exponent(0).saturating_mul(UnboundedPow2::from_exponent(14)),
            UnboundedPow2::from_exponent(14)
        );
    }

    #[test]
    fn unb_pow2_saturating_mul_boundary() {
        assert_eq!(
            UnboundedPow2::from_exponent(254).saturating_mul(UnboundedPow2::from_exponent(1)),
            UnboundedPow2::from_exponent(255)
        );
        assert_eq!(
            UnboundedPow2::from_exponent(254).saturating_mul(UnboundedPow2::from_exponent(2)),
            UnboundedPow2::from_exponent(255)
        );
    }

    #[test]
    fn pow2_saturating_mul_boundary() {
        assert_eq!(
            Pow2::<u32>::from_exponent(12)
                .unwrap()
                .saturating_mul(Pow2::<u32>::from_exponent(18).unwrap()),
            Pow2::<u32>::from_exponent(30).unwrap()
        );
        assert_eq!(
            Pow2::<u32>::from_exponent(12)
                .unwrap()
                .saturating_mul(Pow2::<u32>::from_exponent(19).unwrap()),
            Pow2::<u32>::from_exponent(31).unwrap()
        );
        assert_eq!(
            Pow2::<u32>::from_exponent(12)
                .unwrap()
                .saturating_mul(Pow2::<u32>::from_exponent(20).unwrap()),
            Pow2::<u32>::from_exponent(31).unwrap()
        );
    }

    #[test]
    fn unb_pow2_div_positive() {
        let lhs = UnboundedPow2::from_exponent(6);
        let rhs = UnboundedPow2::from_exponent(3);
        assert_eq!(lhs / rhs, UnboundedPow2::from_exponent(3));
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn unb_pow2_div_overflow() {
        let lhs = UnboundedPow2::from_exponent(127);
        let rhs = UnboundedPow2::from_exponent(128);
        let _ = lhs / rhs;
    }

    #[test]
    fn unb_pow2_div_assign_positive() {
        let mut lhs = UnboundedPow2::from_exponent(6);
        let rhs = UnboundedPow2::from_exponent(3);
        lhs /= rhs;
        assert_eq!(lhs, UnboundedPow2::from_exponent(3));
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn unb_pow2_div_assign_overflow() {
        let mut lhs = UnboundedPow2::from_exponent(127);
        let rhs = UnboundedPow2::from_exponent(128);
        lhs /= rhs;
    }

    #[test]
    fn unb_pow2_checked_div_overflow() {
        let lhs = UnboundedPow2::from_exponent(3);
        let rhs = UnboundedPow2::from_exponent(6);
        assert_eq!(lhs.checked_div(rhs), None);
        assert_eq!(rhs.checked_div(lhs), Some(UnboundedPow2::from_exponent(3)));
    }

    #[test]
    fn pow2_checked_div_overflow() {
        let lhs = Pow2::<u32>::from_exponent(3).unwrap();
        let rhs = Pow2::<u32>::from_exponent(6).unwrap();
        assert_eq!(lhs.checked_div(rhs), None);
        assert_eq!(
            rhs.checked_div(lhs),
            Some(Pow2::<u32>::from_exponent(3).unwrap())
        );
    }

    #[test]
    fn unb_pow2_saturating_div_overflow() {
        let lhs = UnboundedPow2::from_exponent(3);
        let rhs = UnboundedPow2::from_exponent(6);
        assert_eq!(lhs.saturating_div(rhs), UnboundedPow2::from_exponent(0));
        assert_eq!(rhs.saturating_div(lhs), UnboundedPow2::from_exponent(3));
    }

    #[test]
    fn pow2_saturating_div_overflow() {
        let lhs = Pow2::<u32>::from_exponent(3).unwrap();
        let rhs = Pow2::<u32>::from_exponent(6).unwrap();
        assert_eq!(
            lhs.saturating_div(rhs),
            Pow2::<u32>::from_exponent(0).unwrap()
        );
        assert_eq!(
            rhs.saturating_div(lhs),
            Pow2::<u32>::from_exponent(3).unwrap()
        );
    }

    #[test]
    fn unb_pow2_div_one() {
        let lhs = UnboundedPow2::from_exponent(3);
        let rhs = UnboundedPow2::from_exponent(0);
        assert_eq!(lhs / rhs, lhs);
    }

    #[test]
    fn unb_pow2_div_assign_one() {
        let lhs = UnboundedPow2::from_exponent(3);
        let rhs = UnboundedPow2::from_exponent(0);
        let mut new_lhs = lhs;
        new_lhs /= rhs;
        assert_eq!(new_lhs, lhs);
    }

    #[test]
    fn unb_pow2_div_self() {
        let v = UnboundedPow2::from_exponent(123);
        assert_eq!(v / v, UnboundedPow2::from_exponent(0));
    }

    #[test]
    fn unb_pow2_div_assign_self() {
        let mut v = UnboundedPow2::from_exponent(123);
        v /= v;
        assert_eq!(v, UnboundedPow2::from_exponent(0));
    }

    #[test]
    fn unb_pow2_int_mul_one() {
        assert_eq!(123 * UnboundedPow2::from_exponent(0), 123);
        assert_eq!(-123 * UnboundedPow2::from_exponent(0), -123);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn unb_pow2_int_mul_overflow() {
        let _ = 123_i32 * UnboundedPow2::from_exponent(27);
    }

    #[test]
    fn unb_pow2_int_mul_assign_one() {
        let mut v = 123;
        v *= UnboundedPow2::from_exponent(0);
        assert_eq!(v, 123);

        let mut v = -123;
        v *= UnboundedPow2::from_exponent(0);
        assert_eq!(v, -123);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn unb_pow2_int_mul_assign_overflow() {
        let mut lhs = 123_i32;
        lhs *= UnboundedPow2::from_exponent(27);
    }

    #[test]
    fn unb_pow2_int_mul() {
        assert_eq!(123_i32 * UnboundedPow2::from_exponent(15), 123 << 15);
        assert_eq!(-123_i32 * UnboundedPow2::from_exponent(15), -123 << 15);
        assert_eq!(123_i64 * UnboundedPow2::from_exponent(32), 123 << 32);
        assert_eq!(-123_i64 * UnboundedPow2::from_exponent(32), -123 << 32);
        assert_eq!(123_u64 * UnboundedPow2::from_exponent(46), 123 << 46);
    }

    #[test]
    fn unb_pow2_int_mul_assign() {
        let mut v = 123_i32;
        v *= UnboundedPow2::from_exponent(15);
        assert_eq!(v, 123 << 15);

        let mut v = -123_i32;
        v *= UnboundedPow2::from_exponent(15);
        assert_eq!(v, -123 << 15);

        let mut v = 123_i64;
        v *= UnboundedPow2::from_exponent(32);
        assert_eq!(v, 123 << 32);

        let mut v = -123_i64;
        v *= UnboundedPow2::from_exponent(32);
        assert_eq!(v, -123 << 32);

        let mut v = 123_u64;
        v *= UnboundedPow2::from_exponent(46);
        assert_eq!(v, 123 << 46);
    }

    #[test]
    fn unb_pow2_int_checked_mul_boundary() {
        assert_eq!(
            checked_mul(i32::MAX as u32 + 1, UnboundedPow2::from_exponent(1)),
            None
        );
        assert_eq!(
            checked_mul(i32::MAX as u32, UnboundedPow2::from_exponent(1)),
            Some(u32::MAX - 1)
        );
    }

    #[test]
    fn pow2_int_checked_mul_boundary() {
        assert_eq!(
            checked_mul(i32::MAX as u32 + 1, Pow2::<u32>::from_exponent(1).unwrap()),
            None
        );
        assert_eq!(
            checked_mul(i32::MAX as u32 + 1, Pow2::<u8>::from_exponent(1).unwrap()),
            None
        );
        assert_eq!(
            checked_mul(i32::MAX as u32, Pow2::<u32>::from_exponent(1).unwrap()),
            Some(u32::MAX - 1)
        );

        /* // Should not compile.
        assert_eq!(
            checked_mul(i32::MAX as u32, SafePow2::<u64>::from_exponent(1).unwrap()),
            Some(u32::MAX - 1)
        );
        */
    }

    #[test]
    fn unb_pow2_int_div_exact() {
        assert_eq!(32u64 / UnboundedPow2::from_exponent(5), 1);
        assert_eq!(64u64 / UnboundedPow2::from_exponent(3), 8);
        assert_eq!(-64i64 / UnboundedPow2::from_exponent(3), -8);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn unb_pow2_int_div_divisor_out_of_range() {
        let _ = 123_i32 / UnboundedPow2::from_exponent(32);
    }

    #[test]
    fn unb_pow2_int_div_assign_exact() {
        let mut v = 32u64;
        v /= UnboundedPow2::from_exponent(5);
        assert_eq!(v, 1);

        let mut v = 64u64;
        v /= UnboundedPow2::from_exponent(3);
        assert_eq!(v, 8);

        let mut v = -64i64;
        v /= UnboundedPow2::from_exponent(3);
        assert_eq!(v, -8);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn unb_pow2_int_div_assign_divisor_out_of_range() {
        let _ = 123_i32 / UnboundedPow2::from_exponent(32);
    }

    #[test]
    fn unb_pow2_int_div_rounds_towards_zero() {
        assert_eq!(37u64 / UnboundedPow2::from_exponent(5), 1);
        assert_eq!(63u64 / UnboundedPow2::from_exponent(5), 1);
        assert_eq!(-37i64 / UnboundedPow2::from_exponent(5), -1);
        assert_eq!(-1i64 / UnboundedPow2::from_exponent(5), 0);
    }

    #[test]
    fn pow2_int_div_rounds_towards_zero() {
        assert_eq!(37u64 / Pow2::<u8>::from_exponent(5).unwrap(), 1);
        assert_eq!(63u64 / Pow2::<u8>::from_exponent(5).unwrap(), 1);
        assert_eq!(-37i64 / Pow2::<u8>::from_exponent(5).unwrap(), -1);
        assert_eq!(-1i64 / Pow2::<u8>::from_exponent(5).unwrap(), 0);
    }

    #[test]
    fn unb_pow2_int_div_assign_rounds_towards_zero() {
        let mut v = 37u64;
        v /= UnboundedPow2::from_exponent(5);
        assert_eq!(v, 1);

        let mut v = 63u64;
        v /= UnboundedPow2::from_exponent(5);
        assert_eq!(v, 1);

        let mut v = -37i64;
        v /= UnboundedPow2::from_exponent(5);
        assert_eq!(v, -1);

        let mut v = -1i64;
        v /= UnboundedPow2::from_exponent(5);
        assert_eq!(v, 0);
    }

    #[test]
    fn pow2_int_div_assign_rounds_towards_zero() {
        let mut v = 37u64;
        v /= Pow2::<u8>::from_exponent(5).unwrap();
        assert_eq!(v, 1);

        let mut v = 63u64;
        v /= Pow2::<u8>::from_exponent(5).unwrap();
        assert_eq!(v, 1);

        let mut v = -37i64;
        v /= Pow2::<u8>::from_exponent(5).unwrap();
        assert_eq!(v, -1);

        let mut v = -1i64;
        v /= Pow2::<u8>::from_exponent(5).unwrap();
        assert_eq!(v, 0);
    }

    #[test]
    fn unb_pow2_int_div_by_one_is_identity() {
        assert_eq!(42i64 / UnboundedPow2::from_exponent(0), 42);
        assert_eq!(-42i64 / UnboundedPow2::from_exponent(0), -42);
    }

    #[test]
    fn unb_pow2_int_div_assign_by_one_is_identity() {
        let mut v = 42i64;
        v /= UnboundedPow2::from_exponent(0);
        assert_eq!(v, 42);

        let mut v = -42i64;
        v /= UnboundedPow2::from_exponent(0);
        assert_eq!(v, -42);
    }

    #[test]
    fn unb_pow2_int_div_min() {
        assert_eq!(i32::MIN / UnboundedPow2::from_exponent(30), -2);
        assert_eq!(i32::MIN / UnboundedPow2::from_exponent(31), -1);
    }

    #[test]
    fn unb_pow2_int_div_assign_min() {
        let mut v = i32::MIN;
        v /= UnboundedPow2::from_exponent(30);
        assert_eq!(v, -2);

        let mut v = i32::MIN;
        v /= UnboundedPow2::from_exponent(31);
        assert_eq!(v, -1);
    }

    #[test]
    fn unb_pow2_int_div_max() {
        assert_eq!(i32::MAX / UnboundedPow2::from_exponent(30), 1);
        assert_eq!(i32::MAX / UnboundedPow2::from_exponent(31), 0);
        assert_eq!(u32::MAX / UnboundedPow2::from_exponent(31), 1);
    }

    #[test]
    fn unb_pow2_int_div_assign_max() {
        let mut v = i32::MAX;
        v /= UnboundedPow2::from_exponent(30);
        assert_eq!(v, 1);

        let mut v = i32::MAX;
        v /= UnboundedPow2::from_exponent(31);
        assert_eq!(v, 0);

        let mut v = u32::MAX;
        v /= UnboundedPow2::from_exponent(31);
        assert_eq!(v, 1);
    }

    #[test]
    fn unb_pow2_checked_div_boundary() {
        assert_eq!(
            checked_div(i32::MAX, UnboundedPow2::from_exponent(30)),
            Some(1)
        );
        assert_eq!(
            checked_div(i32::MIN, UnboundedPow2::from_exponent(30)),
            Some(-2)
        );
        assert_eq!(
            checked_div(123_i32, UnboundedPow2::from_exponent(31)),
            Some(0)
        );
        assert_eq!(
            checked_div(-123_i32, UnboundedPow2::from_exponent(31)),
            Some(0)
        );
        assert_eq!(checked_div(123_i32, UnboundedPow2::from_exponent(32)), None);
        assert_eq!(
            checked_div(-123_i32, UnboundedPow2::from_exponent(32)),
            None
        );
    }

    #[test]
    fn unb_pow2_unbounded_div_boundary() {
        assert_eq!(unbounded_div(i32::MAX, UnboundedPow2::from_exponent(30)), 1);
        assert_eq!(unbounded_div(i32::MAX, UnboundedPow2::from_exponent(31)), 0);
        assert_eq!(
            unbounded_div(i32::MIN, UnboundedPow2::from_exponent(30)),
            -2
        );
        assert_eq!(
            unbounded_div(i32::MIN, UnboundedPow2::from_exponent(31)),
            -1
        );
        assert_eq!(unbounded_div(i32::MIN, UnboundedPow2::from_exponent(32)), 0);
        assert_eq!(unbounded_div(123_i32, UnboundedPow2::from_exponent(31)), 0);
        assert_eq!(unbounded_div(-123_i32, UnboundedPow2::from_exponent(31)), 0);
        assert_eq!(unbounded_div(123_i32, UnboundedPow2::from_exponent(32)), 0);
        assert_eq!(unbounded_div(-123_i32, UnboundedPow2::from_exponent(32)), 0);
    }

    #[test]
    fn unb_pow2_div_floor_u64_exact() {
        assert_eq!(div_floor(32u64, UnboundedPow2::from_exponent(5)), 1);
        assert_eq!(div_floor(64u64, UnboundedPow2::from_exponent(3)), 8);
    }

    #[test]
    fn pow2_div_floor_u64_exact() {
        assert_eq!(div_floor(32u64, Pow2::<u8>::from_exponent(5).unwrap()), 1);
        assert_eq!(div_floor(64u64, Pow2::<u8>::from_exponent(3).unwrap()), 8);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn unb_pow2_div_floor_divisor_out_of_range() {
        let _ = div_floor(32u64, UnboundedPow2::from_exponent(64));
    }

    #[test]
    fn unb_pow2_div_floor_u64_rounds_down() {
        assert_eq!(div_floor(37u64, UnboundedPow2::from_exponent(5)), 1);
        assert_eq!(div_floor(63u64, UnboundedPow2::from_exponent(5)), 1);
    }

    #[test]
    fn pow2_div_floor_u64_rounds_down() {
        assert_eq!(div_floor(37u64, Pow2::<u8>::from_exponent(5).unwrap()), 1);
        assert_eq!(div_floor(63u64, Pow2::<u8>::from_exponent(5).unwrap()), 1);
    }

    #[test]
    fn unb_pow2_div_floor_u64_zero() {
        assert_eq!(div_floor(0u64, UnboundedPow2::from_exponent(5)), 0);
    }

    #[test]
    fn pow2_div_floor_u64_zero() {
        assert_eq!(div_floor(0u64, Pow2::<u8>::from_exponent(5).unwrap()), 0);
    }

    #[test]
    fn unb_pow2_div_floor_i64_positive() {
        assert_eq!(div_floor(37i64, UnboundedPow2::from_exponent(5)), 1);
    }

    #[test]
    fn pow2_div_floor_i64_positive() {
        assert_eq!(div_floor(37i64, Pow2::<u8>::from_exponent(5).unwrap()), 1);
    }

    #[test]
    fn unb_pow2_div_floor_i64_exact_negative() {
        assert_eq!(div_floor(-32i64, UnboundedPow2::from_exponent(5)), -1);
    }

    #[test]
    fn pow2_div_floor_i64_exact_negative() {
        assert_eq!(div_floor(-32i64, Pow2::<u8>::from_exponent(5).unwrap()), -1);
    }

    #[test]
    fn unb_pow2_div_floor_i64_negative_rounds_toward_neg_inf() {
        assert_eq!(div_floor(-37i64, UnboundedPow2::from_exponent(5)), -2);
        assert_eq!(div_floor(-1i64, UnboundedPow2::from_exponent(5)), -1);
    }

    #[test]
    fn pow2_div_floor_i64_negative_rounds_toward_neg_inf() {
        assert_eq!(div_floor(-37i64, Pow2::<u8>::from_exponent(5).unwrap()), -2);
        assert_eq!(div_floor(-1i64, Pow2::<u8>::from_exponent(5).unwrap()), -1);
    }

    #[test]
    fn unb_pow2_div_floor_by_one_is_identity() {
        assert_eq!(div_floor(42i64, UnboundedPow2::from_exponent(0)), 42);
        assert_eq!(div_floor(-42i64, UnboundedPow2::from_exponent(0)), -42);
    }

    #[test]
    fn pow2_div_floor_by_one_is_identity() {
        assert_eq!(div_floor(42i64, Pow2::<u8>::from_exponent(0).unwrap()), 42);
        assert_eq!(
            div_floor(-42i64, Pow2::<u8>::from_exponent(0).unwrap()),
            -42
        );
    }

    #[test]
    fn unb_pow2_div_floor_min() {
        assert_eq!(div_floor(i32::MIN, UnboundedPow2::from_exponent(30)), -2);
        assert_eq!(div_floor(i32::MIN, UnboundedPow2::from_exponent(31)), -1);
    }

    #[test]
    fn pow2_div_floor_min() {
        assert_eq!(
            div_floor(i32::MIN, Pow2::<u32>::from_exponent(30).unwrap()),
            -2
        );
        assert_eq!(
            div_floor(i32::MIN, Pow2::<u32>::from_exponent(31).unwrap()),
            -1
        );
    }

    #[test]
    fn unb_pow2_div_floor_max() {
        assert_eq!(div_floor(i32::MAX, UnboundedPow2::from_exponent(30)), 1);
        assert_eq!(div_floor(i32::MAX, UnboundedPow2::from_exponent(31)), 0);
        assert_eq!(div_floor(u32::MAX, UnboundedPow2::from_exponent(31)), 1);
    }

    #[test]
    fn pow2_div_floor_max() {
        assert_eq!(
            div_floor(i32::MAX, Pow2::<u32>::from_exponent(30).unwrap()),
            1
        );
        assert_eq!(
            div_floor(i32::MAX, Pow2::<u32>::from_exponent(31).unwrap()),
            0
        );
        assert_eq!(
            div_floor(u32::MAX, Pow2::<u32>::from_exponent(31).unwrap()),
            1
        );
    }

    #[test]
    fn unb_pow2_checked_div_floor_boundary() {
        assert_eq!(
            checked_div_floor(0_i32, UnboundedPow2::from_exponent(30)),
            Some(0)
        );
        assert_eq!(
            checked_div_floor(0_i32, UnboundedPow2::from_exponent(31)),
            Some(0)
        );
        assert_eq!(
            checked_div_floor(0_i32, UnboundedPow2::from_exponent(32)),
            None
        );

        assert_eq!(
            checked_div_floor(0_u32, UnboundedPow2::from_exponent(31)),
            Some(0)
        );
        assert_eq!(
            checked_div_floor(0_u32, UnboundedPow2::from_exponent(32)),
            None
        );
    }

    #[test]
    fn unb_pow2_unbounded_div_floor_boundary() {
        assert_eq!(
            unbounded_div_floor(0_i32, UnboundedPow2::from_exponent(30)),
            0
        );
        assert_eq!(
            unbounded_div_floor(i32::MIN, UnboundedPow2::from_exponent(30)),
            -2
        );
        assert_eq!(
            unbounded_div_floor(0_i32, UnboundedPow2::from_exponent(31)),
            0
        );
        assert_eq!(
            unbounded_div_floor(-1_i32, UnboundedPow2::from_exponent(31)),
            -1
        );
        assert_eq!(
            unbounded_div_floor(i32::MIN, UnboundedPow2::from_exponent(31)),
            -1
        );
        assert_eq!(
            unbounded_div_floor(0_i32, UnboundedPow2::from_exponent(32)),
            0
        );
        assert_eq!(
            unbounded_div_floor(-1_i32, UnboundedPow2::from_exponent(32)),
            -1
        );
        assert_eq!(
            unbounded_div_floor(0_i32, UnboundedPow2::from_exponent(255)),
            0
        );
        assert_eq!(
            unbounded_div_floor(i32::MAX, UnboundedPow2::from_exponent(255)),
            0
        );
        assert_eq!(
            unbounded_div_floor(i32::MIN, UnboundedPow2::from_exponent(255)),
            -1
        );

        assert_eq!(
            unbounded_div_floor(0_u32, UnboundedPow2::from_exponent(31)),
            0
        );
        assert_eq!(
            unbounded_div_floor(u32::MAX, UnboundedPow2::from_exponent(31)),
            1
        );
        assert_eq!(
            unbounded_div_floor(0_u32, UnboundedPow2::from_exponent(32)),
            0
        );
        assert_eq!(
            unbounded_div_floor(u32::MAX, UnboundedPow2::from_exponent(32)),
            0
        );
        assert_eq!(
            unbounded_div_floor(0_u32, UnboundedPow2::from_exponent(255)),
            0
        );
        assert_eq!(
            unbounded_div_floor(u32::MAX, UnboundedPow2::from_exponent(255)),
            0
        );
    }

    #[test]
    fn unb_pow2_div_ceil_u64_exact() {
        assert_eq!(div_ceil(32u64, UnboundedPow2::from_exponent(5)), 1);
    }

    #[test]
    fn pow2_div_ceil_u64_exact() {
        assert_eq!(div_ceil(32u64, Pow2::<u8>::from_exponent(5).unwrap()), 1);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn unb_pow2_div_ceil_divisor_out_of_range() {
        let _ = div_ceil(32u64, UnboundedPow2::from_exponent(64));
    }

    #[test]
    fn unb_pow2_ceil_u64_rounds_up() {
        assert_eq!(div_ceil(33u64, UnboundedPow2::from_exponent(5)), 2);
        assert_eq!(div_ceil(63u64, UnboundedPow2::from_exponent(5)), 2);
    }

    #[test]
    fn pow2_div_ceil_u64_rounds_up() {
        assert_eq!(div_ceil(33u64, Pow2::<u8>::from_exponent(5).unwrap()), 2);
        assert_eq!(div_ceil(63u64, Pow2::<u8>::from_exponent(5).unwrap()), 2);
    }

    #[test]
    fn unb_pow2_div_ceil_u64_zero() {
        assert_eq!(div_ceil(0u64, UnboundedPow2::from_exponent(5)), 0);
    }

    #[test]
    fn pow2_div_ceil_u64_zero() {
        assert_eq!(div_ceil(0u64, Pow2::<u8>::from_exponent(5).unwrap()), 0);
    }

    #[test]
    fn unb_pow2_div_ceil_i64_negative() {
        assert_eq!(div_ceil(-37i64, UnboundedPow2::from_exponent(5)), -1);
        assert_eq!(div_ceil(-32i64, UnboundedPow2::from_exponent(5)), -1);
        assert_eq!(div_ceil(-1i64, UnboundedPow2::from_exponent(5)), 0);
        assert_eq!(div_ceil(-33i64, UnboundedPow2::from_exponent(5)), -1);
    }

    #[test]
    fn pow2_div_ceil_i64_negative() {
        assert_eq!(div_ceil(-37i64, Pow2::<u8>::from_exponent(5).unwrap()), -1);
        assert_eq!(div_ceil(-32i64, Pow2::<u8>::from_exponent(5).unwrap()), -1);
        assert_eq!(div_ceil(-1i64, Pow2::<u8>::from_exponent(5).unwrap()), 0);
        assert_eq!(div_ceil(-33i64, Pow2::<u8>::from_exponent(5).unwrap()), -1);
    }

    #[test]
    fn unb_pow2_div_ceil_by_one_is_identity() {
        assert_eq!(div_ceil(42i64, UnboundedPow2::from_exponent(0)), 42);
        assert_eq!(div_ceil(-42i64, UnboundedPow2::from_exponent(0)), -42);
    }

    #[test]
    fn pow2_div_ceil_by_one_is_identity() {
        assert_eq!(div_ceil(42i64, Pow2::<u8>::from_exponent(0).unwrap()), 42);
        assert_eq!(div_ceil(-42i64, Pow2::<u8>::from_exponent(0).unwrap()), -42);
    }

    #[test]
    fn unb_pow2_div_ceil_min() {
        assert_eq!(div_ceil(i32::MIN, UnboundedPow2::from_exponent(30)), -2);
        assert_eq!(div_ceil(i32::MIN, UnboundedPow2::from_exponent(31)), -1);
        assert_eq!(div_ceil(i64::MIN, UnboundedPow2::from_exponent(62)), -2);
        assert_eq!(div_ceil(i64::MIN, UnboundedPow2::from_exponent(63)), -1);
    }

    #[test]
    fn pow2_div_ceil_min() {
        assert_eq!(
            div_ceil(i32::MIN, Pow2::<u32>::from_exponent(30).unwrap()),
            -2
        );
        assert_eq!(
            div_ceil(i32::MIN, Pow2::<u32>::from_exponent(31).unwrap()),
            -1
        );
        assert_eq!(
            div_ceil(i64::MIN, Pow2::<u64>::from_exponent(62).unwrap()),
            -2
        );
        assert_eq!(
            div_ceil(i64::MIN, Pow2::<u64>::from_exponent(63).unwrap()),
            -1
        );
    }

    #[test]
    fn unb_pow2_div_ceil_max() {
        assert_eq!(div_ceil(i32::MAX, UnboundedPow2::from_exponent(30)), 2);
        assert_eq!(div_ceil(i32::MAX, UnboundedPow2::from_exponent(31)), 1);
        assert_eq!(div_ceil(u32::MAX, UnboundedPow2::from_exponent(31)), 2);
        assert_eq!(div_ceil(i64::MAX, UnboundedPow2::from_exponent(62)), 2);
        assert_eq!(div_ceil(i64::MAX, UnboundedPow2::from_exponent(63)), 1);
        assert_eq!(div_ceil(u64::MAX, UnboundedPow2::from_exponent(63)), 2);
    }

    #[test]
    fn pow2_div_ceil_max() {
        assert_eq!(
            div_ceil(i32::MAX, Pow2::<u32>::from_exponent(30).unwrap()),
            2
        );
        assert_eq!(
            div_ceil(i32::MAX, Pow2::<u32>::from_exponent(31).unwrap()),
            1
        );
        assert_eq!(
            div_ceil(u32::MAX, Pow2::<u32>::from_exponent(31).unwrap()),
            2
        );
        assert_eq!(
            div_ceil(i64::MAX, Pow2::<u64>::from_exponent(62).unwrap()),
            2
        );
        assert_eq!(
            div_ceil(i64::MAX, Pow2::<u64>::from_exponent(63).unwrap()),
            1
        );
        assert_eq!(
            div_ceil(u64::MAX, Pow2::<u64>::from_exponent(63).unwrap()),
            2
        );
    }

    #[test]
    fn unb_pow2_checked_div_ceil_boundary() {
        assert_eq!(
            checked_div_ceil(0_i32, UnboundedPow2::from_exponent(30)),
            Some(0)
        );
        assert_eq!(
            checked_div_ceil(0_i32, UnboundedPow2::from_exponent(31)),
            Some(0)
        );
        assert_eq!(
            checked_div_ceil(0_i32, UnboundedPow2::from_exponent(32)),
            None
        );

        assert_eq!(
            checked_div_ceil(0_u32, UnboundedPow2::from_exponent(31)),
            Some(0)
        );
        assert_eq!(
            checked_div_ceil(0_u32, UnboundedPow2::from_exponent(32)),
            None
        );
    }

    #[test]
    fn unb_pow2_unbounded_div_ceil_boundary() {
        assert_eq!(
            unbounded_div_ceil(0_i32, UnboundedPow2::from_exponent(30)),
            0
        );
        assert_eq!(
            unbounded_div_ceil(i32::MAX, UnboundedPow2::from_exponent(30)),
            2
        );
        assert_eq!(
            unbounded_div_ceil(0_i32, UnboundedPow2::from_exponent(31)),
            0
        );
        assert_eq!(
            unbounded_div_ceil(i32::MIN, UnboundedPow2::from_exponent(31)),
            -1
        );
        assert_eq!(
            unbounded_div_ceil(-1_i32, UnboundedPow2::from_exponent(32)),
            0
        );
        assert_eq!(
            unbounded_div_ceil(0_i32, UnboundedPow2::from_exponent(32)),
            0
        );
        assert_eq!(
            unbounded_div_ceil(1_i32, UnboundedPow2::from_exponent(32)),
            1
        );
        assert_eq!(
            unbounded_div_ceil(i32::MAX, UnboundedPow2::from_exponent(32)),
            1
        );
        assert_eq!(
            unbounded_div_ceil(-1_i32, UnboundedPow2::from_exponent(255)),
            0
        );
        assert_eq!(
            unbounded_div_ceil(i32::MIN, UnboundedPow2::from_exponent(255)),
            0
        );
        assert_eq!(
            unbounded_div_ceil(i32::MAX, UnboundedPow2::from_exponent(255)),
            1
        );

        assert_eq!(
            unbounded_div_ceil(0_u32, UnboundedPow2::from_exponent(31)),
            0
        );
        assert_eq!(
            unbounded_div_ceil(0_u32, UnboundedPow2::from_exponent(32)),
            0
        );
        assert_eq!(
            unbounded_div_ceil(0_u32, UnboundedPow2::from_exponent(255)),
            0
        );
        assert_eq!(
            unbounded_div_ceil(u32::MAX, UnboundedPow2::from_exponent(255)),
            1
        );
    }

    #[test]
    fn unb_pow2_div_round_u64_exact() {
        assert_eq!(div_round(32u64, UnboundedPow2::from_exponent(5)), 1);
    }

    #[test]
    fn pow2_div_round_u64_exact() {
        assert_eq!(div_round(32u64, Pow2::<u8>::from_exponent(5).unwrap()), 1);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn unb_pow2_div_round_divisor_out_of_range() {
        let _ = div_round(32u64, UnboundedPow2::from_exponent(64));
    }

    #[test]
    fn unb_pow2_div_round_u64_rounds() {
        assert_eq!(div_round(17u64, UnboundedPow2::from_exponent(5)), 1);
        assert_eq!(div_round(16u64, UnboundedPow2::from_exponent(5)), 1);
        assert_eq!(div_round(15u64, UnboundedPow2::from_exponent(5)), 0);
        assert_eq!(div_round(33u64, UnboundedPow2::from_exponent(5)), 1);
        assert_eq!(div_round(48u64, UnboundedPow2::from_exponent(5)), 2);
        assert_eq!(div_round(47u64, UnboundedPow2::from_exponent(5)), 1);
    }

    #[test]
    fn pow2_div_round_u64_rounds() {
        assert_eq!(div_round(17u64, Pow2::<u8>::from_exponent(5).unwrap()), 1);
        assert_eq!(div_round(16u64, Pow2::<u8>::from_exponent(5).unwrap()), 1);
        assert_eq!(div_round(15u64, Pow2::<u8>::from_exponent(5).unwrap()), 0);
        assert_eq!(div_round(33u64, Pow2::<u8>::from_exponent(5).unwrap()), 1);
        assert_eq!(div_round(48u64, Pow2::<u8>::from_exponent(5).unwrap()), 2);
        assert_eq!(div_round(47u64, Pow2::<u8>::from_exponent(5).unwrap()), 1);
    }

    #[test]
    fn unb_pow2_div_round_u64_zero() {
        assert_eq!(div_round(0u64, UnboundedPow2::from_exponent(5)), 0);
    }

    #[test]
    fn pow2_div_round_u64_zero() {
        assert_eq!(div_round(0u64, Pow2::<u8>::from_exponent(5).unwrap()), 0);
    }

    #[test]
    fn unb_pow2_div_round_i64_negative() {
        assert_eq!(div_round(-37i64, UnboundedPow2::from_exponent(5)), -1);
        assert_eq!(div_round(-32i64, UnboundedPow2::from_exponent(5)), -1);
        assert_eq!(div_round(-16i64, UnboundedPow2::from_exponent(5)), -1);
        assert_eq!(div_round(-17i64, UnboundedPow2::from_exponent(5)), -1);
        assert_eq!(div_round(-15i64, UnboundedPow2::from_exponent(5)), 0);
        assert_eq!(div_round(-1i64, UnboundedPow2::from_exponent(5)), 0);
        assert_eq!(div_round(-33i64, UnboundedPow2::from_exponent(5)), -1);
        assert_eq!(div_round(-48i64, UnboundedPow2::from_exponent(5)), -2);
        assert_eq!(div_round(-47i64, UnboundedPow2::from_exponent(5)), -1);
    }

    #[test]
    fn pow2_div_round_i64_negative() {
        assert_eq!(div_round(-37i64, Pow2::<u8>::from_exponent(5).unwrap()), -1);
        assert_eq!(div_round(-32i64, Pow2::<u8>::from_exponent(5).unwrap()), -1);
        assert_eq!(div_round(-16i64, Pow2::<u8>::from_exponent(5).unwrap()), -1);
        assert_eq!(div_round(-17i64, Pow2::<u8>::from_exponent(5).unwrap()), -1);
        assert_eq!(div_round(-15i64, Pow2::<u8>::from_exponent(5).unwrap()), 0);
        assert_eq!(div_round(-1i64, Pow2::<u8>::from_exponent(5).unwrap()), 0);
        assert_eq!(div_round(-33i64, Pow2::<u8>::from_exponent(5).unwrap()), -1);
        assert_eq!(div_round(-48i64, Pow2::<u8>::from_exponent(5).unwrap()), -2);
        assert_eq!(div_round(-47i64, Pow2::<u8>::from_exponent(5).unwrap()), -1);
    }

    #[test]
    fn unb_pow2_div_round_by_one_is_identity() {
        assert_eq!(div_round(42i64, UnboundedPow2::from_exponent(0)), 42);
        assert_eq!(div_round(-42i64, UnboundedPow2::from_exponent(0)), -42);
    }

    #[test]
    fn pow2_div_round_by_one_is_identity() {
        assert_eq!(div_round(42i64, Pow2::<u8>::from_exponent(0).unwrap()), 42);
        assert_eq!(
            div_round(-42i64, Pow2::<u8>::from_exponent(0).unwrap()),
            -42
        );
    }

    #[test]
    fn unb_pow2_div_round_min() {
        assert_eq!(div_round(i32::MIN, UnboundedPow2::from_exponent(30)), -2);
        assert_eq!(div_round(i32::MIN, UnboundedPow2::from_exponent(31)), -1);
        assert_eq!(div_round(i64::MIN, UnboundedPow2::from_exponent(62)), -2);
        assert_eq!(div_round(i64::MIN, UnboundedPow2::from_exponent(63)), -1);
    }

    #[test]
    fn pow2_div_round_min() {
        assert_eq!(
            div_round(i32::MIN, Pow2::<u32>::from_exponent(30).unwrap()),
            -2
        );
        assert_eq!(
            div_round(i32::MIN, Pow2::<u32>::from_exponent(31).unwrap()),
            -1
        );
        assert_eq!(
            div_round(i64::MIN, Pow2::<u64>::from_exponent(62).unwrap()),
            -2
        );
        assert_eq!(
            div_round(i64::MIN, Pow2::<u64>::from_exponent(63).unwrap()),
            -1
        );
    }

    #[test]
    fn unb_pow2_div_round_max() {
        assert_eq!(div_round(i32::MAX, UnboundedPow2::from_exponent(30)), 2);
        assert_eq!(div_round(i32::MAX, UnboundedPow2::from_exponent(31)), 1);
        assert_eq!(div_round(u32::MAX, UnboundedPow2::from_exponent(31)), 2);
        assert_eq!(div_round(i64::MAX, UnboundedPow2::from_exponent(62)), 2);
        assert_eq!(div_round(i64::MAX, UnboundedPow2::from_exponent(63)), 1);
        assert_eq!(div_round(u64::MAX, UnboundedPow2::from_exponent(63)), 2);
    }

    #[test]
    fn pow2_div_round_max() {
        assert_eq!(
            div_round(i32::MAX, Pow2::<u32>::from_exponent(30).unwrap()),
            2
        );
        assert_eq!(
            div_round(i32::MAX, Pow2::<u32>::from_exponent(31).unwrap()),
            1
        );
        assert_eq!(
            div_round(u32::MAX, Pow2::<u32>::from_exponent(31).unwrap()),
            2
        );
        assert_eq!(
            div_round(i64::MAX, Pow2::<u64>::from_exponent(62).unwrap()),
            2
        );
        assert_eq!(
            div_round(i64::MAX, Pow2::<u64>::from_exponent(63).unwrap()),
            1
        );
        assert_eq!(
            div_round(u64::MAX, Pow2::<u64>::from_exponent(63).unwrap()),
            2
        );
    }

    #[test]
    fn unb_pow2_checked_div_round_boundary() {
        assert_eq!(
            checked_div_round(0_i32, UnboundedPow2::from_exponent(30)),
            Some(0)
        );
        assert_eq!(
            checked_div_round(0_i32, UnboundedPow2::from_exponent(31)),
            Some(0)
        );
        assert_eq!(
            checked_div_round(0_i32, UnboundedPow2::from_exponent(32)),
            None
        );

        assert_eq!(
            checked_div_round(0_u32, UnboundedPow2::from_exponent(31)),
            Some(0)
        );
        assert_eq!(
            checked_div_round(0_u32, UnboundedPow2::from_exponent(32)),
            None
        );
    }

    #[test]
    fn unb_pow2_unbounded_div_round_boundary() {
        assert_eq!(
            unbounded_div_round(0_i32, UnboundedPow2::from_exponent(30)),
            0
        );
        assert_eq!(
            unbounded_div_round(i32::MAX, UnboundedPow2::from_exponent(30)),
            2
        );
        assert_eq!(
            unbounded_div_round(0_i32, UnboundedPow2::from_exponent(31)),
            0
        );
        assert_eq!(
            unbounded_div_round(i32::MIN, UnboundedPow2::from_exponent(31)),
            -1
        );
        assert_eq!(
            unbounded_div_round(i32::MIN, UnboundedPow2::from_exponent(32)),
            -1
        );
        assert_eq!(
            unbounded_div_round(-1_i32, UnboundedPow2::from_exponent(32)),
            0
        );
        assert_eq!(
            unbounded_div_round(0_i32, UnboundedPow2::from_exponent(32)),
            0
        );
        assert_eq!(
            unbounded_div_round(1_i32, UnboundedPow2::from_exponent(32)),
            0
        );
        assert_eq!(
            unbounded_div_round(i32::MAX, UnboundedPow2::from_exponent(32)),
            0
        );
        assert_eq!(
            unbounded_div_round(-1_i32, UnboundedPow2::from_exponent(255)),
            0
        );
        assert_eq!(
            unbounded_div_round(i32::MIN, UnboundedPow2::from_exponent(255)),
            0
        );
        assert_eq!(
            unbounded_div_round(i32::MAX, UnboundedPow2::from_exponent(255)),
            0
        );

        assert_eq!(
            unbounded_div_round(0_u32, UnboundedPow2::from_exponent(31)),
            0
        );
        assert_eq!(
            unbounded_div_round(0_u32, UnboundedPow2::from_exponent(32)),
            0
        );
        assert_eq!(
            unbounded_div_round(0_u32, UnboundedPow2::from_exponent(255)),
            0
        );
        assert_eq!(
            unbounded_div_round(u32::MAX, UnboundedPow2::from_exponent(255)),
            0
        );
    }

    #[test]
    fn unb_pow2_floor_to_multiple_u64_already_aligned() {
        assert_eq!(
            floor_to_multiple(64u64, UnboundedPow2::from_exponent(6)),
            64
        );
    }

    #[test]
    fn pow2_floor_to_multiple_u64_already_aligned() {
        assert_eq!(
            floor_to_multiple(64u64, Pow2::<u8>::from_exponent(6).unwrap()),
            64
        );
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn unb_pow2_floor_to_multiple_align_out_of_range() {
        let _ = floor_to_multiple(64u64, UnboundedPow2::from_exponent(64));
    }

    #[test]
    fn unb_pow2_floor_to_multiple_u64_unaligned() {
        assert_eq!(
            floor_to_multiple(65u64, UnboundedPow2::from_exponent(6)),
            64
        );
        assert_eq!(
            floor_to_multiple(127u64, UnboundedPow2::from_exponent(6)),
            64
        );
    }

    #[test]
    fn pow2_floor_to_multiple_u64_unaligned() {
        assert_eq!(
            floor_to_multiple(65u64, Pow2::<u8>::from_exponent(6).unwrap()),
            64
        );
        assert_eq!(
            floor_to_multiple(127u64, Pow2::<u8>::from_exponent(6).unwrap()),
            64
        );
    }

    #[test]
    fn unb_pow2_floor_to_multiple_i64_positive() {
        assert_eq!(
            floor_to_multiple(37i64, UnboundedPow2::from_exponent(5)),
            32
        );
    }

    #[test]
    fn pow2_floor_to_multiple_i64_positive() {
        assert_eq!(
            floor_to_multiple(37i64, Pow2::<u8>::from_exponent(5).unwrap()),
            32
        );
    }

    #[test]
    fn unb_pow2_floor_to_multiple_i64_negative() {
        assert_eq!(
            floor_to_multiple(-37i64, UnboundedPow2::from_exponent(5)),
            -64
        );
        assert_eq!(
            floor_to_multiple(-32i64, UnboundedPow2::from_exponent(5)),
            -32
        );
    }

    #[test]
    fn pow2_floor_to_multiple_i64_negative() {
        assert_eq!(
            floor_to_multiple(-37i64, Pow2::<u8>::from_exponent(5).unwrap()),
            -64
        );
        assert_eq!(
            floor_to_multiple(-32i64, Pow2::<u8>::from_exponent(5).unwrap()),
            -32
        );
    }

    #[test]
    fn unb_pow2_floor_to_multiple_by_one_is_identity() {
        assert_eq!(
            floor_to_multiple(37i64, UnboundedPow2::from_exponent(0)),
            37
        );
        assert_eq!(
            floor_to_multiple(-37i64, UnboundedPow2::from_exponent(0)),
            -37
        );
    }

    #[test]
    fn pow2_floor_to_multiple_by_one_is_identity() {
        assert_eq!(
            floor_to_multiple(37i64, Pow2::<u8>::from_exponent(0).unwrap()),
            37
        );
        assert_eq!(
            floor_to_multiple(-37i64, Pow2::<u8>::from_exponent(0).unwrap()),
            -37
        );
    }

    #[test]
    fn unb_pow2_floor_to_multiple_min() {
        assert_eq!(
            floor_to_multiple(i32::MIN, UnboundedPow2::from_exponent(15)),
            i32::MIN
        );
        assert_eq!(
            floor_to_multiple(i32::MIN, UnboundedPow2::from_exponent(30)),
            i32::MIN
        );
        assert_eq!(
            floor_to_multiple(i32::MIN, UnboundedPow2::from_exponent(31)),
            i32::MIN
        );
    }

    #[test]
    fn pow2_floor_to_multiple_min() {
        assert_eq!(
            floor_to_multiple(i32::MIN, Pow2::<u32>::from_exponent(15).unwrap()),
            i32::MIN
        );
        assert_eq!(
            floor_to_multiple(i32::MIN, Pow2::<u32>::from_exponent(30).unwrap()),
            i32::MIN
        );
        assert_eq!(
            floor_to_multiple(i32::MIN, Pow2::<u32>::from_exponent(31).unwrap()),
            i32::MIN
        );
    }

    #[test]
    fn unb_pow2_floor_to_multiple_max() {
        assert_eq!(
            floor_to_multiple(i32::MAX, UnboundedPow2::from_exponent(30)),
            1 << 30
        );
        assert_eq!(
            floor_to_multiple(i32::MAX, UnboundedPow2::from_exponent(31)),
            0
        );
        assert_eq!(
            floor_to_multiple(u32::MAX, UnboundedPow2::from_exponent(31)),
            1 << 31
        );
    }

    #[test]
    fn pow2_floor_to_multiple_max() {
        assert_eq!(
            floor_to_multiple(i32::MAX, Pow2::<u32>::from_exponent(30).unwrap()),
            1 << 30
        );
        assert_eq!(
            floor_to_multiple(i32::MAX, Pow2::<u32>::from_exponent(31).unwrap()),
            0
        );
        assert_eq!(
            floor_to_multiple(u32::MAX, Pow2::<u32>::from_exponent(31).unwrap()),
            1 << 31
        );
    }

    #[test]
    fn unb_pow2_checked_floor_to_multiple_boundary() {
        assert_eq!(
            checked_floor_to_multiple(0_i32, UnboundedPow2::from_exponent(30)),
            Some(0)
        );
        assert_eq!(
            checked_floor_to_multiple(0_i32, UnboundedPow2::from_exponent(31)),
            Some(0)
        );
        assert_eq!(
            checked_floor_to_multiple(0_i32, UnboundedPow2::from_exponent(32)),
            None
        );

        assert_eq!(
            checked_floor_to_multiple(0_u32, UnboundedPow2::from_exponent(31)),
            Some(0)
        );
        assert_eq!(
            checked_floor_to_multiple(0_u32, UnboundedPow2::from_exponent(32)),
            None
        );
    }

    #[test]
    fn unb_pow2_unbounded_floor_to_multiple_boundary() {
        assert_eq!(
            unbounded_floor_to_multiple(0_i32, UnboundedPow2::from_exponent(30)),
            Some(0)
        );
        assert_eq!(
            unbounded_floor_to_multiple(0_i32, UnboundedPow2::from_exponent(31)),
            Some(0)
        );
        assert_eq!(
            unbounded_floor_to_multiple(-1_i32, UnboundedPow2::from_exponent(31)),
            Some(i32::MIN)
        );
        assert_eq!(
            unbounded_floor_to_multiple(0_i32, UnboundedPow2::from_exponent(32)),
            Some(0)
        );
        assert_eq!(
            unbounded_floor_to_multiple(-1_i32, UnboundedPow2::from_exponent(32)),
            None
        );
        assert_eq!(
            unbounded_floor_to_multiple(1_i32, UnboundedPow2::from_exponent(32)),
            Some(0)
        );
        assert_eq!(
            unbounded_floor_to_multiple(0_i32, UnboundedPow2::from_exponent(255)),
            Some(0)
        );

        assert_eq!(
            unbounded_floor_to_multiple(0_u32, UnboundedPow2::from_exponent(31)),
            0
        );
        assert_eq!(
            unbounded_floor_to_multiple(0_u32, UnboundedPow2::from_exponent(32)),
            0
        );
        assert_eq!(
            unbounded_floor_to_multiple(0_u32, UnboundedPow2::from_exponent(255)),
            0
        );
    }

    #[test]
    fn unb_pow2_ceil_to_multiple_u64_already_aligned() {
        assert_eq!(ceil_to_multiple(64u64, UnboundedPow2::from_exponent(6)), 64);
    }

    #[test]
    fn pow2_ceil_to_multiple_u64_already_aligned() {
        assert_eq!(
            ceil_to_multiple(64u64, Pow2::<u8>::from_exponent(6).unwrap()),
            64
        );
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn unb_pow2_ceil_to_multiple_align_out_of_range() {
        let _ = ceil_to_multiple(64u64, UnboundedPow2::from_exponent(64));
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn unb_pow2_ceil_to_multiple_overflow() {
        let _ = ceil_to_multiple(i32::MAX, UnboundedPow2::from_exponent(1));
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn pow2_ceil_to_multiple_overflow() {
        let _ = ceil_to_multiple(i32::MAX, Pow2::<u8>::from_exponent(1).unwrap());
    }

    #[test]
    fn unb_pow2_ceil_to_multiple_u64_unaligned() {
        assert_eq!(
            ceil_to_multiple(65u64, UnboundedPow2::from_exponent(6)),
            128
        );
        assert_eq!(ceil_to_multiple(1u64, UnboundedPow2::from_exponent(6)), 64);
    }

    #[test]
    fn pow2_ceil_to_multiple_u64_unaligned() {
        assert_eq!(
            ceil_to_multiple(65u64, Pow2::<u8>::from_exponent(6).unwrap()),
            128
        );
        assert_eq!(
            ceil_to_multiple(1u64, Pow2::<u8>::from_exponent(6).unwrap()),
            64
        );
    }

    #[test]
    fn unb_pow2_ceil_to_multiple_u64_zero() {
        assert_eq!(ceil_to_multiple(0u64, UnboundedPow2::from_exponent(5)), 0);
    }

    #[test]
    fn pow2_ceil_to_multiple_u64_zero() {
        assert_eq!(
            ceil_to_multiple(0u64, Pow2::<u8>::from_exponent(5).unwrap()),
            0
        );
    }

    #[test]
    fn unb_pow2_ceil_to_multiple_i64_positive() {
        assert_eq!(ceil_to_multiple(33i64, UnboundedPow2::from_exponent(5)), 64);
        assert_eq!(ceil_to_multiple(32i64, UnboundedPow2::from_exponent(5)), 32);
    }

    #[test]
    fn pow2_ceil_to_multiple_i64_positive() {
        assert_eq!(
            ceil_to_multiple(33i64, Pow2::<u8>::from_exponent(5).unwrap()),
            64
        );
        assert_eq!(
            ceil_to_multiple(32i64, Pow2::<u8>::from_exponent(5).unwrap()),
            32
        );
    }

    #[test]
    fn unb_pow2_ceil_to_multiple_i64_negative() {
        assert_eq!(
            ceil_to_multiple(-33i64, UnboundedPow2::from_exponent(5)),
            -32
        );
        assert_eq!(
            ceil_to_multiple(-32i64, UnboundedPow2::from_exponent(5)),
            -32
        );
        assert_eq!(ceil_to_multiple(-16i64, UnboundedPow2::from_exponent(5)), 0);
    }

    #[test]
    fn pow2_ceil_to_multiple_i64_negative() {
        assert_eq!(
            ceil_to_multiple(-33i64, Pow2::<u8>::from_exponent(5).unwrap()),
            -32
        );
        assert_eq!(
            ceil_to_multiple(-32i64, Pow2::<u8>::from_exponent(5).unwrap()),
            -32
        );
        assert_eq!(
            ceil_to_multiple(-16i64, Pow2::<u8>::from_exponent(5).unwrap()),
            0
        );
    }

    #[test]
    fn unb_pow2_ceil_to_multiple_min() {
        assert_eq!(
            ceil_to_multiple(i32::MIN, UnboundedPow2::from_exponent(2)),
            i32::MIN
        );
        assert_eq!(
            ceil_to_multiple(i32::MIN, UnboundedPow2::from_exponent(30)),
            i32::MIN
        );
    }

    #[test]
    fn pow2_ceil_to_multiple_min() {
        assert_eq!(
            ceil_to_multiple(i32::MIN, Pow2::<u32>::from_exponent(2).unwrap()),
            i32::MIN
        );
        assert_eq!(
            ceil_to_multiple(i32::MIN, Pow2::<u32>::from_exponent(30).unwrap()),
            i32::MIN
        );
    }

    #[test]
    fn unb_pow2_ceil_to_multiple_max() {
        assert_eq!(
            ceil_to_multiple(i32::MAX, UnboundedPow2::from_exponent(0)),
            i32::MAX
        );
        assert_eq!(
            ceil_to_multiple(i32::MAX - 1, UnboundedPow2::from_exponent(1)),
            i32::MAX - 1
        );

        assert_eq!(
            ceil_to_multiple(u32::MAX, UnboundedPow2::from_exponent(0)),
            u32::MAX
        );
        assert_eq!(
            ceil_to_multiple(u32::MAX - 1, UnboundedPow2::from_exponent(1)),
            u32::MAX - 1
        );
    }

    #[test]
    fn pow2_ceil_to_multiple_max() {
        assert_eq!(
            ceil_to_multiple(i32::MAX, Pow2::<u32>::from_exponent(0).unwrap()),
            i32::MAX
        );
        assert_eq!(
            ceil_to_multiple(i32::MAX - 1, Pow2::<u32>::from_exponent(1).unwrap()),
            i32::MAX - 1
        );

        assert_eq!(
            ceil_to_multiple(u32::MAX, Pow2::<u32>::from_exponent(0).unwrap()),
            u32::MAX
        );
        assert_eq!(
            ceil_to_multiple(u32::MAX - 1, Pow2::<u32>::from_exponent(1).unwrap()),
            u32::MAX - 1
        );
    }

    #[test]
    fn unb_pow2_checked_ceil_to_multiple_boundary() {
        assert_eq!(
            checked_ceil_to_multiple(0_i32, UnboundedPow2::from_exponent(30)),
            Some(0)
        );
        assert_eq!(
            checked_ceil_to_multiple(0_i32, UnboundedPow2::from_exponent(31)),
            Some(0)
        );
        assert_eq!(
            checked_ceil_to_multiple(i32::MAX, UnboundedPow2::from_exponent(0)),
            Some(i32::MAX)
        );
        assert_eq!(
            checked_ceil_to_multiple(i32::MAX, UnboundedPow2::from_exponent(1)),
            None
        );

        assert_eq!(
            checked_ceil_to_multiple(0_u32, UnboundedPow2::from_exponent(31)),
            Some(0)
        );
        assert_eq!(
            checked_ceil_to_multiple(i32::MAX as u32 + 2, UnboundedPow2::from_exponent(31)),
            None
        );
        assert_eq!(
            checked_ceil_to_multiple(0_u32, UnboundedPow2::from_exponent(32)),
            None
        );
    }

    #[test]
    fn pow2_checked_ceil_to_multiple_boundary() {
        assert_eq!(
            checked_ceil_to_multiple(0_i32, Pow2::<u32>::from_exponent(30).unwrap()),
            Some(0)
        );
        assert_eq!(
            checked_ceil_to_multiple(0_i32, Pow2::<u32>::from_exponent(31).unwrap()),
            Some(0)
        );
        assert_eq!(
            checked_ceil_to_multiple(i32::MAX, Pow2::<u32>::from_exponent(0).unwrap()),
            Some(i32::MAX)
        );
        assert_eq!(
            checked_ceil_to_multiple(i32::MAX, Pow2::<u32>::from_exponent(1).unwrap()),
            None
        );

        assert_eq!(
            checked_ceil_to_multiple(0_u32, Pow2::<u32>::from_exponent(31).unwrap()),
            Some(0)
        );
        assert_eq!(
            checked_ceil_to_multiple(i32::MAX as u32 + 2, Pow2::<u32>::from_exponent(31).unwrap()),
            None
        );
    }

    #[test]
    fn unb_pow2_unbounded_ceil_to_multiple_boundary() {
        assert_eq!(
            unbounded_ceil_to_multiple(0_i32, UnboundedPow2::from_exponent(30)),
            Some(0)
        );
        assert_eq!(
            unbounded_ceil_to_multiple(-1_i32, UnboundedPow2::from_exponent(31)),
            Some(0)
        );
        assert_eq!(
            unbounded_ceil_to_multiple(i32::MIN, UnboundedPow2::from_exponent(31)),
            Some(i32::MIN)
        );
        assert_eq!(
            unbounded_ceil_to_multiple(0_i32, UnboundedPow2::from_exponent(31)),
            Some(0)
        );
        assert_eq!(
            unbounded_ceil_to_multiple(1_i32, UnboundedPow2::from_exponent(31)),
            None
        );
        assert_eq!(
            unbounded_ceil_to_multiple(-1_i32, UnboundedPow2::from_exponent(255)),
            Some(0)
        );
        assert_eq!(
            unbounded_ceil_to_multiple(0_i32, UnboundedPow2::from_exponent(255)),
            Some(0)
        );
        assert_eq!(
            unbounded_ceil_to_multiple(1_i32, UnboundedPow2::from_exponent(255)),
            None
        );
        assert_eq!(
            unbounded_ceil_to_multiple(i32::MIN, UnboundedPow2::from_exponent(255)),
            Some(0)
        );
        assert_eq!(
            unbounded_ceil_to_multiple(i32::MAX, UnboundedPow2::from_exponent(0)),
            Some(i32::MAX)
        );
        assert_eq!(
            unbounded_ceil_to_multiple(i32::MAX, UnboundedPow2::from_exponent(1)),
            None
        );

        assert_eq!(
            unbounded_ceil_to_multiple(0_u32, UnboundedPow2::from_exponent(31)),
            Some(0)
        );
        assert_eq!(
            unbounded_ceil_to_multiple(1_u32, UnboundedPow2::from_exponent(31)),
            Some(1 << 31)
        );
        assert_eq!(
            unbounded_ceil_to_multiple(0_u32, UnboundedPow2::from_exponent(32)),
            Some(0)
        );
        assert_eq!(
            unbounded_ceil_to_multiple(1_u32, UnboundedPow2::from_exponent(32)),
            None
        );
        assert_eq!(
            unbounded_ceil_to_multiple(0_u32, UnboundedPow2::from_exponent(255)),
            Some(0)
        );
        assert_eq!(
            unbounded_ceil_to_multiple(1_u32, UnboundedPow2::from_exponent(255)),
            None
        );
        assert_eq!(
            unbounded_ceil_to_multiple(i32::MAX as u32 + 2, UnboundedPow2::from_exponent(31)),
            None
        );
    }

    #[test]
    fn unb_pow2_checked_round_to_multiple_boundary() {
        assert_eq!(
            checked_round_to_multiple(0_i32, UnboundedPow2::from_exponent(30)),
            Some(0)
        );
        assert_eq!(
            checked_round_to_multiple(0_i32, UnboundedPow2::from_exponent(31)),
            Some(0)
        );
        assert_eq!(
            checked_round_to_multiple(i32::MAX, UnboundedPow2::from_exponent(0)),
            Some(i32::MAX)
        );
        assert_eq!(
            checked_round_to_multiple(i32::MAX, UnboundedPow2::from_exponent(1)),
            None
        );
        assert_eq!(
            checked_round_to_multiple(i32::MIN, UnboundedPow2::from_exponent(1)),
            Some(i32::MIN)
        );
        assert_eq!(
            checked_round_to_multiple(i32::MIN + 1, UnboundedPow2::from_exponent(1)),
            Some(i32::MIN)
        );

        assert_eq!(
            checked_round_to_multiple(0_u32, UnboundedPow2::from_exponent(31)),
            Some(0)
        );
        assert_eq!(
            checked_round_to_multiple(i32::MAX as u32 + 2, UnboundedPow2::from_exponent(31)),
            Some(i32::MAX as u32 + 1)
        );
        assert_eq!(
            checked_round_to_multiple(0_u32, UnboundedPow2::from_exponent(32)),
            None
        );
    }

    #[test]
    fn pow2_checked_round_to_multiple_boundary() {
        assert_eq!(
            checked_round_to_multiple(0_i32, Pow2::<u32>::from_exponent(30).unwrap()),
            Some(0)
        );
        assert_eq!(
            checked_round_to_multiple(0_i32, Pow2::<u32>::from_exponent(31).unwrap()),
            Some(0)
        );
        assert_eq!(
            checked_round_to_multiple(i32::MAX, Pow2::<u32>::from_exponent(0).unwrap()),
            Some(i32::MAX)
        );
        assert_eq!(
            checked_round_to_multiple(i32::MAX, Pow2::<u32>::from_exponent(1).unwrap()),
            None
        );
        assert_eq!(
            checked_round_to_multiple(i32::MIN, Pow2::<u32>::from_exponent(1).unwrap()),
            Some(i32::MIN)
        );
        assert_eq!(
            checked_round_to_multiple(i32::MIN + 1, Pow2::<u32>::from_exponent(1).unwrap()),
            Some(i32::MIN)
        );

        assert_eq!(
            checked_round_to_multiple(0_u32, Pow2::<u32>::from_exponent(31).unwrap()),
            Some(0)
        );
        assert_eq!(
            checked_round_to_multiple(i32::MAX as u32 + 2, Pow2::<u32>::from_exponent(31).unwrap()),
            Some(i32::MAX as u32 + 1)
        );
    }

    #[test]
    fn unb_pow2_unbounded_round_to_multiple_boundary() {
        assert_eq!(
            unbounded_round_to_multiple(0_i32, UnboundedPow2::from_exponent(30)),
            Some(0)
        );
        assert_eq!(
            unbounded_round_to_multiple(-1_i32, UnboundedPow2::from_exponent(31)),
            Some(0)
        );
        assert_eq!(
            unbounded_round_to_multiple(i32::MIN, UnboundedPow2::from_exponent(31)),
            Some(i32::MIN)
        );
        assert_eq!(
            unbounded_round_to_multiple(i32::MIN, UnboundedPow2::from_exponent(32)),
            None
        );
        assert_eq!(
            unbounded_round_to_multiple(0_i32, UnboundedPow2::from_exponent(31)),
            Some(0)
        );
        assert_eq!(
            unbounded_round_to_multiple(1_i32, UnboundedPow2::from_exponent(31)),
            Some(0)
        );
        assert_eq!(
            unbounded_round_to_multiple(-1_i32, UnboundedPow2::from_exponent(255)),
            Some(0)
        );
        assert_eq!(
            unbounded_round_to_multiple(0_i32, UnboundedPow2::from_exponent(255)),
            Some(0)
        );
        assert_eq!(
            unbounded_round_to_multiple(1_i32, UnboundedPow2::from_exponent(255)),
            Some(0)
        );
        assert_eq!(
            unbounded_round_to_multiple(i32::MIN, UnboundedPow2::from_exponent(255)),
            Some(0)
        );
        assert_eq!(
            unbounded_round_to_multiple(i32::MAX, UnboundedPow2::from_exponent(0)),
            Some(i32::MAX)
        );
        assert_eq!(
            unbounded_round_to_multiple(i32::MAX, UnboundedPow2::from_exponent(1)),
            None
        );
        assert_eq!(
            unbounded_round_to_multiple(i32::MAX as u32 + 1, UnboundedPow2::from_exponent(31)),
            Some(i32::MAX as u32 + 1)
        );
        assert_eq!(
            unbounded_round_to_multiple(i32::MAX as u32 + 1, UnboundedPow2::from_exponent(32)),
            None
        );

        assert_eq!(
            unbounded_round_to_multiple(0_u32, UnboundedPow2::from_exponent(31)),
            Some(0)
        );
        assert_eq!(
            unbounded_round_to_multiple(0_u32, UnboundedPow2::from_exponent(32)),
            Some(0)
        );
        assert_eq!(
            unbounded_round_to_multiple(1_u32, UnboundedPow2::from_exponent(32)),
            Some(0)
        );
        assert_eq!(
            unbounded_round_to_multiple(0_u32, UnboundedPow2::from_exponent(255)),
            Some(0)
        );
        assert_eq!(
            unbounded_round_to_multiple(1_u32, UnboundedPow2::from_exponent(255)),
            Some(0)
        );
    }

    #[test]
    fn unb_pow2_mod_floor_u64_exact() {
        assert_eq!(mod_floor(32u64, UnboundedPow2::from_exponent(5)), 0);
    }

    #[test]
    fn pow2_mod_floor_u64_exact() {
        assert_eq!(mod_floor(32u64, Pow2::<u8>::from_exponent(5).unwrap()), 0);
    }

    #[test]
    #[cfg(debug_assertions)]
    #[should_panic]
    fn unb_pow2_mod_floor_divisor_out_of_range() {
        let _ = mod_floor(32u64, UnboundedPow2::from_exponent(64));
    }

    #[test]
    fn unb_pow2_mod_floor_u64_remainder() {
        assert_eq!(mod_floor(37u64, UnboundedPow2::from_exponent(5)), 5);
        assert_eq!(mod_floor(63u64, UnboundedPow2::from_exponent(5)), 31);
    }

    #[test]
    fn pow2_mod_floor_u64_remainder() {
        assert_eq!(mod_floor(37u64, Pow2::<u8>::from_exponent(5).unwrap()), 5);
        assert_eq!(mod_floor(63u64, Pow2::<u8>::from_exponent(5).unwrap()), 31);
    }

    #[test]
    fn unb_pow2_mod_floor_i64_positive() {
        assert_eq!(mod_floor(37i64, UnboundedPow2::from_exponent(5)), 5);
    }

    #[test]
    fn pow2_mod_floor_i64_positive() {
        assert_eq!(mod_floor(37i64, Pow2::<u8>::from_exponent(5).unwrap()), 5);
    }

    #[test]
    fn unb_pow2_mod_floor_i64_negative() {
        assert_eq!(mod_floor(-1i64, UnboundedPow2::from_exponent(5)), 31);
    }

    #[test]
    fn pow2_mod_floor_i64_negative() {
        assert_eq!(mod_floor(-1i64, Pow2::<u8>::from_exponent(5).unwrap()), 31);
    }

    #[test]
    fn unb_pow2_mod_floor_min() {
        assert_eq!(mod_floor(i32::MIN, UnboundedPow2::from_exponent(2)), 0);
        assert_eq!(mod_floor(i32::MIN, UnboundedPow2::from_exponent(31)), 0);
    }

    #[test]
    fn pow2_mod_floor_min() {
        assert_eq!(
            mod_floor(i32::MIN, Pow2::<u8>::from_exponent(2).unwrap()),
            0
        );
        assert_eq!(
            mod_floor(i32::MIN, Pow2::<u32>::from_exponent(31).unwrap()),
            0
        );
    }

    #[test]
    fn unb_pow2_mod_floor_max() {
        assert_eq!(mod_floor(i32::MAX, UnboundedPow2::from_exponent(2)), 3);
        assert_eq!(
            mod_floor(i32::MAX, UnboundedPow2::from_exponent(31)),
            i32::MAX
        );
        assert_eq!(mod_floor(u32::MAX, UnboundedPow2::from_exponent(2)), 3);
        assert_eq!(
            mod_floor(u32::MAX, UnboundedPow2::from_exponent(31)),
            i32::MAX as u32
        );
    }

    #[test]
    fn pow2_mod_floor_max() {
        assert_eq!(
            mod_floor(i32::MAX, Pow2::<u8>::from_exponent(2).unwrap()),
            3
        );
        assert_eq!(
            mod_floor(i32::MAX, Pow2::<u32>::from_exponent(31).unwrap()),
            i32::MAX
        );
        assert_eq!(
            mod_floor(u32::MAX, Pow2::<u8>::from_exponent(2).unwrap()),
            3
        );
        assert_eq!(
            mod_floor(u32::MAX, Pow2::<u32>::from_exponent(31).unwrap()),
            i32::MAX as u32
        );
    }

    #[test]
    fn unb_pow2_mod_floor_i64_negative_is_always_nonnegative() {
        // div_floor(-37, 32) = -64, so mod = -37 - (-64) = 27
        assert_eq!(mod_floor(-37i64, UnboundedPow2::from_exponent(5)), 27);
        // div_floor(-32, 32) = -32, so mod = 0
        assert_eq!(mod_floor(-32i64, UnboundedPow2::from_exponent(5)), 0);
        // div_floor(-1, 32) = -32, so mod = 31
        assert_eq!(mod_floor(-1i64, UnboundedPow2::from_exponent(5)), 31);
    }

    #[test]
    fn pow2_mod_floor_i64_negative_is_always_nonnegative() {
        // div_floor(-37, 32) = -64, so mod = -37 - (-64) = 27
        assert_eq!(mod_floor(-37i64, Pow2::<u8>::from_exponent(5).unwrap()), 27);
        // div_floor(-32, 32) = -32, so mod = 0
        assert_eq!(mod_floor(-32i64, Pow2::<u8>::from_exponent(5).unwrap()), 0);
        // div_floor(-1, 32) = -32, so mod = 31
        assert_eq!(mod_floor(-1i64, Pow2::<u8>::from_exponent(5).unwrap()), 31);
    }

    #[test]
    fn unb_pow2_checked_mod_floor_boundary() {
        assert_eq!(
            checked_mod_floor(0_i32, UnboundedPow2::from_exponent(30)),
            Some(0)
        );
        assert_eq!(
            checked_mod_floor(0_i32, UnboundedPow2::from_exponent(31)),
            Some(0)
        );
        assert_eq!(
            checked_mod_floor(0_i32, UnboundedPow2::from_exponent(32)),
            None
        );

        assert_eq!(
            checked_mod_floor(0_u32, UnboundedPow2::from_exponent(31)),
            Some(0)
        );
        assert_eq!(
            checked_mod_floor(0_u32, UnboundedPow2::from_exponent(32)),
            None
        );
    }

    #[test]
    fn unb_pow2_is_multiple_of_u64_true() {
        assert!(is_multiple_of(0u64, UnboundedPow2::from_exponent(5)));
        assert!(is_multiple_of(32u64, UnboundedPow2::from_exponent(5)));
        assert!(is_multiple_of(64u64, UnboundedPow2::from_exponent(5)));
    }

    #[test]
    fn pow2_is_multiple_of_u64_true() {
        assert!(is_multiple_of(0u64, Pow2::<u64>::from_exponent(5).unwrap()));
        assert!(is_multiple_of(
            32u64,
            Pow2::<u64>::from_exponent(5).unwrap()
        ));
        assert!(is_multiple_of(
            64u64,
            Pow2::<u64>::from_exponent(5).unwrap()
        ));
    }

    #[test]
    fn unb_pow2_is_multiple_of_u64_false() {
        assert!(!is_multiple_of(31u64, UnboundedPow2::from_exponent(5)));
        assert!(!is_multiple_of(33u64, UnboundedPow2::from_exponent(5)));
    }

    #[test]
    fn pow2_is_multiple_of_u64_false() {
        assert!(!is_multiple_of(
            31u64,
            Pow2::<u64>::from_exponent(5).unwrap()
        ));
        assert!(!is_multiple_of(
            33u64,
            Pow2::<u64>::from_exponent(5).unwrap()
        ));
    }

    #[test]
    fn unb_pow2_is_multiple_of_i64_negative() {
        assert!(is_multiple_of(-32i64, UnboundedPow2::from_exponent(5)));
        assert!(is_multiple_of(i32::MIN, UnboundedPow2::from_exponent(31)));
        assert!(!is_multiple_of(-31i64, UnboundedPow2::from_exponent(5)));
    }

    #[test]
    fn pow2_is_multiple_of_i64_negative() {
        assert!(is_multiple_of(
            -32i64,
            Pow2::<u64>::from_exponent(5).unwrap()
        ));
        assert!(is_multiple_of(
            i32::MIN,
            Pow2::<u32>::from_exponent(31).unwrap()
        ));
        assert!(!is_multiple_of(
            -31i64,
            Pow2::<u64>::from_exponent(5).unwrap()
        ));
    }

    #[test]
    fn unb_pow2_unbounded_is_multiple_of_boundary() {
        assert!(unbounded_is_multiple_of(
            0_i32,
            UnboundedPow2::from_exponent(30)
        ));
        assert!(unbounded_is_multiple_of(
            i32::MIN,
            UnboundedPow2::from_exponent(31)
        ));
        assert!(unbounded_is_multiple_of(
            0_i32,
            UnboundedPow2::from_exponent(31)
        ));
        assert!(unbounded_is_multiple_of(
            0_i32,
            UnboundedPow2::from_exponent(32)
        ));
        assert!(unbounded_is_multiple_of(
            0_i32,
            UnboundedPow2::from_exponent(33)
        ));
        assert!(unbounded_is_multiple_of(
            0_i32,
            UnboundedPow2::from_exponent(255)
        ));
        assert!(!unbounded_is_multiple_of(
            i32::MIN,
            UnboundedPow2::from_exponent(255)
        ));
    }

    #[test]
    fn unb_pow2_div_floor_consistent_with_floor_to_multiple() {
        let b = UnboundedPow2::from_exponent(5); // 32
        for a in (-200i64..=200).step_by(7) {
            // floor_to_multiple gives the floored dividend times the divisor,
            // so recover the quotient via div_floor itself rather than `/`
            // (plain `/` truncates toward zero, not toward -∞).
            let q = div_floor(a, b);
            assert_eq!(floor_to_multiple(a, b), q * 32, "mismatch at a={a}");
        }
    }

    #[test]
    fn unb_pow2_floor_and_ceil_to_multiple_bracket_the_input() {
        let b = UnboundedPow2::from_exponent(4); // 16
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
    fn unb_pow2_mod_floor_plus_floor_to_multiple_equals_input() {
        let b = UnboundedPow2::from_exponent(5); // 32
        for a in (-300i64..=300).step_by(11) {
            let reconstructed = floor_to_multiple(a, b) + mod_floor(a, b);
            assert_eq!(reconstructed, a, "reconstruction failed at a={a}");
        }
    }

    #[test]
    fn unb_pow2_constants() {
        assert_eq!(UnboundedPow2::VAL_1, UnboundedPow2::from_exponent(0));
        assert_eq!(UnboundedPow2::VAL_2, UnboundedPow2::from_exponent(1));
        assert_eq!(UnboundedPow2::VAL_4, UnboundedPow2::from_exponent(2));
        assert_eq!(UnboundedPow2::VAL_8, UnboundedPow2::from_exponent(3));
        assert_eq!(UnboundedPow2::VAL_16, UnboundedPow2::from_exponent(4));
        assert_eq!(UnboundedPow2::VAL_32, UnboundedPow2::from_exponent(5));
        assert_eq!(UnboundedPow2::VAL_64, UnboundedPow2::from_exponent(6));
        assert_eq!(UnboundedPow2::VAL_128, UnboundedPow2::from_exponent(7));
        assert_eq!(UnboundedPow2::VAL_256, UnboundedPow2::from_exponent(8));
        assert_eq!(UnboundedPow2::VAL_512, UnboundedPow2::from_exponent(9));

        assert_eq!(UnboundedPow2::KIBI, UnboundedPow2::from_exponent(10));
        assert_eq!(UnboundedPow2::MEBI, UnboundedPow2::from_exponent(20));
        assert_eq!(UnboundedPow2::GIBI, UnboundedPow2::from_exponent(30));
        assert_eq!(UnboundedPow2::TEBI, UnboundedPow2::from_exponent(40));
        assert_eq!(UnboundedPow2::PEBI, UnboundedPow2::from_exponent(50));
        assert_eq!(UnboundedPow2::EXBI, UnboundedPow2::from_exponent(60));
        assert_eq!(UnboundedPow2::ZEBI, UnboundedPow2::from_exponent(70));
        assert_eq!(UnboundedPow2::YOBI, UnboundedPow2::from_exponent(80));
        assert_eq!(UnboundedPow2::ROBI, UnboundedPow2::from_exponent(90));
        assert_eq!(UnboundedPow2::QUEBI, UnboundedPow2::from_exponent(100));
    }

    #[test]
    fn unb_pow2_has_debug() {
        let v = UnboundedPow2::from_exponent(0);
        let mut s = String::new();
        write!(&mut s, "{:?}", v).unwrap();
        assert_eq!(s, "UnboundedPow2 { exponent: 0 }");
    }

    #[test]
    fn unb_pow2_has_hash() {
        let v = UnboundedPow2::from_exponent(123);
        let mut s0 = DefaultHasher::new();
        v.hash(&mut s0);
        let mut s1 = DefaultHasher::new();
        123u8.hash(&mut s1);
        assert_eq!(s0.finish(), s1.finish());
    }

    #[test]
    fn unb_pow2_has_eq() {
        assert_eq!(UnboundedPow2::VAL_1, UnboundedPow2::VAL_1);
        assert_eq!(UnboundedPow2::KIBI, UnboundedPow2::KIBI);
        assert_ne!(UnboundedPow2::VAL_1, UnboundedPow2::VAL_2);
    }

    #[test]
    fn unb_pow2_has_ord() {
        assert!(UnboundedPow2::VAL_1 < UnboundedPow2::VAL_2);
        assert!(UnboundedPow2::VAL_2 <= UnboundedPow2::VAL_2);
        assert!(UnboundedPow2::VAL_2 >= UnboundedPow2::VAL_2);
        assert!(UnboundedPow2::VAL_4 > UnboundedPow2::VAL_2);
    }
}
