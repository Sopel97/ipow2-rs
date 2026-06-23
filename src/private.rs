use std::ops::{Add, BitAnd, Not, Shl, Shr, Sub};

pub trait Int
where
    Self: Sized
        + Copy
        + Clone
        + Eq
        + PartialEq
        + Ord
        + PartialOrd
        + Shr<u8, Output = Self>
        + Shl<u8, Output = Self>
        + Sub<Output = Self>
        + Add<Output = Self>
        + Not<Output = Self>
        + BitAnd<Output = Self>,
{
    type Signed: Int;
    type Unsigned: Int;

    fn is_power_of_two(self) -> bool;
    fn ilog2(self) -> u32;
    fn cast_signed(self) -> Self::Signed;
    fn cast_unsigned(self) -> Self::Unsigned;
    fn from_bool(b: bool) -> Self;
    fn is_zero(self) -> bool;
    fn is_not_zero(self) -> bool;
    fn zero() -> Self;
    fn one() -> Self;
    fn minus_one() -> Self;
    fn min_value() -> Self;
    fn safe_shift_bits() -> u32;
    fn is_signed() -> bool;
    fn is_unsigned() -> bool;
    fn checked_shl(self, rhs: u32) -> Option<Self>;
    fn checked_add(self, rhs: Self) -> Option<Self>;
    fn mask(bits: u32) -> Self;
}

macro_rules! impl_common_int {
    ($t:ty) => {
        #[inline(always)]
        fn ilog2(self) -> u32 {
            self.ilog2()
        }

        #[inline(always)]
        fn from_bool(b: bool) -> Self {
            b as Self
        }

        #[inline(always)]
        fn is_zero(self) -> bool {
            self == 0
        }

        #[inline(always)]
        fn is_not_zero(self) -> bool {
            self != 0
        }

        #[inline(always)]
        fn zero() -> Self {
            0
        }

        #[inline(always)]
        fn one() -> Self {
            1
        }

        #[inline(always)]
        fn min_value() -> Self {
            <$t>::MIN
        }
        
        #[inline(always)]
        fn checked_shl(self, rhs: u32) -> Option<Self> {
            <$t>::checked_shl(self, rhs)
        }

        #[inline(always)]
        fn checked_add(self, rhs: Self) -> Option<Self> {
            <$t>::checked_add(self, rhs)
        }
    };
}

macro_rules! impl_uint {
    ($t:ty, $t_sint:ty) => {
        impl_common_int!($t);

        #[inline(always)]
        fn is_power_of_two(self) -> bool {
            self.is_power_of_two()
        }

        #[inline(always)]
        fn cast_signed(self) -> $t_sint {
            self.cast_signed()
        }

        #[inline(always)]
        fn cast_unsigned(self) -> $t {
            self
        }

        #[inline(always)]
        fn safe_shift_bits() -> u32 {
            <$t>::BITS - 1
        }

        #[inline(always)]
        fn minus_one() -> Self {
            <$t>::MAX
        }

        #[inline(always)]
        fn is_signed() -> bool {
            false
        }

        #[inline(always)]
        fn is_unsigned() -> bool {
            true
        }

        #[inline(always)]
        fn mask(bits: u32) -> Self {
            debug_assert!(bits <= Self::safe_shift_bits());
            (1 << bits) - 1
        }
    };
}

macro_rules! impl_sint {
    ($t:ty, $t_uint:ty) => {
        impl_common_int!($t);

        #[inline(always)]
        fn is_power_of_two(self) -> bool {
            self > 0 && self.cast_unsigned().is_power_of_two()
        }

        #[inline(always)]
        fn cast_signed(self) -> $t {
            self
        }

        #[inline(always)]
        fn cast_unsigned(self) -> $t_uint {
            self.cast_unsigned()
        }

        #[inline(always)]
        fn safe_shift_bits() -> u32 {
            <$t>::BITS - 2
        }

        #[inline(always)]
        fn minus_one() -> Self {
            -1
        }

        #[inline(always)]
        fn is_signed() -> bool {
            true
        }

        #[inline(always)]
        fn is_unsigned() -> bool {
            false
        }

        #[inline(always)]
        fn mask(bits: u32) -> Self {
            debug_assert!(bits <= Self::Unsigned::safe_shift_bits());
            // The mask computation needs to be done as unsigned, because it can overflow
            // to the sign bit before we subtract one.
            (((1 as $t_uint) << bits) - 1) as $t
        }
    };
}

macro_rules! impl_int {
    ($t_uint:ty, $t_sint:ty) => {
        impl Int for $t_uint {
            type Unsigned = $t_uint;
            type Signed = $t_sint;

            impl_uint!($t_uint, $t_sint);
        }

        impl Int for $t_sint {
            type Unsigned = $t_uint;
            type Signed = $t_sint;

            impl_sint!($t_sint, $t_uint);
        }
    };
}

impl_int!(u8, i8);
impl_int!(u16, i16);
impl_int!(u32, i32);
impl_int!(u64, i64);
impl_int!(u128, i128);
impl_int!(usize, isize);

macro_rules! impl_signed_unsigned_trait {
    ($trait:ident, trait_body $base_body:tt, signed_body $signed_body:tt, unsigned_body $unsigned_body:tt) => {
        trait $trait: Int $base_body

        impl $trait for u8  $unsigned_body
        impl $trait for u16 $unsigned_body
        impl $trait for u32 $unsigned_body
        impl $trait for u64 $unsigned_body
        impl $trait for u128 $unsigned_body
        impl $trait for usize $unsigned_body

        impl $trait for i8  $signed_body
        impl $trait for i16 $signed_body
        impl $trait for i32 $signed_body
        impl $trait for i64 $signed_body
        impl $trait for i128 $signed_body
        impl $trait for isize $signed_body
    };
}

macro_rules! impl_for_signed {
    ($m:ident) => {
        $m!(i8);
        $m!(i16);
        $m!(i32);
        $m!(i64);
        $m!(i128);
        $m!(isize);
    }
}

macro_rules! impl_for_unsigned {
    ($m:ident) => {
        $m!(u8);
        $m!(u16);
        $m!(u32);
        $m!(u64);
        $m!(u128);
        $m!(usize);
    }
}
