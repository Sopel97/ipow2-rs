# `public` bench with default target
```
public                                     fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ div                                                   │               │               │               │         │
│  ├─ i8                                                 │               │               │               │         │
│  │  ├─ baseline_identity                 12.29 ns      │ 43.73 ns      │ 13.46 ns      │ 14.08 ns      │ 1000    │ 1024000
│  │  ├─ pow2_div                          293.5 ns      │ 887.2 ns      │ 296.6 ns      │ 298.1 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_ceil                     295.1 ns      │ 462.2 ns      │ 298.2 ns      │ 299.3 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_ceil_const               34.55 ns      │ 56.82 ns      │ 38.07 ns      │ 38.88 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_ceil_reuse               43.14 ns      │ 86.89 ns      │ 47.05 ns      │ 47.19 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_const                    36.5 ns       │ 572 ns        │ 39.63 ns      │ 41.46 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_floor                    199.7 ns      │ 2.284 µs      │ 202.9 ns      │ 212.5 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_floor_const              31.62 ns      │ 111.1 ns      │ 35.72 ns      │ 38.75 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_floor_reuse              27.52 ns      │ 96.66 ns      │ 28.69 ns      │ 30.21 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_reuse                    40.02 ns      │ 93.14 ns      │ 42.75 ns      │ 43.14 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_round                    374.7 ns      │ 552.9 ns      │ 377.9 ns      │ 380.8 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_round_const              49.39 ns      │ 71.27 ns      │ 52.13 ns      │ 53.03 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_round_reuse              60.33 ns      │ 150.9 ns      │ 65.8 ns       │ 65.97 ns      │ 1000    │ 256000
│  │  ├─ std_div                           856 ns        │ 1.424 µs      │ 868.5 ns      │ 872.4 ns      │ 1000    │ 16000
│  │  ├─ std_div_const                     38.85 ns      │ 103.6 ns      │ 42.75 ns      │ 43.46 ns      │ 1000    │ 256000
│  │  ├─ std_div_reuse                     837.2 ns      │ 1.143 µs      │ 849.7 ns      │ 857.5 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_div                      299.7 ns      │ 1.951 µs      │ 302.9 ns      │ 308.9 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil                 304.4 ns      │ 2.407 µs      │ 307.6 ns      │ 319.4 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil_const           41.58 ns      │ 578.6 ns      │ 43.14 ns      │ 45.24 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_ceil_reuse           45.49 ns      │ 572.8 ns      │ 48.61 ns      │ 50.29 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_const                37.29 ns      │ 68.93 ns      │ 40.41 ns      │ 41.21 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_floor                202.9 ns      │ 421.6 ns      │ 206 ns        │ 208.6 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor_const          31.04 ns      │ 162.2 ns      │ 35.53 ns      │ 38.8 ns       │ 1000    │ 512000
│  │  ├─ unb_pow2_div_floor_reuse          34.16 ns      │ 91.97 ns      │ 39.43 ns      │ 42.96 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_reuse                38.07 ns      │ 68.14 ns      │ 41.97 ns      │ 41.9 ns       │ 1000    │ 256000
│  │  ├─ unb_pow2_div_round                381 ns        │ 727.9 ns      │ 384.1 ns      │ 387.9 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_round_const          50.96 ns      │ 88.85 ns      │ 54.86 ns      │ 54.54 ns      │ 1000    │ 256000
│  │  ╰─ unb_pow2_div_round_reuse          51.74 ns      │ 576.7 ns      │ 56.43 ns      │ 57.84 ns      │ 1000    │ 256000
│  ├─ i16                                                │               │               │               │         │
│  │  ├─ baseline_identity                 21.86 ns      │ 79.28 ns      │ 32.6 ns       │ 32.11 ns      │ 1000    │ 512000
│  │  ├─ pow2_div                          431 ns        │ 1.881 µs      │ 440.4 ns      │ 449.9 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_ceil                     399.7 ns      │ 5.381 µs      │ 406 ns        │ 417.6 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_ceil_const               64.63 ns      │ 177.9 ns      │ 73.22 ns      │ 78.28 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_ceil_reuse               71.27 ns      │ 218.5 ns      │ 85.72 ns      │ 90.79 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_const                    66.58 ns      │ 188.8 ns      │ 75.57 ns      │ 80.91 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_floor                    232.6 ns      │ 481 ns        │ 237.2 ns      │ 239.1 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_floor_const              47.44 ns      │ 132.9 ns      │ 57.6 ns       │ 61.18 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_floor_reuse              54.47 ns      │ 184.9 ns      │ 66.58 ns      │ 70.38 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_reuse                    56.82 ns      │ 180.2 ns      │ 60.72 ns      │ 62.97 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_round                    565.4 ns      │ 984.1 ns      │ 571.6 ns      │ 574.1 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_round_const              74.39 ns      │ 215 ns        │ 82.99 ns      │ 88.77 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_round_reuse              76.35 ns      │ 152.1 ns      │ 80.25 ns      │ 81.35 ns      │ 1000    │ 128000
│  │  ├─ std_div                           881 ns        │ 1.368 µs      │ 881 ns        │ 887.8 ns      │ 1000    │ 16000
│  │  ├─ std_div_const                     61.89 ns      │ 187.2 ns      │ 73.22 ns      │ 77.4 ns       │ 1000    │ 256000
│  │  ├─ std_div_reuse                     2.524 µs      │ 5.599 µs      │ 2.574 µs      │ 2.595 µs      │ 1000    │ 4000
│  │  ├─ unb_pow2_div                      434.1 ns      │ 581 ns        │ 440.4 ns      │ 442.8 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_ceil                 412.2 ns      │ 4.462 µs      │ 415.4 ns      │ 427.5 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_ceil_const           65.02 ns      │ 159.9 ns      │ 76.74 ns      │ 80.8 ns       │ 1000    │ 256000
│  │  ├─ unb_pow2_div_ceil_reuse           60.72 ns      │ 203.6 ns      │ 64.63 ns      │ 69.71 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_const                64.24 ns      │ 307.9 ns      │ 78.3 ns       │ 82.14 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_floor                229.4 ns      │ 454.4 ns      │ 234.1 ns      │ 235.4 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor_const          47.05 ns      │ 216.5 ns      │ 56.82 ns      │ 60.98 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_floor_reuse          51.74 ns      │ 141.9 ns      │ 63.46 ns      │ 67.97 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_reuse                66.58 ns      │ 316.5 ns      │ 77.52 ns      │ 83.71 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_round                577.9 ns      │ 1.049 µs      │ 584.1 ns      │ 591.6 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_round_const          77.13 ns      │ 224 ns        │ 89.24 ns      │ 94.1 ns       │ 1000    │ 256000
│  │  ╰─ unb_pow2_div_round_reuse          76.35 ns      │ 147.4 ns      │ 81.82 ns      │ 83.55 ns      │ 1000    │ 128000
│  ├─ i32                                                │               │               │               │         │
│  │  ├─ baseline_identity                 42.36 ns      │ 125.9 ns      │ 53.3 ns       │ 54.58 ns      │ 1000    │ 256000
│  │  ├─ pow2_div                          565.4 ns      │ 5.381 µs      │ 577.9 ns      │ 599.2 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_ceil                     502.9 ns      │ 4.737 µs      │ 509.1 ns      │ 526.6 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_ceil_const               131 ns        │ 950.5 ns      │ 142.7 ns      │ 150.2 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_ceil_reuse               129.4 ns      │ 345.8 ns      │ 145.1 ns      │ 147.4 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_const                    129.4 ns      │ 331 ns        │ 137.2 ns      │ 140.1 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_floor                    287.2 ns      │ 659.1 ns      │ 295.1 ns      │ 300.9 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_floor_const              90.41 ns      │ 338 ns        │ 102.9 ns      │ 109.3 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_floor_reuse              98.22 ns      │ 261.5 ns      │ 118.5 ns      │ 126.8 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_reuse                    127.1 ns      │ 324 ns        │ 143.5 ns      │ 156.8 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_round                    731 ns        │ 1.093 µs      │ 743.5 ns      │ 753.2 ns      │ 1000    │ 16000
│  │  ├─ pow2_div_round_const              159.1 ns      │ 379.4 ns      │ 168.5 ns      │ 171.7 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_round_reuse              185.7 ns      │ 365.4 ns      │ 193.5 ns      │ 199.2 ns      │ 1000    │ 64000
│  │  ├─ std_div                           1.312 µs      │ 1.962 µs      │ 1.312 µs      │ 1.325 µs      │ 1000    │ 8000
│  │  ├─ std_div_const                     124 ns        │ 1.39 µs       │ 133.3 ns      │ 142.7 ns      │ 1000    │ 128000
│  │  ├─ std_div_reuse                     1.274 µs      │ 3.412 µs      │ 1.287 µs      │ 1.329 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div                      534.1 ns      │ 862.2 ns      │ 540.4 ns      │ 544.5 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_ceil                 524.7 ns      │ 696.6 ns      │ 531 ns        │ 533.9 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_ceil_const           128.6 ns      │ 302.1 ns      │ 142.7 ns      │ 151.7 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_ceil_reuse           126.3 ns      │ 319.3 ns      │ 141.9 ns      │ 150.8 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_const                126.3 ns      │ 306.8 ns      │ 140.4 ns      │ 148.2 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_floor                295.1 ns      │ 656 ns        │ 301.3 ns      │ 306.6 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor_const          91.97 ns      │ 269.3 ns      │ 112.2 ns      │ 118.7 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_floor_reuse          98.22 ns      │ 303.6 ns      │ 114.6 ns      │ 126.2 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_reuse                124 ns        │ 435.7 ns      │ 146.6 ns      │ 153.5 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_round                756 ns        │ 9.906 µs      │ 762.2 ns      │ 794.3 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_div_round_const          162.2 ns      │ 1.898 µs      │ 168.5 ns      │ 173.9 ns      │ 1000    │ 64000
│  │  ╰─ unb_pow2_div_round_reuse          185.7 ns      │ 2.193 µs      │ 195.1 ns      │ 204.9 ns      │ 1000    │ 64000
│  ├─ i64                                                │               │               │               │         │
│  │  ├─ baseline_identity                 81.04 ns      │ 281.8 ns      │ 108.7 ns      │ 111 ns        │ 1000    │ 128000
│  │  ├─ pow2_div                          618.5 ns      │ 2.418 µs      │ 637.2 ns      │ 660 ns        │ 1000    │ 16000
│  │  ├─ pow2_div_ceil                     637.2 ns      │ 1.531 µs      │ 649.7 ns      │ 655.1 ns      │ 1000    │ 16000
│  │  ├─ pow2_div_ceil_const               352.9 ns      │ 656 ns        │ 359.1 ns      │ 360.2 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_ceil_reuse               431 ns        │ 1.127 µs      │ 706 ns        │ 629.8 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_const                    399.7 ns      │ 1.64 µs       │ 415.4 ns      │ 429.8 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_floor                    331 ns        │ 40.65 µs      │ 340.4 ns      │ 382.9 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_floor_const              268.5 ns      │ 590.4 ns      │ 284.1 ns      │ 295.9 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_floor_reuse              221.6 ns      │ 579.4 ns      │ 257.6 ns      │ 268.4 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_reuse                    443.5 ns      │ 1.365 µs      │ 459.1 ns      │ 463.2 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_round                    937.2 ns      │ 1.224 µs      │ 949.7 ns      │ 949.9 ns      │ 1000    │ 16000
│  │  ├─ pow2_div_round_const              474.7 ns      │ 631 ns        │ 487.2 ns      │ 489.4 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_round_reuse              649.7 ns      │ 1.231 µs      │ 662.2 ns      │ 664.7 ns      │ 1000    │ 16000
│  │  ├─ std_div                           1.637 µs      │ 2.199 µs      │ 1.649 µs      │ 1.654 µs      │ 1000    │ 8000
│  │  ├─ std_div_const                     390.4 ns      │ 3.668 µs      │ 402.9 ns      │ 420.9 ns      │ 1000    │ 32000
│  │  ├─ std_div_reuse                     1.512 µs      │ 14.76 µs      │ 1.537 µs      │ 1.577 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div                      624.7 ns      │ 7.374 µs      │ 643.5 ns      │ 665.8 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_div_ceil                 674.7 ns      │ 943.5 ns      │ 687.2 ns      │ 690.7 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_div_ceil_const           352.9 ns      │ 665.4 ns      │ 362.2 ns      │ 366 ns        │ 1000    │ 32000
│  │  ├─ unb_pow2_div_ceil_reuse           431 ns        │ 4.077 µs      │ 443.5 ns      │ 457.1 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_const                406 ns        │ 4.774 µs      │ 415.4 ns      │ 434.7 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_floor                321.6 ns      │ 4.443 µs      │ 334.1 ns      │ 347.6 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_floor_const          270.1 ns      │ 785.7 ns      │ 288.8 ns      │ 298.7 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor_reuse          223.2 ns      │ 666.9 ns      │ 252.9 ns      │ 258.4 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_reuse                446.6 ns      │ 752.9 ns      │ 459.1 ns      │ 462.6 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_round                962.2 ns      │ 1.231 µs      │ 974.7 ns      │ 974.6 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_div_round_const          474.7 ns      │ 649.7 ns      │ 484.1 ns      │ 487.1 ns      │ 1000    │ 32000
│  │  ╰─ unb_pow2_div_round_reuse          649.7 ns      │ 956 ns        │ 662.2 ns      │ 664 ns        │ 1000    │ 16000
│  ├─ i128                                               │               │               │               │         │
│  │  ├─ baseline_identity                 223.2 ns      │ 2.413 µs      │ 238.8 ns      │ 251.4 ns      │ 1000    │ 64000
│  │  ├─ pow2_div                          1.837 µs      │ 2.899 µs      │ 1.874 µs      │ 1.881 µs      │ 1000    │ 8000
│  │  ├─ pow2_div_ceil                     1.749 µs      │ 18.71 µs      │ 1.774 µs      │ 1.873 µs      │ 1000    │ 8000
│  │  ├─ pow2_div_ceil_const               1.018 µs      │ 1.912 µs      │ 1.068 µs      │ 1.078 µs      │ 1000    │ 16000
│  │  ├─ pow2_div_ceil_reuse               1.337 µs      │ 2.849 µs      │ 1.349 µs      │ 1.375 µs      │ 1000    │ 8000
│  │  ├─ pow2_div_const                    1.012 µs      │ 2.449 µs      │ 1.037 µs      │ 1.04 µs       │ 1000    │ 8000
│  │  ├─ pow2_div_floor                    1.074 µs      │ 1.399 µs      │ 1.124 µs      │ 1.127 µs      │ 1000    │ 16000
│  │  ├─ pow2_div_floor_const              799.7 ns      │ 1.406 µs      │ 818.5 ns      │ 827.7 ns      │ 1000    │ 16000
│  │  ├─ pow2_div_floor_reuse              1.012 µs      │ 2.274 µs      │ 1.024 µs      │ 1.035 µs      │ 1000    │ 16000
│  │  ├─ pow2_div_reuse                    1.324 µs      │ 3.512 µs      │ 1.349 µs      │ 1.405 µs      │ 1000    │ 8000
│  │  ├─ pow2_div_round                    2.774 µs      │ 9.399 µs      │ 2.824 µs      │ 2.944 µs      │ 1000    │ 4000
│  │  ├─ pow2_div_round_const              1.674 µs      │ 4.724 µs      │ 1.724 µs      │ 1.728 µs      │ 1000    │ 4000
│  │  ├─ pow2_div_round_reuse              2.499 µs      │ 5.424 µs      │ 2.549 µs      │ 2.65 µs       │ 1000    │ 4000
│  │  ├─ std_div                           9.249 µs      │ 11.39 µs      │ 9.299 µs      │ 9.358 µs      │ 1000    │ 2000
│  │  ├─ std_div_const                     1.006 µs      │ 2.481 µs      │ 1.024 µs      │ 1.037 µs      │ 1000    │ 16000
│  │  ├─ std_div_reuse                     8.149 µs      │ 10.94 µs      │ 8.299 µs      │ 8.308 µs      │ 1000    │ 2000
│  │  ├─ unb_pow2_div                      1.824 µs      │ 2.362 µs      │ 1.837 µs      │ 1.849 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div_ceil                 1.787 µs      │ 2.624 µs      │ 1.812 µs      │ 1.813 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div_ceil_const           999.7 ns      │ 1.574 µs      │ 1.018 µs      │ 1.024 µs      │ 1000    │ 16000
│  │  ├─ unb_pow2_div_ceil_reuse           1.324 µs      │ 3.137 µs      │ 1.337 µs      │ 1.348 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div_const                1.012 µs      │ 1.618 µs      │ 1.024 µs      │ 1.03 µs       │ 1000    │ 16000
│  │  ├─ unb_pow2_div_floor                1.056 µs      │ 1.624 µs      │ 1.068 µs      │ 1.071 µs      │ 1000    │ 16000
│  │  ├─ unb_pow2_div_floor_const          799.7 ns      │ 1.381 µs      │ 818.5 ns      │ 822.7 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_div_floor_reuse          1.012 µs      │ 1.812 µs      │ 1.024 µs      │ 1.032 µs      │ 1000    │ 16000
│  │  ├─ unb_pow2_div_reuse                1.262 µs      │ 1.912 µs      │ 1.274 µs      │ 1.285 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div_round                2.849 µs      │ 3.999 µs      │ 2.924 µs      │ 2.929 µs      │ 1000    │ 4000
│  │  ├─ unb_pow2_div_round_const          1.637 µs      │ 11.59 µs      │ 1.662 µs      │ 1.73 µs       │ 1000    │ 8000
│  │  ╰─ unb_pow2_div_round_reuse          2.524 µs      │ 5.774 µs      │ 2.549 µs      │ 2.573 µs      │ 1000    │ 4000
│  ├─ u8                                                 │               │               │               │         │
│  │  ├─ baseline_identity                 11.99 ns      │ 56.62 ns      │ 13.46 ns      │ 13.93 ns      │ 1000    │ 1024000
│  │  ├─ pow2_div                          106.8 ns      │ 152.1 ns      │ 109.1 ns      │ 109.9 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_ceil                     166.9 ns      │ 326.3 ns      │ 168.5 ns      │ 169.1 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_ceil_const               34.16 ns      │ 84.94 ns      │ 37.29 ns      │ 37.2 ns       │ 1000    │ 256000
│  │  ├─ pow2_div_ceil_reuse               39.24 ns      │ 62.29 ns      │ 42.36 ns      │ 42.89 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_const                    27.13 ns      │ 99 ns         │ 30.25 ns      │ 32.96 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_floor                    97.44 ns      │ 323.2 ns      │ 100.5 ns      │ 103 ns        │ 1000    │ 128000
│  │  ├─ pow2_div_floor_const              26.54 ns      │ 91.97 ns      │ 30.25 ns      │ 32.62 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_floor_reuse              29.67 ns      │ 246 ns        │ 33.57 ns      │ 36.22 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_reuse                    29.47 ns      │ 73.61 ns      │ 33.18 ns      │ 33.95 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_round                    166.9 ns      │ 512.2 ns      │ 170.1 ns      │ 170.6 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_round_const              35.92 ns      │ 160.5 ns      │ 41.19 ns      │ 45.03 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_round_reuse              30.25 ns      │ 106.4 ns      │ 32.99 ns      │ 35.05 ns      │ 1000    │ 256000
│  │  ├─ std_div                           843.5 ns      │ 10.01 µs      │ 856 ns        │ 880.8 ns      │ 1000    │ 16000
│  │  ├─ std_div_const                     26.54 ns      │ 292.5 ns      │ 31.23 ns      │ 34.11 ns      │ 1000    │ 512000
│  │  ├─ std_div_reuse                     824.7 ns      │ 2.112 µs      │ 831 ns        │ 837.8 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_div                      105.2 ns      │ 1.254 µs      │ 109.9 ns      │ 117.7 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_ceil                 171.6 ns      │ 2.335 µs      │ 174.7 ns      │ 182.9 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil_const           33.77 ns      │ 152.9 ns      │ 36.5 ns       │ 37.82 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_ceil_reuse           37.29 ns      │ 72.83 ns      │ 40.8 ns       │ 41.71 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_const                27.13 ns      │ 126.3 ns      │ 32.79 ns      │ 35.47 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_floor                105.2 ns      │ 177.1 ns      │ 107.6 ns      │ 109.5 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_floor_const          26.74 ns      │ 151.5 ns      │ 34.55 ns      │ 39.37 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_floor_reuse          21.27 ns      │ 43.54 ns      │ 24.39 ns      │ 24.9 ns       │ 1000    │ 256000
│  │  ├─ unb_pow2_div_reuse                29.67 ns      │ 89.43 ns      │ 33.18 ns      │ 36.39 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_round                174.7 ns      │ 426.3 ns      │ 176.3 ns      │ 177.6 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_round_const          35.33 ns      │ 280.2 ns      │ 40.61 ns      │ 44 ns         │ 1000    │ 512000
│  │  ╰─ unb_pow2_div_round_reuse          30.64 ns      │ 110.3 ns      │ 31.82 ns      │ 33.26 ns      │ 1000    │ 256000
│  ├─ u16                                                │               │               │               │         │
│  │  ├─ baseline_identity                 22.05 ns      │ 72.25 ns      │ 32.01 ns      │ 32.24 ns      │ 1000    │ 512000
│  │  ├─ pow2_div                          231 ns        │ 477.9 ns      │ 237.2 ns      │ 240.7 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_ceil                     399.7 ns      │ 981 ns        │ 406 ns        │ 416.6 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_ceil_const               64.24 ns      │ 230.6 ns      │ 72.05 ns      │ 77.5 ns       │ 1000    │ 256000
│  │  ├─ pow2_div_ceil_reuse               71.27 ns      │ 336.5 ns      │ 84.94 ns      │ 91.57 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_const                    50.18 ns      │ 177.5 ns      │ 61.11 ns      │ 65.16 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_floor                    232.6 ns      │ 421.6 ns      │ 235.7 ns      │ 237.1 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_floor_const              48.61 ns      │ 154 ns        │ 62.48 ns      │ 67.03 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_floor_reuse              50.18 ns      │ 313 ns        │ 54.86 ns      │ 63.47 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_reuse                    52.13 ns      │ 247 ns        │ 61.89 ns      │ 66.75 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_round                    387.2 ns      │ 862.2 ns      │ 393.5 ns      │ 395.7 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_round_const              60.72 ns      │ 193.9 ns      │ 68.93 ns      │ 74.73 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_round_reuse              67.75 ns      │ 232.9 ns      │ 82.6 ns       │ 87.32 ns      │ 1000    │ 256000
│  │  ├─ std_div                           862.2 ns      │ 1.749 µs      │ 881 ns        │ 886.6 ns      │ 1000    │ 16000
│  │  ├─ std_div_const                     50.96 ns      │ 171.6 ns      │ 58.38 ns      │ 62.92 ns      │ 1000    │ 256000
│  │  ├─ std_div_reuse                     843.5 ns      │ 1.556 µs      │ 849.7 ns      │ 856.5 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_div                      231 ns        │ 385.7 ns      │ 234.1 ns      │ 235.2 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil                 412.2 ns      │ 699.7 ns      │ 415.4 ns      │ 419 ns        │ 1000    │ 32000
│  │  ├─ unb_pow2_div_ceil_const           64.63 ns      │ 224.7 ns      │ 74 ns         │ 80.28 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_ceil_reuse           75.57 ns      │ 205.6 ns      │ 93.14 ns      │ 97.26 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_const                50.57 ns      │ 249.7 ns      │ 59.94 ns      │ 64.79 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_floor                229.4 ns      │ 2.527 µs      │ 235.7 ns      │ 271 ns        │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor_const          50.96 ns      │ 254.4 ns      │ 59.55 ns      │ 63.03 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_floor_reuse          51.74 ns      │ 180.2 ns      │ 62.68 ns      │ 66.1 ns       │ 1000    │ 256000
│  │  ├─ unb_pow2_div_reuse                51.74 ns      │ 165.4 ns      │ 62.68 ns      │ 68.82 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_round                393.5 ns      │ 1.993 µs      │ 406 ns        │ 411.5 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_round_const          63.07 ns      │ 163.4 ns      │ 72.83 ns      │ 76.19 ns      │ 1000    │ 256000
│  │  ╰─ unb_pow2_div_round_reuse          65.41 ns      │ 352.9 ns      │ 80.64 ns      │ 86.76 ns      │ 1000    │ 256000
│  ├─ u32                                                │               │               │               │         │
│  │  ├─ baseline_identity                 41.58 ns      │ 144.7 ns      │ 52.71 ns      │ 54.86 ns      │ 1000    │ 256000
│  │  ├─ pow2_div                          296.6 ns      │ 576.3 ns      │ 302.9 ns      │ 306.7 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_ceil                     506 ns        │ 643.5 ns      │ 509.1 ns      │ 513.2 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_ceil_const               126.3 ns      │ 378.6 ns      │ 138.8 ns      │ 144.5 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_ceil_reuse               128.6 ns      │ 389.6 ns      │ 145.8 ns      │ 156 ns        │ 1000    │ 128000
│  │  ├─ pow2_div_const                    95.88 ns      │ 339.6 ns      │ 105.2 ns      │ 107.3 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_floor                    291.9 ns      │ 454.4 ns      │ 298.2 ns      │ 300.8 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_floor_const              95.88 ns      │ 359.1 ns      │ 106 ns        │ 108.4 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_floor_reuse              96.66 ns      │ 409.1 ns      │ 109.1 ns      │ 119.9 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_reuse                    98.22 ns      │ 276.3 ns      │ 123.2 ns      │ 129.8 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_round                    402.9 ns      │ 556 ns        │ 412.2 ns      │ 413.4 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_round_const              120.8 ns      │ 528.6 ns      │ 139.6 ns      │ 148.1 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_round_reuse              128.6 ns      │ 320.8 ns      │ 153.6 ns      │ 161.7 ns      │ 1000    │ 128000
│  │  ├─ std_div                           1.312 µs      │ 8.699 µs      │ 1.337 µs      │ 1.393 µs      │ 1000    │ 8000
│  │  ├─ std_div_const                     94.32 ns      │ 489.6 ns      │ 111.5 ns      │ 120.9 ns      │ 1000    │ 128000
│  │  ├─ std_div_reuse                     1.299 µs      │ 1.899 µs      │ 1.299 µs      │ 1.307 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div                      293.5 ns      │ 491.9 ns      │ 299.7 ns      │ 302 ns        │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil                 524.7 ns      │ 856 ns        │ 531 ns        │ 532.6 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_ceil_const           124.7 ns      │ 522.4 ns      │ 142.7 ns      │ 149.1 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_ceil_reuse           134.1 ns      │ 420.1 ns      │ 149 ns        │ 160.1 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_const                95.88 ns      │ 334.9 ns      │ 105.2 ns      │ 108.6 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_floor                295.1 ns      │ 2.662 µs      │ 301.3 ns      │ 310.7 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor_const          95.88 ns      │ 304.4 ns      │ 106 ns        │ 112.7 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_floor_reuse          99 ns         │ 265.4 ns      │ 122.4 ns      │ 131.1 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_reuse                99 ns         │ 291.9 ns      │ 120.1 ns      │ 128.9 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_round                418.5 ns      │ 565.4 ns      │ 424.7 ns      │ 426.9 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_round_const          120.8 ns      │ 417.7 ns      │ 132.6 ns      │ 135.2 ns      │ 1000    │ 128000
│  │  ╰─ unb_pow2_div_round_reuse          127.1 ns      │ 313 ns        │ 141.9 ns      │ 156.4 ns      │ 1000    │ 128000
│  ├─ u64                                                │               │               │               │         │
│  │  ├─ baseline_identity                 80.25 ns      │ 302.9 ns      │ 109.1 ns      │ 112.2 ns      │ 1000    │ 128000
│  │  ├─ pow2_div                          421.6 ns      │ 809.1 ns      │ 434.1 ns      │ 436.6 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_ceil                     631 ns        │ 8.487 µs      │ 649.7 ns      │ 656.1 ns      │ 1000    │ 16000
│  │  ├─ pow2_div_ceil_const               238.8 ns      │ 2.601 µs      │ 262.2 ns      │ 272 ns        │ 1000    │ 64000
│  │  ├─ pow2_div_ceil_reuse               381 ns        │ 521.6 ns      │ 387.2 ns      │ 389.4 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_const                    184.1 ns      │ 559.1 ns      │ 218.5 ns      │ 231.7 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_floor                    324.7 ns      │ 515.4 ns      │ 340.4 ns      │ 343.3 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_floor_const              184.1 ns      │ 757.6 ns      │ 229.4 ns      │ 246.1 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_floor_reuse              191.9 ns      │ 681 ns        │ 229.4 ns      │ 242.4 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_reuse                    198.2 ns      │ 577.9 ns      │ 226.3 ns      │ 232.9 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_round                    587.2 ns      │ 10.06 µs      │ 606 ns        │ 638.1 ns      │ 1000    │ 16000
│  │  ├─ pow2_div_round_const              237.2 ns      │ 2.421 µs      │ 263.8 ns      │ 283.5 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_round_reuse              381 ns        │ 577.9 ns      │ 390.4 ns      │ 395.3 ns      │ 1000    │ 32000
│  │  ├─ std_div                           1.649 µs      │ 2.212 µs      │ 1.674 µs      │ 1.681 µs      │ 1000    │ 8000
│  │  ├─ std_div_const                     185.7 ns      │ 612.2 ns      │ 227.9 ns      │ 236.4 ns      │ 1000    │ 64000
│  │  ├─ std_div_reuse                     1.512 µs      │ 13.59 µs      │ 1.524 µs      │ 1.555 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div                      424.7 ns      │ 4.681 µs      │ 434.1 ns      │ 456.2 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_ceil                 674.7 ns      │ 7.393 µs      │ 687.2 ns      │ 706.3 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_div_ceil_const           246.6 ns      │ 1.038 µs      │ 268.5 ns      │ 276.2 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil_reuse           381 ns        │ 890.4 ns      │ 387.2 ns      │ 399 ns        │ 1000    │ 32000
│  │  ├─ unb_pow2_div_const                182.6 ns      │ 571.6 ns      │ 199.7 ns      │ 206.3 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor                424.7 ns      │ 1.152 µs      │ 437.2 ns      │ 445.5 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_floor_const          182.6 ns      │ 818.5 ns      │ 207.6 ns      │ 219.9 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor_reuse          195.1 ns      │ 595.1 ns      │ 223.2 ns      │ 230 ns        │ 1000    │ 64000
│  │  ├─ unb_pow2_div_reuse                196.6 ns      │ 687.2 ns      │ 224.7 ns      │ 231.7 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_round                662.2 ns      │ 1.762 µs      │ 687.2 ns      │ 689.7 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_div_round_const          234.1 ns      │ 799.7 ns      │ 262.2 ns      │ 273.9 ns      │ 1000    │ 64000
│  │  ╰─ unb_pow2_div_round_reuse          377.9 ns      │ 806 ns        │ 396.6 ns      │ 400.4 ns      │ 1000    │ 32000
│  ╰─ u128                                               │               │               │               │         │
│     ├─ baseline_identity                 223.2 ns      │ 634.1 ns      │ 241.9 ns      │ 262.6 ns      │ 1000    │ 64000
│     ├─ pow2_div                          956 ns        │ 2.043 µs      │ 1.024 µs      │ 1.045 µs      │ 1000    │ 16000
│     ├─ pow2_div_ceil                     1.762 µs      │ 5.324 µs      │ 1.787 µs      │ 1.866 µs      │ 1000    │ 8000
│     ├─ pow2_div_ceil_const               999.7 ns      │ 9.774 µs      │ 1.024 µs      │ 1.064 µs      │ 1000    │ 16000
│     ├─ pow2_div_ceil_reuse               1.262 µs      │ 14.07 µs      │ 1.287 µs      │ 1.318 µs      │ 1000    │ 8000
│     ├─ pow2_div_const                    793.5 ns      │ 7.324 µs      │ 818.5 ns      │ 846.9 ns      │ 1000    │ 16000
│     ├─ pow2_div_floor                    956 ns        │ 7.374 µs      │ 968.5 ns      │ 1.008 µs      │ 1000    │ 16000
│     ├─ pow2_div_floor_const              799.7 ns      │ 7.399 µs      │ 831 ns        │ 844 ns        │ 1000    │ 16000
│     ├─ pow2_div_floor_reuse              999.7 ns      │ 7.849 µs      │ 1.012 µs      │ 1.047 µs      │ 1000    │ 16000
│     ├─ pow2_div_reuse                    999.7 ns      │ 9.187 µs      │ 1.018 µs      │ 1.041 µs      │ 1000    │ 16000
│     ├─ pow2_div_round                    2.512 µs      │ 13.66 µs      │ 2.574 µs      │ 2.617 µs      │ 1000    │ 8000
│     ├─ pow2_div_round_const              1.031 µs      │ 1.606 µs      │ 1.106 µs      │ 1.104 µs      │ 1000    │ 16000
│     ├─ pow2_div_round_reuse              1.249 µs      │ 3.187 µs      │ 1.274 µs      │ 1.275 µs      │ 1000    │ 8000
│     ├─ std_div                           5.799 µs      │ 7.899 µs      │ 5.849 µs      │ 5.842 µs      │ 1000    │ 2000
│     ├─ std_div_const                     806 ns        │ 1.487 µs      │ 818.5 ns      │ 826.9 ns      │ 1000    │ 16000
│     ├─ std_div_reuse                     4.274 µs      │ 6.474 µs      │ 4.299 µs      │ 4.318 µs      │ 1000    │ 4000
│     ├─ unb_pow2_div                      943.5 ns      │ 1.631 µs      │ 956 ns        │ 961.8 ns      │ 1000    │ 16000
│     ├─ unb_pow2_div_ceil                 1.749 µs      │ 2.299 µs      │ 1.762 µs      │ 1.774 µs      │ 1000    │ 8000
│     ├─ unb_pow2_div_ceil_const           999.7 ns      │ 2.043 µs      │ 1.024 µs      │ 1.03 µs       │ 1000    │ 16000
│     ├─ unb_pow2_div_ceil_reuse           1.231 µs      │ 2.149 µs      │ 1.249 µs      │ 1.253 µs      │ 1000    │ 16000
│     ├─ unb_pow2_div_const                799.7 ns      │ 1.293 µs      │ 818.5 ns      │ 819.4 ns      │ 1000    │ 16000
│     ├─ unb_pow2_div_floor                943.5 ns      │ 1.756 µs      │ 956 ns        │ 959.6 ns      │ 1000    │ 16000
│     ├─ unb_pow2_div_floor_const          799.7 ns      │ 7.724 µs      │ 818.5 ns      │ 853.2 ns      │ 1000    │ 16000
│     ├─ unb_pow2_div_floor_reuse          1.012 µs      │ 7.299 µs      │ 1.031 µs      │ 1.062 µs      │ 1000    │ 16000
│     ├─ unb_pow2_div_reuse                1.012 µs      │ 9.481 µs      │ 1.031 µs      │ 1.068 µs      │ 1000    │ 16000
│     ├─ unb_pow2_div_round                2.549 µs      │ 35.72 µs      │ 2.599 µs      │ 2.682 µs      │ 1000    │ 4000
│     ├─ unb_pow2_div_round_const          1.024 µs      │ 7.681 µs      │ 1.037 µs      │ 1.053 µs      │ 1000    │ 16000
│     ╰─ unb_pow2_div_round_reuse          1.262 µs      │ 1.737 µs      │ 1.274 µs      │ 1.282 µs      │ 1000    │ 8000
├─ mul                                                   │               │               │               │         │
│  ├─ i8                                                 │               │               │               │         │
│  │  ├─ baseline_identity                 11.99 ns      │ 42.17 ns      │ 13.36 ns      │ 13.96 ns      │ 1000    │ 1024000
│  │  ├─ pow2_mul                          97.44 ns      │ 213 ns        │ 99.79 ns      │ 102 ns        │ 1000    │ 128000
│  │  ├─ pow2_mul_const                    26.93 ns      │ 113.6 ns      │ 32.01 ns      │ 34.64 ns      │ 1000    │ 512000
│  │  ├─ pow2_mul_reuse                    30.06 ns      │ 113.2 ns      │ 35.33 ns      │ 39.68 ns      │ 1000    │ 512000
│  │  ├─ std_mul                           54.86 ns      │ 115.4 ns      │ 59.55 ns      │ 60.59 ns      │ 1000    │ 256000
│  │  ├─ std_mul_const                     26.35 ns      │ 86.7 ns       │ 31.04 ns      │ 33.14 ns      │ 1000    │ 512000
│  │  ├─ std_mul_reuse                     35.72 ns      │ 616.5 ns      │ 38.07 ns      │ 39.69 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_mul                      100.5 ns      │ 1.096 µs      │ 105.2 ns      │ 108 ns        │ 1000    │ 128000
│  │  ├─ unb_pow2_mul_const                26.54 ns      │ 165.4 ns      │ 31.62 ns      │ 34.26 ns      │ 1000    │ 512000
│  │  ╰─ unb_pow2_mul_reuse                29.47 ns      │ 124 ns        │ 36.5 ns       │ 39.58 ns      │ 1000    │ 512000
│  ├─ i16                                                │               │               │               │         │
│  │  ├─ baseline_identity                 18.93 ns      │ 205.2 ns      │ 25.57 ns      │ 28.46 ns      │ 1000    │ 256000
│  │  ├─ pow2_mul                          182.6 ns      │ 748.2 ns      │ 188.8 ns      │ 190.1 ns      │ 1000    │ 64000
│  │  ├─ pow2_mul_const                    49 ns         │ 733.7 ns      │ 60.33 ns      │ 67.28 ns      │ 1000    │ 256000
│  │  ├─ pow2_mul_reuse                    54.08 ns      │ 174.7 ns      │ 65.41 ns      │ 70.22 ns      │ 1000    │ 256000
│  │  ├─ std_mul                           61.5 ns       │ 211.5 ns      │ 65.41 ns      │ 67.78 ns      │ 1000    │ 128000
│  │  ├─ std_mul_const                     48.22 ns      │ 150.1 ns      │ 54.47 ns      │ 59.89 ns      │ 1000    │ 256000
│  │  ├─ std_mul_reuse                     52.13 ns      │ 157.2 ns      │ 56.43 ns      │ 57.91 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_mul                      185.7 ns      │ 266.9 ns      │ 190.4 ns      │ 191 ns        │ 1000    │ 64000
│  │  ├─ unb_pow2_mul_const                47.44 ns      │ 288.8 ns      │ 52.91 ns      │ 57.4 ns       │ 1000    │ 256000
│  │  ╰─ unb_pow2_mul_reuse                52.91 ns      │ 332.2 ns      │ 65.02 ns      │ 70.04 ns      │ 1000    │ 256000
│  ├─ i32                                                │               │               │               │         │
│  │  ├─ baseline_identity                 42.75 ns      │ 484.9 ns      │ 53.69 ns      │ 55.85 ns      │ 1000    │ 256000
│  │  ├─ pow2_mul                          276.3 ns      │ 409.1 ns      │ 291.9 ns      │ 292.8 ns      │ 1000    │ 64000
│  │  ├─ pow2_mul_const                    91.19 ns      │ 308.3 ns      │ 109.9 ns      │ 116.7 ns      │ 1000    │ 128000
│  │  ├─ pow2_mul_reuse                    102.1 ns      │ 293.5 ns      │ 121.6 ns      │ 128.9 ns      │ 1000    │ 128000
│  │  ├─ std_mul                           184.1 ns      │ 463.8 ns      │ 193.5 ns      │ 196 ns        │ 1000    │ 64000
│  │  ├─ std_mul_const                     91.19 ns      │ 256 ns        │ 99.79 ns      │ 103.5 ns      │ 1000    │ 128000
│  │  ├─ std_mul_reuse                     142.7 ns      │ 705.2 ns      │ 153.6 ns      │ 158.6 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_mul                      273.2 ns      │ 2.807 µs      │ 282.6 ns      │ 297 ns        │ 1000    │ 64000
│  │  ├─ unb_pow2_mul_const                91.19 ns      │ 1.281 µs      │ 99 ns         │ 104.2 ns      │ 1000    │ 128000
│  │  ╰─ unb_pow2_mul_reuse                96.66 ns      │ 240.4 ns      │ 107.6 ns      │ 114.7 ns      │ 1000    │ 128000
│  ├─ i64                                                │               │               │               │         │
│  │  ├─ baseline_identity                 80.25 ns      │ 456.8 ns      │ 119.3 ns      │ 125 ns        │ 1000    │ 128000
│  │  ├─ pow2_mul                          327.9 ns      │ 5.19 µs       │ 346.6 ns      │ 363 ns        │ 1000    │ 32000
│  │  ├─ pow2_mul_const                    182.6 ns      │ 1.907 µs      │ 223.2 ns      │ 241 ns        │ 1000    │ 64000
│  │  ├─ pow2_mul_reuse                    191.9 ns      │ 1.398 µs      │ 221.6 ns      │ 235.2 ns      │ 1000    │ 64000
│  │  ├─ std_mul                           431 ns        │ 1.534 µs      │ 449.7 ns      │ 455.3 ns      │ 1000    │ 32000
│  │  ├─ std_mul_const                     184.1 ns      │ 510.7 ns      │ 221.6 ns      │ 236.9 ns      │ 1000    │ 64000
│  │  ├─ std_mul_reuse                     307.6 ns      │ 2.726 µs      │ 329.4 ns      │ 342.7 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_mul                      424.7 ns      │ 874.7 ns      │ 437.2 ns      │ 440.4 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_mul_const                182.6 ns      │ 468.5 ns      │ 201.3 ns      │ 208.5 ns      │ 1000    │ 64000
│  │  ╰─ unb_pow2_mul_reuse                195.1 ns      │ 602.9 ns      │ 245.1 ns      │ 256.8 ns      │ 1000    │ 64000
│  ├─ i128                                               │               │               │               │         │
│  │  ├─ baseline_identity                 224.7 ns      │ 626.3 ns      │ 243.5 ns      │ 272 ns        │ 1000    │ 64000
│  │  ├─ pow2_mul                          949.7 ns      │ 10 µs         │ 981 ns        │ 1.012 µs      │ 1000    │ 16000
│  │  ├─ pow2_mul_const                    787.2 ns      │ 2.549 µs      │ 824.7 ns      │ 857.6 ns      │ 1000    │ 16000
│  │  ├─ pow2_mul_reuse                    1.006 µs      │ 1.468 µs      │ 1.037 µs      │ 1.043 µs      │ 1000    │ 16000
│  │  ├─ std_mul                           1.224 µs      │ 1.799 µs      │ 1.274 µs      │ 1.279 µs      │ 1000    │ 8000
│  │  ├─ std_mul_const                     799.7 ns      │ 2.206 µs      │ 818.5 ns      │ 836.2 ns      │ 1000    │ 16000
│  │  ├─ std_mul_reuse                     812.2 ns      │ 2.343 µs      │ 824.7 ns      │ 844 ns        │ 1000    │ 16000
│  │  ├─ unb_pow2_mul                      937.2 ns      │ 1.706 µs      │ 949.7 ns      │ 977.2 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_mul_const                793.5 ns      │ 1.474 µs      │ 812.2 ns      │ 835.4 ns      │ 1000    │ 16000
│  │  ╰─ unb_pow2_mul_reuse                1.012 µs      │ 1.843 µs      │ 1.024 µs      │ 1.047 µs      │ 1000    │ 16000
│  ├─ u8                                                 │               │               │               │         │
│  │  ├─ baseline_identity                 11.99 ns      │ 45 ns         │ 13.55 ns      │ 14.79 ns      │ 1000    │ 1024000
│  │  ├─ pow2_mul                          97.44 ns      │ 202.1 ns      │ 99 ns         │ 100.2 ns      │ 1000    │ 128000
│  │  ├─ pow2_mul_const                    26.93 ns      │ 101.7 ns      │ 31.82 ns      │ 35.35 ns      │ 1000    │ 512000
│  │  ├─ pow2_mul_reuse                    30.25 ns      │ 95.29 ns      │ 34.94 ns      │ 37.76 ns      │ 1000    │ 512000
│  │  ├─ std_mul                           54.47 ns      │ 109.9 ns      │ 59.16 ns      │ 61.29 ns      │ 1000    │ 256000
│  │  ├─ std_mul_const                     26.93 ns      │ 106.2 ns      │ 33.18 ns      │ 36.69 ns      │ 1000    │ 512000
│  │  ├─ std_mul_reuse                     34.94 ns      │ 62.29 ns      │ 36.89 ns      │ 38.12 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_mul                      100.5 ns      │ 203.6 ns      │ 103.6 ns      │ 105.4 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_mul_const                26.74 ns      │ 145.6 ns      │ 32.99 ns      │ 37.11 ns      │ 1000    │ 512000
│  │  ╰─ unb_pow2_mul_reuse                29.08 ns      │ 113 ns        │ 33.96 ns      │ 35.44 ns      │ 1000    │ 512000
│  ├─ u16                                                │               │               │               │         │
│  │  ├─ baseline_identity                 21.46 ns      │ 84.16 ns      │ 32.4 ns       │ 32.42 ns      │ 1000    │ 512000
│  │  ├─ pow2_mul                          182.6 ns      │ 487.2 ns      │ 187.2 ns      │ 189.6 ns      │ 1000    │ 64000
│  │  ├─ pow2_mul_const                    47.44 ns      │ 193.5 ns      │ 55.25 ns      │ 60.92 ns      │ 1000    │ 256000
│  │  ├─ pow2_mul_reuse                    52.52 ns      │ 238.8 ns      │ 63.85 ns      │ 67.86 ns      │ 1000    │ 256000
│  │  ├─ std_mul                           60.72 ns      │ 155.2 ns      │ 65.41 ns      │ 67.16 ns      │ 1000    │ 128000
│  │  ├─ std_mul_const                     48.22 ns      │ 176.3 ns      │ 57.21 ns      │ 61.72 ns      │ 1000    │ 256000
│  │  ├─ std_mul_reuse                     49.39 ns      │ 140.8 ns      │ 54.86 ns      │ 57.78 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_mul                      184.1 ns      │ 2.524 µs      │ 187.2 ns      │ 192.4 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_mul_const                48.22 ns      │ 312.6 ns      │ 54.47 ns      │ 60.69 ns      │ 1000    │ 256000
│  │  ╰─ unb_pow2_mul_reuse                53.69 ns      │ 189.2 ns      │ 63.46 ns      │ 69.19 ns      │ 1000    │ 256000
│  ├─ u32                                                │               │               │               │         │
│  │  ├─ baseline_identity                 43.14 ns      │ 559.1 ns      │ 54.86 ns      │ 56.34 ns      │ 1000    │ 256000
│  │  ├─ pow2_mul                          279.4 ns      │ 612.2 ns      │ 291.1 ns      │ 293.4 ns      │ 1000    │ 64000
│  │  ├─ pow2_mul_const                    91.19 ns      │ 296.6 ns      │ 102.1 ns      │ 108.7 ns      │ 1000    │ 128000
│  │  ├─ pow2_mul_reuse                    101.3 ns      │ 367.7 ns      │ 110.7 ns      │ 118.3 ns      │ 1000    │ 128000
│  │  ├─ std_mul                           182.6 ns      │ 1.89 µs       │ 191.9 ns      │ 199.9 ns      │ 1000    │ 64000
│  │  ├─ std_mul_const                     90.41 ns      │ 266.9 ns      │ 108.7 ns      │ 116.1 ns      │ 1000    │ 128000
│  │  ├─ std_mul_reuse                     127.9 ns      │ 271.6 ns      │ 134.1 ns      │ 135.9 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_mul                      271.6 ns      │ 723.2 ns      │ 279.4 ns      │ 286.6 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_mul_const                91.97 ns      │ 466.1 ns      │ 109.9 ns      │ 117.9 ns      │ 1000    │ 128000
│  │  ╰─ unb_pow2_mul_reuse                98.22 ns      │ 395.1 ns      │ 120.4 ns      │ 128.2 ns      │ 1000    │ 128000
│  ├─ u64                                                │               │               │               │         │
│  │  ├─ baseline_identity                 80.25 ns      │ 346.6 ns      │ 109.5 ns      │ 114 ns        │ 1000    │ 128000
│  │  ├─ pow2_mul                          331 ns        │ 1.212 µs      │ 343.5 ns      │ 360.9 ns      │ 1000    │ 16000
│  │  ├─ pow2_mul_const                    185.7 ns      │ 574.7 ns      │ 206 ns        │ 222.9 ns      │ 1000    │ 64000
│  │  ├─ pow2_mul_reuse                    193.5 ns      │ 551.3 ns      │ 228.6 ns      │ 243.4 ns      │ 1000    │ 64000
│  │  ├─ std_mul                           437.2 ns      │ 1.224 µs      │ 456 ns        │ 468.1 ns      │ 1000    │ 32000
│  │  ├─ std_mul_const                     184.1 ns      │ 677.9 ns      │ 207.6 ns      │ 218.9 ns      │ 1000    │ 64000
│  │  ├─ std_mul_reuse                     309.1 ns      │ 1.96 µs       │ 332.6 ns      │ 340.7 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_mul                      431 ns        │ 940.4 ns      │ 440.4 ns      │ 445.6 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_mul_const                185.7 ns      │ 634.1 ns      │ 212.2 ns      │ 226.3 ns      │ 1000    │ 64000
│  │  ╰─ unb_pow2_mul_reuse                188.8 ns      │ 570.1 ns      │ 232.6 ns      │ 252 ns        │ 1000    │ 64000
│  ╰─ u128                                               │               │               │               │         │
│     ├─ baseline_identity                 223.2 ns      │ 563.8 ns      │ 248.2 ns      │ 271.4 ns      │ 1000    │ 64000
│     ├─ pow2_mul                          974.7 ns      │ 2.031 µs      │ 993.5 ns      │ 1.032 µs      │ 1000    │ 16000
│     ├─ pow2_mul_const                    793.5 ns      │ 1.587 µs      │ 818.5 ns      │ 827.1 ns      │ 1000    │ 16000
│     ├─ pow2_mul_reuse                    1.012 µs      │ 9.599 µs      │ 1.037 µs      │ 1.067 µs      │ 1000    │ 16000
│     ├─ std_mul                           1.231 µs      │ 2.093 µs      │ 1.306 µs      │ 1.326 µs      │ 1000    │ 16000
│     ├─ std_mul_const                     793.5 ns      │ 1.462 µs      │ 812.2 ns      │ 819.3 ns      │ 1000    │ 16000
│     ├─ std_mul_reuse                     806 ns        │ 2.681 µs      │ 818.5 ns      │ 846.2 ns      │ 1000    │ 16000
│     ├─ unb_pow2_mul                      949.7 ns      │ 1.793 µs      │ 962.2 ns      │ 973.1 ns      │ 1000    │ 16000
│     ├─ unb_pow2_mul_const                806 ns        │ 2.081 µs      │ 818.5 ns      │ 829.5 ns      │ 1000    │ 16000
│     ╰─ unb_pow2_mul_reuse                1.012 µs      │ 1.756 µs      │ 1.024 µs      │ 1.041 µs      │ 1000    │ 16000
├─ rem                                                   │               │               │               │         │
│  ├─ i8                                                 │               │               │               │         │
│  │  ├─ baseline_identity                 12.19 ns      │ 53.59 ns      │ 13.46 ns      │ 15.56 ns      │ 1000    │ 1024000
│  │  ├─ pow2_is_multiple_of               77.52 ns      │ 145.1 ns      │ 80.25 ns      │ 81.74 ns      │ 1000    │ 256000
│  │  ├─ pow2_is_multiple_of_const         29.67 ns      │ 115.4 ns      │ 33.77 ns      │ 36.83 ns      │ 1000    │ 512000
│  │  ├─ pow2_is_multiple_of_reuse         67.75 ns      │ 165 ns        │ 70.1 ns       │ 70.74 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem                          356 ns        │ 496.6 ns      │ 359.1 ns      │ 360 ns        │ 1000    │ 32000
│  │  ├─ pow2_rem_const                    30.25 ns      │ 60.72 ns      │ 36.11 ns      │ 36.31 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem_floor                    92.75 ns      │ 1.174 µs      │ 95.88 ns      │ 98.29 ns      │ 1000    │ 128000
│  │  ├─ pow2_rem_floor_const              24.98 ns      │ 362 ns        │ 27.91 ns      │ 31.73 ns      │ 1000    │ 512000
│  │  ├─ pow2_rem_floor_reuse              26.93 ns      │ 121 ns        │ 31.82 ns      │ 35.07 ns      │ 1000    │ 512000
│  │  ├─ pow2_rem_reuse                    51.35 ns      │ 186.5 ns      │ 54.08 ns      │ 54.73 ns      │ 1000    │ 256000
│  │  ├─ std_rem                           862.2 ns      │ 2.012 µs      │ 874.7 ns      │ 883.8 ns      │ 1000    │ 16000
│  │  ├─ std_rem_const                     35.72 ns      │ 85.33 ns      │ 38.46 ns      │ 38.74 ns      │ 1000    │ 256000
│  │  ├─ std_rem_reuse                     837.2 ns      │ 1.512 µs      │ 843.5 ns      │ 850.5 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_is_multiple_of           80.25 ns      │ 187.2 ns      │ 84.94 ns      │ 85.47 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_is_multiple_of_const     29.47 ns      │ 129.8 ns      │ 35.62 ns      │ 38.06 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_is_multiple_of_reuse     68.14 ns      │ 155.6 ns      │ 69.71 ns      │ 70.96 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_rem                      116.9 ns      │ 266.9 ns      │ 120.8 ns      │ 121.5 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_rem_const                31.04 ns      │ 138.8 ns      │ 33.38 ns      │ 34.59 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_rem_floor                98.22 ns      │ 198.2 ns      │ 100.5 ns      │ 102.3 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_rem_floor_const          25.18 ns      │ 82.6 ns       │ 30.06 ns      │ 33.07 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_rem_floor_reuse          27.13 ns      │ 98.81 ns      │ 31.43 ns      │ 34.33 ns      │ 1000    │ 512000
│  │  ╰─ unb_pow2_rem_reuse                31.04 ns      │ 57.21 ns      │ 32.99 ns      │ 33.82 ns      │ 1000    │ 256000
│  ├─ i16                                                │               │               │               │         │
│  │  ├─ baseline_identity                 22.44 ns      │ 298.6 ns      │ 32.4 ns       │ 33.84 ns      │ 1000    │ 512000
│  │  ├─ pow2_is_multiple_of               181 ns        │ 327.9 ns      │ 184.1 ns      │ 185.5 ns      │ 1000    │ 64000
│  │  ├─ pow2_is_multiple_of_const         39.63 ns      │ 93.54 ns      │ 43.14 ns      │ 43.87 ns      │ 1000    │ 256000
│  │  ├─ pow2_is_multiple_of_reuse         182.6 ns      │ 318.5 ns      │ 184.1 ns      │ 186.3 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem                          459.1 ns      │ 837.2 ns      │ 462.2 ns      │ 467.7 ns      │ 1000    │ 32000
│  │  ├─ pow2_rem_const                    71.66 ns      │ 533.7 ns      │ 81.82 ns      │ 86.98 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem_floor                    190.4 ns      │ 640.4 ns      │ 193.5 ns      │ 196.9 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem_floor_const              47.83 ns      │ 613.4 ns      │ 58.38 ns      │ 64.29 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem_floor_reuse              53.3 ns       │ 171.6 ns      │ 65.8 ns       │ 70.72 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem_reuse                    79.47 ns      │ 218.9 ns      │ 95.49 ns      │ 101.3 ns      │ 1000    │ 256000
│  │  ├─ std_rem                           874.7 ns      │ 1.281 µs      │ 893.5 ns      │ 902.7 ns      │ 1000    │ 16000
│  │  ├─ std_rem_const                     59.16 ns      │ 156.8 ns      │ 67.75 ns      │ 68 ns         │ 1000    │ 128000
│  │  ├─ std_rem_reuse                     2.524 µs      │ 5.449 µs      │ 2.549 µs      │ 2.557 µs      │ 1000    │ 4000
│  │  ├─ unb_pow2_is_multiple_of           181 ns        │ 415.4 ns      │ 184.1 ns      │ 187.2 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_is_multiple_of_const     38.07 ns      │ 103.3 ns      │ 43.14 ns      │ 43.76 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_is_multiple_of_reuse     181 ns        │ 574.7 ns      │ 184.1 ns      │ 186 ns        │ 1000    │ 64000
│  │  ├─ unb_pow2_rem                      224.7 ns      │ 440.4 ns      │ 229.4 ns      │ 230.7 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_rem_const                71.27 ns      │ 215 ns        │ 81.82 ns      │ 86.59 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_rem_floor                191.9 ns      │ 424.7 ns      │ 195.1 ns      │ 197.5 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_rem_floor_const          50.18 ns      │ 163.4 ns      │ 61.5 ns       │ 66.16 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_rem_floor_reuse          54.86 ns      │ 157.9 ns      │ 64.63 ns      │ 69.23 ns      │ 1000    │ 256000
│  │  ╰─ unb_pow2_rem_reuse                70.88 ns      │ 205.6 ns      │ 84.55 ns      │ 89.82 ns      │ 1000    │ 256000
│  ├─ i32                                                │               │               │               │         │
│  │  ├─ baseline_identity                 42.75 ns      │ 142.3 ns      │ 52.13 ns      │ 54.54 ns      │ 1000    │ 256000
│  │  ├─ pow2_is_multiple_of               456 ns        │ 1.309 µs      │ 462.2 ns      │ 465 ns        │ 1000    │ 32000
│  │  ├─ pow2_is_multiple_of_const         93.54 ns      │ 224.7 ns      │ 107.6 ns      │ 108.8 ns      │ 1000    │ 128000
│  │  ├─ pow2_is_multiple_of_reuse         421.6 ns      │ 890.4 ns      │ 427.9 ns      │ 430.8 ns      │ 1000    │ 32000
│  │  ├─ pow2_rem                          681 ns        │ 1.556 µs      │ 706 ns        │ 825.5 ns      │ 1000    │ 16000
│  │  ├─ pow2_rem_const                    120.1 ns      │ 298.2 ns      │ 131 ns        │ 154.6 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem_floor                    265.4 ns      │ 454.4 ns      │ 270.1 ns      │ 273.6 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem_floor_const              95.1 ns       │ 396.6 ns      │ 113.8 ns      │ 127.5 ns      │ 1000    │ 128000
│  │  ├─ pow2_rem_floor_reuse              98.22 ns      │ 298.2 ns      │ 117.7 ns      │ 126 ns        │ 1000    │ 128000
│  │  ├─ pow2_rem_reuse                    145.1 ns      │ 364.6 ns      │ 173.2 ns      │ 180.3 ns      │ 1000    │ 128000
│  │  ├─ std_rem                           1.312 µs      │ 3.599 µs      │ 1.337 µs      │ 1.346 µs      │ 1000    │ 8000
│  │  ├─ std_rem_const                     129.4 ns      │ 752.9 ns      │ 146.6 ns      │ 154.3 ns      │ 1000    │ 128000
│  │  ├─ std_rem_reuse                     1.262 µs      │ 1.787 µs      │ 1.287 µs      │ 1.288 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_is_multiple_of           449.7 ns      │ 1.252 µs      │ 462.2 ns      │ 471.4 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_is_multiple_of_const     95.88 ns      │ 224 ns        │ 106.8 ns      │ 107.9 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_is_multiple_of_reuse     418.5 ns      │ 646.6 ns      │ 424.7 ns      │ 429.8 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_rem                      346.6 ns      │ 631 ns        │ 352.9 ns      │ 358 ns        │ 1000    │ 32000
│  │  ├─ unb_pow2_rem_const                120.1 ns      │ 268.5 ns      │ 126.3 ns      │ 130.7 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_rem_floor                274.7 ns      │ 640.4 ns      │ 281 ns        │ 284.6 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_rem_floor_const          92.75 ns      │ 245.1 ns      │ 102.1 ns      │ 104.8 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_rem_floor_reuse          98.22 ns      │ 325.5 ns      │ 109.1 ns      │ 113.7 ns      │ 1000    │ 128000
│  │  ╰─ unb_pow2_rem_reuse                138.8 ns      │ 348.2 ns      │ 164.6 ns      │ 174.6 ns      │ 1000    │ 128000
│  ├─ i64                                                │               │               │               │         │
│  │  ├─ baseline_identity                 80.25 ns      │ 362.2 ns      │ 124.3 ns      │ 129.3 ns      │ 1000    │ 128000
│  │  ├─ pow2_is_multiple_of               343.5 ns      │ 781 ns        │ 362.2 ns      │ 367.6 ns      │ 1000    │ 32000
│  │  ├─ pow2_is_multiple_of_const         262.2 ns      │ 556 ns        │ 268.5 ns      │ 272.3 ns      │ 1000    │ 64000
│  │  ├─ pow2_is_multiple_of_reuse         315.4 ns      │ 852.9 ns      │ 321.6 ns      │ 323.4 ns      │ 1000    │ 32000
│  │  ├─ pow2_rem                          737.2 ns      │ 1.687 µs      │ 749.7 ns      │ 753.6 ns      │ 1000    │ 16000
│  │  ├─ pow2_rem_const                    252.9 ns      │ 565.4 ns      │ 268.5 ns      │ 273.2 ns      │ 1000    │ 32000
│  │  ├─ pow2_rem_floor                    456 ns        │ 1.518 µs      │ 465.4 ns      │ 471.7 ns      │ 1000    │ 32000
│  │  ├─ pow2_rem_floor_const              188.8 ns      │ 548.2 ns      │ 207.6 ns      │ 230.5 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem_floor_reuse              193.5 ns      │ 737.2 ns      │ 227.9 ns      │ 248.2 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem_reuse                    562.2 ns      │ 1.312 µs      │ 574.7 ns      │ 582.2 ns      │ 1000    │ 32000
│  │  ├─ std_rem                           1.662 µs      │ 20.73 µs      │ 1.687 µs      │ 1.74 µs       │ 1000    │ 8000
│  │  ├─ std_rem_const                     273.2 ns      │ 706 ns        │ 302.9 ns      │ 318.7 ns      │ 1000    │ 64000
│  │  ├─ std_rem_reuse                     1.512 µs      │ 2.049 µs      │ 1.524 µs      │ 1.535 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_is_multiple_of           340.4 ns      │ 562.2 ns      │ 356 ns        │ 360.8 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_is_multiple_of_const     262.2 ns      │ 571.6 ns      │ 268.5 ns      │ 275.3 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_is_multiple_of_reuse     315.4 ns      │ 668.5 ns      │ 321.6 ns      │ 324.4 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_rem                      643.5 ns      │ 1.331 µs      │ 656 ns        │ 667.3 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_rem_const                268.5 ns      │ 565.4 ns      │ 296.6 ns      │ 310.9 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_rem_floor                493.5 ns      │ 1.337 µs      │ 502.9 ns      │ 510.6 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_rem_floor_const          187.2 ns      │ 1.976 µs      │ 212.2 ns      │ 229.9 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_rem_floor_reuse          196.6 ns      │ 618.5 ns      │ 240.4 ns      │ 255.5 ns      │ 1000    │ 64000
│  │  ╰─ unb_pow2_rem_reuse                276.3 ns      │ 901.3 ns      │ 316.9 ns      │ 336.5 ns      │ 1000    │ 64000
│  ├─ i128                                               │               │               │               │         │
│  │  ├─ baseline_identity                 223.2 ns      │ 970.1 ns      │ 241.9 ns      │ 269.1 ns      │ 1000    │ 64000
│  │  ├─ pow2_is_multiple_of               577.9 ns      │ 5.446 µs      │ 599.7 ns      │ 625.2 ns      │ 1000    │ 32000
│  │  ├─ pow2_is_multiple_of_const         281 ns        │ 1.843 µs      │ 296.6 ns      │ 307.5 ns      │ 1000    │ 64000
│  │  ├─ pow2_is_multiple_of_reuse         559.1 ns      │ 1.143 µs      │ 571.6 ns      │ 579.7 ns      │ 1000    │ 32000
│  │  ├─ pow2_rem                          2.524 µs      │ 6.274 µs      │ 2.549 µs      │ 2.579 µs      │ 1000    │ 4000
│  │  ├─ pow2_rem_const                    799.7 ns      │ 2.131 µs      │ 818.5 ns      │ 835 ns        │ 1000    │ 16000
│  │  ├─ pow2_rem_floor                    962.2 ns      │ 1.899 µs      │ 981 ns        │ 1.007 µs      │ 1000    │ 16000
│  │  ├─ pow2_rem_floor_const              443.5 ns      │ 1.152 µs      │ 495.1 ns      │ 536 ns        │ 1000    │ 32000
│  │  ├─ pow2_rem_floor_reuse              568.5 ns      │ 1.431 µs      │ 615.4 ns      │ 656.8 ns      │ 1000    │ 32000
│  │  ├─ pow2_rem_reuse                    2.049 µs      │ 4.212 µs      │ 2.074 µs      │ 2.099 µs      │ 1000    │ 8000
│  │  ├─ std_rem                           9.199 µs      │ 158.8 µs      │ 9.299 µs      │ 9.815 µs      │ 1000    │ 1000
│  │  ├─ std_rem_const                     762.2 ns      │ 7.006 µs      │ 781 ns        │ 791.1 ns      │ 1000    │ 16000
│  │  ├─ std_rem_reuse                     8.099 µs      │ 63.29 µs      │ 8.199 µs      │ 8.318 µs      │ 1000    │ 2000
│  │  ├─ unb_pow2_is_multiple_of           577.9 ns      │ 4.318 µs      │ 596.6 ns      │ 609.6 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_is_multiple_of_const     285.7 ns      │ 731 ns        │ 296.6 ns      │ 308.7 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_is_multiple_of_reuse     562.2 ns      │ 2.249 µs      │ 581 ns        │ 590.3 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_rem                      1.549 µs      │ 4.799 µs      │ 1.574 µs      │ 1.589 µs      │ 1000    │ 4000
│  │  ├─ unb_pow2_rem_const                799.7 ns      │ 1.656 µs      │ 818.5 ns      │ 823.7 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_rem_floor                1.099 µs      │ 1.943 µs      │ 1.118 µs      │ 1.132 µs      │ 1000    │ 16000
│  │  ├─ unb_pow2_rem_floor_const          465.4 ns      │ 1.206 µs      │ 499.7 ns      │ 536.1 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_rem_floor_reuse          568.5 ns      │ 1.434 µs      │ 627.9 ns      │ 670.9 ns      │ 1000    │ 32000
│  │  ╰─ unb_pow2_rem_reuse                949.7 ns      │ 1.724 µs      │ 974.7 ns      │ 992 ns        │ 1000    │ 16000
│  ├─ u8                                                 │               │               │               │         │
│  │  ├─ baseline_identity                 12.29 ns      │ 45.2 ns       │ 13.46 ns      │ 14.19 ns      │ 1000    │ 1024000
│  │  ├─ pow2_is_multiple_of               74.79 ns      │ 271.6 ns      │ 77.13 ns      │ 77.85 ns      │ 1000    │ 128000
│  │  ├─ pow2_is_multiple_of_const         29.28 ns      │ 229.4 ns      │ 35.92 ns      │ 39.4 ns       │ 1000    │ 512000
│  │  ├─ pow2_is_multiple_of_reuse         68.14 ns      │ 117.3 ns      │ 69.71 ns      │ 70.16 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem                          195.1 ns      │ 290.4 ns      │ 198.2 ns      │ 200.2 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem_const                    24.98 ns      │ 115.4 ns      │ 28.3 ns       │ 32.41 ns      │ 1000    │ 512000
│  │  ├─ pow2_rem_floor                    92.75 ns      │ 209.9 ns      │ 96.66 ns      │ 97.03 ns      │ 1000    │ 128000
│  │  ├─ pow2_rem_floor_const              25.18 ns      │ 106.6 ns      │ 27.32 ns      │ 31.1 ns       │ 1000    │ 512000
│  │  ├─ pow2_rem_floor_reuse              27.13 ns      │ 109.7 ns      │ 32.4 ns       │ 36.21 ns      │ 1000    │ 512000
│  │  ├─ pow2_rem_reuse                    32.99 ns      │ 64.24 ns      │ 34.16 ns      │ 35.1 ns       │ 1000    │ 256000
│  │  ├─ std_rem                           862.2 ns      │ 1.143 µs      │ 868.5 ns      │ 870.6 ns      │ 1000    │ 16000
│  │  ├─ std_rem_const                     24.98 ns      │ 96.66 ns      │ 29.28 ns      │ 31.67 ns      │ 1000    │ 512000
│  │  ├─ std_rem_reuse                     849.7 ns      │ 1.724 µs      │ 856 ns        │ 876 ns        │ 1000    │ 16000
│  │  ├─ unb_pow2_is_multiple_of           78.69 ns      │ 188.8 ns      │ 81.04 ns      │ 81.86 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_is_multiple_of_const     29.28 ns      │ 112.4 ns      │ 35.72 ns      │ 37.95 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_is_multiple_of_reuse     68.14 ns      │ 116.5 ns      │ 70.1 ns       │ 71.13 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_rem                      100.5 ns      │ 194.3 ns      │ 104.4 ns      │ 105.2 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_rem_const                24.98 ns      │ 113.4 ns      │ 30.45 ns      │ 33.39 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_rem_floor                99 ns         │ 209.1 ns      │ 101.3 ns      │ 102.6 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_rem_floor_const          24.79 ns      │ 76.35 ns      │ 26.35 ns      │ 27.55 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_rem_floor_reuse          26.74 ns      │ 74 ns         │ 28.3 ns       │ 29.87 ns      │ 1000    │ 512000
│  │  ╰─ unb_pow2_rem_reuse                26.93 ns      │ 93.73 ns      │ 31.62 ns      │ 34.34 ns      │ 1000    │ 512000
│  ├─ u16                                                │               │               │               │         │
│  │  ├─ baseline_identity                 19.32 ns      │ 113 ns        │ 28.3 ns       │ 30.93 ns      │ 1000    │ 256000
│  │  ├─ pow2_is_multiple_of               181 ns        │ 398.2 ns      │ 185.7 ns      │ 192.5 ns      │ 1000    │ 64000
│  │  ├─ pow2_is_multiple_of_const         38.46 ns      │ 96.66 ns      │ 42.36 ns      │ 43.22 ns      │ 1000    │ 256000
│  │  ├─ pow2_is_multiple_of_reuse         179.4 ns      │ 352.9 ns      │ 182.6 ns      │ 183.7 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem                          381 ns        │ 749.7 ns      │ 384.1 ns      │ 389.6 ns      │ 1000    │ 32000
│  │  ├─ pow2_rem_const                    50.18 ns      │ 184.9 ns      │ 60.72 ns      │ 65.61 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem_floor                    187.2 ns      │ 402.9 ns      │ 193.5 ns      │ 194.7 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem_floor_const              50.57 ns      │ 235.3 ns      │ 61.5 ns       │ 67.75 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem_floor_reuse              53.69 ns      │ 165.4 ns      │ 64.82 ns      │ 70.2 ns       │ 1000    │ 256000
│  │  ├─ pow2_rem_reuse                    61.89 ns      │ 326.3 ns      │ 75.96 ns      │ 83.03 ns      │ 1000    │ 256000
│  │  ├─ std_rem                           868.5 ns      │ 1.206 µs      │ 874.7 ns      │ 880.8 ns      │ 1000    │ 16000
│  │  ├─ std_rem_const                     50.57 ns      │ 172.4 ns      │ 61.5 ns       │ 68.37 ns      │ 1000    │ 256000
│  │  ├─ std_rem_reuse                     837.2 ns      │ 1.618 µs      │ 843.5 ns      │ 848.5 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_is_multiple_of           181 ns        │ 346.6 ns      │ 184.1 ns      │ 187.2 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_is_multiple_of_const     42.75 ns      │ 101.7 ns      │ 45.1 ns       │ 46.08 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_is_multiple_of_reuse     179.4 ns      │ 274.7 ns      │ 182.6 ns      │ 185.4 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_rem                      191.9 ns      │ 543.5 ns      │ 196.6 ns      │ 200.4 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_rem_const                50.96 ns      │ 248.2 ns      │ 59.16 ns      │ 62.71 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_rem_floor                191.9 ns      │ 621.6 ns      │ 195.1 ns      │ 197.3 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_rem_floor_const          50.57 ns      │ 170.8 ns      │ 62.29 ns      │ 67.3 ns       │ 1000    │ 256000
│  │  ├─ unb_pow2_rem_floor_reuse          55.64 ns      │ 221.6 ns      │ 67.56 ns      │ 73.26 ns      │ 1000    │ 256000
│  │  ╰─ unb_pow2_rem_reuse                55.64 ns      │ 220.1 ns      │ 72.44 ns      │ 78.79 ns      │ 1000    │ 256000
│  ├─ u32                                                │               │               │               │         │
│  │  ├─ baseline_identity                 42.75 ns      │ 193.5 ns      │ 51.15 ns      │ 54.09 ns      │ 1000    │ 256000
│  │  ├─ pow2_is_multiple_of               452.9 ns      │ 846.6 ns      │ 459.1 ns      │ 465 ns        │ 1000    │ 32000
│  │  ├─ pow2_is_multiple_of_const         94.32 ns      │ 293.5 ns      │ 106.8 ns      │ 107.3 ns      │ 1000    │ 128000
│  │  ├─ pow2_is_multiple_of_reuse         421.6 ns      │ 927.9 ns      │ 427.9 ns      │ 432.1 ns      │ 1000    │ 32000
│  │  ├─ pow2_rem                          418.5 ns      │ 852.9 ns      │ 427.9 ns      │ 430.6 ns      │ 1000    │ 32000
│  │  ├─ pow2_rem_const                    91.19 ns      │ 274.7 ns      │ 103.3 ns      │ 112.7 ns      │ 1000    │ 128000
│  │  ├─ pow2_rem_floor                    263.8 ns      │ 466.9 ns      │ 268.5 ns      │ 270.9 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem_floor_const              93.54 ns      │ 298.2 ns      │ 102.9 ns      │ 111.8 ns      │ 1000    │ 128000
│  │  ├─ pow2_rem_floor_reuse              98.22 ns      │ 363 ns        │ 119.3 ns      │ 127.9 ns      │ 1000    │ 128000
│  │  ├─ pow2_rem_reuse                    116.1 ns      │ 357.6 ns      │ 131.8 ns      │ 141.8 ns      │ 1000    │ 128000
│  │  ├─ std_rem                           1.324 µs      │ 2.074 µs      │ 1.337 µs      │ 1.347 µs      │ 1000    │ 8000
│  │  ├─ std_rem_const                     93.54 ns      │ 262.2 ns      │ 104.4 ns      │ 110.2 ns      │ 1000    │ 128000
│  │  ├─ std_rem_reuse                     1.274 µs      │ 1.812 µs      │ 1.287 µs      │ 1.294 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_is_multiple_of           452.9 ns      │ 1.006 µs      │ 462.2 ns      │ 469.4 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_is_multiple_of_const     92.75 ns      │ 195.8 ns      │ 106 ns        │ 106.9 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_is_multiple_of_reuse     421.6 ns      │ 793.5 ns      │ 427.9 ns      │ 430.1 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_rem                      277.9 ns      │ 384.1 ns      │ 284.1 ns      │ 287.3 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_rem_const                92.75 ns      │ 288.8 ns      │ 109.9 ns      │ 116.8 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_rem_floor                277.9 ns      │ 424.7 ns      │ 282.6 ns      │ 285.7 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_rem_floor_const          94.32 ns      │ 320.8 ns      │ 105.2 ns      │ 115 ns        │ 1000    │ 128000
│  │  ├─ unb_pow2_rem_floor_reuse          97.44 ns      │ 783.3 ns      │ 115.4 ns      │ 124.6 ns      │ 1000    │ 128000
│  │  ╰─ unb_pow2_rem_reuse                98.22 ns      │ 535.7 ns      │ 115.4 ns      │ 123.9 ns      │ 1000    │ 128000
│  ├─ u64                                                │               │               │               │         │
│  │  ├─ baseline_identity                 80.25 ns      │ 438.8 ns      │ 123.2 ns      │ 129.7 ns      │ 1000    │ 128000
│  │  ├─ pow2_is_multiple_of               340.4 ns      │ 631 ns        │ 365.4 ns      │ 369.1 ns      │ 1000    │ 32000
│  │  ├─ pow2_is_multiple_of_const         262.2 ns      │ 496.6 ns      │ 270.1 ns      │ 275 ns        │ 1000    │ 64000
│  │  ├─ pow2_is_multiple_of_reuse         315.4 ns      │ 515.4 ns      │ 321.6 ns      │ 325.1 ns      │ 1000    │ 32000
│  │  ├─ pow2_rem                          443.5 ns      │ 11.67 µs      │ 456 ns        │ 569.9 ns      │ 1000    │ 32000
│  │  ├─ pow2_rem_const                    182.6 ns      │ 515.4 ns      │ 209.1 ns      │ 223.9 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem_floor                    459.1 ns      │ 999.7 ns      │ 474.7 ns      │ 480.6 ns      │ 1000    │ 32000
│  │  ├─ pow2_rem_floor_const              185.7 ns      │ 632.6 ns      │ 216.9 ns      │ 227.9 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem_floor_reuse              195.1 ns      │ 607.6 ns      │ 226.3 ns      │ 234.1 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem_reuse                    224.7 ns      │ 1.801 µs      │ 259.1 ns      │ 273.6 ns      │ 1000    │ 64000
│  │  ├─ std_rem                           1.637 µs      │ 3.099 µs      │ 1.649 µs      │ 1.666 µs      │ 1000    │ 8000
│  │  ├─ std_rem_const                     185.7 ns      │ 693.5 ns      │ 212.2 ns      │ 217.4 ns      │ 1000    │ 64000
│  │  ├─ std_rem_reuse                     1.512 µs      │ 2.224 µs      │ 1.524 µs      │ 1.541 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_is_multiple_of           340.4 ns      │ 774.7 ns      │ 352.9 ns      │ 357.1 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_is_multiple_of_const     260.7 ns      │ 515.4 ns      │ 270.1 ns      │ 273.1 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_is_multiple_of_reuse     315.4 ns      │ 884.1 ns      │ 321.6 ns      │ 325.5 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_rem                      496.6 ns      │ 1.262 µs      │ 509.1 ns      │ 515 ns        │ 1000    │ 32000
│  │  ├─ unb_pow2_rem_const                187.2 ns      │ 760.7 ns      │ 210.7 ns      │ 226.3 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_rem_floor                493.5 ns      │ 6.106 µs      │ 509.1 ns      │ 524.7 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_rem_floor_const          185.7 ns      │ 2.465 µs      │ 207.6 ns      │ 221.5 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_rem_floor_reuse          195.1 ns      │ 809.1 ns      │ 251.3 ns      │ 262.8 ns      │ 1000    │ 64000
│  │  ╰─ unb_pow2_rem_reuse                196.6 ns      │ 554.4 ns      │ 223.2 ns      │ 234.4 ns      │ 1000    │ 64000
│  ╰─ u128                                               │               │               │               │         │
│     ├─ baseline_identity                 224.7 ns      │ 824.7 ns      │ 268.5 ns      │ 292.6 ns      │ 1000    │ 64000
│     ├─ pow2_is_multiple_of               590.4 ns      │ 1.437 µs      │ 599.7 ns      │ 610.3 ns      │ 1000    │ 32000
│     ├─ pow2_is_multiple_of_const         285.7 ns      │ 654.4 ns      │ 306 ns        │ 317.9 ns      │ 1000    │ 64000
│     ├─ pow2_is_multiple_of_reuse         549.7 ns      │ 5.359 µs      │ 568.5 ns      │ 581.7 ns      │ 1000    │ 32000
│     ├─ pow2_rem                          1.724 µs      │ 14.34 µs      │ 1.749 µs      │ 1.819 µs      │ 1000    │ 8000
│     ├─ pow2_rem_const                    462.2 ns      │ 4.174 µs      │ 493.5 ns      │ 511.8 ns      │ 1000    │ 32000
│     ├─ pow2_rem_floor                    968.5 ns      │ 1.943 µs      │ 999.7 ns      │ 1.017 µs      │ 1000    │ 16000
│     ├─ pow2_rem_floor_const              468.5 ns      │ 1.284 µs      │ 506 ns        │ 534.3 ns      │ 1000    │ 32000
│     ├─ pow2_rem_floor_reuse              574.7 ns      │ 1.318 µs      │ 612.2 ns      │ 638.1 ns      │ 1000    │ 32000
│     ├─ pow2_rem_reuse                    1.674 µs      │ 3.987 µs      │ 1.712 µs      │ 1.724 µs      │ 1000    │ 8000
│     ├─ std_rem                           4.099 µs      │ 7.349 µs      │ 4.124 µs      │ 4.169 µs      │ 1000    │ 4000
│     ├─ std_rem_const                     468.5 ns      │ 1.737 µs      │ 502.9 ns      │ 542.2 ns      │ 1000    │ 32000
│     ├─ std_rem_reuse                     4.224 µs      │ 5.424 µs      │ 4.299 µs      │ 4.311 µs      │ 1000    │ 4000
│     ├─ unb_pow2_is_multiple_of           581 ns        │ 1.027 µs      │ 593.5 ns      │ 604.8 ns      │ 1000    │ 32000
│     ├─ unb_pow2_is_multiple_of_const     287.2 ns      │ 643.5 ns      │ 296.6 ns      │ 308.6 ns      │ 1000    │ 64000
│     ├─ unb_pow2_is_multiple_of_reuse     552.9 ns      │ 1.015 µs      │ 571.6 ns      │ 578.9 ns      │ 1000    │ 32000
│     ├─ unb_pow2_rem                      1.093 µs      │ 1.924 µs      │ 1.106 µs      │ 1.123 µs      │ 1000    │ 16000
│     ├─ unb_pow2_rem_const                465.4 ns      │ 1.137 µs      │ 506 ns        │ 543.3 ns      │ 1000    │ 32000
│     ├─ unb_pow2_rem_floor                1.093 µs      │ 2.031 µs      │ 1.131 µs      │ 1.151 µs      │ 1000    │ 16000
│     ├─ unb_pow2_rem_floor_const          468.5 ns      │ 1.821 µs      │ 499.7 ns      │ 517.4 ns      │ 1000    │ 32000
│     ├─ unb_pow2_rem_floor_reuse          571.6 ns      │ 4.006 µs      │ 631 ns        │ 661.4 ns      │ 1000    │ 32000
│     ╰─ unb_pow2_rem_reuse                577.9 ns      │ 1.924 µs      │ 656 ns        │ 712.3 ns      │ 1000    │ 32000
╰─ round                                                 │               │               │               │         │
   ├─ i8                                                 │               │               │               │         │
   │  ├─ baseline_identity                 12.29 ns      │ 38.55 ns      │ 13.46 ns      │ 14.64 ns      │ 1000    │ 1024000
   │  ├─ pow2_ceil_to_multiple             97.44 ns      │ 226.3 ns      │ 99.79 ns      │ 101 ns        │ 1000    │ 128000
   │  ├─ pow2_ceil_to_multiple_const       26.74 ns      │ 90.61 ns      │ 29.67 ns      │ 30.58 ns      │ 1000    │ 512000
   │  ├─ pow2_ceil_to_multiple_reuse       29.08 ns      │ 67.75 ns      │ 32.01 ns      │ 32.61 ns      │ 1000    │ 512000
   │  ├─ pow2_floor_to_multiple            93.54 ns      │ 188.8 ns      │ 95.88 ns      │ 96.64 ns      │ 1000    │ 128000
   │  ├─ pow2_floor_to_multiple_const      24.79 ns      │ 88.85 ns      │ 28.01 ns      │ 31.01 ns      │ 1000    │ 512000
   │  ├─ pow2_floor_to_multiple_reuse      26.74 ns      │ 165 ns        │ 32.4 ns       │ 35.99 ns      │ 1000    │ 512000
   │  ├─ pow2_round_to_multiple            127.9 ns      │ 548.2 ns      │ 129.4 ns      │ 137.1 ns      │ 1000    │ 64000
   │  ├─ pow2_round_to_multiple_const      33.96 ns      │ 107.6 ns      │ 38.26 ns      │ 41.2 ns       │ 1000    │ 512000
   │  ├─ pow2_round_to_multiple_reuse      31.43 ns      │ 82.6 ns       │ 33.38 ns      │ 34.28 ns      │ 1000    │ 256000
   │  ├─ std_div_mul                       868.5 ns      │ 1.343 µs      │ 881 ns        │ 887.3 ns      │ 1000    │ 16000
   │  ├─ std_div_mul_const                 33.38 ns      │ 118.9 ns      │ 38.26 ns      │ 41.3 ns       │ 1000    │ 512000
   │  ├─ std_div_mul_reuse                 843.5 ns      │ 1.199 µs      │ 856 ns        │ 855.8 ns      │ 1000    │ 16000
   │  ├─ unb_pow2_ceil_to_multiple         104.4 ns      │ 203.6 ns      │ 107.6 ns      │ 110.3 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_ceil_to_multiple_const   27.32 ns      │ 97.83 ns      │ 33.96 ns      │ 36.51 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   27.91 ns      │ 105.6 ns      │ 33.18 ns      │ 35.82 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_floor_to_multiple        98.22 ns      │ 246.6 ns      │ 102.1 ns      │ 103.2 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_floor_to_multiple_const  24.98 ns      │ 96.66 ns      │ 26.93 ns      │ 29.88 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_floor_to_multiple_reuse  26.93 ns      │ 120.4 ns      │ 29.28 ns      │ 31.88 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_round_to_multiple        130.2 ns      │ 252.1 ns      │ 132.6 ns      │ 133.1 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_round_to_multiple_const  33.38 ns      │ 317.1 ns      │ 39.82 ns      │ 43.21 ns      │ 1000    │ 512000
   │  ╰─ unb_pow2_round_to_multiple_reuse  40.41 ns      │ 136.5 ns      │ 47.44 ns      │ 50.86 ns      │ 1000    │ 512000
   ├─ i16                                                │               │               │               │         │
   │  ├─ baseline_identity                 19.71 ns      │ 99.79 ns      │ 29.08 ns      │ 31.55 ns      │ 1000    │ 256000
   │  ├─ pow2_ceil_to_multiple             193.5 ns      │ 291.9 ns      │ 198.2 ns      │ 199.4 ns      │ 1000    │ 64000
   │  ├─ pow2_ceil_to_multiple_const       53.69 ns      │ 156.8 ns      │ 65.41 ns      │ 69.74 ns      │ 1000    │ 256000
   │  ├─ pow2_ceil_to_multiple_reuse       56.82 ns      │ 162.2 ns      │ 69.32 ns      │ 73.75 ns      │ 1000    │ 256000
   │  ├─ pow2_floor_to_multiple            188.8 ns      │ 421.6 ns      │ 193.5 ns      │ 196 ns        │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple_const      49.39 ns      │ 334.9 ns      │ 59.55 ns      │ 63.65 ns      │ 1000    │ 256000
   │  ├─ pow2_floor_to_multiple_reuse      51.74 ns      │ 193.5 ns      │ 63.85 ns      │ 68.35 ns      │ 1000    │ 256000
   │  ├─ pow2_round_to_multiple            223.2 ns      │ 460.7 ns      │ 226.3 ns      │ 227.2 ns      │ 1000    │ 64000
   │  ├─ pow2_round_to_multiple_const      64.63 ns      │ 581.8 ns      │ 78.69 ns      │ 85.1 ns       │ 1000    │ 256000
   │  ├─ pow2_round_to_multiple_reuse      72.44 ns      │ 281.8 ns      │ 88.07 ns      │ 93.08 ns      │ 1000    │ 256000
   │  ├─ std_div_mul                       2.524 µs      │ 5.699 µs      │ 2.549 µs      │ 2.562 µs      │ 1000    │ 4000
   │  ├─ std_div_mul_const                 64.24 ns      │ 530.6 ns      │ 78.3 ns       │ 84.45 ns      │ 1000    │ 256000
   │  ├─ std_div_mul_reuse                 2.574 µs      │ 3.599 µs      │ 2.599 µs      │ 2.595 µs      │ 1000    │ 4000
   │  ├─ unb_pow2_ceil_to_multiple         198.2 ns      │ 2.509 µs      │ 204.4 ns      │ 208 ns        │ 1000    │ 64000
   │  ├─ unb_pow2_ceil_to_multiple_const   54.08 ns      │ 232.6 ns      │ 63.85 ns      │ 69.34 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   41.97 ns      │ 503.6 ns      │ 45.1 ns       │ 47.06 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_floor_to_multiple        188.8 ns      │ 560.7 ns      │ 196.6 ns      │ 200.6 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple_const  49.39 ns      │ 206.4 ns      │ 60.33 ns      │ 64.46 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_floor_to_multiple_reuse  52.91 ns      │ 152.1 ns      │ 67.75 ns      │ 72.08 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_round_to_multiple        229.4 ns      │ 418.5 ns      │ 232.6 ns      │ 233.9 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_round_to_multiple_const  64.63 ns      │ 116.9 ns      │ 68.54 ns      │ 69.48 ns      │ 1000    │ 128000
   │  ╰─ unb_pow2_round_to_multiple_reuse  68.14 ns      │ 225.1 ns      │ 81.04 ns      │ 88.24 ns      │ 1000    │ 256000
   ├─ i32                                                │               │               │               │         │
   │  ├─ baseline_identity                 42.75 ns      │ 218.5 ns      │ 55.64 ns      │ 60.61 ns      │ 1000    │ 256000
   │  ├─ pow2_ceil_to_multiple             281 ns        │ 429.4 ns      │ 288.8 ns      │ 293 ns        │ 1000    │ 64000
   │  ├─ pow2_ceil_to_multiple_const       104.4 ns      │ 307.6 ns      │ 124 ns        │ 131.9 ns      │ 1000    │ 128000
   │  ├─ pow2_ceil_to_multiple_reuse       106.8 ns      │ 454.4 ns      │ 127.9 ns      │ 135.8 ns      │ 1000    │ 128000
   │  ├─ pow2_floor_to_multiple            262.2 ns      │ 387.2 ns      │ 268.5 ns      │ 273.9 ns      │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple_const      91.97 ns      │ 581 ns        │ 100.5 ns      │ 105.4 ns      │ 1000    │ 128000
   │  ├─ pow2_floor_to_multiple_reuse      98.22 ns      │ 509.9 ns      │ 109.9 ns      │ 119 ns        │ 1000    │ 128000
   │  ├─ pow2_round_to_multiple            282.6 ns      │ 463.8 ns      │ 290.4 ns      │ 293.6 ns      │ 1000    │ 64000
   │  ├─ pow2_round_to_multiple_const      116.9 ns      │ 363.8 ns      │ 129.4 ns      │ 130.5 ns      │ 1000    │ 64000
   │  ├─ pow2_round_to_multiple_reuse      158.3 ns      │ 395.8 ns      │ 178.6 ns      │ 189.3 ns      │ 1000    │ 128000
   │  ├─ std_div_mul                       1.324 µs      │ 3.562 µs      │ 1.337 µs      │ 1.345 µs      │ 1000    │ 8000
   │  ├─ std_div_mul_const                 121.6 ns      │ 547.4 ns      │ 134.9 ns      │ 142.5 ns      │ 1000    │ 128000
   │  ├─ std_div_mul_reuse                 1.262 µs      │ 3.012 µs      │ 1.274 µs      │ 1.282 µs      │ 1000    │ 8000
   │  ├─ unb_pow2_ceil_to_multiple         295.1 ns      │ 491.9 ns      │ 302.9 ns      │ 306.9 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_ceil_to_multiple_const   103.6 ns      │ 397.4 ns      │ 124 ns        │ 131.5 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   107.6 ns      │ 347.4 ns      │ 122.8 ns      │ 132.1 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_floor_to_multiple        276.3 ns      │ 2.749 µs      │ 281 ns        │ 290.2 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple_const  94.32 ns      │ 960.7 ns      │ 102.1 ns      │ 104.9 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_floor_to_multiple_reuse  98.22 ns      │ 603.6 ns      │ 116.9 ns      │ 125.4 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_round_to_multiple        296.6 ns      │ 627.9 ns      │ 302.9 ns      │ 306.2 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_round_to_multiple_const  130.2 ns      │ 476.3 ns      │ 143.5 ns      │ 154.1 ns      │ 1000    │ 128000
   │  ╰─ unb_pow2_round_to_multiple_reuse  143.5 ns      │ 677.9 ns      │ 149.7 ns      │ 152.7 ns      │ 1000    │ 64000
   ├─ i64                                                │               │               │               │         │
   │  ├─ baseline_identity                 80.25 ns      │ 353.6 ns      │ 113.8 ns      │ 119.2 ns      │ 1000    │ 128000
   │  ├─ pow2_ceil_to_multiple             499.7 ns      │ 2.418 µs      │ 512.2 ns      │ 521.5 ns      │ 1000    │ 16000
   │  ├─ pow2_ceil_to_multiple_const       196.6 ns      │ 591.9 ns      │ 224.7 ns      │ 237.6 ns      │ 1000    │ 64000
   │  ├─ pow2_ceil_to_multiple_reuse       204.4 ns      │ 1.023 µs      │ 252.9 ns      │ 268.9 ns      │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple            415.4 ns      │ 846.6 ns      │ 427.9 ns      │ 434.7 ns      │ 1000    │ 32000
   │  ├─ pow2_floor_to_multiple_const      187.2 ns      │ 670.1 ns      │ 232.6 ns      │ 248.2 ns      │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple_reuse      190.4 ns      │ 501.3 ns      │ 216.1 ns      │ 236.4 ns      │ 1000    │ 64000
   │  ├─ pow2_round_to_multiple            812.2 ns      │ 1.531 µs      │ 837.2 ns      │ 851.5 ns      │ 1000    │ 16000
   │  ├─ pow2_round_to_multiple_const      249.7 ns      │ 641.9 ns      │ 274.7 ns      │ 288.3 ns      │ 1000    │ 64000
   │  ├─ pow2_round_to_multiple_reuse      452.9 ns      │ 749.7 ns      │ 465.4 ns      │ 470.7 ns      │ 1000    │ 32000
   │  ├─ std_div_mul                       1.624 µs      │ 2.174 µs      │ 1.637 µs      │ 1.651 µs      │ 1000    │ 8000
   │  ├─ std_div_mul_const                 237.2 ns      │ 726.3 ns      │ 262.2 ns      │ 268.2 ns      │ 1000    │ 64000
   │  ├─ std_div_mul_reuse                 1.512 µs      │ 4.274 µs      │ 1.512 µs      │ 1.52 µs       │ 1000    │ 8000
   │  ├─ unb_pow2_ceil_to_multiple         518.5 ns      │ 5.256 µs      │ 537.2 ns      │ 553.8 ns      │ 1000    │ 32000
   │  ├─ unb_pow2_ceil_to_multiple_const   201.3 ns      │ 1.802 µs      │ 245.1 ns      │ 262.3 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   171.6 ns      │ 368.5 ns      │ 181 ns        │ 192.5 ns      │ 1000    │ 32000
   │  ├─ unb_pow2_floor_to_multiple        493.5 ns      │ 1.034 µs      │ 502.9 ns      │ 510 ns        │ 1000    │ 32000
   │  ├─ unb_pow2_floor_to_multiple_const  187.2 ns      │ 646.6 ns      │ 210.7 ns      │ 218 ns        │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple_reuse  190.4 ns      │ 604.4 ns      │ 213.8 ns      │ 221.4 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_round_to_multiple        981 ns        │ 2.674 µs      │ 1.006 µs      │ 1.03 µs       │ 1000    │ 16000
   │  ├─ unb_pow2_round_to_multiple_const  237.2 ns      │ 2.27 µs       │ 274.7 ns      │ 291.1 ns      │ 1000    │ 64000
   │  ╰─ unb_pow2_round_to_multiple_reuse  452.9 ns      │ 1.49 µs       │ 468.5 ns      │ 474.2 ns      │ 1000    │ 32000
   ├─ i128                                               │               │               │               │         │
   │  ├─ baseline_identity                 221.6 ns      │ 743.5 ns      │ 282.6 ns      │ 297 ns        │ 1000    │ 64000
   │  ├─ pow2_ceil_to_multiple             1.118 µs      │ 1.724 µs      │ 1.137 µs      │ 1.158 µs      │ 1000    │ 16000
   │  ├─ pow2_ceil_to_multiple_const       618.5 ns      │ 1.515 µs      │ 643.5 ns      │ 669.8 ns      │ 1000    │ 32000
   │  ├─ pow2_ceil_to_multiple_reuse       615.4 ns      │ 2.977 µs      │ 637.2 ns      │ 654.9 ns      │ 1000    │ 32000
   │  ├─ pow2_floor_to_multiple            843.5 ns      │ 1.843 µs      │ 874.7 ns      │ 890.3 ns      │ 1000    │ 16000
   │  ├─ pow2_floor_to_multiple_const      462.2 ns      │ 1.384 µs      │ 493.5 ns      │ 507.9 ns      │ 1000    │ 32000
   │  ├─ pow2_floor_to_multiple_reuse      571.6 ns      │ 1.656 µs      │ 646.6 ns      │ 681.8 ns      │ 1000    │ 32000
   │  ├─ pow2_round_to_multiple            2.324 µs      │ 6.224 µs      │ 2.387 µs      │ 2.408 µs      │ 1000    │ 8000
   │  ├─ pow2_round_to_multiple_const      724.7 ns      │ 1.506 µs      │ 737.2 ns      │ 747.4 ns      │ 1000    │ 16000
   │  ├─ pow2_round_to_multiple_reuse      918.5 ns      │ 1.918 µs      │ 937.2 ns      │ 948 ns        │ 1000    │ 16000
   │  ├─ std_div_mul                       9.999 µs      │ 20.79 µs      │ 10.09 µs      │ 10.13 µs      │ 1000    │ 1000
   │  ├─ std_div_mul_const                 656 ns        │ 2.274 µs      │ 668.5 ns      │ 689 ns        │ 1000    │ 16000
   │  ├─ std_div_mul_reuse                 9.249 µs      │ 24.04 µs      │ 9.349 µs      │ 9.415 µs      │ 1000    │ 2000
   │  ├─ unb_pow2_ceil_to_multiple         1.143 µs      │ 2.012 µs      │ 1.162 µs      │ 1.172 µs      │ 1000    │ 16000
   │  ├─ unb_pow2_ceil_to_multiple_const   615.4 ns      │ 2.127 µs      │ 643.5 ns      │ 666.2 ns      │ 1000    │ 32000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   612.2 ns      │ 1.459 µs      │ 649.7 ns      │ 674.6 ns      │ 1000    │ 32000
   │  ├─ unb_pow2_floor_to_multiple        831 ns        │ 10.38 µs      │ 874.7 ns      │ 905.3 ns      │ 1000    │ 16000
   │  ├─ unb_pow2_floor_to_multiple_const  459.1 ns      │ 1.118 µs      │ 493.5 ns      │ 506.6 ns      │ 1000    │ 32000
   │  ├─ unb_pow2_floor_to_multiple_reuse  565.4 ns      │ 1.843 µs      │ 627.9 ns      │ 674.1 ns      │ 1000    │ 32000
   │  ├─ unb_pow2_round_to_multiple        2.349 µs      │ 4.637 µs      │ 2.374 µs      │ 2.401 µs      │ 1000    │ 8000
   │  ├─ unb_pow2_round_to_multiple_const  712.2 ns      │ 1.474 µs      │ 731 ns        │ 741 ns        │ 1000    │ 16000
   │  ╰─ unb_pow2_round_to_multiple_reuse  918.5 ns      │ 1.924 µs      │ 937.2 ns      │ 948.8 ns      │ 1000    │ 16000
   ├─ u8                                                 │               │               │               │         │
   │  ├─ baseline_identity                 12.48 ns      │ 63.85 ns      │ 13.55 ns      │ 15 ns         │ 1000    │ 1024000
   │  ├─ pow2_ceil_to_multiple             99 ns         │ 1.258 µs      │ 100.5 ns      │ 103 ns        │ 1000    │ 128000
   │  ├─ pow2_ceil_to_multiple_const       26.93 ns      │ 256 ns        │ 34.55 ns      │ 38.36 ns      │ 1000    │ 512000
   │  ├─ pow2_ceil_to_multiple_reuse       28.5 ns       │ 207.4 ns      │ 33.57 ns      │ 36.11 ns      │ 1000    │ 512000
   │  ├─ pow2_floor_to_multiple            94.32 ns      │ 323.2 ns      │ 96.66 ns      │ 99.48 ns      │ 1000    │ 128000
   │  ├─ pow2_floor_to_multiple_const      24.59 ns      │ 128.3 ns      │ 27.52 ns      │ 32.06 ns      │ 1000    │ 512000
   │  ├─ pow2_floor_to_multiple_reuse      26.54 ns      │ 74.39 ns      │ 28.11 ns      │ 29.23 ns      │ 1000    │ 512000
   │  ├─ pow2_round_to_multiple            113 ns        │ 355.2 ns      │ 115.4 ns      │ 116.3 ns      │ 1000    │ 128000
   │  ├─ pow2_round_to_multiple_const      26.74 ns      │ 332.9 ns      │ 31.82 ns      │ 34.35 ns      │ 1000    │ 512000
   │  ├─ pow2_round_to_multiple_reuse      28.11 ns      │ 101.9 ns      │ 31.04 ns      │ 34.43 ns      │ 1000    │ 512000
   │  ├─ std_div_mul                       862.2 ns      │ 2.187 µs      │ 887.2 ns      │ 892 ns        │ 1000    │ 8000
   │  ├─ std_div_mul_const                 24.79 ns      │ 147 ns        │ 27.13 ns      │ 30.3 ns       │ 1000    │ 512000
   │  ├─ std_div_mul_reuse                 837.2 ns      │ 1.562 µs      │ 849.7 ns      │ 855.9 ns      │ 1000    │ 16000
   │  ├─ unb_pow2_ceil_to_multiple         102.1 ns      │ 204.4 ns      │ 105.2 ns      │ 106.4 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_ceil_to_multiple_const   26.54 ns      │ 973.4 ns      │ 33.57 ns      │ 37.3 ns       │ 1000    │ 512000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   29.28 ns      │ 113.4 ns      │ 36.31 ns      │ 39.28 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_floor_to_multiple        98.22 ns      │ 225.5 ns      │ 100.9 ns      │ 108.8 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_floor_to_multiple_const  24.98 ns      │ 89.63 ns      │ 29.47 ns      │ 31.91 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_floor_to_multiple_reuse  26.93 ns      │ 195.2 ns      │ 32.21 ns      │ 35.47 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_round_to_multiple        118.5 ns      │ 253.6 ns      │ 120.8 ns      │ 121.8 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_round_to_multiple_const  26.54 ns      │ 122.6 ns      │ 33.38 ns      │ 36.18 ns      │ 1000    │ 512000
   │  ╰─ unb_pow2_round_to_multiple_reuse  27.91 ns      │ 123 ns        │ 32.21 ns      │ 35.15 ns      │ 1000    │ 512000
   ├─ u16                                                │               │               │               │         │
   │  ├─ baseline_identity                 22.25 ns      │ 83.38 ns      │ 33.57 ns      │ 33.59 ns      │ 1000    │ 512000
   │  ├─ pow2_ceil_to_multiple             193.5 ns      │ 510.7 ns      │ 198.2 ns      │ 200.4 ns      │ 1000    │ 64000
   │  ├─ pow2_ceil_to_multiple_const       54.47 ns      │ 144.7 ns      │ 61.5 ns       │ 64.19 ns      │ 1000    │ 256000
   │  ├─ pow2_ceil_to_multiple_reuse       57.21 ns      │ 599 ns        │ 70.49 ns      │ 76.01 ns      │ 1000    │ 256000
   │  ├─ pow2_floor_to_multiple            190.4 ns      │ 554.4 ns      │ 260.7 ns      │ 234.9 ns      │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple_const      47.05 ns      │ 572.8 ns      │ 58.77 ns      │ 62.82 ns      │ 1000    │ 256000
   │  ├─ pow2_floor_to_multiple_reuse      49.39 ns      │ 849 ns        │ 61.89 ns      │ 69.4 ns       │ 1000    │ 256000
   │  ├─ pow2_round_to_multiple            206 ns        │ 618.5 ns      │ 212.2 ns      │ 233.2 ns      │ 1000    │ 64000
   │  ├─ pow2_round_to_multiple_const      53.3 ns       │ 187.2 ns      │ 65.02 ns      │ 70.81 ns      │ 1000    │ 256000
   │  ├─ pow2_round_to_multiple_reuse      55.64 ns      │ 475.9 ns      │ 67.75 ns      │ 72.38 ns      │ 1000    │ 256000
   │  ├─ std_div_mul                       868.5 ns      │ 1.287 µs      │ 881 ns        │ 884.9 ns      │ 1000    │ 16000
   │  ├─ std_div_mul_const                 48.61 ns      │ 171.2 ns      │ 60.33 ns      │ 63.58 ns      │ 1000    │ 256000
   │  ├─ std_div_mul_reuse                 837.2 ns      │ 9.806 µs      │ 856 ns        │ 875.3 ns      │ 1000    │ 16000
   │  ├─ unb_pow2_ceil_to_multiple         199.7 ns      │ 2.559 µs      │ 202.9 ns      │ 209.5 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_ceil_to_multiple_const   54.86 ns      │ 561.8 ns      │ 66.19 ns      │ 70.07 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   56.43 ns      │ 281.4 ns      │ 67.36 ns      │ 71.59 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_floor_to_multiple        190.4 ns      │ 271.6 ns      │ 196.6 ns      │ 196.9 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple_const  49 ns         │ 187.2 ns      │ 60.14 ns      │ 65.47 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_floor_to_multiple_reuse  53.69 ns      │ 256.4 ns      │ 66.19 ns      │ 70.66 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_round_to_multiple        210.7 ns      │ 385.7 ns      │ 215.4 ns      │ 216.3 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_round_to_multiple_const  53.3 ns       │ 182.2 ns      │ 64.63 ns      │ 68.72 ns      │ 1000    │ 256000
   │  ╰─ unb_pow2_round_to_multiple_reuse  57.6 ns       │ 235.3 ns      │ 71.27 ns      │ 75.57 ns      │ 1000    │ 256000
   ├─ u32                                                │               │               │               │         │
   │  ├─ baseline_identity                 42.36 ns      │ 252.5 ns      │ 54.47 ns      │ 57.87 ns      │ 1000    │ 256000
   │  ├─ pow2_ceil_to_multiple             281 ns        │ 537.2 ns      │ 287.2 ns      │ 291.3 ns      │ 1000    │ 64000
   │  ├─ pow2_ceil_to_multiple_const       104.4 ns      │ 289.6 ns      │ 117.7 ns      │ 125.7 ns      │ 1000    │ 128000
   │  ├─ pow2_ceil_to_multiple_reuse       106.8 ns      │ 1.442 µs      │ 120.1 ns      │ 129.9 ns      │ 1000    │ 128000
   │  ├─ pow2_floor_to_multiple            263.8 ns      │ 576.3 ns      │ 268.5 ns      │ 272.5 ns      │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple_const      94.32 ns      │ 310.7 ns      │ 102.1 ns      │ 109.8 ns      │ 1000    │ 128000
   │  ├─ pow2_floor_to_multiple_reuse      97.44 ns      │ 256 ns        │ 109.1 ns      │ 116.5 ns      │ 1000    │ 128000
   │  ├─ pow2_round_to_multiple            212.2 ns      │ 499.7 ns      │ 218.5 ns      │ 221.6 ns      │ 1000    │ 64000
   │  ├─ pow2_round_to_multiple_const      101.3 ns      │ 327.1 ns      │ 120.8 ns      │ 129.8 ns      │ 1000    │ 128000
   │  ├─ pow2_round_to_multiple_reuse      107.6 ns      │ 358.3 ns      │ 127.1 ns      │ 137 ns        │ 1000    │ 128000
   │  ├─ std_div_mul                       1.324 µs      │ 2.437 µs      │ 1.349 µs      │ 1.358 µs      │ 1000    │ 8000
   │  ├─ std_div_mul_const                 93.54 ns      │ 237.2 ns      │ 102.1 ns      │ 106.9 ns      │ 1000    │ 128000
   │  ├─ std_div_mul_reuse                 1.287 µs      │ 3.174 µs      │ 1.299 µs      │ 1.327 µs      │ 1000    │ 8000
   │  ├─ unb_pow2_ceil_to_multiple         293.5 ns      │ 2.624 µs      │ 299.7 ns      │ 307.1 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_ceil_to_multiple_const   103.6 ns      │ 537.2 ns      │ 117.7 ns      │ 126.9 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   99 ns         │ 418.5 ns      │ 123.6 ns      │ 129.7 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_floor_to_multiple        274.7 ns      │ 893.5 ns      │ 281 ns        │ 284.6 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple_const  91.97 ns      │ 335.7 ns      │ 107.6 ns      │ 116.1 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_floor_to_multiple_reuse  98.22 ns      │ 278.6 ns      │ 117.7 ns      │ 126.6 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_round_to_multiple        224.7 ns      │ 329.4 ns      │ 227.9 ns      │ 231 ns        │ 1000    │ 64000
   │  ├─ unb_pow2_round_to_multiple_const  101.3 ns      │ 273.2 ns      │ 120.8 ns      │ 127.5 ns      │ 1000    │ 128000
   │  ╰─ unb_pow2_round_to_multiple_reuse  103.6 ns      │ 1.483 µs      │ 129.4 ns      │ 142 ns        │ 1000    │ 128000
   ├─ u64                                                │               │               │               │         │
   │  ├─ baseline_identity                 81.04 ns      │ 385.7 ns      │ 118.5 ns      │ 124.3 ns      │ 1000    │ 128000
   │  ├─ pow2_ceil_to_multiple             499.7 ns      │ 2.371 µs      │ 509.1 ns      │ 517.3 ns      │ 1000    │ 32000
   │  ├─ pow2_ceil_to_multiple_const       201.3 ns      │ 584.1 ns      │ 224.7 ns      │ 233.8 ns      │ 1000    │ 64000
   │  ├─ pow2_ceil_to_multiple_reuse       202.9 ns      │ 1.065 µs      │ 251.3 ns      │ 269.1 ns      │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple            415.4 ns      │ 971.6 ns      │ 431 ns        │ 437.4 ns      │ 1000    │ 32000
   │  ├─ pow2_floor_to_multiple_const      187.2 ns      │ 893.5 ns      │ 209.1 ns      │ 220.5 ns      │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple_reuse      190.4 ns      │ 587.2 ns      │ 231 ns        │ 248.4 ns      │ 1000    │ 64000
   │  ├─ pow2_round_to_multiple            527.9 ns      │ 946.6 ns      │ 540.4 ns      │ 547.1 ns      │ 1000    │ 32000
   │  ├─ pow2_round_to_multiple_const      195.1 ns      │ 826.3 ns      │ 220.1 ns      │ 244.5 ns      │ 1000    │ 64000
   │  ├─ pow2_round_to_multiple_reuse      206 ns        │ 1.856 µs      │ 235.7 ns      │ 242.6 ns      │ 1000    │ 64000
   │  ├─ std_div_mul                       1.637 µs      │ 20.44 µs      │ 1.649 µs      │ 1.71 µs       │ 1000    │ 8000
   │  ├─ std_div_mul_const                 185.7 ns      │ 763.8 ns      │ 218.5 ns      │ 233.4 ns      │ 1000    │ 64000
   │  ├─ std_div_mul_reuse                 1.512 µs      │ 2.787 µs      │ 1.562 µs      │ 1.56 µs       │ 1000    │ 8000
   │  ├─ unb_pow2_ceil_to_multiple         524.7 ns      │ 934.1 ns      │ 537.2 ns      │ 543.9 ns      │ 1000    │ 32000
   │  ├─ unb_pow2_ceil_to_multiple_const   201.3 ns      │ 427.9 ns      │ 229.4 ns      │ 231.2 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   204.4 ns      │ 720.1 ns      │ 238.8 ns      │ 252.4 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple        490.4 ns      │ 1.002 µs      │ 506 ns        │ 517 ns        │ 1000    │ 32000
   │  ├─ unb_pow2_floor_to_multiple_const  187.2 ns      │ 1.256 µs      │ 223.2 ns      │ 242.3 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple_reuse  193.5 ns      │ 660.7 ns      │ 220.1 ns      │ 232.1 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_round_to_multiple        559.1 ns      │ 1.259 µs      │ 574.7 ns      │ 582.8 ns      │ 1000    │ 32000
   │  ├─ unb_pow2_round_to_multiple_const  196.6 ns      │ 631 ns        │ 223.2 ns      │ 229.8 ns      │ 1000    │ 64000
   │  ╰─ unb_pow2_round_to_multiple_reuse  204.4 ns      │ 690.4 ns      │ 245.1 ns      │ 257.1 ns      │ 1000    │ 64000
   ╰─ u128                                               │               │               │               │         │
      ├─ baseline_identity                 221.6 ns      │ 743.5 ns      │ 288.8 ns      │ 298.4 ns      │ 1000    │ 64000
      ├─ pow2_ceil_to_multiple             1.112 µs      │ 2.912 µs      │ 1.162 µs      │ 1.189 µs      │ 1000    │ 16000
      ├─ pow2_ceil_to_multiple_const       574.7 ns      │ 1.787 µs      │ 612.2 ns      │ 685.3 ns      │ 1000    │ 16000
      ├─ pow2_ceil_to_multiple_reuse       621.6 ns      │ 1.89 µs       │ 649.7 ns      │ 689.3 ns      │ 1000    │ 32000
      ├─ pow2_floor_to_multiple            849.7 ns      │ 3.862 µs      │ 874.7 ns      │ 886.3 ns      │ 1000    │ 16000
      ├─ pow2_floor_to_multiple_const      465.4 ns      │ 1.246 µs      │ 496.6 ns      │ 519.6 ns      │ 1000    │ 32000
      ├─ pow2_floor_to_multiple_reuse      571.6 ns      │ 5.452 µs      │ 615.4 ns      │ 662.8 ns      │ 1000    │ 32000
      ├─ pow2_round_to_multiple            1.862 µs      │ 7.899 µs      │ 1.887 µs      │ 1.915 µs      │ 1000    │ 8000
      ├─ pow2_round_to_multiple_const      615.4 ns      │ 3.977 µs      │ 637.2 ns      │ 670.3 ns      │ 1000    │ 32000
      ├─ pow2_round_to_multiple_reuse      609.1 ns      │ 5.224 µs      │ 659.1 ns      │ 730.2 ns      │ 1000    │ 32000
      ├─ std_div_mul                       4.299 µs      │ 6.049 µs      │ 4.399 µs      │ 4.444 µs      │ 1000    │ 4000
      ├─ std_div_mul_const                 465.4 ns      │ 937.2 ns      │ 496.6 ns      │ 515.7 ns      │ 1000    │ 32000
      ├─ std_div_mul_reuse                 4.249 µs      │ 7.349 µs      │ 4.299 µs      │ 4.338 µs      │ 1000    │ 4000
      ├─ unb_pow2_ceil_to_multiple         1.131 µs      │ 2.537 µs      │ 1.143 µs      │ 1.175 µs      │ 1000    │ 16000
      ├─ unb_pow2_ceil_to_multiple_const   615.4 ns      │ 1.477 µs      │ 656 ns        │ 722.9 ns      │ 1000    │ 32000
      ├─ unb_pow2_ceil_to_multiple_reuse   618.5 ns      │ 1.49 µs       │ 640.4 ns      │ 665.8 ns      │ 1000    │ 32000
      ├─ unb_pow2_floor_to_multiple        824.7 ns      │ 1.281 µs      │ 843.5 ns      │ 859.7 ns      │ 1000    │ 16000
      ├─ unb_pow2_floor_to_multiple_const  465.4 ns      │ 1.087 µs      │ 499.7 ns      │ 511.9 ns      │ 1000    │ 32000
      ├─ unb_pow2_floor_to_multiple_reuse  571.6 ns      │ 1.487 µs      │ 624.7 ns      │ 659.8 ns      │ 1000    │ 32000
      ├─ unb_pow2_round_to_multiple        1.912 µs      │ 21.03 µs      │ 1.937 µs      │ 1.978 µs      │ 1000    │ 8000
      ├─ unb_pow2_round_to_multiple_const  562.2 ns      │ 5.562 µs      │ 581 ns        │ 603.2 ns      │ 1000    │ 16000
      ╰─ unb_pow2_round_to_multiple_reuse  612.2 ns      │ 2.256 µs      │ 640.4 ns      │ 684.1 ns      │ 1000    │ 32000


```
