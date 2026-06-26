# `public` bench with default target
```
public                                     fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ div                                                   │               │               │               │         │
│  ├─ i8                                                 │               │               │               │         │
│  │  ├─ baseline_identity                 8.774 ns      │ 38.07 ns      │ 10.72 ns      │ 10.96 ns      │ 1000    │ 512000
│  │  ├─ pow2_div                          359.1 ns      │ 493.5 ns      │ 362.2 ns      │ 364.3 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_ceil                     462.2 ns      │ 1.077 µs      │ 477.9 ns      │ 484.1 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_ceil_const               58.38 ns      │ 343.5 ns      │ 59.94 ns      │ 60.76 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_ceil_reuse               315.4 ns      │ 506 ns        │ 318.5 ns      │ 321.7 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_const                    47.83 ns      │ 95.88 ns      │ 52.13 ns      │ 52.71 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_floor                    199.7 ns      │ 359.1 ns      │ 202.9 ns      │ 204.3 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_floor_const              34.16 ns      │ 10.36 µs      │ 56.82 ns      │ 86.18 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_floor_reuse              28.3 ns       │ 28.1 µs       │ 50.18 ns      │ 172 ns        │ 1000    │ 256000
│  │  ├─ pow2_div_reuse                    54.47 ns      │ 174.2 µs      │ 90.41 ns      │ 970 ns        │ 1000    │ 128000
│  │  ├─ std_div                           968.5 ns      │ 18.36 µs      │ 1.081 µs      │ 1.147 µs      │ 1000    │ 16000
│  │  ├─ std_div_const                     53.3 ns       │ 1.214 µs      │ 62.29 ns      │ 68.02 ns      │ 1000    │ 256000
│  │  ├─ std_div_reuse                     912.2 ns      │ 17.24 µs      │ 924.7 ns      │ 962 ns        │ 1000    │ 16000
│  │  ├─ unb_pow2_div                      484.1 ns      │ 405.7 µs      │ 552.9 ns      │ 1.908 µs      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_ceil                 593.5 ns      │ 31.76 µs      │ 868.5 ns      │ 1.117 µs      │ 1000    │ 16000
│  │  ├─ unb_pow2_div_ceil_const           68.54 ns      │ 238 ns        │ 78.69 ns      │ 80.49 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_ceil_reuse           421.6 ns      │ 9.884 µs      │ 540.4 ns      │ 567.4 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_const                49 ns         │ 4.332 µs      │ 83.38 ns      │ 106.9 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_floor                210.7 ns      │ 23.48 µs      │ 293.5 ns      │ 446 ns        │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor_const          27.52 ns      │ 17.32 µs      │ 48.61 ns      │ 84.62 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_floor_reuse          41.97 ns      │ 4.869 µs      │ 48.22 ns      │ 66.89 ns      │ 1000    │ 128000
│  │  ╰─ unb_pow2_div_reuse                75.57 ns      │ 4.455 µs      │ 97.44 ns      │ 111.3 ns      │ 1000    │ 128000
│  ├─ i16                                                │               │               │               │         │
│  │  ├─ baseline_identity                 23.22 ns      │ 1.007 µs      │ 42.36 ns      │ 49.01 ns      │ 1000    │ 256000
│  │  ├─ pow2_div                          506 ns        │ 11.28 µs      │ 518.5 ns      │ 616.1 ns      │ 1000    │ 16000
│  │  ├─ pow2_div_ceil                     499.7 ns      │ 5.899 µs      │ 762.2 ns      │ 704.6 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_ceil_const               134.1 ns      │ 224.7 ns      │ 138 ns        │ 146.8 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_ceil_reuse               518.5 ns      │ 909.1 ns      │ 552.9 ns      │ 554.6 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_const                    72.44 ns      │ 411.5 ns      │ 76.35 ns      │ 90.72 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_floor                    243.5 ns      │ 1.574 µs      │ 359.1 ns      │ 357 ns        │ 1000    │ 32000
│  │  ├─ pow2_div_floor_const              49.79 ns      │ 308.7 ns      │ 81.23 ns      │ 83.51 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_floor_reuse              51.74 ns      │ 407.2 ns      │ 85.33 ns      │ 88.01 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_reuse                    76.35 ns      │ 815.4 ns      │ 122.4 ns      │ 108.8 ns      │ 1000    │ 128000
│  │  ├─ std_div                           924.7 ns      │ 10.09 µs      │ 1.068 µs      │ 1.105 µs      │ 1000    │ 16000
│  │  ├─ std_div_const                     57.6 ns       │ 2.667 µs      │ 93.54 ns      │ 94.19 ns      │ 1000    │ 128000
│  │  ├─ std_div_reuse                     2.699 µs      │ 164.3 µs      │ 2.749 µs      │ 3.313 µs      │ 1000    │ 4000
│  │  ├─ unb_pow2_div                      649.7 ns      │ 10.19 µs      │ 818.5 ns      │ 838.3 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_div_ceil                 493.5 ns      │ 11 µs         │ 831 ns        │ 843.8 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_div_ceil_const           134.1 ns      │ 2.079 µs      │ 140.4 ns      │ 149.6 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil_reuse           337.2 ns      │ 26.21 µs      │ 481 ns        │ 489.5 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_div_const                74 ns         │ 1.376 µs      │ 124 ns        │ 112 ns        │ 1000    │ 128000
│  │  ├─ unb_pow2_div_floor                243.5 ns      │ 3.206 µs      │ 359.1 ns      │ 355.8 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_floor_const          43.54 ns      │ 2.229 µs      │ 65.41 ns      │ 62.69 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_floor_reuse          43.54 ns      │ 316.1 ns      │ 68.54 ns      │ 63.62 ns      │ 1000    │ 128000
│  │  ╰─ unb_pow2_div_reuse                76.35 ns      │ 1.548 µs      │ 133.3 ns      │ 128.6 ns      │ 1000    │ 128000
│  ├─ i32                                                │               │               │               │         │
│  │  ├─ baseline_identity                 43.54 ns      │ 746.6 ns      │ 68.93 ns      │ 86.27 ns      │ 1000    │ 256000
│  │  ├─ pow2_div                          643.5 ns      │ 48.44 µs      │ 849.7 ns      │ 1.098 µs      │ 1000    │ 16000
│  │  ├─ pow2_div_ceil                     699.7 ns      │ 43.84 µs      │ 962.2 ns      │ 1.325 µs      │ 1000    │ 16000
│  │  ├─ pow2_div_ceil_const               206 ns        │ 14.59 µs      │ 331 ns        │ 363.6 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_ceil_reuse               360.7 ns      │ 10 µs         │ 391.1 ns      │ 434.6 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_const                    231 ns        │ 2.387 µs      │ 274.7 ns      │ 281.6 ns      │ 1000    │ 16000
│  │  ├─ pow2_div_floor                    302.9 ns      │ 18.57 µs      │ 440.4 ns      │ 467.5 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_floor_const              145.8 ns      │ 180.6 µs      │ 154.4 ns      │ 342.7 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_floor_reuse              98.22 ns      │ 2.03 µs       │ 155.2 ns      │ 167.5 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_reuse                    154.4 ns      │ 1.282 µs      │ 256 ns        │ 236.5 ns      │ 1000    │ 64000
│  │  ├─ std_div                           1.399 µs      │ 3.387 µs      │ 1.412 µs      │ 1.432 µs      │ 1000    │ 8000
│  │  ├─ std_div_const                     120.1 ns      │ 362.2 ns      │ 195.1 ns      │ 185.2 ns      │ 1000    │ 64000
│  │  ├─ std_div_reuse                     1.387 µs      │ 2.399 µs      │ 1.412 µs      │ 1.41 µs       │ 1000    │ 8000
│  │  ├─ unb_pow2_div                      499.7 ns      │ 1.399 µs      │ 918.5 ns      │ 862.4 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_div_ceil                 509.1 ns      │ 2.024 µs      │ 515.4 ns      │ 528.1 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_ceil_const           204.4 ns      │ 320.1 ns      │ 207.6 ns      │ 209 ns        │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil_reuse           279.4 ns      │ 1.009 µs      │ 284.1 ns      │ 332 ns        │ 1000    │ 64000
│  │  ├─ unb_pow2_div_const                152.9 ns      │ 266.9 ns      │ 157.6 ns      │ 162.1 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor                309.1 ns      │ 790.4 ns      │ 382.6 ns      │ 390.7 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_floor_const          95.1 ns       │ 178.6 ns      │ 99 ns         │ 101.2 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_floor_reuse          95.88 ns      │ 1.307 µs      │ 108.3 ns      │ 114.3 ns      │ 1000    │ 128000
│  │  ╰─ unb_pow2_div_reuse                152.9 ns      │ 2.295 µs      │ 159.1 ns      │ 172.5 ns      │ 1000    │ 64000
│  ├─ i64                                                │               │               │               │         │
│  │  ├─ baseline_identity                 76.35 ns      │ 256 ns        │ 86.5 ns       │ 91.1 ns       │ 1000    │ 128000
│  │  ├─ pow2_div                          465.4 ns      │ 5.859 µs      │ 481 ns        │ 513.9 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_ceil                     587.2 ns      │ 4.846 µs      │ 596.6 ns      │ 620.8 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_ceil_const               359.1 ns      │ 787.2 ns      │ 368.5 ns      │ 371.1 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_ceil_reuse               540.4 ns      │ 974.7 ns      │ 546.6 ns      │ 549 ns        │ 1000    │ 32000
│  │  ├─ pow2_div_const                    440.4 ns      │ 1.14 µs       │ 449.7 ns      │ 472 ns        │ 1000    │ 32000
│  │  ├─ pow2_div_floor                    321.6 ns      │ 3.793 µs      │ 331 ns        │ 344.9 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_floor_const              271.6 ns      │ 688.8 ns      │ 288.8 ns      │ 292.3 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_floor_reuse              232.6 ns      │ 543.5 ns      │ 257.6 ns      │ 262.4 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_reuse                    484.1 ns      │ 3.799 µs      │ 493.5 ns      │ 504.8 ns      │ 1000    │ 32000
│  │  ├─ std_div                           1.674 µs      │ 18.12 µs      │ 1.699 µs      │ 1.755 µs      │ 1000    │ 8000
│  │  ├─ std_div_const                     393.5 ns      │ 4.471 µs      │ 402.9 ns      │ 419.1 ns      │ 1000    │ 32000
│  │  ├─ std_div_reuse                     1.524 µs      │ 17.37 µs      │ 1.549 µs      │ 1.607 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div                      502.9 ns      │ 3.684 µs      │ 515.4 ns      │ 525.2 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_ceil                 584.1 ns      │ 2.865 µs      │ 596.6 ns      │ 611.6 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_ceil_const           356 ns        │ 793.5 ns      │ 368.5 ns      │ 373.3 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_ceil_reuse           543.5 ns      │ 2.037 µs      │ 549.7 ns      │ 587.5 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_div_const                434.1 ns      │ 837.2 ns      │ 449.7 ns      │ 506.8 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_floor                324.7 ns      │ 677.9 ns      │ 343.5 ns      │ 369 ns        │ 1000    │ 32000
│  │  ├─ unb_pow2_div_floor_const          271.6 ns      │ 1.02 µs       │ 287.2 ns      │ 302.2 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor_reuse          237.2 ns      │ 599.7 ns      │ 262.2 ns      │ 265.9 ns      │ 1000    │ 64000
│  │  ╰─ unb_pow2_div_reuse                481 ns        │ 5.393 µs      │ 496.6 ns      │ 522.8 ns      │ 1000    │ 32000
│  ├─ i128                                               │               │               │               │         │
│  │  ├─ baseline_identity                 234.1 ns      │ 516.9 ns      │ 254.4 ns      │ 261.1 ns      │ 1000    │ 64000
│  │  ├─ pow2_div                          1.349 µs      │ 1.924 µs      │ 1.399 µs      │ 1.402 µs      │ 1000    │ 8000
│  │  ├─ pow2_div_ceil                     2.137 µs      │ 3.237 µs      │ 2.162 µs      │ 2.166 µs      │ 1000    │ 8000
│  │  ├─ pow2_div_ceil_const               1.037 µs      │ 1.599 µs      │ 1.049 µs      │ 1.055 µs      │ 1000    │ 16000
│  │  ├─ pow2_div_ceil_reuse               2.099 µs      │ 4.562 µs      │ 2.124 µs      │ 2.178 µs      │ 1000    │ 8000
│  │  ├─ pow2_div_const                    1.499 µs      │ 2.774 µs      │ 1.524 µs      │ 1.535 µs      │ 1000    │ 8000
│  │  ├─ pow2_div_floor                    1.062 µs      │ 5.006 µs      │ 1.074 µs      │ 1.093 µs      │ 1000    │ 16000
│  │  ├─ pow2_div_floor_const              856 ns        │ 9.537 µs      │ 874.7 ns      │ 918.6 ns      │ 1000    │ 16000
│  │  ├─ pow2_div_floor_reuse              1.043 µs      │ 5.843 µs      │ 1.068 µs      │ 1.104 µs      │ 1000    │ 16000
│  │  ├─ pow2_div_reuse                    1.899 µs      │ 3.787 µs      │ 1.924 µs      │ 1.955 µs      │ 1000    │ 8000
│  │  ├─ std_div                           9.299 µs      │ 17.94 µs      │ 9.349 µs      │ 9.493 µs      │ 1000    │ 2000
│  │  ├─ std_div_const                     1.056 µs      │ 2.106 µs      │ 1.087 µs      │ 1.109 µs      │ 1000    │ 16000
│  │  ├─ std_div_reuse                     7.999 µs      │ 14.09 µs      │ 8.149 µs      │ 8.206 µs      │ 1000    │ 2000
│  │  ├─ unb_pow2_div                      1.449 µs      │ 2.012 µs      │ 1.487 µs      │ 1.492 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div_ceil                 2.124 µs      │ 3.262 µs      │ 2.149 µs      │ 2.151 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div_ceil_const           1.062 µs      │ 1.656 µs      │ 1.081 µs      │ 1.091 µs      │ 1000    │ 16000
│  │  ├─ unb_pow2_div_ceil_reuse           2.137 µs      │ 18.67 µs      │ 2.162 µs      │ 2.225 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div_const                1.499 µs      │ 14.93 µs      │ 1.512 µs      │ 1.553 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div_floor                1.093 µs      │ 2.318 µs      │ 1.112 µs      │ 1.188 µs      │ 1000    │ 16000
│  │  ├─ unb_pow2_div_floor_const          862.2 ns      │ 2.287 µs      │ 874.7 ns      │ 1.053 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div_floor_reuse          1.024 µs      │ 1.756 µs      │ 1.043 µs      │ 1.051 µs      │ 1000    │ 16000
│  │  ╰─ unb_pow2_div_reuse                1.824 µs      │ 3.537 µs      │ 1.899 µs      │ 1.935 µs      │ 1000    │ 8000
│  ├─ u8                                                 │               │               │               │         │
│  │  ├─ baseline_identity                 9.165 ns      │ 41.39 ns      │ 11.5 ns       │ 11.87 ns      │ 1000    │ 512000
│  │  ├─ pow2_div                          103.6 ns      │ 170.8 ns      │ 106.8 ns      │ 107.4 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_ceil                     449.7 ns      │ 2.065 µs      │ 459.1 ns      │ 460.9 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_ceil_const               36.5 ns       │ 93.54 ns      │ 38.85 ns      │ 40.17 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_ceil_reuse               245.1 ns      │ 487.2 ns      │ 246.6 ns      │ 251.9 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_const                    28.11 ns      │ 67.17 ns      │ 30.84 ns      │ 31.09 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_floor                    99.79 ns      │ 1.085 µs      │ 102.9 ns      │ 107.6 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_floor_const              27.91 ns      │ 295.6 ns      │ 30.64 ns      │ 32.13 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_floor_reuse              29.67 ns      │ 101.7 ns      │ 34.36 ns      │ 35.36 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_reuse                    29.86 ns      │ 82.01 ns      │ 32.99 ns      │ 33.74 ns      │ 1000    │ 512000
│  │  ├─ std_div                           837.2 ns      │ 1.168 µs      │ 849.7 ns      │ 854.5 ns      │ 1000    │ 16000
│  │  ├─ std_div_const                     26.93 ns      │ 71.27 ns      │ 29.67 ns      │ 30.22 ns      │ 1000    │ 512000
│  │  ├─ std_div_reuse                     824.7 ns      │ 9.874 µs      │ 831 ns        │ 862 ns        │ 1000    │ 16000
│  │  ├─ unb_pow2_div                      106.8 ns      │ 1.138 µs      │ 109.9 ns      │ 111.6 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_ceil                 462.2 ns      │ 840.4 ns      │ 471.6 ns      │ 474.4 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_ceil_const           40.61 ns      │ 84.55 ns      │ 44.9 ns       │ 45.42 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_ceil_reuse           243.5 ns      │ 604.4 ns      │ 254.4 ns      │ 264.4 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_const                26.54 ns      │ 82.01 ns      │ 28.89 ns      │ 29.77 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_floor                106.8 ns      │ 212.2 ns      │ 109.9 ns      │ 111.6 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_floor_const          26.93 ns      │ 68.54 ns      │ 28.69 ns      │ 29.62 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_floor_reuse          29.28 ns      │ 219.7 ns      │ 32.99 ns      │ 33.71 ns      │ 1000    │ 512000
│  │  ╰─ unb_pow2_div_reuse                30.64 ns      │ 74.2 ns       │ 33.96 ns      │ 34.4 ns       │ 1000    │ 512000
│  ├─ u16                                                │               │               │               │         │
│  │  ├─ baseline_identity                 18.93 ns      │ 397.4 ns      │ 22.44 ns      │ 22.96 ns      │ 1000    │ 256000
│  │  ├─ pow2_div                          229.4 ns      │ 460.7 ns      │ 232.6 ns      │ 237.2 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_ceil                     468.5 ns      │ 874.7 ns      │ 474.7 ns      │ 478.9 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_ceil_const               79.47 ns      │ 160.7 ns      │ 83.38 ns      │ 83.49 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_ceil_reuse               235.7 ns      │ 1.398 µs      │ 240.4 ns      │ 242.3 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_const                    48.22 ns      │ 175.9 ns      │ 52.52 ns      │ 53.68 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_floor                    227.9 ns      │ 457.6 ns      │ 232.6 ns      │ 233.9 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_floor_const              50.18 ns      │ 161.8 ns      │ 54.47 ns      │ 55.7 ns       │ 1000    │ 256000
│  │  ├─ pow2_div_floor_reuse              52.91 ns      │ 168.1 ns      │ 59.55 ns      │ 61.03 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_reuse                    54.08 ns      │ 150.5 ns      │ 60.33 ns      │ 61.32 ns      │ 1000    │ 256000
│  │  ├─ std_div                           856 ns        │ 1.881 µs      │ 887.2 ns      │ 897.8 ns      │ 1000    │ 16000
│  │  ├─ std_div_const                     49.79 ns      │ 138.4 ns      │ 53.69 ns      │ 55.22 ns      │ 1000    │ 256000
│  │  ├─ std_div_reuse                     843.5 ns      │ 10.36 µs      │ 849.7 ns      │ 883.5 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_div                      229.4 ns      │ 2.26 µs       │ 235.7 ns      │ 248.4 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil                 462.2 ns      │ 4.409 µs      │ 468.5 ns      │ 489.6 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_ceil_const           79.47 ns      │ 1.168 µs      │ 84.16 ns      │ 90.56 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_ceil_reuse           238.8 ns      │ 2.271 µs      │ 241.9 ns      │ 256.5 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_const                49.79 ns      │ 313 ns        │ 54.86 ns      │ 56.2 ns       │ 1000    │ 256000
│  │  ├─ unb_pow2_div_floor                234.1 ns      │ 381 ns        │ 237.2 ns      │ 238 ns        │ 1000    │ 32000
│  │  ├─ unb_pow2_div_floor_const          49 ns         │ 142.3 ns      │ 54.08 ns      │ 56.49 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_floor_reuse          53.3 ns       │ 159.1 ns      │ 60.33 ns      │ 61.02 ns      │ 1000    │ 256000
│  │  ╰─ unb_pow2_div_reuse                53.69 ns      │ 137.2 ns      │ 60.33 ns      │ 61.57 ns      │ 1000    │ 256000
│  ├─ u32                                                │               │               │               │         │
│  │  ├─ baseline_identity                 40.8 ns       │ 196.6 ns      │ 45.1 ns       │ 47.17 ns      │ 1000    │ 256000
│  │  ├─ pow2_div                          295.1 ns      │ 2.712 µs      │ 301.3 ns      │ 312.4 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_ceil                     481 ns        │ 4.702 µs      │ 484.1 ns      │ 501.3 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_ceil_const               151.3 ns      │ 437.2 ns      │ 156 ns        │ 159.5 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_ceil_reuse               193.5 ns      │ 431 ns        │ 198.2 ns      │ 199.1 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_const                    92.75 ns      │ 202.1 ns      │ 106 ns        │ 105.4 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_floor                    287.2 ns      │ 532.6 ns      │ 291.9 ns      │ 295.1 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_floor_const              91.19 ns      │ 334.1 ns      │ 106.8 ns      │ 108.7 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_floor_reuse              104.4 ns      │ 251.3 ns      │ 118.5 ns      │ 120.1 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_reuse                    104.4 ns      │ 289.6 ns      │ 116.1 ns      │ 118.1 ns      │ 1000    │ 128000
│  │  ├─ std_div                           1.299 µs      │ 1.937 µs      │ 1.312 µs      │ 1.314 µs      │ 1000    │ 8000
│  │  ├─ std_div_const                     97.44 ns      │ 949 ns        │ 106.8 ns      │ 109.5 ns      │ 1000    │ 128000
│  │  ├─ std_div_reuse                     1.299 µs      │ 1.962 µs      │ 1.312 µs      │ 1.32 µs       │ 1000    │ 8000
│  │  ├─ unb_pow2_div                      296.6 ns      │ 456 ns        │ 301.3 ns      │ 302.2 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil                 477.9 ns      │ 4.624 µs      │ 484.1 ns      │ 507.2 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_ceil_const           152.9 ns      │ 2.162 µs      │ 159.1 ns      │ 170.8 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil_reuse           193.5 ns      │ 2.538 µs      │ 198.2 ns      │ 208.7 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_const                95.88 ns      │ 240.4 ns      │ 106.8 ns      │ 108.8 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_floor                295.1 ns      │ 2.632 µs      │ 301.3 ns      │ 313.3 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor_const          96.66 ns      │ 591.1 ns      │ 107.6 ns      │ 109.1 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_floor_reuse          105.2 ns      │ 246.6 ns      │ 116.9 ns      │ 118.4 ns      │ 1000    │ 128000
│  │  ╰─ unb_pow2_div_reuse                82.6 ns       │ 388.8 ns      │ 90.41 ns      │ 103.4 ns      │ 1000    │ 64000
│  ├─ u64                                                │               │               │               │         │
│  │  ├─ baseline_identity                 77.13 ns      │ 331.8 ns      │ 95.1 ns       │ 103.1 ns      │ 1000    │ 128000
│  │  ├─ pow2_div                          431 ns        │ 940.4 ns      │ 443.5 ns      │ 457.8 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_ceil                     574.7 ns      │ 990.4 ns      │ 587.2 ns      │ 592.5 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_ceil_const               263.8 ns      │ 623.2 ns      │ 296.6 ns      │ 301.2 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_ceil_reuse               309.1 ns      │ 634.1 ns      │ 341.9 ns      │ 345.5 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_const                    188.8 ns      │ 707.6 ns      │ 213.8 ns      │ 221.6 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_floor                    321.6 ns      │ 4.74 µs       │ 337.2 ns      │ 348.9 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_floor_const              188.8 ns      │ 529.4 ns      │ 202.9 ns      │ 208.4 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_floor_reuse              191.9 ns      │ 1.049 µs      │ 202.9 ns      │ 221.9 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_reuse                    188.8 ns      │ 1.315 µs      │ 207.6 ns      │ 212.2 ns      │ 1000    │ 64000
│  │  ├─ std_div                           1.637 µs      │ 2.887 µs      │ 1.662 µs      │ 1.672 µs      │ 1000    │ 8000
│  │  ├─ std_div_const                     193.5 ns      │ 529.4 ns      │ 221.6 ns      │ 223.2 ns      │ 1000    │ 64000
│  │  ├─ std_div_reuse                     1.512 µs      │ 20.13 µs      │ 1.524 µs      │ 1.585 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div                      418.5 ns      │ 4.509 µs      │ 434.1 ns      │ 447.3 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_ceil                 584.1 ns      │ 1.024 µs      │ 593.5 ns      │ 601.9 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_ceil_const           241.9 ns      │ 616.9 ns      │ 268.5 ns      │ 282.3 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil_reuse           287.2 ns      │ 431 ns        │ 296.6 ns      │ 299 ns        │ 1000    │ 32000
│  │  ├─ unb_pow2_div_const                187.2 ns      │ 757.6 ns      │ 210.7 ns      │ 214.9 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor                421.6 ns      │ 762.2 ns      │ 434.1 ns      │ 437.5 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_floor_const          191.9 ns      │ 637.2 ns      │ 212.2 ns      │ 217.9 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor_reuse          206 ns        │ 604.4 ns      │ 231 ns        │ 233.9 ns      │ 1000    │ 64000
│  │  ╰─ unb_pow2_div_reuse                199.7 ns      │ 491.9 ns      │ 218.5 ns      │ 223.7 ns      │ 1000    │ 64000
│  ╰─ u128                                               │               │               │               │         │
│     ├─ baseline_identity                 237.2 ns      │ 1.063 µs      │ 259.1 ns      │ 269.1 ns      │ 1000    │ 64000
│     ├─ pow2_div                          943.5 ns      │ 6.168 µs      │ 962.2 ns      │ 995.7 ns      │ 1000    │ 16000
│     ├─ pow2_div_ceil                     2.012 µs      │ 5.312 µs      │ 2.037 µs      │ 2.053 µs      │ 1000    │ 8000
│     ├─ pow2_div_ceil_const               1.068 µs      │ 1.999 µs      │ 1.087 µs      │ 1.099 µs      │ 1000    │ 16000
│     ├─ pow2_div_ceil_reuse               1.899 µs      │ 2.562 µs      │ 1.949 µs      │ 1.957 µs      │ 1000    │ 8000
│     ├─ pow2_div_const                    856 ns        │ 7.918 µs      │ 881 ns        │ 894.8 ns      │ 1000    │ 16000
│     ├─ pow2_div_floor                    943.5 ns      │ 7.562 µs      │ 962.2 ns      │ 993.5 ns      │ 1000    │ 16000
│     ├─ pow2_div_floor_const              868.5 ns      │ 1.549 µs      │ 887.2 ns      │ 898.9 ns      │ 1000    │ 16000
│     ├─ pow2_div_floor_reuse              1.024 µs      │ 3.618 µs      │ 1.049 µs      │ 1.069 µs      │ 1000    │ 16000
│     ├─ pow2_div_reuse                    1.049 µs      │ 5.499 µs      │ 1.074 µs      │ 1.114 µs      │ 1000    │ 8000
│     ├─ std_div                           5.799 µs      │ 13.94 µs      │ 5.899 µs      │ 5.997 µs      │ 1000    │ 2000
│     ├─ std_div_const                     868.5 ns      │ 2.199 µs      │ 893.5 ns      │ 923.5 ns      │ 1000    │ 16000
│     ├─ std_div_reuse                     4.324 µs      │ 12.87 µs      │ 4.374 µs      │ 4.419 µs      │ 1000    │ 4000
│     ├─ unb_pow2_div                      956 ns        │ 2.162 µs      │ 968.5 ns      │ 1.006 µs      │ 1000    │ 16000
│     ├─ unb_pow2_div_ceil                 1.974 µs      │ 5.987 µs      │ 2.012 µs      │ 2.033 µs      │ 1000    │ 8000
│     ├─ unb_pow2_div_ceil_const           1.049 µs      │ 2.356 µs      │ 1.068 µs      │ 1.081 µs      │ 1000    │ 16000
│     ├─ unb_pow2_div_ceil_reuse           1.899 µs      │ 2.674 µs      │ 1.912 µs      │ 1.919 µs      │ 1000    │ 8000
│     ├─ unb_pow2_div_const                849.7 ns      │ 1.781 µs      │ 868.5 ns      │ 887.6 ns      │ 1000    │ 16000
│     ├─ unb_pow2_div_floor                956 ns        │ 1.587 µs      │ 974.7 ns      │ 983 ns        │ 1000    │ 16000
│     ├─ unb_pow2_div_floor_const          856 ns        │ 1.306 µs      │ 887.2 ns      │ 892.4 ns      │ 1000    │ 16000
│     ├─ unb_pow2_div_floor_reuse          1.031 µs      │ 1.899 µs      │ 1.043 µs      │ 1.05 µs       │ 1000    │ 16000
│     ╰─ unb_pow2_div_reuse                1.024 µs      │ 3.206 µs      │ 1.043 µs      │ 1.046 µs      │ 1000    │ 16000
├─ modulo                                                │               │               │               │         │
│  ├─ i8                                                 │               │               │               │         │
│  │  ├─ baseline_identity                 10.82 ns      │ 36.02 ns      │ 14.24 ns      │ 14.37 ns      │ 1000    │ 1024000
│  │  ├─ pow2_is_multiple_of               77.13 ns      │ 169.3 ns      │ 78.69 ns      │ 81.23 ns      │ 1000    │ 128000
│  │  ├─ pow2_is_multiple_of_const         28.89 ns      │ 89.24 ns      │ 32.01 ns      │ 33.07 ns      │ 1000    │ 512000
│  │  ├─ pow2_is_multiple_of_reuse         68.93 ns      │ 172.8 ns      │ 70.49 ns      │ 72.16 ns      │ 1000    │ 256000
│  │  ├─ pow2_mod_floor                    96.66 ns      │ 463 ns        │ 100.5 ns      │ 104.2 ns      │ 1000    │ 128000
│  │  ├─ pow2_mod_floor_const              25.57 ns      │ 86.5 ns       │ 27.91 ns      │ 28.17 ns      │ 1000    │ 512000
│  │  ├─ pow2_mod_floor_reuse              28.11 ns      │ 92.95 ns      │ 29.86 ns      │ 31.18 ns      │ 1000    │ 512000
│  │  ├─ std_mod                           856 ns        │ 11.21 µs      │ 893.5 ns      │ 941 ns        │ 1000    │ 16000
│  │  ├─ std_mod_const                     37.87 ns      │ 104.4 ns      │ 43.14 ns      │ 43.3 ns       │ 1000    │ 512000
│  │  ├─ std_mod_reuse                     824.7 ns      │ 1.618 µs      │ 831 ns        │ 837.6 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_is_multiple_of           76.35 ns      │ 132.6 ns      │ 79.47 ns      │ 80.49 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_is_multiple_of_const     28.69 ns      │ 72.83 ns      │ 32.4 ns       │ 32.68 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_is_multiple_of_reuse     68.93 ns      │ 390 ns        │ 70.1 ns       │ 74.81 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_mod_floor                101.3 ns      │ 1.176 µs      │ 102.9 ns      │ 105.1 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_mod_floor_const          26.54 ns      │ 233.1 ns      │ 27.71 ns      │ 28.78 ns      │ 1000    │ 512000
│  │  ╰─ unb_pow2_mod_floor_reuse          27.71 ns      │ 56.82 ns      │ 28.89 ns      │ 30.11 ns      │ 1000    │ 512000
│  ├─ i16                                                │               │               │               │         │
│  │  ├─ baseline_identity                 24.79 ns      │ 74.39 ns      │ 28.11 ns      │ 30.08 ns      │ 1000    │ 512000
│  │  ├─ pow2_is_multiple_of               179.4 ns      │ 379.4 ns      │ 182.6 ns      │ 187.2 ns      │ 1000    │ 64000
│  │  ├─ pow2_is_multiple_of_const         37.68 ns      │ 109.5 ns      │ 43.54 ns      │ 44.53 ns      │ 1000    │ 256000
│  │  ├─ pow2_is_multiple_of_reuse         182.6 ns      │ 479.4 ns      │ 184.1 ns      │ 189.9 ns      │ 1000    │ 64000
│  │  ├─ pow2_mod_floor                    187.2 ns      │ 493.5 ns      │ 191.9 ns      │ 197.2 ns      │ 1000    │ 64000
│  │  ├─ pow2_mod_floor_const              51.74 ns      │ 152.1 ns      │ 57.6 ns       │ 58.79 ns      │ 1000    │ 256000
│  │  ├─ pow2_mod_floor_reuse              54.47 ns      │ 126.7 ns      │ 61.11 ns      │ 61.91 ns      │ 1000    │ 256000
│  │  ├─ std_mod                           874.7 ns      │ 1.843 µs      │ 893.5 ns      │ 902.6 ns      │ 1000    │ 16000
│  │  ├─ std_mod_const                     67.36 ns      │ 157.9 ns      │ 72.83 ns      │ 74.49 ns      │ 1000    │ 256000
│  │  ├─ std_mod_reuse                     2.474 µs      │ 3.062 µs      │ 2.499 µs      │ 2.505 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_is_multiple_of           181 ns        │ 254.4 ns      │ 184.1 ns      │ 185.5 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_is_multiple_of_const     40.02 ns      │ 95.88 ns      │ 44.32 ns      │ 44.64 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_is_multiple_of_reuse     181 ns        │ 248.2 ns      │ 182.6 ns      │ 183.5 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_mod_floor                190.4 ns      │ 513.8 ns      │ 193.5 ns      │ 194.5 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_mod_floor_const          54.47 ns      │ 158.3 ns      │ 56.82 ns      │ 58.01 ns      │ 1000    │ 256000
│  │  ╰─ unb_pow2_mod_floor_reuse          54.86 ns      │ 136.8 ns      │ 63.85 ns      │ 65.41 ns      │ 1000    │ 256000
│  ├─ i32                                                │               │               │               │         │
│  │  ├─ baseline_identity                 27.91 ns      │ 174.7 ns      │ 29.47 ns      │ 30.9 ns       │ 1000    │ 64000
│  │  ├─ pow2_is_multiple_of               465.4 ns      │ 602.9 ns      │ 468.5 ns      │ 470.6 ns      │ 1000    │ 32000
│  │  ├─ pow2_is_multiple_of_const         95.88 ns      │ 274 ns        │ 106.8 ns      │ 107.5 ns      │ 1000    │ 128000
│  │  ├─ pow2_is_multiple_of_reuse         434.1 ns      │ 624.7 ns      │ 437.2 ns      │ 437.9 ns      │ 1000    │ 32000
│  │  ├─ pow2_mod_floor                    270.1 ns      │ 404.4 ns      │ 274.7 ns      │ 275.4 ns      │ 1000    │ 64000
│  │  ├─ pow2_mod_floor_const              97.44 ns      │ 338.8 ns      │ 106.8 ns      │ 109.6 ns      │ 1000    │ 128000
│  │  ├─ pow2_mod_floor_reuse              99 ns         │ 509.1 ns      │ 114.6 ns      │ 117.8 ns      │ 1000    │ 128000
│  │  ├─ std_mod                           1.312 µs      │ 3.412 µs      │ 1.324 µs      │ 1.349 µs      │ 1000    │ 8000
│  │  ├─ std_mod_const                     137.2 ns      │ 326.3 ns      │ 151.7 ns      │ 156.8 ns      │ 1000    │ 128000
│  │  ├─ std_mod_reuse                     1.299 µs      │ 3.462 µs      │ 1.324 µs      │ 1.364 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_is_multiple_of           462.2 ns      │ 1.165 µs      │ 471.6 ns      │ 488.5 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_is_multiple_of_const     85.72 ns      │ 192.7 ns      │ 99 ns         │ 99.22 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_is_multiple_of_reuse     424.7 ns      │ 718.5 ns      │ 431 ns        │ 432.9 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_mod_floor                277.9 ns      │ 421.6 ns      │ 284.1 ns      │ 285.6 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_mod_floor_const          93.54 ns      │ 365.4 ns      │ 106 ns        │ 107.7 ns      │ 1000    │ 128000
│  │  ╰─ unb_pow2_mod_floor_reuse          98.22 ns      │ 386.5 ns      │ 107.6 ns      │ 110.7 ns      │ 1000    │ 128000
│  ├─ i64                                                │               │               │               │         │
│  │  ├─ baseline_identity                 77.13 ns      │ 244.3 ns      │ 98.22 ns      │ 101.4 ns      │ 1000    │ 128000
│  │  ├─ pow2_is_multiple_of               337.2 ns      │ 502.9 ns      │ 359.1 ns      │ 359.5 ns      │ 1000    │ 32000
│  │  ├─ pow2_is_multiple_of_const         260.7 ns      │ 724.7 ns      │ 266.9 ns      │ 301.8 ns      │ 1000    │ 64000
│  │  ├─ pow2_is_multiple_of_reuse         318.5 ns      │ 737.2 ns      │ 321.6 ns      │ 329 ns        │ 1000    │ 32000
│  │  ├─ pow2_mod_floor                    449.7 ns      │ 1.184 µs      │ 462.2 ns      │ 473.7 ns      │ 1000    │ 32000
│  │  ├─ pow2_mod_floor_const              201.3 ns      │ 440.4 ns      │ 218.5 ns      │ 221.7 ns      │ 1000    │ 64000
│  │  ├─ pow2_mod_floor_reuse              191.9 ns      │ 1.72 µs       │ 207.6 ns      │ 216.6 ns      │ 1000    │ 64000
│  │  ├─ std_mod                           1.674 µs      │ 20.09 µs      │ 1.699 µs      │ 1.77 µs       │ 1000    │ 8000
│  │  ├─ std_mod_const                     273.2 ns      │ 2.384 µs      │ 299.7 ns      │ 307.6 ns      │ 1000    │ 64000
│  │  ├─ std_mod_reuse                     1.499 µs      │ 2.037 µs      │ 1.512 µs      │ 1.514 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_is_multiple_of           337.2 ns      │ 512.2 ns      │ 352.9 ns      │ 354.6 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_is_multiple_of_const     260.7 ns      │ 615.4 ns      │ 265.4 ns      │ 267.7 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_is_multiple_of_reuse     315.4 ns      │ 612.2 ns      │ 318.5 ns      │ 320.4 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_mod_floor                484.1 ns      │ 1.299 µs      │ 499.7 ns      │ 505.9 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_mod_floor_const          187.2 ns      │ 579.4 ns      │ 213 ns        │ 217.7 ns      │ 1000    │ 64000
│  │  ╰─ unb_pow2_mod_floor_reuse          190.4 ns      │ 2.476 µs      │ 223.2 ns      │ 231 ns        │ 1000    │ 64000
│  ├─ i128                                               │               │               │               │         │
│  │  ├─ baseline_identity                 231 ns        │ 502.9 ns      │ 243.5 ns      │ 250.3 ns      │ 1000    │ 64000
│  │  ├─ pow2_is_multiple_of               581 ns        │ 1.074 µs      │ 596.6 ns      │ 621.4 ns      │ 1000    │ 32000
│  │  ├─ pow2_is_multiple_of_const         284.1 ns      │ 1.793 µs      │ 302.9 ns      │ 308.6 ns      │ 1000    │ 64000
│  │  ├─ pow2_is_multiple_of_reuse         549.7 ns      │ 1.081 µs      │ 571.6 ns      │ 575.6 ns      │ 1000    │ 32000
│  │  ├─ pow2_mod_floor                    937.2 ns      │ 1.612 µs      │ 956 ns        │ 964 ns        │ 1000    │ 16000
│  │  ├─ pow2_mod_floor_const              531 ns        │ 1.512 µs      │ 568.5 ns      │ 578.5 ns      │ 1000    │ 32000
│  │  ├─ pow2_mod_floor_reuse              609.1 ns      │ 2.565 µs      │ 649.7 ns      │ 675 ns        │ 1000    │ 32000
│  │  ├─ std_mod                           9.299 µs      │ 22.69 µs      │ 9.399 µs      │ 9.567 µs      │ 1000    │ 1000
│  │  ├─ std_mod_const                     862.2 ns      │ 1.656 µs      │ 949.7 ns      │ 953.4 ns      │ 1000    │ 16000
│  │  ├─ std_mod_reuse                     8.299 µs      │ 12.99 µs      │ 8.399 µs      │ 8.433 µs      │ 1000    │ 2000
│  │  ├─ unb_pow2_is_multiple_of           581 ns        │ 2.018 µs      │ 596.6 ns      │ 605.3 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_is_multiple_of_const     281 ns        │ 538.8 ns      │ 293.5 ns      │ 300.5 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_is_multiple_of_reuse     556 ns        │ 993.5 ns      │ 568.5 ns      │ 573.4 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_mod_floor                1.081 µs      │ 2.556 µs      │ 1.099 µs      │ 1.116 µs      │ 1000    │ 16000
│  │  ├─ unb_pow2_mod_floor_const          524.7 ns      │ 934.1 ns      │ 562.2 ns      │ 570.9 ns      │ 1000    │ 32000
│  │  ╰─ unb_pow2_mod_floor_reuse          599.7 ns      │ 1.187 µs      │ 637.2 ns      │ 651.2 ns      │ 1000    │ 32000
│  ├─ u8                                                 │               │               │               │         │
│  │  ├─ baseline_identity                 10.92 ns      │ 62.38 ns      │ 13.36 ns      │ 14.6 ns       │ 1000    │ 1024000
│  │  ├─ pow2_is_multiple_of               76.35 ns      │ 156.8 ns      │ 78.69 ns      │ 79.16 ns      │ 1000    │ 128000
│  │  ├─ pow2_is_multiple_of_const         29.28 ns      │ 87.09 ns      │ 31.82 ns      │ 32.36 ns      │ 1000    │ 512000
│  │  ├─ pow2_is_multiple_of_reuse         69.32 ns      │ 94.71 ns      │ 71.27 ns      │ 71.73 ns      │ 1000    │ 256000
│  │  ├─ pow2_mod_floor                    95.1 ns       │ 171.6 ns      │ 99.79 ns      │ 99.85 ns      │ 1000    │ 128000
│  │  ├─ pow2_mod_floor_const              26.15 ns      │ 61.7 ns       │ 27.13 ns      │ 27.93 ns      │ 1000    │ 512000
│  │  ├─ pow2_mod_floor_reuse              28.11 ns      │ 88.65 ns      │ 29.67 ns      │ 31.48 ns      │ 1000    │ 512000
│  │  ├─ std_mod                           856 ns        │ 2.131 µs      │ 868.5 ns      │ 877.7 ns      │ 1000    │ 16000
│  │  ├─ std_mod_const                     27.13 ns      │ 52.91 ns      │ 28.11 ns      │ 28.41 ns      │ 1000    │ 512000
│  │  ├─ std_mod_reuse                     824.7 ns      │ 1.099 µs      │ 824.7 ns      │ 832 ns        │ 1000    │ 16000
│  │  ├─ unb_pow2_is_multiple_of           75.57 ns      │ 126.3 ns      │ 81.04 ns      │ 81.34 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_is_multiple_of_const     30.06 ns      │ 82.01 ns      │ 32.99 ns      │ 33.39 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_is_multiple_of_reuse     69.32 ns      │ 93.14 ns      │ 71.27 ns      │ 71.91 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_mod_floor                101.3 ns      │ 154.4 ns      │ 105.2 ns      │ 105.4 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_mod_floor_const          26.35 ns      │ 71.86 ns      │ 28.89 ns      │ 29.43 ns      │ 1000    │ 512000
│  │  ╰─ unb_pow2_mod_floor_reuse          27.71 ns      │ 98.42 ns      │ 29.47 ns      │ 30.53 ns      │ 1000    │ 512000
│  ├─ u16                                                │               │               │               │         │
│  │  ├─ baseline_identity                 24.98 ns      │ 70.68 ns      │ 25.96 ns      │ 27.1 ns       │ 1000    │ 512000
│  │  ├─ pow2_is_multiple_of               179.4 ns      │ 246.6 ns      │ 181 ns        │ 182.6 ns      │ 1000    │ 64000
│  │  ├─ pow2_is_multiple_of_const         40.02 ns      │ 110.7 ns      │ 43.93 ns      │ 44.64 ns      │ 1000    │ 256000
│  │  ├─ pow2_is_multiple_of_reuse         179.4 ns      │ 331 ns        │ 182.6 ns      │ 183.1 ns      │ 1000    │ 64000
│  │  ├─ pow2_mod_floor                    185.7 ns      │ 2.298 µs      │ 190.4 ns      │ 194.7 ns      │ 1000    │ 64000
│  │  ├─ pow2_mod_floor_const              51.74 ns      │ 629 ns        │ 57.99 ns      │ 60.83 ns      │ 1000    │ 256000
│  │  ├─ pow2_mod_floor_reuse              54.47 ns      │ 470.8 ns      │ 60.72 ns      │ 63.28 ns      │ 1000    │ 256000
│  │  ├─ std_mod                           849.7 ns      │ 1.137 µs      │ 862.2 ns      │ 870.9 ns      │ 1000    │ 16000
│  │  ├─ std_mod_const                     52.91 ns      │ 134.1 ns      │ 56.82 ns      │ 60.8 ns       │ 1000    │ 256000
│  │  ├─ std_mod_reuse                     856 ns        │ 1.718 µs      │ 862.2 ns      │ 890.8 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_is_multiple_of           181 ns        │ 362.2 ns      │ 184.1 ns      │ 185.1 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_is_multiple_of_const     39.63 ns      │ 571.2 ns      │ 42.75 ns      │ 45.59 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_is_multiple_of_reuse     182.6 ns      │ 2.238 µs      │ 184.1 ns      │ 191.7 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_mod_floor                190.4 ns      │ 1.79 µs       │ 195.1 ns      │ 202.2 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_mod_floor_const          51.74 ns      │ 493.1 ns      │ 56.43 ns      │ 57.5 ns       │ 1000    │ 256000
│  │  ╰─ unb_pow2_mod_floor_reuse          54.47 ns      │ 466.9 ns      │ 62.29 ns      │ 64.65 ns      │ 1000    │ 256000
│  ├─ u32                                                │               │               │               │         │
│  │  ├─ baseline_identity                 41.58 ns      │ 143.1 ns      │ 46.27 ns      │ 50.17 ns      │ 1000    │ 256000
│  │  ├─ pow2_is_multiple_of               459.1 ns      │ 843.5 ns      │ 465.4 ns      │ 473.2 ns      │ 1000    │ 32000
│  │  ├─ pow2_is_multiple_of_const         99 ns         │ 222.4 ns      │ 107.6 ns      │ 110.1 ns      │ 1000    │ 128000
│  │  ├─ pow2_is_multiple_of_reuse         424.7 ns      │ 743.5 ns      │ 431 ns        │ 440.8 ns      │ 1000    │ 32000
│  │  ├─ pow2_mod_floor                    268.5 ns      │ 1.118 µs      │ 271.6 ns      │ 280.1 ns      │ 1000    │ 32000
│  │  ├─ pow2_mod_floor_const              98.22 ns      │ 267.7 ns      │ 107.6 ns      │ 109.5 ns      │ 1000    │ 128000
│  │  ├─ pow2_mod_floor_reuse              99.79 ns      │ 434.1 ns      │ 113.8 ns      │ 120.1 ns      │ 1000    │ 128000
│  │  ├─ std_mod                           1.362 µs      │ 2.162 µs      │ 1.387 µs      │ 1.389 µs      │ 1000    │ 8000
│  │  ├─ std_mod_const                     92.75 ns      │ 236.5 ns      │ 108.3 ns      │ 108.3 ns      │ 1000    │ 128000
│  │  ├─ std_mod_reuse                     1.249 µs      │ 2.437 µs      │ 1.262 µs      │ 1.265 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_is_multiple_of           452.9 ns      │ 593.5 ns      │ 456 ns        │ 460.5 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_is_multiple_of_const     95.88 ns      │ 311.5 ns      │ 106 ns        │ 108.2 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_is_multiple_of_reuse     424.7 ns      │ 571.6 ns      │ 431 ns        │ 435.2 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_mod_floor                279.4 ns      │ 693.5 ns      │ 287.2 ns      │ 294 ns        │ 1000    │ 64000
│  │  ├─ unb_pow2_mod_floor_const          96.66 ns      │ 226.3 ns      │ 109.9 ns      │ 112 ns        │ 1000    │ 128000
│  │  ╰─ unb_pow2_mod_floor_reuse          99 ns         │ 291.1 ns      │ 116.9 ns      │ 119 ns        │ 1000    │ 128000
│  ├─ u64                                                │               │               │               │         │
│  │  ├─ baseline_identity                 76.35 ns      │ 897.4 ns      │ 92.75 ns      │ 96.89 ns      │ 1000    │ 128000
│  │  ├─ pow2_is_multiple_of               340.4 ns      │ 752.9 ns      │ 359.1 ns      │ 367.2 ns      │ 1000    │ 32000
│  │  ├─ pow2_is_multiple_of_const         260.7 ns      │ 612.2 ns      │ 266.9 ns      │ 275.5 ns      │ 1000    │ 64000
│  │  ├─ pow2_is_multiple_of_reuse         315.4 ns      │ 777.9 ns      │ 321.6 ns      │ 330.4 ns      │ 1000    │ 32000
│  │  ├─ pow2_mod_floor                    449.7 ns      │ 1.331 µs      │ 462.2 ns      │ 479.2 ns      │ 1000    │ 16000
│  │  ├─ pow2_mod_floor_const              201.3 ns      │ 613.8 ns      │ 220.1 ns      │ 228.8 ns      │ 1000    │ 64000
│  │  ├─ pow2_mod_floor_reuse              191.9 ns      │ 543.5 ns      │ 213.8 ns      │ 218.3 ns      │ 1000    │ 64000
│  │  ├─ std_mod                           1.674 µs      │ 4.899 µs      │ 1.712 µs      │ 1.724 µs      │ 1000    │ 8000
│  │  ├─ std_mod_const                     190.4 ns      │ 771.6 ns      │ 220.1 ns      │ 223.4 ns      │ 1000    │ 64000
│  │  ├─ std_mod_reuse                     1.512 µs      │ 3.012 µs      │ 1.524 µs      │ 1.535 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_is_multiple_of           340.4 ns      │ 952.9 ns      │ 352.9 ns      │ 366.4 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_is_multiple_of_const     260.7 ns      │ 638.8 ns      │ 265.4 ns      │ 274.5 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_is_multiple_of_reuse     315.4 ns      │ 856 ns        │ 321.6 ns      │ 334.6 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_mod_floor                487.2 ns      │ 956 ns        │ 506 ns        │ 517.3 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_mod_floor_const          193.5 ns      │ 473.2 ns      │ 221.6 ns      │ 226.5 ns      │ 1000    │ 64000
│  │  ╰─ unb_pow2_mod_floor_reuse          193.5 ns      │ 579.4 ns      │ 215.4 ns      │ 219.4 ns      │ 1000    │ 64000
│  ╰─ u128                                               │               │               │               │         │
│     ├─ baseline_identity                 234.1 ns      │ 543.5 ns      │ 256 ns        │ 262.1 ns      │ 1000    │ 64000
│     ├─ pow2_is_multiple_of               581 ns        │ 1.227 µs      │ 596.6 ns      │ 613.1 ns      │ 1000    │ 32000
│     ├─ pow2_is_multiple_of_const         282.6 ns      │ 1.507 µs      │ 293.5 ns      │ 303.9 ns      │ 1000    │ 64000
│     ├─ pow2_is_multiple_of_reuse         552.9 ns      │ 1.152 µs      │ 568.5 ns      │ 590.8 ns      │ 1000    │ 32000
│     ├─ pow2_mod_floor                    949.7 ns      │ 1.974 µs      │ 962.2 ns      │ 992.9 ns      │ 1000    │ 16000
│     ├─ pow2_mod_floor_const              509.1 ns      │ 1.29 µs       │ 571.6 ns      │ 585.8 ns      │ 1000    │ 32000
│     ├─ pow2_mod_floor_reuse              596.6 ns      │ 1.584 µs      │ 637.2 ns      │ 657.3 ns      │ 1000    │ 32000
│     ├─ std_mod                           4.049 µs      │ 8.574 µs      │ 4.174 µs      │ 4.191 µs      │ 1000    │ 4000
│     ├─ std_mod_const                     481 ns        │ 1.193 µs      │ 512.2 ns      │ 514.8 ns      │ 1000    │ 16000
│     ├─ std_mod_reuse                     4.299 µs      │ 21.54 µs      │ 4.349 µs      │ 4.387 µs      │ 1000    │ 4000
│     ├─ unb_pow2_is_multiple_of           584.1 ns      │ 962.2 ns      │ 593.5 ns      │ 600.2 ns      │ 1000    │ 32000
│     ├─ unb_pow2_is_multiple_of_const     284.1 ns      │ 612.2 ns      │ 298.2 ns      │ 312.9 ns      │ 1000    │ 64000
│     ├─ unb_pow2_is_multiple_of_reuse     552.9 ns      │ 1.346 µs      │ 571.6 ns      │ 587.7 ns      │ 1000    │ 32000
│     ├─ unb_pow2_mod_floor                1.074 µs      │ 3.199 µs      │ 1.093 µs      │ 1.136 µs      │ 1000    │ 16000
│     ├─ unb_pow2_mod_floor_const          524.7 ns      │ 1.343 µs      │ 574.7 ns      │ 582.8 ns      │ 1000    │ 32000
│     ╰─ unb_pow2_mod_floor_reuse          602.9 ns      │ 1.262 µs      │ 649.7 ns      │ 680.5 ns      │ 1000    │ 32000
├─ mul                                                   │               │               │               │         │
│  ├─ i8                                                 │               │               │               │         │
│  │  ├─ baseline_identity                 10.82 ns      │ 62.48 ns      │ 13.46 ns      │ 15.05 ns      │ 1000    │ 1024000
│  │  ├─ pow2_mul                          95.1 ns       │ 152.1 ns      │ 99 ns         │ 99.18 ns      │ 1000    │ 128000
│  │  ├─ pow2_mul_const                    27.91 ns      │ 72.83 ns      │ 30.64 ns      │ 30.99 ns      │ 1000    │ 512000
│  │  ├─ pow2_mul_reuse                    32.6 ns       │ 86.7 ns       │ 36.89 ns      │ 37.29 ns      │ 1000    │ 512000
│  │  ├─ std_mul                           53.69 ns      │ 106.8 ns      │ 61.5 ns       │ 61.76 ns      │ 1000    │ 256000
│  │  ├─ std_mul_const                     28.89 ns      │ 123.6 ns      │ 31.82 ns      │ 32.31 ns      │ 1000    │ 512000
│  │  ├─ std_mul_reuse                     35.72 ns      │ 80.64 ns      │ 38.07 ns      │ 39.6 ns       │ 1000    │ 256000
│  │  ├─ unb_pow2_mul                      98.22 ns      │ 194.3 ns      │ 102.9 ns      │ 103 ns        │ 1000    │ 128000
│  │  ├─ unb_pow2_mul_const                26.93 ns      │ 257.7 ns      │ 30.25 ns      │ 30.98 ns      │ 1000    │ 512000
│  │  ╰─ unb_pow2_mul_reuse                28.5 ns       │ 105 ns        │ 32.99 ns      │ 35.28 ns      │ 1000    │ 512000
│  ├─ i16                                                │               │               │               │         │
│  │  ├─ baseline_identity                 24.59 ns      │ 97.83 ns      │ 25.76 ns      │ 27.47 ns      │ 1000    │ 512000
│  │  ├─ pow2_mul                          181 ns        │ 252.9 ns      │ 184.1 ns      │ 185 ns        │ 1000    │ 64000
│  │  ├─ pow2_mul_const                    50.96 ns      │ 133.3 ns      │ 55.25 ns      │ 57.09 ns      │ 1000    │ 256000
│  │  ├─ pow2_mul_reuse                    49.79 ns      │ 165 ns        │ 54.86 ns      │ 58.04 ns      │ 1000    │ 256000
│  │  ├─ std_mul                           66.19 ns      │ 154.4 ns      │ 70.88 ns      │ 72.8 ns       │ 1000    │ 128000
│  │  ├─ std_mul_const                     52.13 ns      │ 141.5 ns      │ 57.21 ns      │ 59.18 ns      │ 1000    │ 256000
│  │  ├─ std_mul_reuse                     54.47 ns      │ 148.2 ns      │ 59.16 ns      │ 60.7 ns       │ 1000    │ 256000
│  │  ├─ unb_pow2_mul                      182.6 ns      │ 395.1 ns      │ 187.2 ns      │ 188.3 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_mul_const                50.96 ns      │ 118.9 ns      │ 56.82 ns      │ 57.43 ns      │ 1000    │ 256000
│  │  ╰─ unb_pow2_mul_reuse                54.86 ns      │ 132.6 ns      │ 62.29 ns      │ 63.57 ns      │ 1000    │ 256000
│  ├─ i32                                                │               │               │               │         │
│  │  ├─ baseline_identity                 41.58 ns      │ 154 ns        │ 45.49 ns      │ 49.34 ns      │ 1000    │ 256000
│  │  ├─ pow2_mul                          276.3 ns      │ 377.9 ns      │ 285.7 ns      │ 287.5 ns      │ 1000    │ 64000
│  │  ├─ pow2_mul_const                    96.66 ns      │ 184.9 ns      │ 110.7 ns      │ 110.8 ns      │ 1000    │ 128000
│  │  ├─ pow2_mul_reuse                    102.9 ns      │ 215.4 ns      │ 114.6 ns      │ 116.1 ns      │ 1000    │ 128000
│  │  ├─ std_mul                           190.4 ns      │ 356 ns        │ 196.6 ns      │ 200.2 ns      │ 1000    │ 64000
│  │  ├─ std_mul_const                     93.54 ns      │ 241.1 ns      │ 111.5 ns      │ 118.8 ns      │ 1000    │ 128000
│  │  ├─ std_mul_reuse                     146.6 ns      │ 374 ns        │ 164.6 ns      │ 165.7 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_mul                      266.9 ns      │ 429.4 ns      │ 273.2 ns      │ 275.9 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_mul_const                99.79 ns      │ 463.8 ns      │ 110.7 ns      │ 113.1 ns      │ 1000    │ 128000
│  │  ╰─ unb_pow2_mul_reuse                101.3 ns      │ 277.9 ns      │ 113.8 ns      │ 115.6 ns      │ 1000    │ 128000
│  ├─ i64                                                │               │               │               │         │
│  │  ├─ baseline_identity                 77.13 ns      │ 243.5 ns      │ 103.6 ns      │ 107.3 ns      │ 1000    │ 128000
│  │  ├─ pow2_mul                          321.6 ns      │ 524.7 ns      │ 331 ns        │ 335.2 ns      │ 1000    │ 32000
│  │  ├─ pow2_mul_const                    188.8 ns      │ 1.166 µs      │ 213.8 ns      │ 217.4 ns      │ 1000    │ 64000
│  │  ├─ pow2_mul_reuse                    204.4 ns      │ 632.6 ns      │ 219.3 ns      │ 227.8 ns      │ 1000    │ 64000
│  │  ├─ std_mul                           446.6 ns      │ 799.7 ns      │ 459.1 ns      │ 462.7 ns      │ 1000    │ 32000
│  │  ├─ std_mul_const                     198.2 ns      │ 456 ns        │ 221.6 ns      │ 224.6 ns      │ 1000    │ 64000
│  │  ├─ std_mul_reuse                     290.4 ns      │ 487.2 ns      │ 306 ns        │ 308.6 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_mul                      415.4 ns      │ 1.081 µs      │ 427.9 ns      │ 432.8 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_mul_const                191.9 ns      │ 715.4 ns      │ 215.4 ns      │ 219.3 ns      │ 1000    │ 64000
│  │  ╰─ unb_pow2_mul_reuse                187.2 ns      │ 532.6 ns      │ 216.9 ns      │ 229 ns        │ 1000    │ 64000
│  ├─ i128                                               │               │               │               │         │
│  │  ├─ baseline_identity                 235.7 ns      │ 682.6 ns      │ 256 ns        │ 271 ns        │ 1000    │ 64000
│  │  ├─ pow2_mul                          937.2 ns      │ 9.349 µs      │ 968.5 ns      │ 989.2 ns      │ 1000    │ 16000
│  │  ├─ pow2_mul_const                    862.2 ns      │ 8.899 µs      │ 881 ns        │ 922.6 ns      │ 1000    │ 16000
│  │  ├─ pow2_mul_reuse                    1.037 µs      │ 7.312 µs      │ 1.049 µs      │ 1.083 µs      │ 1000    │ 16000
│  │  ├─ std_mul                           1.287 µs      │ 24.41 µs      │ 1.312 µs      │ 1.423 µs      │ 1000    │ 8000
│  │  ├─ std_mul_const                     856 ns        │ 9.162 µs      │ 874.7 ns      │ 903.5 ns      │ 1000    │ 16000
│  │  ├─ std_mul_reuse                     831 ns        │ 9.131 µs      │ 849.7 ns      │ 888.4 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_mul                      956 ns        │ 7.374 µs      │ 974.7 ns      │ 996 ns        │ 1000    │ 16000
│  │  ├─ unb_pow2_mul_const                862.2 ns      │ 9.287 µs      │ 874.7 ns      │ 926.4 ns      │ 1000    │ 16000
│  │  ╰─ unb_pow2_mul_reuse                1.043 µs      │ 7.487 µs      │ 1.056 µs      │ 1.084 µs      │ 1000    │ 16000
│  ├─ u8                                                 │               │               │               │         │
│  │  ├─ baseline_identity                 10.82 ns      │ 33.09 ns      │ 11.7 ns       │ 12.6 ns       │ 1000    │ 1024000
│  │  ├─ pow2_mul                          95.1 ns       │ 270.1 ns      │ 96.66 ns      │ 98.32 ns      │ 1000    │ 128000
│  │  ├─ pow2_mul_const                    27.71 ns      │ 69.12 ns      │ 30.25 ns      │ 30.55 ns      │ 1000    │ 512000
│  │  ├─ pow2_mul_reuse                    22.05 ns      │ 50.18 ns      │ 25.96 ns      │ 26.85 ns      │ 1000    │ 256000
│  │  ├─ std_mul                           50.96 ns      │ 98.61 ns      │ 62.68 ns      │ 63.34 ns      │ 1000    │ 256000
│  │  ├─ std_mul_const                     27.13 ns      │ 73.81 ns      │ 29.86 ns      │ 30.86 ns      │ 1000    │ 512000
│  │  ├─ std_mul_reuse                     37.68 ns      │ 65.02 ns      │ 41.97 ns      │ 43.15 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_mul                      99 ns         │ 136.5 ns      │ 103.6 ns      │ 103.6 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_mul_const                27.91 ns      │ 64.24 ns      │ 30.45 ns      │ 30.78 ns      │ 1000    │ 512000
│  │  ╰─ unb_pow2_mul_reuse                31.62 ns      │ 83.57 ns      │ 34.55 ns      │ 35.51 ns      │ 1000    │ 512000
│  ├─ u16                                                │               │               │               │         │
│  │  ├─ baseline_identity                 24.2 ns       │ 90.8 ns       │ 25.37 ns      │ 26.84 ns      │ 1000    │ 512000
│  │  ├─ pow2_mul                          179.4 ns      │ 252.9 ns      │ 184.1 ns      │ 184.7 ns      │ 1000    │ 64000
│  │  ├─ pow2_mul_const                    50.57 ns      │ 107.2 ns      │ 54.08 ns      │ 54.53 ns      │ 1000    │ 256000
│  │  ├─ pow2_mul_reuse                    52.52 ns      │ 106 ns        │ 59.94 ns      │ 60.65 ns      │ 1000    │ 256000
│  │  ├─ std_mul                           64.63 ns      │ 1.281 µs      │ 72.44 ns      │ 77.42 ns      │ 1000    │ 128000
│  │  ├─ std_mul_const                     48.22 ns      │ 115 ns        │ 53.3 ns       │ 55.48 ns      │ 1000    │ 256000
│  │  ├─ std_mul_reuse                     53.3 ns       │ 147.4 ns      │ 56.82 ns      │ 59 ns         │ 1000    │ 256000
│  │  ├─ unb_pow2_mul                      181 ns        │ 2.402 µs      │ 185.7 ns      │ 195.2 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_mul_const                49 ns         │ 120.8 ns      │ 53.69 ns      │ 55.29 ns      │ 1000    │ 256000
│  │  ╰─ unb_pow2_mul_reuse                56.04 ns      │ 684.1 ns      │ 62.29 ns      │ 65.37 ns      │ 1000    │ 256000
│  ├─ u32                                                │               │               │               │         │
│  │  ├─ baseline_identity                 40.41 ns      │ 119.3 ns      │ 45.1 ns       │ 47.68 ns      │ 1000    │ 256000
│  │  ├─ pow2_mul                          273.2 ns      │ 2.504 µs      │ 287.2 ns      │ 298.4 ns      │ 1000    │ 64000
│  │  ├─ pow2_mul_const                    99 ns         │ 214.6 ns      │ 107.6 ns      │ 109.2 ns      │ 1000    │ 128000
│  │  ├─ pow2_mul_reuse                    95.1 ns       │ 1.063 µs      │ 106.8 ns      │ 109.3 ns      │ 1000    │ 128000
│  │  ├─ std_mul                           188.8 ns      │ 346.6 ns      │ 199.7 ns      │ 203.1 ns      │ 1000    │ 64000
│  │  ├─ std_mul_const                     101.3 ns      │ 263.8 ns      │ 112.2 ns      │ 113.4 ns      │ 1000    │ 128000
│  │  ├─ std_mul_reuse                     149.7 ns      │ 938 ns        │ 159.1 ns      │ 163.4 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_mul                      268.5 ns      │ 604.4 ns      │ 274.7 ns      │ 280.2 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_mul_const                97.44 ns      │ 249.7 ns      │ 109.1 ns      │ 110 ns        │ 1000    │ 128000
│  │  ╰─ unb_pow2_mul_reuse                101.3 ns      │ 290.4 ns      │ 115.4 ns      │ 116.8 ns      │ 1000    │ 128000
│  ├─ u64                                                │               │               │               │         │
│  │  ├─ baseline_identity                 68.54 ns      │ 621.6 ns      │ 77.91 ns      │ 80.94 ns      │ 1000    │ 64000
│  │  ├─ pow2_mul                          315.4 ns      │ 468.5 ns      │ 331 ns        │ 333.9 ns      │ 1000    │ 32000
│  │  ├─ pow2_mul_const                    193.5 ns      │ 487.2 ns      │ 215.4 ns      │ 221.5 ns      │ 1000    │ 64000
│  │  ├─ pow2_mul_reuse                    202.9 ns      │ 1.968 µs      │ 221.6 ns      │ 228.2 ns      │ 1000    │ 64000
│  │  ├─ std_mul                           449.7 ns      │ 5.143 µs      │ 462.2 ns      │ 483.6 ns      │ 1000    │ 32000
│  │  ├─ std_mul_const                     193.5 ns      │ 1.77 µs       │ 216.9 ns      │ 221.9 ns      │ 1000    │ 64000
│  │  ├─ std_mul_reuse                     287.2 ns      │ 465.4 ns      │ 302.9 ns      │ 304.1 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_mul                      421.6 ns      │ 746.6 ns      │ 434.1 ns      │ 437.5 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_mul_const                190.4 ns      │ 1.693 µs      │ 206 ns        │ 214.2 ns      │ 1000    │ 64000
│  │  ╰─ unb_pow2_mul_reuse                199.7 ns      │ 2.121 µs      │ 216.9 ns      │ 223.2 ns      │ 1000    │ 64000
│  ╰─ u128                                               │               │               │               │         │
│     ├─ baseline_identity                 237.2 ns      │ 546.6 ns      │ 251.3 ns      │ 265.3 ns      │ 1000    │ 64000
│     ├─ pow2_mul                          931 ns        │ 1.949 µs      │ 956 ns        │ 984.7 ns      │ 1000    │ 16000
│     ├─ pow2_mul_const                    856 ns        │ 2.093 µs      │ 899.7 ns      │ 920.7 ns      │ 1000    │ 16000
│     ├─ pow2_mul_reuse                    1.043 µs      │ 1.887 µs      │ 1.074 µs      │ 1.084 µs      │ 1000    │ 16000
│     ├─ std_mul                           1.224 µs      │ 1.787 µs      │ 1.249 µs      │ 1.257 µs      │ 1000    │ 8000
│     ├─ std_mul_const                     856 ns        │ 1.418 µs      │ 874.7 ns      │ 876.9 ns      │ 1000    │ 16000
│     ├─ std_mul_reuse                     837.2 ns      │ 2.262 µs      │ 856 ns        │ 880.7 ns      │ 1000    │ 16000
│     ├─ unb_pow2_mul                      943.5 ns      │ 1.556 µs      │ 962.2 ns      │ 967.2 ns      │ 1000    │ 16000
│     ├─ unb_pow2_mul_const                849.7 ns      │ 2.049 µs      │ 868.5 ns      │ 871.3 ns      │ 1000    │ 16000
│     ╰─ unb_pow2_mul_reuse                1.024 µs      │ 1.681 µs      │ 1.043 µs      │ 1.051 µs      │ 1000    │ 16000
╰─ round                                                 │               │               │               │         │
   ├─ i8                                                 │               │               │               │         │
   │  ├─ baseline_identity                 11.02 ns      │ 38.85 ns      │ 14.63 ns      │ 14.8 ns       │ 1000    │ 1024000
   │  ├─ pow2_ceil_to_multiple             99.79 ns      │ 1.124 µs      │ 103.6 ns      │ 106 ns        │ 1000    │ 128000
   │  ├─ pow2_ceil_to_multiple_const       26.93 ns      │ 87.09 ns      │ 29.86 ns      │ 30.18 ns      │ 1000    │ 512000
   │  ├─ pow2_ceil_to_multiple_reuse       29.47 ns      │ 66 ns         │ 32.6 ns       │ 32.87 ns      │ 1000    │ 512000
   │  ├─ pow2_floor_to_multiple            91.19 ns      │ 231.8 ns      │ 93.54 ns      │ 95.68 ns      │ 1000    │ 128000
   │  ├─ pow2_floor_to_multiple_const      26.93 ns      │ 80.25 ns      │ 28.4 ns       │ 29.22 ns      │ 1000    │ 512000
   │  ├─ pow2_floor_to_multiple_reuse      27.91 ns      │ 102.1 ns      │ 28.89 ns      │ 32.93 ns      │ 1000    │ 512000
   │  ├─ std_div_mul                       849.7 ns      │ 2.862 µs      │ 887.2 ns      │ 909.8 ns      │ 1000    │ 16000
   │  ├─ std_div_mul_const                 37.09 ns      │ 71.66 ns      │ 41.39 ns      │ 41.53 ns      │ 1000    │ 512000
   │  ├─ std_div_mul_reuse                 824.7 ns      │ 1.168 µs      │ 843.5 ns      │ 848 ns        │ 1000    │ 16000
   │  ├─ unb_pow2_ceil_to_multiple         102.9 ns      │ 182.6 ns      │ 107.6 ns      │ 107 ns        │ 1000    │ 128000
   │  ├─ unb_pow2_ceil_to_multiple_const   27.71 ns      │ 70.68 ns      │ 30.84 ns      │ 31.54 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   28.89 ns      │ 94.71 ns      │ 32.21 ns      │ 32.83 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_floor_to_multiple        99.79 ns      │ 148.2 ns      │ 102.1 ns      │ 102.9 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_floor_to_multiple_const  25.96 ns      │ 66 ns         │ 27.13 ns      │ 27.78 ns      │ 1000    │ 512000
   │  ╰─ unb_pow2_floor_to_multiple_reuse  26.93 ns      │ 66 ns         │ 28.3 ns       │ 29.05 ns      │ 1000    │ 512000
   ├─ i16                                                │               │               │               │         │
   │  ├─ baseline_identity                 20.49 ns      │ 76.35 ns      │ 22.83 ns      │ 23.32 ns      │ 1000    │ 256000
   │  ├─ pow2_ceil_to_multiple             190.4 ns      │ 334.1 ns      │ 195.1 ns      │ 198.7 ns      │ 1000    │ 64000
   │  ├─ pow2_ceil_to_multiple_const       55.25 ns      │ 499.3 ns      │ 60.92 ns      │ 62.02 ns      │ 1000    │ 256000
   │  ├─ pow2_ceil_to_multiple_reuse       59.55 ns      │ 179 ns        │ 68.93 ns      │ 69.78 ns      │ 1000    │ 256000
   │  ├─ pow2_floor_to_multiple            187.2 ns      │ 356 ns        │ 191.9 ns      │ 195.4 ns      │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple_const      50.96 ns      │ 166.5 ns      │ 55.25 ns      │ 56.5 ns       │ 1000    │ 256000
   │  ├─ pow2_floor_to_multiple_reuse      51.35 ns      │ 231 ns        │ 58.38 ns      │ 60.75 ns      │ 1000    │ 256000
   │  ├─ std_div_mul                       2.524 µs      │ 7.649 µs      │ 2.549 µs      │ 2.571 µs      │ 1000    │ 4000
   │  ├─ std_div_mul_const                 68.54 ns      │ 152.5 ns      │ 75.18 ns      │ 76.15 ns      │ 1000    │ 256000
   │  ├─ std_div_mul_reuse                 2.574 µs      │ 35.12 µs      │ 2.599 µs      │ 2.692 µs      │ 1000    │ 4000
   │  ├─ unb_pow2_ceil_to_multiple         195.1 ns      │ 366.9 ns      │ 198.2 ns      │ 200.5 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_ceil_to_multiple_const   57.21 ns      │ 134.1 ns      │ 61.5 ns       │ 62.39 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   58.38 ns      │ 132.6 ns      │ 66.19 ns      │ 67.64 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_floor_to_multiple        187.2 ns      │ 2.46 µs       │ 193.5 ns      │ 207.7 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple_const  51.74 ns      │ 130.6 ns      │ 55.64 ns      │ 56.13 ns      │ 1000    │ 256000
   │  ╰─ unb_pow2_floor_to_multiple_reuse  52.13 ns      │ 162.2 ns      │ 57.21 ns      │ 58.24 ns      │ 1000    │ 256000
   ├─ i32                                                │               │               │               │         │
   │  ├─ baseline_identity                 40.41 ns      │ 182.9 ns      │ 44.71 ns      │ 46.44 ns      │ 1000    │ 256000
   │  ├─ pow2_ceil_to_multiple             285.7 ns      │ 496.6 ns      │ 293.5 ns      │ 295.4 ns      │ 1000    │ 64000
   │  ├─ pow2_ceil_to_multiple_const       106 ns        │ 254.4 ns      │ 116.1 ns      │ 117.2 ns      │ 1000    │ 128000
   │  ├─ pow2_ceil_to_multiple_reuse       110.7 ns      │ 404.4 ns      │ 125.5 ns      │ 127.9 ns      │ 1000    │ 128000
   │  ├─ pow2_floor_to_multiple            266.9 ns      │ 2.735 µs      │ 273.2 ns      │ 279.6 ns      │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple_const      91.19 ns      │ 332.6 ns      │ 102.1 ns      │ 104.3 ns      │ 1000    │ 128000
   │  ├─ pow2_floor_to_multiple_reuse      96.66 ns      │ 243.5 ns      │ 107.6 ns      │ 110.2 ns      │ 1000    │ 128000
   │  ├─ std_div_mul                       1.324 µs      │ 15.79 µs      │ 1.349 µs      │ 1.373 µs      │ 1000    │ 8000
   │  ├─ std_div_mul_const                 131 ns        │ 1.006 µs      │ 144.3 ns      │ 146.3 ns      │ 1000    │ 128000
   │  ├─ std_div_mul_reuse                 1.287 µs      │ 1.799 µs      │ 1.287 µs      │ 1.292 µs      │ 1000    │ 8000
   │  ├─ unb_pow2_ceil_to_multiple         299.7 ns      │ 468.5 ns      │ 304.4 ns      │ 307.3 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_ceil_to_multiple_const   101.3 ns      │ 289.6 ns      │ 113 ns        │ 114.7 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   84.16 ns      │ 396.6 ns      │ 90.41 ns      │ 95.47 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple        276.3 ns      │ 598.2 ns      │ 282.6 ns      │ 289.5 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple_const  97.44 ns      │ 1.245 µs      │ 107.6 ns      │ 111.8 ns      │ 1000    │ 128000
   │  ╰─ unb_pow2_floor_to_multiple_reuse  102.9 ns      │ 260.7 ns      │ 114.6 ns      │ 117.6 ns      │ 1000    │ 128000
   ├─ i64                                                │               │               │               │         │
   │  ├─ baseline_identity                 76.35 ns      │ 434.1 ns      │ 98.22 ns      │ 105 ns        │ 1000    │ 128000
   │  ├─ pow2_ceil_to_multiple             493.5 ns      │ 5.646 µs      │ 502.9 ns      │ 533.7 ns      │ 1000    │ 32000
   │  ├─ pow2_ceil_to_multiple_const       213.8 ns      │ 529.4 ns      │ 245.1 ns      │ 245.9 ns      │ 1000    │ 64000
   │  ├─ pow2_ceil_to_multiple_reuse       212.2 ns      │ 554.4 ns      │ 245.1 ns      │ 248.3 ns      │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple            402.9 ns      │ 2.334 µs      │ 415.4 ns      │ 421.5 ns      │ 1000    │ 32000
   │  ├─ pow2_floor_to_multiple_const      188.8 ns      │ 588.8 ns      │ 216.9 ns      │ 221.8 ns      │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple_reuse      190.4 ns      │ 1.274 µs      │ 212.2 ns      │ 217.4 ns      │ 1000    │ 64000
   │  ├─ std_div_mul                       1.662 µs      │ 16.28 µs      │ 1.687 µs      │ 1.727 µs      │ 1000    │ 8000
   │  ├─ std_div_mul_const                 240.4 ns      │ 585.7 ns      │ 266.9 ns      │ 274.7 ns      │ 1000    │ 64000
   │  ├─ std_div_mul_reuse                 1.512 µs      │ 2.812 µs      │ 1.537 µs      │ 1.547 µs      │ 1000    │ 8000
   │  ├─ unb_pow2_ceil_to_multiple         521.6 ns      │ 4.199 µs      │ 531 ns        │ 550.4 ns      │ 1000    │ 32000
   │  ├─ unb_pow2_ceil_to_multiple_const   209.1 ns      │ 576.3 ns      │ 248.2 ns      │ 249.6 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   196.6 ns      │ 698.2 ns      │ 235.7 ns      │ 238.5 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple        487.2 ns      │ 1.302 µs      │ 502.9 ns      │ 517.7 ns      │ 1000    │ 32000
   │  ├─ unb_pow2_floor_to_multiple_const  188.8 ns      │ 438.8 ns      │ 221.6 ns      │ 223.3 ns      │ 1000    │ 64000
   │  ╰─ unb_pow2_floor_to_multiple_reuse  209.1 ns      │ 827.9 ns      │ 221.6 ns      │ 226.3 ns      │ 1000    │ 64000
   ├─ i128                                               │               │               │               │         │
   │  ├─ baseline_identity                 234.1 ns      │ 671.6 ns      │ 257.6 ns      │ 271.5 ns      │ 1000    │ 64000
   │  ├─ pow2_ceil_to_multiple             1.118 µs      │ 1.787 µs      │ 1.137 µs      │ 1.148 µs      │ 1000    │ 16000
   │  ├─ pow2_ceil_to_multiple_const       618.5 ns      │ 1.849 µs      │ 637.2 ns      │ 640.5 ns      │ 1000    │ 16000
   │  ├─ pow2_ceil_to_multiple_reuse       606 ns        │ 999.7 ns      │ 631 ns        │ 635.2 ns      │ 1000    │ 16000
   │  ├─ pow2_floor_to_multiple            818.5 ns      │ 1.549 µs      │ 837.2 ns      │ 845.3 ns      │ 1000    │ 16000
   │  ├─ pow2_floor_to_multiple_const      502.9 ns      │ 1.184 µs      │ 556 ns        │ 578.1 ns      │ 1000    │ 32000
   │  ├─ pow2_floor_to_multiple_reuse      606 ns        │ 2.265 µs      │ 649.7 ns      │ 657.7 ns      │ 1000    │ 32000
   │  ├─ std_div_mul                       10.19 µs      │ 146.7 µs      │ 10.29 µs      │ 10.64 µs      │ 1000    │ 1000
   │  ├─ std_div_mul_const                 718.5 ns      │ 3.074 µs      │ 737.2 ns      │ 746.2 ns      │ 1000    │ 16000
   │  ├─ std_div_mul_reuse                 9.099 µs      │ 77.69 µs      │ 9.149 µs      │ 9.532 µs      │ 1000    │ 2000
   │  ├─ unb_pow2_ceil_to_multiple         1.106 µs      │ 9.012 µs      │ 1.131 µs      │ 1.185 µs      │ 1000    │ 16000
   │  ├─ unb_pow2_ceil_to_multiple_const   618.5 ns      │ 6.881 µs      │ 643.5 ns      │ 660.3 ns      │ 1000    │ 16000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   599.7 ns      │ 993.5 ns      │ 624.7 ns      │ 631 ns        │ 1000    │ 16000
   │  ├─ unb_pow2_floor_to_multiple        849.7 ns      │ 2.056 µs      │ 862.2 ns      │ 869.5 ns      │ 1000    │ 16000
   │  ├─ unb_pow2_floor_to_multiple_const  521.6 ns      │ 940.4 ns      │ 559.1 ns      │ 566.2 ns      │ 1000    │ 32000
   │  ╰─ unb_pow2_floor_to_multiple_reuse  612.2 ns      │ 2.118 µs      │ 646.6 ns      │ 656.6 ns      │ 1000    │ 32000
   ├─ u8                                                 │               │               │               │         │
   │  ├─ baseline_identity                 10.92 ns      │ 46.86 ns      │ 13.65 ns      │ 14.29 ns      │ 1000    │ 1024000
   │  ├─ pow2_ceil_to_multiple             99.79 ns      │ 211.5 ns      │ 103.6 ns      │ 104 ns        │ 1000    │ 128000
   │  ├─ pow2_ceil_to_multiple_const       27.13 ns      │ 62.48 ns      │ 30.64 ns      │ 30.99 ns      │ 1000    │ 512000
   │  ├─ pow2_ceil_to_multiple_reuse       29.47 ns      │ 67.75 ns      │ 32.4 ns       │ 33.15 ns      │ 1000    │ 512000
   │  ├─ pow2_floor_to_multiple            91.19 ns      │ 252.1 ns      │ 94.32 ns      │ 97.76 ns      │ 1000    │ 128000
   │  ├─ pow2_floor_to_multiple_const      26.15 ns      │ 73.42 ns      │ 27.52 ns      │ 28.29 ns      │ 1000    │ 512000
   │  ├─ pow2_floor_to_multiple_reuse      26.54 ns      │ 75.18 ns      │ 28.69 ns      │ 31.07 ns      │ 1000    │ 512000
   │  ├─ std_div_mul                       843.5 ns      │ 5.106 µs      │ 874.7 ns      │ 880.7 ns      │ 1000    │ 16000
   │  ├─ std_div_mul_const                 25.96 ns      │ 59.55 ns      │ 27.13 ns      │ 28.01 ns      │ 1000    │ 512000
   │  ├─ std_div_mul_reuse                 831 ns        │ 1.456 µs      │ 843.5 ns      │ 848.8 ns      │ 1000    │ 16000
   │  ├─ unb_pow2_ceil_to_multiple         102.9 ns      │ 140.4 ns      │ 106 ns        │ 106.8 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_ceil_to_multiple_const   26.93 ns      │ 65.41 ns      │ 29.08 ns      │ 29.34 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   29.28 ns      │ 68.14 ns      │ 31.82 ns      │ 32.55 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_floor_to_multiple        100.5 ns      │ 217.7 ns      │ 102.1 ns      │ 105 ns        │ 1000    │ 128000
   │  ├─ unb_pow2_floor_to_multiple_const  27.32 ns      │ 67.36 ns      │ 28.5 ns       │ 29.05 ns      │ 1000    │ 512000
   │  ╰─ unb_pow2_floor_to_multiple_reuse  28.3 ns       │ 226.5 ns      │ 29.47 ns      │ 31.15 ns      │ 1000    │ 512000
   ├─ u16                                                │               │               │               │         │
   │  ├─ baseline_identity                 24.59 ns      │ 337.2 ns      │ 25.76 ns      │ 27.12 ns      │ 1000    │ 512000
   │  ├─ pow2_ceil_to_multiple             190.4 ns      │ 352.9 ns      │ 195.1 ns      │ 195.7 ns      │ 1000    │ 64000
   │  ├─ pow2_ceil_to_multiple_const       53.69 ns      │ 129.8 ns      │ 59.55 ns      │ 60.36 ns      │ 1000    │ 256000
   │  ├─ pow2_ceil_to_multiple_reuse       59.16 ns      │ 171.2 ns      │ 65.41 ns      │ 66.73 ns      │ 1000    │ 256000
   │  ├─ pow2_floor_to_multiple            185.7 ns      │ 2.618 µs      │ 191.9 ns      │ 197.7 ns      │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple_const      50.57 ns      │ 466.5 ns      │ 55.25 ns      │ 57.23 ns      │ 1000    │ 256000
   │  ├─ pow2_floor_to_multiple_reuse      51.35 ns      │ 125.9 ns      │ 57.6 ns       │ 58.55 ns      │ 1000    │ 256000
   │  ├─ std_div_mul                       868.5 ns      │ 9.124 µs      │ 881 ns        │ 908.6 ns      │ 1000    │ 16000
   │  ├─ std_div_mul_const                 52.52 ns      │ 592.7 ns      │ 54.86 ns      │ 56.71 ns      │ 1000    │ 256000
   │  ├─ std_div_mul_reuse                 831 ns        │ 1.474 µs      │ 849.7 ns      │ 854.3 ns      │ 1000    │ 16000
   │  ├─ unb_pow2_ceil_to_multiple         195.1 ns      │ 274.7 ns      │ 202.9 ns      │ 203.2 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_ceil_to_multiple_const   56.82 ns      │ 151.3 ns      │ 60.33 ns      │ 61.62 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   59.94 ns      │ 150.9 ns      │ 66.97 ns      │ 67.58 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_floor_to_multiple        190.4 ns      │ 409.1 ns      │ 193.5 ns      │ 194.5 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple_const  50.96 ns      │ 152.1 ns      │ 54.86 ns      │ 56.08 ns      │ 1000    │ 256000
   │  ╰─ unb_pow2_floor_to_multiple_reuse  51.74 ns      │ 246.2 ns      │ 56.43 ns      │ 57.83 ns      │ 1000    │ 256000
   ├─ u32                                                │               │               │               │         │
   │  ├─ baseline_identity                 41.97 ns      │ 177.5 ns      │ 49 ns         │ 52.63 ns      │ 1000    │ 256000
   │  ├─ pow2_ceil_to_multiple             285.7 ns      │ 871.6 ns      │ 293.5 ns      │ 294.8 ns      │ 1000    │ 64000
   │  ├─ pow2_ceil_to_multiple_const       100.5 ns      │ 255.2 ns      │ 116.1 ns      │ 116.9 ns      │ 1000    │ 128000
   │  ├─ pow2_ceil_to_multiple_reuse       106 ns        │ 1.089 µs      │ 126.3 ns      │ 128.3 ns      │ 1000    │ 128000
   │  ├─ pow2_floor_to_multiple            265.4 ns      │ 8.921 µs      │ 273.2 ns      │ 293.4 ns      │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple_const      91.97 ns      │ 916.9 ns      │ 106.8 ns      │ 108.9 ns      │ 1000    │ 128000
   │  ├─ pow2_floor_to_multiple_reuse      99.79 ns      │ 262.2 ns      │ 110.7 ns      │ 112.1 ns      │ 1000    │ 128000
   │  ├─ std_div_mul                       1.312 µs      │ 3.149 µs      │ 1.324 µs      │ 1.346 µs      │ 1000    │ 8000
   │  ├─ std_div_mul_const                 95.1 ns       │ 620.8 ns      │ 104.4 ns      │ 108.4 ns      │ 1000    │ 128000
   │  ├─ std_div_mul_reuse                 1.299 µs      │ 19.78 µs      │ 1.299 µs      │ 1.352 µs      │ 1000    │ 8000
   │  ├─ unb_pow2_ceil_to_multiple         298.2 ns      │ 2.382 µs      │ 302.9 ns      │ 315.4 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_ceil_to_multiple_const   99.79 ns      │ 253.6 ns      │ 113 ns        │ 113.5 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   109.9 ns      │ 870.1 ns      │ 123.2 ns      │ 125.5 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_floor_to_multiple        276.3 ns      │ 435.7 ns      │ 282.6 ns      │ 284 ns        │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple_const  98.22 ns      │ 255.2 ns      │ 107.6 ns      │ 108.8 ns      │ 1000    │ 128000
   │  ╰─ unb_pow2_floor_to_multiple_reuse  105.2 ns      │ 272.4 ns      │ 114.6 ns      │ 117.2 ns      │ 1000    │ 128000
   ├─ u64                                                │               │               │               │         │
   │  ├─ baseline_identity                 79.47 ns      │ 314.6 ns      │ 106.8 ns      │ 112.5 ns      │ 1000    │ 128000
   │  ├─ pow2_ceil_to_multiple             487.2 ns      │ 837.2 ns      │ 499.7 ns      │ 505.5 ns      │ 1000    │ 32000
   │  ├─ pow2_ceil_to_multiple_const       206 ns        │ 602.9 ns      │ 237.2 ns      │ 249.1 ns      │ 1000    │ 64000
   │  ├─ pow2_ceil_to_multiple_reuse       213.8 ns      │ 502.9 ns      │ 237.2 ns      │ 242.5 ns      │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple            406 ns        │ 615.4 ns      │ 415.4 ns      │ 420.9 ns      │ 1000    │ 32000
   │  ├─ pow2_floor_to_multiple_const      190.4 ns      │ 423.2 ns      │ 215.4 ns      │ 219.7 ns      │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple_reuse      195.1 ns      │ 741.9 ns      │ 215.4 ns      │ 224.7 ns      │ 1000    │ 64000
   │  ├─ std_div_mul                       1.687 µs      │ 3.837 µs      │ 1.699 µs      │ 1.71 µs       │ 1000    │ 8000
   │  ├─ std_div_mul_const                 190.4 ns      │ 385.7 ns      │ 218.5 ns      │ 220.3 ns      │ 1000    │ 64000
   │  ├─ std_div_mul_reuse                 1.524 µs      │ 19.93 µs      │ 1.537 µs      │ 1.598 µs      │ 1000    │ 8000
   │  ├─ unb_pow2_ceil_to_multiple         518.5 ns      │ 4.618 µs      │ 534.1 ns      │ 556.3 ns      │ 1000    │ 32000
   │  ├─ unb_pow2_ceil_to_multiple_const   218.5 ns      │ 2.338 µs      │ 251.3 ns      │ 255.5 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   218.5 ns      │ 581 ns        │ 235.7 ns      │ 241.3 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple        484.1 ns      │ 821.6 ns      │ 499.7 ns      │ 503.3 ns      │ 1000    │ 32000
   │  ├─ unb_pow2_floor_to_multiple_const  190.4 ns      │ 431 ns        │ 210.7 ns      │ 214.9 ns      │ 1000    │ 64000
   │  ╰─ unb_pow2_floor_to_multiple_reuse  196.6 ns      │ 563.8 ns      │ 218.5 ns      │ 252.9 ns      │ 1000    │ 64000
   ╰─ u128                                               │               │               │               │         │
      ├─ baseline_identity                 237.2 ns      │ 660.7 ns      │ 268.5 ns      │ 281.7 ns      │ 1000    │ 64000
      ├─ pow2_ceil_to_multiple             1.106 µs      │ 1.593 µs      │ 1.137 µs      │ 1.146 µs      │ 1000    │ 16000
      ├─ pow2_ceil_to_multiple_const       631 ns        │ 1.412 µs      │ 649.7 ns      │ 656.4 ns      │ 1000    │ 16000
      ├─ pow2_ceil_to_multiple_reuse       599.7 ns      │ 968.5 ns      │ 624.7 ns      │ 627.6 ns      │ 1000    │ 16000
      ├─ pow2_floor_to_multiple            824.7 ns      │ 1.906 µs      │ 843.5 ns      │ 849.1 ns      │ 1000    │ 16000
      ├─ pow2_floor_to_multiple_const      521.6 ns      │ 965.4 ns      │ 562.2 ns      │ 571.6 ns      │ 1000    │ 32000
      ├─ pow2_floor_to_multiple_reuse      615.4 ns      │ 1.34 µs       │ 659.1 ns      │ 674.7 ns      │ 1000    │ 32000
      ├─ std_div_mul                       4.299 µs      │ 8.149 µs      │ 4.374 µs      │ 4.455 µs      │ 1000    │ 4000
      ├─ std_div_mul_const                 534.1 ns      │ 1.081 µs      │ 562.2 ns      │ 569.2 ns      │ 1000    │ 32000
      ├─ std_div_mul_reuse                 4.224 µs      │ 8.224 µs      │ 4.299 µs      │ 4.363 µs      │ 1000    │ 4000
      ├─ unb_pow2_ceil_to_multiple         1.112 µs      │ 2.687 µs      │ 1.131 µs      │ 1.153 µs      │ 1000    │ 16000
      ├─ unb_pow2_ceil_to_multiple_const   637.2 ns      │ 4.643 µs      │ 693.5 ns      │ 692 ns        │ 1000    │ 16000
      ├─ unb_pow2_ceil_to_multiple_reuse   606 ns        │ 1.881 µs      │ 631 ns        │ 661.5 ns      │ 1000    │ 16000
      ├─ unb_pow2_floor_to_multiple        843.5 ns      │ 4.218 µs      │ 856 ns        │ 863 ns        │ 1000    │ 16000
      ├─ unb_pow2_floor_to_multiple_const  527.9 ns      │ 1.196 µs      │ 568.5 ns      │ 576.3 ns      │ 1000    │ 32000
      ╰─ unb_pow2_floor_to_multiple_reuse  556 ns        │ 981 ns        │ 587.2 ns      │ 593.2 ns      │ 1000    │ 16000


```
