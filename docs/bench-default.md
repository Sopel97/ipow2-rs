# `public` bench with default target
```
public                                     fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ div                                                   │               │               │               │         │
│  ├─ i8                                                 │               │               │               │         │
│  │  ├─ baseline_identity                 10.14 ns      │ 66.39 ns      │ 12.09 ns      │ 13.67 ns      │ 1000    │ 512000
│  │  ├─ pow2_div                          365.4 ns      │ 656 ns        │ 368.5 ns      │ 370.7 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_ceil                     296.6 ns      │ 370.1 ns      │ 298.2 ns      │ 299 ns        │ 1000    │ 64000
│  │  ├─ pow2_div_ceil_const               34.16 ns      │ 55.64 ns      │ 37.29 ns      │ 37.63 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_ceil_reuse               44.32 ns      │ 85.33 ns      │ 47.44 ns      │ 47.3 ns       │ 1000    │ 256000
│  │  ├─ pow2_div_const                    50.96 ns      │ 78.3 ns       │ 52.13 ns      │ 52.92 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_floor                    199.7 ns      │ 266.9 ns      │ 201.3 ns      │ 202.4 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_floor_const              33.38 ns      │ 64.43 ns      │ 37.29 ns      │ 37.84 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_floor_reuse              36.11 ns      │ 133.5 ns      │ 40.61 ns      │ 41.92 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_reuse                    53.3 ns       │ 499.3 ns      │ 54.47 ns      │ 56.59 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_round                    374.7 ns      │ 3.765 µs      │ 381 ns        │ 391.7 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_round_const              49.39 ns      │ 627.9 ns      │ 53.3 ns       │ 54.93 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_round_reuse              60.33 ns      │ 92.36 ns      │ 67.36 ns      │ 67.25 ns      │ 1000    │ 256000
│  │  ├─ std_div                           856 ns        │ 1.543 µs      │ 874.7 ns      │ 878.6 ns      │ 1000    │ 16000
│  │  ├─ std_div_const                     37.68 ns      │ 95.49 ns      │ 40.02 ns      │ 40.38 ns      │ 1000    │ 256000
│  │  ├─ std_div_reuse                     849.7 ns      │ 1.974 µs      │ 856 ns        │ 864.8 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_div                      371.6 ns      │ 509.1 ns      │ 374.7 ns      │ 375.2 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_ceil                 302.9 ns      │ 470.1 ns      │ 306 ns        │ 307.9 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil_const           39.24 ns      │ 74.79 ns      │ 41.58 ns      │ 42.05 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_ceil_reuse           44.71 ns      │ 72.83 ns      │ 48.22 ns      │ 48.47 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_const                49.79 ns      │ 100.5 ns      │ 52.13 ns      │ 52.46 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_floor                202.9 ns      │ 357.6 ns      │ 206 ns        │ 208.7 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor_const          32.99 ns      │ 116.3 ns      │ 37.87 ns      │ 39.48 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_floor_reuse          37.09 ns      │ 236.5 ns      │ 41.58 ns      │ 42.4 ns       │ 1000    │ 512000
│  │  ├─ unb_pow2_div_reuse                52.52 ns      │ 96.27 ns      │ 58.38 ns      │ 58.16 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_round                381 ns        │ 659.1 ns      │ 384.1 ns      │ 387 ns        │ 1000    │ 32000
│  │  ├─ unb_pow2_div_round_const          50.57 ns      │ 94.32 ns      │ 56.04 ns      │ 56.11 ns      │ 1000    │ 256000
│  │  ╰─ unb_pow2_div_round_reuse          57.6 ns       │ 158.7 ns      │ 61.89 ns      │ 61.99 ns      │ 1000    │ 256000
│  ├─ i16                                                │               │               │               │         │
│  │  ├─ baseline_identity                 24.79 ns      │ 69.51 ns      │ 27.32 ns      │ 29.5 ns       │ 1000    │ 512000
│  │  ├─ pow2_div                          465.4 ns      │ 684.1 ns      │ 493.5 ns      │ 499.9 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_ceil                     409.1 ns      │ 549.7 ns      │ 415.4 ns      │ 417.4 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_ceil_const               62.29 ns      │ 145.1 ns      │ 70.88 ns      │ 72.26 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_ceil_reuse               62.29 ns      │ 126.3 ns      │ 65.41 ns      │ 67.11 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_const                    74.79 ns      │ 174 ns        │ 82.6 ns       │ 82.66 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_floor                    234.1 ns      │ 306 ns        │ 238.8 ns      │ 238.8 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_floor_const              49.79 ns      │ 585.3 ns      │ 53.69 ns      │ 55.13 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_floor_reuse              55.64 ns      │ 132.2 ns      │ 61.11 ns      │ 62.3 ns       │ 1000    │ 256000
│  │  ├─ pow2_div_reuse                    76.35 ns      │ 198.2 ns      │ 81.82 ns      │ 83.71 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_round                    565.4 ns      │ 912.2 ns      │ 568.5 ns      │ 572.9 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_round_const              65.41 ns      │ 111.5 ns      │ 71.66 ns      │ 72.28 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_round_reuse              88.85 ns      │ 129.4 ns      │ 95.1 ns       │ 95.94 ns      │ 1000    │ 128000
│  │  ├─ std_div                           874.7 ns      │ 1.162 µs      │ 887.2 ns      │ 892.3 ns      │ 1000    │ 16000
│  │  ├─ std_div_const                     62.68 ns      │ 155.6 ns      │ 68.93 ns      │ 70.39 ns      │ 1000    │ 256000
│  │  ├─ std_div_reuse                     2.499 µs      │ 3.599 µs      │ 2.549 µs      │ 2.555 µs      │ 1000    │ 4000
│  │  ├─ unb_pow2_div                      606 ns        │ 896.6 ns      │ 615.4 ns      │ 619.4 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_ceil                 412.2 ns      │ 718.5 ns      │ 418.5 ns      │ 420.3 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_ceil_const           67.75 ns      │ 205.6 ns      │ 75.18 ns      │ 77.53 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_ceil_reuse           72.83 ns      │ 218.1 ns      │ 84.55 ns      │ 87.68 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_const                70.1 ns       │ 188 ns        │ 73.22 ns      │ 75.7 ns       │ 1000    │ 128000
│  │  ├─ unb_pow2_div_floor                231 ns        │ 379.4 ns      │ 234.1 ns      │ 235.6 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor_const          50.18 ns      │ 170.1 ns      │ 54.08 ns      │ 55.18 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_floor_reuse          53.3 ns       │ 156 ns        │ 60.72 ns      │ 62.22 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_reuse                77.91 ns      │ 202.1 ns      │ 86.5 ns       │ 87.09 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_round                584.1 ns      │ 727.9 ns      │ 590.4 ns      │ 592.3 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_round_const          77.52 ns      │ 146.6 ns      │ 84.55 ns      │ 84.73 ns      │ 1000    │ 256000
│  │  ╰─ unb_pow2_div_round_reuse          79.47 ns      │ 249 ns        │ 84.16 ns      │ 94.8 ns       │ 1000    │ 128000
│  ├─ i32                                                │               │               │               │         │
│  │  ├─ baseline_identity                 43.54 ns      │ 137.6 ns      │ 50.18 ns      │ 50.91 ns      │ 1000    │ 256000
│  │  ├─ pow2_div                          440.4 ns      │ 677.9 ns      │ 465.4 ns      │ 470.1 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_ceil                     509.1 ns      │ 652.9 ns      │ 515.4 ns      │ 519.3 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_ceil_const               129.4 ns      │ 248.2 ns      │ 140.4 ns      │ 141.9 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_ceil_reuse               139.6 ns      │ 306.8 ns      │ 155.2 ns      │ 156.7 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_const                    146.6 ns      │ 220.1 ns      │ 151.3 ns      │ 153.5 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_floor                    291.9 ns      │ 432.6 ns      │ 298.2 ns      │ 300 ns        │ 1000    │ 64000
│  │  ├─ pow2_div_floor_const              93.54 ns      │ 280.2 ns      │ 102.9 ns      │ 104.4 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_floor_reuse              109.1 ns      │ 358.3 ns      │ 127.1 ns      │ 129.4 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_reuse                    160.7 ns      │ 377.9 ns      │ 168.5 ns      │ 173.7 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_round                    731 ns        │ 1.293 µs      │ 743.5 ns      │ 753.8 ns      │ 1000    │ 16000
│  │  ├─ pow2_div_round_const              160.7 ns      │ 481 ns        │ 166.9 ns      │ 172.5 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_round_reuse              193.5 ns      │ 495.1 ns      │ 212.2 ns      │ 215.4 ns      │ 1000    │ 64000
│  │  ├─ std_div                           1.312 µs      │ 5.062 µs      │ 1.324 µs      │ 1.348 µs      │ 1000    │ 8000
│  │  ├─ std_div_const                     133.3 ns      │ 330.2 ns      │ 147.4 ns      │ 150.4 ns      │ 1000    │ 128000
│  │  ├─ std_div_reuse                     1.262 µs      │ 2.887 µs      │ 1.274 µs      │ 1.283 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div                      468.5 ns      │ 937.2 ns      │ 490.4 ns      │ 501.8 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_ceil                 527.9 ns      │ 1.152 µs      │ 534.1 ns      │ 545.1 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_ceil_const           115.4 ns      │ 320.1 ns      │ 123.2 ns      │ 127.7 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil_reuse           135.7 ns      │ 341.1 ns      │ 163 ns        │ 166.4 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_const                159.9 ns      │ 450.5 ns      │ 181.8 ns      │ 188 ns        │ 1000    │ 128000
│  │  ├─ unb_pow2_div_floor                295.1 ns      │ 460.7 ns      │ 299.7 ns      │ 301.9 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor_const          92.75 ns      │ 180.2 ns      │ 106 ns        │ 108.4 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_floor_reuse          108.3 ns      │ 263.8 ns      │ 121.6 ns      │ 123.5 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_reuse                166.9 ns      │ 340.4 ns      │ 173.2 ns      │ 175 ns        │ 1000    │ 64000
│  │  ├─ unb_pow2_div_round                762.2 ns      │ 1.187 µs      │ 774.7 ns      │ 777.1 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_div_round_const          160.7 ns      │ 235.7 ns      │ 168.5 ns      │ 169 ns        │ 1000    │ 64000
│  │  ╰─ unb_pow2_div_round_reuse          199.7 ns      │ 313.8 ns      │ 207.6 ns      │ 209.5 ns      │ 1000    │ 64000
│  ├─ i64                                                │               │               │               │         │
│  │  ├─ baseline_identity                 82.6 ns       │ 266.1 ns      │ 99.79 ns      │ 104 ns        │ 1000    │ 128000
│  │  ├─ pow2_div                          471.6 ns      │ 1.115 µs      │ 490.4 ns      │ 504.6 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_ceil                     643.5 ns      │ 1.462 µs      │ 656 ns        │ 676.3 ns      │ 1000    │ 16000
│  │  ├─ pow2_div_ceil_const               362.2 ns      │ 784.1 ns      │ 368.5 ns      │ 375 ns        │ 1000    │ 32000
│  │  ├─ pow2_div_ceil_reuse               443.5 ns      │ 1.431 µs      │ 456 ns        │ 470.3 ns      │ 1000    │ 16000
│  │  ├─ pow2_div_const                    434.1 ns      │ 990.4 ns      │ 449.7 ns      │ 460.4 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_floor                    321.6 ns      │ 831 ns        │ 346.6 ns      │ 352.7 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_floor_const              271.6 ns      │ 635.7 ns      │ 288.8 ns      │ 296 ns        │ 1000    │ 64000
│  │  ├─ pow2_div_floor_reuse              246.6 ns      │ 560.7 ns      │ 274.7 ns      │ 280 ns        │ 1000    │ 64000
│  │  ├─ pow2_div_reuse                    509.1 ns      │ 1.615 µs      │ 524.7 ns      │ 538 ns        │ 1000    │ 32000
│  │  ├─ pow2_div_round                    912.2 ns      │ 1.787 µs      │ 937.2 ns      │ 956.5 ns      │ 1000    │ 16000
│  │  ├─ pow2_div_round_const              484.1 ns      │ 796.6 ns      │ 490.4 ns      │ 493 ns        │ 1000    │ 32000
│  │  ├─ pow2_div_round_reuse              699.7 ns      │ 1.006 µs      │ 712.2 ns      │ 712.5 ns      │ 1000    │ 16000
│  │  ├─ std_div                           1.637 µs      │ 2.274 µs      │ 1.662 µs      │ 1.667 µs      │ 1000    │ 8000
│  │  ├─ std_div_const                     390.4 ns      │ 537.2 ns      │ 406 ns        │ 406.4 ns      │ 1000    │ 32000
│  │  ├─ std_div_reuse                     1.537 µs      │ 2.074 µs      │ 1.537 µs      │ 1.55 µs       │ 1000    │ 8000
│  │  ├─ unb_pow2_div                      499.7 ns      │ 1.19 µs       │ 515.4 ns      │ 522.6 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_ceil                 668.5 ns      │ 968.5 ns      │ 674.7 ns      │ 679.3 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_div_ceil_const           359.1 ns      │ 712.2 ns      │ 371.6 ns      │ 372.6 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_ceil_reuse           471.6 ns      │ 740.4 ns      │ 481 ns        │ 484.1 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_const                440.4 ns      │ 809.1 ns      │ 446.6 ns      │ 449.1 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_floor                321.6 ns      │ 524.7 ns      │ 332.6 ns      │ 334.8 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_floor_const          271.6 ns      │ 621.6 ns      │ 287.2 ns      │ 294.9 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor_reuse          229.4 ns      │ 2.543 µs      │ 265.4 ns      │ 284.1 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_reuse                512.2 ns      │ 815.4 ns      │ 534.1 ns      │ 543.6 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_round                968.5 ns      │ 1.624 µs      │ 987.2 ns      │ 1.029 µs      │ 1000    │ 16000
│  │  ├─ unb_pow2_div_round_const          487.2 ns      │ 702.9 ns      │ 496.6 ns      │ 501.1 ns      │ 1000    │ 32000
│  │  ╰─ unb_pow2_div_round_reuse          718.5 ns      │ 1.443 µs      │ 724.7 ns      │ 731.2 ns      │ 1000    │ 16000
│  ├─ i128                                               │               │               │               │         │
│  │  ├─ baseline_identity                 201.3 ns      │ 666.9 ns      │ 254.4 ns      │ 256.4 ns      │ 1000    │ 64000
│  │  ├─ pow2_div                          1.412 µs      │ 2.287 µs      │ 1.474 µs      │ 1.479 µs      │ 1000    │ 8000
│  │  ├─ pow2_div_ceil                     1.799 µs      │ 3.124 µs      │ 1.824 µs      │ 1.827 µs      │ 1000    │ 8000
│  │  ├─ pow2_div_ceil_const               1.037 µs      │ 1.512 µs      │ 1.056 µs      │ 1.067 µs      │ 1000    │ 16000
│  │  ├─ pow2_div_ceil_reuse               1.437 µs      │ 2.137 µs      │ 1.574 µs      │ 1.568 µs      │ 1000    │ 8000
│  │  ├─ pow2_div_const                    1.462 µs      │ 2.574 µs      │ 1.474 µs      │ 1.484 µs      │ 1000    │ 8000
│  │  ├─ pow2_div_floor                    1.118 µs      │ 1.681 µs      │ 1.137 µs      │ 1.143 µs      │ 1000    │ 16000
│  │  ├─ pow2_div_floor_const              849.7 ns      │ 1.399 µs      │ 868.5 ns      │ 873.1 ns      │ 1000    │ 16000
│  │  ├─ pow2_div_floor_reuse              1.212 µs      │ 2.899 µs      │ 1.237 µs      │ 1.258 µs      │ 1000    │ 8000
│  │  ├─ pow2_div_reuse                    2.024 µs      │ 3.849 µs      │ 2.099 µs      │ 2.111 µs      │ 1000    │ 8000
│  │  ├─ pow2_div_round                    2.824 µs      │ 3.874 µs      │ 2.849 µs      │ 2.851 µs      │ 1000    │ 4000
│  │  ├─ pow2_div_round_const              1.687 µs      │ 2.624 µs      │ 1.712 µs      │ 1.714 µs      │ 1000    │ 8000
│  │  ├─ pow2_div_round_reuse              2.524 µs      │ 3.087 µs      │ 2.549 µs      │ 2.559 µs      │ 1000    │ 8000
│  │  ├─ std_div                           9.199 µs      │ 20.69 µs      │ 9.299 µs      │ 9.345 µs      │ 1000    │ 1000
│  │  ├─ std_div_const                     1.043 µs      │ 1.518 µs      │ 1.056 µs      │ 1.065 µs      │ 1000    │ 16000
│  │  ├─ std_div_reuse                     8.049 µs      │ 13.04 µs      │ 8.149 µs      │ 8.165 µs      │ 1000    │ 2000
│  │  ├─ unb_pow2_div                      1.474 µs      │ 2.687 µs      │ 1.499 µs      │ 1.544 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div_ceil                 1.812 µs      │ 3.199 µs      │ 1.849 µs      │ 1.856 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div_ceil_const           1.043 µs      │ 1.462 µs      │ 1.062 µs      │ 1.07 µs       │ 1000    │ 16000
│  │  ├─ unb_pow2_div_ceil_reuse           1.437 µs      │ 2.187 µs      │ 1.462 µs      │ 1.466 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div_const                1.474 µs      │ 3.099 µs      │ 1.487 µs      │ 1.503 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div_floor                1.099 µs      │ 1.706 µs      │ 1.112 µs      │ 1.12 µs       │ 1000    │ 16000
│  │  ├─ unb_pow2_div_floor_const          843.5 ns      │ 1.149 µs      │ 856 ns        │ 858.1 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_div_floor_reuse          1.143 µs      │ 1.743 µs      │ 1.156 µs      │ 1.162 µs      │ 1000    │ 16000
│  │  ├─ unb_pow2_div_reuse                2.037 µs      │ 2.649 µs      │ 2.087 µs      │ 2.092 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div_round                2.824 µs      │ 3.949 µs      │ 2.849 µs      │ 2.854 µs      │ 1000    │ 4000
│  │  ├─ unb_pow2_div_round_const          1.687 µs      │ 2.387 µs      │ 1.699 µs      │ 1.71 µs       │ 1000    │ 8000
│  │  ╰─ unb_pow2_div_round_reuse          2.499 µs      │ 4.774 µs      │ 2.524 µs      │ 2.536 µs      │ 1000    │ 4000
│  ├─ u8                                                 │               │               │               │         │
│  │  ├─ baseline_identity                 11.99 ns      │ 37.77 ns      │ 12.77 ns      │ 13.43 ns      │ 1000    │ 1024000
│  │  ├─ pow2_div                          109.1 ns      │ 166.9 ns      │ 111.5 ns      │ 112.6 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_ceil                     165.4 ns      │ 238.8 ns      │ 168.5 ns      │ 168.6 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_ceil_const               37.68 ns      │ 97.05 ns      │ 39.43 ns      │ 39.75 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_ceil_reuse               32.99 ns      │ 120.4 ns      │ 34.55 ns      │ 36.04 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_const                    26.74 ns      │ 162.8 ns      │ 29.86 ns      │ 32.7 ns       │ 1000    │ 512000
│  │  ├─ pow2_div_floor                    99 ns         │ 204.4 ns      │ 100.5 ns      │ 101.6 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_floor_const              26.35 ns      │ 60.14 ns      │ 29.67 ns      │ 29.94 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_floor_reuse              29.28 ns      │ 57.4 ns       │ 32.01 ns      │ 32.59 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_reuse                    30.25 ns      │ 71.86 ns      │ 33.57 ns      │ 34.06 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_round                    165.4 ns      │ 234.1 ns      │ 168.5 ns      │ 168.7 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_round_const              38.07 ns      │ 72.64 ns      │ 40.8 ns       │ 41.18 ns      │ 1000    │ 512000
│  │  ├─ pow2_div_round_reuse              31.43 ns      │ 60.33 ns      │ 34.16 ns      │ 34.76 ns      │ 1000    │ 256000
│  │  ├─ std_div                           856 ns        │ 7.487 µs      │ 862.2 ns      │ 883.2 ns      │ 1000    │ 16000
│  │  ├─ std_div_const                     27.71 ns      │ 357 ns        │ 30.84 ns      │ 33.05 ns      │ 1000    │ 512000
│  │  ├─ std_div_reuse                     824.7 ns      │ 9.056 µs      │ 843.5 ns      │ 888.5 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_div                      106.8 ns      │ 1.139 µs      │ 110.7 ns      │ 118.3 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_ceil                 170.1 ns      │ 1.851 µs      │ 174.7 ns      │ 177.8 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil_const           36.89 ns      │ 79.08 ns      │ 39.04 ns      │ 39.79 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_ceil_reuse           33.38 ns      │ 62.29 ns      │ 35.72 ns      │ 36.81 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_const                26.74 ns      │ 72.05 ns      │ 31.43 ns      │ 31.75 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_floor                107.6 ns      │ 172.4 ns      │ 109.9 ns      │ 110.6 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_floor_const          29.47 ns      │ 73.22 ns      │ 32.01 ns      │ 32.63 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_floor_reuse          31.04 ns      │ 81.82 ns      │ 34.75 ns      │ 35.33 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_reuse                30.64 ns      │ 175.1 ns      │ 35.33 ns      │ 39.14 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_div_round                171.6 ns      │ 252.9 ns      │ 176.3 ns      │ 177.7 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_round_const          33.96 ns      │ 84.55 ns      │ 39.24 ns      │ 39.73 ns      │ 1000    │ 512000
│  │  ╰─ unb_pow2_div_round_reuse          30.64 ns      │ 76.35 ns      │ 32.99 ns      │ 33.74 ns      │ 1000    │ 256000
│  ├─ u16                                                │               │               │               │         │
│  │  ├─ baseline_identity                 26.15 ns      │ 70.88 ns      │ 26.93 ns      │ 28.06 ns      │ 1000    │ 512000
│  │  ├─ pow2_div                          232.6 ns      │ 437.2 ns      │ 237.2 ns      │ 237.6 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_ceil                     399.7 ns      │ 537.2 ns      │ 406 ns        │ 406.7 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_ceil_const               66.19 ns      │ 222.4 ns      │ 74.39 ns      │ 77.41 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_ceil_reuse               63.07 ns      │ 128.6 ns      │ 67.75 ns      │ 69.34 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_const                    48.22 ns      │ 114.2 ns      │ 52.91 ns      │ 53.46 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_floor                    232.6 ns      │ 445.1 ns      │ 237.2 ns      │ 241.3 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_floor_const              50.96 ns      │ 123.2 ns      │ 54.08 ns      │ 55.17 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_floor_reuse              54.08 ns      │ 167.3 ns      │ 61.5 ns       │ 63.89 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_reuse                    52.13 ns      │ 151.7 ns      │ 59.16 ns      │ 60.39 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_round                    384.1 ns      │ 524.7 ns      │ 390.4 ns      │ 394 ns        │ 1000    │ 32000
│  │  ├─ pow2_div_round_const              67.75 ns      │ 165.8 ns      │ 77.52 ns      │ 77.28 ns      │ 1000    │ 256000
│  │  ├─ pow2_div_round_reuse              73.22 ns      │ 186.8 ns      │ 82.99 ns      │ 84.98 ns      │ 1000    │ 256000
│  │  ├─ std_div                           881 ns        │ 1.162 µs      │ 893.5 ns      │ 901.8 ns      │ 1000    │ 16000
│  │  ├─ std_div_const                     47.05 ns      │ 127.1 ns      │ 52.13 ns      │ 55.48 ns      │ 1000    │ 256000
│  │  ├─ std_div_reuse                     837.2 ns      │ 1.312 µs      │ 849.7 ns      │ 852.3 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_div                      231 ns        │ 421.6 ns      │ 237.2 ns      │ 237.3 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_ceil                 412.2 ns      │ 909.1 ns      │ 415.4 ns      │ 419 ns        │ 1000    │ 32000
│  │  ├─ unb_pow2_div_ceil_const           66.58 ns      │ 152.1 ns      │ 73.61 ns      │ 74.11 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_ceil_reuse           75.57 ns      │ 206.8 ns      │ 86.11 ns      │ 87.47 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_const                47.44 ns      │ 183.3 ns      │ 54.08 ns      │ 54.94 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_floor                231 ns        │ 634.1 ns      │ 235.7 ns      │ 243.9 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor_const          48.22 ns      │ 138 ns        │ 52.91 ns      │ 55.81 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_floor_reuse          51.35 ns      │ 141.1 ns      │ 59.16 ns      │ 59.08 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_reuse                52.13 ns      │ 104 ns        │ 57.99 ns      │ 58.86 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_div_round                399.7 ns      │ 1.096 µs      │ 402.9 ns      │ 405.3 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_round_const          67.36 ns      │ 146.6 ns      │ 75.18 ns      │ 78.12 ns      │ 1000    │ 256000
│  │  ╰─ unb_pow2_div_round_reuse          68.54 ns      │ 196.2 ns      │ 79.47 ns      │ 80.75 ns      │ 1000    │ 256000
│  ├─ u32                                                │               │               │               │         │
│  │  ├─ baseline_identity                 44.32 ns      │ 122 ns        │ 50.57 ns      │ 50.44 ns      │ 1000    │ 256000
│  │  ├─ pow2_div                          296.6 ns      │ 631 ns        │ 304.4 ns      │ 310.6 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_ceil                     506 ns        │ 1.046 µs      │ 512.2 ns      │ 520.9 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_ceil_const               131.8 ns      │ 320.1 ns      │ 145.1 ns      │ 148.8 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_ceil_reuse               141.1 ns      │ 257.6 ns      │ 154.4 ns      │ 156.6 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_const                    97.44 ns      │ 248.2 ns      │ 105.2 ns      │ 106.1 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_floor                    291.9 ns      │ 395.1 ns      │ 298.2 ns      │ 298.8 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_floor_const              95.1 ns       │ 220.1 ns      │ 104.4 ns      │ 105.2 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_floor_reuse              106 ns        │ 255.2 ns      │ 126.3 ns      │ 127.4 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_reuse                    103.6 ns      │ 244.3 ns      │ 117.7 ns      │ 119.9 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_round                    406 ns        │ 756 ns        │ 412.2 ns      │ 416.1 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_round_const              129.4 ns      │ 266.1 ns      │ 143.5 ns      │ 144.1 ns      │ 1000    │ 128000
│  │  ├─ pow2_div_round_reuse              139.6 ns      │ 288.8 ns      │ 155.2 ns      │ 156.9 ns      │ 1000    │ 128000
│  │  ├─ std_div                           1.324 µs      │ 1.874 µs      │ 1.337 µs      │ 1.341 µs      │ 1000    │ 8000
│  │  ├─ std_div_const                     93.54 ns      │ 261.5 ns      │ 102.1 ns      │ 103.9 ns      │ 1000    │ 128000
│  │  ├─ std_div_reuse                     1.274 µs      │ 2.812 µs      │ 1.299 µs      │ 1.306 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div                      296.6 ns      │ 451.3 ns      │ 302.9 ns      │ 305.6 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil                 527.9 ns      │ 671.6 ns      │ 531 ns        │ 533.6 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_ceil_const           123.2 ns      │ 298.2 ns      │ 142.7 ns      │ 142.5 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_ceil_reuse           132.6 ns      │ 270.1 ns      │ 152.9 ns      │ 153.9 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_const                95.1 ns       │ 242.7 ns      │ 108.3 ns      │ 112.8 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_floor                295.1 ns      │ 1.866 µs      │ 302.9 ns      │ 309.8 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor_const          91.19 ns      │ 1.006 µs      │ 101.3 ns      │ 106.8 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_floor_reuse          99.79 ns      │ 306 ns        │ 112.2 ns      │ 116.2 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_reuse                99.79 ns      │ 979.4 ns      │ 117.7 ns      │ 121.8 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_div_round                418.5 ns      │ 715.4 ns      │ 424.7 ns      │ 428.2 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_round_const          110.7 ns      │ 196.6 ns      │ 120.1 ns      │ 121.4 ns      │ 1000    │ 64000
│  │  ╰─ unb_pow2_div_round_reuse          132.6 ns      │ 374 ns        │ 153.6 ns      │ 154.9 ns      │ 1000    │ 128000
│  ├─ u64                                                │               │               │               │         │
│  │  ├─ baseline_identity                 82.6 ns       │ 406 ns        │ 101.3 ns      │ 107.2 ns      │ 1000    │ 128000
│  │  ├─ pow2_div                          421.6 ns      │ 824.7 ns      │ 431 ns        │ 435.6 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_ceil                     637.2 ns      │ 912.2 ns      │ 649.7 ns      │ 651.7 ns      │ 1000    │ 16000
│  │  ├─ pow2_div_ceil_const               241.9 ns      │ 796.6 ns      │ 281 ns        │ 296.5 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_ceil_reuse               396.6 ns      │ 665.4 ns      │ 406 ns        │ 414.5 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_const                    184.1 ns      │ 656 ns        │ 221.6 ns      │ 220.6 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_floor                    334.1 ns      │ 534.1 ns      │ 346.6 ns      │ 349.7 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_floor_const              191.9 ns      │ 593.5 ns      │ 218.5 ns      │ 221.2 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_floor_reuse              209.1 ns      │ 499.7 ns      │ 238.8 ns      │ 246.1 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_reuse                    204.4 ns      │ 782.6 ns      │ 235.7 ns      │ 248.7 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_round                    581 ns        │ 4.837 µs      │ 596.6 ns      │ 629.3 ns      │ 1000    │ 32000
│  │  ├─ pow2_div_round_const              246.6 ns      │ 2.024 µs      │ 276.3 ns      │ 285.7 ns      │ 1000    │ 64000
│  │  ├─ pow2_div_round_reuse              396.6 ns      │ 809.1 ns      │ 446.6 ns      │ 454.3 ns      │ 1000    │ 32000
│  │  ├─ std_div                           1.649 µs      │ 3.587 µs      │ 1.674 µs      │ 1.692 µs      │ 1000    │ 8000
│  │  ├─ std_div_const                     199.7 ns      │ 727.9 ns      │ 232.6 ns      │ 236.5 ns      │ 1000    │ 64000
│  │  ├─ std_div_reuse                     1.512 µs      │ 5.312 µs      │ 1.524 µs      │ 1.538 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_div                      431 ns        │ 865.4 ns      │ 440.4 ns      │ 450.9 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_ceil                 662.2 ns      │ 1.731 µs      │ 674.7 ns      │ 692 ns        │ 1000    │ 16000
│  │  ├─ unb_pow2_div_ceil_const           257.6 ns      │ 701.3 ns      │ 281 ns        │ 285.7 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_ceil_reuse           393.5 ns      │ 768.5 ns      │ 406 ns        │ 410.7 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_const                190.4 ns      │ 596.6 ns      │ 221.6 ns      │ 224.6 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor                434.1 ns      │ 1.112 µs      │ 443.5 ns      │ 453.3 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_div_floor_const          184.1 ns      │ 835.7 ns      │ 213.8 ns      │ 217.8 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_floor_reuse          195.1 ns      │ 449.7 ns      │ 231 ns        │ 236.4 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_div_reuse                195.1 ns      │ 496.6 ns      │ 227.9 ns      │ 232 ns        │ 1000    │ 64000
│  │  ├─ unb_pow2_div_round                637.2 ns      │ 1.274 µs      │ 668.5 ns      │ 675 ns        │ 1000    │ 16000
│  │  ├─ unb_pow2_div_round_const          235.7 ns      │ 493.5 ns      │ 256 ns        │ 260.2 ns      │ 1000    │ 64000
│  │  ╰─ unb_pow2_div_round_reuse          393.5 ns      │ 634.1 ns      │ 440.4 ns      │ 438.3 ns      │ 1000    │ 32000
│  ╰─ u128                                               │               │               │               │         │
│     ├─ baseline_identity                 202.9 ns      │ 562.2 ns      │ 259.1 ns      │ 262.1 ns      │ 1000    │ 64000
│     ├─ pow2_div                          1.006 µs      │ 9.899 µs      │ 1.043 µs      │ 1.076 µs      │ 1000    │ 16000
│     ├─ pow2_div_ceil                     1.774 µs      │ 3.249 µs      │ 1.824 µs      │ 1.826 µs      │ 1000    │ 8000
│     ├─ pow2_div_ceil_const               1.006 µs      │ 1.462 µs      │ 1.018 µs      │ 1.021 µs      │ 1000    │ 16000
│     ├─ pow2_div_ceil_reuse               1.349 µs      │ 2.162 µs      │ 1.374 µs      │ 1.374 µs      │ 1000    │ 8000
│     ├─ pow2_div_const                    793.5 ns      │ 1.562 µs      │ 812.2 ns      │ 818.3 ns      │ 1000    │ 16000
│     ├─ pow2_div_floor                    999.7 ns      │ 1.674 µs      │ 1.018 µs      │ 1.022 µs      │ 1000    │ 16000
│     ├─ pow2_div_floor_const              799.7 ns      │ 1.962 µs      │ 812.2 ns      │ 818.4 ns      │ 1000    │ 16000
│     ├─ pow2_div_floor_reuse              1.068 µs      │ 1.743 µs      │ 1.087 µs      │ 1.114 µs      │ 1000    │ 16000
│     ├─ pow2_div_reuse                    1.074 µs      │ 1.356 µs      │ 1.087 µs      │ 1.088 µs      │ 1000    │ 16000
│     ├─ pow2_div_round                    2.524 µs      │ 3.299 µs      │ 2.549 µs      │ 2.548 µs      │ 1000    │ 4000
│     ├─ pow2_div_round_const              1.031 µs      │ 6.149 µs      │ 1.043 µs      │ 1.075 µs      │ 1000    │ 16000
│     ├─ pow2_div_round_reuse              1.362 µs      │ 4.337 µs      │ 1.387 µs      │ 1.396 µs      │ 1000    │ 8000
│     ├─ std_div                           5.899 µs      │ 8.849 µs      │ 5.999 µs      │ 5.998 µs      │ 1000    │ 2000
│     ├─ std_div_const                     799.7 ns      │ 1.168 µs      │ 812.2 ns      │ 817 ns        │ 1000    │ 16000
│     ├─ std_div_reuse                     4.274 µs      │ 12.14 µs      │ 4.324 µs      │ 4.348 µs      │ 1000    │ 4000
│     ├─ unb_pow2_div                      962.2 ns      │ 1.531 µs      │ 987.2 ns      │ 992.5 ns      │ 1000    │ 16000
│     ├─ unb_pow2_div_ceil                 1.762 µs      │ 3.187 µs      │ 1.774 µs      │ 1.786 µs      │ 1000    │ 8000
│     ├─ unb_pow2_div_ceil_const           1.012 µs      │ 1.443 µs      │ 1.024 µs      │ 1.027 µs      │ 1000    │ 16000
│     ├─ unb_pow2_div_ceil_reuse           1.312 µs      │ 3.612 µs      │ 1.349 µs      │ 1.357 µs      │ 1000    │ 8000
│     ├─ unb_pow2_div_const                799.7 ns      │ 1.374 µs      │ 812.2 ns      │ 818.4 ns      │ 1000    │ 16000
│     ├─ unb_pow2_div_floor                968.5 ns      │ 1.568 µs      │ 981 ns        │ 989.7 ns      │ 1000    │ 16000
│     ├─ unb_pow2_div_floor_const          799.7 ns      │ 7.406 µs      │ 818.5 ns      │ 849.9 ns      │ 1000    │ 16000
│     ├─ unb_pow2_div_floor_reuse          1.062 µs      │ 8.449 µs      │ 1.081 µs      │ 1.116 µs      │ 1000    │ 16000
│     ├─ unb_pow2_div_reuse                1.062 µs      │ 7.881 µs      │ 1.081 µs      │ 1.113 µs      │ 1000    │ 16000
│     ├─ unb_pow2_div_round                2.524 µs      │ 4.974 µs      │ 2.574 µs      │ 2.635 µs      │ 1000    │ 4000
│     ├─ unb_pow2_div_round_const          1.018 µs      │ 2.137 µs      │ 1.037 µs      │ 1.06 µs       │ 1000    │ 16000
│     ╰─ unb_pow2_div_round_reuse          1.362 µs      │ 2.987 µs      │ 1.474 µs      │ 1.503 µs      │ 1000    │ 8000
├─ mul                                                   │               │               │               │         │
│  ├─ i8                                                 │               │               │               │         │
│  │  ├─ baseline_identity                 10.14 ns      │ 47.44 ns      │ 11.5 ns       │ 11.85 ns      │ 1000    │ 512000
│  │  ├─ pow2_mul                          98.22 ns      │ 1.2 µs        │ 100.5 ns      │ 106.4 ns      │ 1000    │ 128000
│  │  ├─ pow2_mul_const                    25.96 ns      │ 337.4 ns      │ 30.06 ns      │ 31.95 ns      │ 1000    │ 512000
│  │  ├─ pow2_mul_reuse                    32.4 ns       │ 84.94 ns      │ 36.31 ns      │ 36.64 ns      │ 1000    │ 512000
│  │  ├─ std_mul                           56.43 ns      │ 112.6 ns      │ 65.02 ns      │ 64.1 ns       │ 1000    │ 256000
│  │  ├─ std_mul_const                     26.93 ns      │ 75.96 ns      │ 30.25 ns      │ 30.76 ns      │ 1000    │ 512000
│  │  ├─ std_mul_reuse                     37.29 ns      │ 69.32 ns      │ 40.02 ns      │ 41.19 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_mul                      101.3 ns      │ 210.7 ns      │ 104.4 ns      │ 105.2 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_mul_const                26.74 ns      │ 295.4 ns      │ 28.89 ns      │ 31.1 ns       │ 1000    │ 512000
│  │  ╰─ unb_pow2_mul_reuse                30.06 ns      │ 213.2 ns      │ 34.55 ns      │ 39.06 ns      │ 1000    │ 512000
│  ├─ i16                                                │               │               │               │         │
│  │  ├─ baseline_identity                 26.35 ns      │ 222.6 ns      │ 27.13 ns      │ 28.56 ns      │ 1000    │ 512000
│  │  ├─ pow2_mul                          184.1 ns      │ 254.4 ns      │ 188.8 ns      │ 188.9 ns      │ 1000    │ 64000
│  │  ├─ pow2_mul_const                    49.39 ns      │ 142.3 ns      │ 54.08 ns      │ 54.56 ns      │ 1000    │ 256000
│  │  ├─ pow2_mul_reuse                    53.3 ns       │ 488 ns        │ 57.6 ns       │ 59.23 ns      │ 1000    │ 256000
│  │  ├─ std_mul                           79.08 ns      │ 484.1 ns      │ 88.07 ns      │ 92.45 ns      │ 1000    │ 256000
│  │  ├─ std_mul_const                     46.66 ns      │ 161.5 ns      │ 52.13 ns      │ 52.61 ns      │ 1000    │ 256000
│  │  ├─ std_mul_reuse                     57.21 ns      │ 157.2 ns      │ 59.55 ns      │ 60.16 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_mul                      185.7 ns      │ 387.2 ns      │ 190.4 ns      │ 190.7 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_mul_const                48.22 ns      │ 384.5 ns      │ 53.3 ns       │ 54.37 ns      │ 1000    │ 256000
│  │  ╰─ unb_pow2_mul_reuse                54.47 ns      │ 127.1 ns      │ 60.72 ns      │ 61.27 ns      │ 1000    │ 256000
│  ├─ i32                                                │               │               │               │         │
│  │  ├─ baseline_identity                 43.93 ns      │ 237.2 ns      │ 51.74 ns      │ 55.25 ns      │ 1000    │ 256000
│  │  ├─ pow2_mul                          277.9 ns      │ 504.4 ns      │ 291.9 ns      │ 296.5 ns      │ 1000    │ 64000
│  │  ├─ pow2_mul_const                    94.32 ns      │ 273.2 ns      │ 106.8 ns      │ 108.3 ns      │ 1000    │ 128000
│  │  ├─ pow2_mul_reuse                    99.79 ns      │ 263 ns        │ 119.3 ns      │ 119.6 ns      │ 1000    │ 128000
│  │  ├─ std_mul                           187.2 ns      │ 396.6 ns      │ 193.5 ns      │ 197.4 ns      │ 1000    │ 64000
│  │  ├─ std_mul_const                     99.79 ns      │ 255.2 ns      │ 108.3 ns      │ 109.9 ns      │ 1000    │ 128000
│  │  ├─ std_mul_reuse                     143.5 ns      │ 232.6 ns      │ 148.2 ns      │ 151.6 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_mul                      274.7 ns      │ 495.1 ns      │ 281 ns        │ 282.5 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_mul_const                91.97 ns      │ 308.3 ns      │ 101.3 ns      │ 103.2 ns      │ 1000    │ 128000
│  │  ╰─ unb_pow2_mul_reuse                99 ns         │ 862.2 ns      │ 116.1 ns      │ 119.4 ns      │ 1000    │ 128000
│  ├─ i64                                                │               │               │               │         │
│  │  ├─ baseline_identity                 90.41 ns      │ 252.1 ns      │ 110.7 ns      │ 113.6 ns      │ 1000    │ 128000
│  │  ├─ pow2_mul                          331 ns        │ 502.9 ns      │ 346.6 ns      │ 349.1 ns      │ 1000    │ 32000
│  │  ├─ pow2_mul_const                    187.2 ns      │ 446.6 ns      │ 215.4 ns      │ 216.3 ns      │ 1000    │ 64000
│  │  ├─ pow2_mul_reuse                    198.2 ns      │ 837.2 ns      │ 232.6 ns      │ 246.2 ns      │ 1000    │ 64000
│  │  ├─ std_mul                           462.2 ns      │ 1.146 µs      │ 496.6 ns      │ 512.3 ns      │ 1000    │ 32000
│  │  ├─ std_mul_const                     198.2 ns      │ 641.9 ns      │ 221.6 ns      │ 225.9 ns      │ 1000    │ 64000
│  │  ├─ std_mul_reuse                     290.4 ns      │ 509.1 ns      │ 302.9 ns      │ 306.7 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_mul                      431 ns        │ 827.9 ns      │ 440.4 ns      │ 444.7 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_mul_const                193.5 ns      │ 2.021 µs      │ 220.1 ns      │ 228.2 ns      │ 1000    │ 64000
│  │  ╰─ unb_pow2_mul_reuse                204.4 ns      │ 2.996 µs      │ 235.7 ns      │ 248.2 ns      │ 1000    │ 64000
│  ├─ i128                                               │               │               │               │         │
│  │  ├─ baseline_identity                 207.6 ns      │ 718.5 ns      │ 277.9 ns      │ 289.9 ns      │ 1000    │ 64000
│  │  ├─ pow2_mul                          987.2 ns      │ 1.856 µs      │ 1.024 µs      │ 1.031 µs      │ 1000    │ 16000
│  │  ├─ pow2_mul_const                    824.7 ns      │ 1.693 µs      │ 849.7 ns      │ 887.8 ns      │ 1000    │ 16000
│  │  ├─ pow2_mul_reuse                    1.068 µs      │ 10.15 µs      │ 1.162 µs      │ 1.19 µs       │ 1000    │ 16000
│  │  ├─ std_mul                           1.287 µs      │ 9.474 µs      │ 1.337 µs      │ 1.399 µs      │ 1000    │ 16000
│  │  ├─ std_mul_const                     818.5 ns      │ 9.268 µs      │ 843.5 ns      │ 872.4 ns      │ 1000    │ 16000
│  │  ├─ std_mul_reuse                     874.7 ns      │ 8.806 µs      │ 887.2 ns      │ 927 ns        │ 1000    │ 16000
│  │  ├─ unb_pow2_mul                      981 ns        │ 9.081 µs      │ 993.5 ns      │ 1.03 µs       │ 1000    │ 16000
│  │  ├─ unb_pow2_mul_const                837.2 ns      │ 8.943 µs      │ 849.7 ns      │ 874.2 ns      │ 1000    │ 16000
│  │  ╰─ unb_pow2_mul_reuse                1.068 µs      │ 9.681 µs      │ 1.093 µs      │ 1.134 µs      │ 1000    │ 16000
│  ├─ u8                                                 │               │               │               │         │
│  │  ├─ baseline_identity                 11.99 ns      │ 37.29 ns      │ 12.68 ns      │ 13.19 ns      │ 1000    │ 1024000
│  │  ├─ pow2_mul                          97.44 ns      │ 170.8 ns      │ 99.79 ns      │ 100.6 ns      │ 1000    │ 128000
│  │  ├─ pow2_mul_const                    26.15 ns      │ 162 ns        │ 29.08 ns      │ 29.52 ns      │ 1000    │ 512000
│  │  ├─ pow2_mul_reuse                    22.83 ns      │ 70.49 ns      │ 27.13 ns      │ 28.91 ns      │ 1000    │ 256000
│  │  ├─ std_mul                           56.04 ns      │ 143.1 ns      │ 61.11 ns      │ 62.19 ns      │ 1000    │ 256000
│  │  ├─ std_mul_const                     25.76 ns      │ 195.1 ns      │ 29.08 ns      │ 30.15 ns      │ 1000    │ 512000
│  │  ├─ std_mul_reuse                     36.11 ns      │ 85.72 ns      │ 37.68 ns      │ 39.32 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_mul                      101.3 ns      │ 417.7 ns      │ 103.6 ns      │ 105.7 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_mul_const                26.35 ns      │ 385.9 ns      │ 30.45 ns      │ 33.09 ns      │ 1000    │ 512000
│  │  ╰─ unb_pow2_mul_reuse                29.47 ns      │ 67.17 ns      │ 34.16 ns      │ 34.91 ns      │ 1000    │ 512000
│  ├─ u16                                                │               │               │               │         │
│  │  ├─ baseline_identity                 26.35 ns      │ 79.08 ns      │ 27.13 ns      │ 27.68 ns      │ 1000    │ 512000
│  │  ├─ pow2_mul                          185.7 ns      │ 391.9 ns      │ 188.8 ns      │ 190 ns        │ 1000    │ 64000
│  │  ├─ pow2_mul_const                    47.44 ns      │ 111.8 ns      │ 52.52 ns      │ 53.22 ns      │ 1000    │ 256000
│  │  ├─ pow2_mul_reuse                    54.08 ns      │ 105.6 ns      │ 58.77 ns      │ 59.44 ns      │ 1000    │ 256000
│  │  ├─ std_mul                           62.29 ns      │ 709.9 ns      │ 66.19 ns      │ 68.86 ns      │ 1000    │ 128000
│  │  ├─ std_mul_const                     49.79 ns      │ 142.7 ns      │ 55.64 ns      │ 56.82 ns      │ 1000    │ 256000
│  │  ├─ std_mul_reuse                     52.13 ns      │ 270.1 ns      │ 57.21 ns      │ 62.23 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_mul                      185.7 ns      │ 304.4 ns      │ 190.4 ns      │ 192.6 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_mul_const                47.05 ns      │ 195.4 ns      │ 50.57 ns      │ 53.19 ns      │ 1000    │ 256000
│  │  ╰─ unb_pow2_mul_reuse                51.74 ns      │ 601.3 ns      │ 56.82 ns      │ 61.85 ns      │ 1000    │ 256000
│  ├─ u32                                                │               │               │               │         │
│  │  ├─ baseline_identity                 44.71 ns      │ 458.3 ns      │ 53.69 ns      │ 56.38 ns      │ 1000    │ 256000
│  │  ├─ pow2_mul                          279.4 ns      │ 445.1 ns      │ 293.5 ns      │ 296.5 ns      │ 1000    │ 64000
│  │  ├─ pow2_mul_const                    91.19 ns      │ 213 ns        │ 99 ns         │ 102.1 ns      │ 1000    │ 128000
│  │  ├─ pow2_mul_reuse                    97.44 ns      │ 391.1 ns      │ 108.3 ns      │ 109.9 ns      │ 1000    │ 128000
│  │  ├─ std_mul                           191.9 ns      │ 434.1 ns      │ 199.7 ns      │ 205.2 ns      │ 1000    │ 64000
│  │  ├─ std_mul_const                     91.19 ns      │ 207.6 ns      │ 98.22 ns      │ 101 ns        │ 1000    │ 128000
│  │  ├─ std_mul_reuse                     148.2 ns      │ 335.7 ns      │ 168.5 ns      │ 172.2 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_mul                      274.7 ns      │ 485.7 ns      │ 282.6 ns      │ 286.6 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_mul_const                84.16 ns      │ 231 ns        │ 93.54 ns      │ 97.61 ns      │ 1000    │ 64000
│  │  ╰─ unb_pow2_mul_reuse                98.22 ns      │ 300.5 ns      │ 110.7 ns      │ 114.9 ns      │ 1000    │ 128000
│  ├─ u64                                                │               │               │               │         │
│  │  ├─ baseline_identity                 83.38 ns      │ 1.134 µs      │ 104.4 ns      │ 112.4 ns      │ 1000    │ 128000
│  │  ├─ pow2_mul                          331 ns        │ 4.809 µs      │ 346.6 ns      │ 387.5 ns      │ 1000    │ 32000
│  │  ├─ pow2_mul_const                    184.1 ns      │ 1.506 µs      │ 202.9 ns      │ 211 ns        │ 1000    │ 64000
│  │  ├─ pow2_mul_reuse                    193.5 ns      │ 470.1 ns      │ 224.7 ns      │ 226.9 ns      │ 1000    │ 64000
│  │  ├─ std_mul                           459.1 ns      │ 796.6 ns      │ 474.7 ns      │ 479.1 ns      │ 1000    │ 32000
│  │  ├─ std_mul_const                     182.6 ns      │ 341.9 ns      │ 195.1 ns      │ 198.2 ns      │ 1000    │ 64000
│  │  ├─ std_mul_reuse                     334.1 ns      │ 559.1 ns      │ 346.6 ns      │ 354.1 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_mul                      437.2 ns      │ 665.4 ns      │ 446.6 ns      │ 450 ns        │ 1000    │ 32000
│  │  ├─ unb_pow2_mul_const                184.1 ns      │ 393.5 ns      │ 198.2 ns      │ 201.5 ns      │ 1000    │ 64000
│  │  ╰─ unb_pow2_mul_reuse                193.5 ns      │ 1.546 µs      │ 221.6 ns      │ 229.8 ns      │ 1000    │ 64000
│  ╰─ u128                                               │               │               │               │         │
│     ├─ baseline_identity                 209.1 ns      │ 2.224 µs      │ 256 ns        │ 263.6 ns      │ 1000    │ 64000
│     ├─ pow2_mul                          1.006 µs      │ 1.837 µs      │ 1.037 µs      │ 1.048 µs      │ 1000    │ 16000
│     ├─ pow2_mul_const                    806 ns        │ 1.674 µs      │ 824.7 ns      │ 835.8 ns      │ 1000    │ 16000
│     ├─ pow2_mul_reuse                    1.099 µs      │ 2.749 µs      │ 1.137 µs      │ 1.162 µs      │ 1000    │ 16000
│     ├─ std_mul                           1.237 µs      │ 2.824 µs      │ 1.324 µs      │ 1.319 µs      │ 1000    │ 8000
│     ├─ std_mul_const                     806 ns        │ 1.499 µs      │ 843.5 ns      │ 867.4 ns      │ 1000    │ 16000
│     ├─ std_mul_reuse                     881 ns        │ 1.731 µs      │ 956 ns        │ 953.9 ns      │ 1000    │ 16000
│     ├─ unb_pow2_mul                      968.5 ns      │ 1.724 µs      │ 999.7 ns      │ 1.01 µs       │ 1000    │ 16000
│     ├─ unb_pow2_mul_const                799.7 ns      │ 1.906 µs      │ 818.5 ns      │ 822.6 ns      │ 1000    │ 16000
│     ╰─ unb_pow2_mul_reuse                1.074 µs      │ 2.062 µs      │ 1.099 µs      │ 1.112 µs      │ 1000    │ 16000
├─ rem                                                   │               │               │               │         │
│  ├─ i8                                                 │               │               │               │         │
│  │  ├─ baseline_identity                 11.99 ns      │ 36.41 ns      │ 12.58 ns      │ 13.24 ns      │ 1000    │ 1024000
│  │  ├─ pow2_is_multiple_of               77.13 ns      │ 166.1 ns      │ 79.86 ns      │ 83.47 ns      │ 1000    │ 256000
│  │  ├─ pow2_is_multiple_of_const         29.67 ns      │ 315.4 ns      │ 32.6 ns       │ 34.72 ns      │ 1000    │ 512000
│  │  ├─ pow2_is_multiple_of_reuse         69.32 ns      │ 666.5 ns      │ 70.88 ns      │ 76.9 ns       │ 1000    │ 256000
│  │  ├─ pow2_rem                          121.6 ns      │ 1.469 µs      │ 124.7 ns      │ 135.9 ns      │ 1000    │ 128000
│  │  ├─ pow2_rem_const                    41.19 ns      │ 630.6 ns      │ 44.32 ns      │ 48.03 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem_floor                    94.32 ns      │ 1.284 µs      │ 98.22 ns      │ 107.7 ns      │ 1000    │ 128000
│  │  ├─ pow2_rem_floor_const              24.98 ns      │ 108.1 ns      │ 26.35 ns      │ 27.8 ns       │ 1000    │ 512000
│  │  ├─ pow2_rem_floor_reuse              26.93 ns      │ 104.4 ns      │ 29.47 ns      │ 31.02 ns      │ 1000    │ 512000
│  │  ├─ pow2_rem_reuse                    42.75 ns      │ 650.5 ns      │ 45.88 ns      │ 50.66 ns      │ 1000    │ 256000
│  │  ├─ std_rem                           868.5 ns      │ 10.43 µs      │ 874.7 ns      │ 958.8 ns      │ 1000    │ 16000
│  │  ├─ std_rem_const                     35.72 ns      │ 368.7 ns      │ 39.04 ns      │ 41.48 ns      │ 1000    │ 512000
│  │  ├─ std_rem_reuse                     849.7 ns      │ 1.224 µs      │ 874.7 ns      │ 881.6 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_is_multiple_of           77.91 ns      │ 136.5 ns      │ 81.04 ns      │ 82.1 ns       │ 1000    │ 128000
│  │  ├─ unb_pow2_is_multiple_of_const     28.89 ns      │ 162 ns        │ 31.82 ns      │ 34.45 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_is_multiple_of_reuse     69.71 ns      │ 113 ns        │ 71.66 ns      │ 72.16 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_rem                      128.6 ns      │ 245.8 ns      │ 131.8 ns      │ 132.5 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_rem_const                41.19 ns      │ 70.49 ns      │ 43.14 ns      │ 43.7 ns       │ 1000    │ 256000
│  │  ├─ unb_pow2_rem_floor                99.79 ns      │ 205.2 ns      │ 103.6 ns      │ 104.3 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_rem_floor_const          24.39 ns      │ 74.59 ns      │ 25.37 ns      │ 25.77 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_rem_floor_reuse          27.52 ns      │ 60.53 ns      │ 28.89 ns      │ 29.55 ns      │ 1000    │ 512000
│  │  ╰─ unb_pow2_rem_reuse                41.97 ns      │ 117.3 ns      │ 43.54 ns      │ 46.22 ns      │ 1000    │ 256000
│  ├─ i16                                                │               │               │               │         │
│  │  ├─ baseline_identity                 23.42 ns      │ 137.8 ns      │ 27.32 ns      │ 30.14 ns      │ 1000    │ 512000
│  │  ├─ pow2_is_multiple_of               184.1 ns      │ 281 ns        │ 185.7 ns      │ 187.2 ns      │ 1000    │ 64000
│  │  ├─ pow2_is_multiple_of_const         40.41 ns      │ 100.1 ns      │ 44.32 ns      │ 44.76 ns      │ 1000    │ 256000
│  │  ├─ pow2_is_multiple_of_reuse         182.6 ns      │ 288.8 ns      │ 187.2 ns      │ 187.7 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem                          248.2 ns      │ 446.6 ns      │ 252.9 ns      │ 253.7 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem_const                    85.72 ns      │ 173.2 ns      │ 92.75 ns      │ 91.95 ns      │ 1000    │ 128000
│  │  ├─ pow2_rem_floor                    188.8 ns      │ 340.4 ns      │ 193.5 ns      │ 194.5 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem_floor_const              50.57 ns      │ 135.7 ns      │ 53.69 ns      │ 54.42 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem_floor_reuse              51.74 ns      │ 131.8 ns      │ 59.75 ns      │ 62.38 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem_reuse                    84.16 ns      │ 223.2 ns      │ 88.07 ns      │ 92.82 ns      │ 1000    │ 128000
│  │  ├─ std_rem                           874.7 ns      │ 1.843 µs      │ 893.5 ns      │ 911.4 ns      │ 1000    │ 16000
│  │  ├─ std_rem_const                     66.19 ns      │ 147.8 ns      │ 73.61 ns      │ 75.68 ns      │ 1000    │ 256000
│  │  ├─ std_rem_reuse                     2.549 µs      │ 3.649 µs      │ 2.574 µs      │ 2.601 µs      │ 1000    │ 4000
│  │  ├─ unb_pow2_is_multiple_of           181 ns        │ 251.3 ns      │ 184.1 ns      │ 185.5 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_is_multiple_of_const     38.85 ns      │ 91.97 ns      │ 43.54 ns      │ 43.69 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_is_multiple_of_reuse     181 ns        │ 259.1 ns      │ 184.1 ns      │ 185.3 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_rem                      257.6 ns      │ 329.4 ns      │ 260.7 ns      │ 262.4 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_rem_const                82.6 ns       │ 204.4 ns      │ 89.63 ns      │ 90.72 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_rem_floor                193.5 ns      │ 398.2 ns      │ 196.6 ns      │ 201.1 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_rem_floor_const          48.61 ns      │ 166.5 ns      │ 53.69 ns      │ 55.97 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_rem_floor_reuse          54.47 ns      │ 139.2 ns      │ 62.68 ns      │ 64.16 ns      │ 1000    │ 256000
│  │  ╰─ unb_pow2_rem_reuse                84.16 ns      │ 159.9 ns      │ 92.75 ns      │ 94.19 ns      │ 1000    │ 128000
│  ├─ i32                                                │               │               │               │         │
│  │  ├─ baseline_identity                 44.32 ns      │ 152.9 ns      │ 51.74 ns      │ 53.72 ns      │ 1000    │ 256000
│  │  ├─ pow2_is_multiple_of               459.1 ns      │ 624.7 ns      │ 462.2 ns      │ 466.1 ns      │ 1000    │ 32000
│  │  ├─ pow2_is_multiple_of_const         85.72 ns      │ 157.6 ns      │ 97.44 ns      │ 97.62 ns      │ 1000    │ 128000
│  │  ├─ pow2_is_multiple_of_reuse         427.9 ns      │ 593.5 ns      │ 434.1 ns      │ 436.8 ns      │ 1000    │ 32000
│  │  ├─ pow2_rem                          399.7 ns      │ 581 ns        │ 402.9 ns      │ 405.4 ns      │ 1000    │ 32000
│  │  ├─ pow2_rem_const                    163.8 ns      │ 256 ns        │ 168.5 ns      │ 170.1 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem_floor                    266.9 ns      │ 479.4 ns      │ 273.2 ns      │ 273.8 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem_floor_const              94.32 ns      │ 799 ns        │ 104.4 ns      │ 109.4 ns      │ 1000    │ 128000
│  │  ├─ pow2_rem_floor_reuse              98.22 ns      │ 434.9 ns      │ 113.8 ns      │ 119.4 ns      │ 1000    │ 128000
│  │  ├─ pow2_rem_reuse                    170.1 ns      │ 265.4 ns      │ 177.9 ns      │ 179.3 ns      │ 1000    │ 64000
│  │  ├─ std_rem                           1.337 µs      │ 1.887 µs      │ 1.362 µs      │ 1.368 µs      │ 1000    │ 8000
│  │  ├─ std_rem_const                     134.1 ns      │ 260.7 ns      │ 141.9 ns      │ 144.5 ns      │ 1000    │ 128000
│  │  ├─ std_rem_reuse                     1.287 µs      │ 1.849 µs      │ 1.312 µs      │ 1.316 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_is_multiple_of           456 ns        │ 627.9 ns      │ 462.2 ns      │ 464.7 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_is_multiple_of_const     87.29 ns      │ 171.6 ns      │ 99.79 ns      │ 100 ns        │ 1000    │ 128000
│  │  ├─ unb_pow2_is_multiple_of_reuse     431 ns        │ 927.9 ns      │ 437.2 ns      │ 439.7 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_rem                      415.4 ns      │ 552.9 ns      │ 418.5 ns      │ 421.4 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_rem_const                162.2 ns      │ 271.6 ns      │ 177.9 ns      │ 176.7 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_rem_floor                276.3 ns      │ 435.7 ns      │ 284.1 ns      │ 285.3 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_rem_floor_const          93.54 ns      │ 216.1 ns      │ 99 ns         │ 101.8 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_rem_floor_reuse          97.44 ns      │ 1.013 µs      │ 108.3 ns      │ 112.2 ns      │ 1000    │ 128000
│  │  ╰─ unb_pow2_rem_reuse                177.9 ns      │ 2.399 µs      │ 184.1 ns      │ 200.3 ns      │ 1000    │ 64000
│  ├─ i64                                                │               │               │               │         │
│  │  ├─ baseline_identity                 82.6 ns       │ 1.075 µs      │ 99 ns         │ 106.1 ns      │ 1000    │ 128000
│  │  ├─ pow2_is_multiple_of               346.6 ns      │ 1.587 µs      │ 365.4 ns      │ 368.9 ns      │ 1000    │ 32000
│  │  ├─ pow2_is_multiple_of_const         260.7 ns      │ 465.4 ns      │ 268.5 ns      │ 271.5 ns      │ 1000    │ 64000
│  │  ├─ pow2_is_multiple_of_reuse         318.5 ns      │ 706 ns        │ 324.7 ns      │ 326.5 ns      │ 1000    │ 32000
│  │  ├─ pow2_rem                          768.5 ns      │ 1.137 µs      │ 781 ns        │ 786.7 ns      │ 1000    │ 16000
│  │  ├─ pow2_rem_const                    356 ns        │ 1.04 µs       │ 368.5 ns      │ 369.6 ns      │ 1000    │ 32000
│  │  ├─ pow2_rem_floor                    462.2 ns      │ 709.1 ns      │ 471.6 ns      │ 473 ns        │ 1000    │ 32000
│  │  ├─ pow2_rem_floor_const              190.4 ns      │ 426.3 ns      │ 215.4 ns      │ 220.1 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem_floor_reuse              195.1 ns      │ 496.6 ns      │ 223.2 ns      │ 229.2 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem_reuse                    346.6 ns      │ 837.2 ns      │ 359.1 ns      │ 361.6 ns      │ 1000    │ 32000
│  │  ├─ std_rem                           1.674 µs      │ 3.299 µs      │ 1.712 µs      │ 1.715 µs      │ 1000    │ 8000
│  │  ├─ std_rem_const                     268.5 ns      │ 2.715 µs      │ 287.2 ns      │ 294.5 ns      │ 1000    │ 64000
│  │  ├─ std_rem_reuse                     1.549 µs      │ 17.04 µs      │ 1.587 µs      │ 1.612 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_is_multiple_of           346.6 ns      │ 759.1 ns      │ 362.2 ns      │ 374.7 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_is_multiple_of_const     262.2 ns      │ 482.6 ns      │ 270.1 ns      │ 272.3 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_is_multiple_of_reuse     318.5 ns      │ 456 ns        │ 324.7 ns      │ 325.6 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_rem                      768.5 ns      │ 8.999 µs      │ 781 ns        │ 798.9 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_rem_const                349.7 ns      │ 4.471 µs      │ 359.1 ns      │ 378.7 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_rem_floor                493.5 ns      │ 4.834 µs      │ 509.1 ns      │ 550.6 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_rem_floor_const          188.8 ns      │ 2.502 µs      │ 206 ns        │ 210.5 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_rem_floor_reuse          193.5 ns      │ 481 ns        │ 216.9 ns      │ 220.1 ns      │ 1000    │ 64000
│  │  ╰─ unb_pow2_rem_reuse                365.4 ns      │ 715.4 ns      │ 377.9 ns      │ 380.7 ns      │ 1000    │ 32000
│  ├─ i128                                               │               │               │               │         │
│  │  ├─ baseline_identity                 202.9 ns      │ 834.1 ns      │ 221.6 ns      │ 235.9 ns      │ 1000    │ 64000
│  │  ├─ pow2_is_multiple_of               587.2 ns      │ 852.9 ns      │ 599.7 ns      │ 603.8 ns      │ 1000    │ 32000
│  │  ├─ pow2_is_multiple_of_const         287.2 ns      │ 701.3 ns      │ 298.2 ns      │ 315.3 ns      │ 1000    │ 64000
│  │  ├─ pow2_is_multiple_of_reuse         556 ns        │ 777.9 ns      │ 574.7 ns      │ 580 ns        │ 1000    │ 32000
│  │  ├─ pow2_rem                          1.549 µs      │ 3.149 µs      │ 1.574 µs      │ 1.583 µs      │ 1000    │ 8000
│  │  ├─ pow2_rem_const                    1.062 µs      │ 1.768 µs      │ 1.087 µs      │ 1.093 µs      │ 1000    │ 16000
│  │  ├─ pow2_rem_floor                    981 ns        │ 1.606 µs      │ 1.012 µs      │ 1.017 µs      │ 1000    │ 16000
│  │  ├─ pow2_rem_floor_const              440.4 ns      │ 952.9 ns      │ 468.5 ns      │ 475 ns        │ 1000    │ 32000
│  │  ├─ pow2_rem_floor_reuse              624.7 ns      │ 974.7 ns      │ 699.7 ns      │ 697.4 ns      │ 1000    │ 16000
│  │  ├─ pow2_rem_reuse                    1.274 µs      │ 1.899 µs      │ 1.312 µs      │ 1.324 µs      │ 1000    │ 8000
│  │  ├─ std_rem                           9.299 µs      │ 13.69 µs      │ 9.499 µs      │ 9.585 µs      │ 1000    │ 1000
│  │  ├─ std_rem_const                     918.5 ns      │ 10.51 µs      │ 987.2 ns      │ 1.059 µs      │ 1000    │ 16000
│  │  ├─ std_rem_reuse                     8.049 µs      │ 80.39 µs      │ 8.249 µs      │ 8.864 µs      │ 1000    │ 2000
│  │  ├─ unb_pow2_is_multiple_of           584.1 ns      │ 5.212 µs      │ 593.5 ns      │ 638.1 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_is_multiple_of_const     284.1 ns      │ 3.281 µs      │ 295.1 ns      │ 319.8 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_is_multiple_of_reuse     562.2 ns      │ 5.393 µs      │ 574.7 ns      │ 623.6 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_rem                      1.562 µs      │ 22.04 µs      │ 1.599 µs      │ 1.766 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_rem_const                1.062 µs      │ 9.968 µs      │ 1.081 µs      │ 1.133 µs      │ 1000    │ 16000
│  │  ├─ unb_pow2_rem_floor                1.074 µs      │ 2.099 µs      │ 1.099 µs      │ 1.109 µs      │ 1000    │ 8000
│  │  ├─ unb_pow2_rem_floor_const          440.4 ns      │ 906 ns        │ 465.4 ns      │ 476.7 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_rem_floor_reuse          624.7 ns      │ 943.5 ns      │ 656 ns        │ 656.1 ns      │ 1000    │ 16000
│  │  ╰─ unb_pow2_rem_reuse                1.262 µs      │ 1.887 µs      │ 1.299 µs      │ 1.301 µs      │ 1000    │ 8000
│  ├─ u8                                                 │               │               │               │         │
│  │  ├─ baseline_identity                 12.09 ns      │ 38.55 ns      │ 12.58 ns      │ 13 ns         │ 1000    │ 1024000
│  │  ├─ pow2_is_multiple_of               77.91 ns      │ 128.3 ns      │ 80.64 ns      │ 81.48 ns      │ 1000    │ 256000
│  │  ├─ pow2_is_multiple_of_const         29.67 ns      │ 60.53 ns      │ 31.62 ns      │ 32.02 ns      │ 1000    │ 512000
│  │  ├─ pow2_is_multiple_of_reuse         68.93 ns      │ 99.79 ns      │ 71.27 ns      │ 71.67 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem                          94.32 ns      │ 166.9 ns      │ 98.22 ns      │ 99.03 ns      │ 1000    │ 128000
│  │  ├─ pow2_rem_const                    24.98 ns      │ 95.1 ns       │ 26.35 ns      │ 27.49 ns      │ 1000    │ 512000
│  │  ├─ pow2_rem_floor                    93.54 ns      │ 228.6 ns      │ 96.66 ns      │ 102 ns        │ 1000    │ 128000
│  │  ├─ pow2_rem_floor_const              25.18 ns      │ 63.07 ns      │ 27.52 ns      │ 29.15 ns      │ 1000    │ 512000
│  │  ├─ pow2_rem_floor_reuse              27.52 ns      │ 71.66 ns      │ 30.45 ns      │ 31.18 ns      │ 1000    │ 512000
│  │  ├─ pow2_rem_reuse                    27.52 ns      │ 74.59 ns      │ 29.47 ns      │ 30.27 ns      │ 1000    │ 512000
│  │  ├─ std_rem                           849.7 ns      │ 1.649 µs      │ 874.7 ns      │ 877.6 ns      │ 1000    │ 16000
│  │  ├─ std_rem_const                     25.76 ns      │ 73.22 ns      │ 27.91 ns      │ 28.25 ns      │ 1000    │ 512000
│  │  ├─ std_rem_reuse                     843.5 ns      │ 1.412 µs      │ 856 ns        │ 856.7 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_is_multiple_of           77.91 ns      │ 120.1 ns      │ 81.04 ns      │ 81.23 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_is_multiple_of_const     30.45 ns      │ 201.1 ns      │ 32.99 ns      │ 35.49 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_is_multiple_of_reuse     71.27 ns      │ 127.9 ns      │ 72.44 ns      │ 73.7 ns       │ 1000    │ 256000
│  │  ├─ unb_pow2_rem                      99.79 ns      │ 217.7 ns      │ 102.9 ns      │ 103.4 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_rem_const                26.74 ns      │ 48.03 ns      │ 27.71 ns      │ 28.27 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_rem_floor                99.79 ns      │ 190.4 ns      │ 102.9 ns      │ 103.7 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_rem_floor_const          26.74 ns      │ 278.3 ns      │ 27.91 ns      │ 29.34 ns      │ 1000    │ 512000
│  │  ├─ unb_pow2_rem_floor_reuse          28.5 ns       │ 305.6 ns      │ 29.86 ns      │ 32.56 ns      │ 1000    │ 512000
│  │  ╰─ unb_pow2_rem_reuse                27.52 ns      │ 57.99 ns      │ 28.89 ns      │ 29.74 ns      │ 1000    │ 512000
│  ├─ u16                                                │               │               │               │         │
│  │  ├─ baseline_identity                 26.35 ns      │ 247 ns        │ 27.71 ns      │ 31.56 ns      │ 1000    │ 512000
│  │  ├─ pow2_is_multiple_of               184.1 ns      │ 2.431 µs      │ 187.2 ns      │ 203 ns        │ 1000    │ 64000
│  │  ├─ pow2_is_multiple_of_const         39.24 ns      │ 615 ns        │ 43.14 ns      │ 46.23 ns      │ 1000    │ 256000
│  │  ├─ pow2_is_multiple_of_reuse         179.4 ns      │ 2.296 µs      │ 184.1 ns      │ 198.7 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem                          188.8 ns      │ 2.488 µs      │ 195.1 ns      │ 209 ns        │ 1000    │ 64000
│  │  ├─ pow2_rem_const                    49.39 ns      │ 602.1 ns      │ 53.3 ns       │ 55.01 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem_floor                    190.4 ns      │ 270.1 ns      │ 195.1 ns      │ 196 ns        │ 1000    │ 64000
│  │  ├─ pow2_rem_floor_const              49.79 ns      │ 113 ns        │ 53.3 ns       │ 54.09 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem_floor_reuse              52.52 ns      │ 109.1 ns      │ 57.6 ns       │ 58.01 ns      │ 1000    │ 256000
│  │  ├─ pow2_rem_reuse                    50.57 ns      │ 128.6 ns      │ 56.82 ns      │ 57.58 ns      │ 1000    │ 256000
│  │  ├─ std_rem                           881 ns        │ 1.156 µs      │ 893.5 ns      │ 894.4 ns      │ 1000    │ 16000
│  │  ├─ std_rem_const                     49.79 ns      │ 324 ns        │ 54.08 ns      │ 54.83 ns      │ 1000    │ 256000
│  │  ├─ std_rem_reuse                     849.7 ns      │ 1.131 µs      │ 862.2 ns      │ 867.3 ns      │ 1000    │ 16000
│  │  ├─ unb_pow2_is_multiple_of           181 ns        │ 366.9 ns      │ 184.1 ns      │ 185.3 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_is_multiple_of_const     41.19 ns      │ 68.54 ns      │ 43.93 ns      │ 44.29 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_is_multiple_of_reuse     181 ns        │ 377.9 ns      │ 185.7 ns      │ 185.6 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_rem                      193.5 ns      │ 385.7 ns      │ 196.6 ns      │ 197.6 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_rem_const                51.74 ns      │ 106 ns        │ 54.47 ns      │ 55.22 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_rem_floor                191.9 ns      │ 385.7 ns      │ 198.2 ns      │ 210.6 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_rem_floor_const          51.35 ns      │ 105.2 ns      │ 54.47 ns      │ 55.23 ns      │ 1000    │ 256000
│  │  ├─ unb_pow2_rem_floor_reuse          54.86 ns      │ 148.2 ns      │ 59.16 ns      │ 60.26 ns      │ 1000    │ 256000
│  │  ╰─ unb_pow2_rem_reuse                54.47 ns      │ 99.39 ns      │ 58.77 ns      │ 59.29 ns      │ 1000    │ 256000
│  ├─ u32                                                │               │               │               │         │
│  │  ├─ baseline_identity                 43.54 ns      │ 393.9 ns      │ 51.35 ns      │ 51.73 ns      │ 1000    │ 256000
│  │  ├─ pow2_is_multiple_of               452.9 ns      │ 5.087 µs      │ 465.4 ns      │ 498.9 ns      │ 1000    │ 32000
│  │  ├─ pow2_is_multiple_of_const         85.72 ns      │ 1.723 µs      │ 97.44 ns      │ 106.8 ns      │ 1000    │ 128000
│  │  ├─ pow2_is_multiple_of_reuse         427.9 ns      │ 865.4 ns      │ 437.2 ns      │ 437.6 ns      │ 1000    │ 32000
│  │  ├─ pow2_rem                          268.5 ns      │ 370.1 ns      │ 273.2 ns      │ 275.7 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem_const                    91.97 ns      │ 1.32 µs       │ 99 ns         │ 105.3 ns      │ 1000    │ 128000
│  │  ├─ pow2_rem_floor                    268.5 ns      │ 2.141 µs      │ 273.2 ns      │ 282.4 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem_floor_const              90.41 ns      │ 196.6 ns      │ 99.79 ns      │ 101 ns        │ 1000    │ 128000
│  │  ├─ pow2_rem_floor_reuse              97.44 ns      │ 327.9 ns      │ 108.3 ns      │ 110.2 ns      │ 1000    │ 128000
│  │  ├─ pow2_rem_reuse                    96.66 ns      │ 1.352 µs      │ 109.1 ns      │ 114.7 ns      │ 1000    │ 128000
│  │  ├─ std_rem                           1.324 µs      │ 20.94 µs      │ 1.349 µs      │ 1.437 µs      │ 1000    │ 8000
│  │  ├─ std_rem_const                     94.32 ns      │ 1.159 µs      │ 99 ns         │ 101.3 ns      │ 1000    │ 128000
│  │  ├─ std_rem_reuse                     1.299 µs      │ 2.787 µs      │ 1.312 µs      │ 1.32 µs       │ 1000    │ 8000
│  │  ├─ unb_pow2_is_multiple_of           456 ns        │ 627.9 ns      │ 465.4 ns      │ 466.3 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_is_multiple_of_const     90.41 ns      │ 163.8 ns      │ 102.1 ns      │ 102.4 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_is_multiple_of_reuse     431 ns        │ 1.799 µs      │ 440.4 ns      │ 444 ns        │ 1000    │ 32000
│  │  ├─ unb_pow2_rem                      276.3 ns      │ 2.423 µs      │ 284.1 ns      │ 304.2 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_rem_const                94.32 ns      │ 1.289 µs      │ 101.3 ns      │ 106.8 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_rem_floor                276.3 ns      │ 2.416 µs      │ 284.1 ns      │ 303.2 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_rem_floor_const          92.75 ns      │ 1.175 µs      │ 100.5 ns      │ 106.4 ns      │ 1000    │ 128000
│  │  ├─ unb_pow2_rem_floor_reuse          100.5 ns      │ 899.7 ns      │ 116.9 ns      │ 122.9 ns      │ 1000    │ 128000
│  │  ╰─ unb_pow2_rem_reuse                98.22 ns      │ 1.394 µs      │ 114.6 ns      │ 119.8 ns      │ 1000    │ 128000
│  ├─ u64                                                │               │               │               │         │
│  │  ├─ baseline_identity                 84.16 ns      │ 246.6 ns      │ 103.6 ns      │ 108 ns        │ 1000    │ 128000
│  │  ├─ pow2_is_multiple_of               349.7 ns      │ 1.496 µs      │ 365.4 ns      │ 457.2 ns      │ 1000    │ 32000
│  │  ├─ pow2_is_multiple_of_const         263.8 ns      │ 846.6 ns      │ 270.1 ns      │ 302.5 ns      │ 1000    │ 64000
│  │  ├─ pow2_is_multiple_of_reuse         315.4 ns      │ 918.5 ns      │ 324.7 ns      │ 337.6 ns      │ 1000    │ 32000
│  │  ├─ pow2_rem                          462.2 ns      │ 1.084 µs      │ 474.7 ns      │ 482.9 ns      │ 1000    │ 32000
│  │  ├─ pow2_rem_const                    188.8 ns      │ 2.463 µs      │ 210.7 ns      │ 227.3 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem_floor                    456 ns        │ 5.074 µs      │ 471.6 ns      │ 517.3 ns      │ 1000    │ 32000
│  │  ├─ pow2_rem_floor_const              187.2 ns      │ 2.512 µs      │ 207.6 ns      │ 214.6 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem_floor_reuse              193.5 ns      │ 743.5 ns      │ 216.9 ns      │ 219.4 ns      │ 1000    │ 64000
│  │  ├─ pow2_rem_reuse                    195.1 ns      │ 357.6 ns      │ 212.2 ns      │ 215.4 ns      │ 1000    │ 64000
│  │  ├─ std_rem                           1.662 µs      │ 2.249 µs      │ 1.712 µs      │ 1.712 µs      │ 1000    │ 8000
│  │  ├─ std_rem_const                     187.2 ns      │ 710.7 ns      │ 207.6 ns      │ 226.9 ns      │ 1000    │ 64000
│  │  ├─ std_rem_reuse                     1.549 µs      │ 4.124 µs      │ 1.562 µs      │ 1.58 µs       │ 1000    │ 8000
│  │  ├─ unb_pow2_is_multiple_of           349.7 ns      │ 2.768 µs      │ 362.2 ns      │ 372.1 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_is_multiple_of_const     265.4 ns      │ 749.7 ns      │ 273.2 ns      │ 306.9 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_is_multiple_of_reuse     321.6 ns      │ 806 ns        │ 327.9 ns      │ 329.3 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_rem                      502.9 ns      │ 977.9 ns      │ 512.2 ns      │ 514.2 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_rem_const                187.2 ns      │ 429.4 ns      │ 202.9 ns      │ 205.4 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_rem_floor                499.7 ns      │ 959.1 ns      │ 512.2 ns      │ 528.5 ns      │ 1000    │ 32000
│  │  ├─ unb_pow2_rem_floor_const          188.8 ns      │ 445.1 ns      │ 207.6 ns      │ 211.5 ns      │ 1000    │ 64000
│  │  ├─ unb_pow2_rem_floor_reuse          193.5 ns      │ 515.4 ns      │ 218.5 ns      │ 224.7 ns      │ 1000    │ 64000
│  │  ╰─ unb_pow2_rem_reuse                195.1 ns      │ 410.7 ns      │ 216.9 ns      │ 224.3 ns      │ 1000    │ 64000
│  ╰─ u128                                               │               │               │               │         │
│     ├─ baseline_identity                 229.4 ns      │ 2.274 µs      │ 270.1 ns      │ 287.7 ns      │ 1000    │ 64000
│     ├─ pow2_is_multiple_of               584.1 ns      │ 5.277 µs      │ 602.9 ns      │ 655.7 ns      │ 1000    │ 32000
│     ├─ pow2_is_multiple_of_const         285.7 ns      │ 460.7 ns      │ 299.7 ns      │ 305.1 ns      │ 1000    │ 64000
│     ├─ pow2_is_multiple_of_reuse         565.4 ns      │ 1.002 µs      │ 577.9 ns      │ 584.2 ns      │ 1000    │ 32000
│     ├─ pow2_rem                          993.5 ns      │ 1.424 µs      │ 1.018 µs      │ 1.022 µs      │ 1000    │ 16000
│     ├─ pow2_rem_const                    443.5 ns      │ 1.237 µs      │ 474.7 ns      │ 480.1 ns      │ 1000    │ 32000
│     ├─ pow2_rem_floor                    999.7 ns      │ 1.524 µs      │ 1.031 µs      │ 1.038 µs      │ 1000    │ 16000
│     ├─ pow2_rem_floor_const              434.1 ns      │ 1.484 µs      │ 471.6 ns      │ 489.9 ns      │ 1000    │ 32000
│     ├─ pow2_rem_floor_reuse              681 ns        │ 1.112 µs      │ 706 ns        │ 712.2 ns      │ 1000    │ 16000
│     ├─ pow2_rem_reuse                    674.7 ns      │ 1.137 µs      │ 699.7 ns      │ 707.4 ns      │ 1000    │ 16000
│     ├─ std_rem                           4.174 µs      │ 5.399 µs      │ 4.249 µs      │ 4.252 µs      │ 1000    │ 4000
│     ├─ std_rem_const                     434.1 ns      │ 1.031 µs      │ 462.2 ns      │ 476.5 ns      │ 1000    │ 32000
│     ├─ std_rem_reuse                     4.324 µs      │ 24.04 µs      │ 4.399 µs      │ 4.474 µs      │ 1000    │ 4000
│     ├─ unb_pow2_is_multiple_of           593.5 ns      │ 1.243 µs      │ 612.2 ns      │ 740.9 ns      │ 1000    │ 16000
│     ├─ unb_pow2_is_multiple_of_const     281 ns        │ 593.5 ns      │ 291.9 ns      │ 302.4 ns      │ 1000    │ 64000
│     ├─ unb_pow2_is_multiple_of_reuse     562.2 ns      │ 1.365 µs      │ 574.7 ns      │ 601.5 ns      │ 1000    │ 32000
│     ├─ unb_pow2_rem                      1.068 µs      │ 2.212 µs      │ 1.093 µs      │ 1.14 µs       │ 1000    │ 16000
│     ├─ unb_pow2_rem_const                440.4 ns      │ 4.906 µs      │ 468.5 ns      │ 497.7 ns      │ 1000    │ 32000
│     ├─ unb_pow2_rem_floor                1.074 µs      │ 9.549 µs      │ 1.099 µs      │ 1.172 µs      │ 1000    │ 16000
│     ├─ unb_pow2_rem_floor_const          437.2 ns      │ 5.084 µs      │ 471.6 ns      │ 505.7 ns      │ 1000    │ 32000
│     ├─ unb_pow2_rem_floor_reuse          681 ns        │ 1.381 µs      │ 712.2 ns      │ 723 ns        │ 1000    │ 16000
│     ╰─ unb_pow2_rem_reuse                681 ns        │ 1.087 µs      │ 712.2 ns      │ 717.6 ns      │ 1000    │ 16000
╰─ round                                                 │               │               │               │         │
   ├─ i8                                                 │               │               │               │         │
   │  ├─ baseline_identity                 12.09 ns      │ 59.36 ns      │ 12.58 ns      │ 13.2 ns       │ 1000    │ 1024000
   │  ├─ pow2_ceil_to_multiple             99 ns         │ 226.3 ns      │ 101.3 ns      │ 102.1 ns      │ 1000    │ 128000
   │  ├─ pow2_ceil_to_multiple_const       26.93 ns      │ 58.18 ns      │ 28.69 ns      │ 28.95 ns      │ 1000    │ 512000
   │  ├─ pow2_ceil_to_multiple_reuse       28.89 ns      │ 67.56 ns      │ 31.04 ns      │ 31.74 ns      │ 1000    │ 512000
   │  ├─ pow2_floor_to_multiple            95.1 ns       │ 131 ns        │ 97.44 ns      │ 97.7 ns       │ 1000    │ 128000
   │  ├─ pow2_floor_to_multiple_const      24.59 ns      │ 338.4 ns      │ 26.93 ns      │ 29.64 ns      │ 1000    │ 512000
   │  ├─ pow2_floor_to_multiple_reuse      27.71 ns      │ 330.2 ns      │ 29.28 ns      │ 31.75 ns      │ 1000    │ 512000
   │  ├─ pow2_round_to_multiple            125.5 ns      │ 175.5 ns      │ 127.9 ns      │ 128.2 ns      │ 1000    │ 128000
   │  ├─ pow2_round_to_multiple_const      34.36 ns      │ 324.5 ns      │ 38.07 ns      │ 39.25 ns      │ 1000    │ 512000
   │  ├─ pow2_round_to_multiple_reuse      32.21 ns      │ 72.83 ns      │ 34.94 ns      │ 36.08 ns      │ 1000    │ 256000
   │  ├─ std_div_mul                       868.5 ns      │ 1.181 µs      │ 887.2 ns      │ 890 ns        │ 1000    │ 16000
   │  ├─ std_div_mul_const                 35.33 ns      │ 100.5 ns      │ 38.46 ns      │ 39.23 ns      │ 1000    │ 512000
   │  ├─ std_div_mul_reuse                 849.7 ns      │ 2.056 µs      │ 868.5 ns      │ 869.6 ns      │ 1000    │ 16000
   │  ├─ unb_pow2_ceil_to_multiple         102.1 ns      │ 1.258 µs      │ 106 ns        │ 118.6 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_ceil_to_multiple_const   27.13 ns      │ 328.3 ns      │ 30.06 ns      │ 31.3 ns       │ 1000    │ 512000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   28.89 ns      │ 329.4 ns      │ 31.43 ns      │ 34.43 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_floor_to_multiple        99.79 ns      │ 1.313 µs      │ 102.1 ns      │ 109.4 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_floor_to_multiple_const  25.76 ns      │ 318.5 ns      │ 26.74 ns      │ 28.72 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_floor_to_multiple_reuse  27.71 ns      │ 284.5 ns      │ 29.28 ns      │ 30.59 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_round_to_multiple        131.8 ns      │ 170.8 ns      │ 134.1 ns      │ 134.4 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_round_to_multiple_const  33.96 ns      │ 74 ns         │ 36.11 ns      │ 36.74 ns      │ 1000    │ 512000
   │  ╰─ unb_pow2_round_to_multiple_reuse  39.63 ns      │ 73.61 ns      │ 42.36 ns      │ 43.04 ns      │ 1000    │ 512000
   ├─ i16                                                │               │               │               │         │
   │  ├─ baseline_identity                 24.39 ns      │ 333.1 ns      │ 27.91 ns      │ 31.04 ns      │ 1000    │ 512000
   │  ├─ pow2_ceil_to_multiple             196.6 ns      │ 382.6 ns      │ 199.7 ns      │ 201.3 ns      │ 1000    │ 64000
   │  ├─ pow2_ceil_to_multiple_const       54.47 ns      │ 143.1 ns      │ 58.38 ns      │ 59.01 ns      │ 1000    │ 256000
   │  ├─ pow2_ceil_to_multiple_reuse       57.21 ns      │ 107.2 ns      │ 63.46 ns      │ 64.32 ns      │ 1000    │ 256000
   │  ├─ pow2_floor_to_multiple            190.4 ns      │ 441.9 ns      │ 195.1 ns      │ 199.4 ns      │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple_const      48.61 ns      │ 138.4 ns      │ 56.04 ns      │ 57.89 ns      │ 1000    │ 256000
   │  ├─ pow2_floor_to_multiple_reuse      51.74 ns      │ 108.7 ns      │ 55.64 ns      │ 56.35 ns      │ 1000    │ 256000
   │  ├─ pow2_round_to_multiple            223.2 ns      │ 2.699 µs      │ 231 ns        │ 245.1 ns      │ 1000    │ 64000
   │  ├─ pow2_round_to_multiple_const      61.89 ns      │ 118.1 ns      │ 66.19 ns      │ 66.98 ns      │ 1000    │ 256000
   │  ├─ pow2_round_to_multiple_reuse      68.14 ns      │ 613 ns        │ 74.39 ns      │ 77.67 ns      │ 1000    │ 256000
   │  ├─ std_div_mul                       2.574 µs      │ 41.27 µs      │ 2.599 µs      │ 2.785 µs      │ 1000    │ 4000
   │  ├─ std_div_mul_const                 63.07 ns      │ 649 ns        │ 66.97 ns      │ 71.51 ns      │ 1000    │ 256000
   │  ├─ std_div_mul_reuse                 2.624 µs      │ 41.02 µs      │ 2.674 µs      │ 2.898 µs      │ 1000    │ 4000
   │  ├─ unb_pow2_ceil_to_multiple         199.7 ns      │ 2.248 µs      │ 204.4 ns      │ 220.1 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_ceil_to_multiple_const   54.86 ns      │ 693.5 ns      │ 58.38 ns      │ 61.49 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   57.6 ns       │ 118.5 ns      │ 60.72 ns      │ 61.29 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_floor_to_multiple        193.5 ns      │ 266.9 ns      │ 196.6 ns      │ 198.2 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple_const  53.69 ns      │ 113.4 ns      │ 56.43 ns      │ 56.73 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_floor_to_multiple_reuse  56.43 ns      │ 145.8 ns      │ 62.68 ns      │ 64.87 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_round_to_multiple        229.4 ns      │ 301.3 ns      │ 234.1 ns      │ 235.2 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_round_to_multiple_const  64.63 ns      │ 182.6 ns      │ 70.1 ns       │ 71.13 ns      │ 1000    │ 256000
   │  ╰─ unb_pow2_round_to_multiple_reuse  70.88 ns      │ 203.3 ns      │ 81.04 ns      │ 84.62 ns      │ 1000    │ 256000
   ├─ i32                                                │               │               │               │         │
   │  ├─ baseline_identity                 43.54 ns      │ 391.1 ns      │ 51.74 ns      │ 52.47 ns      │ 1000    │ 256000
   │  ├─ pow2_ceil_to_multiple             281 ns        │ 2.638 µs      │ 288.8 ns      │ 308.5 ns      │ 1000    │ 64000
   │  ├─ pow2_ceil_to_multiple_const       100.5 ns      │ 1.309 µs      │ 110.7 ns      │ 114 ns        │ 1000    │ 128000
   │  ├─ pow2_ceil_to_multiple_reuse       104.4 ns      │ 214.6 ns      │ 117.7 ns      │ 118.6 ns      │ 1000    │ 128000
   │  ├─ pow2_floor_to_multiple            265.4 ns      │ 391.9 ns      │ 271.6 ns      │ 274.3 ns      │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple_const      92.75 ns      │ 266.1 ns      │ 101.3 ns      │ 110.8 ns      │ 1000    │ 128000
   │  ├─ pow2_floor_to_multiple_reuse      98.22 ns      │ 293.5 ns      │ 113.8 ns      │ 122.3 ns      │ 1000    │ 128000
   │  ├─ pow2_round_to_multiple            285.7 ns      │ 638.8 ns      │ 291.9 ns      │ 304.5 ns      │ 1000    │ 64000
   │  ├─ pow2_round_to_multiple_const      124.7 ns      │ 263 ns        │ 133.3 ns      │ 137.5 ns      │ 1000    │ 128000
   │  ├─ pow2_round_to_multiple_reuse      157.6 ns      │ 265.4 ns      │ 163.8 ns      │ 165.1 ns      │ 1000    │ 64000
   │  ├─ std_div_mul                       1.337 µs      │ 9.062 µs      │ 1.374 µs      │ 1.379 µs      │ 1000    │ 8000
   │  ├─ std_div_mul_const                 127.9 ns      │ 221.6 ns      │ 134.9 ns      │ 136.7 ns      │ 1000    │ 128000
   │  ├─ std_div_mul_reuse                 1.299 µs      │ 1.824 µs      │ 1.312 µs      │ 1.314 µs      │ 1000    │ 8000
   │  ├─ unb_pow2_ceil_to_multiple         296.6 ns      │ 498.2 ns      │ 306 ns        │ 308.6 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_ceil_to_multiple_const   103.6 ns      │ 221.6 ns      │ 113.8 ns      │ 116.4 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   105.2 ns      │ 302.1 ns      │ 121.6 ns      │ 127.7 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_floor_to_multiple        277.9 ns      │ 407.6 ns      │ 284.1 ns      │ 285.8 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple_const  92.75 ns      │ 188 ns        │ 99.79 ns      │ 101.6 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_floor_to_multiple_reuse  98.22 ns      │ 322.4 ns      │ 113.8 ns      │ 115.4 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_round_to_multiple        298.2 ns      │ 510.7 ns      │ 306 ns        │ 307.7 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_round_to_multiple_const  110.7 ns      │ 276.3 ns      │ 120.1 ns      │ 120.7 ns      │ 1000    │ 64000
   │  ╰─ unb_pow2_round_to_multiple_reuse  156 ns        │ 1.599 µs      │ 166.9 ns      │ 170.7 ns      │ 1000    │ 64000
   ├─ i64                                                │               │               │               │         │
   │  ├─ baseline_identity                 82.6 ns       │ 301.3 ns      │ 100.5 ns      │ 103 ns        │ 1000    │ 128000
   │  ├─ pow2_ceil_to_multiple             499.7 ns      │ 843.5 ns      │ 518.5 ns      │ 523.1 ns      │ 1000    │ 16000
   │  ├─ pow2_ceil_to_multiple_const       207.6 ns      │ 573.2 ns      │ 234.1 ns      │ 247 ns        │ 1000    │ 64000
   │  ├─ pow2_ceil_to_multiple_reuse       204.4 ns      │ 629.4 ns      │ 227.9 ns      │ 232.6 ns      │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple            415.4 ns      │ 834.1 ns      │ 431 ns        │ 440.3 ns      │ 1000    │ 32000
   │  ├─ pow2_floor_to_multiple_const      184.1 ns      │ 434.1 ns      │ 201.3 ns      │ 205.5 ns      │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple_reuse      191.9 ns      │ 2.507 µs      │ 210.7 ns      │ 216.5 ns      │ 1000    │ 64000
   │  ├─ pow2_round_to_multiple            799.7 ns      │ 1.249 µs      │ 824.7 ns      │ 832.3 ns      │ 1000    │ 16000
   │  ├─ pow2_round_to_multiple_const      235.7 ns      │ 2.871 µs      │ 268.5 ns      │ 279.2 ns      │ 1000    │ 64000
   │  ├─ pow2_round_to_multiple_reuse      471.6 ns      │ 1.812 µs      │ 481 ns        │ 490.2 ns      │ 1000    │ 32000
   │  ├─ std_div_mul                       1.699 µs      │ 27.04 µs      │ 1.737 µs      │ 1.855 µs      │ 1000    │ 8000
   │  ├─ std_div_mul_const                 234.1 ns      │ 1.713 µs      │ 254.4 ns      │ 261.5 ns      │ 1000    │ 64000
   │  ├─ std_div_mul_reuse                 1.562 µs      │ 2.187 µs      │ 1.587 µs      │ 1.588 µs      │ 1000    │ 8000
   │  ├─ unb_pow2_ceil_to_multiple         531 ns        │ 1.256 µs      │ 543.5 ns      │ 547.3 ns      │ 1000    │ 32000
   │  ├─ unb_pow2_ceil_to_multiple_const   201.3 ns      │ 452.9 ns      │ 226.3 ns      │ 230.7 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   204.4 ns      │ 2.302 µs      │ 226.3 ns      │ 241.1 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple        496.6 ns      │ 5.459 µs      │ 512.2 ns      │ 557.4 ns      │ 1000    │ 32000
   │  ├─ unb_pow2_floor_to_multiple_const  184.1 ns      │ 427.9 ns      │ 201.3 ns      │ 203.7 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple_reuse  191.9 ns      │ 2.631 µs      │ 212.2 ns      │ 223.4 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_round_to_multiple        993.5 ns      │ 9.862 µs      │ 1.018 µs      │ 1.104 µs      │ 1000    │ 16000
   │  ├─ unb_pow2_round_to_multiple_const  245.1 ns      │ 2.632 µs      │ 271.6 ns      │ 294 ns        │ 1000    │ 64000
   │  ╰─ unb_pow2_round_to_multiple_reuse  484.1 ns      │ 1.043 µs      │ 499.7 ns      │ 523.8 ns      │ 1000    │ 32000
   ├─ i128                                               │               │               │               │         │
   │  ├─ baseline_identity                 207.6 ns      │ 531 ns        │ 290.4 ns      │ 293.2 ns      │ 1000    │ 64000
   │  ├─ pow2_ceil_to_multiple             1.149 µs      │ 2.024 µs      │ 1.187 µs      │ 1.201 µs      │ 1000    │ 16000
   │  ├─ pow2_ceil_to_multiple_const       649.7 ns      │ 974.7 ns      │ 668.5 ns      │ 672.8 ns      │ 1000    │ 16000
   │  ├─ pow2_ceil_to_multiple_reuse       712.2 ns      │ 1.499 µs      │ 743.5 ns      │ 748.8 ns      │ 1000    │ 16000
   │  ├─ pow2_floor_to_multiple            881 ns        │ 1.231 µs      │ 906 ns        │ 910 ns        │ 1000    │ 16000
   │  ├─ pow2_floor_to_multiple_const      437.2 ns      │ 1.177 µs      │ 477.9 ns      │ 491.4 ns      │ 1000    │ 32000
   │  ├─ pow2_floor_to_multiple_reuse      637.2 ns      │ 2.518 µs      │ 662.2 ns      │ 721.8 ns      │ 1000    │ 16000
   │  ├─ pow2_round_to_multiple            2.349 µs      │ 4.424 µs      │ 2.381 µs      │ 2.476 µs      │ 1000    │ 8000
   │  ├─ pow2_round_to_multiple_const      918.5 ns      │ 2.793 µs      │ 937.2 ns      │ 986.9 ns      │ 1000    │ 16000
   │  ├─ pow2_round_to_multiple_reuse      1.043 µs      │ 3.868 µs      │ 1.062 µs      │ 1.115 µs      │ 1000    │ 16000
   │  ├─ std_div_mul                       10.19 µs      │ 24.69 µs      │ 10.39 µs      │ 10.55 µs      │ 1000    │ 1000
   │  ├─ std_div_mul_const                 831 ns        │ 2.568 µs      │ 856 ns        │ 888.6 ns      │ 1000    │ 16000
   │  ├─ std_div_mul_reuse                 9.249 µs      │ 16.99 µs      │ 9.399 µs      │ 9.581 µs      │ 1000    │ 2000
   │  ├─ unb_pow2_ceil_to_multiple         1.187 µs      │ 2.456 µs      │ 1.231 µs      │ 1.29 µs       │ 1000    │ 16000
   │  ├─ unb_pow2_ceil_to_multiple_const   681 ns        │ 1.687 µs      │ 699.7 ns      │ 724.9 ns      │ 1000    │ 16000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   693.5 ns      │ 1.731 µs      │ 712.2 ns      │ 746.1 ns      │ 1000    │ 16000
   │  ├─ unb_pow2_floor_to_multiple        843.5 ns      │ 2.131 µs      │ 874.7 ns      │ 925.6 ns      │ 1000    │ 16000
   │  ├─ unb_pow2_floor_to_multiple_const  446.6 ns      │ 899.7 ns      │ 481 ns        │ 489.5 ns      │ 1000    │ 32000
   │  ├─ unb_pow2_floor_to_multiple_reuse  624.7 ns      │ 1.662 µs      │ 649.7 ns      │ 696.6 ns      │ 1000    │ 16000
   │  ├─ unb_pow2_round_to_multiple        2.349 µs      │ 5.924 µs      │ 2.374 µs      │ 2.495 µs      │ 1000    │ 4000
   │  ├─ unb_pow2_round_to_multiple_const  912.2 ns      │ 2.643 µs      │ 943.5 ns      │ 988.9 ns      │ 1000    │ 16000
   │  ╰─ unb_pow2_round_to_multiple_reuse  1.037 µs      │ 2.543 µs      │ 1.081 µs      │ 1.129 µs      │ 1000    │ 16000
   ├─ u8                                                 │               │               │               │         │
   │  ├─ baseline_identity                 11.89 ns      │ 45.29 ns      │ 12.48 ns      │ 13 ns         │ 1000    │ 1024000
   │  ├─ pow2_ceil_to_multiple             98.22 ns      │ 177.9 ns      │ 101.3 ns      │ 101.9 ns      │ 1000    │ 128000
   │  ├─ pow2_ceil_to_multiple_const       26.93 ns      │ 346.6 ns      │ 31.23 ns      │ 35.11 ns      │ 1000    │ 512000
   │  ├─ pow2_ceil_to_multiple_reuse       29.28 ns      │ 324.3 ns      │ 31.23 ns      │ 33.29 ns      │ 1000    │ 512000
   │  ├─ pow2_floor_to_multiple            94.32 ns      │ 1.315 µs      │ 97.44 ns      │ 104 ns        │ 1000    │ 128000
   │  ├─ pow2_floor_to_multiple_const      24.79 ns      │ 67.36 ns      │ 25.76 ns      │ 26.78 ns      │ 1000    │ 512000
   │  ├─ pow2_floor_to_multiple_reuse      26.93 ns      │ 71.66 ns      │ 28.5 ns       │ 29.06 ns      │ 1000    │ 512000
   │  ├─ pow2_round_to_multiple            114.6 ns      │ 195.1 ns      │ 116.1 ns      │ 117.4 ns      │ 1000    │ 128000
   │  ├─ pow2_round_to_multiple_const      26.35 ns      │ 103.6 ns      │ 29.86 ns      │ 30.7 ns       │ 1000    │ 512000
   │  ├─ pow2_round_to_multiple_reuse      28.5 ns       │ 317.1 ns      │ 30.64 ns      │ 34.48 ns      │ 1000    │ 512000
   │  ├─ std_div_mul                       862.2 ns      │ 9.699 µs      │ 874.7 ns      │ 935.2 ns      │ 1000    │ 16000
   │  ├─ std_div_mul_const                 24.39 ns      │ 98.22 ns      │ 25.76 ns      │ 26.67 ns      │ 1000    │ 512000
   │  ├─ std_div_mul_reuse                 849.7 ns      │ 1.299 µs      │ 862.2 ns      │ 866.3 ns      │ 1000    │ 16000
   │  ├─ unb_pow2_ceil_to_multiple         102.9 ns      │ 295.1 ns      │ 106 ns        │ 106.9 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_ceil_to_multiple_const   27.32 ns      │ 325.7 ns      │ 31.23 ns      │ 32.47 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   28.5 ns       │ 309.9 ns      │ 31.62 ns      │ 33.2 ns       │ 1000    │ 512000
   │  ├─ unb_pow2_floor_to_multiple        98.22 ns      │ 391.1 ns      │ 102.1 ns      │ 106.8 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_floor_to_multiple_const  24.79 ns      │ 204.8 ns      │ 26.93 ns      │ 27.78 ns      │ 1000    │ 512000
   │  ├─ unb_pow2_floor_to_multiple_reuse  26.93 ns      │ 93.54 ns      │ 29.08 ns      │ 31.7 ns       │ 1000    │ 512000
   │  ├─ unb_pow2_round_to_multiple        120.8 ns      │ 309.1 ns      │ 123.2 ns      │ 124.3 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_round_to_multiple_const  26.74 ns      │ 53.89 ns      │ 28.69 ns      │ 29.21 ns      │ 1000    │ 512000
   │  ╰─ unb_pow2_round_to_multiple_reuse  29.08 ns      │ 60.33 ns      │ 31.04 ns      │ 31.38 ns      │ 1000    │ 512000
   ├─ u16                                                │               │               │               │         │
   │  ├─ baseline_identity                 26.54 ns      │ 117.5 ns      │ 27.71 ns      │ 28.94 ns      │ 1000    │ 512000
   │  ├─ pow2_ceil_to_multiple             195.1 ns      │ 534.1 ns      │ 199.7 ns      │ 206.1 ns      │ 1000    │ 64000
   │  ├─ pow2_ceil_to_multiple_const       52.13 ns      │ 140 ns        │ 57.99 ns      │ 59.58 ns      │ 1000    │ 256000
   │  ├─ pow2_ceil_to_multiple_reuse       57.6 ns       │ 106.8 ns      │ 62.29 ns      │ 63.42 ns      │ 1000    │ 256000
   │  ├─ pow2_floor_to_multiple            190.4 ns      │ 260.7 ns      │ 195.1 ns      │ 195.7 ns      │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple_const      49 ns         │ 147.4 ns      │ 52.91 ns      │ 53.61 ns      │ 1000    │ 256000
   │  ├─ pow2_floor_to_multiple_reuse      52.13 ns      │ 98.61 ns      │ 57.6 ns       │ 58.32 ns      │ 1000    │ 256000
   │  ├─ pow2_round_to_multiple            209.1 ns      │ 279.4 ns      │ 212.2 ns      │ 213.4 ns      │ 1000    │ 64000
   │  ├─ pow2_round_to_multiple_const      51.35 ns      │ 123.2 ns      │ 54.86 ns      │ 55.85 ns      │ 1000    │ 256000
   │  ├─ pow2_round_to_multiple_reuse      56.82 ns      │ 129.8 ns      │ 61.89 ns      │ 63.26 ns      │ 1000    │ 256000
   │  ├─ std_div_mul                       874.7 ns      │ 1.143 µs      │ 887.2 ns      │ 891 ns        │ 1000    │ 16000
   │  ├─ std_div_mul_const                 50.18 ns      │ 127.1 ns      │ 53.69 ns      │ 54.31 ns      │ 1000    │ 256000
   │  ├─ std_div_mul_reuse                 843.5 ns      │ 10.84 µs      │ 862.2 ns      │ 902.5 ns      │ 1000    │ 16000
   │  ├─ unb_pow2_ceil_to_multiple         199.7 ns      │ 2.368 µs      │ 204.4 ns      │ 221.1 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_ceil_to_multiple_const   52.91 ns      │ 640.8 ns      │ 57.21 ns      │ 63.23 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   56.43 ns      │ 235.7 ns      │ 61.5 ns       │ 63.92 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_floor_to_multiple        193.5 ns      │ 321.6 ns      │ 196.6 ns      │ 198.3 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple_const  48.61 ns      │ 109.9 ns      │ 54.08 ns      │ 55.54 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_floor_to_multiple_reuse  55.64 ns      │ 536.8 ns      │ 61.11 ns      │ 63.14 ns      │ 1000    │ 256000
   │  ├─ unb_pow2_round_to_multiple        210.7 ns      │ 570.1 ns      │ 216.9 ns      │ 218 ns        │ 1000    │ 64000
   │  ├─ unb_pow2_round_to_multiple_const  54.08 ns      │ 413.4 ns      │ 56.82 ns      │ 58.29 ns      │ 1000    │ 256000
   │  ╰─ unb_pow2_round_to_multiple_reuse  56.04 ns      │ 672.8 ns      │ 63.46 ns      │ 69.75 ns      │ 1000    │ 256000
   ├─ u32                                                │               │               │               │         │
   │  ├─ baseline_identity                 43.93 ns      │ 139.6 ns      │ 51.74 ns      │ 52.16 ns      │ 1000    │ 256000
   │  ├─ pow2_ceil_to_multiple             282.6 ns      │ 485.7 ns      │ 290.4 ns      │ 294.2 ns      │ 1000    │ 64000
   │  ├─ pow2_ceil_to_multiple_const       101.3 ns      │ 369.3 ns      │ 112.2 ns      │ 119.2 ns      │ 1000    │ 128000
   │  ├─ pow2_ceil_to_multiple_reuse       103.6 ns      │ 241.1 ns      │ 116.9 ns      │ 120.6 ns      │ 1000    │ 128000
   │  ├─ pow2_floor_to_multiple            263.8 ns      │ 810.7 ns      │ 270.1 ns      │ 277.9 ns      │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple_const      92.75 ns      │ 214.6 ns      │ 99.79 ns      │ 101.8 ns      │ 1000    │ 128000
   │  ├─ pow2_floor_to_multiple_reuse      97.44 ns      │ 196.6 ns      │ 109.9 ns      │ 110.6 ns      │ 1000    │ 128000
   │  ├─ pow2_round_to_multiple            215.4 ns      │ 420.1 ns      │ 223.2 ns      │ 226.7 ns      │ 1000    │ 64000
   │  ├─ pow2_round_to_multiple_const      98.22 ns      │ 288 ns        │ 113 ns        │ 115.1 ns      │ 1000    │ 128000
   │  ├─ pow2_round_to_multiple_reuse      107.6 ns      │ 240.4 ns      │ 120.8 ns      │ 123.4 ns      │ 1000    │ 128000
   │  ├─ std_div_mul                       1.337 µs      │ 2.712 µs      │ 1.362 µs      │ 1.362 µs      │ 1000    │ 8000
   │  ├─ std_div_mul_const                 82.6 ns       │ 252.9 ns      │ 88.85 ns      │ 90.02 ns      │ 1000    │ 64000
   │  ├─ std_div_mul_reuse                 1.287 µs      │ 2.587 µs      │ 1.324 µs      │ 1.322 µs      │ 1000    │ 8000
   │  ├─ unb_pow2_ceil_to_multiple         295.1 ns      │ 448.2 ns      │ 302.9 ns      │ 305.6 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_ceil_to_multiple_const   102.9 ns      │ 258.3 ns      │ 112.2 ns      │ 113.8 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   106.8 ns      │ 285.7 ns      │ 120.8 ns      │ 122.7 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_floor_to_multiple        274.7 ns      │ 507.6 ns      │ 284.1 ns      │ 287.2 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple_const  94.32 ns      │ 220.8 ns      │ 100.5 ns      │ 102 ns        │ 1000    │ 128000
   │  ├─ unb_pow2_floor_to_multiple_reuse  98.22 ns      │ 231.8 ns      │ 113 ns        │ 116.5 ns      │ 1000    │ 128000
   │  ├─ unb_pow2_round_to_multiple        226.3 ns      │ 516.9 ns      │ 232.6 ns      │ 234.5 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_round_to_multiple_const  107.6 ns      │ 349 ns        │ 116.9 ns      │ 121.8 ns      │ 1000    │ 128000
   │  ╰─ unb_pow2_round_to_multiple_reuse  105.2 ns      │ 1.376 µs      │ 119.3 ns      │ 133.1 ns      │ 1000    │ 128000
   ├─ u64                                                │               │               │               │         │
   │  ├─ baseline_identity                 82.6 ns       │ 1.269 µs      │ 97.44 ns      │ 102.2 ns      │ 1000    │ 128000
   │  ├─ pow2_ceil_to_multiple             502.9 ns      │ 843.5 ns      │ 518.5 ns      │ 535.6 ns      │ 1000    │ 32000
   │  ├─ pow2_ceil_to_multiple_const       191.9 ns      │ 766.9 ns      │ 224.7 ns      │ 236.6 ns      │ 1000    │ 64000
   │  ├─ pow2_ceil_to_multiple_reuse       207.6 ns      │ 2.574 µs      │ 238.8 ns      │ 251.2 ns      │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple            418.5 ns      │ 1.131 µs      │ 437.2 ns      │ 460.6 ns      │ 1000    │ 32000
   │  ├─ pow2_floor_to_multiple_const      188.8 ns      │ 632.6 ns      │ 204.4 ns      │ 213.9 ns      │ 1000    │ 64000
   │  ├─ pow2_floor_to_multiple_reuse      195.1 ns      │ 431 ns        │ 215.4 ns      │ 222.8 ns      │ 1000    │ 64000
   │  ├─ pow2_round_to_multiple            537.2 ns      │ 1.649 µs      │ 549.7 ns      │ 577.7 ns      │ 1000    │ 16000
   │  ├─ pow2_round_to_multiple_const      202.9 ns      │ 431 ns        │ 221.6 ns      │ 226.3 ns      │ 1000    │ 64000
   │  ├─ pow2_round_to_multiple_reuse      210.7 ns      │ 610.7 ns      │ 241.9 ns      │ 250 ns        │ 1000    │ 64000
   │  ├─ std_div_mul                       1.687 µs      │ 2.512 µs      │ 1.712 µs      │ 1.715 µs      │ 1000    │ 8000
   │  ├─ std_div_mul_const                 184.1 ns      │ 512.2 ns      │ 199.7 ns      │ 203.2 ns      │ 1000    │ 64000
   │  ├─ std_div_mul_reuse                 1.537 µs      │ 2.162 µs      │ 1.562 µs      │ 1.569 µs      │ 1000    │ 8000
   │  ├─ unb_pow2_ceil_to_multiple         524.7 ns      │ 984.1 ns      │ 543.5 ns      │ 544.5 ns      │ 1000    │ 32000
   │  ├─ unb_pow2_ceil_to_multiple_const   201.3 ns      │ 445.1 ns      │ 218.5 ns      │ 221.6 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_ceil_to_multiple_reuse   207.6 ns      │ 627.9 ns      │ 232.6 ns      │ 237.4 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple        499.7 ns      │ 849.7 ns      │ 509.1 ns      │ 515.4 ns      │ 1000    │ 32000
   │  ├─ unb_pow2_floor_to_multiple_const  185.7 ns      │ 391.9 ns      │ 202.9 ns      │ 207.7 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_floor_to_multiple_reuse  196.6 ns      │ 2.31 µs       │ 221.6 ns      │ 230.8 ns      │ 1000    │ 64000
   │  ├─ unb_pow2_round_to_multiple        568.5 ns      │ 5.449 µs      │ 581 ns        │ 626.5 ns      │ 1000    │ 32000
   │  ├─ unb_pow2_round_to_multiple_const  199.7 ns      │ 2.584 µs      │ 221.6 ns      │ 236.4 ns      │ 1000    │ 64000
   │  ╰─ unb_pow2_round_to_multiple_reuse  209.1 ns      │ 2.484 µs      │ 240.4 ns      │ 258.5 ns      │ 1000    │ 64000
   ╰─ u128                                               │               │               │               │         │
      ├─ baseline_identity                 206 ns        │ 2.602 µs      │ 254.4 ns      │ 264 ns        │ 1000    │ 64000
      ├─ pow2_ceil_to_multiple             1.168 µs      │ 2.049 µs      │ 1.218 µs      │ 1.231 µs      │ 1000    │ 16000
      ├─ pow2_ceil_to_multiple_const       615.4 ns      │ 1.274 µs      │ 637.2 ns      │ 650.7 ns      │ 1000    │ 32000
      ├─ pow2_ceil_to_multiple_reuse       668.5 ns      │ 1.018 µs      │ 687.2 ns      │ 691.8 ns      │ 1000    │ 16000
      ├─ pow2_floor_to_multiple            881 ns        │ 1.581 µs      │ 899.7 ns      │ 903.5 ns      │ 1000    │ 16000
      ├─ pow2_floor_to_multiple_const      440.4 ns      │ 4.518 µs      │ 468.5 ns      │ 489.3 ns      │ 1000    │ 32000
      ├─ pow2_floor_to_multiple_reuse      674.7 ns      │ 11.88 µs      │ 712.2 ns      │ 770.2 ns      │ 1000    │ 16000
      ├─ pow2_round_to_multiple            1.874 µs      │ 19.13 µs      │ 1.912 µs      │ 2.05 µs       │ 1000    │ 8000
      ├─ pow2_round_to_multiple_const      606 ns        │ 9.462 µs      │ 624.7 ns      │ 660.7 ns      │ 1000    │ 16000
      ├─ pow2_round_to_multiple_reuse      662.2 ns      │ 8.949 µs      │ 681 ns        │ 720.3 ns      │ 1000    │ 16000
      ├─ std_div_mul                       4.274 µs      │ 46.17 µs      │ 4.399 µs      │ 4.73 µs       │ 1000    │ 4000
      ├─ std_div_mul_const                 427.9 ns      │ 4.527 µs      │ 465.4 ns      │ 488.8 ns      │ 1000    │ 32000
      ├─ std_div_mul_reuse                 4.324 µs      │ 6.099 µs      │ 4.374 µs      │ 4.395 µs      │ 1000    │ 4000
      ├─ unb_pow2_ceil_to_multiple         1.162 µs      │ 1.937 µs      │ 1.193 µs      │ 1.207 µs      │ 1000    │ 16000
      ├─ unb_pow2_ceil_to_multiple_const   606 ns        │ 1.787 µs      │ 631 ns        │ 649.7 ns      │ 1000    │ 16000
      ├─ unb_pow2_ceil_to_multiple_reuse   681 ns        │ 2.493 µs      │ 699.7 ns      │ 744.7 ns      │ 1000    │ 16000
      ├─ unb_pow2_floor_to_multiple        856 ns        │ 1.449 µs      │ 881 ns        │ 905.7 ns      │ 1000    │ 16000
      ├─ unb_pow2_floor_to_multiple_const  434.1 ns      │ 5.259 µs      │ 468.5 ns      │ 506.4 ns      │ 1000    │ 32000
      ├─ unb_pow2_floor_to_multiple_reuse  624.7 ns      │ 3.643 µs      │ 693.5 ns      │ 695.7 ns      │ 1000    │ 16000
      ├─ unb_pow2_round_to_multiple        1.937 µs      │ 3.799 µs      │ 1.962 µs      │ 1.979 µs      │ 1000    │ 8000
      ├─ unb_pow2_round_to_multiple_const  624.7 ns      │ 9.574 µs      │ 643.5 ns      │ 689.8 ns      │ 1000    │ 16000
      ╰─ unb_pow2_round_to_multiple_reuse  687.2 ns      │ 9.031 µs      │ 706 ns        │ 743 ns        │ 1000    │ 16000


```
