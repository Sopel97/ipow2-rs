# `public` bench with native target (AMD Ryzen 7 7800X3D 8-Core Processor)
```
public                                     fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ div                                                   │               │               │               │         │
│  ├─ i8                                                 │               │               │               │         │
│  │  ├─ baseline_identity                 11.31 ns      │ 59.84 ns      │ 17.17 ns      │ 18.26 ns      │ 1000    │ 1024000
│  │  ├─ pow2_div                          40.41 ns      │ 149 ns        │ 49.79 ns      │ 49.9 ns       │ 1000    │ 256000
│  │  ├─ pow2_div_ceil                     43.54 ns      │ 433.7 ns      │ 48.22 ns      │ 49.25 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_ceil_const               22.44 ns      │ 223.8 ns      │ 25.57 ns      │ 27 ns         │ 1000    │ 512000
│  │  ├─ pow2_div_ceil_reuse               25.37 ns      │ 254.4 ns      │ 28.5 ns       │ 30.24 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_const                    20.68 ns      │ 69.9 ns       │ 24.49 ns      │ 25.97 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_floor                    32.21 ns      │ 86.11 ns      │ 36.11 ns      │ 36.58 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_floor_const              19.51 ns      │ 57.6 ns       │ 22.05 ns      │ 23.63 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_floor_reuse              21.66 ns      │ 93.14 ns      │ 24.59 ns      │ 26 ns         │ 1000    │ 512000
│  │  ├─ pow2_div_reuse                    24.39 ns      │ 248.4 ns      │ 27.52 ns      │ 29.03 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_round                    51.35 ns      │ 153.6 ns      │ 60.33 ns      │ 60.38 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_round_const              24.2 ns       │ 159.3 ns      │ 29.28 ns      │ 31.02 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_round_reuse              32.99 ns      │ 103.6 ns      │ 36.89 ns      │ 38.67 ns      │ 1000    │ 512000
│  │  ├─ std_div                           874.7 ns      │ 1.981 µs      │ 887.2 ns      │ 903 ns        │ 1000    │ 16000
│  │  ├─ std_div_const                     20.88 ns      │ 85.14 ns      │ 24.98 ns      │ 26.23 ns      │ 1000    │ 512000
│  │  ├─ std_div_reuse                     837.2 ns      │ 1.149 µs      │ 849.7 ns      │ 850.4 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_div                      37.68 ns      │ 482.6 ns      │ 45.49 ns      │ 46.32 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_ceil                 41.97 ns      │ 441.9 ns      │ 49.79 ns      │ 49.85 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_ceil_const           22.44 ns      │ 73.03 ns      │ 25.66 ns      │ 26.93 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_ceil_reuse           25.37 ns      │ 70.68 ns      │ 29.08 ns      │ 30.44 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_const                18.54 ns      │ 76.35 ns      │ 22.05 ns      │ 22.18 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_floor                35.33 ns      │ 94.71 ns      │ 41.58 ns      │ 41.92 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_floor_const          19.51 ns      │ 72.05 ns      │ 22.25 ns      │ 23.5 ns       │ 1000    │ 512000
│  │  ├─ unb_pow2_div_floor_reuse          21.46 ns      │ 301.1 ns      │ 24.39 ns      │ 26.04 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_reuse                24.2 ns       │ 63.65 ns      │ 27.91 ns      │ 29.1 ns       │ 1000    │ 512000
│  │  ├─ unb_pow2_div_round                48.61 ns      │ 247.4 ns      │ 55.64 ns      │ 56.89 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_round_const          22.44 ns      │ 78.69 ns      │ 29.67 ns      │ 31.72 ns      │ 1000    │ 512000
│  │  ╰─ unb_pow2_div_round_reuse          33.18 ns      │ 239.2 ns      │ 36.89 ns      │ 38.79 ns      │ 1000    │ 512000
│  ├─ i16                                                │               │               │               │         │
│  │  ├─ baseline_identity                 22.25 ns      │ 244.1 ns      │ 28.59 ns      │ 32.72 ns      │ 1000    │ 512000
│  │  ├─ pow2_div                          71.27 ns      │ 354 ns        │ 78.69 ns      │ 84.95 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_ceil                     70.49 ns      │ 207.6 ns      │ 82.99 ns      │ 87.78 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_ceil_const               43.93 ns      │ 158.3 ns      │ 50.96 ns      │ 53.79 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_ceil_reuse               48.22 ns      │ 119.3 ns      │ 54.86 ns      │ 57.31 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_const                    42.75 ns      │ 142.3 ns      │ 47.83 ns      │ 50.07 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_floor                    55.25 ns      │ 169.3 ns      │ 61.89 ns      │ 68.37 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_floor_const              38.07 ns      │ 125.5 ns      │ 42.75 ns      │ 45.8 ns       │ 1000    │ 256000
│  │  ├─ pow2_div_floor_reuse              40.8 ns       │ 141.1 ns      │ 52.52 ns      │ 59.09 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_reuse                    45.88 ns      │ 249 ns        │ 52.13 ns      │ 56.08 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_round                    78.69 ns      │ 209.1 ns      │ 92.75 ns      │ 97.56 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_round_const              51.74 ns      │ 608.7 ns      │ 57.99 ns      │ 61.99 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_round_reuse              58.77 ns      │ 154.8 ns      │ 64.63 ns      │ 67.19 ns      │ 1000    │ 256000
│  │  ├─ std_div                           874.7 ns      │ 1.249 µs      │ 899.7 ns      │ 901.2 ns      │ 1000    │ 16000
│  │  ├─ std_div_const                     43.93 ns      │ 218.9 ns      │ 50.96 ns      │ 53.56 ns      │ 1000    │ 256000
│  │  ├─ std_div_reuse                     2.499 µs      │ 5.174 µs      │ 2.524 µs      │ 2.53 µs       │ 1000    │ 4000
│  │  ├─ unb_pow2_div                      63.85 ns      │ 245.1 ns      │ 76.35 ns      │ 80.59 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_ceil                 64.63 ns      │ 552.1 ns      │ 78.3 ns       │ 84.4 ns       │ 1000    │ 256000
│  │  ├─ unb_pow2_div_ceil_const           43.54 ns      │ 156 ns        │ 50.57 ns      │ 52.94 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_ceil_reuse           48.22 ns      │ 143.5 ns      │ 54.47 ns      │ 57.13 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_const                41.97 ns      │ 106.4 ns      │ 48.22 ns      │ 50.03 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_floor                56.04 ns      │ 213.4 ns      │ 69.51 ns      │ 76.15 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_floor_const          37.68 ns      │ 107.6 ns      │ 42.75 ns      │ 46 ns         │ 1000    │ 256000
│  │  ├─ unb_pow2_div_floor_reuse          40.8 ns       │ 128.6 ns      │ 45.88 ns      │ 49.06 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_reuse                44.32 ns      │ 146.2 ns      │ 51.35 ns      │ 54.74 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_round                75.18 ns      │ 178.3 ns      │ 89.63 ns      │ 94.52 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_round_const          53.3 ns       │ 640.4 ns      │ 58.38 ns      │ 61.79 ns      │ 1000    │ 256000
│  │  ╰─ unb_pow2_div_round_reuse          59.16 ns      │ 120.4 ns      │ 65.02 ns      │ 67.57 ns      │ 1000    │ 256000
│  ├─ i32                                                │               │               │               │         │
│  │  ├─ baseline_identity                 41.19 ns      │ 199.7 ns      │ 55.06 ns      │ 61.52 ns      │ 1000    │ 256000
│  │  ├─ pow2_div                          118.5 ns      │ 361.5 ns      │ 137.2 ns      │ 144 ns        │ 1000    │ 128000
│  │  ├─ pow2_div_ceil                     121.6 ns      │ 355.2 ns      │ 141.1 ns      │ 148.3 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_ceil_const               80.25 ns      │ 328.6 ns      │ 92.75 ns      │ 96.54 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_ceil_reuse               83.38 ns      │ 278.6 ns      │ 98.22 ns      │ 103.4 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_const                    82.6 ns       │ 215 ns        │ 91.19 ns      │ 96.87 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_floor                    94.32 ns      │ 434.9 ns      │ 113.8 ns      │ 120.7 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_floor_const              79.47 ns      │ 305.2 ns      │ 84.94 ns      │ 91.95 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_floor_reuse              79.47 ns      │ 413 ns        │ 85.33 ns      │ 93.03 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_reuse                    81.82 ns      │ 269.3 ns      │ 92.75 ns      │ 99.47 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_round                    147.4 ns      │ 1.321 µs      │ 171.6 ns      │ 177.8 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_round_const              99 ns         │ 317.7 ns      │ 112.2 ns      │ 117.5 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_round_reuse              117.7 ns      │ 293.5 ns      │ 131 ns        │ 137.3 ns      │ 1000    │ 128000
│  │  ├─ std_div                           1.324 µs      │ 12.09 µs      │ 1.337 µs      │ 1.356 µs      │ 1000    │ 8000
│  │  ├─ std_div_const                     78.69 ns      │ 948.2 ns      │ 92.75 ns      │ 97.68 ns      │ 1000    │ 128000
│  │  ├─ std_div_reuse                     1.274 µs      │ 3.574 µs      │ 1.287 µs      │ 1.31 µs       │ 1000    │ 8000
│  │  ├─ unb_pow2_div                      123.2 ns      │ 402.1 ns      │ 152.9 ns      │ 157.4 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_ceil                 124.7 ns      │ 356.8 ns      │ 152.1 ns      │ 155.9 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_ceil_const           79.47 ns      │ 1.445 µs      │ 91.97 ns      │ 97.69 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_ceil_reuse           87.29 ns      │ 243.5 ns      │ 99 ns         │ 103.5 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_const                77.13 ns      │ 202.1 ns      │ 87.29 ns      │ 91.69 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_floor                94.32 ns      │ 343.5 ns      │ 118.5 ns      │ 124.1 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_floor_const          79.86 ns      │ 309.5 ns      │ 85.92 ns      │ 93.79 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_floor_reuse          79.47 ns      │ 263.8 ns      │ 85.72 ns      │ 93.68 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_reuse                84.16 ns      │ 274 ns        │ 95.1 ns       │ 100.3 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_round                144.3 ns      │ 470.8 ns      │ 168.5 ns      │ 175 ns        │ 1000    │ 128000
│  │  ├─ unb_pow2_div_round_const          102.1 ns      │ 278.6 ns      │ 112.2 ns      │ 117.1 ns      │ 1000    │ 128000
│  │  ╰─ unb_pow2_div_round_reuse          107.6 ns      │ 282.6 ns      │ 127.9 ns      │ 136.4 ns      │ 1000    │ 128000
│  ├─ i64                                                │               │               │               │         │
│  │  ├─ baseline_identity                 74.79 ns      │ 263 ns        │ 114.6 ns      │ 121 ns        │ 1000    │ 128000
│  │  ├─ pow2_div                          223.2 ns      │ 710.7 ns      │ 249.7 ns      │ 262.4 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_ceil                     226.3 ns      │ 643.5 ns      │ 254.4 ns      │ 265.5 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_ceil_const               163.8 ns      │ 651.3 ns      │ 187.2 ns      │ 197.8 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_ceil_reuse               176.3 ns      │ 570.1 ns      │ 198.2 ns      │ 208.7 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_const                    156 ns        │ 402.9 ns      │ 185.7 ns      │ 194.6 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_floor                    176.3 ns      │ 745.1 ns      │ 218.5 ns      │ 232.2 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_floor_const              151.3 ns      │ 391.9 ns      │ 178.6 ns      │ 186.7 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_floor_reuse              146.6 ns      │ 590.4 ns      │ 165.4 ns      │ 174.4 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_reuse                    168.5 ns      │ 543.5 ns      │ 195.1 ns      │ 216.6 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_round                    277.9 ns      │ 1.298 µs      │ 324.7 ns      │ 339 ns        │ 1000    │ 64000
│  │  ├─ pow2_div_round_const              202.9 ns      │ 460.7 ns      │ 229.4 ns      │ 239.2 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_round_reuse              232.6 ns      │ 615.4 ns      │ 259.1 ns      │ 269.9 ns      │ 1000    │ 64000
│  │  ├─ std_div                           1.624 µs      │ 4.924 µs      │ 1.674 µs      │ 1.689 µs      │ 1000    │ 8000
│  │  ├─ std_div_const                     162.2 ns      │ 679.4 ns      │ 185.7 ns      │ 195.3 ns      │ 1000    │ 64000
│  │  ├─ std_div_reuse                     1.512 µs      │ 3.187 µs      │ 1.537 µs      │ 1.545 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div                      227.9 ns      │ 924.7 ns      │ 252.9 ns      │ 264.3 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil                 232.6 ns      │ 557.6 ns      │ 256 ns        │ 269.2 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil_const           157.6 ns      │ 618.5 ns      │ 185.7 ns      │ 195.4 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil_reuse           174.7 ns      │ 402.9 ns      │ 199.7 ns      │ 208.1 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_const                157.6 ns      │ 662.2 ns      │ 187.2 ns      │ 197.3 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor                179.4 ns      │ 562.2 ns      │ 227.9 ns      │ 242.6 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor_const          138.8 ns      │ 570.1 ns      │ 156 ns        │ 167 ns        │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor_reuse          160.7 ns      │ 710.7 ns      │ 187.2 ns      │ 204.2 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_reuse                163.8 ns      │ 434.1 ns      │ 179.4 ns      │ 190.3 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_round                271.6 ns      │ 2.193 µs      │ 312.2 ns      │ 329.3 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_round_const          201.3 ns      │ 554.4 ns      │ 227.9 ns      │ 241.1 ns      │ 1000    │ 64000
│  │  ╰─ unb_pow2_div_round_reuse          232.6 ns      │ 818.5 ns      │ 262.2 ns      │ 273.1 ns      │ 1000    │ 64000
│  ├─ i128                                               │               │               │               │         │
│  │  ├─ baseline_identity                 209.1 ns      │ 571.6 ns      │ 234.1 ns      │ 242.4 ns      │ 1000    │ 64000
│  │  ├─ pow2_div                          1.674 µs      │ 4.499 µs      │ 1.724 µs      │ 1.738 µs      │ 1000    │ 4000
│  │  ├─ pow2_div_ceil                     1.924 µs      │ 16.33 µs      │ 1.974 µs      │ 2.029 µs      │ 1000    │ 8000
│  │  ├─ pow2_div_ceil_const               1.106 µs      │ 9.112 µs      │ 1.187 µs      │ 1.204 µs      │ 1000    │ 16000
│  │  ├─ pow2_div_ceil_reuse               1.512 µs      │ 15.08 µs      │ 1.537 µs      │ 1.567 µs      │ 1000    │ 8000
│  │  ├─ pow2_div_const                    1.074 µs      │ 9.437 µs      │ 1.162 µs      │ 1.223 µs      │ 1000    │ 16000
│  │  ├─ pow2_div_floor                    1.037 µs      │ 2.093 µs      │ 1.118 µs      │ 1.127 µs      │ 1000    │ 16000
│  │  ├─ pow2_div_floor_const              793.5 ns      │ 1.518 µs      │ 812.2 ns      │ 827 ns        │ 1000    │ 16000
│  │  ├─ pow2_div_floor_reuse              1.081 µs      │ 1.924 µs      │ 1.099 µs      │ 1.137 µs      │ 1000    │ 16000
│  │  ├─ pow2_div_reuse                    1.299 µs      │ 4.224 µs      │ 1.312 µs      │ 1.347 µs      │ 1000    │ 8000
│  │  ├─ pow2_div_round                    2.799 µs      │ 3.874 µs      │ 2.824 µs      │ 2.832 µs      │ 1000    │ 4000
│  │  ├─ pow2_div_round_const              1.687 µs      │ 3.137 µs      │ 1.724 µs      │ 1.742 µs      │ 1000    │ 8000
│  │  ├─ pow2_div_round_reuse              2.362 µs      │ 19.14 µs      │ 2.374 µs      │ 2.427 µs      │ 1000    │ 8000
│  │  ├─ std_div                           8.899 µs      │ 60.09 µs      │ 8.999 µs      │ 9.187 µs      │ 1000    │ 2000
│  │  ├─ std_div_const                     1.037 µs      │ 7.843 µs      │ 1.056 µs      │ 1.088 µs      │ 1000    │ 16000
│  │  ├─ std_div_reuse                     8.049 µs      │ 69.94 µs      │ 8.099 µs      │ 8.176 µs      │ 1000    │ 2000
│  │  ├─ unb_pow2_div                      1.787 µs      │ 18.63 µs      │ 1.812 µs      │ 1.873 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div_ceil                 1.974 µs      │ 18.57 µs      │ 1.999 µs      │ 2.075 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div_ceil_const           1.074 µs      │ 2.462 µs      │ 1.099 µs      │ 1.115 µs      │ 1000    │ 16000
│  │  ├─ unb_pow2_div_ceil_reuse           1.524 µs      │ 3.174 µs      │ 1.537 µs      │ 1.555 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div_const                1.062 µs      │ 1.943 µs      │ 1.074 µs      │ 1.081 µs      │ 1000    │ 16000
│  │  ├─ unb_pow2_div_floor                1.181 µs      │ 2.337 µs      │ 1.193 µs      │ 1.205 µs      │ 1000    │ 16000
│  │  ├─ unb_pow2_div_floor_const          806 ns        │ 1.662 µs      │ 818.5 ns      │ 828.8 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_div_floor_reuse          1.093 µs      │ 2.093 µs      │ 1.099 µs      │ 1.112 µs      │ 1000    │ 16000
│  │  ├─ unb_pow2_div_reuse                1.349 µs      │ 2.224 µs      │ 1.362 µs      │ 1.364 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div_round                2.849 µs      │ 6.049 µs      │ 2.899 µs      │ 2.917 µs      │ 1000    │ 4000
│  │  ├─ unb_pow2_div_round_const          1.712 µs      │ 2.812 µs      │ 1.737 µs      │ 1.748 µs      │ 1000    │ 8000
│  │  ╰─ unb_pow2_div_round_reuse          2.362 µs      │ 4.287 µs      │ 2.374 µs      │ 2.394 µs      │ 1000    │ 8000
│  ├─ u8                                                 │               │               │               │         │
│  │  ├─ baseline_identity                 11.02 ns      │ 56.23 ns      │ 16.09 ns      │ 17.85 ns      │ 1000    │ 1024000
│  │  ├─ pow2_div                          34.94 ns      │ 118.1 ns      │ 39.63 ns      │ 40.27 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_ceil                     45.1 ns       │ 126.3 ns      │ 50.57 ns      │ 51.69 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_ceil_const               21.27 ns      │ 93.14 ns      │ 25.18 ns      │ 27.62 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_ceil_reuse               23.03 ns      │ 203.8 ns      │ 26.35 ns      │ 28 ns         │ 1000    │ 512000
│  │  ├─ pow2_div_const                    19.32 ns      │ 277.9 ns      │ 21.86 ns      │ 24.22 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_floor                    35.14 ns      │ 171 ns        │ 39.04 ns      │ 42.25 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_floor_const              19.51 ns      │ 65.02 ns      │ 21.66 ns      │ 23.31 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_floor_reuse              23.61 ns      │ 368.9 ns      │ 25.57 ns      │ 27.48 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_reuse                    23.42 ns      │ 69.12 ns      │ 25.76 ns      │ 27.55 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_round                    41.19 ns      │ 89.24 ns      │ 48.61 ns      │ 48.31 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_round_const              23.42 ns      │ 76.74 ns      │ 26.35 ns      │ 27.56 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_round_reuse              23.81 ns      │ 132 ns        │ 26.93 ns      │ 28.56 ns      │ 1000    │ 512000
│  │  ├─ std_div                           856 ns        │ 1.637 µs      │ 868.5 ns      │ 871.7 ns      │ 1000    │ 16000
│  │  ├─ std_div_const                     19.51 ns      │ 252.7 ns      │ 22.44 ns      │ 24.19 ns      │ 1000    │ 512000
│  │  ├─ std_div_reuse                     837.2 ns      │ 1.137 µs      │ 837.2 ns      │ 841.2 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_div                      32.21 ns      │ 151.3 ns      │ 39.24 ns      │ 39.22 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_ceil                 43.14 ns      │ 99.39 ns      │ 48.22 ns      │ 48.93 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_ceil_const           21.07 ns      │ 74.59 ns      │ 25.37 ns      │ 26.98 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_ceil_reuse           23.03 ns      │ 62.68 ns      │ 26.35 ns      │ 27.64 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_const                19.32 ns      │ 92.75 ns      │ 22.05 ns      │ 23.38 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_floor                38.65 ns      │ 124.3 ns      │ 51.15 ns      │ 54.16 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_floor_const          19.32 ns      │ 59.36 ns      │ 21.86 ns      │ 23.69 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_floor_reuse          23.42 ns      │ 97.64 ns      │ 25.57 ns      │ 27.67 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_reuse                23.42 ns      │ 70.1 ns       │ 25.57 ns      │ 27.45 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_round                35.33 ns      │ 70.1 ns       │ 41.58 ns      │ 42.34 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_round_const          23.42 ns      │ 82.79 ns      │ 26.54 ns      │ 28.12 ns      │ 1000    │ 512000
│  │  ╰─ unb_pow2_div_round_reuse          23.03 ns      │ 78.5 ns       │ 26.35 ns      │ 27.99 ns      │ 1000    │ 512000
│  ├─ u16                                                │               │               │               │         │
│  │  ├─ baseline_identity                 21.07 ns      │ 83.96 ns      │ 26.93 ns      │ 30.8 ns       │ 1000    │ 512000
│  │  ├─ pow2_div                          49.39 ns      │ 293.1 ns      │ 54.08 ns      │ 56.53 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_ceil                     66.97 ns      │ 240.8 ns      │ 75.18 ns      │ 83.49 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_ceil_const               43.14 ns      │ 181.4 ns      │ 49.79 ns      │ 52.43 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_ceil_reuse               46.66 ns      │ 274.7 ns      │ 52.52 ns      │ 54.91 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_const                    42.56 ns      │ 159.7 ns      │ 44.71 ns      │ 49.16 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_floor                    54.08 ns      │ 153.6 ns      │ 57.6 ns       │ 60.18 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_floor_const              37.29 ns      │ 113.8 ns      │ 40.8 ns       │ 43.02 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_floor_reuse              40.8 ns       │ 166.9 ns      │ 46.66 ns      │ 49.87 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_reuse                    40.41 ns      │ 699.3 ns      │ 45.88 ns      │ 49 ns         │ 1000    │ 256000
│  │  ├─ pow2_div_round                    69.32 ns      │ 769.3 ns      │ 78.69 ns      │ 85.9 ns       │ 1000    │ 256000
│  │  ├─ pow2_div_round_const              45.88 ns      │ 129.4 ns      │ 52.13 ns      │ 54.91 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_round_reuse              45.1 ns       │ 318.9 ns      │ 54.47 ns      │ 57.19 ns      │ 1000    │ 256000
│  │  ├─ std_div                           862.2 ns      │ 20.24 µs      │ 887.2 ns      │ 913.9 ns      │ 1000    │ 16000
│  │  ├─ std_div_const                     37.29 ns      │ 248.2 ns      │ 42.75 ns      │ 46.93 ns      │ 1000    │ 256000
│  │  ├─ std_div_reuse                     849.7 ns      │ 2.312 µs      │ 856 ns        │ 865 ns        │ 1000    │ 16000
│  │  ├─ unb_pow2_div                      56.04 ns      │ 148.6 ns      │ 59.55 ns      │ 63.14 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_ceil                 65.02 ns      │ 166.9 ns      │ 75.96 ns      │ 81.66 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_ceil_const           43.93 ns      │ 228.6 ns      │ 50.18 ns      │ 52.84 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_ceil_reuse           47.83 ns      │ 165.8 ns      │ 52.91 ns      │ 56 ns         │ 1000    │ 256000
│  │  ├─ unb_pow2_div_const                37.68 ns      │ 130.6 ns      │ 41.97 ns      │ 44 ns         │ 1000    │ 256000
│  │  ├─ unb_pow2_div_floor                56.04 ns      │ 194.3 ns      │ 63.46 ns      │ 69.42 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_floor_const          37.68 ns      │ 306.8 ns      │ 41.97 ns      │ 45.04 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_floor_reuse          40.8 ns       │ 172.8 ns      │ 45.88 ns      │ 49.93 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_reuse                41.19 ns      │ 137.2 ns      │ 45.49 ns      │ 49.13 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_round                63.85 ns      │ 169.3 ns      │ 75.57 ns      │ 81.17 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_round_const          42.36 ns      │ 150.1 ns      │ 52.13 ns      │ 54.92 ns      │ 1000    │ 256000
│  │  ╰─ unb_pow2_div_round_reuse          47.44 ns      │ 129.4 ns      │ 53.3 ns       │ 56.53 ns      │ 1000    │ 256000
│  ├─ u32                                                │               │               │               │         │
│  │  ├─ baseline_identity                 40.8 ns       │ 138.8 ns      │ 48.61 ns      │ 57.12 ns      │ 1000    │ 256000
│  │  ├─ pow2_div                          88.85 ns      │ 902.9 ns      │ 102.9 ns      │ 111.6 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_ceil                     115.4 ns      │ 403.6 ns      │ 141.1 ns      │ 148.4 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_ceil_const               82.6 ns       │ 262.2 ns      │ 91.97 ns      │ 97.23 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_ceil_reuse               87.29 ns      │ 484.9 ns      │ 97.44 ns      │ 103 ns        │ 1000    │ 128000
│  │  ├─ pow2_div_const                    74 ns         │ 549 ns        │ 81.82 ns      │ 89.51 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_floor                    95.88 ns      │ 341.1 ns      │ 118.5 ns      │ 124.3 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_floor_const              74 ns         │ 302.9 ns      │ 80.25 ns      │ 87.17 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_floor_reuse              78.69 ns      │ 334.9 ns      │ 84.94 ns      │ 91.59 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_reuse                    79.47 ns      │ 515.4 ns      │ 88.85 ns      │ 97.08 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_round                    121.6 ns      │ 416.1 ns      │ 141.9 ns      │ 148.9 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_round_const              80.25 ns      │ 248.2 ns      │ 91.97 ns      │ 98.26 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_round_reuse              85.72 ns      │ 465.4 ns      │ 97.44 ns      │ 103.6 ns      │ 1000    │ 128000
│  │  ├─ std_div                           1.312 µs      │ 1.862 µs      │ 1.337 µs      │ 1.34 µs       │ 1000    │ 8000
│  │  ├─ std_div_const                     74 ns         │ 226.3 ns      │ 81.04 ns      │ 87.45 ns      │ 1000    │ 128000
│  │  ├─ std_div_reuse                     1.274 µs      │ 1.799 µs      │ 1.287 µs      │ 1.294 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div                      95.88 ns      │ 429.4 ns      │ 124 ns        │ 129.8 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_ceil                 121.6 ns      │ 320.1 ns      │ 143.5 ns      │ 149 ns        │ 1000    │ 128000
│  │  ├─ unb_pow2_div_ceil_const           81.04 ns      │ 258.3 ns      │ 92.36 ns      │ 97.79 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_ceil_reuse           86.5 ns       │ 326.3 ns      │ 98.22 ns      │ 104.5 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_const                74 ns         │ 416.1 ns      │ 82.6 ns       │ 88.48 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_floor                95.88 ns      │ 351.3 ns      │ 116.9 ns      │ 122.6 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_floor_const          74 ns         │ 218.5 ns      │ 82.6 ns       │ 90.83 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_floor_reuse          80.25 ns      │ 1.765 µs      │ 103.6 ns      │ 111.6 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_reuse                79.47 ns      │ 303.6 ns      │ 90.41 ns      │ 95.18 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_round                120.1 ns      │ 354.4 ns      │ 138 ns        │ 144.2 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_round_const          79.47 ns      │ 277.1 ns      │ 90.41 ns      │ 96.59 ns      │ 1000    │ 128000
│  │  ╰─ unb_pow2_div_round_reuse          87.29 ns      │ 216.9 ns      │ 96.66 ns      │ 101.5 ns      │ 1000    │ 128000
│  ├─ u64                                                │               │               │               │         │
│  │  ├─ baseline_identity                 75.57 ns      │ 389.6 ns      │ 102.1 ns      │ 113 ns        │ 1000    │ 128000
│  │  ├─ pow2_div                          174.7 ns      │ 963.8 ns      │ 206 ns        │ 219.1 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_ceil                     224.7 ns      │ 616.9 ns      │ 251.3 ns      │ 262.6 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_ceil_const               165.4 ns      │ 593.5 ns      │ 193.5 ns      │ 203.1 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_ceil_reuse               174.7 ns      │ 563.8 ns      │ 201.3 ns      │ 210.8 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_const                    138.8 ns      │ 624.7 ns      │ 157.6 ns      │ 167.1 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_floor                    174.7 ns      │ 527.9 ns      │ 215.4 ns      │ 230.8 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_floor_const              152.1 ns      │ 510.7 ns      │ 170.1 ns      │ 177.2 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_floor_reuse              161.5 ns      │ 495.8 ns      │ 172.4 ns      │ 180.9 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_reuse                    163 ns        │ 1.277 µs      │ 181 ns        │ 187.6 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_round                    224.7 ns      │ 577.9 ns      │ 270.1 ns      │ 278.1 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_round_const              166.9 ns      │ 741.9 ns      │ 181 ns        │ 191.8 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_round_reuse              171.6 ns      │ 601.3 ns      │ 191.9 ns      │ 203.9 ns      │ 1000    │ 64000
│  │  ├─ std_div                           1.637 µs      │ 3.524 µs      │ 1.649 µs      │ 1.672 µs      │ 1000    │ 8000
│  │  ├─ std_div_const                     137.2 ns      │ 1.888 µs      │ 154.4 ns      │ 164.8 ns      │ 1000    │ 64000
│  │  ├─ std_div_reuse                     1.537 µs      │ 3.537 µs      │ 1.549 µs      │ 1.564 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div                      182.6 ns      │ 595.1 ns      │ 216.9 ns      │ 228.6 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil                 231 ns        │ 688.8 ns      │ 266.9 ns      │ 279.5 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil_const           160.7 ns      │ 510.7 ns      │ 190.4 ns      │ 198.9 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil_reuse           176.3 ns      │ 2.526 µs      │ 201.3 ns      │ 219.9 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_const                141.9 ns      │ 531 ns        │ 174.7 ns      │ 184.3 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor                181 ns        │ 568.5 ns      │ 224.7 ns      │ 237.1 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor_const          137.2 ns      │ 557.6 ns      │ 159.1 ns      │ 167.5 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor_reuse          148.2 ns      │ 556 ns        │ 171.6 ns      │ 182.1 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_reuse                146.6 ns      │ 709.1 ns      │ 173.2 ns      │ 183.7 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_round                229.4 ns      │ 574.7 ns      │ 254.4 ns      │ 269.8 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_round_const          168.5 ns      │ 2.659 µs      │ 198.2 ns      │ 211.3 ns      │ 1000    │ 64000
│  │  ╰─ unb_pow2_div_round_reuse          174.7 ns      │ 548.2 ns      │ 199.7 ns      │ 208.8 ns      │ 1000    │ 64000
│  ╰─ u128                                               │               │               │               │         │
│     ├─ baseline_identity                 184.1 ns      │ 676.3 ns      │ 198.2 ns      │ 212.5 ns      │ 1000    │ 64000
│     ├─ pow2_div                          1.181 µs      │ 11.11 µs      │ 1.218 µs      │ 1.283 µs      │ 1000    │ 16000
│     ├─ pow2_div_ceil                     1.849 µs      │ 3.224 µs      │ 1.874 µs      │ 1.923 µs      │ 1000    │ 8000
│     ├─ pow2_div_ceil_const               1.081 µs      │ 2.137 µs      │ 1.099 µs      │ 1.155 µs      │ 1000    │ 16000
│     ├─ pow2_div_ceil_reuse               1.462 µs      │ 2.337 µs      │ 1.487 µs      │ 1.524 µs      │ 1000    │ 8000
│     ├─ pow2_div_const                    799.7 ns      │ 1.937 µs      │ 818.5 ns      │ 858.4 ns      │ 1000    │ 16000
│     ├─ pow2_div_floor                    1.087 µs      │ 1.899 µs      │ 1.099 µs      │ 1.106 µs      │ 1000    │ 16000
│     ├─ pow2_div_floor_const              806 ns        │ 1.862 µs      │ 824.7 ns      │ 846.5 ns      │ 1000    │ 16000
│     ├─ pow2_div_floor_reuse              931 ns        │ 1.981 µs      │ 949.7 ns      │ 961.4 ns      │ 1000    │ 16000
│     ├─ pow2_div_reuse                    924.7 ns      │ 2.624 µs      │ 943.5 ns      │ 956.1 ns      │ 1000    │ 16000
│     ├─ pow2_div_round                    2.599 µs      │ 5.799 µs      │ 2.624 µs      │ 2.64 µs       │ 1000    │ 4000
│     ├─ pow2_div_round_const              1.031 µs      │ 2.012 µs      │ 1.043 µs      │ 1.053 µs      │ 1000    │ 16000
│     ├─ pow2_div_round_reuse              1.437 µs      │ 3.774 µs      │ 1.462 µs      │ 1.467 µs      │ 1000    │ 8000
│     ├─ std_div                           5.699 µs      │ 9.999 µs      │ 5.749 µs      │ 5.808 µs      │ 1000    │ 2000
│     ├─ std_div_const                     799.7 ns      │ 2.231 µs      │ 824.7 ns      │ 856.5 ns      │ 1000    │ 16000
│     ├─ std_div_reuse                     4.324 µs      │ 10.19 µs      │ 4.399 µs      │ 4.431 µs      │ 1000    │ 4000
│     ├─ unb_pow2_div                      1.168 µs      │ 1.862 µs      │ 1.181 µs      │ 1.192 µs      │ 1000    │ 16000
│     ├─ unb_pow2_div_ceil                 1.924 µs      │ 5.487 µs      │ 1.949 µs      │ 1.972 µs      │ 1000    │ 8000
│     ├─ unb_pow2_div_ceil_const           1.068 µs      │ 2.912 µs      │ 1.081 µs      │ 1.097 µs      │ 1000    │ 16000
│     ├─ unb_pow2_div_ceil_reuse           1.462 µs      │ 2.787 µs      │ 1.474 µs      │ 1.481 µs      │ 1000    │ 8000
│     ├─ unb_pow2_div_const                806 ns        │ 1.681 µs      │ 812.2 ns      │ 823.7 ns      │ 1000    │ 16000
│     ├─ unb_pow2_div_floor                1.168 µs      │ 2.749 µs      │ 1.181 µs      │ 1.215 µs      │ 1000    │ 16000
│     ├─ unb_pow2_div_floor_const          806 ns        │ 2.681 µs      │ 812.2 ns      │ 830.9 ns      │ 1000    │ 16000
│     ├─ unb_pow2_div_floor_reuse          931 ns        │ 2.356 µs      │ 943.5 ns      │ 953.2 ns      │ 1000    │ 16000
│     ├─ unb_pow2_div_reuse                931 ns        │ 1.762 µs      │ 943.5 ns      │ 949.1 ns      │ 1000    │ 16000
│     ├─ unb_pow2_div_round                2.649 µs      │ 7.699 µs      │ 2.699 µs      │ 2.731 µs      │ 1000    │ 2000
│     ├─ unb_pow2_div_round_const          1.043 µs      │ 2.849 µs      │ 1.081 µs      │ 1.116 µs      │ 1000    │ 16000
│     ╰─ unb_pow2_div_round_reuse          1.462 µs      │ 3.112 µs      │ 1.474 µs      │ 1.483 µs      │ 1000    │ 8000
├─ mul                                                   │               │               │               │         │
│  ├─ i8                                                 │               │               │               │         │
│  │  ├─ baseline_identity                 11.11 ns      │ 51.05 ns      │ 14.82 ns      │ 17.5 ns       │ 1000    │ 1024000
│  │  ├─ pow2_mul                          34.36 ns      │ 116.9 ns      │ 45.49 ns      │ 47.72 ns      │ 1000    │ 512000
│  │  ├─ pow2_mul_const                    19.51 ns      │ 80.25 ns      │ 22.44 ns      │ 24.3 ns       │ 1000    │ 512000
│  │  ├─ pow2_mul_reuse                    23.42 ns      │ 108.7 ns      │ 25.57 ns      │ 27.05 ns      │ 1000    │ 512000
│  │  ├─ std_mul                           36.11 ns      │ 95.49 ns      │ 39.82 ns      │ 42.27 ns      │ 1000    │ 512000
│  │  ├─ std_mul_const                     19.51 ns      │ 74.79 ns      │ 21.66 ns      │ 22.95 ns      │ 1000    │ 512000
│  │  ├─ std_mul_reuse                     22.64 ns      │ 66 ns         │ 25.37 ns      │ 28.11 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_mul                      34.94 ns      │ 103.3 ns      │ 36.89 ns      │ 40.22 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_mul_const                19.32 ns      │ 86.89 ns      │ 21.46 ns      │ 23.2 ns       │ 1000    │ 512000
│  │  ╰─ unb_pow2_mul_reuse                23.42 ns      │ 75.37 ns      │ 25.96 ns      │ 27.86 ns      │ 1000    │ 512000
│  ├─ i16                                                │               │               │               │         │
│  │  ├─ baseline_identity                 21.27 ns      │ 116 ns        │ 27.32 ns      │ 31.92 ns      │ 1000    │ 512000
│  │  ├─ pow2_mul                          53.69 ns      │ 153.3 ns      │ 57.6 ns       │ 59.49 ns      │ 1000    │ 256000
│  │  ├─ pow2_mul_const                    37.29 ns      │ 104.4 ns      │ 40.41 ns      │ 42.93 ns      │ 1000    │ 256000
│  │  ├─ pow2_mul_reuse                    41.19 ns      │ 219.3 ns      │ 46.66 ns      │ 50.7 ns       │ 1000    │ 256000
│  │  ├─ std_mul                           65.8 ns       │ 197 ns        │ 70.1 ns       │ 77.28 ns      │ 1000    │ 256000
│  │  ├─ std_mul_const                     37.68 ns      │ 145.4 ns      │ 41.19 ns      │ 43.93 ns      │ 1000    │ 256000
│  │  ├─ std_mul_reuse                     41.58 ns      │ 116.1 ns      │ 47.05 ns      │ 50.14 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_mul                      56.04 ns      │ 177.1 ns      │ 59.94 ns      │ 64.27 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_mul_const                37.68 ns      │ 122.8 ns      │ 43.14 ns      │ 46.35 ns      │ 1000    │ 256000
│  │  ╰─ unb_pow2_mul_reuse                41.19 ns      │ 175.5 ns      │ 47.05 ns      │ 50.18 ns      │ 1000    │ 256000
│  ├─ i32                                                │               │               │               │         │
│  │  ├─ baseline_identity                 40.8 ns       │ 199.3 ns      │ 54.47 ns      │ 60.89 ns      │ 1000    │ 256000
│  │  ├─ pow2_mul                          95.1 ns       │ 389.6 ns      │ 120.8 ns      │ 124.7 ns      │ 1000    │ 128000
│  │  ├─ pow2_mul_const                    74.79 ns      │ 955.2 ns      │ 81.82 ns      │ 88.92 ns      │ 1000    │ 128000
│  │  ├─ pow2_mul_reuse                    79.47 ns      │ 596.6 ns      │ 88.07 ns      │ 95.09 ns      │ 1000    │ 128000
│  │  ├─ std_mul                           125.5 ns      │ 377.9 ns      │ 149 ns        │ 166.5 ns      │ 1000    │ 128000
│  │  ├─ std_mul_const                     74 ns         │ 306 ns        │ 81.82 ns      │ 88.03 ns      │ 1000    │ 128000
│  │  ├─ std_mul_reuse                     78.69 ns      │ 352.9 ns      │ 87.29 ns      │ 94.75 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_mul                      95.88 ns      │ 323.2 ns      │ 110.7 ns      │ 117.8 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_mul_const                74 ns         │ 400.5 ns      │ 79.86 ns      │ 86.65 ns      │ 1000    │ 128000
│  │  ╰─ unb_pow2_mul_reuse                79.47 ns      │ 2.155 µs      │ 91.19 ns      │ 98.33 ns      │ 1000    │ 128000
│  ├─ i64                                                │               │               │               │         │
│  │  ├─ baseline_identity                 74 ns         │ 281 ns        │ 108.3 ns      │ 112.6 ns      │ 1000    │ 128000
│  │  ├─ pow2_mul                          176.3 ns      │ 2.787 µs      │ 210.7 ns      │ 224.8 ns      │ 1000    │ 64000
│  │  ├─ pow2_mul_const                    151.3 ns      │ 400.5 ns      │ 168.5 ns      │ 177.5 ns      │ 1000    │ 128000
│  │  ├─ pow2_mul_reuse                    160.7 ns      │ 349 ns        │ 174 ns        │ 181.2 ns      │ 1000    │ 128000
│  │  ├─ std_mul                           259.1 ns      │ 682.6 ns      │ 287.2 ns      │ 298.4 ns      │ 1000    │ 64000
│  │  ├─ std_mul_const                     152.9 ns      │ 441.1 ns      │ 181 ns        │ 188.4 ns      │ 1000    │ 128000
│  │  ├─ std_mul_reuse                     156 ns        │ 407.6 ns      │ 168.5 ns      │ 178.9 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_mul                      181 ns        │ 2.676 µs      │ 232.6 ns      │ 258.8 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_mul_const                138.8 ns      │ 495.1 ns      │ 167.7 ns      │ 181.1 ns      │ 1000    │ 64000
│  │  ╰─ unb_pow2_mul_reuse                148.2 ns      │ 526.3 ns      │ 174.7 ns      │ 182.8 ns      │ 1000    │ 64000
│  ├─ i128                                               │               │               │               │         │
│  │  ├─ baseline_identity                 204.4 ns      │ 752.9 ns      │ 238.8 ns      │ 248.7 ns      │ 1000    │ 64000
│  │  ├─ pow2_mul                          1.037 µs      │ 2.424 µs      │ 1.056 µs      │ 1.069 µs      │ 1000    │ 16000
│  │  ├─ pow2_mul_const                    799.7 ns      │ 1.956 µs      │ 812.2 ns      │ 828.4 ns      │ 1000    │ 16000
│  │  ├─ pow2_mul_reuse                    968.5 ns      │ 2.049 µs      │ 987.2 ns      │ 997.3 ns      │ 1000    │ 16000
│  │  ├─ std_mul                           1.237 µs      │ 2.324 µs      │ 1.287 µs      │ 1.296 µs      │ 1000    │ 8000
│  │  ├─ std_mul_const                     806 ns        │ 2.243 µs      │ 818.5 ns      │ 825.5 ns      │ 1000    │ 16000
│  │  ├─ std_mul_reuse                     824.7 ns      │ 1.437 µs      │ 837.2 ns      │ 843.9 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_mul                      1.168 µs      │ 2.618 µs      │ 1.181 µs      │ 1.194 µs      │ 1000    │ 16000
│  │  ├─ unb_pow2_mul_const                806 ns        │ 1.768 µs      │ 818.5 ns      │ 827.1 ns      │ 1000    │ 16000
│  │  ╰─ unb_pow2_mul_reuse                974.7 ns      │ 1.981 µs      │ 993.5 ns      │ 1.001 µs      │ 1000    │ 16000
│  ├─ u8                                                 │               │               │               │         │
│  │  ├─ baseline_identity                 11.31 ns      │ 49.79 ns      │ 15.02 ns      │ 17.05 ns      │ 1000    │ 1024000
│  │  ├─ pow2_mul                          34.36 ns      │ 238 ns        │ 39.63 ns      │ 47.31 ns      │ 1000    │ 512000
│  │  ├─ pow2_mul_const                    17.36 ns      │ 99 ns         │ 18.54 ns      │ 18.75 ns      │ 1000    │ 256000
│  │  ├─ pow2_mul_reuse                    23.22 ns      │ 106.6 ns      │ 25.57 ns      │ 27.7 ns       │ 1000    │ 512000
│  │  ├─ std_mul                           36.31 ns      │ 99.79 ns      │ 39.63 ns      │ 44.34 ns      │ 1000    │ 512000
│  │  ├─ std_mul_const                     19.32 ns      │ 84.55 ns      │ 22.05 ns      │ 23.98 ns      │ 1000    │ 512000
│  │  ├─ std_mul_reuse                     21.86 ns      │ 167.3 ns      │ 25.57 ns      │ 27.95 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_mul                      38.07 ns      │ 146.8 ns      │ 50.18 ns      │ 53.23 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_mul_const                19.51 ns      │ 83.18 ns      │ 22.05 ns      │ 23.52 ns      │ 1000    │ 512000
│  │  ╰─ unb_pow2_mul_reuse                23.22 ns      │ 77.32 ns      │ 25.37 ns      │ 27.67 ns      │ 1000    │ 512000
│  ├─ u16                                                │               │               │               │         │
│  │  ├─ baseline_identity                 22.44 ns      │ 103.6 ns      │ 32.79 ns      │ 36.03 ns      │ 1000    │ 512000
│  │  ├─ pow2_mul                          53.69 ns      │ 275.1 ns      │ 57.21 ns      │ 59.7 ns       │ 1000    │ 256000
│  │  ├─ pow2_mul_const                    37.29 ns      │ 166.9 ns      │ 40.8 ns       │ 44.81 ns      │ 1000    │ 256000
│  │  ├─ pow2_mul_reuse                    41.19 ns      │ 212.6 ns      │ 46.66 ns      │ 49.69 ns      │ 1000    │ 256000
│  │  ├─ std_mul                           64.24 ns      │ 206.4 ns      │ 69.32 ns      │ 76.98 ns      │ 1000    │ 256000
│  │  ├─ std_mul_const                     37.29 ns      │ 106.8 ns      │ 40.8 ns       │ 43.36 ns      │ 1000    │ 256000
│  │  ├─ std_mul_reuse                     41.58 ns      │ 143.5 ns      │ 48.22 ns      │ 53.81 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_mul                      56.04 ns      │ 470.8 ns      │ 60.33 ns      │ 68.5 ns       │ 1000    │ 256000
│  │  ├─ unb_pow2_mul_const                37.68 ns      │ 116.1 ns      │ 41.97 ns      │ 44.08 ns      │ 1000    │ 256000
│  │  ╰─ unb_pow2_mul_reuse                41.19 ns      │ 124.3 ns      │ 45.88 ns      │ 48.6 ns       │ 1000    │ 256000
│  ├─ u32                                                │               │               │               │         │
│  │  ├─ baseline_identity                 40.8 ns       │ 179.4 ns      │ 55.25 ns      │ 59.51 ns      │ 1000    │ 256000
│  │  ├─ pow2_mul                          93.54 ns      │ 977.9 ns      │ 116.1 ns      │ 123.7 ns      │ 1000    │ 128000
│  │  ├─ pow2_mul_const                    74 ns         │ 247.4 ns      │ 81.04 ns      │ 87.03 ns      │ 1000    │ 128000
│  │  ├─ pow2_mul_reuse                    78.69 ns      │ 486.5 ns      │ 87.29 ns      │ 94.42 ns      │ 1000    │ 128000
│  │  ├─ std_mul                           127.1 ns      │ 1.322 µs      │ 162.2 ns      │ 175.5 ns      │ 1000    │ 128000
│  │  ├─ std_mul_const                     79.08 ns      │ 232.2 ns      │ 84.94 ns      │ 90.77 ns      │ 1000    │ 256000
│  │  ├─ std_mul_reuse                     79.47 ns      │ 258.3 ns      │ 84.16 ns      │ 98.83 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_mul                      99.79 ns      │ 376.3 ns      │ 124 ns        │ 130.3 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_mul_const                79.08 ns      │ 284.9 ns      │ 84.94 ns      │ 93.02 ns      │ 1000    │ 256000
│  │  ╰─ unb_pow2_mul_reuse                78.69 ns      │ 278.6 ns      │ 86.5 ns       │ 91.41 ns      │ 1000    │ 128000
│  ├─ u64                                                │               │               │               │         │
│  │  ├─ baseline_identity                 74.79 ns      │ 270.1 ns      │ 101.3 ns      │ 111 ns        │ 1000    │ 128000
│  │  ├─ pow2_mul                          176.3 ns      │ 626.3 ns      │ 202.9 ns      │ 219.5 ns      │ 1000    │ 64000
│  │  ├─ pow2_mul_const                    138.8 ns      │ 570.1 ns      │ 160.7 ns      │ 171.3 ns      │ 1000    │ 64000
│  │  ├─ pow2_mul_reuse                    148.2 ns      │ 781 ns        │ 173.2 ns      │ 185.6 ns      │ 1000    │ 64000
│  │  ├─ std_mul                           257.6 ns      │ 779.4 ns      │ 279.4 ns      │ 300.6 ns      │ 1000    │ 64000
│  │  ├─ std_mul_const                     138.8 ns      │ 415.4 ns      │ 148.2 ns      │ 154 ns        │ 1000    │ 64000
│  │  ├─ std_mul_reuse                     157.6 ns      │ 560.7 ns      │ 187.2 ns      │ 208.8 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_mul                      181 ns        │ 2.915 µs      │ 210.7 ns      │ 231.6 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_mul_const                152.9 ns      │ 349 ns        │ 176.3 ns      │ 185.3 ns      │ 1000    │ 128000
│  │  ╰─ unb_pow2_mul_reuse                146.6 ns      │ 2.47 µs       │ 160.7 ns      │ 172.8 ns      │ 1000    │ 64000
│  ╰─ u128                                               │               │               │               │         │
│     ├─ baseline_identity                 187.2 ns      │ 532.6 ns      │ 202.9 ns      │ 222.1 ns      │ 1000    │ 64000
│     ├─ pow2_mul                          1.074 µs      │ 2.824 µs      │ 1.149 µs      │ 1.163 µs      │ 1000    │ 16000
│     ├─ pow2_mul_const                    818.5 ns      │ 1.649 µs      │ 912.2 ns      │ 925.1 ns      │ 1000    │ 16000
│     ├─ pow2_mul_reuse                    1.012 µs      │ 2.124 µs      │ 1.074 µs      │ 1.122 µs      │ 1000    │ 16000
│     ├─ std_mul                           1.224 µs      │ 3.624 µs      │ 1.349 µs      │ 1.36 µs       │ 1000    │ 8000
│     ├─ std_mul_const                     837.2 ns      │ 1.762 µs      │ 912.2 ns      │ 920.3 ns      │ 1000    │ 16000
│     ├─ std_mul_reuse                     837.2 ns      │ 2.274 µs      │ 924.7 ns      │ 940.5 ns      │ 1000    │ 16000
│     ├─ unb_pow2_mul                      1.162 µs      │ 2.187 µs      │ 1.174 µs      │ 1.204 µs      │ 1000    │ 16000
│     ├─ unb_pow2_mul_const                806 ns        │ 1.981 µs      │ 887.2 ns      │ 885.7 ns      │ 1000    │ 16000
│     ╰─ unb_pow2_mul_reuse                981 ns        │ 2.056 µs      │ 993.5 ns      │ 1.004 µs      │ 1000    │ 16000
├─ rem                                                   │               │               │               │         │
│  ├─ i8                                                 │               │               │               │         │
│  │  ├─ baseline_identity                 11.31 ns      │ 55.94 ns      │ 16.29 ns      │ 17.99 ns      │ 1000    │ 1024000
│  │  ├─ pow2_is_multiple_of               33.57 ns      │ 92.17 ns      │ 36.5 ns       │ 39.22 ns      │ 1000    │ 512000
│  │  ├─ pow2_is_multiple_of_const         19.71 ns      │ 93.34 ns      │ 23.03 ns      │ 25.81 ns      │ 1000    │ 512000
│  │  ├─ pow2_is_multiple_of_reuse         22.64 ns      │ 110.3 ns      │ 26.93 ns      │ 28.78 ns      │ 1000    │ 512000
│  │  ├─ pow2_rem                          54.08 ns      │ 105.2 ns      │ 58.77 ns      │ 59.42 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem_const                    22.83 ns      │ 69.51 ns      │ 25.76 ns      │ 27.44 ns      │ 1000    │ 512000
│  │  ├─ pow2_rem_floor                    33.38 ns      │ 99.59 ns      │ 41.78 ns      │ 44.06 ns      │ 1000    │ 512000
│  │  ├─ pow2_rem_floor_const              19.12 ns      │ 75.18 ns      │ 21.46 ns      │ 23.23 ns      │ 1000    │ 512000
│  │  ├─ pow2_rem_floor_reuse              23.03 ns      │ 330.2 ns      │ 25.37 ns      │ 28.19 ns      │ 1000    │ 512000
│  │  ├─ pow2_rem_reuse                    28.89 ns      │ 99.79 ns      │ 32.79 ns      │ 34.79 ns      │ 1000    │ 512000
│  │  ├─ std_rem                           862.2 ns      │ 1.749 µs      │ 881 ns        │ 892.3 ns      │ 1000    │ 16000
│  │  ├─ std_rem_const                     23.22 ns      │ 73.22 ns      │ 26.35 ns      │ 28.46 ns      │ 1000    │ 512000
│  │  ├─ std_rem_reuse                     843.5 ns      │ 2.131 µs      │ 849.7 ns      │ 855.8 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_is_multiple_of           36.7 ns       │ 132 ns        │ 46.07 ns      │ 48.45 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_is_multiple_of_const     19.9 ns       │ 73.22 ns      │ 22.44 ns      │ 24.48 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_is_multiple_of_reuse     23.61 ns      │ 70.88 ns      │ 25.96 ns      │ 27.69 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_rem                      40.02 ns      │ 107.2 ns      │ 41.58 ns      │ 42.22 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_rem_const                22.83 ns      │ 78.3 ns       │ 25.96 ns      │ 27.86 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_rem_floor                38.65 ns      │ 170.2 ns      │ 46.86 ns      │ 50.41 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_rem_floor_const          19.12 ns      │ 71.27 ns      │ 21.46 ns      │ 22.5 ns       │ 1000    │ 512000
│  │  ├─ unb_pow2_rem_floor_reuse          23.03 ns      │ 89.24 ns      │ 25.18 ns      │ 27.61 ns      │ 1000    │ 512000
│  │  ╰─ unb_pow2_rem_reuse                24.2 ns       │ 81.04 ns      │ 26.93 ns      │ 28.71 ns      │ 1000    │ 512000
│  ├─ i16                                                │               │               │               │         │
│  │  ├─ baseline_identity                 18.14 ns      │ 105.6 ns      │ 23.61 ns      │ 26.23 ns      │ 1000    │ 256000
│  │  ├─ pow2_is_multiple_of               47.83 ns      │ 532.2 ns      │ 54.08 ns      │ 57.9 ns       │ 1000    │ 256000
│  │  ├─ pow2_is_multiple_of_const         27.71 ns      │ 101.1 ns      │ 36.89 ns      │ 38.97 ns      │ 1000    │ 512000
│  │  ├─ pow2_is_multiple_of_reuse         30.25 ns      │ 72.44 ns      │ 32.6 ns       │ 33.77 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem                          78.3 ns       │ 440 ns        │ 90.41 ns      │ 94.64 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem_const                    46.66 ns      │ 125.5 ns      │ 53.3 ns       │ 56.19 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem_floor                    56.04 ns      │ 158.3 ns      │ 65.8 ns       │ 70.1 ns       │ 1000    │ 256000
│  │  ├─ pow2_rem_floor_const              38.46 ns      │ 147.4 ns      │ 42.36 ns      │ 46.2 ns       │ 1000    │ 256000
│  │  ├─ pow2_rem_floor_reuse              40.8 ns       │ 122 ns        │ 45.49 ns      │ 48.9 ns       │ 1000    │ 256000
│  │  ├─ pow2_rem_reuse                    51.35 ns      │ 165 ns        │ 56.82 ns      │ 60.24 ns      │ 1000    │ 256000
│  │  ├─ std_rem                           874.7 ns      │ 1.231 µs      │ 893.5 ns      │ 900 ns        │ 1000    │ 16000
│  │  ├─ std_rem_const                     45.88 ns      │ 131 ns        │ 54.08 ns      │ 56.68 ns      │ 1000    │ 256000
│  │  ├─ std_rem_reuse                     2.524 µs      │ 6.649 µs      │ 2.549 µs      │ 2.561 µs      │ 1000    │ 4000
│  │  ├─ unb_pow2_is_multiple_of           49.79 ns      │ 197 ns        │ 57.6 ns       │ 62.29 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_is_multiple_of_const     27.52 ns      │ 130 ns        │ 34.16 ns      │ 37.91 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_is_multiple_of_reuse     37.68 ns      │ 128.5 ns      │ 41.58 ns      │ 45.38 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_rem                      65.41 ns      │ 211.5 ns      │ 77.52 ns      │ 83.03 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_rem_const                44.71 ns      │ 445.1 ns      │ 52.91 ns      │ 55.82 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_rem_floor                56.82 ns      │ 458.7 ns      │ 66.97 ns      │ 71.88 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_rem_floor_const          38.07 ns      │ 411.5 ns      │ 42.75 ns      │ 45.81 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_rem_floor_reuse          41.19 ns      │ 173.6 ns      │ 47.05 ns      │ 52.02 ns      │ 1000    │ 256000
│  │  ╰─ unb_pow2_rem_reuse                45.49 ns      │ 191.1 ns      │ 56.04 ns      │ 60.02 ns      │ 1000    │ 256000
│  ├─ i32                                                │               │               │               │         │
│  │  ├─ baseline_identity                 41.19 ns      │ 235.7 ns      │ 48.22 ns      │ 58.1 ns       │ 1000    │ 256000
│  │  ├─ pow2_is_multiple_of               60.72 ns      │ 152.1 ns      │ 64.63 ns      │ 67.61 ns      │ 1000    │ 128000
│  │  ├─ pow2_is_multiple_of_const         45.1 ns       │ 193.9 ns      │ 54.47 ns      │ 60.16 ns      │ 1000    │ 256000
│  │  ├─ pow2_is_multiple_of_reuse         62.68 ns      │ 690.8 ns      │ 67.75 ns      │ 73.73 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem                          128.6 ns      │ 388 ns        │ 147.4 ns      │ 155.4 ns      │ 1000    │ 128000
│  │  ├─ pow2_rem_const                    84.16 ns      │ 270.1 ns      │ 95.1 ns       │ 100.5 ns      │ 1000    │ 128000
│  │  ├─ pow2_rem_floor                    98.22 ns      │ 281 ns        │ 120.8 ns      │ 126.2 ns      │ 1000    │ 128000
│  │  ├─ pow2_rem_floor_const              74.79 ns      │ 313.8 ns      │ 83.38 ns      │ 90.47 ns      │ 1000    │ 128000
│  │  ├─ pow2_rem_floor_reuse              78.69 ns      │ 965.4 ns      │ 85.72 ns      │ 93.66 ns      │ 1000    │ 128000
│  │  ├─ pow2_rem_reuse                    95.88 ns      │ 256 ns        │ 105.2 ns      │ 110.8 ns      │ 1000    │ 128000
│  │  ├─ std_rem                           1.337 µs      │ 1.874 µs      │ 1.349 µs      │ 1.352 µs      │ 1000    │ 8000
│  │  ├─ std_rem_const                     83.38 ns      │ 298.2 ns      │ 100.5 ns      │ 107.6 ns      │ 1000    │ 128000
│  │  ├─ std_rem_reuse                     1.287 µs      │ 4.187 µs      │ 1.312 µs      │ 1.314 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_is_multiple_of           63.85 ns      │ 316.9 ns      │ 70.1 ns       │ 72.36 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_is_multiple_of_const     44.32 ns      │ 165.4 ns      │ 52.13 ns      │ 58.71 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_is_multiple_of_reuse     62.68 ns      │ 194.3 ns      │ 72.83 ns      │ 77.71 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_rem                      123.2 ns      │ 314.6 ns      │ 149 ns        │ 156.5 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_rem_const                85.72 ns      │ 215.4 ns      │ 93.54 ns      │ 99.51 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_rem_floor                102.1 ns      │ 1.245 µs      │ 128.6 ns      │ 136.1 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_rem_floor_const          74.79 ns      │ 252.1 ns      │ 81.04 ns      │ 89.7 ns       │ 1000    │ 128000
│  │  ├─ unb_pow2_rem_floor_reuse          77.91 ns      │ 296.6 ns      │ 85.72 ns      │ 93.19 ns      │ 1000    │ 128000
│  │  ╰─ unb_pow2_rem_reuse                87.29 ns      │ 268.5 ns      │ 99.79 ns      │ 105.8 ns      │ 1000    │ 128000
│  ├─ i64                                                │               │               │               │         │
│  │  ├─ baseline_identity                 74 ns         │ 316.1 ns      │ 93.54 ns      │ 105.9 ns      │ 1000    │ 128000
│  │  ├─ pow2_is_multiple_of               125.5 ns      │ 374.7 ns      │ 149 ns        │ 162.4 ns      │ 1000    │ 128000
│  │  ├─ pow2_is_multiple_of_const         83.38 ns      │ 383.3 ns      │ 123.2 ns      │ 127.4 ns      │ 1000    │ 128000
│  │  ├─ pow2_is_multiple_of_reuse         108.3 ns      │ 256.8 ns      │ 123.2 ns      │ 130.7 ns      │ 1000    │ 128000
│  │  ├─ pow2_rem                          238.8 ns      │ 787.2 ns      │ 282.6 ns      │ 294.8 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem_const                    173.2 ns      │ 704.4 ns      │ 199.7 ns      │ 213.2 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem_floor                    184.1 ns      │ 1.816 µs      │ 206 ns        │ 218.9 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem_floor_const              153.6 ns      │ 1.199 µs      │ 175.5 ns      │ 186.5 ns      │ 1000    │ 128000
│  │  ├─ pow2_rem_floor_reuse              163 ns        │ 1.08 µs       │ 181 ns        │ 190 ns        │ 1000    │ 128000
│  │  ├─ pow2_rem_reuse                    188.8 ns      │ 506 ns        │ 207.6 ns      │ 216.7 ns      │ 1000    │ 64000
│  │  ├─ std_rem                           1.637 µs      │ 3.287 µs      │ 1.662 µs      │ 1.683 µs      │ 1000    │ 8000
│  │  ├─ std_rem_const                     179.4 ns      │ 521.6 ns      │ 196.6 ns      │ 208.8 ns      │ 1000    │ 64000
│  │  ├─ std_rem_reuse                     1.524 µs      │ 2.999 µs      │ 1.549 µs      │ 1.552 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_is_multiple_of           129.4 ns      │ 474.7 ns      │ 157.6 ns      │ 167 ns        │ 1000    │ 128000
│  │  ├─ unb_pow2_is_multiple_of_const     83.38 ns      │ 270.8 ns      │ 112.2 ns      │ 117.1 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_is_multiple_of_reuse     110.7 ns      │ 275.5 ns      │ 124 ns        │ 130.5 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_rem                      241.9 ns      │ 1.041 µs      │ 268.5 ns      │ 286.5 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_rem_const                173.2 ns      │ 601.3 ns      │ 198.2 ns      │ 208.2 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_rem_floor                188.8 ns      │ 618.5 ns      │ 213.8 ns      │ 226.3 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_rem_floor_const          140.4 ns      │ 545.1 ns      │ 174.7 ns      │ 181.4 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_rem_floor_reuse          146.6 ns      │ 443.5 ns      │ 170.1 ns      │ 181.6 ns      │ 1000    │ 64000
│  │  ╰─ unb_pow2_rem_reuse                181 ns        │ 496.6 ns      │ 199.7 ns      │ 211.4 ns      │ 1000    │ 64000
│  ├─ i128                                               │               │               │               │         │
│  │  ├─ baseline_identity                 202.9 ns      │ 562.2 ns      │ 224.7 ns      │ 241.9 ns      │ 1000    │ 64000
│  │  ├─ pow2_is_multiple_of               399.7 ns      │ 1.456 µs      │ 406 ns        │ 428.2 ns      │ 1000    │ 16000
│  │  ├─ pow2_is_multiple_of_const         334.1 ns      │ 756 ns        │ 340.4 ns      │ 341.8 ns      │ 1000    │ 32000
│  │  ├─ pow2_is_multiple_of_reuse         377.9 ns      │ 1.574 µs      │ 384.1 ns      │ 390.4 ns      │ 1000    │ 32000
│  │  ├─ pow2_rem                          2.624 µs      │ 8.924 µs      │ 2.674 µs      │ 2.682 µs      │ 1000    │ 4000
│  │  ├─ pow2_rem_const                    899.7 ns      │ 1.593 µs      │ 906 ns        │ 916.5 ns      │ 1000    │ 16000
│  │  ├─ pow2_rem_floor                    812.2 ns      │ 2.018 µs      │ 824.7 ns      │ 833.2 ns      │ 1000    │ 16000
│  │  ├─ pow2_rem_floor_const              437.2 ns      │ 3.724 µs      │ 518.5 ns      │ 531.2 ns      │ 1000    │ 32000
│  │  ├─ pow2_rem_floor_reuse              552.9 ns      │ 4.04 µs       │ 593.5 ns      │ 631.6 ns      │ 1000    │ 32000
│  │  ├─ pow2_rem_reuse                    2.187 µs      │ 20.49 µs      │ 2.199 µs      │ 2.265 µs      │ 1000    │ 8000
│  │  ├─ std_rem                           9.199 µs      │ 142.1 µs      │ 9.299 µs      │ 9.905 µs      │ 1000    │ 1000
│  │  ├─ std_rem_const                     824.7 ns      │ 1.874 µs      │ 918.5 ns      │ 932 ns        │ 1000    │ 16000
│  │  ├─ std_rem_reuse                     8.099 µs      │ 14.99 µs      │ 8.199 µs      │ 8.255 µs      │ 1000    │ 2000
│  │  ├─ unb_pow2_is_multiple_of           393.5 ns      │ 931 ns        │ 399.7 ns      │ 407.5 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_is_multiple_of_const     334.1 ns      │ 715.4 ns      │ 340.4 ns      │ 343.5 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_is_multiple_of_reuse     377.9 ns      │ 784.1 ns      │ 384.1 ns      │ 386.3 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_rem                      1.649 µs      │ 3.562 µs      │ 1.674 µs      │ 1.687 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_rem_const                893.5 ns      │ 1.943 µs      │ 918.5 ns      │ 953.7 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_rem_floor                818.5 ns      │ 1.218 µs      │ 831 ns        │ 840.8 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_rem_floor_const          440.4 ns      │ 1.221 µs      │ 490.4 ns      │ 521.5 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_rem_floor_reuse          552.9 ns      │ 4.046 µs      │ 584.1 ns      │ 620.5 ns      │ 1000    │ 32000
│  │  ╰─ unb_pow2_rem_reuse                1.049 µs      │ 2.781 µs      │ 1.137 µs      │ 1.154 µs      │ 1000    │ 16000
│  ├─ u8                                                 │               │               │               │         │
│  │  ├─ baseline_identity                 11.41 ns      │ 46.17 ns      │ 15.36 ns      │ 17.03 ns      │ 1000    │ 1024000
│  │  ├─ pow2_is_multiple_of               33.38 ns      │ 114.2 ns      │ 36.31 ns      │ 39.31 ns      │ 1000    │ 512000
│  │  ├─ pow2_is_multiple_of_const         19.71 ns      │ 211.5 ns      │ 22.05 ns      │ 23.44 ns      │ 1000    │ 512000
│  │  ├─ pow2_is_multiple_of_reuse         23.61 ns      │ 250.5 ns      │ 26.15 ns      │ 28.04 ns      │ 1000    │ 512000
│  │  ├─ pow2_rem                          45.49 ns      │ 118.1 ns      │ 54.47 ns      │ 54.18 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem_const                    19.32 ns      │ 70.1 ns       │ 21.46 ns      │ 23.15 ns      │ 1000    │ 512000
│  │  ├─ pow2_rem_floor                    33.18 ns      │ 109.3 ns      │ 40.21 ns      │ 43.85 ns      │ 1000    │ 512000
│  │  ├─ pow2_rem_floor_const              19.12 ns      │ 63.65 ns      │ 22.05 ns      │ 24.14 ns      │ 1000    │ 512000
│  │  ├─ pow2_rem_floor_reuse              23.22 ns      │ 75.18 ns      │ 25.18 ns      │ 27.34 ns      │ 1000    │ 512000
│  │  ├─ pow2_rem_reuse                    23.61 ns      │ 64.82 ns      │ 25.96 ns      │ 27.87 ns      │ 1000    │ 512000
│  │  ├─ std_rem                           849.7 ns      │ 1.762 µs      │ 862.2 ns      │ 866.7 ns      │ 1000    │ 16000
│  │  ├─ std_rem_const                     19.32 ns      │ 87.48 ns      │ 21.66 ns      │ 23.69 ns      │ 1000    │ 512000
│  │  ├─ std_rem_reuse                     837.2 ns      │ 2.893 µs      │ 849.7 ns      │ 856.4 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_is_multiple_of           36.5 ns       │ 361.5 ns      │ 39.24 ns      │ 44.15 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_is_multiple_of_const     19.9 ns       │ 63.85 ns      │ 21.86 ns      │ 23.96 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_is_multiple_of_reuse     23.61 ns      │ 85.33 ns      │ 26.15 ns      │ 27.9 ns       │ 1000    │ 512000
│  │  ├─ unb_pow2_rem                      37.29 ns      │ 110.3 ns      │ 38.85 ns      │ 39.59 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_rem_const                19.12 ns      │ 99.39 ns      │ 22.05 ns      │ 24.49 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_rem_floor                36.89 ns      │ 135.3 ns      │ 38.46 ns      │ 39.68 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_rem_floor_const          19.12 ns      │ 71.86 ns      │ 21.27 ns      │ 23.48 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_rem_floor_reuse          23.22 ns      │ 74.79 ns      │ 25.18 ns      │ 27.11 ns      │ 1000    │ 512000
│  │  ╰─ unb_pow2_rem_reuse                22.83 ns      │ 95.88 ns      │ 24.98 ns      │ 26.72 ns      │ 1000    │ 512000
│  ├─ u16                                                │               │               │               │         │
│  │  ├─ baseline_identity                 21.46 ns      │ 105.2 ns      │ 31.62 ns      │ 36.73 ns      │ 1000    │ 512000
│  │  ├─ pow2_is_multiple_of               47.05 ns      │ 178.3 ns      │ 55.64 ns      │ 58.91 ns      │ 1000    │ 256000
│  │  ├─ pow2_is_multiple_of_const         27.91 ns      │ 90.41 ns      │ 32.21 ns      │ 36.02 ns      │ 1000    │ 512000
│  │  ├─ pow2_is_multiple_of_reuse         38.26 ns      │ 142.7 ns      │ 42.46 ns      │ 46.79 ns      │ 1000    │ 512000
│  │  ├─ pow2_rem                          58.77 ns      │ 159.9 ns      │ 65.8 ns       │ 71.41 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem_const                    38.07 ns      │ 141.1 ns      │ 42.36 ns      │ 45.85 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem_floor                    56.43 ns      │ 189.6 ns      │ 62.29 ns      │ 68.43 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem_floor_const              42.95 ns      │ 219.7 ns      │ 44.9 ns       │ 49.68 ns      │ 1000    │ 512000
│  │  ├─ pow2_rem_floor_reuse              40.41 ns      │ 154 ns        │ 45.49 ns      │ 49.65 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem_reuse                    43.54 ns      │ 160.3 ns      │ 49.79 ns      │ 54.17 ns      │ 1000    │ 256000
│  │  ├─ std_rem                           868.5 ns      │ 1.262 µs      │ 918.5 ns      │ 921.4 ns      │ 1000    │ 16000
│  │  ├─ std_rem_const                     38.07 ns      │ 483.7 ns      │ 43.54 ns      │ 47.16 ns      │ 1000    │ 256000
│  │  ├─ std_rem_reuse                     849.7 ns      │ 1.137 µs      │ 856 ns        │ 860.5 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_is_multiple_of           51.35 ns      │ 180.2 ns      │ 57.6 ns       │ 61.06 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_is_multiple_of_const     27.71 ns      │ 82.99 ns      │ 33.96 ns      │ 38 ns         │ 1000    │ 512000
│  │  ├─ unb_pow2_is_multiple_of_reuse     38.26 ns      │ 95.49 ns      │ 42.17 ns      │ 45.26 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_rem                      56.82 ns      │ 182.6 ns      │ 61.5 ns       │ 67.34 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_rem_const                38.07 ns      │ 138.4 ns      │ 42.75 ns      │ 45.84 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_rem_floor                57.21 ns      │ 256 ns        │ 65.8 ns       │ 71.82 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_rem_floor_const          38.07 ns      │ 153.6 ns      │ 42.36 ns      │ 45.26 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_rem_floor_reuse          41.58 ns      │ 136.8 ns      │ 46.66 ns      │ 49.88 ns      │ 1000    │ 256000
│  │  ╰─ unb_pow2_rem_reuse                40.8 ns       │ 447.4 ns      │ 45.1 ns       │ 48.68 ns      │ 1000    │ 256000
│  ├─ u32                                                │               │               │               │         │
│  │  ├─ baseline_identity                 41.19 ns      │ 355.6 ns      │ 54.86 ns      │ 61.09 ns      │ 1000    │ 256000
│  │  ├─ pow2_is_multiple_of               73.22 ns      │ 286.5 ns      │ 88.07 ns      │ 97.25 ns      │ 1000    │ 256000
│  │  ├─ pow2_is_multiple_of_const         44.71 ns      │ 141.9 ns      │ 50.96 ns      │ 57.59 ns      │ 1000    │ 256000
│  │  ├─ pow2_is_multiple_of_reuse         61.5 ns       │ 163.4 ns      │ 69.32 ns      │ 74.24 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem                          108.3 ns      │ 307.6 ns      │ 124 ns        │ 131.1 ns      │ 1000    │ 128000
│  │  ├─ pow2_rem_const                    74 ns         │ 1.406 µs      │ 81.82 ns      │ 89.22 ns      │ 1000    │ 128000
│  │  ├─ pow2_rem_floor                    99.79 ns      │ 314.6 ns      │ 121.6 ns      │ 127.9 ns      │ 1000    │ 128000
│  │  ├─ pow2_rem_floor_const              80.64 ns      │ 281.8 ns      │ 85.72 ns      │ 93.98 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem_floor_reuse              78.69 ns      │ 223.2 ns      │ 82.6 ns       │ 86.44 ns      │ 1000    │ 128000
│  │  ├─ pow2_rem_reuse                    84.16 ns      │ 1.087 µs      │ 88.85 ns      │ 93.97 ns      │ 1000    │ 128000
│  │  ├─ std_rem                           1.312 µs      │ 2.487 µs      │ 1.324 µs      │ 1.333 µs      │ 1000    │ 8000
│  │  ├─ std_rem_const                     74.79 ns      │ 228.6 ns      │ 84.94 ns      │ 93.4 ns       │ 1000    │ 128000
│  │  ├─ std_rem_reuse                     1.274 µs      │ 2.012 µs      │ 1.299 µs      │ 1.305 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_is_multiple_of           63.85 ns      │ 176.3 ns      │ 71.66 ns      │ 72.95 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_is_multiple_of_const     44.71 ns      │ 152.9 ns      │ 52.91 ns      │ 58.84 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_is_multiple_of_reuse     61.89 ns      │ 212.2 ns      │ 72.44 ns      │ 78.23 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_rem                      99.79 ns      │ 387.2 ns      │ 121.6 ns      │ 127.9 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_rem_const                74.79 ns      │ 288.8 ns      │ 81.04 ns      │ 89.73 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_rem_floor                99 ns         │ 420.8 ns      │ 120.8 ns      │ 127.8 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_rem_floor_const          74.79 ns      │ 304.4 ns      │ 82.6 ns       │ 88.63 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_rem_floor_reuse          79.47 ns      │ 275.5 ns      │ 87.29 ns      │ 96.6 ns       │ 1000    │ 128000
│  │  ╰─ unb_pow2_rem_reuse                78.69 ns      │ 352.9 ns      │ 86.5 ns       │ 94.15 ns      │ 1000    │ 128000
│  ├─ u64                                                │               │               │               │         │
│  │  ├─ baseline_identity                 74.79 ns      │ 291.9 ns      │ 110.7 ns      │ 116.4 ns      │ 1000    │ 128000
│  │  ├─ pow2_is_multiple_of               124.7 ns      │ 356.8 ns      │ 149.7 ns      │ 161.3 ns      │ 1000    │ 128000
│  │  ├─ pow2_is_multiple_of_const         84.94 ns      │ 1.453 µs      │ 116.1 ns      │ 124.2 ns      │ 1000    │ 128000
│  │  ├─ pow2_is_multiple_of_reuse         106.8 ns      │ 392.7 ns      │ 123.2 ns      │ 132.8 ns      │ 1000    │ 128000
│  │  ├─ pow2_rem                          204.4 ns      │ 562.2 ns      │ 227.9 ns      │ 241.1 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem_const                    140.4 ns      │ 945.1 ns      │ 154.4 ns      │ 165.7 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem_floor                    182.6 ns      │ 534.1 ns      │ 204.4 ns      │ 218.9 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem_floor_const              152.1 ns      │ 492.7 ns      │ 173.2 ns      │ 183.2 ns      │ 1000    │ 128000
│  │  ├─ pow2_rem_floor_reuse              162.2 ns      │ 372.4 ns      │ 174.7 ns      │ 185 ns        │ 1000    │ 128000
│  │  ├─ pow2_rem_reuse                    159.1 ns      │ 470.1 ns      │ 182.6 ns      │ 196.1 ns      │ 1000    │ 64000
│  │  ├─ std_rem                           1.637 µs      │ 2.262 µs      │ 1.662 µs      │ 1.672 µs      │ 1000    │ 8000
│  │  ├─ std_rem_const                     152.1 ns      │ 477.9 ns      │ 177.1 ns      │ 185.3 ns      │ 1000    │ 128000
│  │  ├─ std_rem_reuse                     1.524 µs      │ 4.799 µs      │ 1.562 µs      │ 1.575 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_is_multiple_of           113.8 ns      │ 252.9 ns      │ 124.7 ns      │ 133.3 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_is_multiple_of_const     83.38 ns      │ 306.8 ns      │ 113 ns        │ 119.2 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_is_multiple_of_reuse     109.9 ns      │ 255.2 ns      │ 120.8 ns      │ 130 ns        │ 1000    │ 128000
│  │  ├─ unb_pow2_rem                      187.2 ns      │ 643.5 ns      │ 212.2 ns      │ 228.3 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_rem_const                152.9 ns      │ 480.2 ns      │ 170.8 ns      │ 183.3 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_rem_floor                162.2 ns      │ 1.39 µs       │ 177.9 ns      │ 186.9 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_rem_floor_const          140.4 ns      │ 526.3 ns      │ 152.9 ns      │ 165.7 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_rem_floor_reuse          146.6 ns      │ 449.7 ns      │ 162.2 ns      │ 172 ns        │ 1000    │ 64000
│  │  ╰─ unb_pow2_rem_reuse                146.6 ns      │ 666.9 ns      │ 160.7 ns      │ 172.1 ns      │ 1000    │ 64000
│  ╰─ u128                                               │               │               │               │         │
│     ├─ baseline_identity                 187.2 ns      │ 610.7 ns      │ 212.2 ns      │ 224.3 ns      │ 1000    │ 64000
│     ├─ pow2_is_multiple_of               396.6 ns      │ 746.6 ns      │ 402.9 ns      │ 410 ns        │ 1000    │ 32000
│     ├─ pow2_is_multiple_of_const         334.1 ns      │ 896.6 ns      │ 343.5 ns      │ 347 ns        │ 1000    │ 32000
│     ├─ pow2_is_multiple_of_reuse         377.9 ns      │ 1.396 µs      │ 384.1 ns      │ 388.8 ns      │ 1000    │ 32000
│     ├─ pow2_rem                          2.049 µs      │ 5.324 µs      │ 2.074 µs      │ 2.096 µs      │ 1000    │ 8000
│     ├─ pow2_rem_const                    415.4 ns      │ 1.146 µs      │ 477.9 ns      │ 496.9 ns      │ 1000    │ 32000
│     ├─ pow2_rem_floor                    849.7 ns      │ 2.662 µs      │ 912.2 ns      │ 928.2 ns      │ 1000    │ 16000
│     ├─ pow2_rem_floor_const              421.6 ns      │ 10.78 µs      │ 496.6 ns      │ 524.4 ns      │ 1000    │ 32000
│     ├─ pow2_rem_floor_reuse              552.9 ns      │ 1.527 µs      │ 621.6 ns      │ 650.4 ns      │ 1000    │ 32000
│     ├─ pow2_rem_reuse                    1.674 µs      │ 3.949 µs      │ 1.687 µs      │ 1.705 µs      │ 1000    │ 8000
│     ├─ std_rem                           4.124 µs      │ 6.199 µs      │ 4.149 µs      │ 4.18 µs       │ 1000    │ 4000
│     ├─ std_rem_const                     418.5 ns      │ 1.34 µs       │ 474.7 ns      │ 494.7 ns      │ 1000    │ 32000
│     ├─ std_rem_reuse                     4.224 µs      │ 6.499 µs      │ 4.274 µs      │ 4.291 µs      │ 1000    │ 4000
│     ├─ unb_pow2_is_multiple_of           393.5 ns      │ 1.159 µs      │ 399.7 ns      │ 412.6 ns      │ 1000    │ 32000
│     ├─ unb_pow2_is_multiple_of_const     331 ns        │ 1.043 µs      │ 340.4 ns      │ 346.9 ns      │ 1000    │ 32000
│     ├─ unb_pow2_is_multiple_of_reuse     377.9 ns      │ 634.1 ns      │ 381 ns        │ 385.5 ns      │ 1000    │ 32000
│     ├─ unb_pow2_rem                      831 ns        │ 1.981 µs      │ 912.2 ns      │ 926.7 ns      │ 1000    │ 16000
│     ├─ unb_pow2_rem_const                418.5 ns      │ 1.174 µs      │ 496.6 ns      │ 511.7 ns      │ 1000    │ 32000
│     ├─ unb_pow2_rem_floor                862.2 ns      │ 2.287 µs      │ 912.2 ns      │ 936.6 ns      │ 1000    │ 16000
│     ├─ unb_pow2_rem_floor_const          418.5 ns      │ 1.599 µs      │ 499.7 ns      │ 528.9 ns      │ 1000    │ 32000
│     ├─ unb_pow2_rem_floor_reuse          568.5 ns      │ 1.893 µs      │ 618.5 ns      │ 636.3 ns      │ 1000    │ 16000
│     ╰─ unb_pow2_rem_reuse                549.7 ns      │ 2.134 µs      │ 615.4 ns      │ 667.1 ns      │ 1000    │ 32000
╰─ round                                                 │               │               │               │         │
   ├─ i8                                                 │               │               │               │         │
   │  ├─ baseline_identity                 11.5 ns       │ 49 ns         │ 15.51 ns      │ 17.2 ns       │ 1000    │ 1024000
   │  ├─ pow2_ceil_to_multiple             32.4 ns       │ 287.8 ns      │ 41.48 ns      │ 44.23 ns      │ 1000    │ 512000
   │  ├─ pow2_ceil_to_multiple_const       19.71 ns      │ 68.54 ns      │ 23.03 ns      │ 24.58 ns      │ 1000    │ 512000
   │  ├─ pow2_ceil_to_multiple_reuse       23.03 ns      │ 108.5 ns      │ 24.98 ns      │ 27.34 ns      │ 1000    │ 512000
   │  ├─ pow2_floor_to_multiple            32.99 ns      │ 127.3 ns      │ 39.04 ns      │ 43.56 ns      │ 1000    │ 512000
   │  ├─ pow2_floor_to_multiple_const      19.12 ns      │ 54.67 ns      │ 21.27 ns      │ 22.71 ns      │ 1000    │ 512000
   │  ├─ pow2_floor_to_multiple_reuse      23.22 ns      │ 64.43 ns      │ 25.37 ns      │ 27.04 ns      │ 1000    │ 512000
   │  ├─ pow2_round_to_multiple            30.64 ns      │ 91.97 ns      │ 34.16 ns      │ 35.41 ns      │ 1000    │ 256000
   │  ├─ pow2_round_to_multiple_const      23.22 ns      │ 62.48 ns      │ 26.15 ns      │ 27.4 ns       │ 1000    │ 512000
   │  ├─ pow2_round_to_multiple_reuse      24.59 ns      │ 77.13 ns      │ 27.52 ns      │ 29.33 ns      │ 1000    │ 512000
   │  ├─ std_div_mul                       868.5 ns      │ 1.187 µs      │ 893.5 ns      │ 900.1 ns      │ 1000    │ 16000
   │  ├─ std_div_mul_const                 21.46 ns      │ 80.45 ns      │ 24.79 ns      │ 26.4 ns       │ 1000    │ 512000
   │  ├─ std_div_mul_reuse                 843.5 ns      │ 1.281 µs      │ 856 ns        │ 862.1 ns      │ 1000    │ 16000
   │  ├─ unb_pow2_ceil_to_multiple         37.29 ns      │ 86.11 ns      │ 38.85 ns      │ 39.6 ns       │ 1000    │ 256000
   │  ├─ unb_pow2_ceil_to_multiple_const   19.9 ns       │ 82.99 ns      │ 22.83 ns      │ 24.54 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   22.83 ns      │ 77.13 ns      │ 25.37 ns      │ 27.23 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_floor_to_multiple        30.64 ns      │ 103.8 ns      │ 39.24 ns      │ 41.78 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_floor_to_multiple_const  19.32 ns      │ 272.4 ns      │ 21.66 ns      │ 23.57 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_floor_to_multiple_reuse  23.03 ns      │ 96.66 ns      │ 25.57 ns      │ 27.33 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_round_to_multiple        39.63 ns      │ 171.6 ns      │ 42.36 ns      │ 43.38 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_round_to_multiple_const  21.86 ns      │ 71.07 ns      │ 26.93 ns      │ 30.47 ns      │ 1000    │ 512000
   │  ╰─ unb_pow2_round_to_multiple_reuse  24 ns         │ 82.79 ns      │ 27.13 ns      │ 29.24 ns      │ 1000    │ 512000
   ├─ i16                                                │               │               │               │         │
   │  ├─ baseline_identity                 23.42 ns      │ 108.5 ns      │ 30.84 ns      │ 35.2 ns       │ 1000    │ 512000
   │  ├─ pow2_ceil_to_multiple             59.55 ns      │ 166.1 ns      │ 67.56 ns      │ 73.34 ns      │ 1000    │ 256000
   │  ├─ pow2_ceil_to_multiple_const       40.02 ns      │ 148.2 ns      │ 45.88 ns      │ 49.9 ns       │ 1000    │ 256000
   │  ├─ pow2_ceil_to_multiple_reuse       42.36 ns      │ 625.1 ns      │ 46.66 ns      │ 49.94 ns      │ 1000    │ 256000
   │  ├─ pow2_floor_to_multiple            56.82 ns      │ 205.6 ns      │ 66.19 ns      │ 71.62 ns      │ 1000    │ 256000
   │  ├─ pow2_floor_to_multiple_const      38.07 ns      │ 432.6 ns      │ 42.75 ns      │ 46.18 ns      │ 1000    │ 256000
   │  ├─ pow2_floor_to_multiple_reuse      40.8 ns       │ 182.2 ns      │ 45.49 ns      │ 48.92 ns      │ 1000    │ 256000
   │  ├─ pow2_round_to_multiple            72.83 ns      │ 265.8 ns      │ 80.25 ns      │ 86.02 ns      │ 1000    │ 256000
   │  ├─ pow2_round_to_multiple_const      45.88 ns      │ 132.9 ns      │ 53.3 ns       │ 56.2 ns       │ 1000    │ 256000
   │  ├─ pow2_round_to_multiple_reuse      47.44 ns      │ 186.1 ns      │ 54.86 ns      │ 60.57 ns      │ 1000    │ 256000
   │  ├─ std_div_mul                       2.624 µs      │ 3.699 µs      │ 2.649 µs      │ 2.66 µs       │ 1000    │ 4000
   │  ├─ std_div_mul_const                 44.71 ns      │ 141.1 ns      │ 50.96 ns      │ 53.38 ns      │ 1000    │ 256000
   │  ├─ std_div_mul_reuse                 2.574 µs      │ 6.924 µs      │ 2.599 µs      │ 2.655 µs      │ 1000    │ 4000
   │  ├─ unb_pow2_ceil_to_multiple         58.38 ns      │ 224.3 ns      │ 68.54 ns      │ 74.33 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_ceil_to_multiple_const   40.02 ns      │ 128.3 ns      │ 45.88 ns      │ 48.62 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   42.75 ns      │ 145.4 ns      │ 49 ns         │ 52.49 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_floor_to_multiple        49.79 ns      │ 252.1 ns      │ 51.35 ns      │ 52.65 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_floor_to_multiple_const  38.07 ns      │ 187.6 ns      │ 42.36 ns      │ 46.01 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_floor_to_multiple_reuse  40.8 ns       │ 241.5 ns      │ 44.71 ns      │ 48.6 ns       │ 1000    │ 256000
   │  ├─ unb_pow2_round_to_multiple        68.93 ns      │ 226.3 ns      │ 82.21 ns      │ 86.14 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_round_to_multiple_const  45.1 ns       │ 231.4 ns      │ 52.13 ns      │ 55.06 ns      │ 1000    │ 256000
   │  ╰─ unb_pow2_round_to_multiple_reuse  48.22 ns      │ 129.4 ns      │ 54.08 ns      │ 56.19 ns      │ 1000    │ 256000
   ├─ i32                                                │               │               │               │         │
   │  ├─ baseline_identity                 40.8 ns       │ 249.3 ns      │ 45.49 ns      │ 53.28 ns      │ 1000    │ 256000
   │  ├─ pow2_ceil_to_multiple             103.6 ns      │ 392.7 ns      │ 121.6 ns      │ 129.5 ns      │ 1000    │ 128000
   │  ├─ pow2_ceil_to_multiple_const       77.13 ns      │ 219.3 ns      │ 85.72 ns      │ 90.3 ns       │ 1000    │ 128000
   │  ├─ pow2_ceil_to_multiple_reuse       81.04 ns      │ 224 ns        │ 88.85 ns      │ 94.78 ns      │ 1000    │ 128000
   │  ├─ pow2_floor_to_multiple            98.22 ns      │ 1.276 µs      │ 116.9 ns      │ 124.3 ns      │ 1000    │ 128000
   │  ├─ pow2_floor_to_multiple_const      74.79 ns      │ 236.5 ns      │ 82.6 ns       │ 90.66 ns      │ 1000    │ 128000
   │  ├─ pow2_floor_to_multiple_reuse      71.66 ns      │ 340.4 ns      │ 74.79 ns      │ 81.17 ns      │ 1000    │ 64000
   │  ├─ pow2_round_to_multiple            128.6 ns      │ 438 ns        │ 152.9 ns      │ 161.6 ns      │ 1000    │ 128000
   │  ├─ pow2_round_to_multiple_const      82.6 ns       │ 1.121 µs      │ 88.85 ns      │ 93.76 ns      │ 1000    │ 128000
   │  ├─ pow2_round_to_multiple_reuse      93.54 ns      │ 219.3 ns      │ 102.1 ns      │ 107.8 ns      │ 1000    │ 128000
   │  ├─ std_div_mul                       1.324 µs      │ 3.799 µs      │ 1.337 µs      │ 1.349 µs      │ 1000    │ 8000
   │  ├─ std_div_mul_const                 81.82 ns      │ 1.141 µs      │ 88.07 ns      │ 92.74 ns      │ 1000    │ 128000
   │  ├─ std_div_mul_reuse                 1.274 µs      │ 4.537 µs      │ 1.312 µs      │ 1.317 µs      │ 1000    │ 8000
   │  ├─ unb_pow2_ceil_to_multiple         108.3 ns      │ 337.2 ns      │ 128.6 ns      │ 135.6 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_ceil_to_multiple_const   77.13 ns      │ 331.8 ns      │ 85.72 ns      │ 90.99 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   81.82 ns      │ 220.1 ns      │ 95.1 ns       │ 104.7 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_floor_to_multiple        100.5 ns      │ 332.6 ns      │ 125.5 ns      │ 132.3 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_floor_to_multiple_const  75.57 ns      │ 252.9 ns      │ 91.97 ns      │ 98.43 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_floor_to_multiple_reuse  79.47 ns      │ 291.9 ns      │ 95.88 ns      │ 108.3 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_round_to_multiple        132.6 ns      │ 435.7 ns      │ 152.9 ns      │ 161.9 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_round_to_multiple_const  82.6 ns       │ 478.6 ns      │ 88.85 ns      │ 92.21 ns      │ 1000    │ 128000
   │  ╰─ unb_pow2_round_to_multiple_reuse  92.75 ns      │ 295.8 ns      │ 101.3 ns      │ 106.7 ns      │ 1000    │ 128000
   ├─ i64                                                │               │               │               │         │
   │  ├─ baseline_identity                 74.79 ns      │ 253.6 ns      │ 91.19 ns      │ 101.7 ns      │ 1000    │ 128000
   │  ├─ pow2_ceil_to_multiple             193.5 ns      │ 646.6 ns      │ 221.6 ns      │ 232 ns        │ 1000    │ 64000
   │  ├─ pow2_ceil_to_multiple_const       156.8 ns      │ 424.7 ns      │ 177.1 ns      │ 188.8 ns      │ 1000    │ 128000
   │  ├─ pow2_ceil_to_multiple_reuse       154.4 ns      │ 448.2 ns      │ 173.2 ns      │ 185.1 ns      │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple            182.6 ns      │ 749.7 ns      │ 215.4 ns      │ 237.2 ns      │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple_const      153.6 ns      │ 488.8 ns      │ 169.3 ns      │ 181 ns        │ 1000    │ 128000
   │  ├─ pow2_floor_to_multiple_reuse      162.2 ns      │ 627.9 ns      │ 181 ns        │ 191.6 ns      │ 1000    │ 128000
   │  ├─ pow2_round_to_multiple            248.2 ns      │ 921.6 ns      │ 291.9 ns      │ 304.1 ns      │ 1000    │ 64000
   │  ├─ pow2_round_to_multiple_const      170.1 ns      │ 695.1 ns      │ 185.7 ns      │ 197 ns        │ 1000    │ 64000
   │  ├─ pow2_round_to_multiple_reuse      185.7 ns      │ 1.409 µs      │ 212.2 ns      │ 222.9 ns      │ 1000    │ 64000
   │  ├─ std_div_mul                       1.649 µs      │ 3.299 µs      │ 1.674 µs      │ 1.683 µs      │ 1000    │ 8000
   │  ├─ std_div_mul_const                 163.8 ns      │ 640.4 ns      │ 196.6 ns      │ 207.9 ns      │ 1000    │ 64000
   │  ├─ std_div_mul_reuse                 1.537 µs      │ 3.224 µs      │ 1.549 µs      │ 1.56 µs       │ 1000    │ 8000
   │  ├─ unb_pow2_ceil_to_multiple         204.4 ns      │ 1.809 µs      │ 265.4 ns      │ 276.6 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_ceil_to_multiple_const   148.2 ns      │ 487.2 ns      │ 174.7 ns      │ 187.7 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   156 ns        │ 374.7 ns      │ 182.6 ns      │ 193.2 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple        179.4 ns      │ 604.4 ns      │ 256 ns        │ 265.7 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple_const  140.4 ns      │ 856 ns        │ 166.9 ns      │ 176.4 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple_reuse  146.6 ns      │ 493.5 ns      │ 166.9 ns      │ 178.3 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_round_to_multiple        260.7 ns      │ 663.8 ns      │ 304.4 ns      │ 316.5 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_round_to_multiple_const  168.5 ns      │ 459.1 ns      │ 188.8 ns      │ 200.9 ns      │ 1000    │ 64000
   │  ╰─ unb_pow2_round_to_multiple_reuse  182.6 ns      │ 735.7 ns      │ 204.4 ns      │ 218.1 ns      │ 1000    │ 64000
   ├─ i128                                               │               │               │               │         │
   │  ├─ baseline_identity                 206 ns        │ 782.6 ns      │ 226.3 ns      │ 254.1 ns      │ 1000    │ 64000
   │  ├─ pow2_ceil_to_multiple             1.074 µs      │ 2.018 µs      │ 1.093 µs      │ 1.117 µs      │ 1000    │ 16000
   │  ├─ pow2_ceil_to_multiple_const       596.6 ns      │ 1.196 µs      │ 624.7 ns      │ 640.4 ns      │ 1000    │ 32000
   │  ├─ pow2_ceil_to_multiple_reuse       668.5 ns      │ 1.318 µs      │ 687.2 ns      │ 697.5 ns      │ 1000    │ 16000
   │  ├─ pow2_floor_to_multiple            799.7 ns      │ 1.649 µs      │ 818.5 ns      │ 853.1 ns      │ 1000    │ 16000
   │  ├─ pow2_floor_to_multiple_const      440.4 ns      │ 2.496 µs      │ 527.9 ns      │ 543.4 ns      │ 1000    │ 32000
   │  ├─ pow2_floor_to_multiple_reuse      549.7 ns      │ 1.421 µs      │ 584.1 ns      │ 607.8 ns      │ 1000    │ 32000
   │  ├─ pow2_round_to_multiple            2.424 µs      │ 20.66 µs      │ 2.462 µs      │ 2.528 µs      │ 1000    │ 8000
   │  ├─ pow2_round_to_multiple_const      812.2 ns      │ 7.537 µs      │ 868.5 ns      │ 893.9 ns      │ 1000    │ 16000
   │  ├─ pow2_round_to_multiple_reuse      906 ns        │ 9.724 µs      │ 999.7 ns      │ 1.041 µs      │ 1000    │ 16000
   │  ├─ std_div_mul                       9.599 µs      │ 21.89 µs      │ 9.699 µs      │ 10.03 µs      │ 1000    │ 1000
   │  ├─ std_div_mul_const                 674.7 ns      │ 1.843 µs      │ 787.2 ns      │ 788.5 ns      │ 1000    │ 16000
   │  ├─ std_div_mul_reuse                 8.799 µs      │ 15.04 µs      │ 8.999 µs      │ 9.097 µs      │ 1000    │ 2000
   │  ├─ unb_pow2_ceil_to_multiple         1.124 µs      │ 2.162 µs      │ 1.137 µs      │ 1.168 µs      │ 1000    │ 16000
   │  ├─ unb_pow2_ceil_to_multiple_const   606 ns        │ 5.531 µs      │ 637.2 ns      │ 677.7 ns      │ 1000    │ 32000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   699.7 ns      │ 1.693 µs      │ 806 ns        │ 821.7 ns      │ 1000    │ 16000
   │  ├─ unb_pow2_floor_to_multiple        887.2 ns      │ 3.343 µs      │ 943.5 ns      │ 958.2 ns      │ 1000    │ 16000
   │  ├─ unb_pow2_floor_to_multiple_const  440.4 ns      │ 1.121 µs      │ 546.6 ns      │ 563.7 ns      │ 1000    │ 32000
   │  ├─ unb_pow2_floor_to_multiple_reuse  549.7 ns      │ 1.512 µs      │ 587.2 ns      │ 615.2 ns      │ 1000    │ 32000
   │  ├─ unb_pow2_round_to_multiple        2.499 µs      │ 4.524 µs      │ 2.549 µs      │ 2.596 µs      │ 1000    │ 4000
   │  ├─ unb_pow2_round_to_multiple_const  774.7 ns      │ 1.768 µs      │ 868.5 ns      │ 888 ns        │ 1000    │ 16000
   │  ╰─ unb_pow2_round_to_multiple_reuse  906 ns        │ 3.181 µs      │ 999.7 ns      │ 1.012 µs      │ 1000    │ 16000
   ├─ u8                                                 │               │               │               │         │
   │  ├─ baseline_identity                 10.92 ns      │ 46.86 ns      │ 16 ns         │ 17.52 ns      │ 1000    │ 1024000
   │  ├─ pow2_ceil_to_multiple             32.21 ns      │ 176.9 ns      │ 41.29 ns      │ 43.47 ns      │ 1000    │ 512000
   │  ├─ pow2_ceil_to_multiple_const       19.9 ns       │ 339 ns        │ 22.64 ns      │ 24.46 ns      │ 1000    │ 512000
   │  ├─ pow2_ceil_to_multiple_reuse       23.03 ns      │ 78.3 ns       │ 25.18 ns      │ 27.12 ns      │ 1000    │ 512000
   │  ├─ pow2_floor_to_multiple            32.99 ns      │ 159.9 ns      │ 40.31 ns      │ 42.65 ns      │ 1000    │ 512000
   │  ├─ pow2_floor_to_multiple_const      19.32 ns      │ 67.95 ns      │ 21.46 ns      │ 23.53 ns      │ 1000    │ 512000
   │  ├─ pow2_floor_to_multiple_reuse      23.22 ns      │ 115.8 ns      │ 25.37 ns      │ 27.06 ns      │ 1000    │ 512000
   │  ├─ pow2_round_to_multiple            34.16 ns      │ 158.5 ns      │ 47.83 ns      │ 50.32 ns      │ 1000    │ 512000
   │  ├─ pow2_round_to_multiple_const      19.71 ns      │ 90.21 ns      │ 22.44 ns      │ 24.21 ns      │ 1000    │ 512000
   │  ├─ pow2_round_to_multiple_reuse      23.03 ns      │ 83.38 ns      │ 25.37 ns      │ 27.61 ns      │ 1000    │ 512000
   │  ├─ std_div_mul                       862.2 ns      │ 1.656 µs      │ 868.5 ns      │ 873.5 ns      │ 1000    │ 16000
   │  ├─ std_div_mul_const                 18.93 ns      │ 99.79 ns      │ 22.05 ns      │ 24.58 ns      │ 1000    │ 512000
   │  ├─ std_div_mul_reuse                 856 ns        │ 1.218 µs      │ 862.2 ns      │ 867.1 ns      │ 1000    │ 16000
   │  ├─ unb_pow2_ceil_to_multiple         39.82 ns      │ 158.9 ns      │ 48.42 ns      │ 50.79 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_ceil_to_multiple_const   19.9 ns       │ 69.12 ns      │ 22.44 ns      │ 23.99 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   23.22 ns      │ 75.96 ns      │ 25.18 ns      │ 27.21 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_floor_to_multiple        30.45 ns      │ 90.02 ns      │ 39.24 ns      │ 41.3 ns       │ 1000    │ 512000
   │  ├─ unb_pow2_floor_to_multiple_const  19.12 ns      │ 80.06 ns      │ 21.46 ns      │ 23.39 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_floor_to_multiple_reuse  22.83 ns      │ 164.6 ns      │ 25.76 ns      │ 30.26 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_round_to_multiple        38.85 ns      │ 75.57 ns      │ 40.41 ns      │ 41.01 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_round_to_multiple_const  20.1 ns       │ 110.9 ns      │ 23.03 ns      │ 24.87 ns      │ 1000    │ 512000
   │  ╰─ unb_pow2_round_to_multiple_reuse  23.22 ns      │ 87.68 ns      │ 25.96 ns      │ 28.48 ns      │ 1000    │ 512000
   ├─ u16                                                │               │               │               │         │
   │  ├─ baseline_identity                 21.27 ns      │ 107.2 ns      │ 28.89 ns      │ 34.96 ns      │ 1000    │ 512000
   │  ├─ pow2_ceil_to_multiple             58.77 ns      │ 163 ns        │ 66.19 ns      │ 71.91 ns      │ 1000    │ 256000
   │  ├─ pow2_ceil_to_multiple_const       40.41 ns      │ 184.5 ns      │ 45.49 ns      │ 48.14 ns      │ 1000    │ 256000
   │  ├─ pow2_ceil_to_multiple_reuse       41.97 ns      │ 232.2 ns      │ 47.83 ns      │ 51.23 ns      │ 1000    │ 256000
   │  ├─ pow2_floor_to_multiple            56.43 ns      │ 246.2 ns      │ 67.36 ns      │ 72.04 ns      │ 1000    │ 256000
   │  ├─ pow2_floor_to_multiple_const      38.07 ns      │ 113.4 ns      │ 41.97 ns      │ 45.18 ns      │ 1000    │ 256000
   │  ├─ pow2_floor_to_multiple_reuse      40.41 ns      │ 175.1 ns      │ 45.88 ns      │ 49.49 ns      │ 1000    │ 256000
   │  ├─ pow2_round_to_multiple            63.46 ns      │ 186.5 ns      │ 72.83 ns      │ 78.42 ns      │ 1000    │ 256000
   │  ├─ pow2_round_to_multiple_const      39.63 ns      │ 177.5 ns      │ 45.49 ns      │ 49.36 ns      │ 1000    │ 256000
   │  ├─ pow2_round_to_multiple_reuse      41.58 ns      │ 144.7 ns      │ 48.22 ns      │ 52.73 ns      │ 1000    │ 256000
   │  ├─ std_div_mul                       868.5 ns      │ 1.287 µs      │ 887.2 ns      │ 903.2 ns      │ 1000    │ 16000
   │  ├─ std_div_mul_const                 37.68 ns      │ 111.5 ns      │ 43.54 ns      │ 46.44 ns      │ 1000    │ 256000
   │  ├─ std_div_mul_reuse                 837.2 ns      │ 6.206 µs      │ 849.7 ns      │ 856.8 ns      │ 1000    │ 16000
   │  ├─ unb_pow2_ceil_to_multiple         59.55 ns      │ 171.2 ns      │ 71.27 ns      │ 75.53 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_ceil_to_multiple_const   39.63 ns      │ 543.1 ns      │ 45.88 ns      │ 49.75 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   41.97 ns      │ 112.6 ns      │ 47.05 ns      │ 50.56 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_floor_to_multiple        57.6 ns       │ 181.8 ns      │ 67.95 ns      │ 72.29 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_floor_to_multiple_const  38.07 ns      │ 143.9 ns      │ 43.14 ns      │ 47.5 ns       │ 1000    │ 256000
   │  ├─ unb_pow2_floor_to_multiple_reuse  41.19 ns      │ 256.8 ns      │ 45.88 ns      │ 49.89 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_round_to_multiple        62.68 ns      │ 220.1 ns      │ 74.39 ns      │ 79.16 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_round_to_multiple_const  39.24 ns      │ 165 ns        │ 45.1 ns       │ 49.47 ns      │ 1000    │ 256000
   │  ╰─ unb_pow2_round_to_multiple_reuse  42.36 ns      │ 188 ns        │ 48.22 ns      │ 52.25 ns      │ 1000    │ 256000
   ├─ u32                                                │               │               │               │         │
   │  ├─ baseline_identity                 41.19 ns      │ 254.8 ns      │ 52.52 ns      │ 60.5 ns       │ 1000    │ 256000
   │  ├─ pow2_ceil_to_multiple             99.79 ns      │ 274 ns        │ 124 ns        │ 130.7 ns      │ 1000    │ 128000
   │  ├─ pow2_ceil_to_multiple_const       77.13 ns      │ 359.1 ns      │ 85.72 ns      │ 94.32 ns      │ 1000    │ 128000
   │  ├─ pow2_ceil_to_multiple_reuse       80.25 ns      │ 391.1 ns      │ 88.85 ns      │ 97.72 ns      │ 1000    │ 128000
   │  ├─ pow2_floor_to_multiple            99.79 ns      │ 456.8 ns      │ 121.6 ns      │ 127.6 ns      │ 1000    │ 128000
   │  ├─ pow2_floor_to_multiple_const      81.43 ns      │ 287.2 ns      │ 86.5 ns       │ 95.46 ns      │ 1000    │ 256000
   │  ├─ pow2_floor_to_multiple_reuse      78.69 ns      │ 240.4 ns      │ 82.6 ns       │ 88.79 ns      │ 1000    │ 128000
   │  ├─ pow2_round_to_multiple            115.4 ns      │ 1.258 µs      │ 134.1 ns      │ 142.9 ns      │ 1000    │ 128000
   │  ├─ pow2_round_to_multiple_const      77.13 ns      │ 380.2 ns      │ 87.29 ns      │ 93.53 ns      │ 1000    │ 128000
   │  ├─ pow2_round_to_multiple_reuse      81.04 ns      │ 798.2 ns      │ 88.85 ns      │ 95.94 ns      │ 1000    │ 128000
   │  ├─ std_div_mul                       1.324 µs      │ 3.737 µs      │ 1.337 µs      │ 1.353 µs      │ 1000    │ 8000
   │  ├─ std_div_mul_const                 74.79 ns      │ 252.1 ns      │ 81.04 ns      │ 87.13 ns      │ 1000    │ 128000
   │  ├─ std_div_mul_reuse                 1.287 µs      │ 3.337 µs      │ 1.299 µs      │ 1.324 µs      │ 1000    │ 8000
   │  ├─ unb_pow2_ceil_to_multiple         108.3 ns      │ 408.3 ns      │ 136.5 ns      │ 146.3 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_ceil_to_multiple_const   77.13 ns      │ 1.133 µs      │ 84.94 ns      │ 91.96 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   81.04 ns      │ 232.6 ns      │ 89.63 ns      │ 95.35 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_floor_to_multiple        98.22 ns      │ 295.8 ns      │ 121.6 ns      │ 127.6 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_floor_to_multiple_const  68.54 ns      │ 293.5 ns      │ 71.66 ns      │ 74.57 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple_reuse  77.91 ns      │ 341.1 ns      │ 82.6 ns       │ 89.35 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_round_to_multiple        116.1 ns      │ 996.6 ns      │ 136.5 ns      │ 143.2 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_round_to_multiple_const  77.13 ns      │ 249 ns        │ 85.72 ns      │ 92.1 ns       │ 1000    │ 128000
   │  ╰─ unb_pow2_round_to_multiple_reuse  81.04 ns      │ 238.8 ns      │ 88.85 ns      │ 95.59 ns      │ 1000    │ 128000
   ├─ u64                                                │               │               │               │         │
   │  ├─ baseline_identity                 73.22 ns      │ 299.7 ns      │ 90.41 ns      │ 101.3 ns      │ 1000    │ 128000
   │  ├─ pow2_ceil_to_multiple             195.1 ns      │ 712.2 ns      │ 215.4 ns      │ 232.2 ns      │ 1000    │ 64000
   │  ├─ pow2_ceil_to_multiple_const       145.1 ns      │ 704.4 ns      │ 171.6 ns      │ 180.6 ns      │ 1000    │ 64000
   │  ├─ pow2_ceil_to_multiple_reuse       152.9 ns      │ 1.77 µs       │ 179.4 ns      │ 198 ns        │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple            184.1 ns      │ 787.2 ns      │ 201.3 ns      │ 224.7 ns      │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple_const      156 ns        │ 594.3 ns      │ 182.6 ns      │ 193 ns        │ 1000    │ 128000
   │  ├─ pow2_floor_to_multiple_reuse      146.6 ns      │ 465.4 ns      │ 165.4 ns      │ 177.1 ns      │ 1000    │ 64000
   │  ├─ pow2_round_to_multiple            199.7 ns      │ 681 ns        │ 240.4 ns      │ 254.5 ns      │ 1000    │ 64000
   │  ├─ pow2_round_to_multiple_const      143.5 ns      │ 657.6 ns      │ 173.2 ns      │ 184.6 ns      │ 1000    │ 64000
   │  ├─ pow2_round_to_multiple_reuse      154.4 ns      │ 643.5 ns      │ 170.1 ns      │ 182.2 ns      │ 1000    │ 64000
   │  ├─ std_div_mul                       1.637 µs      │ 3.124 µs      │ 1.662 µs      │ 1.677 µs      │ 1000    │ 8000
   │  ├─ std_div_mul_const                 141.9 ns      │ 596.6 ns      │ 163.8 ns      │ 172.6 ns      │ 1000    │ 64000
   │  ├─ std_div_mul_reuse                 1.512 µs      │ 2.962 µs      │ 1.524 µs      │ 1.534 µs      │ 1000    │ 8000
   │  ├─ unb_pow2_ceil_to_multiple         202.9 ns      │ 595.1 ns      │ 226.3 ns      │ 240.7 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_ceil_to_multiple_const   157.6 ns      │ 2.197 µs      │ 175.5 ns      │ 193 ns        │ 1000    │ 128000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   151.3 ns      │ 487.2 ns      │ 167.7 ns      │ 183 ns        │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple        190.4 ns      │ 613.8 ns      │ 221.6 ns      │ 241.2 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple_const  140.4 ns      │ 510.7 ns      │ 173.2 ns      │ 188.3 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple_reuse  160.7 ns      │ 524.7 ns      │ 170.8 ns      │ 185.5 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_round_to_multiple        221.6 ns      │ 548.2 ns      │ 235.7 ns      │ 252.1 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_round_to_multiple_const  146.6 ns      │ 384.1 ns      │ 165.4 ns      │ 174.7 ns      │ 1000    │ 64000
   │  ╰─ unb_pow2_round_to_multiple_reuse  154.4 ns      │ 629.4 ns      │ 173.2 ns      │ 186.2 ns      │ 1000    │ 64000
   ╰─ u128                                               │               │               │               │         │
      ├─ baseline_identity                 188.8 ns      │ 1.009 µs      │ 210.7 ns      │ 227.3 ns      │ 1000    │ 64000
      ├─ pow2_ceil_to_multiple             1.081 µs      │ 2.018 µs      │ 1.093 µs      │ 1.113 µs      │ 1000    │ 16000
      ├─ pow2_ceil_to_multiple_const       596.6 ns      │ 1.293 µs      │ 615.4 ns      │ 647 ns        │ 1000    │ 32000
      ├─ pow2_ceil_to_multiple_reuse       731 ns        │ 1.306 µs      │ 781 ns        │ 790.7 ns      │ 1000    │ 16000
      ├─ pow2_floor_to_multiple            812.2 ns      │ 3.187 µs      │ 899.7 ns      │ 914.9 ns      │ 1000    │ 16000
      ├─ pow2_floor_to_multiple_const      412.2 ns      │ 1.093 µs      │ 484.1 ns      │ 509.3 ns      │ 1000    │ 32000
      ├─ pow2_floor_to_multiple_reuse      543.5 ns      │ 1.693 µs      │ 615.4 ns      │ 649.2 ns      │ 1000    │ 32000
      ├─ pow2_round_to_multiple            2.099 µs      │ 4.824 µs      │ 3.362 µs      │ 3.102 µs      │ 1000    │ 4000
      ├─ pow2_round_to_multiple_const      606 ns        │ 1.543 µs      │ 706 ns        │ 796.4 ns      │ 1000    │ 16000
      ├─ pow2_round_to_multiple_reuse      718.5 ns      │ 1.762 µs      │ 787.2 ns      │ 796.3 ns      │ 1000    │ 16000
      ├─ std_div_mul                       4.224 µs      │ 8.499 µs      │ 4.349 µs      │ 4.494 µs      │ 1000    │ 4000
      ├─ std_div_mul_const                 418.5 ns      │ 1.449 µs      │ 518.5 ns      │ 543.7 ns      │ 1000    │ 32000
      ├─ std_div_mul_reuse                 4.349 µs      │ 10.89 µs      │ 4.399 µs      │ 4.447 µs      │ 1000    │ 4000
      ├─ unb_pow2_ceil_to_multiple         1.168 µs      │ 2.074 µs      │ 1.224 µs      │ 1.238 µs      │ 1000    │ 16000
      ├─ unb_pow2_ceil_to_multiple_const   593.5 ns      │ 2.087 µs      │ 615.4 ns      │ 650.7 ns      │ 1000    │ 32000
      ├─ unb_pow2_ceil_to_multiple_reuse   674.7 ns      │ 2.906 µs      │ 693.5 ns      │ 705.6 ns      │ 1000    │ 16000
      ├─ unb_pow2_floor_to_multiple        806 ns        │ 1.743 µs      │ 831 ns        │ 867.8 ns      │ 1000    │ 16000
      ├─ unb_pow2_floor_to_multiple_const  415.4 ns      │ 1.318 µs      │ 477.9 ns      │ 494.9 ns      │ 1000    │ 32000
      ├─ unb_pow2_floor_to_multiple_reuse  543.5 ns      │ 1.462 µs      │ 606 ns        │ 626.8 ns      │ 1000    │ 32000
      ├─ unb_pow2_round_to_multiple        2.074 µs      │ 4.499 µs      │ 2.099 µs      │ 2.245 µs      │ 1000    │ 4000
      ├─ unb_pow2_round_to_multiple_const  596.6 ns      │ 1.806 µs      │ 634.1 ns      │ 672.2 ns      │ 1000    │ 32000
      ╰─ unb_pow2_round_to_multiple_reuse  668.5 ns      │ 1.662 µs      │ 687.2 ns      │ 699.7 ns      │ 1000    │ 16000


```
