# `public` bench with native target (AMD Ryzen 7 7800X3D 8-Core Processor)
```
public                                     fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ div                                                   │               │               │               │         │
│  ├─ i8                                                 │               │               │               │         │
│  │  ├─ baseline_identity                 11.22 ns      │ 49.5 ns       │ 13.56 ns      │ 14.5 ns       │ 1000    │ 1024000
│  │  ├─ pow2_div                          46.67 ns      │ 101.7 ns      │ 53.7 ns       │ 54.39 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_ceil                     137.3 ns      │ 507.6 ns      │ 140.4 ns      │ 142.4 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_ceil_const               27.14 ns      │ 61.51 ns      │ 29.87 ns      │ 30.03 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_ceil_reuse               80.26 ns      │ 159.1 ns      │ 83.39 ns      │ 83.28 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_const                    21.47 ns      │ 58 ns         │ 25.38 ns      │ 26.01 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_floor                    31.44 ns      │ 95.89 ns      │ 34.95 ns      │ 36.29 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_floor_const              19.33 ns      │ 69.91 ns      │ 21.87 ns      │ 22.42 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_floor_reuse              24.21 ns      │ 62.88 ns      │ 26.75 ns      │ 27.25 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_reuse                    29.48 ns      │ 73.82 ns      │ 33.78 ns      │ 34.44 ns      │ 1000    │ 512000
│  │  ├─ std_div                           856 ns        │ 1.137 µs      │ 862.3 ns      │ 869.8 ns      │ 1000    │ 16000
│  │  ├─ std_div_const                     22.06 ns      │ 81.24 ns      │ 24.4 ns       │ 24.94 ns      │ 1000    │ 512000
│  │  ├─ std_div_reuse                     824.8 ns      │ 1.568 µs      │ 831 ns        │ 832.8 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_div                      53.7 ns       │ 113 ns        │ 59.56 ns      │ 60.11 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_ceil                 170.1 ns      │ 381 ns        │ 173.2 ns      │ 173.7 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil_const           26.94 ns      │ 71.08 ns      │ 29.87 ns      │ 30.53 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_ceil_reuse           80.26 ns      │ 218.5 ns      │ 81.83 ns      │ 83.41 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_const                24.01 ns      │ 56.63 ns      │ 25.77 ns      │ 26.36 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_floor                41.59 ns      │ 110.3 ns      │ 45.5 ns       │ 46.35 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_floor_const          19.33 ns      │ 75.58 ns      │ 20.89 ns      │ 21.6 ns       │ 1000    │ 512000
│  │  ├─ unb_pow2_div_floor_reuse          24.01 ns      │ 56.24 ns      │ 26.55 ns      │ 26.9 ns       │ 1000    │ 512000
│  │  ╰─ unb_pow2_div_reuse                28.7 ns       │ 78.12 ns      │ 33.97 ns      │ 34.2 ns       │ 1000    │ 512000
│  ├─ i16                                                │               │               │               │         │
│  │  ├─ baseline_identity                 26.16 ns      │ 294.3 ns      │ 27.72 ns      │ 29.73 ns      │ 1000    │ 512000
│  │  ├─ pow2_div                          70.5 ns       │ 161.5 ns      │ 77.92 ns      │ 80.91 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_ceil                     140.4 ns      │ 1.211 µs      │ 144.3 ns      │ 147.7 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_ceil_const               56.83 ns      │ 123.2 ns      │ 63.86 ns      │ 64.45 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_ceil_reuse               91.2 ns       │ 274 ns        │ 95.11 ns      │ 96.85 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_const                    43.55 ns      │ 139.2 ns      │ 51.36 ns      │ 51.87 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_floor                    67.37 ns      │ 179 ns        │ 69.72 ns      │ 71.02 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_floor_const              36.51 ns      │ 120.1 ns      │ 39.64 ns      │ 40.06 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_floor_reuse              40.42 ns      │ 115.4 ns      │ 42.76 ns      │ 44.33 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_reuse                    51.36 ns      │ 167.7 ns      │ 61.51 ns      │ 62.6 ns       │ 1000    │ 256000
│  │  ├─ std_div                           856 ns        │ 4.562 µs      │ 912.3 ns      │ 959.1 ns      │ 1000    │ 16000
│  │  ├─ std_div_const                     40.03 ns      │ 108 ns        │ 50.19 ns      │ 49.95 ns      │ 1000    │ 256000
│  │  ├─ std_div_reuse                     2.474 µs      │ 6.699 µs      │ 2.499 µs      │ 2.527 µs      │ 1000    │ 4000
│  │  ├─ unb_pow2_div                      74.4 ns       │ 182.2 ns      │ 82.61 ns      │ 83.81 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_ceil                 174.8 ns      │ 1.927 µs      │ 177.9 ns      │ 184.2 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil_const           57.61 ns      │ 153.3 ns      │ 65.42 ns      │ 66.9 ns       │ 1000    │ 256000
│  │  ├─ unb_pow2_div_ceil_reuse           92.76 ns      │ 176.3 ns      │ 96.67 ns      │ 97.57 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_const                44.72 ns      │ 136.1 ns      │ 51.75 ns      │ 53.01 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_floor                50.97 ns      │ 185.3 ns      │ 55.65 ns      │ 56.99 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_floor_const          37.3 ns       │ 95.89 ns      │ 41.98 ns      │ 43.04 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_floor_reuse          40.81 ns      │ 111.1 ns      │ 45.5 ns       │ 46.59 ns      │ 1000    │ 256000
│  │  ╰─ unb_pow2_div_reuse                55.26 ns      │ 159.1 ns      │ 59.95 ns      │ 62.07 ns      │ 1000    │ 256000
│  ├─ i32                                                │               │               │               │         │
│  │  ├─ baseline_identity                 43.55 ns      │ 699 ns        │ 54.87 ns      │ 57.28 ns      │ 1000    │ 256000
│  │  ├─ pow2_div                          95.11 ns      │ 413.8 ns      │ 124.8 ns      │ 131.7 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_ceil                     157.6 ns      │ 379.4 ns      │ 168.5 ns      │ 177.9 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_ceil_const               114.6 ns      │ 342.7 ns      │ 128.7 ns      │ 130.6 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_ceil_reuse               125.5 ns      │ 335.7 ns      │ 134.1 ns      │ 137 ns        │ 1000    │ 128000
│  │  ├─ pow2_div_const                    90.42 ns      │ 238 ns        │ 101.3 ns      │ 104 ns        │ 1000    │ 128000
│  │  ├─ pow2_div_floor                    91.2 ns       │ 274 ns        │ 100.5 ns      │ 103.3 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_floor_const              75.19 ns      │ 187.6 ns      │ 81.05 ns      │ 84.22 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_floor_reuse              78.7 ns       │ 226.3 ns      │ 84.95 ns      │ 86.63 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_reuse                    96.67 ns      │ 214.6 ns      │ 108.3 ns      │ 109.4 ns      │ 1000    │ 128000
│  │  ├─ std_div                           1.299 µs      │ 1.849 µs      │ 1.312 µs      │ 1.321 µs      │ 1000    │ 8000
│  │  ├─ std_div_const                     87.3 ns       │ 237.3 ns      │ 101.3 ns      │ 104.1 ns      │ 1000    │ 128000
│  │  ├─ std_div_reuse                     1.312 µs      │ 20.71 µs      │ 1.324 µs      │ 1.373 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div                      122.4 ns      │ 420.8 ns      │ 136.5 ns      │ 139.2 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_ceil                 176.3 ns      │ 481 ns        │ 185.7 ns      │ 187.4 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil_const           113 ns        │ 274 ns        │ 121.6 ns      │ 124.5 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_ceil_reuse           126.3 ns      │ 350.5 ns      │ 139.6 ns      │ 142.7 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_const                88.86 ns      │ 950.5 ns      │ 99.01 ns      │ 101.8 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_floor                93.55 ns      │ 219.3 ns      │ 106 ns        │ 107.5 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_floor_const          75.19 ns      │ 248.6 ns      │ 81.44 ns      │ 84.74 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_floor_reuse          77.14 ns      │ 277.9 ns      │ 83.39 ns      │ 85.84 ns      │ 1000    │ 128000
│  │  ╰─ unb_pow2_div_reuse                99.8 ns       │ 1.174 µs      │ 108.3 ns      │ 112.1 ns      │ 1000    │ 128000
│  ├─ i64                                                │               │               │               │         │
│  │  ├─ baseline_identity                 81.83 ns      │ 306 ns        │ 94.33 ns      │ 101.5 ns      │ 1000    │ 128000
│  │  ├─ pow2_div                          224.8 ns      │ 665.4 ns      │ 246.6 ns      │ 251.8 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_ceil                     209.1 ns      │ 424.8 ns      │ 232.6 ns      │ 236.6 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_ceil_const               163.8 ns      │ 1.87 µs       │ 193.5 ns      │ 200.8 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_ceil_reuse               188.8 ns      │ 484.1 ns      │ 212.3 ns      │ 214.1 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_const                    181 ns        │ 481 ns        │ 202.9 ns      │ 206.3 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_floor                    174.8 ns      │ 1.935 µs      │ 193.5 ns      │ 199.1 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_floor_const              148.2 ns      │ 345.1 ns      │ 163.8 ns      │ 167.1 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_floor_reuse              170.1 ns      │ 607.6 ns      │ 182.6 ns      │ 186 ns        │ 1000    │ 64000
│  │  ├─ pow2_div_reuse                    195.1 ns      │ 432.6 ns      │ 213.8 ns      │ 219.7 ns      │ 1000    │ 64000
│  │  ├─ std_div                           1.624 µs      │ 2.749 µs      │ 1.662 µs      │ 1.666 µs      │ 1000    │ 8000
│  │  ├─ std_div_const                     174.8 ns      │ 457.6 ns      │ 199.8 ns      │ 202.3 ns      │ 1000    │ 64000
│  │  ├─ std_div_reuse                     1.549 µs      │ 20.21 µs      │ 1.562 µs      │ 1.645 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div                      241.9 ns      │ 468.5 ns      │ 263.8 ns      │ 265.9 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil                 216.9 ns      │ 1.295 µs      │ 241.9 ns      │ 248.9 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil_const           170.1 ns      │ 429.4 ns      │ 199.8 ns      │ 202.4 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil_reuse           202.9 ns      │ 541.9 ns      │ 220.1 ns      │ 224 ns        │ 1000    │ 64000
│  │  ├─ unb_pow2_div_const                181 ns        │ 418.5 ns      │ 206 ns        │ 208.7 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor                179.4 ns      │ 677.9 ns      │ 199.8 ns      │ 202.1 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor_const          149.8 ns      │ 393.5 ns      │ 166.9 ns      │ 170.2 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor_reuse          160.7 ns      │ 437.3 ns      │ 179.4 ns      │ 183.4 ns      │ 1000    │ 64000
│  │  ╰─ unb_pow2_div_reuse                193.5 ns      │ 410.7 ns      │ 215.4 ns      │ 217.6 ns      │ 1000    │ 64000
│  ├─ i128                                               │               │               │               │         │
│  │  ├─ baseline_identity                 220.1 ns      │ 463.8 ns      │ 232.6 ns      │ 237.3 ns      │ 1000    │ 64000
│  │  ├─ pow2_div                          1.299 µs      │ 20.04 µs      │ 1.337 µs      │ 1.439 µs      │ 1000    │ 8000
│  │  ├─ pow2_div_ceil                     2.174 µs      │ 4.087 µs      │ 2.199 µs      │ 2.209 µs      │ 1000    │ 8000
│  │  ├─ pow2_div_ceil_const               1.037 µs      │ 1.581 µs      │ 1.056 µs      │ 1.062 µs      │ 1000    │ 16000
│  │  ├─ pow2_div_ceil_reuse               2.112 µs      │ 3.262 µs      │ 2.149 µs      │ 2.16 µs       │ 1000    │ 8000
│  │  ├─ pow2_div_const                    1.712 µs      │ 2.749 µs      │ 1.737 µs      │ 1.745 µs      │ 1000    │ 8000
│  │  ├─ pow2_div_floor                    1.049 µs      │ 1.756 µs      │ 1.062 µs      │ 1.07 µs       │ 1000    │ 16000
│  │  ├─ pow2_div_floor_const              812.3 ns      │ 1.918 µs      │ 843.5 ns      │ 1.085 µs      │ 1000    │ 16000
│  │  ├─ pow2_div_floor_reuse              1.081 µs      │ 1.737 µs      │ 1.093 µs      │ 1.101 µs      │ 1000    │ 16000
│  │  ├─ pow2_div_reuse                    2.037 µs      │ 5.062 µs      │ 2.074 µs      │ 2.465 µs      │ 1000    │ 8000
│  │  ├─ std_div                           9.049 µs      │ 11.79 µs      │ 9.249 µs      │ 9.285 µs      │ 1000    │ 2000
│  │  ├─ std_div_const                     1.024 µs      │ 1.837 µs      │ 1.037 µs      │ 1.045 µs      │ 1000    │ 16000
│  │  ├─ std_div_reuse                     7.999 µs      │ 14.29 µs      │ 8.099 µs      │ 8.139 µs      │ 1000    │ 2000
│  │  ├─ unb_pow2_div                      1.287 µs      │ 2.499 µs      │ 1.312 µs      │ 1.321 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div_ceil                 2.099 µs      │ 3.412 µs      │ 2.124 µs      │ 2.129 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div_ceil_const           1.037 µs      │ 1.449 µs      │ 1.062 µs      │ 1.067 µs      │ 1000    │ 16000
│  │  ├─ unb_pow2_div_ceil_reuse           2.137 µs      │ 2.674 µs      │ 2.149 µs      │ 2.16 µs       │ 1000    │ 8000
│  │  ├─ unb_pow2_div_const                1.724 µs      │ 4.037 µs      │ 1.749 µs      │ 1.791 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div_floor                1.162 µs      │ 2.106 µs      │ 1.193 µs      │ 1.208 µs      │ 1000    │ 16000
│  │  ├─ unb_pow2_div_floor_const          818.5 ns      │ 1.337 µs      │ 837.3 ns      │ 843.7 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_div_floor_reuse          1.081 µs      │ 1.887 µs      │ 1.093 µs      │ 1.103 µs      │ 1000    │ 16000
│  │  ╰─ unb_pow2_div_reuse                2.024 µs      │ 5.124 µs      │ 2.074 µs      │ 2.106 µs      │ 1000    │ 8000
│  ├─ u8                                                 │               │               │               │         │
│  │  ├─ baseline_identity                 11.22 ns      │ 51.65 ns      │ 12.3 ns       │ 13.63 ns      │ 1000    │ 1024000
│  │  ├─ pow2_div                          35.73 ns      │ 133.7 ns      │ 40.03 ns      │ 40.62 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_ceil                     135.7 ns      │ 341.9 ns      │ 138 ns        │ 139.3 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_ceil_const               26.16 ns      │ 59.95 ns      │ 29.68 ns      │ 30.1 ns       │ 1000    │ 512000
│  │  ├─ pow2_div_ceil_reuse               80.26 ns      │ 122.4 ns      │ 83.39 ns      │ 83.5 ns       │ 1000    │ 128000
│  │  ├─ pow2_div_const                    19.33 ns      │ 52.14 ns      │ 20.89 ns      │ 21.47 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_floor                    30.26 ns      │ 86.12 ns      │ 35.73 ns      │ 36.17 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_floor_const              18.94 ns      │ 50.38 ns      │ 20.69 ns      │ 21.62 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_floor_reuse              21.08 ns      │ 77.92 ns      │ 23.43 ns      │ 23.91 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_reuse                    20.69 ns      │ 95.69 ns      │ 23.62 ns      │ 24.21 ns      │ 1000    │ 512000
│  │  ├─ std_div                           849.8 ns      │ 1.112 µs      │ 856 ns        │ 859.4 ns      │ 1000    │ 16000
│  │  ├─ std_div_const                     19.13 ns      │ 300.5 ns      │ 20.5 ns       │ 21.12 ns      │ 1000    │ 512000
│  │  ├─ std_div_reuse                     824.8 ns      │ 1.562 µs      │ 837.3 ns      │ 845.2 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_div                      34.95 ns      │ 85.73 ns      │ 40.42 ns      │ 41.06 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_ceil                 170.1 ns      │ 290.4 ns      │ 206 ns        │ 201.9 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil_const           24.01 ns      │ 57.61 ns      │ 26.36 ns      │ 29.6 ns       │ 1000    │ 256000
│  │  ├─ unb_pow2_div_ceil_reuse           81.05 ns      │ 120.8 ns      │ 81.83 ns      │ 82.04 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_const                18.94 ns      │ 76.55 ns      │ 21.08 ns      │ 22.2 ns       │ 1000    │ 512000
│  │  ├─ unb_pow2_div_floor                35.34 ns      │ 92.76 ns      │ 40.42 ns      │ 40.78 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_floor_const          19.91 ns      │ 66.4 ns       │ 22.45 ns      │ 23.05 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_floor_reuse          22.06 ns      │ 58.97 ns      │ 23.23 ns      │ 23.86 ns      │ 1000    │ 512000
│  │  ╰─ unb_pow2_div_reuse                21.08 ns      │ 72.65 ns      │ 23.04 ns      │ 23.69 ns      │ 1000    │ 512000
│  ├─ u16                                                │               │               │               │         │
│  │  ├─ baseline_identity                 26.36 ns      │ 73.23 ns      │ 27.33 ns      │ 28.96 ns      │ 1000    │ 512000
│  │  ├─ pow2_div                          59.95 ns      │ 175.5 ns      │ 65.03 ns      │ 67.9 ns       │ 1000    │ 256000
│  │  ├─ pow2_div_ceil                     138.8 ns      │ 903.7 ns      │ 141.9 ns      │ 146.5 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_ceil_const               55.26 ns      │ 130.2 ns      │ 63.47 ns      │ 63.67 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_ceil_reuse               91.2 ns       │ 170.8 ns      │ 95.89 ns      │ 96.36 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_const                    37.69 ns      │ 125.9 ns      │ 41.98 ns      │ 43.81 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_floor                    66.59 ns      │ 665.8 ns      │ 69.72 ns      │ 71.97 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_floor_const              35.73 ns      │ 418.5 ns      │ 39.25 ns      │ 41.61 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_floor_reuse              40.42 ns      │ 137.3 ns      │ 43.15 ns      │ 44.2 ns       │ 1000    │ 256000
│  │  ├─ pow2_div_reuse                    41.2 ns       │ 116.5 ns      │ 44.33 ns      │ 45.16 ns      │ 1000    │ 256000
│  │  ├─ std_div                           856 ns        │ 9.243 µs      │ 868.5 ns      │ 900.8 ns      │ 1000    │ 16000
│  │  ├─ std_div_const                     36.51 ns      │ 122.8 ns      │ 39.64 ns      │ 40.6 ns       │ 1000    │ 256000
│  │  ├─ std_div_reuse                     843.5 ns      │ 1.106 µs      │ 849.8 ns      │ 852.8 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_div                      53.7 ns       │ 489.6 ns      │ 58.39 ns      │ 60.69 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_ceil                 174.8 ns      │ 2.534 µs      │ 177.9 ns      │ 187.4 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil_const           52.92 ns      │ 469.7 ns      │ 61.12 ns      │ 62.75 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_ceil_reuse           91.98 ns      │ 1.279 µs      │ 96.67 ns      │ 101.3 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_const                37.3 ns       │ 459.1 ns      │ 40.03 ns      │ 41.92 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_floor                54.87 ns      │ 216.2 ns      │ 60.73 ns      │ 62.89 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_floor_const          37.3 ns       │ 110.3 ns      │ 40.42 ns      │ 41.89 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_floor_reuse          41.59 ns      │ 113.4 ns      │ 46.67 ns      │ 47.96 ns      │ 1000    │ 256000
│  │  ╰─ unb_pow2_div_reuse                40.42 ns      │ 187.3 ns      │ 47.84 ns      │ 49.51 ns      │ 1000    │ 256000
│  ├─ u32                                                │               │               │               │         │
│  │  ├─ baseline_identity                 43.55 ns      │ 720.1 ns      │ 49.01 ns      │ 52.07 ns      │ 1000    │ 256000
│  │  ├─ pow2_div                          102.1 ns      │ 225.5 ns      │ 115.4 ns      │ 117.8 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_ceil                     160.7 ns      │ 290.4 ns      │ 169.3 ns      │ 172.2 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_ceil_const               115.4 ns      │ 277.1 ns      │ 127.9 ns      │ 128.8 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_ceil_reuse               118.5 ns      │ 306.8 ns      │ 134.9 ns      │ 138.3 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_const                    69.33 ns      │ 240.4 ns      │ 77.14 ns      │ 81.71 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_floor                    92.76 ns      │ 245.8 ns      │ 101.3 ns      │ 103.7 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_floor_const              75.19 ns      │ 288 ns        │ 81.44 ns      │ 86.15 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_floor_reuse              76.36 ns      │ 352.1 ns      │ 84.95 ns      │ 88.85 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_reuse                    77.92 ns      │ 376.3 ns      │ 87.3 ns       │ 89.47 ns      │ 1000    │ 128000
│  │  ├─ std_div                           1.287 µs      │ 1.974 µs      │ 1.299 µs      │ 1.306 µs      │ 1000    │ 8000
│  │  ├─ std_div_const                     68.55 ns      │ 218.5 ns      │ 76.36 ns      │ 77.98 ns      │ 1000    │ 128000
│  │  ├─ std_div_reuse                     1.262 µs      │ 1.974 µs      │ 1.287 µs      │ 1.293 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div                      96.67 ns      │ 319.3 ns      │ 106.8 ns      │ 109 ns        │ 1000    │ 128000
│  │  ├─ unb_pow2_div_ceil                 177.9 ns      │ 274.8 ns      │ 182.6 ns      │ 183.9 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil_const           115.4 ns      │ 269.3 ns      │ 127.9 ns      │ 127.4 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_ceil_reuse           125.5 ns      │ 237.3 ns      │ 134.1 ns      │ 136 ns        │ 1000    │ 128000
│  │  ├─ unb_pow2_div_const                76.36 ns      │ 186.9 ns      │ 84.17 ns      │ 87.64 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_floor                93.55 ns      │ 181.8 ns      │ 104.4 ns      │ 106.1 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_floor_const          67.76 ns      │ 187.3 ns      │ 77.14 ns      │ 79.28 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_floor_reuse          74.01 ns      │ 202.1 ns      │ 80.26 ns      │ 83.84 ns      │ 1000    │ 128000
│  │  ╰─ unb_pow2_div_reuse                74.8 ns       │ 193.5 ns      │ 81.05 ns      │ 83.9 ns       │ 1000    │ 128000
│  ├─ u64                                                │               │               │               │         │
│  │  ├─ baseline_identity                 81.83 ns      │ 263.8 ns      │ 96.67 ns      │ 99.91 ns      │ 1000    │ 128000
│  │  ├─ pow2_div                          202.9 ns      │ 452.9 ns      │ 235.7 ns      │ 242.3 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_ceil                     218.5 ns      │ 538.8 ns      │ 243.5 ns      │ 245.6 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_ceil_const               185.7 ns      │ 427.9 ns      │ 201.3 ns      │ 205.1 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_ceil_reuse               196.6 ns      │ 418.5 ns      │ 223.2 ns      │ 225 ns        │ 1000    │ 64000
│  │  ├─ pow2_div_const                    149.8 ns      │ 527.9 ns      │ 165.4 ns      │ 169.2 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_floor                    168.5 ns      │ 481 ns        │ 182.6 ns      │ 186.5 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_floor_const              148.2 ns      │ 391.9 ns      │ 166.9 ns      │ 169.9 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_floor_reuse              159.1 ns      │ 2.359 µs      │ 181 ns        │ 194.7 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_reuse                    160.7 ns      │ 602.9 ns      │ 184.1 ns      │ 188 ns        │ 1000    │ 64000
│  │  ├─ std_div                           1.637 µs      │ 17.48 µs      │ 1.649 µs      │ 1.705 µs      │ 1000    │ 8000
│  │  ├─ std_div_const                     149.8 ns      │ 445.1 ns      │ 165.4 ns      │ 171.1 ns      │ 1000    │ 64000
│  │  ├─ std_div_reuse                     1.499 µs      │ 18.59 µs      │ 1.524 µs      │ 1.573 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div                      184.1 ns      │ 556 ns        │ 201.3 ns      │ 205.7 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil                 224.8 ns      │ 540.4 ns      │ 245.1 ns      │ 245.7 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil_const           182.6 ns      │ 395.1 ns      │ 201.3 ns      │ 204.4 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil_reuse           202.9 ns      │ 506 ns        │ 216.9 ns      │ 224.5 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_const                148.2 ns      │ 777.9 ns      │ 168.5 ns      │ 175.4 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor                177.9 ns      │ 2.452 µs      │ 199.8 ns      │ 205.9 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor_const          152.9 ns      │ 473.2 ns      │ 176.3 ns      │ 186.8 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor_reuse          156 ns        │ 446.6 ns      │ 173.2 ns      │ 177.1 ns      │ 1000    │ 64000
│  │  ╰─ unb_pow2_div_reuse                156 ns        │ 387.3 ns      │ 170.1 ns      │ 173.1 ns      │ 1000    │ 64000
│  ╰─ u128                                               │               │               │               │         │
│     ├─ baseline_identity                 195.1 ns      │ 534.1 ns      │ 207.6 ns      │ 215.4 ns      │ 1000    │ 64000
│     ├─ pow2_div                          1.156 µs      │ 2.906 µs      │ 1.174 µs      │ 1.209 µs      │ 1000    │ 16000
│     ├─ pow2_div_ceil                     2.087 µs      │ 4.112 µs      │ 2.124 µs      │ 2.178 µs      │ 1000    │ 8000
│     ├─ pow2_div_ceil_const               1.043 µs      │ 2.762 µs      │ 1.062 µs      │ 1.085 µs      │ 1000    │ 16000
│     ├─ pow2_div_ceil_reuse               2.099 µs      │ 4.912 µs      │ 2.137 µs      │ 2.187 µs      │ 1000    │ 8000
│     ├─ pow2_div_const                    812.3 ns      │ 2.199 µs      │ 824.8 ns      │ 849.4 ns      │ 1000    │ 16000
│     ├─ pow2_div_floor                    1.074 µs      │ 1.999 µs      │ 1.087 µs      │ 1.115 µs      │ 1000    │ 16000
│     ├─ pow2_div_floor_const              812.3 ns      │ 1.656 µs      │ 824.8 ns      │ 844.6 ns      │ 1000    │ 16000
│     ├─ pow2_div_floor_reuse              924.8 ns      │ 1.993 µs      │ 937.3 ns      │ 966.7 ns      │ 1000    │ 16000
│     ├─ pow2_div_reuse                    931 ns        │ 2.237 µs      │ 943.5 ns      │ 978.5 ns      │ 1000    │ 16000
│     ├─ std_div                           5.799 µs      │ 14.29 µs      │ 5.849 µs      │ 5.902 µs      │ 1000    │ 2000
│     ├─ std_div_const                     818.5 ns      │ 1.637 µs      │ 837.3 ns      │ 883.9 ns      │ 1000    │ 16000
│     ├─ std_div_reuse                     4.324 µs      │ 7.374 µs      │ 4.349 µs      │ 4.372 µs      │ 1000    │ 4000
│     ├─ unb_pow2_div                      1.168 µs      │ 1.793 µs      │ 1.199 µs      │ 1.209 µs      │ 1000    │ 16000
│     ├─ unb_pow2_div_ceil                 2.099 µs      │ 3.649 µs      │ 2.174 µs      │ 2.2 µs        │ 1000    │ 8000
│     ├─ unb_pow2_div_ceil_const           1.043 µs      │ 1.756 µs      │ 1.062 µs      │ 1.066 µs      │ 1000    │ 16000
│     ├─ unb_pow2_div_ceil_reuse           2.112 µs      │ 7.762 µs      │ 2.137 µs      │ 2.19 µs       │ 1000    │ 8000
│     ├─ unb_pow2_div_const                818.5 ns      │ 1.843 µs      │ 831 ns        │ 837.4 ns      │ 1000    │ 16000
│     ├─ unb_pow2_div_floor                1.162 µs      │ 1.956 µs      │ 1.187 µs      │ 1.19 µs       │ 1000    │ 16000
│     ├─ unb_pow2_div_floor_const          799.8 ns      │ 1.256 µs      │ 818.5 ns      │ 820.9 ns      │ 1000    │ 16000
│     ├─ unb_pow2_div_floor_reuse          906 ns        │ 1.649 µs      │ 924.8 ns      │ 929.6 ns      │ 1000    │ 16000
│     ╰─ unb_pow2_div_reuse                918.5 ns      │ 1.256 µs      │ 931 ns        │ 939.4 ns      │ 1000    │ 16000
├─ modulo                                                │               │               │               │         │
│  ├─ i8                                                 │               │               │               │         │
│  │  ├─ baseline_identity                 11.12 ns      │ 52.24 ns      │ 12.39 ns      │ 13.49 ns      │ 1000    │ 1024000
│  │  ├─ pow2_is_multiple_of               38.86 ns      │ 135.9 ns      │ 45.69 ns      │ 46.44 ns      │ 1000    │ 512000
│  │  ├─ pow2_is_multiple_of_const         20.11 ns      │ 77.92 ns      │ 22.65 ns      │ 24.57 ns      │ 1000    │ 512000
│  │  ├─ pow2_is_multiple_of_reuse         26.16 ns      │ 106.4 ns      │ 30.46 ns      │ 33.14 ns      │ 1000    │ 512000
│  │  ├─ pow2_mod_floor                    33.39 ns      │ 121 ns        │ 40.22 ns      │ 41.72 ns      │ 1000    │ 512000
│  │  ├─ pow2_mod_floor_const              19.52 ns      │ 74.8 ns       │ 21.28 ns      │ 21.93 ns      │ 1000    │ 512000
│  │  ├─ pow2_mod_floor_reuse              20.3 ns       │ 65.62 ns      │ 22.06 ns      │ 22.64 ns      │ 1000    │ 512000
│  │  ├─ std_mod                           856 ns        │ 1.874 µs      │ 874.8 ns      │ 888.8 ns      │ 1000    │ 16000
│  │  ├─ std_mod_const                     22.84 ns      │ 55.85 ns      │ 24.99 ns      │ 25.52 ns      │ 1000    │ 512000
│  │  ├─ std_mod_reuse                     824.8 ns      │ 1.449 µs      │ 837.3 ns      │ 838.2 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_is_multiple_of           36.9 ns       │ 91.98 ns      │ 39.83 ns      │ 41.96 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_is_multiple_of_const     19.52 ns      │ 279.8 ns      │ 21.67 ns      │ 23.76 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_is_multiple_of_reuse     27.33 ns      │ 77.14 ns      │ 30.07 ns      │ 31.51 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_mod_floor                33.97 ns      │ 124.4 ns      │ 39.44 ns      │ 40.71 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_mod_floor_const          18.74 ns      │ 94.72 ns      │ 20.3 ns       │ 22.02 ns      │ 1000    │ 512000
│  │  ╰─ unb_pow2_mod_floor_reuse          20.11 ns      │ 59.76 ns      │ 21.28 ns      │ 22.11 ns      │ 1000    │ 512000
│  ├─ i16                                                │               │               │               │         │
│  │  ├─ baseline_identity                 26.36 ns      │ 69.13 ns      │ 27.72 ns      │ 29.13 ns      │ 1000    │ 512000
│  │  ├─ pow2_is_multiple_of               54.48 ns      │ 539.2 ns      │ 61.9 ns       │ 64.47 ns      │ 1000    │ 256000
│  │  ├─ pow2_is_multiple_of_const         30.65 ns      │ 82.61 ns      │ 37.49 ns      │ 37.68 ns      │ 1000    │ 512000
│  │  ├─ pow2_is_multiple_of_reuse         30.65 ns      │ 89.64 ns      │ 34.95 ns      │ 35.36 ns      │ 1000    │ 256000
│  │  ├─ pow2_mod_floor                    67.37 ns      │ 164.2 ns      │ 75.97 ns      │ 77.46 ns      │ 1000    │ 256000
│  │  ├─ pow2_mod_floor_const              37.3 ns       │ 111.9 ns      │ 42.37 ns      │ 43.45 ns      │ 1000    │ 256000
│  │  ├─ pow2_mod_floor_reuse              41.98 ns      │ 110.3 ns      │ 44.72 ns      │ 45.72 ns      │ 1000    │ 256000
│  │  ├─ std_mod                           849.8 ns      │ 1.237 µs      │ 893.5 ns      │ 901 ns        │ 1000    │ 16000
│  │  ├─ std_mod_const                     47.45 ns      │ 140.8 ns      │ 54.48 ns      │ 55.58 ns      │ 1000    │ 256000
│  │  ├─ std_mod_reuse                     2.499 µs      │ 6.149 µs      │ 2.524 µs      │ 2.541 µs      │ 1000    │ 4000
│  │  ├─ unb_pow2_is_multiple_of           50.97 ns      │ 133 ns        │ 55.26 ns      │ 57.35 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_is_multiple_of_const     30.65 ns      │ 177.9 ns      │ 36.51 ns      │ 37.02 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_is_multiple_of_reuse     37.69 ns      │ 132.8 ns      │ 43.35 ns      │ 45.44 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_mod_floor                53.7 ns       │ 126.7 ns      │ 60.73 ns      │ 61.49 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_mod_floor_const          36.9 ns       │ 420.8 ns      │ 40.81 ns      │ 41.74 ns      │ 1000    │ 256000
│  │  ╰─ unb_pow2_mod_floor_reuse          41.59 ns      │ 584.9 ns      │ 48.62 ns      │ 50.66 ns      │ 1000    │ 256000
│  ├─ i32                                                │               │               │               │         │
│  │  ├─ baseline_identity                 43.94 ns      │ 163.4 ns      │ 49.6 ns       │ 52.33 ns      │ 1000    │ 256000
│  │  ├─ pow2_is_multiple_of               63.08 ns      │ 141.2 ns      │ 71.67 ns      │ 72.86 ns      │ 1000    │ 128000
│  │  ├─ pow2_is_multiple_of_const         48.23 ns      │ 224.4 ns      │ 55.65 ns      │ 58.73 ns      │ 1000    │ 256000
│  │  ├─ pow2_is_multiple_of_reuse         63.86 ns      │ 157.6 ns      │ 70.89 ns      │ 72.57 ns      │ 1000    │ 256000
│  │  ├─ pow2_mod_floor                    95.11 ns      │ 358.3 ns      │ 108.3 ns      │ 110.1 ns      │ 1000    │ 128000
│  │  ├─ pow2_mod_floor_const              74.01 ns      │ 1.134 µs      │ 80.26 ns      │ 83.78 ns      │ 1000    │ 128000
│  │  ├─ pow2_mod_floor_reuse              77.14 ns      │ 256 ns        │ 84.17 ns      │ 87.31 ns      │ 1000    │ 128000
│  │  ├─ std_mod                           1.287 µs      │ 1.849 µs      │ 1.312 µs      │ 1.316 µs      │ 1000    │ 8000
│  │  ├─ std_mod_const                     97.45 ns      │ 360.7 ns      │ 109.9 ns      │ 112.6 ns      │ 1000    │ 128000
│  │  ├─ std_mod_reuse                     1.274 µs      │ 1.874 µs      │ 1.287 µs      │ 1.296 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_is_multiple_of           65.42 ns      │ 177.1 ns      │ 75.58 ns      │ 75.81 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_is_multiple_of_const     49.4 ns       │ 161.5 ns      │ 56.24 ns      │ 57.41 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_is_multiple_of_reuse     70.5 ns       │ 145.8 ns      │ 77.53 ns      │ 78.51 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_mod_floor                104.4 ns      │ 209.1 ns      │ 110.7 ns      │ 112.6 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_mod_floor_const          76.36 ns      │ 377.1 ns      │ 85.34 ns      │ 89.77 ns      │ 1000    │ 256000
│  │  ╰─ unb_pow2_mod_floor_reuse          74.01 ns      │ 397.4 ns      │ 82.61 ns      │ 84.27 ns      │ 1000    │ 128000
│  ├─ i64                                                │               │               │               │         │
│  │  ├─ baseline_identity                 82.61 ns      │ 238.8 ns      │ 92.76 ns      │ 97.42 ns      │ 1000    │ 128000
│  │  ├─ pow2_is_multiple_of               143.5 ns      │ 327.1 ns      │ 162.3 ns      │ 164.5 ns      │ 1000    │ 128000
│  │  ├─ pow2_is_multiple_of_const         81.83 ns      │ 265.4 ns      │ 85.73 ns      │ 88.57 ns      │ 1000    │ 128000
│  │  ├─ pow2_is_multiple_of_reuse         116.9 ns      │ 358.3 ns      │ 124.8 ns      │ 128.5 ns      │ 1000    │ 128000
│  │  ├─ pow2_mod_floor                    174.8 ns      │ 896.6 ns      │ 195.1 ns      │ 200.8 ns      │ 1000    │ 64000
│  │  ├─ pow2_mod_floor_const              151.3 ns      │ 371.6 ns      │ 168.5 ns      │ 172.2 ns      │ 1000    │ 64000
│  │  ├─ pow2_mod_floor_reuse              159.1 ns      │ 471.6 ns      │ 181 ns        │ 184.8 ns      │ 1000    │ 64000
│  │  ├─ std_mod                           1.624 µs      │ 2.199 µs      │ 1.649 µs      │ 1.664 µs      │ 1000    │ 8000
│  │  ├─ std_mod_const                     185.7 ns      │ 507.6 ns      │ 212.3 ns      │ 217.4 ns      │ 1000    │ 64000
│  │  ├─ std_mod_reuse                     1.524 µs      │ 20.84 µs      │ 1.537 µs      │ 1.6 µs        │ 1000    │ 8000
│  │  ├─ unb_pow2_is_multiple_of           133.3 ns      │ 356.8 ns      │ 148.2 ns      │ 150.8 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_is_multiple_of_const     79.48 ns      │ 229.4 ns      │ 83.39 ns      │ 85.71 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_is_multiple_of_reuse     115.4 ns      │ 227.9 ns      │ 121.6 ns      │ 123.8 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_mod_floor                188.8 ns      │ 498.2 ns      │ 207.6 ns      │ 212.1 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_mod_floor_const          151.3 ns      │ 457.6 ns      │ 171.6 ns      │ 176.7 ns      │ 1000    │ 64000
│  │  ╰─ unb_pow2_mod_floor_reuse          157.6 ns      │ 427.9 ns      │ 171.6 ns      │ 175.1 ns      │ 1000    │ 64000
│  ├─ i128                                               │               │               │               │         │
│  │  ├─ baseline_identity                 218.5 ns      │ 487.3 ns      │ 243.5 ns      │ 251 ns        │ 1000    │ 64000
│  │  ├─ pow2_is_multiple_of               387.3 ns      │ 1.177 µs      │ 393.5 ns      │ 400.9 ns      │ 1000    │ 32000
│  │  ├─ pow2_is_multiple_of_const         321.6 ns      │ 590.4 ns      │ 331 ns        │ 332.5 ns      │ 1000    │ 32000
│  │  ├─ pow2_is_multiple_of_reuse         371.6 ns      │ 877.9 ns      │ 377.9 ns      │ 384.9 ns      │ 1000    │ 32000
│  │  ├─ pow2_mod_floor                    818.5 ns      │ 1.599 µs      │ 831 ns        │ 836.7 ns      │ 1000    │ 16000
│  │  ├─ pow2_mod_floor_const              440.4 ns      │ 1.327 µs      │ 474.8 ns      │ 487.9 ns      │ 1000    │ 32000
│  │  ├─ pow2_mod_floor_reuse              568.5 ns      │ 5.459 µs      │ 606 ns        │ 631 ns        │ 1000    │ 32000
│  │  ├─ std_mod                           9.099 µs      │ 15.39 µs      │ 9.099 µs      │ 9.186 µs      │ 1000    │ 1000
│  │  ├─ std_mod_const                     762.3 ns      │ 1.381 µs      │ 781 ns        │ 786.8 ns      │ 1000    │ 16000
│  │  ├─ std_mod_reuse                     8.149 µs      │ 12.49 µs      │ 8.249 µs      │ 8.296 µs      │ 1000    │ 2000
│  │  ├─ unb_pow2_is_multiple_of           393.5 ns      │ 4.627 µs      │ 409.1 ns      │ 425.5 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_is_multiple_of_const     327.9 ns      │ 731 ns        │ 334.1 ns      │ 337.6 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_is_multiple_of_reuse     377.9 ns      │ 593.5 ns      │ 381 ns        │ 383.6 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_mod_floor                818.5 ns      │ 2.343 µs      │ 837.3 ns      │ 850 ns        │ 1000    │ 16000
│  │  ├─ unb_pow2_mod_floor_const          427.9 ns      │ 3.256 µs      │ 456 ns        │ 468.4 ns      │ 1000    │ 32000
│  │  ╰─ unb_pow2_mod_floor_reuse          562.3 ns      │ 1.356 µs      │ 596.6 ns      │ 612.3 ns      │ 1000    │ 32000
│  ├─ u8                                                 │               │               │               │         │
│  │  ├─ baseline_identity                 11.03 ns      │ 36.32 ns      │ 14.25 ns      │ 14.4 ns       │ 1000    │ 1024000
│  │  ├─ pow2_is_multiple_of               29.87 ns      │ 72.84 ns      │ 33.39 ns      │ 34.17 ns      │ 1000    │ 256000
│  │  ├─ pow2_is_multiple_of_const         19.52 ns      │ 75.97 ns      │ 21.67 ns      │ 22.5 ns       │ 1000    │ 512000
│  │  ├─ pow2_is_multiple_of_reuse         27.14 ns      │ 87.88 ns      │ 30.26 ns      │ 31.05 ns      │ 1000    │ 512000
│  │  ├─ pow2_mod_floor                    32.41 ns      │ 80.85 ns      │ 40.42 ns      │ 40.62 ns      │ 1000    │ 512000
│  │  ├─ pow2_mod_floor_const              19.13 ns      │ 61.71 ns      │ 21.28 ns      │ 21.85 ns      │ 1000    │ 512000
│  │  ├─ pow2_mod_floor_reuse              20.5 ns       │ 78.7 ns       │ 22.26 ns      │ 22.88 ns      │ 1000    │ 512000
│  │  ├─ std_mod                           849.8 ns      │ 2.287 µs      │ 862.3 ns      │ 869.7 ns      │ 1000    │ 16000
│  │  ├─ std_mod_const                     18.55 ns      │ 91.59 ns      │ 20.5 ns       │ 21.28 ns      │ 1000    │ 512000
│  │  ├─ std_mod_reuse                     843.5 ns      │ 8.124 µs      │ 849.8 ns      │ 855.3 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_is_multiple_of           33.78 ns      │ 230 ns        │ 40.03 ns      │ 41.21 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_is_multiple_of_const     19.91 ns      │ 64.25 ns      │ 22.84 ns      │ 23.24 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_is_multiple_of_reuse     27.92 ns      │ 75.58 ns      │ 30.07 ns      │ 30.72 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_mod_floor                36.12 ns      │ 179 ns        │ 44.52 ns      │ 47.48 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_mod_floor_const          18.74 ns      │ 304.2 ns      │ 20.11 ns      │ 21.24 ns      │ 1000    │ 512000
│  │  ╰─ unb_pow2_mod_floor_reuse          20.89 ns      │ 80.85 ns      │ 22.06 ns      │ 22.59 ns      │ 1000    │ 512000
│  ├─ u16                                                │               │               │               │         │
│  │  ├─ baseline_identity                 26.75 ns      │ 78.9 ns       │ 27.53 ns      │ 28.64 ns      │ 1000    │ 512000
│  │  ├─ pow2_is_multiple_of               55.26 ns      │ 134.1 ns      │ 61.12 ns      │ 62.43 ns      │ 1000    │ 256000
│  │  ├─ pow2_is_multiple_of_const         30.07 ns      │ 76.36 ns      │ 33.58 ns      │ 35.14 ns      │ 1000    │ 512000
│  │  ├─ pow2_is_multiple_of_reuse         35.54 ns      │ 108 ns        │ 46.67 ns      │ 47.16 ns      │ 1000    │ 512000
│  │  ├─ pow2_mod_floor                    68.55 ns      │ 194.7 ns      │ 75.97 ns      │ 77.42 ns      │ 1000    │ 256000
│  │  ├─ pow2_mod_floor_const              38.08 ns      │ 126.3 ns      │ 42.76 ns      │ 46.16 ns      │ 1000    │ 256000
│  │  ├─ pow2_mod_floor_reuse              41.2 ns       │ 130.6 ns      │ 47.84 ns      │ 49.73 ns      │ 1000    │ 256000
│  │  ├─ std_mod                           868.5 ns      │ 9.799 µs      │ 874.8 ns      │ 905.7 ns      │ 1000    │ 16000
│  │  ├─ std_mod_const                     36.51 ns      │ 170.8 ns      │ 40.03 ns      │ 40.94 ns      │ 1000    │ 256000
│  │  ├─ std_mod_reuse                     831 ns        │ 1.462 µs      │ 837.3 ns      │ 842.5 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_is_multiple_of           50.58 ns      │ 126.7 ns      │ 56.05 ns      │ 57.02 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_is_multiple_of_const     30.07 ns      │ 89.64 ns      │ 34.56 ns      │ 35.71 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_is_multiple_of_reuse     28.7 ns       │ 102.9 ns      │ 35.34 ns      │ 35.6 ns       │ 1000    │ 256000
│  │  ├─ unb_pow2_mod_floor                54.48 ns      │ 194.7 ns      │ 60.73 ns      │ 62.03 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_mod_floor_const          36.51 ns      │ 520.8 ns      │ 40.03 ns      │ 42.61 ns      │ 1000    │ 256000
│  │  ╰─ unb_pow2_mod_floor_reuse          40.42 ns      │ 192.3 ns      │ 46.67 ns      │ 47.96 ns      │ 1000    │ 256000
│  ├─ u32                                                │               │               │               │         │
│  │  ├─ baseline_identity                 43.94 ns      │ 254.8 ns      │ 48.62 ns      │ 51.47 ns      │ 1000    │ 256000
│  │  ├─ pow2_is_multiple_of               60.73 ns      │ 249.8 ns      │ 67.76 ns      │ 69.68 ns      │ 1000    │ 128000
│  │  ├─ pow2_is_multiple_of_const         46.28 ns      │ 133 ns        │ 50.97 ns      │ 53.75 ns      │ 1000    │ 256000
│  │  ├─ pow2_is_multiple_of_reuse         66.2 ns       │ 150.9 ns      │ 74.01 ns      │ 75.35 ns      │ 1000    │ 256000
│  │  ├─ pow2_mod_floor                    99.01 ns      │ 246.6 ns      │ 112.3 ns      │ 114 ns        │ 1000    │ 128000
│  │  ├─ pow2_mod_floor_const              70.11 ns      │ 276.3 ns      │ 81.83 ns      │ 83.22 ns      │ 1000    │ 128000
│  │  ├─ pow2_mod_floor_reuse              81.44 ns      │ 195.5 ns      │ 92.57 ns      │ 96.62 ns      │ 1000    │ 256000
│  │  ├─ std_mod                           1.324 µs      │ 19.03 µs      │ 1.337 µs      │ 1.369 µs      │ 1000    │ 8000
│  │  ├─ std_mod_const                     75.97 ns      │ 550.1 ns      │ 81.63 ns      │ 83.72 ns      │ 1000    │ 256000
│  │  ├─ std_mod_reuse                     1.249 µs      │ 2.737 µs      │ 1.262 µs      │ 1.267 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_is_multiple_of           59.17 ns      │ 156 ns        │ 72.45 ns      │ 72.4 ns       │ 1000    │ 128000
│  │  ├─ unb_pow2_is_multiple_of_const     48.23 ns      │ 129.4 ns      │ 56.44 ns      │ 57.26 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_is_multiple_of_reuse     64.64 ns      │ 166.9 ns      │ 73.62 ns      │ 74.12 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_mod_floor                96.67 ns      │ 877.9 ns      │ 109.1 ns      │ 113.9 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_mod_floor_const          69.33 ns      │ 216.2 ns      │ 79.48 ns      │ 84.04 ns      │ 1000    │ 128000
│  │  ╰─ unb_pow2_mod_floor_reuse          77.14 ns      │ 247.4 ns      │ 88.86 ns      │ 90.81 ns      │ 1000    │ 128000
│  ├─ u64                                                │               │               │               │         │
│  │  ├─ baseline_identity                 74.8 ns       │ 237.3 ns      │ 87.3 ns       │ 88.21 ns      │ 1000    │ 64000
│  │  ├─ pow2_is_multiple_of               134.9 ns      │ 332.6 ns      │ 150.5 ns      │ 154.5 ns      │ 1000    │ 128000
│  │  ├─ pow2_is_multiple_of_const         79.48 ns      │ 646.6 ns      │ 84.17 ns      │ 86.71 ns      │ 1000    │ 128000
│  │  ├─ pow2_is_multiple_of_reuse         106 ns        │ 270.1 ns      │ 118.5 ns      │ 121.1 ns      │ 1000    │ 128000
│  │  ├─ pow2_mod_floor                    170.1 ns      │ 1.941 µs      │ 201.3 ns      │ 206.8 ns      │ 1000    │ 64000
│  │  ├─ pow2_mod_floor_const              148.2 ns      │ 432.6 ns      │ 173.2 ns      │ 175.7 ns      │ 1000    │ 64000
│  │  ├─ pow2_mod_floor_reuse              157.6 ns      │ 404.4 ns      │ 176.3 ns      │ 179.8 ns      │ 1000    │ 64000
│  │  ├─ std_mod                           1.624 µs      │ 3.249 µs      │ 1.637 µs      │ 1.643 µs      │ 1000    │ 8000
│  │  ├─ std_mod_const                     152.9 ns      │ 1.884 µs      │ 168.5 ns      │ 173.2 ns      │ 1000    │ 64000
│  │  ├─ std_mod_reuse                     1.499 µs      │ 2.974 µs      │ 1.512 µs      │ 1.525 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_is_multiple_of           122.4 ns      │ 284.1 ns      │ 143.5 ns      │ 144.6 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_is_multiple_of_const     79.48 ns      │ 214.6 ns      │ 82.61 ns      │ 85.75 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_is_multiple_of_reuse     109.9 ns      │ 991.9 ns      │ 121.6 ns      │ 125.3 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_mod_floor                188.8 ns      │ 465.4 ns      │ 204.4 ns      │ 206.9 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_mod_floor_const          149.8 ns      │ 1.695 µs      │ 174.8 ns      │ 180.2 ns      │ 1000    │ 64000
│  │  ╰─ unb_pow2_mod_floor_reuse          157.6 ns      │ 1.682 µs      │ 177.9 ns      │ 187.3 ns      │ 1000    │ 64000
│  ╰─ u128                                               │               │               │               │         │
│     ├─ baseline_identity                 195.1 ns      │ 593.5 ns      │ 209.1 ns      │ 225.9 ns      │ 1000    │ 64000
│     ├─ pow2_is_multiple_of               384.1 ns      │ 718.5 ns      │ 393.5 ns      │ 397 ns        │ 1000    │ 32000
│     ├─ pow2_is_multiple_of_const         324.8 ns      │ 662.3 ns      │ 331 ns        │ 334 ns        │ 1000    │ 32000
│     ├─ pow2_is_multiple_of_reuse         368.5 ns      │ 574.8 ns      │ 377.9 ns      │ 378.5 ns      │ 1000    │ 32000
│     ├─ pow2_mod_floor                    818.5 ns      │ 7.449 µs      │ 837.3 ns      │ 859.7 ns      │ 1000    │ 16000
│     ├─ pow2_mod_floor_const              406 ns        │ 884.1 ns      │ 437.3 ns      │ 444.2 ns      │ 1000    │ 32000
│     ├─ pow2_mod_floor_reuse              549.8 ns      │ 1.218 µs      │ 596.6 ns      │ 608.3 ns      │ 1000    │ 32000
│     ├─ std_mod                           4.074 µs      │ 9.249 µs      │ 4.124 µs      │ 4.155 µs      │ 1000    │ 4000
│     ├─ std_mod_const                     368.5 ns      │ 1.324 µs      │ 381 ns        │ 394.4 ns      │ 1000    │ 16000
│     ├─ std_mod_reuse                     4.299 µs      │ 14.39 µs      │ 4.399 µs      │ 4.413 µs      │ 1000    │ 1000
│     ├─ unb_pow2_is_multiple_of           396.6 ns      │ 871.6 ns      │ 402.9 ns      │ 414 ns        │ 1000    │ 32000
│     ├─ unb_pow2_is_multiple_of_const     327.9 ns      │ 718.5 ns      │ 334.1 ns      │ 340.3 ns      │ 1000    │ 32000
│     ├─ unb_pow2_is_multiple_of_reuse     374.8 ns      │ 1.543 µs      │ 381 ns        │ 392.2 ns      │ 1000    │ 32000
│     ├─ unb_pow2_mod_floor                831 ns        │ 2.112 µs      │ 843.5 ns      │ 859.1 ns      │ 1000    │ 16000
│     ├─ unb_pow2_mod_floor_const          409.1 ns      │ 1.084 µs      │ 434.1 ns      │ 462.5 ns      │ 1000    │ 32000
│     ╰─ unb_pow2_mod_floor_reuse          512.3 ns      │ 1.037 µs      │ 524.8 ns      │ 563 ns        │ 1000    │ 16000
├─ mul                                                   │               │               │               │         │
│  ├─ i8                                                 │               │               │               │         │
│  │  ├─ baseline_identity                 11.22 ns      │ 51.06 ns      │ 13.66 ns      │ 13.84 ns      │ 1000    │ 1024000
│  │  ├─ pow2_mul                          31.05 ns      │ 86.12 ns      │ 34.95 ns      │ 36.1 ns       │ 1000    │ 256000
│  │  ├─ pow2_mul_const                    18.55 ns      │ 60.15 ns      │ 20.5 ns       │ 21.13 ns      │ 1000    │ 512000
│  │  ├─ pow2_mul_reuse                    20.69 ns      │ 53.9 ns       │ 22.84 ns      │ 23.67 ns      │ 1000    │ 512000
│  │  ├─ std_mul                           33.97 ns      │ 197.4 ns      │ 43.15 ns      │ 43.63 ns      │ 1000    │ 512000
│  │  ├─ std_mul_const                     19.13 ns      │ 58.39 ns      │ 22.26 ns      │ 22.78 ns      │ 1000    │ 512000
│  │  ├─ std_mul_reuse                     25.77 ns      │ 64.83 ns      │ 27.92 ns      │ 28.43 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_mul                      26.75 ns      │ 64.25 ns      │ 33.78 ns      │ 34.82 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_mul_const                18.94 ns      │ 259.7 ns      │ 21.47 ns      │ 22.28 ns      │ 1000    │ 512000
│  │  ╰─ unb_pow2_mul_reuse                20.89 ns      │ 55.07 ns      │ 22.45 ns      │ 23.25 ns      │ 1000    │ 512000
│  ├─ i16                                                │               │               │               │         │
│  │  ├─ baseline_identity                 25.77 ns      │ 77.53 ns      │ 27.33 ns      │ 29.4 ns       │ 1000    │ 512000
│  │  ├─ pow2_mul                          66.59 ns      │ 184.9 ns      │ 69.33 ns      │ 72.4 ns       │ 1000    │ 256000
│  │  ├─ pow2_mul_const                    36.51 ns      │ 97.06 ns      │ 39.64 ns      │ 40.17 ns      │ 1000    │ 256000
│  │  ├─ pow2_mul_reuse                    40.81 ns      │ 588.4 ns      │ 44.33 ns      │ 46.18 ns      │ 1000    │ 256000
│  │  ├─ std_mul                           61.12 ns      │ 117.7 ns      │ 64.64 ns      │ 66.69 ns      │ 1000    │ 256000
│  │  ├─ std_mul_const                     37.3 ns       │ 107.6 ns      │ 40.03 ns      │ 41.19 ns      │ 1000    │ 256000
│  │  ├─ std_mul_reuse                     42.37 ns      │ 100.9 ns      │ 47.06 ns      │ 47.76 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_mul                      51.75 ns      │ 135.3 ns      │ 57.22 ns      │ 57.67 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_mul_const                36.12 ns      │ 131.8 ns      │ 39.25 ns      │ 39.69 ns      │ 1000    │ 256000
│  │  ╰─ unb_pow2_mul_reuse                40.42 ns      │ 434.9 ns      │ 43.55 ns      │ 47.67 ns      │ 1000    │ 256000
│  ├─ i32                                                │               │               │               │         │
│  │  ├─ baseline_identity                 43.55 ns      │ 156.8 ns      │ 47.06 ns      │ 49.16 ns      │ 1000    │ 256000
│  │  ├─ pow2_mul                          91.2 ns       │ 238.8 ns      │ 102.9 ns      │ 105.7 ns      │ 1000    │ 128000
│  │  ├─ pow2_mul_const                    67.76 ns      │ 189.6 ns      │ 74.8 ns       │ 76.13 ns      │ 1000    │ 128000
│  │  ├─ pow2_mul_reuse                    74.8 ns       │ 1.054 µs      │ 80.26 ns      │ 84.6 ns       │ 1000    │ 128000
│  │  ├─ std_mul                           117.7 ns      │ 904.4 ns      │ 139.6 ns      │ 144.7 ns      │ 1000    │ 128000
│  │  ├─ std_mul_const                     73.62 ns      │ 172.8 ns      │ 81.05 ns      │ 85.69 ns      │ 1000    │ 256000
│  │  ├─ std_mul_reuse                     76.36 ns      │ 844.3 ns      │ 84.17 ns      │ 87.97 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_mul                      93.55 ns      │ 1.024 µs      │ 107.6 ns      │ 110.4 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_mul_const                75.19 ns      │ 232.6 ns      │ 81.05 ns      │ 83.9 ns       │ 1000    │ 256000
│  │  ╰─ unb_pow2_mul_reuse                76.36 ns      │ 162.3 ns      │ 83.39 ns      │ 84.95 ns      │ 1000    │ 128000
│  ├─ i64                                                │               │               │               │         │
│  │  ├─ baseline_identity                 71.67 ns      │ 316.9 ns      │ 81.05 ns      │ 83.93 ns      │ 1000    │ 64000
│  │  ├─ pow2_mul                          165.4 ns      │ 924.8 ns      │ 196.6 ns      │ 204 ns        │ 1000    │ 64000
│  │  ├─ pow2_mul_const                    151.3 ns      │ 490.4 ns      │ 166.9 ns      │ 172.3 ns      │ 1000    │ 64000
│  │  ├─ pow2_mul_reuse                    157.6 ns      │ 420.1 ns      │ 173.2 ns      │ 176.1 ns      │ 1000    │ 64000
│  │  ├─ std_mul                           296.6 ns      │ 596.6 ns      │ 323.2 ns      │ 330.4 ns      │ 1000    │ 64000
│  │  ├─ std_mul_const                     151.3 ns      │ 362.3 ns      │ 171.6 ns      │ 173.5 ns      │ 1000    │ 64000
│  │  ├─ std_mul_reuse                     149.8 ns      │ 540.4 ns      │ 166.9 ns      │ 169.6 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_mul                      174.8 ns      │ 518.5 ns      │ 195.1 ns      │ 198.2 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_mul_const                152.9 ns      │ 527.9 ns      │ 168.5 ns      │ 172.6 ns      │ 1000    │ 64000
│  │  ╰─ unb_pow2_mul_reuse                160.7 ns      │ 1.86 µs       │ 176.3 ns      │ 182.4 ns      │ 1000    │ 64000
│  ├─ i128                                               │               │               │               │         │
│  │  ├─ baseline_identity                 232.6 ns      │ 562.3 ns      │ 265.4 ns      │ 273.4 ns      │ 1000    │ 64000
│  │  ├─ pow2_mul                          1.062 µs      │ 5.118 µs      │ 1.124 µs      │ 1.181 µs      │ 1000    │ 16000
│  │  ├─ pow2_mul_const                    824.8 ns      │ 4.624 µs      │ 862.3 ns      │ 931.6 ns      │ 1000    │ 16000
│  │  ├─ pow2_mul_reuse                    981 ns        │ 2.687 µs      │ 1.006 µs      │ 1.15 µs       │ 1000    │ 16000
│  │  ├─ std_mul                           1.174 µs      │ 4.062 µs      │ 1.199 µs      │ 1.216 µs      │ 1000    │ 8000
│  │  ├─ std_mul_const                     843.5 ns      │ 1.693 µs      │ 856 ns        │ 867.6 ns      │ 1000    │ 16000
│  │  ├─ std_mul_reuse                     812.3 ns      │ 1.449 µs      │ 831 ns        │ 835.1 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_mul                      1.162 µs      │ 1.874 µs      │ 1.187 µs      │ 1.193 µs      │ 1000    │ 16000
│  │  ├─ unb_pow2_mul_const                831 ns        │ 1.262 µs      │ 849.8 ns      │ 851.4 ns      │ 1000    │ 16000
│  │  ╰─ unb_pow2_mul_reuse                962.3 ns      │ 1.618 µs      │ 987.3 ns      │ 992.6 ns      │ 1000    │ 16000
│  ├─ u8                                                 │               │               │               │         │
│  │  ├─ baseline_identity                 11.03 ns      │ 41.69 ns      │ 12.1 ns       │ 13.28 ns      │ 1000    │ 1024000
│  │  ├─ pow2_mul                          29.87 ns      │ 496.2 ns      │ 35.73 ns      │ 36.49 ns      │ 1000    │ 256000
│  │  ├─ pow2_mul_const                    19.33 ns      │ 277.5 ns      │ 21.28 ns      │ 22.27 ns      │ 1000    │ 512000
│  │  ├─ pow2_mul_reuse                    20.3 ns       │ 64.83 ns      │ 22.26 ns      │ 22.45 ns      │ 1000    │ 512000
│  │  ├─ std_mul                           35.54 ns      │ 97.26 ns      │ 41.2 ns       │ 44.85 ns      │ 1000    │ 512000
│  │  ├─ std_mul_const                     19.33 ns      │ 78.9 ns       │ 22.45 ns      │ 23.2 ns       │ 1000    │ 512000
│  │  ├─ std_mul_reuse                     24.6 ns       │ 69.91 ns      │ 28.7 ns       │ 28.96 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_mul                      28.7 ns       │ 192.3 ns      │ 35.73 ns      │ 36.13 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_mul_const                19.33 ns      │ 279 ns        │ 20.89 ns      │ 22.1 ns       │ 1000    │ 512000
│  │  ╰─ unb_pow2_mul_reuse                20.69 ns      │ 165.8 ns      │ 23.04 ns      │ 24.16 ns      │ 1000    │ 512000
│  ├─ u16                                                │               │               │               │         │
│  │  ├─ baseline_identity                 21.28 ns      │ 84.95 ns      │ 23.62 ns      │ 27 ns         │ 1000    │ 256000
│  │  ├─ pow2_mul                          67.37 ns      │ 145.8 ns      │ 69.72 ns      │ 71.87 ns      │ 1000    │ 256000
│  │  ├─ pow2_mul_const                    41.4 ns       │ 152.5 ns      │ 50.19 ns      │ 50.46 ns      │ 1000    │ 512000
│  │  ├─ pow2_mul_reuse                    40.81 ns      │ 566.9 ns      │ 45.5 ns       │ 47.94 ns      │ 1000    │ 256000
│  │  ├─ std_mul                           60.73 ns      │ 141.9 ns      │ 65.03 ns      │ 66.65 ns      │ 1000    │ 256000
│  │  ├─ std_mul_const                     36.51 ns      │ 100.1 ns      │ 39.25 ns      │ 40.24 ns      │ 1000    │ 256000
│  │  ├─ std_mul_reuse                     40.81 ns      │ 105.2 ns      │ 44.72 ns      │ 45.93 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_mul                      52.53 ns      │ 140.8 ns      │ 57.61 ns      │ 58.61 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_mul_const                36.12 ns      │ 103.7 ns      │ 39.25 ns      │ 39.64 ns      │ 1000    │ 256000
│  │  ╰─ unb_pow2_mul_reuse                40.42 ns      │ 662.3 ns      │ 43.94 ns      │ 45.32 ns      │ 1000    │ 256000
│  ├─ u32                                                │               │               │               │         │
│  │  ├─ baseline_identity                 44.33 ns      │ 120.8 ns      │ 46.67 ns      │ 47.69 ns      │ 1000    │ 256000
│  │  ├─ pow2_mul                          91.98 ns      │ 308.3 ns      │ 103.7 ns      │ 106.7 ns      │ 1000    │ 128000
│  │  ├─ pow2_mul_const                    71.67 ns      │ 232.6 ns      │ 78.7 ns       │ 80.86 ns      │ 1000    │ 128000
│  │  ├─ pow2_mul_reuse                    74.01 ns      │ 241.9 ns      │ 84.95 ns      │ 86.2 ns       │ 1000    │ 128000
│  │  ├─ std_mul                           116.9 ns      │ 461.5 ns      │ 131.8 ns      │ 136.3 ns      │ 1000    │ 128000
│  │  ├─ std_mul_const                     67.76 ns      │ 252.9 ns      │ 77.92 ns      │ 79.77 ns      │ 1000    │ 128000
│  │  ├─ std_mul_reuse                     74.8 ns       │ 149 ns        │ 81.83 ns      │ 82.8 ns       │ 1000    │ 128000
│  │  ├─ unb_pow2_mul                      95.11 ns      │ 1.241 µs      │ 105.2 ns      │ 107.8 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_mul_const                75.58 ns      │ 533.3 ns      │ 81.44 ns      │ 86.28 ns      │ 1000    │ 256000
│  │  ╰─ unb_pow2_mul_reuse                75.58 ns      │ 231.8 ns      │ 84.95 ns      │ 88.28 ns      │ 1000    │ 128000
│  ├─ u64                                                │               │               │               │         │
│  │  ├─ baseline_identity                 81.83 ns      │ 226.3 ns      │ 90.42 ns      │ 93.7 ns       │ 1000    │ 128000
│  │  ├─ pow2_mul                          170.1 ns      │ 727.9 ns      │ 191.9 ns      │ 197.1 ns      │ 1000    │ 64000
│  │  ├─ pow2_mul_const                    149.8 ns      │ 499.8 ns      │ 166.9 ns      │ 172.3 ns      │ 1000    │ 64000
│  │  ├─ pow2_mul_reuse                    157.6 ns      │ 352.9 ns      │ 173.2 ns      │ 177.4 ns      │ 1000    │ 64000
│  │  ├─ std_mul                           298.2 ns      │ 746.6 ns      │ 316.9 ns      │ 334.7 ns      │ 1000    │ 64000
│  │  ├─ std_mul_const                     149.8 ns      │ 529.4 ns      │ 163.8 ns      │ 167.3 ns      │ 1000    │ 64000
│  │  ├─ std_mul_reuse                     148.2 ns      │ 1.645 µs      │ 162.3 ns      │ 167.1 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_mul                      177.9 ns      │ 479.4 ns      │ 195.1 ns      │ 199.1 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_mul_const                146.6 ns      │ 465.4 ns      │ 165.4 ns      │ 170.3 ns      │ 1000    │ 64000
│  │  ╰─ unb_pow2_mul_reuse                163.8 ns      │ 620.1 ns      │ 184.1 ns      │ 188 ns        │ 1000    │ 64000
│  ╰─ u128                                               │               │               │               │         │
│     ├─ baseline_identity                 199.8 ns      │ 1.263 µs      │ 229.4 ns      │ 244.6 ns      │ 1000    │ 64000
│     ├─ pow2_mul                          1.043 µs      │ 1.349 µs      │ 1.074 µs      │ 1.077 µs      │ 1000    │ 16000
│     ├─ pow2_mul_const                    806 ns        │ 1.387 µs      │ 824.8 ns      │ 828.9 ns      │ 1000    │ 16000
│     ├─ pow2_mul_reuse                    968.5 ns      │ 1.687 µs      │ 981 ns        │ 985.6 ns      │ 1000    │ 16000
│     ├─ std_mul                           1.249 µs      │ 1.812 µs      │ 1.287 µs      │ 1.296 µs      │ 1000    │ 8000
│     ├─ std_mul_const                     818.5 ns      │ 1.556 µs      │ 831 ns        │ 831.1 ns      │ 1000    │ 16000
│     ├─ std_mul_reuse                     806 ns        │ 1.062 µs      │ 824.8 ns      │ 825.9 ns      │ 1000    │ 16000
│     ├─ unb_pow2_mul                      1.168 µs      │ 2.399 µs      │ 1.193 µs      │ 1.2 µs        │ 1000    │ 16000
│     ├─ unb_pow2_mul_const                818.5 ns      │ 1.524 µs      │ 837.3 ns      │ 840.2 ns      │ 1000    │ 16000
│     ╰─ unb_pow2_mul_reuse                956 ns        │ 1.643 µs      │ 974.8 ns      │ 978.8 ns      │ 1000    │ 16000
╰─ round                                                 │               │               │               │         │
   ├─ i8                                                 │               │               │               │         │
   │  ├─ baseline_identity                 10.54 ns      │ 53.51 ns      │ 12.3 ns       │ 12.8 ns       │ 1000    │ 512000
   │  ├─ pow2_ceil_to_multiple             30.26 ns      │ 88.86 ns      │ 33 ns         │ 33.85 ns      │ 1000    │ 256000
   │  ├─ pow2_ceil_to_multiple_const       20.89 ns      │ 82.61 ns      │ 23.43 ns      │ 23.8 ns       │ 1000    │ 512000
   │  ├─ pow2_ceil_to_multiple_reuse       20.69 ns      │ 110.9 ns      │ 23.23 ns      │ 24.15 ns      │ 1000    │ 512000
   │  ├─ pow2_floor_to_multiple            32.41 ns      │ 82.8 ns       │ 36.9 ns       │ 37.27 ns      │ 1000    │ 512000
   │  ├─ pow2_floor_to_multiple_const      20.79 ns      │ 44.62 ns      │ 21.77 ns      │ 22.25 ns      │ 1000    │ 1024000
   │  ├─ pow2_floor_to_multiple_reuse      23.04 ns      │ 46.67 ns      │ 23.82 ns      │ 24.13 ns      │ 1000    │ 512000
   │  ├─ std_div_mul                       856 ns        │ 10.14 µs      │ 887.3 ns      │ 929.7 ns      │ 1000    │ 16000
   │  ├─ std_div_mul_const                 21.67 ns      │ 59.37 ns      │ 23.62 ns      │ 24.42 ns      │ 1000    │ 512000
   │  ├─ std_div_mul_reuse                 824.8 ns      │ 1.124 µs      │ 837.3 ns      │ 839.4 ns      │ 1000    │ 16000
   │  ├─ unb_pow2_ceil_to_multiple         35.54 ns      │ 108 ns        │ 39.64 ns      │ 41.23 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_ceil_to_multiple_const   19.72 ns      │ 319.5 ns      │ 21.28 ns      │ 22.03 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   21.47 ns      │ 156.4 ns      │ 23.23 ns      │ 24.17 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_floor_to_multiple        37.1 ns       │ 255.8 ns      │ 43.74 ns      │ 44.9 ns       │ 1000    │ 512000
   │  ├─ unb_pow2_floor_to_multiple_const  20.79 ns      │ 51.75 ns      │ 22.26 ns      │ 23.15 ns      │ 1000    │ 1024000
   │  ╰─ unb_pow2_floor_to_multiple_reuse  20.3 ns       │ 78.9 ns       │ 21.28 ns      │ 21.77 ns      │ 1000    │ 512000
   ├─ i16                                                │               │               │               │         │
   │  ├─ baseline_identity                 25.97 ns      │ 73.62 ns      │ 27.33 ns      │ 28.53 ns      │ 1000    │ 512000
   │  ├─ pow2_ceil_to_multiple             68.15 ns      │ 276.3 ns      │ 73.23 ns      │ 74.23 ns      │ 1000    │ 256000
   │  ├─ pow2_ceil_to_multiple_const       38.86 ns      │ 123.2 ns      │ 45.5 ns       │ 46.23 ns      │ 1000    │ 256000
   │  ├─ pow2_ceil_to_multiple_reuse       43.94 ns      │ 157.2 ns      │ 49.01 ns      │ 49.38 ns      │ 1000    │ 256000
   │  ├─ pow2_floor_to_multiple            67.76 ns      │ 613.8 ns      │ 71.28 ns      │ 75.24 ns      │ 1000    │ 256000
   │  ├─ pow2_floor_to_multiple_const      36.51 ns      │ 325.5 ns      │ 38.08 ns      │ 39.6 ns       │ 1000    │ 256000
   │  ├─ pow2_floor_to_multiple_reuse      40.03 ns      │ 293.9 ns      │ 43.94 ns      │ 45.03 ns      │ 1000    │ 256000
   │  ├─ std_div_mul                       2.624 µs      │ 4.549 µs      │ 2.649 µs      │ 2.668 µs      │ 1000    │ 4000
   │  ├─ std_div_mul_const                 30.26 ns      │ 150.5 ns      │ 34.95 ns      │ 37.37 ns      │ 1000    │ 128000
   │  ├─ std_div_mul_reuse                 2.574 µs      │ 6.149 µs      │ 2.599 µs      │ 2.63 µs       │ 1000    │ 4000
   │  ├─ unb_pow2_ceil_to_multiple         52.92 ns      │ 191.2 ns      │ 64.25 ns      │ 66.83 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_ceil_to_multiple_const   41.59 ns      │ 113 ns        │ 45.11 ns      │ 46.66 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   45.11 ns      │ 118.5 ns      │ 48.23 ns      │ 49.24 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_floor_to_multiple        52.92 ns      │ 649.4 ns      │ 59.17 ns      │ 61.51 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_floor_to_multiple_const  37.3 ns       │ 662.3 ns      │ 39.25 ns      │ 40.97 ns      │ 1000    │ 256000
   │  ╰─ unb_pow2_floor_to_multiple_reuse  39.64 ns      │ 121.2 ns      │ 44.72 ns      │ 45.51 ns      │ 1000    │ 256000
   ├─ i32                                                │               │               │               │         │
   │  ├─ baseline_identity                 44.33 ns      │ 148.6 ns      │ 49.01 ns      │ 52.04 ns      │ 1000    │ 256000
   │  ├─ pow2_ceil_to_multiple             102.9 ns      │ 256 ns        │ 115.4 ns      │ 117.5 ns      │ 1000    │ 128000
   │  ├─ pow2_ceil_to_multiple_const       77.92 ns      │ 274.8 ns      │ 84.17 ns      │ 87.15 ns      │ 1000    │ 128000
   │  ├─ pow2_ceil_to_multiple_reuse       77.14 ns      │ 252.9 ns      │ 87.3 ns       │ 88.94 ns      │ 1000    │ 128000
   │  ├─ pow2_floor_to_multiple            95.11 ns      │ 263 ns        │ 108.3 ns      │ 112.6 ns      │ 1000    │ 128000
   │  ├─ pow2_floor_to_multiple_const      70.89 ns      │ 195.8 ns      │ 77.92 ns      │ 79.13 ns      │ 1000    │ 128000
   │  ├─ pow2_floor_to_multiple_reuse      75.58 ns      │ 228.7 ns      │ 82.61 ns      │ 84.06 ns      │ 1000    │ 128000
   │  ├─ std_div_mul                       1.299 µs      │ 2.824 µs      │ 1.312 µs      │ 1.329 µs      │ 1000    │ 8000
   │  ├─ std_div_mul_const                 85.73 ns      │ 260.7 ns      │ 101.3 ns      │ 103.2 ns      │ 1000    │ 128000
   │  ├─ std_div_mul_reuse                 1.299 µs      │ 20.81 µs      │ 1.312 µs      │ 1.358 µs      │ 1000    │ 8000
   │  ├─ unb_pow2_ceil_to_multiple         96.67 ns      │ 1.238 µs      │ 117.7 ns      │ 120.1 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_ceil_to_multiple_const   69.33 ns      │ 278.7 ns      │ 78.7 ns       │ 80.13 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   82.61 ns      │ 250.5 ns      │ 88.08 ns      │ 89.9 ns       │ 1000    │ 128000
   │  ├─ unb_pow2_floor_to_multiple        105.2 ns      │ 882.6 ns      │ 115.4 ns      │ 119.1 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_floor_to_multiple_const  71.67 ns      │ 305.2 ns      │ 79.48 ns      │ 82.23 ns      │ 1000    │ 128000
   │  ╰─ unb_pow2_floor_to_multiple_reuse  77.92 ns      │ 196.6 ns      │ 84.95 ns      │ 86.58 ns      │ 1000    │ 128000
   ├─ i64                                                │               │               │               │         │
   │  ├─ baseline_identity                 80.26 ns      │ 1.382 µs      │ 95.89 ns      │ 106 ns        │ 1000    │ 128000
   │  ├─ pow2_ceil_to_multiple             195.1 ns      │ 1.876 µs      │ 223.2 ns      │ 228.1 ns      │ 1000    │ 64000
   │  ├─ pow2_ceil_to_multiple_const       162.3 ns      │ 487.3 ns      │ 187.3 ns      │ 190.6 ns      │ 1000    │ 64000
   │  ├─ pow2_ceil_to_multiple_reuse       174.8 ns      │ 552.9 ns      │ 201.3 ns      │ 205.4 ns      │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple            190.4 ns      │ 621.6 ns      │ 218.5 ns      │ 223.3 ns      │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple_const      152.9 ns      │ 485.7 ns      │ 176.3 ns      │ 186 ns        │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple_reuse      157.6 ns      │ 2.735 µs      │ 171.6 ns      │ 187.2 ns      │ 1000    │ 64000
   │  ├─ std_div_mul                       1.637 µs      │ 23.88 µs      │ 1.674 µs      │ 1.765 µs      │ 1000    │ 8000
   │  ├─ std_div_mul_const                 182.6 ns      │ 552.9 ns      │ 204.4 ns      │ 207.3 ns      │ 1000    │ 64000
   │  ├─ std_div_mul_reuse                 1.512 µs      │ 3.762 µs      │ 1.524 µs      │ 1.54 µs       │ 1000    │ 8000
   │  ├─ unb_pow2_ceil_to_multiple         207.6 ns      │ 645.1 ns      │ 227.9 ns      │ 232.1 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_ceil_to_multiple_const   165.4 ns      │ 737.3 ns      │ 184.1 ns      │ 188.8 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   159.1 ns      │ 1.993 µs      │ 174.8 ns      │ 184.9 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple        179.4 ns      │ 446.6 ns      │ 209.1 ns      │ 210.1 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple_const  152.9 ns      │ 481 ns        │ 173.2 ns      │ 177.1 ns      │ 1000    │ 64000
   │  ╰─ unb_pow2_floor_to_multiple_reuse  156 ns        │ 499.8 ns      │ 181 ns        │ 184.8 ns      │ 1000    │ 64000
   ├─ i128                                               │               │               │               │         │
   │  ├─ baseline_identity                 199.8 ns      │ 774.8 ns      │ 227.9 ns      │ 232.8 ns      │ 1000    │ 32000
   │  ├─ pow2_ceil_to_multiple             1.043 µs      │ 2.312 µs      │ 1.087 µs      │ 1.088 µs      │ 1000    │ 16000
   │  ├─ pow2_ceil_to_multiple_const       627.9 ns      │ 5.412 µs      │ 659.1 ns      │ 683 ns        │ 1000    │ 32000
   │  ├─ pow2_ceil_to_multiple_reuse       668.5 ns      │ 8.818 µs      │ 687.3 ns      │ 711.4 ns      │ 1000    │ 16000
   │  ├─ pow2_floor_to_multiple            806 ns        │ 8.899 µs      │ 831 ns        │ 872.9 ns      │ 1000    │ 16000
   │  ├─ pow2_floor_to_multiple_const      424.8 ns      │ 32.7 µs       │ 452.9 ns      │ 501.5 ns      │ 1000    │ 32000
   │  ├─ pow2_floor_to_multiple_reuse      565.4 ns      │ 1.146 µs      │ 626.3 ns      │ 631.1 ns      │ 1000    │ 32000
   │  ├─ std_div_mul                       9.499 µs      │ 20.89 µs      │ 9.599 µs      │ 9.653 µs      │ 1000    │ 1000
   │  ├─ std_div_mul_const                 687.3 ns      │ 2.462 µs      │ 718.5 ns      │ 750.8 ns      │ 1000    │ 16000
   │  ├─ std_div_mul_reuse                 8.899 µs      │ 16.09 µs      │ 9.049 µs      │ 9.176 µs      │ 1000    │ 2000
   │  ├─ unb_pow2_ceil_to_multiple         1.112 µs      │ 1.956 µs      │ 1.131 µs      │ 1.155 µs      │ 1000    │ 16000
   │  ├─ unb_pow2_ceil_to_multiple_const   593.5 ns      │ 1.487 µs      │ 624.8 ns      │ 635.6 ns      │ 1000    │ 16000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   656 ns        │ 2.524 µs      │ 674.8 ns      │ 694.3 ns      │ 1000    │ 16000
   │  ├─ unb_pow2_floor_to_multiple        818.5 ns      │ 2.343 µs      │ 837.3 ns      │ 856.5 ns      │ 1000    │ 16000
   │  ├─ unb_pow2_floor_to_multiple_const  427.9 ns      │ 1.84 µs       │ 456 ns        │ 467.4 ns      │ 1000    │ 32000
   │  ╰─ unb_pow2_floor_to_multiple_reuse  565.4 ns      │ 3.587 µs      │ 602.9 ns      │ 623.8 ns      │ 1000    │ 32000
   ├─ u8                                                 │               │               │               │         │
   │  ├─ baseline_identity                 11.03 ns      │ 56.73 ns      │ 12.2 ns       │ 13.39 ns      │ 1000    │ 1024000
   │  ├─ pow2_ceil_to_multiple             30.26 ns      │ 81.83 ns      │ 33.39 ns      │ 34.43 ns      │ 1000    │ 256000
   │  ├─ pow2_ceil_to_multiple_const       19.91 ns      │ 242.9 ns      │ 23.23 ns      │ 24.06 ns      │ 1000    │ 512000
   │  ├─ pow2_ceil_to_multiple_reuse       21.67 ns      │ 98.04 ns      │ 24.8 ns       │ 25.52 ns      │ 1000    │ 512000
   │  ├─ pow2_floor_to_multiple            32.02 ns      │ 134.7 ns      │ 35.34 ns      │ 37.92 ns      │ 1000    │ 512000
   │  ├─ pow2_floor_to_multiple_const      18.94 ns      │ 94.13 ns      │ 20.69 ns      │ 22.03 ns      │ 1000    │ 512000
   │  ├─ pow2_floor_to_multiple_reuse      20.5 ns       │ 71.87 ns      │ 21.87 ns      │ 22.25 ns      │ 1000    │ 512000
   │  ├─ std_div_mul                       837.3 ns      │ 1.174 µs      │ 868.5 ns      │ 869.8 ns      │ 1000    │ 16000
   │  ├─ std_div_mul_const                 18.74 ns      │ 61.71 ns      │ 20.11 ns      │ 20.78 ns      │ 1000    │ 512000
   │  ├─ std_div_mul_reuse                 849.8 ns      │ 2.124 µs      │ 862.3 ns      │ 868.5 ns      │ 1000    │ 16000
   │  ├─ unb_pow2_ceil_to_multiple         35.73 ns      │ 115 ns        │ 41.79 ns      │ 43.05 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_ceil_to_multiple_const   19.13 ns      │ 57.8 ns       │ 21.47 ns      │ 22.52 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   21.28 ns      │ 208.5 ns      │ 23.04 ns      │ 24.1 ns       │ 1000    │ 512000
   │  ├─ unb_pow2_floor_to_multiple        36.32 ns      │ 106.4 ns      │ 44.52 ns      │ 45.28 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_floor_to_multiple_const  19.91 ns      │ 72.26 ns      │ 21.67 ns      │ 22.55 ns      │ 1000    │ 512000
   │  ╰─ unb_pow2_floor_to_multiple_reuse  21.67 ns      │ 60.15 ns      │ 22.65 ns      │ 23.3 ns       │ 1000    │ 512000
   ├─ u16                                                │               │               │               │         │
   │  ├─ baseline_identity                 26.16 ns      │ 89.64 ns      │ 27.33 ns      │ 28.7 ns       │ 1000    │ 512000
   │  ├─ pow2_ceil_to_multiple             68.94 ns      │ 190 ns        │ 73.62 ns      │ 75.29 ns      │ 1000    │ 256000
   │  ├─ pow2_ceil_to_multiple_const       41.59 ns      │ 117.7 ns      │ 46.28 ns      │ 46.78 ns      │ 1000    │ 256000
   │  ├─ pow2_ceil_to_multiple_reuse       41.98 ns      │ 124 ns        │ 48.62 ns      │ 49.12 ns      │ 1000    │ 256000
   │  ├─ pow2_floor_to_multiple            67.37 ns      │ 165.4 ns      │ 72.06 ns      │ 75.87 ns      │ 1000    │ 256000
   │  ├─ pow2_floor_to_multiple_const      37.3 ns       │ 102.9 ns      │ 43.55 ns      │ 44.54 ns      │ 1000    │ 256000
   │  ├─ pow2_floor_to_multiple_reuse      41.2 ns       │ 106.4 ns      │ 46.28 ns      │ 47.14 ns      │ 1000    │ 256000
   │  ├─ std_div_mul                       856 ns        │ 1.506 µs      │ 881 ns        │ 883.5 ns      │ 1000    │ 16000
   │  ├─ std_div_mul_const                 36.9 ns       │ 106.4 ns      │ 42.37 ns      │ 43.11 ns      │ 1000    │ 256000
   │  ├─ std_div_mul_reuse                 831 ns        │ 9.599 µs      │ 856 ns        │ 893.5 ns      │ 1000    │ 16000
   │  ├─ unb_pow2_ceil_to_multiple         57.22 ns      │ 155.6 ns      │ 63.86 ns      │ 64.78 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_ceil_to_multiple_const   39.25 ns      │ 113 ns        │ 45.5 ns       │ 46.5 ns       │ 1000    │ 256000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   42.37 ns      │ 108 ns        │ 47.06 ns      │ 47.84 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_floor_to_multiple        54.48 ns      │ 452.1 ns      │ 60.34 ns      │ 62.19 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_floor_to_multiple_const  38.86 ns      │ 128.7 ns      │ 41.98 ns      │ 43.26 ns      │ 1000    │ 256000
   │  ╰─ unb_pow2_floor_to_multiple_reuse  40.81 ns      │ 231.4 ns      │ 44.72 ns      │ 46.37 ns      │ 1000    │ 256000
   ├─ u32                                                │               │               │               │         │
   │  ├─ baseline_identity                 43.94 ns      │ 115 ns        │ 47.84 ns      │ 50.42 ns      │ 1000    │ 256000
   │  ├─ pow2_ceil_to_multiple             101.3 ns      │ 233.3 ns      │ 113 ns        │ 114.8 ns      │ 1000    │ 128000
   │  ├─ pow2_ceil_to_multiple_const       78.7 ns       │ 192.7 ns      │ 84.95 ns      │ 86.32 ns      │ 1000    │ 128000
   │  ├─ pow2_ceil_to_multiple_reuse       77.14 ns      │ 202.1 ns      │ 84.17 ns      │ 86.46 ns      │ 1000    │ 128000
   │  ├─ pow2_floor_to_multiple            93.55 ns      │ 335.7 ns      │ 106.8 ns      │ 108 ns        │ 1000    │ 128000
   │  ├─ pow2_floor_to_multiple_const      75.58 ns      │ 344.3 ns      │ 85.34 ns      │ 93.06 ns      │ 1000    │ 256000
   │  ├─ pow2_floor_to_multiple_reuse      80.65 ns      │ 456.8 ns      │ 87.69 ns      │ 91 ns         │ 1000    │ 256000
   │  ├─ std_div_mul                       1.287 µs      │ 1.824 µs      │ 1.312 µs      │ 1.307 µs      │ 1000    │ 8000
   │  ├─ std_div_mul_const                 70.89 ns      │ 1.946 µs      │ 79.48 ns      │ 82.98 ns      │ 1000    │ 128000
   │  ├─ std_div_mul_reuse                 1.262 µs      │ 18.17 µs      │ 1.274 µs      │ 1.393 µs      │ 1000    │ 8000
   │  ├─ unb_pow2_ceil_to_multiple         97.45 ns      │ 915.4 ns      │ 118.5 ns      │ 121.7 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_ceil_to_multiple_const   77.14 ns      │ 202.9 ns      │ 85.73 ns      │ 86.12 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   81.83 ns      │ 227.1 ns      │ 89.64 ns      │ 91.77 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_floor_to_multiple        101.3 ns      │ 290.4 ns      │ 114.6 ns      │ 117.7 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_floor_to_multiple_const  71.67 ns      │ 214.6 ns      │ 78.7 ns       │ 80.05 ns      │ 1000    │ 128000
   │  ╰─ unb_pow2_floor_to_multiple_reuse  75.58 ns      │ 281 ns        │ 85.73 ns      │ 90.85 ns      │ 1000    │ 128000
   ├─ u64                                                │               │               │               │         │
   │  ├─ baseline_identity                 81.83 ns      │ 212.3 ns      │ 91.98 ns      │ 95.76 ns      │ 1000    │ 128000
   │  ├─ pow2_ceil_to_multiple             193.5 ns      │ 565.4 ns      │ 209.9 ns      │ 213.7 ns      │ 1000    │ 64000
   │  ├─ pow2_ceil_to_multiple_const       159.1 ns      │ 581 ns        │ 179.4 ns      │ 182.8 ns      │ 1000    │ 64000
   │  ├─ pow2_ceil_to_multiple_reuse       163.8 ns      │ 545.1 ns      │ 182.6 ns      │ 186.5 ns      │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple            177.9 ns      │ 423.2 ns      │ 196.6 ns      │ 199.3 ns      │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple_const      152.9 ns      │ 393.5 ns      │ 171.6 ns      │ 173.6 ns      │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple_reuse      156 ns        │ 448.2 ns      │ 176.3 ns      │ 179.7 ns      │ 1000    │ 64000
   │  ├─ std_div_mul                       1.637 µs      │ 3.274 µs      │ 1.687 µs      │ 1.698 µs      │ 1000    │ 8000
   │  ├─ std_div_mul_const                 151.3 ns      │ 524.8 ns      │ 170.1 ns      │ 173.6 ns      │ 1000    │ 64000
   │  ├─ std_div_mul_reuse                 1.499 µs      │ 2.362 µs      │ 1.512 µs      │ 1.517 µs      │ 1000    │ 8000
   │  ├─ unb_pow2_ceil_to_multiple         191.9 ns      │ 677.9 ns      │ 223.2 ns      │ 223.6 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_ceil_to_multiple_const   157.6 ns      │ 545.1 ns      │ 179.4 ns      │ 184.4 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   166.9 ns      │ 396.6 ns      │ 187.3 ns      │ 190.4 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple        193.5 ns      │ 563.8 ns      │ 216.9 ns      │ 221.1 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple_const  151.3 ns      │ 373.2 ns      │ 166.9 ns      │ 170.8 ns      │ 1000    │ 64000
   │  ╰─ unb_pow2_floor_to_multiple_reuse  160.7 ns      │ 341.9 ns      │ 174.8 ns      │ 177.7 ns      │ 1000    │ 64000
   ╰─ u128                                               │               │               │               │         │
      ├─ baseline_identity                 196.6 ns      │ 562.3 ns      │ 209.1 ns      │ 220.3 ns      │ 1000    │ 64000
      ├─ pow2_ceil_to_multiple             1.062 µs      │ 2.193 µs      │ 1.074 µs      │ 1.104 µs      │ 1000    │ 16000
      ├─ pow2_ceil_to_multiple_const       599.8 ns      │ 1.206 µs      │ 615.4 ns      │ 628.3 ns      │ 1000    │ 32000
      ├─ pow2_ceil_to_multiple_reuse       656 ns        │ 1.449 µs      │ 674.8 ns      │ 682.6 ns      │ 1000    │ 16000
      ├─ pow2_floor_to_multiple            812.3 ns      │ 2.543 µs      │ 862.3 ns      │ 869.8 ns      │ 1000    │ 16000
      ├─ pow2_floor_to_multiple_const      399.8 ns      │ 752.9 ns      │ 427.9 ns      │ 431.9 ns      │ 1000    │ 32000
      ├─ pow2_floor_to_multiple_reuse      540.4 ns      │ 993.5 ns      │ 584.1 ns      │ 593.9 ns      │ 1000    │ 32000
      ├─ std_div_mul                       4.124 µs      │ 5.449 µs      │ 4.249 µs      │ 4.281 µs      │ 1000    │ 4000
      ├─ std_div_mul_const                 402.9 ns      │ 1.037 µs      │ 440.4 ns      │ 447.3 ns      │ 1000    │ 32000
      ├─ std_div_mul_reuse                 4.349 µs      │ 9.874 µs      │ 4.399 µs      │ 4.466 µs      │ 1000    │ 4000
      ├─ unb_pow2_ceil_to_multiple         1.112 µs      │ 1.587 µs      │ 1.143 µs      │ 1.145 µs      │ 1000    │ 16000
      ├─ unb_pow2_ceil_to_multiple_const   606 ns        │ 1.559 µs      │ 624.8 ns      │ 658.7 ns      │ 1000    │ 32000
      ├─ unb_pow2_ceil_to_multiple_reuse   662.3 ns      │ 9.599 µs      │ 674.8 ns      │ 699.2 ns      │ 1000    │ 16000
      ├─ unb_pow2_floor_to_multiple        812.3 ns      │ 9.087 µs      │ 824.8 ns      │ 863.4 ns      │ 1000    │ 16000
      ├─ unb_pow2_floor_to_multiple_const  402.9 ns      │ 1.187 µs      │ 437.3 ns      │ 449.8 ns      │ 1000    │ 32000
      ╰─ unb_pow2_floor_to_multiple_reuse  543.5 ns      │ 1.39 µs       │ 565.4 ns      │ 579.6 ns      │ 1000    │ 32000


```
