fn main() {
    divan::main();
}

fn make_inputs_i8() -> [(i8, u8); 256] {
    let inputs: [(i8, u8); 256] =
        std::array::from_fn(|i| (13i8 + (i % 79) as i8, 1u8 + (i % 5) as u8));
    inputs
}

fn make_inputs_i16() -> [(i16, u8); 256] {
    let inputs: [(i16, u8); 256] =
        std::array::from_fn(|i| (1231i16 + i as i16, 1u8 + (i % 13) as u8));
    inputs
}

fn make_inputs_i32() -> [(i32, u8); 256] {
    let inputs: [(i32, u8); 256] =
        std::array::from_fn(|i| (123123123i32 + i as i32, 1u8 + (i % 13) as u8));
    inputs
}

fn make_inputs_i64() -> [(i64, u8); 256] {
    let inputs: [(i64, u8); 256] =
        std::array::from_fn(|i| (123123123123123i64 + i as i64, 1u8 + (i % 13) as u8));
    inputs
}

fn make_inputs_i64_2() -> [(i64, i64, u8); 256] {
    let inputs: [(i64, i64, u8); 256] = std::array::from_fn(|i| {
        (
            123123123123123i64 + i as i64,
            2151323151123i64 + i as i64,
            1u8 + (i % 13) as u8,
        )
    });
    inputs
}

fn make_inputs_i64_mask() -> [(i64, i64); 256] {
    let inputs: [(i64, i64); 256] =
        std::array::from_fn(|i| (123123123123123i64 + i as i64, 1 << (1u8 + (i % 13) as u8)));
    inputs
}

mod div_floor {
    use super::*;

    #[divan::bench]
    fn div_floor_i8(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| make_inputs_i8())
            .bench_values(|inputs| {
                for (a, b) in inputs {
                    let _ = divan::black_box(a >> b);
                }
            });
    }

    #[divan::bench]
    fn div_floor_i8_via_i32(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| make_inputs_i8())
            .bench_values(|inputs| {
                for (a, b) in inputs {
                    let _ = divan::black_box(((a as i32) >> b) as i8);
                }
            });
    }

    #[divan::bench]
    fn div_floor_i8_via_unchecked(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| make_inputs_i8())
            .bench_values(|inputs| {
                for (a, b) in inputs {
                    let _ = divan::black_box(unsafe { a.unchecked_shr(b as u32) });
                }
            });
    }

    #[divan::bench]
    fn div_floor_i32(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| make_inputs_i32())
            .bench_values(|inputs| {
                for (a, b) in inputs {
                    let _ = divan::black_box(a >> b);
                }
            });
    }

    #[divan::bench]
    fn div_floor_i32_via_unchecked(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| make_inputs_i32())
            .bench_values(|inputs| {
                for (a, b) in inputs {
                    let _ = divan::black_box(unsafe { a.unchecked_shr(b as u32) });
                }
            });
    }
}

mod ceil_to_multiple {
    use crate::make_inputs_i32;

    #[divan::bench]
    fn ceil_to_multiple_mask(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| make_inputs_i32())
            .bench_values(|inputs| {
                for (a, b) in inputs {
                    let mask = ((1u32 << b) - 1) as i32;
                    let _ = divan::black_box((a + mask) & !mask);
                }
            });
    }

    #[divan::bench]
    fn ceil_to_multiple_no_mask(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| make_inputs_i32())
            .bench_values(|inputs| {
                for (a, b) in inputs {
                    let floored = a >> b;
                    let rem = a - (floored << b);
                    let _ = divan::black_box((floored + (rem != 0) as i32) << b);
                }
            });
    }
}

mod div_ceil {
    use crate::{
        make_inputs_i16, make_inputs_i32, make_inputs_i64, make_inputs_i64_2, make_inputs_i64_mask,
    };

    #[divan::bench]
    fn div_ceil_mask_a_i32_via_i64(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| make_inputs_i32())
            .bench_values(|inputs| {
                for (a, b) in inputs {
                    let mask = (!(!0u32 << b)) as i64;
                    divan::black_box(((a as i64 + mask) >> b) as i32);
                }
            });
    }

    #[divan::bench]
    fn div_ceil_mask_b_i32_via_i64(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| make_inputs_i32())
            .bench_values(|inputs| {
                for (a, b) in inputs {
                    let mask = ((!0u32 >> 1) >> (31 - b)) as i64;
                    divan::black_box(((a as i64 + mask) >> b) as i32);
                }
            });
    }

    #[divan::bench]
    fn div_ceil_mask_c_i16_via_i64(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| make_inputs_i16())
            .bench_values(|inputs| {
                for (a, b) in inputs {
                    let mask = (1 << b) - 1;
                    divan::black_box(((a as i64 + mask) >> b) as i16);
                }
            });
    }

    #[divan::bench]
    fn div_ceil_mask_c_i32_via_i64(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| make_inputs_i32())
            .bench_values(|inputs| {
                for (a, b) in inputs {
                    let mask = (1 << b) - 1;
                    divan::black_box(((a as i64 + mask) >> b) as i32);
                }
            });
    }

    #[divan::bench]
    fn div_ceil_mask_c_i64_via_i128(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| make_inputs_i64())
            .bench_values(|inputs| {
                for (a, b) in inputs {
                    let mask = (1 << b) - 1;
                    divan::black_box(((a as i128 + mask) >> b) as i64);
                }
            });
    }

    #[divan::bench]
    fn div_ceil_no_mask_i16(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| make_inputs_i16())
            .bench_values(|inputs| {
                for (a, b) in inputs {
                    let floored = a >> b;
                    let rem = a - (floored << b);
                    divan::black_box(floored + (rem != 0) as i16);
                }
            });
    }

    #[divan::bench]
    fn div_ceil_no_mask_i64(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| make_inputs_i64())
            .bench_values(|inputs| {
                for (a, b) in inputs {
                    let floored = a >> b;
                    let rem = a - (floored << b);
                    divan::black_box(floored + (rem != 0) as i64);
                }
            });
    }

    #[divan::bench]
    fn div_ceil_no_mask_i64_2(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| make_inputs_i64_2())
            .bench_values(|inputs| {
                for (a, c, b) in inputs {
                    let flooreda = a >> b;
                    let flooredc = c >> b;
                    let rema = a - (flooreda << b);
                    let remc = c - (flooredc << b);
                    divan::black_box((
                        flooreda + (rema != 0) as i64,
                        flooredc + (remc != 0) as i64,
                    ));
                }
            });
    }

    #[divan::bench]
    fn div_ceil_no_mask_i64_kindamask(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| make_inputs_i64())
            .bench_values(|inputs| {
                for (a, b) in inputs {
                    let mask = (1 << b) - 1;
                    let floored = a >> b;
                    divan::black_box(floored + (a & mask != 0) as i64);
                }
            });
    }

    #[divan::bench]
    fn div_ceil_no_mask_i64_kindamask_2(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| make_inputs_i64_2())
            .bench_values(|inputs| {
                for (a, c, b) in inputs {
                    let mask = (1 << b) - 1;
                    let flooreda = a >> b;
                    let flooredc = c >> b;
                    divan::black_box((
                        flooreda + (a & mask != 0) as i64,
                        flooredc + (c & mask != 0) as i64,
                    ));
                }
            });
    }

    #[divan::bench]
    fn div_ceil_from_mask_i64(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| make_inputs_i64_mask())
            .bench_values(|inputs| {
                for (a, mask) in inputs {
                    let b = mask.trailing_ones();
                    let floored = a >> b;
                    divan::black_box(floored + (a & mask != 0) as i64);
                }
            });
    }
}

mod div_round {
    use crate::make_inputs_i64;

    // NOTE: We CANNOT do b-1, so all of these, while nice, don't cut it.
    #[divan::bench]
    fn div_round_a(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| make_inputs_i64())
            .bench_values(|inputs| {
                for (a, b) in inputs {
                    let floored = a >> b;
                    divan::black_box(floored + ((a & (1 << (b - 1))) != 0) as i64);
                }
            });
    }

    #[divan::bench]
    fn div_round_b(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| make_inputs_i64())
            .bench_values(|inputs| {
                for (a, b) in inputs {
                    let mask = (1 << (b - 1)) - 1;
                    divan::black_box((a + mask) >> b);
                }
            });
    }

    #[divan::bench]
    fn div_round_c(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| make_inputs_i64())
            .bench_values(|inputs| {
                for (a, b) in inputs {
                    let floored = a >> b;
                    divan::black_box(floored + (a >> (b - 1)) & 1);
                }
            });
    }

    #[divan::bench]
    fn div_round_d(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| make_inputs_i64())
            .bench_values(|inputs| {
                for (a, b) in inputs {
                    let floored = a >> (b - 1);
                    divan::black_box((floored + 1) >> 1);
                }
            });
    }
}

mod floor_to_multiple {
    use crate::make_inputs_i32;

    #[divan::bench]
    fn floor_to_multiple_mask(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| make_inputs_i32())
            .bench_values(|inputs| {
                for (a, b) in inputs {
                    let mask = ((1u32 << b) - 1) as i32;
                    divan::black_box(a & !mask);
                }
            });
    }

    #[divan::bench]
    fn floor_to_multiple_no_mask(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| make_inputs_i32())
            .bench_values(|inputs| {
                for (a, b) in inputs {
                    divan::black_box(a >> b << b);
                }
            });
    }
}

mod is_multiple_of {
    use crate::make_inputs_i32;

    #[divan::bench]
    fn is_multiple_of_mask(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| make_inputs_i32())
            .bench_values(|inputs| {
                for (a, b) in inputs {
                    let mask = ((1u32 << b) - 1) as i32;
                    divan::black_box((a & mask) == 0);
                }
            });
    }

    #[divan::bench]
    fn is_multiple_of_via_floor_to_multiple(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| make_inputs_i32())
            .bench_values(|inputs| {
                for (a, b) in inputs {
                    divan::black_box(a << b >> b == a);
                }
            });
    }

    #[divan::bench]
    fn is_multiple_of_via_tzc(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| make_inputs_i32())
            .bench_values(|inputs| {
                for (a, b) in inputs {
                    let tzc = a.trailing_zeros();
                    divan::black_box(b as u32 >= tzc);
                }
            });
    }
}
