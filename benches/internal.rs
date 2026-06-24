fn main() {
    divan::main();
}

fn make_inputs() -> [(i32, u8); 256] {
    let inputs: [(i32, u8); 256] =
        std::array::from_fn(|i| (123123123i32 + i as i32, 1u8 + (i % 13) as u8));
    inputs
}

mod ceil_to_multiple {
    use crate::make_inputs;

    #[divan::bench]
    fn ceil_to_multiple_mask(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| make_inputs())
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
            .with_inputs(|| make_inputs())
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
    use crate::make_inputs;

    #[divan::bench]
    fn div_ceil_mask_a(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| make_inputs())
            .bench_values(|inputs| {
                for (a, b) in inputs {
                    let mask = (!(!0u32 << b)) as i64;
                    let _ = divan::black_box(((a as i64 + mask) >> b) as i32);
                }
            });
    }

    #[divan::bench]
    fn div_ceil_mask_b(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| make_inputs())
            .bench_values(|inputs| {
                for (a, b) in inputs {
                    let mask = ((!0u32 >> 1) >> (31 - b)) as i64;
                    let _ = divan::black_box(((a as i64 + mask) >> b) as i32);
                }
            });
    }

    #[divan::bench]
    fn div_ceil_mask_c(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| make_inputs())
            .bench_values(|inputs| {
                for (a, b) in inputs {
                    let mask = (1 << b) - 1;
                    divan::black_box(((a as i64 + mask) >> b) as i32);
                }
            });
    }

    #[divan::bench]
    fn div_ceil_no_mask(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| make_inputs())
            .bench_values(|inputs| {
                for (a, b) in inputs {
                    let floored = a >> b;
                    let rem = a - (floored << b);
                    divan::black_box(floored + (rem != 0) as i32);
                }
            });
    }
}

mod floor_to_multiple {
    use crate::make_inputs;

    #[divan::bench]
    fn floor_to_multiple_mask(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| make_inputs())
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
            .with_inputs(|| make_inputs())
            .bench_values(|inputs| {
                for (a, b) in inputs {
                    divan::black_box(a >> b << b);
                }
            });
    }
}

mod is_multiple_of {
    use crate::make_inputs;

    #[divan::bench]
    fn is_multiple_of_mask(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| make_inputs())
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
            .with_inputs(|| make_inputs())
            .bench_values(|inputs| {
                for (a, b) in inputs {
                    divan::black_box(a << b >> b == a);
                }
            });
    }

    #[divan::bench]
    fn is_multiple_of_via_tzc(bencher: divan::Bencher) {
        bencher
            .with_inputs(|| make_inputs())
            .bench_values(|inputs| {
                for (a, b) in inputs {
                    let tzc = a.trailing_zeros();
                    divan::black_box(b as u32 >= tzc);
                }
            });
    }
}
