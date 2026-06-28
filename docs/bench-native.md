# `public` bench with native target (AMD Ryzen 7 7800X3D 8-Core Processor)
```
public                                     fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ div                                                   │               │               │               │         │
│  ├─ i8                                                 │               │               │               │         │
│  │  ├─ baseline_identity                 9.36 ns       │ 266 ns        │ 10.33 ns      │ 11.47 ns      │ 1000    │ 512000
│  │  ├─ pow2_div                          41.19 ns      │ 638.8 ns      │ 49.39 ns      │ 53.09 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_ceil                     41.58 ns      │ 577.5 ns      │ 45.88 ns      │ 48.63 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_ceil_const               21.07 ns      │ 44.51 ns      │ 22.83 ns      │ 23.33 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_ceil_reuse               26.54 ns      │ 368.3 ns      │ 28.5 ns       │ 29.8 ns       │ 1000    │ 512000
│  │  ├─ pow2_div_const                    21.86 ns      │ 68.73 ns      │ 24.2 ns       │ 24.73 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_floor                    29.08 ns      │ 94.71 ns      │ 31.43 ns      │ 32.91 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_floor_const              19.12 ns      │ 69.51 ns      │ 20.88 ns      │ 22.59 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_floor_reuse              23.42 ns      │ 354.4 ns      │ 26.15 ns      │ 28.15 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_reuse                    25.76 ns      │ 69.32 ns      │ 31.23 ns      │ 31.88 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_round                    46.27 ns      │ 93.54 ns      │ 53.3 ns       │ 54.17 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_round_const              25.18 ns      │ 66.97 ns      │ 28.3 ns       │ 29.18 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_round_reuse              33.38 ns      │ 70.49 ns      │ 37.48 ns      │ 38.55 ns      │ 1000    │ 512000
│  │  ├─ std_div                           856 ns        │ 1.799 µs      │ 868.5 ns      │ 878.7 ns      │ 1000    │ 16000
│  │  ├─ std_div_const                     20.68 ns      │ 427.7 ns      │ 22.44 ns      │ 24.09 ns      │ 1000    │ 512000
│  │  ├─ std_div_reuse                     849.7 ns      │ 4.031 µs      │ 856 ns        │ 866.6 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_div                      52.13 ns      │ 129 ns        │ 56.82 ns      │ 57.49 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_ceil                 44.32 ns      │ 114.6 ns      │ 49.79 ns      │ 50.25 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_ceil_const           21.07 ns      │ 57.4 ns       │ 22.83 ns      │ 23.39 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_ceil_reuse           26.15 ns      │ 55.84 ns      │ 28.3 ns       │ 28.58 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_const                21.46 ns      │ 58.96 ns      │ 23.61 ns      │ 24.16 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_floor                39.24 ns      │ 96.27 ns      │ 42.75 ns      │ 43.56 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_floor_const          21.27 ns      │ 80.16 ns      │ 21.95 ns      │ 22.86 ns      │ 1000    │ 1024000
│  │  ├─ unb_pow2_div_floor_reuse          21.86 ns      │ 206.8 ns      │ 24.59 ns      │ 25.26 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_reuse                28.69 ns      │ 312 ns        │ 31.23 ns      │ 32.44 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_round                54.86 ns      │ 594.7 ns      │ 59.55 ns      │ 61.66 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_round_const          24.39 ns      │ 323.4 ns      │ 27.13 ns      │ 29.76 ns      │ 1000    │ 512000
│  │  ╰─ unb_pow2_div_round_reuse          33.38 ns      │ 113.6 ns      │ 37.68 ns      │ 40.2 ns       │ 1000    │ 512000
│  ├─ i16                                                │               │               │               │         │
│  │  ├─ baseline_identity                 26.54 ns      │ 102.9 ns      │ 27.71 ns      │ 29.79 ns      │ 1000    │ 512000
│  │  ├─ pow2_div                          59.16 ns      │ 127.1 ns      │ 65.41 ns      │ 67.05 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_ceil                     64.63 ns      │ 147.4 ns      │ 71.27 ns      │ 73.74 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_ceil_const               40.8 ns       │ 537.2 ns      │ 46.27 ns      │ 48.06 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_ceil_reuse               43.54 ns      │ 104.8 ns      │ 50.18 ns      │ 52.52 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_const                    40.41 ns      │ 112.6 ns      │ 46.66 ns      │ 48.52 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_floor                    65.8 ns       │ 598.2 ns      │ 68.54 ns      │ 71.52 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_floor_const              36.5 ns       │ 200.9 ns      │ 38.46 ns      │ 40.03 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_floor_reuse              40.41 ns      │ 99.39 ns      │ 43.93 ns      │ 46.87 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_reuse                    50.57 ns      │ 168.5 ns      │ 57.6 ns       │ 60.85 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_round                    65.41 ns      │ 149.7 ns      │ 72.44 ns      │ 74.88 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_round_const              49.39 ns      │ 122.8 ns      │ 53.3 ns       │ 54.7 ns       │ 1000    │ 256000
│  │  ├─ pow2_div_round_reuse              55.64 ns      │ 199.7 ns      │ 63.46 ns      │ 67.5 ns       │ 1000    │ 256000
│  │  ├─ std_div                           874.7 ns      │ 1.899 µs      │ 893.5 ns      │ 907.2 ns      │ 1000    │ 16000
│  │  ├─ std_div_const                     41.58 ns      │ 109.1 ns      │ 46.27 ns      │ 47.47 ns      │ 1000    │ 256000
│  │  ├─ std_div_reuse                     2.499 µs      │ 3.699 µs      │ 2.599 µs      │ 2.585 µs      │ 1000    │ 4000
│  │  ├─ unb_pow2_div                      72.05 ns      │ 235.7 ns      │ 81.04 ns      │ 85.5 ns       │ 1000    │ 256000
│  │  ├─ unb_pow2_div_ceil                 59.94 ns      │ 178.6 ns      │ 87.29 ns      │ 81.59 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_ceil_const           39.24 ns      │ 158.3 ns      │ 48.61 ns      │ 50.91 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_ceil_reuse           43.93 ns      │ 116.1 ns      │ 50.18 ns      │ 51.69 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_const                41.19 ns      │ 99.79 ns      │ 47.05 ns      │ 47.83 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_floor                49.79 ns      │ 108.3 ns      │ 53.69 ns      │ 54.31 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_floor_const          36.11 ns      │ 82.99 ns      │ 39.24 ns      │ 41.84 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_floor_reuse          39.24 ns      │ 93.14 ns      │ 41.19 ns      │ 43.12 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_reuse                49.79 ns      │ 116.9 ns      │ 53.3 ns       │ 54.84 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_round                65.41 ns      │ 169.3 ns      │ 77.52 ns      │ 78.17 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_round_const          48.22 ns      │ 790.4 ns      │ 52.13 ns      │ 54.28 ns      │ 1000    │ 256000
│  │  ╰─ unb_pow2_div_round_reuse          58.77 ns      │ 556.8 ns      │ 63.85 ns      │ 66.81 ns      │ 1000    │ 256000
│  ├─ i32                                                │               │               │               │         │
│  │  ├─ baseline_identity                 43.14 ns      │ 633.7 ns      │ 46.66 ns      │ 52.9 ns       │ 1000    │ 256000
│  │  ├─ pow2_div                          102.9 ns      │ 311.5 ns      │ 127.9 ns      │ 131.9 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_ceil                     112.2 ns      │ 524.7 ns      │ 125.5 ns      │ 129.7 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_ceil_const               80.25 ns      │ 1.065 µs      │ 89.63 ns      │ 92.73 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_ceil_reuse               89.63 ns      │ 311.5 ns      │ 101.7 ns      │ 111.7 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_const                    84.94 ns      │ 233.3 ns      │ 97.44 ns      │ 101.8 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_floor                    89.63 ns      │ 501.3 ns      │ 104.4 ns      │ 107.4 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_floor_const              75.96 ns      │ 223.6 ns      │ 88.07 ns      │ 90.86 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_floor_reuse              74 ns         │ 832.6 ns      │ 81.04 ns      │ 88.68 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_reuse                    90.41 ns      │ 237.2 ns      │ 97.44 ns      │ 98.58 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_round                    136.5 ns      │ 1.288 µs      │ 147.4 ns      │ 152.9 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_round_const              106.8 ns      │ 1.234 µs      │ 113 ns        │ 117.2 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_round_reuse              119.3 ns      │ 1.27 µs       │ 129.4 ns      │ 132.9 ns      │ 1000    │ 128000
│  │  ├─ std_div                           1.312 µs      │ 4.262 µs      │ 1.349 µs      │ 1.364 µs      │ 1000    │ 8000
│  │  ├─ std_div_const                     85.72 ns      │ 1.272 µs      │ 92.75 ns      │ 97.5 ns       │ 1000    │ 128000
│  │  ├─ std_div_reuse                     1.287 µs      │ 18.58 µs      │ 1.299 µs      │ 1.369 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div                      119.3 ns      │ 282.6 ns      │ 134.9 ns      │ 138.6 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_ceil                 113 ns        │ 327.1 ns      │ 126.3 ns      │ 131.1 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_ceil_const           83.38 ns      │ 290.4 ns      │ 91.97 ns      │ 95.43 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_ceil_reuse           87.29 ns      │ 304.4 ns      │ 97.44 ns      │ 99.77 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_const                85.72 ns      │ 279.4 ns      │ 91.97 ns      │ 97.22 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_floor                90.41 ns      │ 322.4 ns      │ 105.2 ns      │ 110 ns        │ 1000    │ 128000
│  │  ├─ unb_pow2_div_floor_const          68.54 ns      │ 224 ns        │ 74.79 ns      │ 77.2 ns       │ 1000    │ 128000
│  │  ├─ unb_pow2_div_floor_reuse          74.79 ns      │ 197.4 ns      │ 79.47 ns      │ 82.56 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_reuse                90.41 ns      │ 202.1 ns      │ 99.79 ns      │ 101.7 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_round                138.8 ns      │ 896.6 ns      │ 161.5 ns      │ 172.3 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_round_const          93.54 ns      │ 263 ns        │ 113 ns        │ 115 ns        │ 1000    │ 128000
│  │  ╰─ unb_pow2_div_round_reuse          117.7 ns      │ 234.1 ns      │ 129.4 ns      │ 131.5 ns      │ 1000    │ 128000
│  ├─ i64                                                │               │               │               │         │
│  │  ├─ baseline_identity                 81.82 ns      │ 217.7 ns      │ 92.75 ns      │ 96.27 ns      │ 1000    │ 128000
│  │  ├─ pow2_div                          221.6 ns      │ 2.69 µs       │ 252.9 ns      │ 274.3 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_ceil                     218.5 ns      │ 637.2 ns      │ 246.6 ns      │ 255.5 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_ceil_const               174.7 ns      │ 449.7 ns      │ 198.2 ns      │ 202.4 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_ceil_reuse               176.3 ns      │ 524.7 ns      │ 195.1 ns      │ 201.4 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_const                    176.3 ns      │ 498.2 ns      │ 196.6 ns      │ 199.9 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_floor                    165.4 ns      │ 431 ns        │ 177.9 ns      │ 185.5 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_floor_const              159.1 ns      │ 377.9 ns      │ 176.3 ns      │ 182.6 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_floor_reuse              159.1 ns      │ 2.643 µs      │ 171.6 ns      │ 190.7 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_reuse                    184.1 ns      │ 359.1 ns      │ 199.7 ns      │ 203.8 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_round                    262.2 ns      │ 2.182 µs      │ 284.1 ns      │ 293.2 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_round_const              202.9 ns      │ 427.9 ns      │ 224.7 ns      │ 229.4 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_round_reuse              231 ns        │ 951.3 ns      │ 256 ns        │ 260.4 ns      │ 1000    │ 64000
│  │  ├─ std_div                           1.637 µs      │ 16.02 µs      │ 1.699 µs      │ 1.716 µs      │ 1000    │ 8000
│  │  ├─ std_div_const                     173.2 ns      │ 2.485 µs      │ 190.4 ns      │ 197.1 ns      │ 1000    │ 64000
│  │  ├─ std_div_reuse                     1.537 µs      │ 2.299 µs      │ 1.562 µs      │ 1.572 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div                      220.1 ns      │ 465.4 ns      │ 249.7 ns      │ 253.2 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil                 218.5 ns      │ 2.774 µs      │ 240.4 ns      │ 262.6 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil_const           165.4 ns      │ 2.065 µs      │ 179.4 ns      │ 185.1 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil_reuse           174.7 ns      │ 2.416 µs      │ 193.5 ns      │ 204.3 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_const                168.5 ns      │ 443.5 ns      │ 185.7 ns      │ 189.3 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor                176.3 ns      │ 399.7 ns      │ 193.5 ns      │ 197.4 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor_const          149.7 ns      │ 624.7 ns      │ 163.8 ns      │ 172 ns        │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor_reuse          156 ns        │ 790.4 ns      │ 163.8 ns      │ 168.2 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_reuse                173.2 ns      │ 2.081 µs      │ 195.1 ns      │ 207.6 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_round                273.2 ns      │ 2.473 µs      │ 298.2 ns      │ 306 ns        │ 1000    │ 64000
│  │  ├─ unb_pow2_div_round_const          213.8 ns      │ 2.932 µs      │ 232.6 ns      │ 251.3 ns      │ 1000    │ 64000
│  │  ╰─ unb_pow2_div_round_reuse          229.4 ns      │ 2.363 µs      │ 260.7 ns      │ 271.3 ns      │ 1000    │ 64000
│  ├─ i128                                               │               │               │               │         │
│  │  ├─ baseline_identity                 206 ns        │ 581 ns        │ 234.1 ns      │ 243.3 ns      │ 1000    │ 32000
│  │  ├─ pow2_div                          1.287 µs      │ 3.587 µs      │ 1.312 µs      │ 1.336 µs      │ 1000    │ 8000
│  │  ├─ pow2_div_ceil                     1.912 µs      │ 3.799 µs      │ 1.937 µs      │ 1.957 µs      │ 1000    │ 8000
│  │  ├─ pow2_div_ceil_const               1.056 µs      │ 2.218 µs      │ 1.081 µs      │ 1.09 µs       │ 1000    │ 16000
│  │  ├─ pow2_div_ceil_reuse               1.512 µs      │ 3.174 µs      │ 1.549 µs      │ 1.564 µs      │ 1000    │ 8000
│  │  ├─ pow2_div_const                    1.749 µs      │ 3.399 µs      │ 1.774 µs      │ 1.798 µs      │ 1000    │ 8000
│  │  ├─ pow2_div_floor                    1.062 µs      │ 9.449 µs      │ 1.093 µs      │ 1.145 µs      │ 1000    │ 16000
│  │  ├─ pow2_div_floor_const              837.2 ns      │ 10.59 µs      │ 862.2 ns      │ 943 ns        │ 1000    │ 16000
│  │  ├─ pow2_div_floor_reuse              1.087 µs      │ 10.32 µs      │ 1.112 µs      │ 1.193 µs      │ 1000    │ 16000
│  │  ├─ pow2_div_reuse                    2.049 µs      │ 21.87 µs      │ 2.074 µs      │ 2.235 µs      │ 1000    │ 8000
│  │  ├─ pow2_div_round                    2.824 µs      │ 37.22 µs      │ 2.874 µs      │ 3.129 µs      │ 1000    │ 4000
│  │  ├─ pow2_div_round_const              1.649 µs      │ 29.18 µs      │ 1.674 µs      │ 1.816 µs      │ 1000    │ 8000
│  │  ├─ pow2_div_round_reuse              2.349 µs      │ 20.54 µs      │ 2.387 µs      │ 2.571 µs      │ 1000    │ 8000
│  │  ├─ std_div                           8.999 µs      │ 101.1 µs      │ 9.099 µs      │ 9.751 µs      │ 1000    │ 2000
│  │  ├─ std_div_const                     1.024 µs      │ 9.887 µs      │ 1.043 µs      │ 1.116 µs      │ 1000    │ 16000
│  │  ├─ std_div_reuse                     8.049 µs      │ 81.44 µs      │ 8.199 µs      │ 8.513 µs      │ 1000    │ 2000
│  │  ├─ unb_pow2_div                      1.299 µs      │ 3.724 µs      │ 1.312 µs      │ 1.329 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div_ceil                 2.012 µs      │ 3.962 µs      │ 2.049 µs      │ 2.06 µs       │ 1000    │ 8000
│  │  ├─ unb_pow2_div_ceil_const           1.068 µs      │ 2.062 µs      │ 1.081 µs      │ 1.1 µs        │ 1000    │ 16000
│  │  ├─ unb_pow2_div_ceil_reuse           1.512 µs      │ 4.674 µs      │ 1.537 µs      │ 1.55 µs       │ 1000    │ 8000
│  │  ├─ unb_pow2_div_const                1.737 µs      │ 3.749 µs      │ 1.774 µs      │ 1.79 µs       │ 1000    │ 8000
│  │  ├─ unb_pow2_div_floor                1.149 µs      │ 2.474 µs      │ 1.168 µs      │ 1.182 µs      │ 1000    │ 16000
│  │  ├─ unb_pow2_div_floor_const          824.7 ns      │ 1.699 µs      │ 849.7 ns      │ 862.9 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_div_floor_reuse          1.099 µs      │ 1.806 µs      │ 1.124 µs      │ 1.133 µs      │ 1000    │ 16000
│  │  ├─ unb_pow2_div_reuse                2.074 µs      │ 4.187 µs      │ 2.099 µs      │ 2.123 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div_round                2.924 µs      │ 9.474 µs      │ 2.974 µs      │ 3.009 µs      │ 1000    │ 4000
│  │  ├─ unb_pow2_div_round_const          1.662 µs      │ 4.587 µs      │ 1.687 µs      │ 1.704 µs      │ 1000    │ 8000
│  │  ╰─ unb_pow2_div_round_reuse          2.362 µs      │ 3.699 µs      │ 2.374 µs      │ 2.405 µs      │ 1000    │ 8000
│  ├─ u8                                                 │               │               │               │         │
│  │  ├─ baseline_identity                 11.11 ns      │ 34.36 ns      │ 11.89 ns      │ 12.46 ns      │ 1000    │ 1024000
│  │  ├─ pow2_div                          29.47 ns      │ 83.38 ns      │ 34.94 ns      │ 36.5 ns       │ 1000    │ 256000
│  │  ├─ pow2_div_ceil                     38.85 ns      │ 106.4 ns      │ 44.71 ns      │ 45.47 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_ceil_const               20.88 ns      │ 140.8 ns      │ 22.64 ns      │ 23.3 ns       │ 1000    │ 512000
│  │  ├─ pow2_div_ceil_reuse               24.79 ns      │ 60.92 ns      │ 27.32 ns      │ 28.32 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_const                    18.73 ns      │ 52.13 ns      │ 19.71 ns      │ 20.27 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_floor                    37.68 ns      │ 91.58 ns      │ 41 ns         │ 41.84 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_floor_const              18.93 ns      │ 53.11 ns      │ 20.1 ns       │ 21.02 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_floor_reuse              21.46 ns      │ 314.6 ns      │ 23.61 ns      │ 24.8 ns       │ 1000    │ 512000
│  │  ├─ pow2_div_reuse                    21.27 ns      │ 56.62 ns      │ 22.05 ns      │ 22.57 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_round                    34.55 ns      │ 83.38 ns      │ 43.14 ns      │ 43.09 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_round_const              21.27 ns      │ 48.61 ns      │ 23.42 ns      │ 23.64 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_round_reuse              24.39 ns      │ 62.68 ns      │ 26.93 ns      │ 27.48 ns      │ 1000    │ 512000
│  │  ├─ std_div                           856 ns        │ 1.881 µs      │ 868.5 ns      │ 876.7 ns      │ 1000    │ 16000
│  │  ├─ std_div_const                     18.54 ns      │ 55.25 ns      │ 19.71 ns      │ 20.47 ns      │ 1000    │ 512000
│  │  ├─ std_div_reuse                     849.7 ns      │ 1.643 µs      │ 862.2 ns      │ 870.1 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_div                      35.72 ns      │ 149.3 ns      │ 39.24 ns      │ 39.98 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_ceil                 42.75 ns      │ 110.3 ns      │ 47.44 ns      │ 48.11 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_ceil_const           20.88 ns      │ 48.81 ns      │ 22.64 ns      │ 23.28 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_ceil_reuse           24.2 ns       │ 57.01 ns      │ 26.35 ns      │ 27.23 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_const                18.93 ns      │ 73.22 ns      │ 19.9 ns       │ 20.26 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_floor                34.94 ns      │ 111.1 ns      │ 38.07 ns      │ 38.89 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_floor_const          18.73 ns      │ 54.67 ns      │ 19.51 ns      │ 20.08 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_floor_reuse          20.68 ns      │ 269.5 ns      │ 21.86 ns      │ 23.53 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_reuse                21.27 ns      │ 342.5 ns      │ 23.81 ns      │ 25.46 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_round                40.02 ns      │ 92.36 ns      │ 47.05 ns      │ 47.26 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_round_const          21.66 ns      │ 78.3 ns       │ 24.39 ns      │ 25.02 ns      │ 1000    │ 512000
│  │  ╰─ unb_pow2_div_round_reuse          24.39 ns      │ 84.16 ns      │ 27.32 ns      │ 28.93 ns      │ 1000    │ 512000
│  ├─ u16                                                │               │               │               │         │
│  │  ├─ baseline_identity                 26.35 ns      │ 103.5 ns      │ 31.62 ns      │ 34.61 ns      │ 1000    │ 512000
│  │  ├─ pow2_div                          59.16 ns      │ 176.7 ns      │ 63.46 ns      │ 66.18 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_ceil                     64.63 ns      │ 130.6 ns      │ 72.05 ns      │ 73.98 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_ceil_const               42.75 ns      │ 566.1 ns      │ 48.22 ns      │ 49.9 ns       │ 1000    │ 256000
│  │  ├─ pow2_div_ceil_reuse               43.14 ns      │ 309.5 ns      │ 49.79 ns      │ 52.16 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_const                    36.5 ns       │ 87.29 ns      │ 38.07 ns      │ 39.31 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_floor                    66.19 ns      │ 206.4 ns      │ 68.14 ns      │ 69.53 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_floor_const              41.39 ns      │ 345.8 ns      │ 42.56 ns      │ 43.82 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_floor_reuse              39.24 ns      │ 138.8 ns      │ 40.8 ns       │ 42.26 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_reuse                    39.24 ns      │ 97.05 ns      │ 41.58 ns      │ 43.19 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_round                    64.24 ns      │ 176.3 ns      │ 76.74 ns      │ 78.37 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_round_const              43.14 ns      │ 166.1 ns      │ 47.44 ns      │ 49.14 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_round_reuse              43.93 ns      │ 191.9 ns      │ 49 ns         │ 49.99 ns      │ 1000    │ 256000
│  │  ├─ std_div                           868.5 ns      │ 1.174 µs      │ 874.7 ns      │ 881.7 ns      │ 1000    │ 16000
│  │  ├─ std_div_const                     36.5 ns       │ 118.9 ns      │ 40.8 ns       │ 43.13 ns      │ 1000    │ 256000
│  │  ├─ std_div_reuse                     856 ns        │ 1.206 µs      │ 868.5 ns      │ 873.6 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_div                      50.96 ns      │ 116.9 ns      │ 54.47 ns      │ 56.15 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_ceil                 56.82 ns      │ 159.1 ns      │ 61.5 ns       │ 63.1 ns       │ 1000    │ 128000
│  │  ├─ unb_pow2_div_ceil_const           41.58 ns      │ 524.3 ns      │ 45.88 ns      │ 48.67 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_ceil_reuse           45.1 ns       │ 130.2 ns      │ 49.79 ns      │ 51.19 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_const                41.58 ns      │ 108.5 ns      │ 43.34 ns      │ 45.98 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_floor                50.96 ns      │ 124.3 ns      │ 54.86 ns      │ 55.72 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_floor_const          41.19 ns      │ 78.11 ns      │ 42.56 ns      │ 43.41 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_floor_reuse          39.24 ns      │ 148.6 ns      │ 42.75 ns      │ 44.1 ns       │ 1000    │ 256000
│  │  ├─ unb_pow2_div_reuse                40.02 ns      │ 166.9 ns      │ 44.32 ns      │ 47.62 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_round                74 ns         │ 702.1 ns      │ 84.55 ns      │ 91.17 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_round_const          41.58 ns      │ 593.1 ns      │ 46.66 ns      │ 48.22 ns      │ 1000    │ 256000
│  │  ╰─ unb_pow2_div_round_reuse          45.49 ns      │ 109.9 ns      │ 50.57 ns      │ 51.6 ns       │ 1000    │ 256000
│  ├─ u32                                                │               │               │               │         │
│  │  ├─ baseline_identity                 37.29 ns      │ 112.2 ns      │ 40.41 ns      │ 41.37 ns      │ 1000    │ 128000
│  │  ├─ pow2_div                          98.22 ns      │ 1.344 µs      │ 111.5 ns      │ 115.7 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_ceil                     113 ns        │ 434.1 ns      │ 121.6 ns      │ 125.1 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_ceil_const               82.6 ns       │ 356 ns        │ 89.63 ns      │ 91.75 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_ceil_reuse               88.07 ns      │ 191.9 ns      │ 99.79 ns      │ 102.6 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_const                    74.39 ns      │ 296.2 ns      │ 82.99 ns      │ 88.12 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_floor                    88.07 ns      │ 1.191 µs      │ 94.32 ns      │ 99.55 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_floor_const              74.39 ns      │ 185.3 ns      │ 79.47 ns      │ 80.38 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_floor_reuse              81.04 ns      │ 766.9 ns      │ 84.94 ns      │ 89.23 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_reuse                    75.57 ns      │ 220.1 ns      │ 81.04 ns      │ 85.2 ns       │ 1000    │ 128000
│  │  ├─ pow2_div_round                    112.2 ns      │ 1.264 µs      │ 124 ns        │ 131.3 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_round_const              85.72 ns      │ 248.2 ns      │ 93.54 ns      │ 95.22 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_round_reuse              89.63 ns      │ 355.2 ns      │ 97.44 ns      │ 99.88 ns      │ 1000    │ 128000
│  │  ├─ std_div                           1.312 µs      │ 18.57 µs      │ 1.337 µs      │ 1.437 µs      │ 1000    │ 8000
│  │  ├─ std_div_const                     74.79 ns      │ 468.5 ns      │ 80.64 ns      │ 86.96 ns      │ 1000    │ 256000
│  │  ├─ std_div_reuse                     1.287 µs      │ 3.037 µs      │ 1.299 µs      │ 1.315 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div                      91.97 ns      │ 291.9 ns      │ 101.3 ns      │ 106.6 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_ceil                 108.3 ns      │ 541.1 ns      │ 126.3 ns      │ 131.4 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_ceil_const           84.16 ns      │ 175.5 ns      │ 91.19 ns      │ 92.92 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_ceil_reuse           88.07 ns      │ 1.385 µs      │ 96.66 ns      │ 100.6 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_const                74.79 ns      │ 440 ns        │ 81.04 ns      │ 88.27 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_floor                91.19 ns      │ 249 ns        │ 101.3 ns      │ 105.7 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_floor_const          68.54 ns      │ 192.7 ns      │ 73.22 ns      │ 75.38 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_floor_reuse          74.79 ns      │ 259.9 ns      │ 82.6 ns       │ 89.21 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_reuse                74 ns         │ 184.1 ns      │ 82.6 ns       │ 84.56 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_round                114.6 ns      │ 324 ns        │ 125.5 ns      │ 128.6 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_round_const          84.16 ns      │ 331.8 ns      │ 91.97 ns      │ 94.11 ns      │ 1000    │ 128000
│  │  ╰─ unb_pow2_div_round_reuse          88.07 ns      │ 981.8 ns      │ 97.44 ns      │ 102 ns        │ 1000    │ 128000
│  ├─ u64                                                │               │               │               │         │
│  │  ├─ baseline_identity                 81.04 ns      │ 1.059 µs      │ 95.88 ns      │ 109.5 ns      │ 1000    │ 128000
│  │  ├─ pow2_div                          196.6 ns      │ 545.1 ns      │ 218.5 ns      │ 223.8 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_ceil                     209.1 ns      │ 615.4 ns      │ 231 ns        │ 236.5 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_ceil_const               174.7 ns      │ 559.1 ns      │ 191.9 ns      │ 196.8 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_ceil_reuse               176.3 ns      │ 426.3 ns      │ 199.7 ns      │ 205.1 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_const                    149.7 ns      │ 474.7 ns      │ 171.6 ns      │ 182.4 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_floor                    168.5 ns      │ 482.6 ns      │ 193.5 ns      │ 197.6 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_floor_const              149.7 ns      │ 413.8 ns      │ 166.9 ns      │ 172.5 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_floor_reuse              157.6 ns      │ 2.057 µs      │ 166.9 ns      │ 175.1 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_reuse                    157.6 ns      │ 346.6 ns      │ 166.9 ns      │ 170.7 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_round                    215.4 ns      │ 2.184 µs      │ 231 ns        │ 240.7 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_round_const              173.2 ns      │ 660.7 ns      │ 188.8 ns      │ 194.2 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_round_reuse              176.3 ns      │ 590.4 ns      │ 193.5 ns      │ 198.7 ns      │ 1000    │ 64000
│  │  ├─ std_div                           1.649 µs      │ 4.062 µs      │ 1.674 µs      │ 1.709 µs      │ 1000    │ 8000
│  │  ├─ std_div_const                     148.2 ns      │ 554.4 ns      │ 162.2 ns      │ 165.7 ns      │ 1000    │ 64000
│  │  ├─ std_div_reuse                     1.537 µs      │ 4.974 µs      │ 1.574 µs      │ 1.583 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div                      173.2 ns      │ 749.7 ns      │ 190.4 ns      │ 195.8 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil                 224.7 ns      │ 795.1 ns      │ 251.3 ns      │ 264.1 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil_const           176.3 ns      │ 457.6 ns      │ 204.4 ns      │ 210.2 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil_reuse           174.7 ns      │ 915.4 ns      │ 193.5 ns      │ 202.4 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_const                148.2 ns      │ 413.8 ns      │ 160.7 ns      │ 165.8 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor                174.7 ns      │ 407.6 ns      │ 191.9 ns      │ 195.7 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor_const          159.9 ns      │ 500.5 ns      │ 181 ns        │ 193.5 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_floor_reuse          154.4 ns      │ 457.6 ns      │ 162.2 ns      │ 165.3 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_reuse                156 ns        │ 366.9 ns      │ 163.8 ns      │ 166.9 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_round                218.5 ns      │ 2.732 µs      │ 237.2 ns      │ 245.3 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_round_const          179.4 ns      │ 2.698 µs      │ 198.2 ns      │ 228.2 ns      │ 1000    │ 64000
│  │  ╰─ unb_pow2_div_round_reuse          174.7 ns      │ 2.563 µs      │ 193.5 ns      │ 204.5 ns      │ 1000    │ 64000
│  ╰─ u128                                               │               │               │               │         │
│     ├─ baseline_identity                 199.7 ns      │ 1.604 µs      │ 212.2 ns      │ 219.9 ns      │ 1000    │ 64000
│     ├─ pow2_div                          1.162 µs      │ 10.48 µs      │ 1.181 µs      │ 1.258 µs      │ 1000    │ 16000
│     ├─ pow2_div_ceil                     1.787 µs      │ 21.61 µs      │ 1.849 µs      │ 2.012 µs      │ 1000    │ 8000
│     ├─ pow2_div_ceil_const               1.043 µs      │ 9.931 µs      │ 1.062 µs      │ 1.151 µs      │ 1000    │ 16000
│     ├─ pow2_div_ceil_reuse               1.449 µs      │ 21.59 µs      │ 1.462 µs      │ 1.566 µs      │ 1000    │ 8000
│     ├─ pow2_div_const                    812.2 ns      │ 9.856 µs      │ 831 ns        │ 901.8 ns      │ 1000    │ 16000
│     ├─ pow2_div_floor                    1.062 µs      │ 9.612 µs      │ 1.093 µs      │ 1.136 µs      │ 1000    │ 16000
│     ├─ pow2_div_floor_const              812.2 ns      │ 10.14 µs      │ 831 ns        │ 869.3 ns      │ 1000    │ 16000
│     ├─ pow2_div_floor_reuse              937.2 ns      │ 2.137 µs      │ 949.7 ns      │ 968.7 ns      │ 1000    │ 16000
│     ├─ pow2_div_reuse                    924.7 ns      │ 2.018 µs      │ 937.2 ns      │ 950.2 ns      │ 1000    │ 16000
│     ├─ pow2_div_round                    2.649 µs      │ 5.674 µs      │ 2.699 µs      │ 2.718 µs      │ 1000    │ 4000
│     ├─ pow2_div_round_const              993.5 ns      │ 2.268 µs      │ 1.012 µs      │ 1.023 µs      │ 1000    │ 16000
│     ├─ pow2_div_round_reuse              1.437 µs      │ 3.774 µs      │ 1.462 µs      │ 1.473 µs      │ 1000    │ 8000
│     ├─ std_div                           5.799 µs      │ 13.84 µs      │ 5.899 µs      │ 5.95 µs       │ 1000    │ 2000
│     ├─ std_div_const                     812.2 ns      │ 1.674 µs      │ 837.2 ns      │ 842.7 ns      │ 1000    │ 16000
│     ├─ std_div_reuse                     4.299 µs      │ 7.924 µs      │ 4.349 µs      │ 4.413 µs      │ 1000    │ 4000
│     ├─ unb_pow2_div                      1.193 µs      │ 4.762 µs      │ 1.212 µs      │ 1.23 µs       │ 1000    │ 16000
│     ├─ unb_pow2_div_ceil                 1.899 µs      │ 8.612 µs      │ 1.937 µs      │ 1.983 µs      │ 1000    │ 8000
│     ├─ unb_pow2_div_ceil_const           1.062 µs      │ 2.056 µs      │ 1.081 µs      │ 1.11 µs       │ 1000    │ 16000
│     ├─ unb_pow2_div_ceil_reuse           1.449 µs      │ 2.912 µs      │ 1.474 µs      │ 1.523 µs      │ 1000    │ 8000
│     ├─ unb_pow2_div_const                818.5 ns      │ 1.856 µs      │ 837.2 ns      │ 900.5 ns      │ 1000    │ 16000
│     ├─ unb_pow2_div_floor                1.187 µs      │ 2.437 µs      │ 1.218 µs      │ 1.308 µs      │ 1000    │ 16000
│     ├─ unb_pow2_div_floor_const          818.5 ns      │ 2.231 µs      │ 837.2 ns      │ 879.7 ns      │ 1000    │ 16000
│     ├─ unb_pow2_div_floor_reuse          918.5 ns      │ 1.849 µs      │ 937.2 ns      │ 974.2 ns      │ 1000    │ 16000
│     ├─ unb_pow2_div_reuse                918.5 ns      │ 1.793 µs      │ 931 ns        │ 968.9 ns      │ 1000    │ 16000
│     ├─ unb_pow2_div_round                2.674 µs      │ 5.099 µs      │ 2.724 µs      │ 2.813 µs      │ 1000    │ 4000
│     ├─ unb_pow2_div_round_const          999.7 ns      │ 2.031 µs      │ 1.018 µs      │ 1.061 µs      │ 1000    │ 16000
│     ╰─ unb_pow2_div_round_reuse          1.437 µs      │ 2.774 µs      │ 1.449 µs      │ 1.517 µs      │ 1000    │ 8000
├─ mul                                                   │               │               │               │         │
│  ├─ i8                                                 │               │               │               │         │
│  │  ├─ baseline_identity                 11.02 ns      │ 33.28 ns      │ 11.7 ns       │ 12.21 ns      │ 1000    │ 1024000
│  │  ├─ pow2_mul                          37.68 ns      │ 79.67 ns      │ 41 ns         │ 41.92 ns      │ 1000    │ 512000
│  │  ├─ pow2_mul_const                    18.54 ns      │ 239.8 ns      │ 20.88 ns      │ 22.36 ns      │ 1000    │ 512000
│  │  ├─ pow2_mul_reuse                    20.88 ns      │ 60.33 ns      │ 22.25 ns      │ 22.86 ns      │ 1000    │ 512000
│  │  ├─ std_mul                           33.57 ns      │ 294.3 ns      │ 38.46 ns      │ 40.33 ns      │ 1000    │ 512000
│  │  ├─ std_mul_const                     18.93 ns      │ 45.29 ns      │ 20.49 ns      │ 20.89 ns      │ 1000    │ 512000
│  │  ├─ std_mul_reuse                     24.39 ns      │ 73.81 ns      │ 26.84 ns      │ 27.66 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_mul                      36.11 ns      │ 142.3 ns      │ 43.93 ns      │ 46.2 ns       │ 1000    │ 512000
│  │  ├─ unb_pow2_mul_const                19.32 ns      │ 56.43 ns      │ 21.86 ns      │ 22.47 ns      │ 1000    │ 512000
│  │  ╰─ unb_pow2_mul_reuse                21.66 ns      │ 58.38 ns      │ 23.61 ns      │ 24.26 ns      │ 1000    │ 512000
│  ├─ i16                                                │               │               │               │         │
│  │  ├─ baseline_identity                 26.35 ns      │ 132.4 ns      │ 30.84 ns      │ 32.78 ns      │ 1000    │ 512000
│  │  ├─ pow2_mul                          66.97 ns      │ 171.6 ns      │ 69.71 ns      │ 73.19 ns      │ 1000    │ 256000
│  │  ├─ pow2_mul_const                    35.72 ns      │ 96.27 ns      │ 39.24 ns      │ 41.07 ns      │ 1000    │ 256000
│  │  ├─ pow2_mul_reuse                    39.24 ns      │ 115.8 ns      │ 41.97 ns      │ 44.42 ns      │ 1000    │ 256000
│  │  ├─ std_mul                           59.55 ns      │ 161.8 ns      │ 61.89 ns      │ 63.69 ns      │ 1000    │ 256000
│  │  ├─ std_mul_const                     41.58 ns      │ 117.3 ns      │ 42.75 ns      │ 44.39 ns      │ 1000    │ 512000
│  │  ├─ std_mul_reuse                     39.63 ns      │ 107.6 ns      │ 41.58 ns      │ 42.42 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_mul                      52.13 ns      │ 143.9 ns      │ 57.99 ns      │ 59.55 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_mul_const                35.72 ns      │ 420.1 ns      │ 38.07 ns      │ 39.28 ns      │ 1000    │ 256000
│  │  ╰─ unb_pow2_mul_reuse                39.63 ns      │ 569.3 ns      │ 43.54 ns      │ 48.25 ns      │ 1000    │ 256000
│  ├─ i32                                                │               │               │               │         │
│  │  ├─ baseline_identity                 43.54 ns      │ 129.8 ns      │ 54.47 ns      │ 55.26 ns      │ 1000    │ 256000
│  │  ├─ pow2_mul                          88.07 ns      │ 677.1 ns      │ 98.22 ns      │ 102.2 ns      │ 1000    │ 128000
│  │  ├─ pow2_mul_const                    68.54 ns      │ 169.3 ns      │ 72.44 ns      │ 74.98 ns      │ 1000    │ 128000
│  │  ├─ pow2_mul_reuse                    74 ns         │ 209.1 ns      │ 78.69 ns      │ 81.74 ns      │ 1000    │ 128000
│  │  ├─ std_mul                           120.8 ns      │ 577.9 ns      │ 128.6 ns      │ 134.8 ns      │ 1000    │ 128000
│  │  ├─ std_mul_const                     75.18 ns      │ 165 ns        │ 81.04 ns      │ 85.46 ns      │ 1000    │ 256000
│  │  ├─ std_mul_reuse                     74.79 ns      │ 1.17 µs       │ 78.69 ns      │ 83.94 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_mul                      89.63 ns      │ 320.1 ns      │ 99 ns         │ 106.1 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_mul_const                75.18 ns      │ 217.7 ns      │ 79.47 ns      │ 80.56 ns      │ 1000    │ 256000
│  │  ╰─ unb_pow2_mul_reuse                81.43 ns      │ 209.9 ns      │ 87.68 ns      │ 90.88 ns      │ 1000    │ 256000
│  ├─ i64                                                │               │               │               │         │
│  │  ├─ baseline_identity                 81.82 ns      │ 649 ns        │ 93.54 ns      │ 101.2 ns      │ 1000    │ 128000
│  │  ├─ pow2_mul                          166.9 ns      │ 470.1 ns      │ 182.6 ns      │ 188.2 ns      │ 1000    │ 64000
│  │  ├─ pow2_mul_const                    151.3 ns      │ 427.9 ns      │ 168.5 ns      │ 175.3 ns      │ 1000    │ 64000
│  │  ├─ pow2_mul_reuse                    156 ns        │ 554.4 ns      │ 168.5 ns      │ 174.5 ns      │ 1000    │ 64000
│  │  ├─ std_mul                           307.6 ns      │ 785.7 ns      │ 335.7 ns      │ 351.4 ns      │ 1000    │ 64000
│  │  ├─ std_mul_const                     149.7 ns      │ 406 ns        │ 170.1 ns      │ 175.6 ns      │ 1000    │ 64000
│  │  ├─ std_mul_reuse                     149.7 ns      │ 402.9 ns      │ 160.7 ns      │ 167.5 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_mul                      171.6 ns      │ 557.6 ns      │ 190.4 ns      │ 195.2 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_mul_const                148.2 ns      │ 2.543 µs      │ 163.8 ns      │ 174.4 ns      │ 1000    │ 64000
│  │  ╰─ unb_pow2_mul_reuse                156 ns        │ 363.8 ns      │ 168.5 ns      │ 172.4 ns      │ 1000    │ 64000
│  ├─ i128                                               │               │               │               │         │
│  │  ├─ baseline_identity                 227.9 ns      │ 1.488 µs      │ 266.9 ns      │ 276.8 ns      │ 1000    │ 64000
│  │  ├─ pow2_mul                          1.068 µs      │ 2.362 µs      │ 1.099 µs      │ 1.111 µs      │ 1000    │ 16000
│  │  ├─ pow2_mul_const                    837.2 ns      │ 1.493 µs      │ 862.2 ns      │ 869.7 ns      │ 1000    │ 16000
│  │  ├─ pow2_mul_reuse                    981 ns        │ 2.212 µs      │ 999.7 ns      │ 1.023 µs      │ 1000    │ 16000
│  │  ├─ std_mul                           1.206 µs      │ 2.781 µs      │ 1.249 µs      │ 1.264 µs      │ 1000    │ 16000
│  │  ├─ std_mul_const                     837.2 ns      │ 1.668 µs      │ 856 ns        │ 865.2 ns      │ 1000    │ 16000
│  │  ├─ std_mul_reuse                     812.2 ns      │ 1.912 µs      │ 843.5 ns      │ 849.7 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_mul                      1.181 µs      │ 2.443 µs      │ 1.206 µs      │ 1.217 µs      │ 1000    │ 16000
│  │  ├─ unb_pow2_mul_const                837.2 ns      │ 1.537 µs      │ 856 ns        │ 866.2 ns      │ 1000    │ 16000
│  │  ╰─ unb_pow2_mul_reuse                981 ns        │ 1.749 µs      │ 999.7 ns      │ 1.008 µs      │ 1000    │ 16000
│  ├─ u8                                                 │               │               │               │         │
│  │  ├─ baseline_identity                 11.02 ns      │ 28.11 ns      │ 11.8 ns       │ 12.42 ns      │ 1000    │ 1024000
│  │  ├─ pow2_mul                          30.25 ns      │ 596.2 ns      │ 34.94 ns      │ 38.46 ns      │ 1000    │ 256000
│  │  ├─ pow2_mul_const                    18.73 ns      │ 316.9 ns      │ 20.29 ns      │ 24.81 ns      │ 1000    │ 512000
│  │  ├─ pow2_mul_reuse                    20.88 ns      │ 43.73 ns      │ 21.86 ns      │ 22.34 ns      │ 1000    │ 512000
│  │  ├─ std_mul                           33.57 ns      │ 326.3 ns      │ 36.5 ns       │ 38.16 ns      │ 1000    │ 512000
│  │  ├─ std_mul_const                     18.54 ns      │ 80.25 ns      │ 20.1 ns       │ 20.6 ns       │ 1000    │ 512000
│  │  ├─ std_mul_reuse                     23.61 ns      │ 87.68 ns      │ 27.13 ns      │ 27.95 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_mul                      35.72 ns      │ 365.2 ns      │ 42.17 ns      │ 44.72 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_mul_const                19.32 ns      │ 297.4 ns      │ 20.1 ns       │ 21.44 ns      │ 1000    │ 512000
│  │  ╰─ unb_pow2_mul_reuse                21.46 ns      │ 292.1 ns      │ 22.83 ns      │ 24.46 ns      │ 1000    │ 512000
│  ├─ u16                                                │               │               │               │         │
│  │  ├─ baseline_identity                 25.96 ns      │ 107.2 ns      │ 27.32 ns      │ 30.51 ns      │ 1000    │ 512000
│  │  ├─ pow2_mul                          66.97 ns      │ 284.5 ns      │ 69.32 ns      │ 71.56 ns      │ 1000    │ 256000
│  │  ├─ pow2_mul_const                    38.07 ns      │ 86.11 ns      │ 41.19 ns      │ 42.78 ns      │ 1000    │ 256000
│  │  ├─ pow2_mul_reuse                    41.97 ns      │ 168.5 ns      │ 43.93 ns      │ 45.84 ns      │ 1000    │ 256000
│  │  ├─ std_mul                           59.55 ns      │ 225.5 ns      │ 61.89 ns      │ 63.63 ns      │ 1000    │ 256000
│  │  ├─ std_mul_const                     41 ns         │ 291.7 ns      │ 42.75 ns      │ 45.36 ns      │ 1000    │ 512000
│  │  ├─ std_mul_reuse                     40.02 ns      │ 594.7 ns      │ 42.36 ns      │ 44.6 ns       │ 1000    │ 256000
│  │  ├─ unb_pow2_mul                      50.96 ns      │ 139.6 ns      │ 56.82 ns      │ 58.68 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_mul_const                41.39 ns      │ 133.1 ns      │ 44.12 ns      │ 47 ns         │ 1000    │ 512000
│  │  ╰─ unb_pow2_mul_reuse                40.8 ns       │ 590.8 ns      │ 45.88 ns      │ 47.85 ns      │ 1000    │ 256000
│  ├─ u32                                                │               │               │               │         │
│  │  ├─ baseline_identity                 43.54 ns      │ 516.1 ns      │ 46.66 ns      │ 48.71 ns      │ 1000    │ 256000
│  │  ├─ pow2_mul                          87.29 ns      │ 358.3 ns      │ 99 ns         │ 101.6 ns      │ 1000    │ 128000
│  │  ├─ pow2_mul_const                    68.54 ns      │ 311.5 ns      │ 74.79 ns      │ 80.32 ns      │ 1000    │ 128000
│  │  ├─ pow2_mul_reuse                    81.43 ns      │ 691.1 ns      │ 86.11 ns      │ 92.99 ns      │ 1000    │ 256000
│  │  ├─ std_mul                           120.1 ns      │ 1.183 µs      │ 125.5 ns      │ 131.6 ns      │ 1000    │ 128000
│  │  ├─ std_mul_const                     74.79 ns      │ 437.6 ns      │ 79.86 ns      │ 82.95 ns      │ 1000    │ 256000
│  │  ├─ std_mul_reuse                     76.35 ns      │ 192.7 ns      │ 79.47 ns      │ 82.23 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_mul                      91.97 ns      │ 323.2 ns      │ 101.3 ns      │ 106.2 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_mul_const                70.88 ns      │ 158.3 ns      │ 77.91 ns      │ 79.91 ns      │ 1000    │ 128000
│  │  ╰─ unb_pow2_mul_reuse                77.13 ns      │ 274 ns        │ 86.5 ns       │ 93.92 ns      │ 1000    │ 128000
│  ├─ u64                                                │               │               │               │         │
│  │  ├─ baseline_identity                 81.82 ns      │ 385.7 ns      │ 94.32 ns      │ 103.7 ns      │ 1000    │ 128000
│  │  ├─ pow2_mul                          168.5 ns      │ 591.9 ns      │ 185.7 ns      │ 192 ns        │ 1000    │ 64000
│  │  ├─ pow2_mul_const                    151.3 ns      │ 526.3 ns      │ 170.1 ns      │ 175 ns        │ 1000    │ 64000
│  │  ├─ pow2_mul_reuse                    159.1 ns      │ 427.9 ns      │ 174.7 ns      │ 179.5 ns      │ 1000    │ 64000
│  │  ├─ std_mul                           265.4 ns      │ 827.9 ns      │ 284.1 ns      │ 293.4 ns      │ 1000    │ 32000
│  │  ├─ std_mul_const                     149.7 ns      │ 551.3 ns      │ 166.9 ns      │ 181.8 ns      │ 1000    │ 64000
│  │  ├─ std_mul_reuse                     149.7 ns      │ 745.1 ns      │ 156 ns        │ 172.4 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_mul                      170.1 ns      │ 404.4 ns      │ 182.6 ns      │ 187 ns        │ 1000    │ 64000
│  │  ├─ unb_pow2_mul_const                148.2 ns      │ 601.3 ns      │ 163.8 ns      │ 169.7 ns      │ 1000    │ 64000
│  │  ╰─ unb_pow2_mul_reuse                157.6 ns      │ 668.5 ns      │ 171.6 ns      │ 178.3 ns      │ 1000    │ 64000
│  ╰─ u128                                               │               │               │               │         │
│     ├─ baseline_identity                 198.2 ns      │ 546.6 ns      │ 215.4 ns      │ 229.9 ns      │ 1000    │ 64000
│     ├─ pow2_mul                          1.062 µs      │ 2.168 µs      │ 1.081 µs      │ 1.132 µs      │ 1000    │ 16000
│     ├─ pow2_mul_const                    818.5 ns      │ 1.662 µs      │ 831 ns        │ 871.5 ns      │ 1000    │ 16000
│     ├─ pow2_mul_reuse                    956 ns        │ 1.868 µs      │ 974.7 ns      │ 1.021 µs      │ 1000    │ 16000
│     ├─ std_mul                           1.237 µs      │ 2.062 µs      │ 1.287 µs      │ 1.321 µs      │ 1000    │ 8000
│     ├─ std_mul_const                     812.2 ns      │ 1.712 µs      │ 831 ns        │ 879.7 ns      │ 1000    │ 16000
│     ├─ std_mul_reuse                     806 ns        │ 1.831 µs      │ 824.7 ns      │ 861.9 ns      │ 1000    │ 16000
│     ├─ unb_pow2_mul                      1.181 µs      │ 2.193 µs      │ 1.199 µs      │ 1.247 µs      │ 1000    │ 16000
│     ├─ unb_pow2_mul_const                818.5 ns      │ 1.868 µs      │ 837.2 ns      │ 873.1 ns      │ 1000    │ 16000
│     ╰─ unb_pow2_mul_reuse                962.2 ns      │ 2.499 µs      │ 974.7 ns      │ 1.026 µs      │ 1000    │ 8000
├─ rem                                                   │               │               │               │         │
│  ├─ i8                                                 │               │               │               │         │
│  │  ├─ baseline_identity                 11.11 ns      │ 29.47 ns      │ 11.7 ns       │ 12.58 ns      │ 1000    │ 1024000
│  │  ├─ pow2_is_multiple_of               35.14 ns      │ 285.9 ns      │ 40.41 ns      │ 44.85 ns      │ 1000    │ 512000
│  │  ├─ pow2_is_multiple_of_const         19.12 ns      │ 49.79 ns      │ 19.9 ns       │ 20.61 ns      │ 1000    │ 512000
│  │  ├─ pow2_is_multiple_of_reuse         25.18 ns      │ 81.43 ns      │ 27.13 ns      │ 27.62 ns      │ 1000    │ 512000
│  │  ├─ pow2_rem                          38.65 ns      │ 76.74 ns      │ 43.73 ns      │ 45.43 ns      │ 1000    │ 512000
│  │  ├─ pow2_rem_const                    23.22 ns      │ 57.01 ns      │ 25.96 ns      │ 27.17 ns      │ 1000    │ 512000
│  │  ├─ pow2_rem_floor                    31.23 ns      │ 89.24 ns      │ 33.18 ns      │ 35.74 ns      │ 1000    │ 512000
│  │  ├─ pow2_rem_floor_const              18.73 ns      │ 296.2 ns      │ 19.71 ns      │ 20.69 ns      │ 1000    │ 512000
│  │  ├─ pow2_rem_floor_reuse              20.88 ns      │ 271.4 ns      │ 22.05 ns      │ 23.47 ns      │ 1000    │ 512000
│  │  ├─ pow2_rem_reuse                    24.39 ns      │ 209.1 ns      │ 27.13 ns      │ 28.33 ns      │ 1000    │ 512000
│  │  ├─ std_rem                           856 ns        │ 1.668 µs      │ 887.2 ns      │ 896.1 ns      │ 1000    │ 16000
│  │  ├─ std_rem_const                     21.46 ns      │ 66.39 ns      │ 24 ns         │ 24.65 ns      │ 1000    │ 512000
│  │  ├─ std_rem_reuse                     843.5 ns      │ 1.112 µs      │ 856 ns        │ 859 ns        │ 1000    │ 16000
│  │  ├─ unb_pow2_is_multiple_of           32.79 ns      │ 226.9 ns      │ 35.33 ns      │ 36.59 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_is_multiple_of_const     19.32 ns      │ 279.2 ns      │ 20.49 ns      │ 21.8 ns       │ 1000    │ 512000
│  │  ├─ unb_pow2_is_multiple_of_reuse     24.59 ns      │ 200.1 ns      │ 26.93 ns      │ 28.45 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_rem                      38.46 ns      │ 131.2 ns      │ 45.49 ns      │ 47.32 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_rem_const                23.03 ns      │ 124.3 ns      │ 25.76 ns      │ 27.5 ns       │ 1000    │ 512000
│  │  ├─ unb_pow2_rem_floor                35.53 ns      │ 109.5 ns      │ 42.95 ns      │ 44.9 ns       │ 1000    │ 512000
│  │  ├─ unb_pow2_rem_floor_const          19.12 ns      │ 81.82 ns      │ 20.88 ns      │ 21.61 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_rem_floor_reuse          20.29 ns      │ 54.08 ns      │ 21.46 ns      │ 22.31 ns      │ 1000    │ 512000
│  │  ╰─ unb_pow2_rem_reuse                20.49 ns      │ 86.89 ns      │ 23.22 ns      │ 23.68 ns      │ 1000    │ 256000
│  ├─ i16                                                │               │               │               │         │
│  │  ├─ baseline_identity                 26.74 ns      │ 337.2 ns      │ 30.84 ns      │ 32.81 ns      │ 1000    │ 512000
│  │  ├─ pow2_is_multiple_of               52.13 ns      │ 204.4 ns      │ 57.99 ns      │ 59.63 ns      │ 1000    │ 256000
│  │  ├─ pow2_is_multiple_of_const         29.67 ns      │ 73.03 ns      │ 35.33 ns      │ 35.92 ns      │ 1000    │ 512000
│  │  ├─ pow2_is_multiple_of_reuse         27.52 ns      │ 89.63 ns      │ 33.77 ns      │ 33.62 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem                          68.14 ns      │ 140 ns        │ 73.61 ns      │ 74.87 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem_const                    48.61 ns      │ 502.9 ns      │ 53.69 ns      │ 56.18 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem_floor                    66.97 ns      │ 618.1 ns      │ 75.96 ns      │ 78.75 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem_floor_const              36.5 ns       │ 100.1 ns      │ 38.07 ns      │ 39.97 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem_floor_reuse              39.63 ns      │ 96.66 ns      │ 41.97 ns      │ 44.89 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem_reuse                    47.44 ns      │ 541.5 ns      │ 54.86 ns      │ 56.58 ns      │ 1000    │ 256000
│  │  ├─ std_rem                           874.7 ns      │ 1.768 µs      │ 906 ns        │ 915.2 ns      │ 1000    │ 16000
│  │  ├─ std_rem_const                     43.14 ns      │ 110.3 ns      │ 51.74 ns      │ 53.63 ns      │ 1000    │ 256000
│  │  ├─ std_rem_reuse                     2.549 µs      │ 3.824 µs      │ 2.574 µs      │ 2.586 µs      │ 1000    │ 4000
│  │  ├─ unb_pow2_is_multiple_of           49 ns         │ 148.6 ns      │ 55.64 ns      │ 57.74 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_is_multiple_of_const     29.28 ns      │ 102.7 ns      │ 33.18 ns      │ 35.13 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_is_multiple_of_reuse     30.25 ns      │ 90.02 ns      │ 34.94 ns      │ 34.91 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_rem                      72.83 ns      │ 159.9 ns      │ 79.47 ns      │ 81.54 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_rem_const                49 ns         │ 146.2 ns      │ 56.04 ns      │ 58.07 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_rem_floor                52.91 ns      │ 127.5 ns      │ 61.5 ns       │ 63.36 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_rem_floor_const          41.58 ns      │ 107.6 ns      │ 42.95 ns      │ 44.5 ns       │ 1000    │ 512000
│  │  ├─ unb_pow2_rem_floor_reuse          39.63 ns      │ 85.33 ns      │ 43.54 ns      │ 45.29 ns      │ 1000    │ 256000
│  │  ╰─ unb_pow2_rem_reuse                35.72 ns      │ 145.8 ns      │ 43.54 ns      │ 44.1 ns       │ 1000    │ 128000
│  ├─ i32                                                │               │               │               │         │
│  │  ├─ baseline_identity                 43.54 ns      │ 231.4 ns      │ 47.05 ns      │ 51.85 ns      │ 1000    │ 256000
│  │  ├─ pow2_is_multiple_of               62.29 ns      │ 133.3 ns      │ 66.19 ns      │ 69.04 ns      │ 1000    │ 128000
│  │  ├─ pow2_is_multiple_of_const         47.05 ns      │ 215 ns        │ 53.3 ns       │ 54.54 ns      │ 1000    │ 256000
│  │  ├─ pow2_is_multiple_of_reuse         62.68 ns      │ 144.7 ns      │ 67.75 ns      │ 69.18 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem                          117.7 ns      │ 277.9 ns      │ 132.6 ns      │ 135.6 ns      │ 1000    │ 128000
│  │  ├─ pow2_rem_const                    97.44 ns      │ 333.3 ns      │ 106.8 ns      │ 111.8 ns      │ 1000    │ 128000
│  │  ├─ pow2_rem_floor                    94.32 ns      │ 274 ns        │ 100.5 ns      │ 103.6 ns      │ 1000    │ 128000
│  │  ├─ pow2_rem_floor_const              75.57 ns      │ 153.6 ns      │ 80.25 ns      │ 80.89 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem_floor_reuse              82.21 ns      │ 240 ns        │ 93.54 ns      │ 99.94 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem_reuse                    99 ns         │ 241.9 ns      │ 106.8 ns      │ 109.7 ns      │ 1000    │ 128000
│  │  ├─ std_rem                           1.324 µs      │ 4.337 µs      │ 1.362 µs      │ 1.385 µs      │ 1000    │ 8000
│  │  ├─ std_rem_const                     91.19 ns      │ 210.7 ns      │ 98.22 ns      │ 100.4 ns      │ 1000    │ 128000
│  │  ├─ std_rem_reuse                     1.299 µs      │ 3.024 µs      │ 1.312 µs      │ 1.325 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_is_multiple_of           59.16 ns      │ 416.9 ns      │ 65.02 ns      │ 67.77 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_is_multiple_of_const     46.66 ns      │ 761.8 ns      │ 49 ns         │ 54 ns         │ 1000    │ 256000
│  │  ├─ unb_pow2_is_multiple_of_reuse     62.68 ns      │ 605.2 ns      │ 70.88 ns      │ 76.03 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_rem                      118.5 ns      │ 1.407 µs      │ 131.8 ns      │ 146.9 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_rem_const                97.44 ns      │ 1.18 µs       │ 105.2 ns      │ 108.5 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_rem_floor                93.54 ns      │ 340.4 ns      │ 107.6 ns      │ 109.8 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_rem_floor_const          68.54 ns      │ 220.1 ns      │ 74 ns         │ 75.54 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_rem_floor_reuse          74.79 ns      │ 278.6 ns      │ 80.25 ns      │ 87.1 ns       │ 1000    │ 128000
│  │  ╰─ unb_pow2_rem_reuse                95.88 ns      │ 307.6 ns      │ 108.3 ns      │ 112.8 ns      │ 1000    │ 128000
│  ├─ i64                                                │               │               │               │         │
│  │  ├─ baseline_identity                 81.82 ns      │ 199.7 ns      │ 89.63 ns      │ 92.73 ns      │ 1000    │ 128000
│  │  ├─ pow2_is_multiple_of               129.4 ns      │ 698.2 ns      │ 138 ns        │ 141.4 ns      │ 1000    │ 128000
│  │  ├─ pow2_is_multiple_of_const         83.38 ns      │ 384.9 ns      │ 91.19 ns      │ 103.4 ns      │ 1000    │ 128000
│  │  ├─ pow2_is_multiple_of_reuse         112.2 ns      │ 424 ns        │ 117.7 ns      │ 121 ns        │ 1000    │ 128000
│  │  ├─ pow2_rem                          231 ns        │ 941.9 ns      │ 243.5 ns      │ 249.4 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem_const                    198.2 ns      │ 432.6 ns      │ 223.2 ns      │ 228.9 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem_floor                    182.6 ns      │ 454.4 ns      │ 195.1 ns      │ 201.6 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem_floor_const              149.7 ns      │ 502.9 ns      │ 168.5 ns      │ 175.4 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem_floor_reuse              162.2 ns      │ 821.6 ns      │ 174.7 ns      │ 179.3 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem_reuse                    199.7 ns      │ 460.7 ns      │ 218.5 ns      │ 223.2 ns      │ 1000    │ 64000
│  │  ├─ std_rem                           1.649 µs      │ 2.312 µs      │ 1.687 µs      │ 1.695 µs      │ 1000    │ 8000
│  │  ├─ std_rem_const                     193.5 ns      │ 452.9 ns      │ 213.8 ns      │ 217.7 ns      │ 1000    │ 64000
│  │  ├─ std_rem_reuse                     1.537 µs      │ 2.849 µs      │ 1.549 µs      │ 1.563 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_is_multiple_of           124 ns        │ 272.4 ns      │ 135.7 ns      │ 138.3 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_is_multiple_of_const     83.38 ns      │ 284.9 ns      │ 92.36 ns      │ 102.6 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_is_multiple_of_reuse     106 ns        │ 283.3 ns      │ 116.9 ns      │ 123.7 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_rem                      226.3 ns      │ 570.1 ns      │ 254.4 ns      │ 261.4 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_rem_const                193.5 ns      │ 437.2 ns      │ 207.6 ns      │ 211.9 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_rem_floor                184.1 ns      │ 606 ns        │ 201.3 ns      │ 210.8 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_rem_floor_const          149.7 ns      │ 449.7 ns      │ 165.4 ns      │ 173.7 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_rem_floor_reuse          156 ns        │ 431 ns        │ 163.8 ns      │ 168.7 ns      │ 1000    │ 64000
│  │  ╰─ unb_pow2_rem_reuse                198.2 ns      │ 507.6 ns      │ 212.2 ns      │ 220.9 ns      │ 1000    │ 64000
│  ├─ i128                                               │               │               │               │         │
│  │  ├─ baseline_identity                 229.4 ns      │ 2.37 µs       │ 256 ns        │ 272.3 ns      │ 1000    │ 64000
│  │  ├─ pow2_is_multiple_of               393.5 ns      │ 709.1 ns      │ 399.7 ns      │ 416.5 ns      │ 1000    │ 32000
│  │  ├─ pow2_is_multiple_of_const         327.9 ns      │ 640.4 ns      │ 334.1 ns      │ 348.6 ns      │ 1000    │ 32000
│  │  ├─ pow2_is_multiple_of_reuse         374.7 ns      │ 671.6 ns      │ 381 ns        │ 397 ns        │ 1000    │ 32000
│  │  ├─ pow2_rem                          1.749 µs      │ 3.312 µs      │ 1.812 µs      │ 1.884 µs      │ 1000    │ 8000
│  │  ├─ pow2_rem_const                    1.149 µs      │ 2.393 µs      │ 1.168 µs      │ 1.221 µs      │ 1000    │ 16000
│  │  ├─ pow2_rem_floor                    824.7 ns      │ 9.274 µs      │ 843.5 ns      │ 895.4 ns      │ 1000    │ 16000
│  │  ├─ pow2_rem_floor_const              449.7 ns      │ 4.771 µs      │ 491.9 ns      │ 545.3 ns      │ 1000    │ 32000
│  │  ├─ pow2_rem_floor_reuse              577.9 ns      │ 5.14 µs       │ 602.9 ns      │ 650.3 ns      │ 1000    │ 32000
│  │  ├─ pow2_rem_reuse                    1.349 µs      │ 14.78 µs      │ 1.412 µs      │ 1.476 µs      │ 1000    │ 8000
│  │  ├─ std_rem                           9.199 µs      │ 157.9 µs      │ 9.299 µs      │ 9.906 µs      │ 1000    │ 1000
│  │  ├─ std_rem_const                     768.5 ns      │ 10.36 µs      │ 793.5 ns      │ 827.7 ns      │ 1000    │ 16000
│  │  ├─ std_rem_reuse                     8.099 µs      │ 91.14 µs      │ 8.299 µs      │ 8.844 µs      │ 1000    │ 2000
│  │  ├─ unb_pow2_is_multiple_of           390.4 ns      │ 4.724 µs      │ 399.7 ns      │ 418.4 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_is_multiple_of_const     327.9 ns      │ 4.437 µs      │ 334.1 ns      │ 353.7 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_is_multiple_of_reuse     371.6 ns      │ 4.968 µs      │ 381 ns        │ 404.9 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_rem                      1.924 µs      │ 4.899 µs      │ 1.962 µs      │ 1.985 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_rem_const                1.156 µs      │ 2.137 µs      │ 1.181 µs      │ 1.197 µs      │ 1000    │ 16000
│  │  ├─ unb_pow2_rem_floor                831 ns        │ 2.081 µs      │ 856 ns        │ 867 ns        │ 1000    │ 16000
│  │  ├─ unb_pow2_rem_floor_const          459.1 ns      │ 1.568 µs      │ 487.2 ns      │ 495.3 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_rem_floor_reuse          581 ns        │ 1.281 µs      │ 609.1 ns      │ 628.9 ns      │ 1000    │ 32000
│  │  ╰─ unb_pow2_rem_reuse                1.374 µs      │ 7.412 µs      │ 1.424 µs      │ 1.5 µs        │ 1000    │ 8000
│  ├─ u8                                                 │               │               │               │         │
│  │  ├─ baseline_identity                 11.02 ns      │ 30.84 ns      │ 11.7 ns       │ 12.14 ns      │ 1000    │ 1024000
│  │  ├─ pow2_is_multiple_of               35.53 ns      │ 88.65 ns      │ 38.46 ns      │ 39.57 ns      │ 1000    │ 512000
│  │  ├─ pow2_is_multiple_of_const         19.12 ns      │ 48.81 ns      │ 20.1 ns       │ 21.01 ns      │ 1000    │ 512000
│  │  ├─ pow2_is_multiple_of_reuse         25.57 ns      │ 106 ns        │ 28.3 ns       │ 30.47 ns      │ 1000    │ 512000
│  │  ├─ pow2_rem                          31.62 ns      │ 91.19 ns      │ 38.26 ns      │ 39.42 ns      │ 1000    │ 512000
│  │  ├─ pow2_rem_const                    18.73 ns      │ 53.11 ns      │ 20.1 ns       │ 20.84 ns      │ 1000    │ 512000
│  │  ├─ pow2_rem_floor                    31.23 ns      │ 324.2 ns      │ 32.6 ns       │ 36.34 ns      │ 1000    │ 512000
│  │  ├─ pow2_rem_floor_const              21.07 ns      │ 87.77 ns      │ 25.47 ns      │ 27.15 ns      │ 1000    │ 1024000
│  │  ├─ pow2_rem_floor_reuse              20.29 ns      │ 314.6 ns      │ 22.05 ns      │ 23.73 ns      │ 1000    │ 512000
│  │  ├─ pow2_rem_reuse                    20.1 ns       │ 302.5 ns      │ 21.46 ns      │ 23.05 ns      │ 1000    │ 512000
│  │  ├─ std_rem                           856 ns        │ 3.162 µs      │ 881 ns        │ 891.2 ns      │ 1000    │ 16000
│  │  ├─ std_rem_const                     18.54 ns      │ 61.89 ns      │ 19.9 ns       │ 21.03 ns      │ 1000    │ 512000
│  │  ├─ std_rem_reuse                     843.5 ns      │ 1.693 µs      │ 856 ns        │ 858.3 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_is_multiple_of           33.96 ns      │ 114 ns        │ 36.7 ns       │ 39.6 ns       │ 1000    │ 512000
│  │  ├─ unb_pow2_is_multiple_of_const     18.93 ns      │ 274 ns        │ 19.9 ns       │ 21.54 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_is_multiple_of_reuse     24.79 ns      │ 314.8 ns      │ 26.74 ns      │ 28.14 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_rem                      33.96 ns      │ 122.4 ns      │ 39.24 ns      │ 40.61 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_rem_const                18.54 ns      │ 114.8 ns      │ 19.71 ns      │ 20.67 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_rem_floor                32.6 ns       │ 135.1 ns      │ 36.11 ns      │ 39.4 ns       │ 1000    │ 512000
│  │  ├─ unb_pow2_rem_floor_const          19.71 ns      │ 64.24 ns      │ 20.68 ns      │ 22.23 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_rem_floor_reuse          20.29 ns      │ 48.03 ns      │ 22.44 ns      │ 23.7 ns       │ 1000    │ 512000
│  │  ╰─ unb_pow2_rem_reuse                20.29 ns      │ 58.38 ns      │ 21.07 ns      │ 21.96 ns      │ 1000    │ 512000
│  ├─ u16                                                │               │               │               │         │
│  │  ├─ baseline_identity                 25.96 ns      │ 86.11 ns      │ 27.13 ns      │ 28.35 ns      │ 1000    │ 512000
│  │  ├─ pow2_is_multiple_of               50.96 ns      │ 112.6 ns      │ 55.64 ns      │ 57.04 ns      │ 1000    │ 256000
│  │  ├─ pow2_is_multiple_of_const         29.08 ns      │ 73.03 ns      │ 31.43 ns      │ 33.94 ns      │ 1000    │ 512000
│  │  ├─ pow2_is_multiple_of_reuse         35.92 ns      │ 394.1 ns      │ 41.58 ns      │ 43.45 ns      │ 1000    │ 512000
│  │  ├─ pow2_rem                          66.97 ns      │ 182.6 ns      │ 71.46 ns      │ 76.54 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem_const                    41.39 ns      │ 331.4 ns      │ 42.56 ns      │ 44.24 ns      │ 1000    │ 512000
│  │  ├─ pow2_rem_floor                    66.19 ns      │ 138.4 ns      │ 70.1 ns       │ 72.85 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem_floor_const              36.5 ns       │ 93.93 ns      │ 38.07 ns      │ 39.37 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem_floor_reuse              39.63 ns      │ 183.7 ns      │ 42.75 ns      │ 44.49 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem_reuse                    39.24 ns      │ 100.5 ns      │ 41.19 ns      │ 42.59 ns      │ 1000    │ 256000
│  │  ├─ std_rem                           862.2 ns      │ 1.174 µs      │ 881 ns        │ 887.8 ns      │ 1000    │ 16000
│  │  ├─ std_rem_const                     36.11 ns      │ 85.72 ns      │ 38.46 ns      │ 39.78 ns      │ 1000    │ 256000
│  │  ├─ std_rem_reuse                     849.7 ns      │ 1.618 µs      │ 862.2 ns      │ 869.2 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_is_multiple_of           43.93 ns      │ 108.3 ns      │ 51.35 ns      │ 52.79 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_is_multiple_of_const     28.89 ns      │ 293.3 ns      │ 30.64 ns      │ 32.12 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_is_multiple_of_reuse     37.48 ns      │ 396 ns        │ 41.58 ns      │ 46.89 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_rem                      53.3 ns       │ 636.8 ns      │ 57.21 ns      │ 61.7 ns       │ 1000    │ 256000
│  │  ├─ unb_pow2_rem_const                36.89 ns      │ 90.02 ns      │ 38.46 ns      │ 39.28 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_rem_floor                54.08 ns      │ 104.8 ns      │ 58.38 ns      │ 59.4 ns       │ 1000    │ 256000
│  │  ├─ unb_pow2_rem_floor_const          41.58 ns      │ 334.3 ns      │ 42.95 ns      │ 45.22 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_rem_floor_reuse          39.63 ns      │ 121.2 ns      │ 43.54 ns      │ 44.97 ns      │ 1000    │ 256000
│  │  ╰─ unb_pow2_rem_reuse                40.02 ns      │ 99.79 ns      │ 43.54 ns      │ 44.68 ns      │ 1000    │ 256000
│  ├─ u32                                                │               │               │               │         │
│  │  ├─ baseline_identity                 43.93 ns      │ 111.5 ns      │ 46.27 ns      │ 49.64 ns      │ 1000    │ 256000
│  │  ├─ pow2_is_multiple_of               66.19 ns      │ 1.182 µs      │ 72.44 ns      │ 77.04 ns      │ 1000    │ 128000
│  │  ├─ pow2_is_multiple_of_const         47.05 ns      │ 255.6 ns      │ 51.35 ns      │ 54.83 ns      │ 1000    │ 256000
│  │  ├─ pow2_is_multiple_of_reuse         63.85 ns      │ 157.6 ns      │ 71.66 ns      │ 73.95 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem                          93.54 ns      │ 219.3 ns      │ 106.8 ns      │ 110.3 ns      │ 1000    │ 128000
│  │  ├─ pow2_rem_const                    70.1 ns       │ 1.043 µs      │ 74 ns         │ 78.32 ns      │ 1000    │ 128000
│  │  ├─ pow2_rem_floor                    92.75 ns      │ 1.255 µs      │ 101.3 ns      │ 107.9 ns      │ 1000    │ 128000
│  │  ├─ pow2_rem_floor_const              75.18 ns      │ 627.9 ns      │ 81.43 ns      │ 86.31 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem_floor_reuse              81.04 ns      │ 240 ns        │ 91.58 ns      │ 95.56 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem_reuse                    81.82 ns      │ 230.6 ns      │ 85.72 ns      │ 90.85 ns      │ 1000    │ 256000
│  │  ├─ std_rem                           1.324 µs      │ 2.099 µs      │ 1.362 µs      │ 1.362 µs      │ 1000    │ 8000
│  │  ├─ std_rem_const                     75.96 ns      │ 159.9 ns      │ 80.64 ns      │ 84.51 ns      │ 1000    │ 256000
│  │  ├─ std_rem_reuse                     1.287 µs      │ 1.962 µs      │ 1.324 µs      │ 1.323 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_is_multiple_of           59.94 ns      │ 134.1 ns      │ 66.19 ns      │ 68.1 ns       │ 1000    │ 128000
│  │  ├─ unb_pow2_is_multiple_of_const     47.05 ns      │ 137.2 ns      │ 50.18 ns      │ 53.04 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_is_multiple_of_reuse     48.22 ns      │ 108.3 ns      │ 54.47 ns      │ 55.81 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_rem                      92.75 ns      │ 999 ns        │ 103.6 ns      │ 116.1 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_rem_const                75.57 ns      │ 454.4 ns      │ 80.25 ns      │ 82.75 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_rem_floor                93.54 ns      │ 375.5 ns      │ 104.4 ns      │ 107.2 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_rem_floor_const          68.54 ns      │ 201.3 ns      │ 74.79 ns      │ 77.94 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_rem_floor_reuse          73.22 ns      │ 224.7 ns      │ 80.25 ns      │ 84.46 ns      │ 1000    │ 128000
│  │  ╰─ unb_pow2_rem_reuse                74 ns         │ 224 ns        │ 81.82 ns      │ 85.91 ns      │ 1000    │ 128000
│  ├─ u64                                                │               │               │               │         │
│  │  ├─ baseline_identity                 81.82 ns      │ 203.6 ns      │ 95.88 ns      │ 99.64 ns      │ 1000    │ 128000
│  │  ├─ pow2_is_multiple_of               127.1 ns      │ 279.4 ns      │ 139.6 ns      │ 142.2 ns      │ 1000    │ 128000
│  │  ├─ pow2_is_multiple_of_const         82.6 ns       │ 195.8 ns      │ 85.72 ns      │ 90.66 ns      │ 1000    │ 128000
│  │  ├─ pow2_is_multiple_of_reuse         108.3 ns      │ 299 ns        │ 116.1 ns      │ 120.4 ns      │ 1000    │ 128000
│  │  ├─ pow2_rem                          177.9 ns      │ 452.9 ns      │ 202.9 ns      │ 208.5 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem_const                    151.3 ns      │ 431 ns        │ 171.6 ns      │ 177.5 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem_floor                    174.7 ns      │ 579.4 ns      │ 193.5 ns      │ 202 ns        │ 1000    │ 64000
│  │  ├─ pow2_rem_floor_const              149.7 ns      │ 523.2 ns      │ 173.2 ns      │ 191.3 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem_floor_reuse              154.4 ns      │ 449.7 ns      │ 168.5 ns      │ 175.5 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem_reuse                    160.7 ns      │ 316.9 ns      │ 172.4 ns      │ 177.2 ns      │ 1000    │ 64000
│  │  ├─ std_rem                           1.649 µs      │ 3.062 µs      │ 1.687 µs      │ 1.697 µs      │ 1000    │ 8000
│  │  ├─ std_rem_const                     151.3 ns      │ 479.4 ns      │ 163.8 ns      │ 167.6 ns      │ 1000    │ 64000
│  │  ├─ std_rem_reuse                     1.549 µs      │ 2.974 µs      │ 1.574 µs      │ 1.578 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_is_multiple_of           122.4 ns      │ 323.2 ns      │ 132.6 ns      │ 134.8 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_is_multiple_of_const     83.38 ns      │ 218.5 ns      │ 87.29 ns      │ 90.59 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_is_multiple_of_reuse     109.1 ns      │ 291.9 ns      │ 128.6 ns      │ 133 ns        │ 1000    │ 128000
│  │  ├─ unb_pow2_rem                      176.3 ns      │ 2.062 µs      │ 204.4 ns      │ 219 ns        │ 1000    │ 64000
│  │  ├─ unb_pow2_rem_const                160.7 ns      │ 427.1 ns      │ 201.3 ns      │ 209.5 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_rem_floor                184.1 ns      │ 657.6 ns      │ 202.9 ns      │ 214.2 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_rem_floor_const          151.3 ns      │ 1.721 µs      │ 166.9 ns      │ 178.8 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_rem_floor_reuse          154.4 ns      │ 2.443 µs      │ 162.2 ns      │ 171.9 ns      │ 1000    │ 64000
│  │  ╰─ unb_pow2_rem_reuse                156 ns        │ 766.9 ns      │ 162.2 ns      │ 166.2 ns      │ 1000    │ 64000
│  ╰─ u128                                               │               │               │               │         │
│     ├─ baseline_identity                 201.3 ns      │ 2.238 µs      │ 234.1 ns      │ 249 ns        │ 1000    │ 64000
│     ├─ pow2_is_multiple_of               387.2 ns      │ 709.1 ns      │ 396.6 ns      │ 401.2 ns      │ 1000    │ 32000
│     ├─ pow2_is_multiple_of_const         327.9 ns      │ 987.2 ns      │ 334.1 ns      │ 337.8 ns      │ 1000    │ 32000
│     ├─ pow2_is_multiple_of_reuse         374.7 ns      │ 909.1 ns      │ 387.2 ns      │ 391.8 ns      │ 1000    │ 32000
│     ├─ pow2_rem                          837.2 ns      │ 10.37 µs      │ 862.2 ns      │ 884.9 ns      │ 1000    │ 16000
│     ├─ pow2_rem_const                    402.9 ns      │ 5.368 µs      │ 421.6 ns      │ 452.2 ns      │ 1000    │ 32000
│     ├─ pow2_rem_floor                    831 ns        │ 1.399 µs      │ 849.7 ns      │ 863.8 ns      │ 1000    │ 16000
│     ├─ pow2_rem_floor_const              402.9 ns      │ 1.006 µs      │ 421.6 ns      │ 438.2 ns      │ 1000    │ 32000
│     ├─ pow2_rem_floor_reuse              552.9 ns      │ 1.09 µs       │ 571.6 ns      │ 583.4 ns      │ 1000    │ 32000
│     ├─ pow2_rem_reuse                    546.6 ns      │ 987.2 ns      │ 571.6 ns      │ 586 ns        │ 1000    │ 32000
│     ├─ std_rem                           4.124 µs      │ 8.024 µs      │ 4.199 µs      │ 4.299 µs      │ 1000    │ 4000
│     ├─ std_rem_const                     402.9 ns      │ 1.268 µs      │ 434.1 ns      │ 460 ns        │ 1000    │ 32000
│     ├─ std_rem_reuse                     4.324 µs      │ 8.199 µs      │ 4.374 µs      │ 4.418 µs      │ 1000    │ 4000
│     ├─ unb_pow2_is_multiple_of           390.4 ns      │ 943.5 ns      │ 399.7 ns      │ 404.4 ns      │ 1000    │ 32000
│     ├─ unb_pow2_is_multiple_of_const     327.9 ns      │ 840.4 ns      │ 334.1 ns      │ 338.5 ns      │ 1000    │ 32000
│     ├─ unb_pow2_is_multiple_of_reuse     374.7 ns      │ 790.4 ns      │ 384.1 ns      │ 387.6 ns      │ 1000    │ 32000
│     ├─ unb_pow2_rem                      824.7 ns      │ 1.606 µs      │ 843.5 ns      │ 853.7 ns      │ 1000    │ 16000
│     ├─ unb_pow2_rem_const                412.2 ns      │ 1.052 µs      │ 446.6 ns      │ 459.5 ns      │ 1000    │ 32000
│     ├─ unb_pow2_rem_floor                824.7 ns      │ 1.643 µs      │ 843.5 ns      │ 855.6 ns      │ 1000    │ 16000
│     ├─ unb_pow2_rem_floor_const          412.2 ns      │ 899.7 ns      │ 431 ns        │ 445.6 ns      │ 1000    │ 32000
│     ├─ unb_pow2_rem_floor_reuse          562.2 ns      │ 1.312 µs      │ 599.7 ns      │ 612.1 ns      │ 1000    │ 32000
│     ╰─ unb_pow2_rem_reuse                556 ns        │ 1.237 µs      │ 584.1 ns      │ 612.6 ns      │ 1000    │ 32000
╰─ round                                                 │               │               │               │         │
   ├─ i8                                                 │               │               │               │         │
   │  ├─ baseline_identity                 11.02 ns      │ 152.2 ns      │ 11.6 ns       │ 12.51 ns      │ 1000    │ 1024000
   │  ├─ pow2_ceil_to_multiple             36.31 ns      │ 95.1 ns       │ 39.43 ns      │ 41.87 ns      │ 1000    │ 512000
   │  ├─ pow2_ceil_to_multiple_const       19.51 ns      │ 65.02 ns      │ 20.49 ns      │ 21.05 ns      │ 1000    │ 512000
   │  ├─ pow2_ceil_to_multiple_reuse       20.88 ns      │ 106.6 ns      │ 23.81 ns      │ 26.16 ns      │ 1000    │ 512000
   │  ├─ pow2_floor_to_multiple            30.64 ns      │ 66.39 ns      │ 32.01 ns      │ 33.61 ns      │ 1000    │ 512000
   │  ├─ pow2_floor_to_multiple_const      19.32 ns      │ 42.75 ns      │ 20.49 ns      │ 21.57 ns      │ 1000    │ 512000
   │  ├─ pow2_floor_to_multiple_reuse      21.07 ns      │ 95.88 ns      │ 23.22 ns      │ 24.29 ns      │ 1000    │ 512000
   │  ├─ pow2_round_to_multiple            31.43 ns      │ 90.02 ns      │ 37.29 ns      │ 38.26 ns      │ 1000    │ 256000
   │  ├─ pow2_round_to_multiple_const      22.25 ns      │ 85.14 ns      │ 25.18 ns      │ 26.68 ns      │ 1000    │ 512000
   │  ├─ pow2_round_to_multiple_reuse      23.03 ns      │ 101.3 ns      │ 26.15 ns      │ 27.93 ns      │ 1000    │ 512000
   │  ├─ std_div_mul                       868.5 ns      │ 1.318 µs      │ 899.7 ns      │ 919.3 ns      │ 1000    │ 16000
   │  ├─ std_div_mul_const                 20.68 ns      │ 159.1 ns      │ 22.64 ns      │ 23.6 ns       │ 1000    │ 512000
   │  ├─ std_div_mul_reuse                 843.5 ns      │ 1.843 µs      │ 856 ns        │ 863 ns        │ 1000    │ 16000
   │  ├─ unb_pow2_ceil_to_multiple         34.94 ns      │ 111.5 ns      │ 41.58 ns      │ 43.55 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_ceil_to_multiple_const   18.73 ns      │ 61.11 ns      │ 21.07 ns      │ 23.26 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   20.88 ns      │ 91.19 ns      │ 23.42 ns      │ 25.28 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_floor_to_multiple        33.77 ns      │ 102.1 ns      │ 35.72 ns      │ 36.09 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_floor_to_multiple_const  18.54 ns      │ 55.06 ns      │ 20.88 ns      │ 22.19 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_floor_to_multiple_reuse  20.49 ns      │ 76.54 ns      │ 22.64 ns      │ 23.51 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_round_to_multiple        30.64 ns      │ 601.3 ns      │ 37.68 ns      │ 39.34 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_round_to_multiple_const  21.07 ns      │ 310.7 ns      │ 24 ns         │ 25.41 ns      │ 1000    │ 512000
   │  ╰─ unb_pow2_round_to_multiple_reuse  23.42 ns      │ 79.67 ns      │ 25.76 ns      │ 26.38 ns      │ 1000    │ 512000
   ├─ i16                                                │               │               │               │         │
   │  ├─ baseline_identity                 26.35 ns      │ 134.1 ns      │ 27.91 ns      │ 31.86 ns      │ 1000    │ 512000
   │  ├─ pow2_ceil_to_multiple             66.97 ns      │ 188.4 ns      │ 70.1 ns       │ 74.81 ns      │ 1000    │ 256000
   │  ├─ pow2_ceil_to_multiple_const       38.85 ns      │ 83.38 ns      │ 41.97 ns      │ 43.56 ns      │ 1000    │ 256000
   │  ├─ pow2_ceil_to_multiple_reuse       41.58 ns      │ 104.8 ns      │ 45.49 ns      │ 46.92 ns      │ 1000    │ 256000
   │  ├─ pow2_floor_to_multiple            66.19 ns      │ 238 ns        │ 72.44 ns      │ 78.91 ns      │ 1000    │ 256000
   │  ├─ pow2_floor_to_multiple_const      36.11 ns      │ 115.4 ns      │ 38.46 ns      │ 39.57 ns      │ 1000    │ 256000
   │  ├─ pow2_floor_to_multiple_reuse      39.24 ns      │ 113 ns        │ 40.8 ns       │ 42.11 ns      │ 1000    │ 256000
   │  ├─ pow2_round_to_multiple            67.75 ns      │ 535.7 ns      │ 74 ns         │ 77.58 ns      │ 1000    │ 256000
   │  ├─ pow2_round_to_multiple_const      43.14 ns      │ 631.8 ns      │ 49 ns         │ 51.39 ns      │ 1000    │ 256000
   │  ├─ pow2_round_to_multiple_reuse      45.1 ns       │ 643.9 ns      │ 49 ns         │ 52.3 ns       │ 1000    │ 256000
   │  ├─ std_div_mul                       2.649 µs      │ 37.04 µs      │ 2.674 µs      │ 2.823 µs      │ 1000    │ 4000
   │  ├─ std_div_mul_const                 41.58 ns      │ 89.24 ns      │ 45.49 ns      │ 46.26 ns      │ 1000    │ 256000
   │  ├─ std_div_mul_reuse                 2.624 µs      │ 4.649 µs      │ 2.649 µs      │ 2.672 µs      │ 1000    │ 4000
   │  ├─ unb_pow2_ceil_to_multiple         56.04 ns      │ 166.5 ns      │ 59.55 ns      │ 60.95 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_ceil_to_multiple_const   37.29 ns      │ 337.6 ns      │ 40.8 ns       │ 41.95 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   40.8 ns       │ 120.4 ns      │ 45.1 ns       │ 46.07 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_floor_to_multiple        53.69 ns      │ 628.3 ns      │ 57.6 ns       │ 66.34 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_floor_to_multiple_const  36.11 ns      │ 129 ns        │ 38.07 ns      │ 39.46 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_floor_to_multiple_reuse  39.24 ns      │ 131 ns        │ 41.58 ns      │ 43.36 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_round_to_multiple        72.83 ns      │ 673.6 ns      │ 80.25 ns      │ 84.33 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_round_to_multiple_const  43.54 ns      │ 162.2 ns      │ 47.83 ns      │ 50.01 ns      │ 1000    │ 256000
   │  ╰─ unb_pow2_round_to_multiple_reuse  45.1 ns       │ 233.7 ns      │ 49.39 ns      │ 50.85 ns      │ 1000    │ 256000
   ├─ i32                                                │               │               │               │         │
   │  ├─ baseline_identity                 43.54 ns      │ 206.8 ns      │ 52.13 ns      │ 54.34 ns      │ 1000    │ 256000
   │  ├─ pow2_ceil_to_multiple             98.22 ns      │ 1.05 µs       │ 110.7 ns      │ 116.1 ns      │ 1000    │ 128000
   │  ├─ pow2_ceil_to_multiple_const       74 ns         │ 259.9 ns      │ 82.6 ns       │ 89.04 ns      │ 1000    │ 128000
   │  ├─ pow2_ceil_to_multiple_reuse       77.13 ns      │ 363.8 ns      │ 83.38 ns      │ 87.82 ns      │ 1000    │ 128000
   │  ├─ pow2_floor_to_multiple            92.75 ns      │ 232.6 ns      │ 99 ns         │ 101.7 ns      │ 1000    │ 128000
   │  ├─ pow2_floor_to_multiple_const      75.57 ns      │ 202.1 ns      │ 80.64 ns      │ 84.73 ns      │ 1000    │ 256000
   │  ├─ pow2_floor_to_multiple_reuse      81.82 ns      │ 182.6 ns      │ 85.33 ns      │ 86.97 ns      │ 1000    │ 256000
   │  ├─ pow2_round_to_multiple            122.4 ns      │ 239.6 ns      │ 134.9 ns      │ 137.2 ns      │ 1000    │ 128000
   │  ├─ pow2_round_to_multiple_const      86.5 ns       │ 223.2 ns      │ 95.88 ns      │ 98.24 ns      │ 1000    │ 128000
   │  ├─ pow2_round_to_multiple_reuse      97.44 ns      │ 290.4 ns      │ 106.8 ns      │ 110.3 ns      │ 1000    │ 128000
   │  ├─ std_div_mul                       1.337 µs      │ 3.174 µs      │ 1.362 µs      │ 1.389 µs      │ 1000    │ 8000
   │  ├─ std_div_mul_const                 83.38 ns      │ 234.9 ns      │ 91.97 ns      │ 94.15 ns      │ 1000    │ 128000
   │  ├─ std_div_mul_reuse                 1.287 µs      │ 19.38 µs      │ 1.312 µs      │ 1.343 µs      │ 1000    │ 8000
   │  ├─ unb_pow2_ceil_to_multiple         100.5 ns      │ 1.272 µs      │ 109.9 ns      │ 116 ns        │ 1000    │ 128000
   │  ├─ unb_pow2_ceil_to_multiple_const   74 ns         │ 290.4 ns      │ 84.94 ns      │ 89.13 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   78.69 ns      │ 195.8 ns      │ 86.5 ns       │ 90.23 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_floor_to_multiple        95.1 ns       │ 1.005 µs      │ 102.9 ns      │ 107.6 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_floor_to_multiple_const  75.96 ns      │ 284.5 ns      │ 81.04 ns      │ 86.98 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_floor_to_multiple_reuse  81.43 ns      │ 209.1 ns      │ 85.72 ns      │ 90.83 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_round_to_multiple        127.9 ns      │ 282.6 ns      │ 147 ns        │ 149.8 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_round_to_multiple_const  88.07 ns      │ 203.6 ns      │ 95.88 ns      │ 97.54 ns      │ 1000    │ 128000
   │  ╰─ unb_pow2_round_to_multiple_reuse  95.1 ns       │ 307.6 ns      │ 106.8 ns      │ 113.2 ns      │ 1000    │ 128000
   ├─ i64                                                │               │               │               │         │
   │  ├─ baseline_identity                 81.04 ns      │ 219.3 ns      │ 90.41 ns      │ 96.37 ns      │ 1000    │ 128000
   │  ├─ pow2_ceil_to_multiple             184.1 ns      │ 574.7 ns      │ 199.7 ns      │ 204 ns        │ 1000    │ 64000
   │  ├─ pow2_ceil_to_multiple_const       157.6 ns      │ 454.4 ns      │ 174.7 ns      │ 178.7 ns      │ 1000    │ 64000
   │  ├─ pow2_ceil_to_multiple_reuse       163.8 ns      │ 1.56 µs       │ 176.3 ns      │ 182.7 ns      │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple            173.2 ns      │ 1.898 µs      │ 190.4 ns      │ 196 ns        │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple_const      152.9 ns      │ 1.685 µs      │ 170.1 ns      │ 177.4 ns      │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple_reuse      157.6 ns      │ 501.3 ns      │ 165.4 ns      │ 170.3 ns      │ 1000    │ 64000
   │  ├─ pow2_round_to_multiple            223.2 ns      │ 552.9 ns      │ 251.3 ns      │ 257.8 ns      │ 1000    │ 64000
   │  ├─ pow2_round_to_multiple_const      171.6 ns      │ 665.4 ns      │ 188.8 ns      │ 194.7 ns      │ 1000    │ 64000
   │  ├─ pow2_round_to_multiple_reuse      190.4 ns      │ 885.7 ns      │ 209.1 ns      │ 218.9 ns      │ 1000    │ 64000
   │  ├─ std_div_mul                       1.649 µs      │ 5.837 µs      │ 1.687 µs      │ 1.724 µs      │ 1000    │ 8000
   │  ├─ std_div_mul_const                 173.2 ns      │ 491.9 ns      │ 201.3 ns      │ 208.4 ns      │ 1000    │ 64000
   │  ├─ std_div_mul_reuse                 1.537 µs      │ 3.137 µs      │ 1.562 µs      │ 1.574 µs      │ 1000    │ 8000
   │  ├─ unb_pow2_ceil_to_multiple         191.9 ns      │ 2.54 µs       │ 209.1 ns      │ 216.7 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_ceil_to_multiple_const   159.1 ns      │ 2.454 µs      │ 173.2 ns      │ 185.2 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   162.2 ns      │ 356 ns        │ 173.2 ns      │ 177.4 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple        181 ns        │ 545.1 ns      │ 206 ns        │ 216 ns        │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple_const  151.3 ns      │ 2.429 µs      │ 168.5 ns      │ 182.5 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple_reuse  157.6 ns      │ 2.159 µs      │ 166.9 ns      │ 173.8 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_round_to_multiple        245.1 ns      │ 2.332 µs      │ 271.6 ns      │ 282.8 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_round_to_multiple_const  177.9 ns      │ 445.1 ns      │ 198.2 ns      │ 201.2 ns      │ 1000    │ 64000
   │  ╰─ unb_pow2_round_to_multiple_reuse  191.9 ns      │ 776.3 ns      │ 218.5 ns      │ 230.2 ns      │ 1000    │ 64000
   ├─ i128                                               │               │               │               │         │
   │  ├─ baseline_identity                 227.9 ns      │ 607.6 ns      │ 251.3 ns      │ 260.5 ns      │ 1000    │ 64000
   │  ├─ pow2_ceil_to_multiple             1.049 µs      │ 39.34 µs      │ 1.099 µs      │ 1.267 µs      │ 1000    │ 4000
   │  ├─ pow2_ceil_to_multiple_const       612.2 ns      │ 9.218 µs      │ 624.7 ns      │ 658.4 ns      │ 1000    │ 16000
   │  ├─ pow2_ceil_to_multiple_reuse       681 ns        │ 3.362 µs      │ 706 ns        │ 721.3 ns      │ 1000    │ 16000
   │  ├─ pow2_floor_to_multiple            806 ns        │ 10.53 µs      │ 824.7 ns      │ 894.1 ns      │ 1000    │ 16000
   │  ├─ pow2_floor_to_multiple_const      440.4 ns      │ 3.299 µs      │ 465.4 ns      │ 481 ns        │ 1000    │ 32000
   │  ├─ pow2_floor_to_multiple_reuse      565.4 ns      │ 4.909 µs      │ 606 ns        │ 646.8 ns      │ 1000    │ 32000
   │  ├─ pow2_round_to_multiple            2.437 µs      │ 20.71 µs      │ 2.487 µs      │ 2.702 µs      │ 1000    │ 8000
   │  ├─ pow2_round_to_multiple_const      718.5 ns      │ 10.37 µs      │ 731 ns        │ 775.8 ns      │ 1000    │ 16000
   │  ├─ pow2_round_to_multiple_reuse      874.7 ns      │ 12.98 µs      │ 893.5 ns      │ 981.6 ns      │ 1000    │ 16000
   │  ├─ std_div_mul                       9.699 µs      │ 146.7 µs      │ 9.899 µs      │ 10.7 µs       │ 1000    │ 1000
   │  ├─ std_div_mul_const                 712.2 ns      │ 10.09 µs      │ 731 ns        │ 775.3 ns      │ 1000    │ 16000
   │  ├─ std_div_mul_reuse                 8.799 µs      │ 132.8 µs      │ 9.099 µs      │ 9.41 µs       │ 1000    │ 1000
   │  ├─ unb_pow2_ceil_to_multiple         1.118 µs      │ 9.068 µs      │ 1.149 µs      │ 1.201 µs      │ 1000    │ 16000
   │  ├─ unb_pow2_ceil_to_multiple_const   643.5 ns      │ 5.19 µs       │ 687.2 ns      │ 733.3 ns      │ 1000    │ 32000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   674.7 ns      │ 1.318 µs      │ 699.7 ns      │ 710.9 ns      │ 1000    │ 16000
   │  ├─ unb_pow2_floor_to_multiple        818.5 ns      │ 1.774 µs      │ 837.2 ns      │ 852.1 ns      │ 1000    │ 16000
   │  ├─ unb_pow2_floor_to_multiple_const  440.4 ns      │ 968.5 ns      │ 468.5 ns      │ 476.6 ns      │ 1000    │ 32000
   │  ├─ unb_pow2_floor_to_multiple_reuse  568.5 ns      │ 1.162 µs      │ 593.5 ns      │ 605.7 ns      │ 1000    │ 32000
   │  ├─ unb_pow2_round_to_multiple        2.574 µs      │ 5.349 µs      │ 2.599 µs      │ 2.632 µs      │ 1000    │ 4000
   │  ├─ unb_pow2_round_to_multiple_const  718.5 ns      │ 2.412 µs      │ 743.5 ns      │ 750.5 ns      │ 1000    │ 16000
   │  ╰─ unb_pow2_round_to_multiple_reuse  881 ns        │ 1.849 µs      │ 893.5 ns      │ 904.6 ns      │ 1000    │ 16000
   ├─ u8                                                 │               │               │               │         │
   │  ├─ baseline_identity                 11.11 ns      │ 28.59 ns      │ 11.7 ns       │ 12.09 ns      │ 1000    │ 1024000
   │  ├─ pow2_ceil_to_multiple             36.11 ns      │ 101.3 ns      │ 38.85 ns      │ 40.89 ns      │ 1000    │ 512000
   │  ├─ pow2_ceil_to_multiple_const       18.93 ns      │ 59.36 ns      │ 20.88 ns      │ 21.68 ns      │ 1000    │ 512000
   │  ├─ pow2_ceil_to_multiple_reuse       20.88 ns      │ 63.26 ns      │ 22.05 ns      │ 22.53 ns      │ 1000    │ 512000
   │  ├─ pow2_floor_to_multiple            31.04 ns      │ 322.4 ns      │ 34.55 ns      │ 37.51 ns      │ 1000    │ 512000
   │  ├─ pow2_floor_to_multiple_const      21.17 ns      │ 76.84 ns      │ 21.86 ns      │ 22.58 ns      │ 1000    │ 1024000
   │  ├─ pow2_floor_to_multiple_reuse      20.29 ns      │ 53.3 ns       │ 22.05 ns      │ 23.2 ns       │ 1000    │ 512000
   │  ├─ pow2_round_to_multiple            38.07 ns      │ 297 ns        │ 41.39 ns      │ 44.34 ns      │ 1000    │ 512000
   │  ├─ pow2_round_to_multiple_const      19.32 ns      │ 48.61 ns      │ 20.29 ns      │ 20.66 ns      │ 1000    │ 512000
   │  ├─ pow2_round_to_multiple_reuse      21.07 ns      │ 59.55 ns      │ 23.22 ns      │ 23.9 ns       │ 1000    │ 512000
   │  ├─ std_div_mul                       856 ns        │ 12.09 µs      │ 874.7 ns      │ 940.1 ns      │ 1000    │ 16000
   │  ├─ std_div_mul_const                 21.17 ns      │ 213.9 ns      │ 21.86 ns      │ 22.76 ns      │ 1000    │ 1024000
   │  ├─ std_div_mul_reuse                 887.2 ns      │ 10.98 µs      │ 924.7 ns      │ 1.007 µs      │ 1000    │ 16000
   │  ├─ unb_pow2_ceil_to_multiple         28.3 ns       │ 726.3 ns      │ 32.21 ns      │ 39.46 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_ceil_to_multiple_const   19.12 ns      │ 318.5 ns      │ 20.88 ns      │ 24.08 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   21.07 ns      │ 324.5 ns      │ 24 ns         │ 26.47 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_floor_to_multiple        33.38 ns      │ 77.13 ns      │ 35.33 ns      │ 36.03 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_floor_to_multiple_const  18.54 ns      │ 41.19 ns      │ 19.51 ns      │ 20.3 ns       │ 1000    │ 512000
   │  ├─ unb_pow2_floor_to_multiple_reuse  20.49 ns      │ 83.96 ns      │ 21.86 ns      │ 22.98 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_round_to_multiple        29.86 ns      │ 104.8 ns      │ 34.94 ns      │ 35.94 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_round_to_multiple_const  19.12 ns      │ 57.4 ns       │ 20.88 ns      │ 21.37 ns      │ 1000    │ 512000
   │  ╰─ unb_pow2_round_to_multiple_reuse  16.19 ns      │ 114.6 ns      │ 17.75 ns      │ 18.71 ns      │ 1000    │ 128000
   ├─ u16                                                │               │               │               │         │
   │  ├─ baseline_identity                 26.35 ns      │ 288.8 ns      │ 29.08 ns      │ 32.41 ns      │ 1000    │ 512000
   │  ├─ pow2_ceil_to_multiple             66.58 ns      │ 143.9 ns      │ 68.93 ns      │ 70.69 ns      │ 1000    │ 256000
   │  ├─ pow2_ceil_to_multiple_const       38.85 ns      │ 124.7 ns      │ 41.58 ns      │ 42.4 ns       │ 1000    │ 256000
   │  ├─ pow2_ceil_to_multiple_reuse       41.19 ns      │ 109.1 ns      │ 45.49 ns      │ 46.4 ns       │ 1000    │ 256000
   │  ├─ pow2_floor_to_multiple            67.36 ns      │ 234.5 ns      │ 74.79 ns      │ 80.57 ns      │ 1000    │ 256000
   │  ├─ pow2_floor_to_multiple_const      37.68 ns      │ 96.66 ns      │ 39.24 ns      │ 40.62 ns      │ 1000    │ 256000
   │  ├─ pow2_floor_to_multiple_reuse      39.63 ns      │ 114.6 ns      │ 43.14 ns      │ 45.39 ns      │ 1000    │ 256000
   │  ├─ pow2_round_to_multiple            69.71 ns      │ 586.1 ns      │ 72.83 ns      │ 75.95 ns      │ 1000    │ 256000
   │  ├─ pow2_round_to_multiple_const      38.07 ns      │ 110.3 ns      │ 41.19 ns      │ 41.71 ns      │ 1000    │ 256000
   │  ├─ pow2_round_to_multiple_reuse      41.19 ns      │ 272.8 ns      │ 46.27 ns      │ 49.91 ns      │ 1000    │ 256000
   │  ├─ std_div_mul                       887.2 ns      │ 10.49 µs      │ 893.5 ns      │ 924.2 ns      │ 1000    │ 16000
   │  ├─ std_div_mul_const                 41.58 ns      │ 361.3 ns      │ 43.14 ns      │ 45.57 ns      │ 1000    │ 512000
   │  ├─ std_div_mul_reuse                 843.5 ns      │ 1.224 µs      │ 856 ns        │ 860.2 ns      │ 1000    │ 16000
   │  ├─ unb_pow2_ceil_to_multiple         56.82 ns      │ 119.3 ns      │ 59.75 ns      │ 61.57 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_ceil_to_multiple_const   37.68 ns      │ 74.79 ns      │ 41.19 ns      │ 41.57 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   40.8 ns       │ 138.4 ns      │ 45.49 ns      │ 47.22 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_floor_to_multiple        55.25 ns      │ 136.8 ns      │ 64.63 ns      │ 66 ns         │ 1000    │ 256000
   │  ├─ unb_pow2_floor_to_multiple_const  36.11 ns      │ 168.1 ns      │ 40.02 ns      │ 41.61 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_floor_to_multiple_reuse  40.02 ns      │ 162.6 ns      │ 44.71 ns      │ 48.34 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_round_to_multiple        69.71 ns      │ 233.7 ns      │ 77.52 ns      │ 81.98 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_round_to_multiple_const  37.68 ns      │ 703.6 ns      │ 41.19 ns      │ 44.78 ns      │ 1000    │ 256000
   │  ╰─ unb_pow2_round_to_multiple_reuse  40.8 ns       │ 107.6 ns      │ 43.93 ns      │ 44.66 ns      │ 1000    │ 256000
   ├─ u32                                                │               │               │               │         │
   │  ├─ baseline_identity                 43.14 ns      │ 138.8 ns      │ 50.57 ns      │ 51.59 ns      │ 1000    │ 256000
   │  ├─ pow2_ceil_to_multiple             99 ns         │ 1.181 µs      │ 107.6 ns      │ 112.2 ns      │ 1000    │ 128000
   │  ├─ pow2_ceil_to_multiple_const       77.13 ns      │ 386.1 ns      │ 85.33 ns      │ 90.6 ns       │ 1000    │ 256000
   │  ├─ pow2_ceil_to_multiple_reuse       77.91 ns      │ 311.5 ns      │ 89.63 ns      │ 93.74 ns      │ 1000    │ 128000
   │  ├─ pow2_floor_to_multiple            91.97 ns      │ 372.4 ns      │ 102.9 ns      │ 106 ns        │ 1000    │ 128000
   │  ├─ pow2_floor_to_multiple_const      70.1 ns       │ 280.2 ns      │ 74.79 ns      │ 77.14 ns      │ 1000    │ 128000
   │  ├─ pow2_floor_to_multiple_reuse      74.79 ns      │ 281 ns        │ 84.16 ns      │ 90.33 ns      │ 1000    │ 128000
   │  ├─ pow2_round_to_multiple            110.7 ns      │ 292.7 ns      │ 127.1 ns      │ 128.7 ns      │ 1000    │ 128000
   │  ├─ pow2_round_to_multiple_const      70.88 ns      │ 201.3 ns      │ 81.04 ns      │ 83.16 ns      │ 1000    │ 128000
   │  ├─ pow2_round_to_multiple_reuse      79.47 ns      │ 224 ns        │ 86.5 ns       │ 88.59 ns      │ 1000    │ 128000
   │  ├─ std_div_mul                       1.337 µs      │ 2.762 µs      │ 1.362 µs      │ 1.369 µs      │ 1000    │ 8000
   │  ├─ std_div_mul_const                 76.35 ns      │ 513 ns        │ 91.19 ns      │ 96.69 ns      │ 1000    │ 256000
   │  ├─ std_div_mul_reuse                 1.299 µs      │ 3.037 µs      │ 1.312 µs      │ 1.321 µs      │ 1000    │ 8000
   │  ├─ unb_pow2_ceil_to_multiple         98.22 ns      │ 1.182 µs      │ 109.9 ns      │ 117.2 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_ceil_to_multiple_const   74.79 ns      │ 238.8 ns      │ 86.11 ns      │ 90.1 ns       │ 1000    │ 128000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   78.69 ns      │ 248.2 ns      │ 84.94 ns      │ 88.85 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_floor_to_multiple        93.54 ns      │ 350.5 ns      │ 101.3 ns      │ 103 ns        │ 1000    │ 128000
   │  ├─ unb_pow2_floor_to_multiple_const  75.96 ns      │ 536.1 ns      │ 81.04 ns      │ 83.42 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_floor_to_multiple_reuse  74.79 ns      │ 226.3 ns      │ 84.94 ns      │ 86.85 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_round_to_multiple        109.1 ns      │ 246.6 ns      │ 122.4 ns      │ 126.2 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_round_to_multiple_const  77.91 ns      │ 377.1 ns      │ 88.07 ns      │ 92.32 ns      │ 1000    │ 256000
   │  ╰─ unb_pow2_round_to_multiple_reuse  77.13 ns      │ 268.5 ns      │ 88.07 ns      │ 90.38 ns      │ 1000    │ 128000
   ├─ u64                                                │               │               │               │         │
   │  ├─ baseline_identity                 81.82 ns      │ 385.7 ns      │ 101.3 ns      │ 105.9 ns      │ 1000    │ 128000
   │  ├─ pow2_ceil_to_multiple             188.8 ns      │ 2.67 µs       │ 201.3 ns      │ 214.4 ns      │ 1000    │ 64000
   │  ├─ pow2_ceil_to_multiple_const       156 ns        │ 459.1 ns      │ 174.7 ns      │ 185.2 ns      │ 1000    │ 64000
   │  ├─ pow2_ceil_to_multiple_reuse       160.7 ns      │ 801.3 ns      │ 176.3 ns      │ 194 ns        │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple            176.3 ns      │ 2.791 µs      │ 190.4 ns      │ 197.3 ns      │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple_const      149.7 ns      │ 501.3 ns      │ 179.4 ns      │ 184.7 ns      │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple_reuse      160.7 ns      │ 531 ns        │ 171.6 ns      │ 178.1 ns      │ 1000    │ 64000
   │  ├─ pow2_round_to_multiple            199.7 ns      │ 498.2 ns      │ 223.2 ns      │ 226.6 ns      │ 1000    │ 64000
   │  ├─ pow2_round_to_multiple_const      159.1 ns      │ 604.4 ns      │ 176.3 ns      │ 191.9 ns      │ 1000    │ 64000
   │  ├─ pow2_round_to_multiple_reuse      162.2 ns      │ 552.9 ns      │ 176.3 ns      │ 194.5 ns      │ 1000    │ 64000
   │  ├─ std_div_mul                       1.674 µs      │ 3.549 µs      │ 1.699 µs      │ 1.717 µs      │ 1000    │ 8000
   │  ├─ std_div_mul_const                 149.7 ns      │ 462.2 ns      │ 179.4 ns      │ 184.9 ns      │ 1000    │ 64000
   │  ├─ std_div_mul_reuse                 1.524 µs      │ 19.99 µs      │ 1.562 µs      │ 1.686 µs      │ 1000    │ 8000
   │  ├─ unb_pow2_ceil_to_multiple         193.5 ns      │ 745.1 ns      │ 215.4 ns      │ 221.1 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_ceil_to_multiple_const   160.7 ns      │ 365.4 ns      │ 176.3 ns      │ 178.9 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   159.1 ns      │ 423.2 ns      │ 173.2 ns      │ 178.5 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple        185.7 ns      │ 501.3 ns      │ 206 ns        │ 211.2 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple_const  152.9 ns      │ 407.6 ns      │ 170.1 ns      │ 175.4 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple_reuse  156 ns        │ 448.2 ns      │ 166.9 ns      │ 173.4 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_round_to_multiple        213.8 ns      │ 510.7 ns      │ 237.2 ns      │ 243.4 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_round_to_multiple_const  159.1 ns      │ 1.813 µs      │ 181 ns        │ 191 ns        │ 1000    │ 64000
   │  ╰─ unb_pow2_round_to_multiple_reuse  163.8 ns      │ 868.5 ns      │ 179.4 ns      │ 184.4 ns      │ 1000    │ 64000
   ╰─ u128                                               │               │               │               │         │
      ├─ baseline_identity                 199.7 ns      │ 473.2 ns      │ 229.4 ns      │ 236.8 ns      │ 1000    │ 64000
      ├─ pow2_ceil_to_multiple             1.043 µs      │ 1.874 µs      │ 1.068 µs      │ 1.085 µs      │ 1000    │ 16000
      ├─ pow2_ceil_to_multiple_const       612.2 ns      │ 1.559 µs      │ 634.1 ns      │ 648.8 ns      │ 1000    │ 32000
      ├─ pow2_ceil_to_multiple_reuse       656 ns        │ 1.487 µs      │ 674.7 ns      │ 682.6 ns      │ 1000    │ 16000
      ├─ pow2_floor_to_multiple            812.2 ns      │ 2.099 µs      │ 824.7 ns      │ 837.2 ns      │ 1000    │ 16000
      ├─ pow2_floor_to_multiple_const      406 ns        │ 1.006 µs      │ 427.9 ns      │ 444.9 ns      │ 1000    │ 32000
      ├─ pow2_floor_to_multiple_reuse      540.4 ns      │ 4.237 µs      │ 565.4 ns      │ 588.5 ns      │ 1000    │ 32000
      ├─ pow2_round_to_multiple            2.124 µs      │ 3.699 µs      │ 2.162 µs      │ 2.19 µs       │ 1000    │ 8000
      ├─ pow2_round_to_multiple_const      615.4 ns      │ 1.393 µs      │ 643.5 ns      │ 655.4 ns      │ 1000    │ 32000
      ├─ pow2_round_to_multiple_reuse      656 ns        │ 1.406 µs      │ 674.7 ns      │ 689.9 ns      │ 1000    │ 16000
      ├─ std_div_mul                       4.274 µs      │ 7.999 µs      │ 4.324 µs      │ 4.38 µs       │ 1000    │ 4000
      ├─ std_div_mul_const                 399.7 ns      │ 884.1 ns      │ 434.1 ns      │ 444.9 ns      │ 1000    │ 32000
      ├─ std_div_mul_reuse                 4.349 µs      │ 7.474 µs      │ 4.424 µs      │ 4.454 µs      │ 1000    │ 4000
      ├─ unb_pow2_ceil_to_multiple         1.124 µs      │ 1.706 µs      │ 1.149 µs      │ 1.161 µs      │ 1000    │ 16000
      ├─ unb_pow2_ceil_to_multiple_const   606 ns        │ 4.127 µs      │ 634.1 ns      │ 650.6 ns      │ 1000    │ 32000
      ├─ unb_pow2_ceil_to_multiple_reuse   649.7 ns      │ 8.781 µs      │ 668.5 ns      │ 717.3 ns      │ 1000    │ 16000
      ├─ unb_pow2_floor_to_multiple        818.5 ns      │ 8.981 µs      │ 843.5 ns      │ 1.115 µs      │ 1000    │ 16000
      ├─ unb_pow2_floor_to_multiple_const  406 ns        │ 5.234 µs      │ 421.6 ns      │ 434.8 ns      │ 1000    │ 32000
      ├─ unb_pow2_floor_to_multiple_reuse  543.5 ns      │ 1.356 µs      │ 565.4 ns      │ 602.2 ns      │ 1000    │ 32000
      ├─ unb_pow2_round_to_multiple        2.087 µs      │ 19.93 µs      │ 2.137 µs      │ 2.317 µs      │ 1000    │ 8000
      ├─ unb_pow2_round_to_multiple_const  593.5 ns      │ 1.124 µs      │ 612.2 ns      │ 619.9 ns      │ 1000    │ 16000
      ╰─ unb_pow2_round_to_multiple_reuse  662.2 ns      │ 1.724 µs      │ 674.7 ns      │ 701 ns        │ 1000    │ 16000


```
