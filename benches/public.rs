use ipow2::{Int, Pow2, Pow2OutOfRange, SafePow2};

fn main() {
    divan::main();
}

trait MakeSample: Sized {
    fn make_sample_val_lhs_val_rhs(i: usize) -> (Self, Self);
    fn make_sample_val_lhs_pow2_rhs(i: usize) -> (Self, Pow2);
    fn make_sample_val_lhs(i: usize) -> Self;
    fn make_sample_val_rhs() -> Self;
    fn make_sample_pow2_rhs() -> Pow2;
    fn make_sample_safe_pow2_rhs<T: Int>() -> SafePow2<T::Unsigned>;
}

macro_rules! samples_for_int {
    ($t:ty, $signed:literal) => {
        impl MakeSample for $t {
            fn make_sample_val_lhs_val_rhs(i: usize) -> (Self, Self) {
                (
                    Self::make_sample_val_lhs(i),
                    1 << (i % <$t>::BITS as usize) as u8,
                )
            }

            fn make_sample_val_lhs_pow2_rhs(i: usize) -> (Self, Pow2) {
                (
                    Self::make_sample_val_lhs(i),
                    Pow2::from_exponent((i % <$t>::BITS as usize) as u8),
                )
            }

            fn make_sample_val_lhs(i: usize) -> Self {
                if $signed {
                    if i.is_multiple_of(2) {
                        ((<$t>::MIN as i128) / (i + 1) as i128) as Self
                    } else {
                        ((<$t>::MAX as i128) / (i + 1) as i128) as Self
                    }
                } else {
                    ((<$t>::MAX as u128) / (i + 1) as u128) as Self
                }
            }

            fn make_sample_pow2_rhs() -> Pow2 {
                Pow2::from_exponent((<$t>::BITS / 3) as u8)
            }

            fn make_sample_val_rhs() -> Self {
                1 << (<$t>::BITS / 3)
            }

            fn make_sample_safe_pow2_rhs<T: Int>() -> SafePow2<T::Unsigned> {
                SafePow2::from_exponent((<$t>::BITS / 3) as u8).unwrap()
            }
        }
    };
}

samples_for_int!(i8, true);
samples_for_int!(i16, true);
samples_for_int!(i32, true);
samples_for_int!(i64, true);
samples_for_int!(i128, true);
samples_for_int!(u8, false);
samples_for_int!(u16, false);
samples_for_int!(u32, false);
samples_for_int!(u64, false);
samples_for_int!(u128, false);

const INPUT_SAMPLE_COUNT: usize = 1000;
const DIVAN_SAMPLE_SIZE: u32 = 1000;

const CONST_POW2: Pow2 = Pow2::from_exponent(6);
const CONST_SAFE_POW2: SafePow2<u8> = SafePow2::<u8>::VAL_64;
macro_rules! const_int {
    () => {
        64
    };
}

fn make_samples_val_lhs_pow2_rhs<T: MakeSample>() -> [(T, Pow2); INPUT_SAMPLE_COUNT] {
    std::array::from_fn(T::make_sample_val_lhs_pow2_rhs)
}

fn make_samples_val_lhs_safe_pow2_rhs<T: MakeSample + Int>() -> [(T, SafePow2<T::Unsigned>); INPUT_SAMPLE_COUNT]
where SafePow2<<T as Int>::Unsigned>: TryFrom<Pow2, Error=Pow2OutOfRange> {
    std::array::from_fn(T::make_sample_val_lhs_pow2_rhs).map(|(a, b)| (a, SafePow2::try_from(b).unwrap()))
}

fn make_samples_val_lhs_val_rhs<T: MakeSample>() -> [(T, T); INPUT_SAMPLE_COUNT] {
    std::array::from_fn(T::make_sample_val_lhs_val_rhs)
}

fn make_samples_val_lhs<T: MakeSample>() -> [T; INPUT_SAMPLE_COUNT] {
    std::array::from_fn(T::make_sample_val_lhs)
}

macro_rules! make_bench_single {
    ($t:ty; $func_name:ident : for_pow2 => $block:expr) => {
        #[divan::bench(sample_count = DIVAN_SAMPLE_SIZE)]
        fn $func_name(bencher: divan::Bencher) {
            bencher
                .with_inputs(|| make_samples_val_lhs_pow2_rhs::<$t>())
                .bench_values(|inputs| {
                    for (a, b) in inputs {
                        divan::black_box($block(a, b));
                    }
                });
        }
    };

    ($t:ty; $func_name:ident : for_safe_pow2 => $block:expr) => {
        #[divan::bench(sample_count = DIVAN_SAMPLE_SIZE)]
        fn $func_name(bencher: divan::Bencher) {
            bencher
                .with_inputs(|| make_samples_val_lhs_safe_pow2_rhs::<$t>())
                .bench_values(|inputs| {
                    for (a, b) in inputs {
                        divan::black_box($block(a, b));
                    }
                });
        }
    };

    ($t:ty; $func_name:ident : for_pow2_reuse => $block:expr) => {
        #[divan::bench(sample_count = DIVAN_SAMPLE_SIZE)]
        fn $func_name(bencher: divan::Bencher) {
            bencher
                .with_inputs(|| (make_samples_val_lhs::<$t>(), <$t>::make_sample_pow2_rhs()))
                .bench_values(|(inputs, b)| {
                    for a in inputs {
                        divan::black_box($block(a, b));
                    }
                });
        }
    };

    ($t:ty; $func_name:ident : for_safe_pow2_reuse => $block:expr) => {
        #[divan::bench(sample_count = DIVAN_SAMPLE_SIZE)]
        fn $func_name(bencher: divan::Bencher) {
            bencher
                .with_inputs(|| (make_samples_val_lhs::<$t>(), <$t>::make_sample_safe_pow2_rhs::<$t>()))
                .bench_values(|(inputs, b)| {
                    for a in inputs {
                        divan::black_box($block(a, b));
                    }
                });
        }
    };

    ($t:ty; $func_name:ident : for_std_reuse => $block:expr) => {
        #[divan::bench(sample_count = DIVAN_SAMPLE_SIZE)]
        fn $func_name(bencher: divan::Bencher) {
            bencher
                .with_inputs(|| (make_samples_val_lhs::<$t>(), <$t>::make_sample_val_rhs()))
                .bench_values(|(inputs, b)| {
                    for a in inputs {
                        divan::black_box($block(a, b));
                    }
                });
        }
    };

    ($t:ty; $func_name:ident : for_std => $block:expr) => {
        #[divan::bench(sample_count = DIVAN_SAMPLE_SIZE)]
        fn $func_name(bencher: divan::Bencher) {
            bencher
                .with_inputs(|| make_samples_val_lhs_val_rhs::<$t>())
                .bench_values(|inputs| {
                    for (a, b) in inputs {
                        divan::black_box($block(a, b));
                    }
                });
        }
    };

    ($t:ty; $func_name:ident : for_const => $block:expr) => {
        #[divan::bench(sample_count = DIVAN_SAMPLE_SIZE)]
        fn $func_name(bencher: divan::Bencher) {
            bencher
                .with_inputs(|| make_samples_val_lhs::<$t>())
                .bench_values(|inputs| {
                    for a in inputs {
                        divan::black_box($block(a));
                    }
                });
        }
    };
}

macro_rules! make_bench_inner {
    ($t:ty; [ $( $func_name:ident : $input:ident => $block:expr ),* $(,)? ]) => {
        $(
            make_bench_single!($t; $func_name: $input => $block);
        )*
    };
}

macro_rules! make_bench {
    (
        @modules [ $( $mod_name:ident : $t:ty ),* $(,)? ]
        @funcs $funcs:tt
    ) => {
        $(
            mod $mod_name {
                use super::*;

                make_bench_inner!($t; $funcs);
            }
        )*
    };

    (
        $( $mod_name:ident : $t:ty ),* $(,)?
        ;
        $( $func_name:ident : $input:ident => $block:expr ),* $(,)?
    ) => {
        make_bench!(
            @modules [ $( $mod_name : $t ),* ]
            @funcs [ $( $func_name : $input => $block ),* ]
        );
    };
}

macro_rules! make_bench_for_all_ints {
    ($( $func_name:ident : $input:ident => $block:expr ),* $(,)?) => {
        make_bench!(
            @modules [
                i8:  i8,
                i16: i16,
                i32: i32,
                i64: i64,
                i128: i128,
                u8: u8,
                u16: u16,
                u32: u32,
                u64: u64,
                u128: u128,
            ]
            @funcs [ $( $func_name : $input => $block ),* ]
        );
    };
}

mod div {
    use super::*;

    #[rustfmt::skip]
    make_bench_for_all_ints!(
        pow2_div_floor            : for_pow2            => |a, b| ipow2::div_floor(a, b),
        pow2_div_floor_const      : for_const           => |a|    ipow2::div_floor(a, CONST_POW2),
        pow2_div_floor_reuse      : for_pow2_reuse      => |a, b| ipow2::div_floor(a, b),
        safe_pow2_div_floor       : for_safe_pow2       => |a, b| ipow2::div_floor(a, b),
        safe_pow2_div_floor_const : for_const           => |a|    ipow2::div_floor(a, CONST_SAFE_POW2),
        safe_pow2_div_floor_reuse : for_safe_pow2_reuse => |a, b| ipow2::div_floor(a, b),
        pow2_div_ceil             : for_pow2            => |a, b| ipow2::div_ceil(a, b),
        pow2_div_ceil_const       : for_const           => |a|    ipow2::div_ceil(a, CONST_POW2),
        pow2_div_ceil_reuse       : for_pow2_reuse      => |a, b| ipow2::div_ceil(a, b),
        safe_pow2_div_ceil        : for_safe_pow2       => |a, b| ipow2::div_ceil(a, b),
        safe_pow2_div_ceil_const  : for_const           => |a|    ipow2::div_ceil(a, CONST_SAFE_POW2),
        safe_pow2_div_ceil_reuse  : for_safe_pow2_reuse => |a, b| ipow2::div_ceil(a, b),
        pow2_div                  : for_pow2            => |a, b| a / b,
        pow2_div_const            : for_const           => |a|    a / CONST_POW2,
        pow2_div_reuse            : for_pow2_reuse      => |a, b| a / b,
        safe_pow2_div             : for_safe_pow2       => |a, b| a / b,
        safe_pow2_div_const       : for_const           => |a|    a / CONST_SAFE_POW2,
        safe_pow2_div_reuse       : for_safe_pow2_reuse => |a, b| a / b,
        std_div                   : for_std             => |a, b| a / b,
        std_div_const             : for_const           => |a|    a / const_int!(),
        std_div_reuse             : for_std_reuse       => |a, b| a / b,
    );
}

mod mul {
    use super::*;

    #[rustfmt::skip]
    make_bench_for_all_ints!(
        pow2_mul            : for_pow2            => |a, b| a * b,
        pow2_mul_const      : for_const           => |a|    a * CONST_POW2,
        pow2_mul_reuse      : for_pow2_reuse      => |a, b| a * b,
        safe_pow2_mul       : for_safe_pow2       => |a, b| a * b,
        safe_pow2_mul_const : for_const           => |a|    a * CONST_SAFE_POW2,
        safe_pow2_mul_reuse : for_safe_pow2_reuse => |a, b| a * b,
        std_mul             : for_std             => |a, b| a * b,
        std_mul_const       : for_const           => |a|    a * const_int!(),
        std_mul_reuse       : for_std_reuse       => |a, b| a * b,
    );
}

mod round {
    use super::*;

    #[rustfmt::skip]
    make_bench_for_all_ints!(
        pow2_floor_to_multiple            : for_pow2            => |a, b| ipow2::floor_to_multiple(a, b),
        pow2_floor_to_multiple_const      : for_const           => |a|    ipow2::floor_to_multiple(a, CONST_POW2),
        pow2_floor_to_multiple_reuse      : for_pow2_reuse      => |a, b| ipow2::floor_to_multiple(a, b),
        safe_pow2_floor_to_multiple       : for_safe_pow2       => |a, b| ipow2::floor_to_multiple(a, b),
        safe_pow2_floor_to_multiple_const : for_const           => |a|    ipow2::floor_to_multiple(a, CONST_SAFE_POW2),
        safe_pow2_floor_to_multiple_reuse : for_safe_pow2_reuse => |a, b| ipow2::floor_to_multiple(a, b),
        pow2_ceil_to_multiple             : for_pow2            => |a, b| ipow2::ceil_to_multiple(a, b),
        pow2_ceil_to_multiple_const       : for_const           => |a|    ipow2::ceil_to_multiple(a, CONST_POW2),
        pow2_ceil_to_multiple_reuse       : for_pow2_reuse      => |a, b| ipow2::ceil_to_multiple(a, b),
        safe_pow2_ceil_to_multiple        : for_safe_pow2       => |a, b| ipow2::ceil_to_multiple(a, b),
        safe_pow2_ceil_to_multiple_const  : for_const           => |a|    ipow2::ceil_to_multiple(a, CONST_SAFE_POW2),
        safe_pow2_ceil_to_multiple_reuse  : for_safe_pow2_reuse => |a, b| ipow2::ceil_to_multiple(a, b),
        std_div_mul                       : for_std             => |a, b| a / b * b,
        std_div_mul_const                 : for_const           => |a|    a / const_int!() * const_int!(),
        std_div_mul_reuse                 : for_std_reuse       => |a, b| a / b * b,
    );
}

mod modulo {
    use super::*;

    #[rustfmt::skip]
    make_bench_for_all_ints!(
        pow2_is_multiple_of            : for_pow2            => |a, b| ipow2::is_multiple_of(a, b),
        pow2_is_multiple_of_const      : for_const           => |a|    ipow2::is_multiple_of(a, CONST_POW2),
        pow2_is_multiple_of_reuse      : for_pow2_reuse      => |a, b| ipow2::is_multiple_of(a, b),
        safe_pow2_is_multiple_of       : for_safe_pow2       => |a, b| ipow2::is_multiple_of(a, b),
        safe_pow2_is_multiple_of_const : for_const           => |a|    ipow2::is_multiple_of(a, CONST_SAFE_POW2),
        safe_pow2_is_multiple_of_reuse : for_safe_pow2_reuse => |a, b| ipow2::is_multiple_of(a, b),
        pow2_mod_floor                 : for_pow2            => |a, b| ipow2::mod_floor(a, b),
        pow2_mod_floor_const           : for_const           => |a|    ipow2::mod_floor(a, CONST_POW2),
        pow2_mod_floor_reuse           : for_pow2_reuse      => |a, b| ipow2::mod_floor(a, b),
        safe_pow2_mod_floor            : for_safe_pow2       => |a, b| ipow2::mod_floor(a, b),
        safe_pow2_mod_floor_const      : for_const           => |a|    ipow2::mod_floor(a, CONST_SAFE_POW2),
        safe_pow2_mod_floor_reuse      : for_safe_pow2_reuse => |a, b| ipow2::mod_floor(a, b),
        std_mod                        : for_std             => |a, b| a % b,
        std_mod_const                  : for_const           => |a|    a % const_int!(),
        std_mod_reuse                  : for_std_reuse       => |a, b| a % b,
    );
}
