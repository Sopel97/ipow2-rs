use ipow2::{Int, UnboundedPow2, Pow2OutOfRange, Pow2};

fn main() {
    divan::main();
}

trait MakeSample: Sized {
    fn make_sample_val_rhs(i: usize) -> Self;
    fn make_sample_unb_pow2_rhs(i: usize) -> UnboundedPow2;
    fn make_sample_val_lhs(i: usize) -> Self;
    fn make_one_sample_val_rhs() -> Self;
    fn make_one_sample_unb_pow2_rhs() -> UnboundedPow2;
    fn make_one_sample_pow2_rhs<T: Int>() -> Pow2<T::Unsigned>;
}

macro_rules! samples_for_int {
    ($t:ty, $signed:literal) => {
        impl MakeSample for $t {
            fn make_sample_val_rhs(i: usize) -> Self {
                1 << (i % <$t>::BITS as usize) as u8
            }

            fn make_sample_unb_pow2_rhs(i: usize) -> UnboundedPow2 {
                UnboundedPow2::from_exponent((i % <$t>::BITS as usize) as u8)
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

            fn make_one_sample_unb_pow2_rhs() -> UnboundedPow2 {
                UnboundedPow2::from_exponent((<$t>::BITS / 3) as u8)
            }

            fn make_one_sample_val_rhs() -> Self {
                1 << (<$t>::BITS / 3)
            }

            fn make_one_sample_pow2_rhs<T: Int>() -> Pow2<T::Unsigned> {
                Pow2::from_exponent((<$t>::BITS / 3) as u8).unwrap()
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

const INPUT_SAMPLE_COUNT: usize = 1024;
const DIVAN_SAMPLE_SIZE: u32 = 1000;

const CONST_UNB_POW2: UnboundedPow2 = UnboundedPow2::from_exponent(6);
const CONST_POW2: Pow2<u8> = Pow2::<u8>::VAL_64;
macro_rules! const_int {
    () => {
        64
    };
}

fn make_samples_val_lhs_unb_pow2_rhs<T: MakeSample>() -> ([T; INPUT_SAMPLE_COUNT], [UnboundedPow2; INPUT_SAMPLE_COUNT]) {
    (
        std::array::from_fn(T::make_sample_val_lhs),
        std::array::from_fn(T::make_sample_unb_pow2_rhs),
    )
}

fn make_samples_val_lhs_pow2_rhs<T: MakeSample + Int>() -> ([T; INPUT_SAMPLE_COUNT], [Pow2<T::Unsigned>; INPUT_SAMPLE_COUNT])
where Pow2<<T as Int>::Unsigned>: TryFrom<UnboundedPow2, Error=Pow2OutOfRange> {
    (
        std::array::from_fn(T::make_sample_val_lhs),
        std::array::from_fn(T::make_sample_unb_pow2_rhs).map(|b| Pow2::try_from(b).unwrap())
    )
}

fn make_samples_val_lhs_val_rhs<T: MakeSample>() -> ([T; INPUT_SAMPLE_COUNT], [T; INPUT_SAMPLE_COUNT]) {
    (
        std::array::from_fn(T::make_sample_val_lhs),
        std::array::from_fn(T::make_sample_val_rhs),
    )
}

fn make_samples_val_lhs<T: MakeSample>() -> [T; INPUT_SAMPLE_COUNT] {
    std::array::from_fn(T::make_sample_val_lhs)
}

fn make_samples_output<T: Default + Copy>() -> [T; INPUT_SAMPLE_COUNT] {
    [T::default(); INPUT_SAMPLE_COUNT]
}

macro_rules! make_bench_single {
    ($t:ty; $func_name:ident : for_unb_pow2 => $block:expr) => {
        #[divan::bench(sample_count = DIVAN_SAMPLE_SIZE)]
        fn $func_name(bencher: divan::Bencher) {
            let inputs = make_samples_val_lhs_unb_pow2_rhs::<$t>();
            bencher
                .with_inputs(|| (inputs, make_samples_output()))
                .bench_refs(|&mut ((lhs, rhs), mut outputs)| {
                    let len = lhs.len();
                    assert!(rhs.len() >= len);
                    assert!(outputs.len() >= len);
                    for i in 0..len {
                        outputs[i] = $block(lhs[i], rhs[i]);
                    }
                    std::hint::black_box(&mut outputs);
                });
        }
    };

    ($t:ty; $func_name:ident : for_pow2 => $block:expr) => {
        #[divan::bench(sample_count = DIVAN_SAMPLE_SIZE)]
        fn $func_name(bencher: divan::Bencher) {
            let inputs = make_samples_val_lhs_pow2_rhs::<$t>();
            bencher
                .with_inputs(|| (inputs, make_samples_output()))
                .bench_refs(|&mut ((lhs, rhs), mut outputs)| {
                    let len = lhs.len();
                    assert!(rhs.len() >= len);
                    assert!(outputs.len() >= len);
                    for i in 0..len {
                        outputs[i] = $block(lhs[i], rhs[i]);
                    }
                    std::hint::black_box(&mut outputs);
                });
        }
    };

    ($t:ty; $func_name:ident : for_unb_pow2_reuse => $block:expr) => {
        #[divan::bench(sample_count = DIVAN_SAMPLE_SIZE)]
        fn $func_name(bencher: divan::Bencher) {
            let lhs = make_samples_val_lhs::<$t>();
            bencher
                .with_inputs(|| (lhs, <$t>::make_one_sample_unb_pow2_rhs(), make_samples_output()))
                .bench_refs(|&mut (lhs, b, mut outputs)| {
                    let len = lhs.len();
                    assert!(outputs.len() >= len);
                    for i in 0..len {
                        outputs[i] = $block(lhs[i], b);
                    }
                    std::hint::black_box(&mut outputs);
                });
        }
    };

    ($t:ty; $func_name:ident : for_pow2_reuse => $block:expr) => {
        #[divan::bench(sample_count = DIVAN_SAMPLE_SIZE)]
        fn $func_name(bencher: divan::Bencher) {
            let lhs = make_samples_val_lhs::<$t>();
            bencher
                .with_inputs(|| (lhs, <$t>::make_one_sample_pow2_rhs::<$t>(), make_samples_output()))
                .bench_refs(|&mut (lhs, b, mut outputs)| {
                    let len = lhs.len();
                    assert!(outputs.len() >= len);
                    for i in 0..len {
                        outputs[i] = $block(lhs[i], b);
                    }
                    std::hint::black_box(&mut outputs);
                });
        }
    };

    ($t:ty; $func_name:ident : for_std_reuse => $block:expr) => {
        #[divan::bench(sample_count = DIVAN_SAMPLE_SIZE)]
        fn $func_name(bencher: divan::Bencher) {
            let lhs = make_samples_val_lhs::<$t>();
            bencher
                .with_inputs(|| (lhs, <$t>::make_one_sample_val_rhs(), make_samples_output()))
                .bench_refs(|&mut (lhs, b, mut outputs)| {
                    let len = lhs.len();
                    assert!(outputs.len() >= len);
                    for i in 0..len {
                        outputs[i] = $block(lhs[i], b);
                    }
                    std::hint::black_box(&mut outputs);
                });
        }
    };

    ($t:ty; $func_name:ident : for_std => $block:expr) => {
        #[divan::bench(sample_count = DIVAN_SAMPLE_SIZE)]
        fn $func_name(bencher: divan::Bencher) {
            let inputs = make_samples_val_lhs_val_rhs::<$t>();
            bencher
                .with_inputs(|| (inputs, make_samples_output()))
                .bench_refs(|&mut ((lhs, rhs), mut outputs)| {
                    let len = lhs.len();
                    assert!(rhs.len() >= len);
                    assert!(outputs.len() >= len);
                    for i in 0..len {
                        outputs[i] = $block(lhs[i], rhs[i]);
                    }
                    std::hint::black_box(&mut outputs);
                });
        }
    };

    ($t:ty; $func_name:ident : for_const => $block:expr) => {
        #[divan::bench(sample_count = DIVAN_SAMPLE_SIZE)]
        fn $func_name(bencher: divan::Bencher) {
            let lhs = make_samples_val_lhs::<$t>();
            bencher
                .with_inputs(|| (lhs, make_samples_output()))
                .bench_refs(|&mut (lhs, mut outputs)| {
                    let len = lhs.len();
                    assert!(outputs.len() >= len);
                    for i in 0..len {
                        outputs[i] = $block(lhs[i]);
                    }
                    std::hint::black_box(&mut outputs);
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
        baseline_identity        : for_const          => |a|    a,
        unb_pow2_div_floor       : for_unb_pow2       => |a, b| ipow2::div_floor(a, b),
        unb_pow2_div_floor_const : for_const          => |a|    ipow2::div_floor(a, CONST_UNB_POW2),
        unb_pow2_div_floor_reuse : for_unb_pow2_reuse => |a, b| ipow2::div_floor(a, b),
        pow2_div_floor           : for_pow2           => |a, b| ipow2::div_floor(a, b),
        pow2_div_floor_const     : for_const          => |a|    ipow2::div_floor(a, CONST_POW2),
        pow2_div_floor_reuse     : for_pow2_reuse     => |a, b| ipow2::div_floor(a, b),
        unb_pow2_div_ceil        : for_unb_pow2       => |a, b| ipow2::div_ceil(a, b),
        unb_pow2_div_ceil_const  : for_const          => |a|    ipow2::div_ceil(a, CONST_UNB_POW2),
        unb_pow2_div_ceil_reuse  : for_unb_pow2_reuse => |a, b| ipow2::div_ceil(a, b),
        pow2_div_ceil            : for_pow2           => |a, b| ipow2::div_ceil(a, b),
        pow2_div_ceil_const      : for_const          => |a|    ipow2::div_ceil(a, CONST_POW2),
        pow2_div_ceil_reuse      : for_pow2_reuse     => |a, b| ipow2::div_ceil(a, b),
        unb_pow2_div             : for_unb_pow2       => |a, b| a / b,
        unb_pow2_div_const       : for_const          => |a|    a / CONST_UNB_POW2,
        unb_pow2_div_reuse       : for_unb_pow2_reuse => |a, b| a / b,
        pow2_div                 : for_pow2           => |a, b| a / b,
        pow2_div_const           : for_const          => |a|    a / CONST_POW2,
        pow2_div_reuse           : for_pow2_reuse     => |a, b| a / b,
        std_div                  : for_std            => |a, b| a / b,
        std_div_const            : for_const          => |a|    a / const_int!(),
        std_div_reuse            : for_std_reuse      => |a, b| a / b,
    );
}

mod mul {
    use super::*;

    #[rustfmt::skip]
    make_bench_for_all_ints!(
        baseline_identity  : for_const          => |a|    a,
        unb_pow2_mul       : for_unb_pow2       => |a, b| a * b,
        unb_pow2_mul_const : for_const          => |a|    a * CONST_UNB_POW2,
        unb_pow2_mul_reuse : for_unb_pow2_reuse => |a, b| a * b,
        pow2_mul           : for_pow2           => |a, b| a * b,
        pow2_mul_const     : for_const          => |a|    a * CONST_POW2,
        pow2_mul_reuse     : for_pow2_reuse     => |a, b| a * b,
        std_mul            : for_std            => |a, b| a * b,
        std_mul_const      : for_const          => |a|    a * const_int!(),
        std_mul_reuse      : for_std_reuse      => |a, b| a * b,
    );
}

mod round {
    use super::*;

    #[rustfmt::skip]
    make_bench_for_all_ints!(
        baseline_identity                : for_const          => |a|    a,
        unb_pow2_floor_to_multiple       : for_unb_pow2       => |a, b| ipow2::floor_to_multiple(a, b),
        unb_pow2_floor_to_multiple_const : for_const          => |a|    ipow2::floor_to_multiple(a, CONST_UNB_POW2),
        unb_pow2_floor_to_multiple_reuse : for_unb_pow2_reuse => |a, b| ipow2::floor_to_multiple(a, b),
        pow2_floor_to_multiple           : for_pow2           => |a, b| ipow2::floor_to_multiple(a, b),
        pow2_floor_to_multiple_const     : for_const          => |a|    ipow2::floor_to_multiple(a, CONST_POW2),
        pow2_floor_to_multiple_reuse     : for_pow2_reuse     => |a, b| ipow2::floor_to_multiple(a, b),
        unb_pow2_ceil_to_multiple        : for_unb_pow2       => |a, b| ipow2::ceil_to_multiple(a, b),
        unb_pow2_ceil_to_multiple_const  : for_const          => |a|    ipow2::ceil_to_multiple(a, CONST_UNB_POW2),
        unb_pow2_ceil_to_multiple_reuse  : for_unb_pow2_reuse => |a, b| ipow2::ceil_to_multiple(a, b),
        pow2_ceil_to_multiple            : for_pow2           => |a, b| ipow2::ceil_to_multiple(a, b),
        pow2_ceil_to_multiple_const      : for_const          => |a|    ipow2::ceil_to_multiple(a, CONST_POW2),
        pow2_ceil_to_multiple_reuse      : for_pow2_reuse     => |a, b| ipow2::ceil_to_multiple(a, b),
        std_div_mul                      : for_std            => |a, b| a / b * b,
        std_div_mul_const                : for_const          => |a|    a / const_int!() * const_int!(),
        std_div_mul_reuse                : for_std_reuse      => |a, b| a / b * b,
    );
}

mod modulo {
    use super::*;

    #[rustfmt::skip]
    make_bench_for_all_ints!(
        baseline_identity             : for_const          => |a|    a,
        unb_pow2_is_multiple_of       : for_unb_pow2       => |a, b| ipow2::is_multiple_of(a, b),
        unb_pow2_is_multiple_of_const : for_const          => |a|    ipow2::is_multiple_of(a, CONST_UNB_POW2),
        unb_pow2_is_multiple_of_reuse : for_unb_pow2_reuse => |a, b| ipow2::is_multiple_of(a, b),
        pow2_is_multiple_of           : for_pow2           => |a, b| ipow2::is_multiple_of(a, b),
        pow2_is_multiple_of_const     : for_const          => |a|    ipow2::is_multiple_of(a, CONST_POW2),
        pow2_is_multiple_of_reuse     : for_pow2_reuse     => |a, b| ipow2::is_multiple_of(a, b),
        unb_pow2_mod_floor            : for_unb_pow2       => |a, b| ipow2::mod_floor(a, b),
        unb_pow2_mod_floor_const      : for_const          => |a|    ipow2::mod_floor(a, CONST_UNB_POW2),
        unb_pow2_mod_floor_reuse      : for_unb_pow2_reuse => |a, b| ipow2::mod_floor(a, b),
        pow2_mod_floor                : for_pow2           => |a, b| ipow2::mod_floor(a, b),
        pow2_mod_floor_const          : for_const          => |a|    ipow2::mod_floor(a, CONST_POW2),
        pow2_mod_floor_reuse          : for_pow2_reuse     => |a, b| ipow2::mod_floor(a, b),
        std_mod                       : for_std            => |a, b| a % b,
        std_mod_const                 : for_const          => |a|    a % const_int!(),
        std_mod_reuse                 : for_std_reuse      => |a, b| a % b,
    );
}
