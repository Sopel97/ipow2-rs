pub trait Int
where
    Self: Sized + Copy + Clone,
{
    type Signed;
    type Unsigned;

    fn is_power_of_two(self) -> bool;
    fn ilog2(self) -> u32;
    fn cast_signed(self) -> Self::Signed;
    fn cast_unsigned(self) -> Self::Unsigned;
}

macro_rules! impl_common_int {
    ($t:ty) => {
        #[inline(always)]
        fn ilog2(self) -> u32 {
            self.ilog2()
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
