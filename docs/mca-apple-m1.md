# Instruction Info:
```
[1]: #uOps
[2]: Latency
[3]: RThroughput
[4]: MayLoad
[5]: MayStore
[6]: HasSideEffects (U)
```
# Resources:
```
[0.0] - CyUnitB
[0.1] - CyUnitB
[1]   - CyUnitBR
[2.0] - CyUnitFloatDiv
[2.1] - CyUnitFloatDiv
[3.0] - CyUnitI
[3.1] - CyUnitI
[3.2] - CyUnitI
[3.3] - CyUnitI
[4]   - CyUnitID
[5]   - CyUnitIM
[6.0] - CyUnitIS
[6.1] - CyUnitIS
[7]   - CyUnitIntDiv
[8.0] - CyUnitLS
[8.1] - CyUnitLS
[9.0] - CyUnitV
[9.1] - CyUnitV
[9.2] - CyUnitV
[10]  - CyUnitVC
[11]  - CyUnitVD
[12.0] - CyUnitVM
[12.1] - CyUnitVM
```
# Functions:
## `ceil_to_multiple_i8_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      9
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.44
IPC:               0.44
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	w8, #-1
 1      1     0.50                        lsl	w8, w8, w1
 1      2     1.00                        sub	w9, w8, w0
 1      2     1.00                        bic	w0, w8, w9

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   2.00   1.00   1.00    -      -     2.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     sub	w9, w8, w0
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     bic	w0, w8, w9
```
## `ceil_to_multiple_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      9
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.56
IPC:               0.56
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	w8, #-1
 1      1     0.25                        and	x9, x1, #0x7
 1      1     0.50                        lsl	w8, w8, w9
 1      2     1.00                        sub	w9, w8, w0
 1      2     1.00                        bic	w0, w8, w9

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   1.00   1.00   3.00    -      -     2.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x9, x1, #0x7
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w9
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     sub	w9, w8, w0
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     bic	w0, w8, w9
```
## `ceil_to_multiple_i16_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      9
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.44
IPC:               0.44
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	w8, #-1
 1      1     0.50                        lsl	w8, w8, w1
 1      2     1.00                        sub	w9, w8, w0
 1      2     1.00                        bic	w0, w8, w9

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   2.00   1.00   1.00    -      -     2.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     sub	w9, w8, w0
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     bic	w0, w8, w9
```
## `ceil_to_multiple_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      9
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.56
IPC:               0.56
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	w8, #-1
 1      1     0.25                        and	x9, x1, #0xf
 1      1     0.50                        lsl	w8, w8, w9
 1      2     1.00                        sub	w9, w8, w0
 1      2     1.00                        bic	w0, w8, w9

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   1.00   1.00   3.00    -      -     2.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x9, x1, #0xf
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w9
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     sub	w9, w8, w0
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     bic	w0, w8, w9
```
## `ceil_to_multiple_i32_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      9
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.44
IPC:               0.44
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	w8, #-1
 1      1     0.50                        lsl	w8, w8, w1
 1      2     1.00                        sub	w9, w8, w0
 1      2     1.00                        bic	w0, w8, w9

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   2.00   1.00   1.00    -      -     2.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     sub	w9, w8, w0
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     bic	w0, w8, w9
```
## `ceil_to_multiple_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      9
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.44
IPC:               0.44
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	w8, #-1
 1      1     0.50                        lsl	w8, w8, w1
 1      2     1.00                        sub	w9, w8, w0
 1      2     1.00                        bic	w0, w8, w9

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   2.00   1.00   1.00    -      -     2.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     sub	w9, w8, w0
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     bic	w0, w8, w9
```
## `ceil_to_multiple_i64_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      9
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.44
IPC:               0.44
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	x8, #-1
 1      1     0.50                        lsl	x8, x8, x1
 1      2     1.00                        sub	x9, x8, x0
 1      2     1.00                        bic	x0, x8, x9

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   2.00   1.00   1.00    -      -     2.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     sub	x9, x8, x0
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     bic	x0, x8, x9
```
## `ceil_to_multiple_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      9
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.44
IPC:               0.44
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	x8, #-1
 1      1     0.50                        lsl	x8, x8, x1
 1      2     1.00                        sub	x9, x8, x0
 1      2     1.00                        bic	x0, x8, x9

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   2.00   1.00   1.00    -      -     2.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     sub	x9, x8, x0
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     bic	x0, x8, x9
```
## `ceil_to_multiple_i128_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      12
Total uOps:        14

Dispatch Width:    6
uOps Per Cycle:    1.17
IPC:               1.17
Block RThroughput: 6.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	x8, #-1
 1      2     1.00                        mvn	w9, w2
 1      1     0.25                        mov	x10, #9223372036854775807
 1      1     0.50                        lsl	x8, x8, x2
 1      1     0.50                        lsr	x9, x10, x9
 1      1     0.25                        and	x10, x2, #0xff
 1      1     0.25                        tst	x10, #0x40
 1      2     1.00                        orr	x9, x8, x9
 1      1     0.25                        csel	x10, xzr, x8, ne
 1      1     0.25                        csel	x8, x8, x9, ne
 1      2     1.00                        subs	x9, x10, x0
 1      1     0.25                        sbc	x11, x8, x1
 1      2     1.00                        bic	x0, x10, x9
 1      2     1.00                        bic	x1, x8, x11

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     3.00   6.00   5.00   5.00    -      -     5.00   7.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	w9, w2
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x10, #9223372036854775807
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x2
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x9, x10, x9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x10, x2, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x10, #0x40
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x9, x8, x9
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x10, xzr, x8, ne
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x8, x8, x9, ne
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     subs	x9, x10, x0
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sbc	x11, x8, x1
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     bic	x0, x10, x9
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     bic	x1, x8, x11
```
## `ceil_to_multiple_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      12
Total uOps:        14

Dispatch Width:    6
uOps Per Cycle:    1.17
IPC:               1.17
Block RThroughput: 6.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	x8, #-1
 1      2     1.00                        mvn	w9, w2
 1      1     0.25                        mov	x10, #9223372036854775807
 1      1     0.50                        lsl	x8, x8, x2
 1      1     0.50                        lsr	x9, x10, x9
 1      1     0.25                        and	x10, x2, #0x7f
 1      1     0.25                        tst	x10, #0x40
 1      2     1.00                        orr	x9, x8, x9
 1      1     0.25                        csel	x10, xzr, x8, ne
 1      1     0.25                        csel	x8, x8, x9, ne
 1      2     1.00                        subs	x9, x10, x0
 1      1     0.25                        sbc	x11, x8, x1
 1      2     1.00                        bic	x0, x10, x9
 1      2     1.00                        bic	x1, x8, x11

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     3.00   6.00   5.00   5.00    -      -     5.00   7.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	w9, w2
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x10, #9223372036854775807
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x2
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x9, x10, x9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x10, x2, #0x7f
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x10, #0x40
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x9, x8, x9
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x10, xzr, x8, ne
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x8, x8, x9, ne
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     subs	x9, x10, x0
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sbc	x11, x8, x1
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     bic	x0, x10, x9
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     bic	x1, x8, x11
```
## `checked_ceil_to_multiple_i8_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      12
Total uOps:        8

Dispatch Width:    6
uOps Per Cycle:    0.67
IPC:               0.67
Block RThroughput: 5.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	w8, #-1
 1      1     0.50                        sxtb	w9, w0
 1      1     0.50                        lsl	w8, w8, w1
 1      2     1.00                        mvn	w10, w8
 1      2     1.00                        add	w9, w9, w10, sxtb
 1      2     1.00                        cmp	w9, w9, sxtb
 1      2     1.00                        and	w1, w9, w8
 1      1     0.25                        cset	w0, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     3.00   3.00   3.00   3.00    -      -     5.00   5.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     sxtb	w9, w0
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w1
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	w10, w8
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     add	w9, w9, w10, sxtb
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     cmp	w9, w9, sxtb
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     and	w1, w9, w8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, eq
```
## `checked_ceil_to_multiple_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      14
Total uOps:        15

Dispatch Width:    6
uOps Per Cycle:    1.07
IPC:               1.07
Block RThroughput: 6.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w1, #0xff
 1      1     0.25                        cmp	w8, #7
 1      0     0.50                        b.hi	.LBB24_2
 1      1     0.25                        mov	w8, #-1
 1      1     0.50                        sxtb	w9, w0
 1      1     0.50                        lsl	w8, w8, w1
 1      2     1.00                        mvn	w10, w8
 1      2     1.00                        add	w9, w9, w10, sxtb
 1      1     0.50                        sxtb	w10, w9
 1      2     1.00                        cmp	w10, w9
 1      0     0.50                        b.eq	.LBB24_3
 1      2     1.00                        mov	w0, wzr
 1      0     1.00                  U     ret
 1      2     1.00                        and	w1, w9, w8
 1      1     0.25                        mov	w0, #1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   2.00   1.00    -      -     4.00   6.00   5.00   5.00    -      -     7.00   6.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w1, #0xff
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w8, #7
 -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.hi	.LBB24_2
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     sxtb	w9, w0
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w1
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	w10, w8
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     add	w9, w9, w10, sxtb
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     sxtb	w10, w9
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     cmp	w10, w9
 -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.eq	.LBB24_3
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mov	w0, wzr
1.00    -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     and	w1, w9, w8
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w0, #1
```
## `checked_ceil_to_multiple_i16_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      12
Total uOps:        8

Dispatch Width:    6
uOps Per Cycle:    0.67
IPC:               0.67
Block RThroughput: 5.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	w8, #-1
 1      1     0.50                        sxth	w9, w0
 1      1     0.50                        lsl	w8, w8, w1
 1      2     1.00                        mvn	w10, w8
 1      2     1.00                        add	w9, w9, w10, sxth
 1      2     1.00                        cmp	w9, w9, sxth
 1      2     1.00                        and	w1, w9, w8
 1      1     0.25                        cset	w0, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     3.00   3.00   3.00   3.00    -      -     5.00   5.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     sxth	w9, w0
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w1
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	w10, w8
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     add	w9, w9, w10, sxth
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     cmp	w9, w9, sxth
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     and	w1, w9, w8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, eq
```
## `checked_ceil_to_multiple_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      14
Total uOps:        15

Dispatch Width:    6
uOps Per Cycle:    1.07
IPC:               1.07
Block RThroughput: 6.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w1, #0xff
 1      1     0.25                        cmp	w8, #15
 1      0     0.50                        b.hi	.LBB18_2
 1      1     0.25                        mov	w8, #-1
 1      1     0.50                        sxth	w9, w0
 1      1     0.50                        lsl	w8, w8, w1
 1      2     1.00                        mvn	w10, w8
 1      2     1.00                        add	w9, w9, w10, sxth
 1      1     0.50                        sxth	w10, w9
 1      2     1.00                        cmp	w10, w9
 1      0     0.50                        b.eq	.LBB18_3
 1      2     1.00                        mov	w0, wzr
 1      0     1.00                  U     ret
 1      2     1.00                        and	w1, w9, w8
 1      1     0.25                        mov	w0, #1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   2.00   1.00    -      -     4.00   6.00   5.00   5.00    -      -     7.00   6.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w1, #0xff
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w8, #15
 -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.hi	.LBB18_2
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     sxth	w9, w0
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w1
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	w10, w8
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     add	w9, w9, w10, sxth
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     sxth	w10, w9
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     cmp	w10, w9
 -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.eq	.LBB18_3
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mov	w0, wzr
1.00    -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     and	w1, w9, w8
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w0, #1
```
## `checked_ceil_to_multiple_i32_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      11
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.55
IPC:               0.55
Block RThroughput: 3.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	w8, #-1
 1      1     0.50                        lsl	w8, w8, w1
 1      2     1.00                        mvn	w9, w8
 1      2     1.00                        adds	w9, w0, w9
 1      1     0.25                        cset	w0, vc
 1      2     1.00                        and	w1, w9, w8

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   2.00   3.00   2.00    -      -     4.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	w9, w8
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     adds	w9, w0, w9
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, vc
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     and	w1, w9, w8
```
## `checked_ceil_to_multiple_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      12
Total Cycles:      11
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.09
IPC:               1.09
Block RThroughput: 4.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w1, #0xff
 1      1     0.25                        cmp	w8, #31
 1      0     0.50                        b.hi	.LBB20_2
 1      1     0.25                        mov	w8, #-1
 1      1     0.50                        lsl	w8, w8, w1
 1      2     1.00                        mvn	w9, w8
 1      2     1.00                        adds	w9, w0, w9
 1      0     0.50                        b.vc	.LBB20_3
 1      2     1.00                        mov	w0, wzr
 1      0     1.00                  U     ret
 1      2     1.00                        and	w1, w9, w8
 1      1     0.25                        mov	w0, #1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   2.00   1.00    -      -     5.00   3.00   4.00   4.00    -      -     4.00   5.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w1, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w8, #31
1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.hi	.LBB20_2
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w1
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	w9, w8
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     adds	w9, w0, w9
 -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.vc	.LBB20_3
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mov	w0, wzr
 -     1.00   1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     and	w1, w9, w8
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w0, #1
```
## `checked_ceil_to_multiple_i64_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      11
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.55
IPC:               0.55
Block RThroughput: 3.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	x8, #-1
 1      1     0.50                        lsl	x8, x8, x1
 1      2     1.00                        mvn	x9, x8
 1      2     1.00                        adds	x9, x0, x9
 1      1     0.25                        cset	w0, vc
 1      2     1.00                        and	x1, x9, x8

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   2.00   3.00   2.00    -      -     4.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	x9, x8
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     adds	x9, x0, x9
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, vc
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     and	x1, x9, x8
```
## `checked_ceil_to_multiple_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      12
Total Cycles:      11
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.09
IPC:               1.09
Block RThroughput: 4.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w1, #0xff
 1      1     0.25                        cmp	w8, #63
 1      0     0.50                        b.hi	.LBB22_2
 1      1     0.25                        mov	x8, #-1
 1      1     0.50                        lsl	x8, x8, x1
 1      2     1.00                        mvn	x9, x8
 1      2     1.00                        adds	x9, x0, x9
 1      0     0.50                        b.vc	.LBB22_3
 1      2     1.00                        mov	x0, xzr
 1      0     1.00                  U     ret
 1      2     1.00                        and	x1, x9, x8
 1      1     0.25                        mov	w0, #1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   2.00   1.00    -      -     5.00   3.00   4.00   4.00    -      -     4.00   5.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w1, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w8, #63
1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.hi	.LBB22_2
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x1
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	x9, x8
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     adds	x9, x0, x9
 -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.vc	.LBB22_3
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mov	x0, xzr
 -     1.00   1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     and	x1, x9, x8
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w0, #1
```
## `checked_ceil_to_multiple_i128_pow2`
```asm
Iterations:        1
Instructions:      23
Total Cycles:      21
Total uOps:        26

Dispatch Width:    6
uOps Per Cycle:    1.24
IPC:               1.10
Block RThroughput: 8.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	x9, #-1
 1      2     1.00                        mvn	w10, w2
 1      1     0.25                        mov	x11, #9223372036854775807
 1      1     0.50                        lsl	x12, x9, x2
 1      1     0.25                        and	x9, x2, #0xff
 1      1     0.50                        lsr	x10, x11, x10
 1      1     0.25                        tst	x9, #0x40
 1      1     0.25                        csel	x9, xzr, x12, ne
 1      2     1.00                        orr	x10, x12, x10
 1      1     0.25                        csel	x10, x12, x10, ne
 1      2     1.00                        mvn	x11, x9
 1      2     1.00                        mvn	x12, x10
 1      2     1.00                        adds	x11, x0, x11
 1      1     0.25                        adcs	x12, x1, x12
 1      1     0.25                        cset	w13, vs
 1      0     0.50                        tbnz	w13, #0, .LBB15_2
 1      2     1.00                        and	x9, x11, x9
 1      2     1.00                        and	x10, x12, x10
 2      6     0.50           *            stp	x9, x10, [x8, #16]
 1      1     0.25                        mov	w9, #1
 2      6     0.50           *            stp	x9, xzr, [x8]
 1      0     1.00                  U     ret
 2      6     0.50           *            stp	xzr, xzr, [x8]

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   1.00   1.00    -      -     7.00   7.00   6.00   7.00    -      -     7.00   9.00    -     1.00   2.00   1.00   1.00   1.00    -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x9, #-1
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	w10, w2
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x11, #9223372036854775807
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x12, x9, x2
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x9, x2, #0xff
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x10, x11, x10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x9, #0x40
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x9, xzr, x12, ne
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x10, x12, x10
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x10, x12, x10, ne
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	x11, x9
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	x12, x10
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     adds	x11, x0, x11
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     adcs	x12, x1, x12
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w13, vs
1.00    -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tbnz	w13, #0, .LBB15_2
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     and	x9, x11, x9
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     and	x10, x12, x10
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -     stp	x9, x10, [x8, #16]
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w9, #1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     stp	x9, xzr, [x8]
 -     1.00   1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00   1.00    -      -      -      -      -      -     stp	xzr, xzr, [x8]
```
## `checked_ceil_to_multiple_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      24
Total Cycles:      21
Total uOps:        27

Dispatch Width:    6
uOps Per Cycle:    1.29
IPC:               1.14
Block RThroughput: 8.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.50                        tbnz	w2, #7, .LBB16_3
 1      1     0.25                        mov	x9, #-1
 1      2     1.00                        mvn	w10, w2
 1      1     0.25                        mov	x11, #9223372036854775807
 1      1     0.50                        lsl	x12, x9, x2
 1      1     0.25                        and	x9, x2, #0xff
 1      1     0.50                        lsr	x10, x11, x10
 1      1     0.25                        tst	x9, #0x40
 1      1     0.25                        csel	x9, xzr, x12, ne
 1      2     1.00                        orr	x10, x12, x10
 1      1     0.25                        csel	x10, x12, x10, ne
 1      2     1.00                        mvn	x11, x9
 1      2     1.00                        mvn	x12, x10
 1      2     1.00                        adds	x11, x0, x11
 1      1     0.25                        adcs	x12, x1, x12
 1      1     0.25                        cset	w13, vs
 1      0     0.50                        tbnz	w13, #0, .LBB16_3
 1      2     1.00                        and	x9, x11, x9
 1      2     1.00                        and	x10, x12, x10
 2      6     0.50           *            stp	x9, x10, [x8, #16]
 1      1     0.25                        mov	w9, #1
 2      6     0.50           *            stp	x9, xzr, [x8]
 1      0     1.00                  U     ret
 2      6     0.50           *            stp	xzr, xzr, [x8]

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   2.00   1.00    -      -     8.00   7.00   7.00   6.00    -      -     7.00   9.00    -     1.00   2.00   1.00   1.00   1.00    -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tbnz	w2, #7, .LBB16_3
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x9, #-1
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	w10, w2
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x11, #9223372036854775807
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x12, x9, x2
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x9, x2, #0xff
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x10, x11, x10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x9, #0x40
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x9, xzr, x12, ne
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x10, x12, x10
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x10, x12, x10, ne
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	x11, x9
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	x12, x10
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     adds	x11, x0, x11
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     adcs	x12, x1, x12
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w13, vs
 -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tbnz	w13, #0, .LBB16_3
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     and	x9, x11, x9
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     and	x10, x12, x10
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -     stp	x9, x10, [x8, #16]
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w9, #1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     stp	x9, xzr, [x8]
1.00    -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00   1.00    -      -      -      -      -      -     stp	xzr, xzr, [x8]
```
## `checked_ceil_to_multiple_u8_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      12
Total uOps:        8

Dispatch Width:    6
uOps Per Cycle:    0.67
IPC:               0.67
Block RThroughput: 4.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	w8, #-1
 1      1     0.25                        mov	w10, #255
 1      1     0.50                        lsl	w8, w8, w1
 1      2     1.00                        sub	w9, w8, w0
 1      2     1.00                        bic	w10, w10, w9
 1      2     1.00                        bic	w1, w8, w9
 1      2     1.00                        cmp	w10, w0, uxtb
 1      1     0.25                        cset	w0, hs

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     3.00   3.00   3.00   3.00    -      -     4.00   5.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w10, #255
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w1
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     sub	w9, w8, w0
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     bic	w10, w10, w9
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     bic	w1, w8, w9
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     cmp	w10, w0, uxtb
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, hs
```
## `checked_ceil_to_multiple_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      13
Total Cycles:      13
Total uOps:        13

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 5.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w1, #0xff
 1      1     0.25                        cmp	w8, #7
 1      0     0.50                        b.hi	.LBB34_2
 1      1     0.25                        mov	w8, #-1
 1      1     0.25                        mov	w10, #255
 1      1     0.50                        lsl	w8, w8, w1
 1      2     1.00                        sub	w9, w8, w0
 1      2     1.00                        bic	w10, w10, w9
 1      2     1.00                        bic	w1, w8, w9
 1      2     1.00                        cmp	w10, w0, uxtb
 1      1     0.25                        cset	w0, hs
 1      0     1.00                  U     ret
 1      2     1.00                        mov	w0, wzr

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   1.00   1.00    -      -     4.00   4.00   4.00   6.00    -      -     6.00   5.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w1, #0xff
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w8, #7
1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.hi	.LBB34_2
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w10, #255
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w1
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     sub	w9, w8, w0
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     bic	w10, w10, w9
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     bic	w1, w8, w9
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     cmp	w10, w0, uxtb
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, hs
 -     1.00   1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mov	w0, wzr
```
## `checked_ceil_to_multiple_u16_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      12
Total uOps:        8

Dispatch Width:    6
uOps Per Cycle:    0.67
IPC:               0.67
Block RThroughput: 4.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	w8, #-1
 1      1     0.25                        mov	w10, #65535
 1      1     0.50                        lsl	w8, w8, w1
 1      2     1.00                        sub	w9, w8, w0
 1      2     1.00                        bic	w10, w10, w9
 1      2     1.00                        bic	w1, w8, w9
 1      2     1.00                        cmp	w10, w0, uxth
 1      1     0.25                        cset	w0, hs

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     3.00   3.00   3.00   3.00    -      -     4.00   5.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w10, #65535
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w1
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     sub	w9, w8, w0
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     bic	w10, w10, w9
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     bic	w1, w8, w9
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     cmp	w10, w0, uxth
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, hs
```
## `checked_ceil_to_multiple_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      13
Total Cycles:      13
Total uOps:        13

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 5.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w1, #0xff
 1      1     0.25                        cmp	w8, #15
 1      0     0.50                        b.hi	.LBB28_2
 1      1     0.25                        mov	w8, #-1
 1      1     0.25                        mov	w10, #65535
 1      1     0.50                        lsl	w8, w8, w1
 1      2     1.00                        sub	w9, w8, w0
 1      2     1.00                        bic	w10, w10, w9
 1      2     1.00                        bic	w1, w8, w9
 1      2     1.00                        cmp	w10, w0, uxth
 1      1     0.25                        cset	w0, hs
 1      0     1.00                  U     ret
 1      2     1.00                        mov	w0, wzr

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   1.00   1.00    -      -     4.00   4.00   4.00   6.00    -      -     6.00   5.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w1, #0xff
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w8, #15
1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.hi	.LBB28_2
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w10, #65535
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w1
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     sub	w9, w8, w0
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     bic	w10, w10, w9
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     bic	w1, w8, w9
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     cmp	w10, w0, uxth
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, hs
 -     1.00   1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mov	w0, wzr
```
## `checked_ceil_to_multiple_u32_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      14
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    0.64
IPC:               0.64
Block RThroughput: 6.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	w8, #-1
 1      1     0.50                        lsl	w9, w8, w1
 1      2     1.00                        mvn	w8, w9
 1      2     1.00                        sub	w10, w9, w0
 1      2     1.00                        add	w8, w0, w8
 1      2     1.00                        bic	w1, w9, w10
 1      2     1.00                        cmp	w8, w0
 1      1     0.25                        cset	w8, hs
 1      2     1.00                        mov	w0, w8

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     3.00   4.00   3.00   5.00    -      -     6.00   7.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w9, w8, w1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	w8, w9
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     sub	w10, w9, w0
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     add	w8, w0, w8
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     bic	w1, w9, w10
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     cmp	w8, w0
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w8, hs
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mov	w0, w8
```
## `checked_ceil_to_multiple_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      13
Total Cycles:      13
Total uOps:        13

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 6.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w1, #0xff
 1      1     0.25                        cmp	w8, #31
 1      0     0.50                        b.hi	.LBB30_2
 1      1     0.25                        mov	w8, #-1
 1      1     0.50                        lsl	w8, w8, w1
 1      2     1.00                        mvn	w9, w8
 1      2     1.00                        add	w9, w0, w9
 1      2     1.00                        cmp	w9, w0
 1      2     1.00                        sub	w9, w8, w0
 1      1     0.25                        cset	w0, hs
 1      2     1.00                        bic	w1, w8, w9
 1      0     1.00                  U     ret
 1      2     1.00                        mov	w0, wzr

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   1.00   1.00    -      -     4.00   5.00   4.00   6.00    -      -     6.00   7.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w1, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w8, #31
1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.hi	.LBB30_2
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	w9, w8
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     add	w9, w0, w9
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     cmp	w9, w0
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     sub	w9, w8, w0
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, hs
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     bic	w1, w8, w9
 -     1.00   1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mov	w0, wzr
```
## `checked_ceil_to_multiple_u64_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      14
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    0.64
IPC:               0.64
Block RThroughput: 6.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	x8, #-1
 1      1     0.50                        lsl	x9, x8, x1
 1      2     1.00                        mvn	x8, x9
 1      2     1.00                        sub	x10, x9, x0
 1      2     1.00                        add	x8, x0, x8
 1      2     1.00                        bic	x1, x9, x10
 1      2     1.00                        cmp	x8, x0
 1      1     0.25                        cset	w8, hs
 1      2     1.00                        mov	x0, x8

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     3.00   4.00   3.00   5.00    -      -     6.00   7.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x9, x8, x1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	x8, x9
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     sub	x10, x9, x0
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     add	x8, x0, x8
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     bic	x1, x9, x10
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     cmp	x8, x0
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w8, hs
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mov	x0, x8
```
## `checked_ceil_to_multiple_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      13
Total Cycles:      13
Total uOps:        13

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 6.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w1, #0xff
 1      1     0.25                        cmp	w8, #63
 1      0     0.50                        b.hi	.LBB32_2
 1      1     0.25                        mov	x8, #-1
 1      1     0.50                        lsl	x8, x8, x1
 1      2     1.00                        mvn	x9, x8
 1      2     1.00                        add	x9, x0, x9
 1      2     1.00                        cmp	x9, x0
 1      2     1.00                        sub	x9, x8, x0
 1      1     0.25                        cset	w0, hs
 1      2     1.00                        bic	x1, x8, x9
 1      0     1.00                  U     ret
 1      2     1.00                        mov	x0, xzr

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   1.00   1.00    -      -     4.00   5.00   4.00   6.00    -      -     6.00   7.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w1, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w8, #63
1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.hi	.LBB32_2
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	x9, x8
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     add	x9, x0, x9
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     cmp	x9, x0
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     sub	x9, x8, x0
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, hs
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     bic	x1, x8, x9
 -     1.00   1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mov	x0, xzr
```
## `checked_ceil_to_multiple_u128_pow2`
```asm
Iterations:        1
Instructions:      23
Total Cycles:      21
Total uOps:        26

Dispatch Width:    6
uOps Per Cycle:    1.24
IPC:               1.10
Block RThroughput: 8.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	x9, #-1
 1      2     1.00                        mvn	w10, w2
 1      1     0.25                        mov	x11, #9223372036854775807
 1      1     0.50                        lsl	x12, x9, x2
 1      1     0.25                        and	x9, x2, #0xff
 1      1     0.50                        lsr	x10, x11, x10
 1      1     0.25                        tst	x9, #0x40
 1      1     0.25                        csel	x9, xzr, x12, ne
 1      2     1.00                        orr	x10, x12, x10
 1      1     0.25                        csel	x10, x12, x10, ne
 1      2     1.00                        mvn	x11, x9
 1      2     1.00                        mvn	x12, x10
 1      2     1.00                        adds	x11, x0, x11
 1      1     0.25                        adcs	x12, x1, x12
 1      1     0.25                        cset	w13, hs
 1      0     0.50                        tbnz	w13, #0, .LBB25_2
 1      2     1.00                        and	x9, x11, x9
 1      2     1.00                        and	x10, x12, x10
 2      6     0.50           *            stp	x9, x10, [x8, #16]
 1      1     0.25                        mov	w9, #1
 2      6     0.50           *            stp	x9, xzr, [x8]
 1      0     1.00                  U     ret
 2      6     0.50           *            stp	xzr, xzr, [x8]

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   1.00   1.00    -      -     7.00   7.00   6.00   7.00    -      -     7.00   9.00    -     1.00   2.00   1.00   1.00   1.00    -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x9, #-1
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	w10, w2
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x11, #9223372036854775807
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x12, x9, x2
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x9, x2, #0xff
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x10, x11, x10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x9, #0x40
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x9, xzr, x12, ne
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x10, x12, x10
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x10, x12, x10, ne
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	x11, x9
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	x12, x10
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     adds	x11, x0, x11
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     adcs	x12, x1, x12
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w13, hs
1.00    -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tbnz	w13, #0, .LBB25_2
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     and	x9, x11, x9
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     and	x10, x12, x10
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -     stp	x9, x10, [x8, #16]
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w9, #1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     stp	x9, xzr, [x8]
 -     1.00   1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00   1.00    -      -      -      -      -      -     stp	xzr, xzr, [x8]
```
## `checked_ceil_to_multiple_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      24
Total Cycles:      21
Total uOps:        27

Dispatch Width:    6
uOps Per Cycle:    1.29
IPC:               1.14
Block RThroughput: 8.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.50                        tbnz	w2, #7, .LBB26_3
 1      1     0.25                        mov	x9, #-1
 1      2     1.00                        mvn	w10, w2
 1      1     0.25                        mov	x11, #9223372036854775807
 1      1     0.50                        lsl	x12, x9, x2
 1      1     0.25                        and	x9, x2, #0xff
 1      1     0.50                        lsr	x10, x11, x10
 1      1     0.25                        tst	x9, #0x40
 1      1     0.25                        csel	x9, xzr, x12, ne
 1      2     1.00                        orr	x10, x12, x10
 1      1     0.25                        csel	x10, x12, x10, ne
 1      2     1.00                        mvn	x11, x9
 1      2     1.00                        mvn	x12, x10
 1      2     1.00                        adds	x11, x0, x11
 1      1     0.25                        adcs	x12, x1, x12
 1      1     0.25                        cset	w13, hs
 1      0     0.50                        tbnz	w13, #0, .LBB26_3
 1      2     1.00                        and	x9, x11, x9
 1      2     1.00                        and	x10, x12, x10
 2      6     0.50           *            stp	x9, x10, [x8, #16]
 1      1     0.25                        mov	w9, #1
 2      6     0.50           *            stp	x9, xzr, [x8]
 1      0     1.00                  U     ret
 2      6     0.50           *            stp	xzr, xzr, [x8]

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   2.00   1.00    -      -     8.00   7.00   7.00   6.00    -      -     7.00   9.00    -     1.00   2.00   1.00   1.00   1.00    -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tbnz	w2, #7, .LBB26_3
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x9, #-1
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	w10, w2
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x11, #9223372036854775807
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x12, x9, x2
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x9, x2, #0xff
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x10, x11, x10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x9, #0x40
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x9, xzr, x12, ne
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x10, x12, x10
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x10, x12, x10, ne
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	x11, x9
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	x12, x10
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     adds	x11, x0, x11
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     adcs	x12, x1, x12
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w13, hs
 -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tbnz	w13, #0, .LBB26_3
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     and	x9, x11, x9
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     and	x10, x12, x10
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -     stp	x9, x10, [x8, #16]
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w9, #1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     stp	x9, xzr, [x8]
1.00    -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00   1.00    -      -      -      -      -      -     stp	xzr, xzr, [x8]
```
## `checked_div_ceil_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      12
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    0.92
IPC:               0.92
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w1, #0xff
 1      1     0.25                        cmp	w8, #7
 1      0     0.50                        b.hi	.LBB39_2
 1      1     0.25                        mov	x8, #-1
 1      1     0.50                        lsl	x8, x8, x1
 1      2     1.00                        mvn	x8, x8
 1      2     1.00                        add	x8, x8, w0, sxtb
 1      1     0.50                        asr	x8, x8, x1
 1      1     0.25                        tst	w1, #0xf8
 1      2     1.00                        mov	w1, w8
 1      1     0.25                        cset	w0, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -     1.00    -      -      -     2.00   5.00   3.00   4.00    -      -     3.00   5.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w1, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w8, #7
 -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.hi	.LBB39_2
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	x8, x8
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     add	x8, x8, w0, sxtb
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	x8, x8, x1
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w1, #0xf8
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mov	w1, w8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, eq
```
## `checked_div_ceil_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      10
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    1.10
IPC:               1.10
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w1, #0xff
 1      1     0.25                        cmp	w8, #15
 1      0     0.50                        b.hi	.LBB36_2
 1      1     0.25                        mov	x8, #-1
 1      1     0.50                        lsl	x8, x8, x1
 1      2     1.00                        mvn	x8, x8
 1      2     1.00                        add	x8, x8, w0, sxth
 1      1     0.25                        mov	w0, #1
 1      1     0.50                        asr	x1, x8, x1
 1      0     1.00                  U     ret
 1      2     1.00                        mov	w0, wzr

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   1.00   1.00    -      -     3.00   3.00   4.00   4.00    -      -     4.00   4.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w1, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w8, #15
1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.hi	.LBB36_2
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x1
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	x8, x8
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     add	x8, x8, w0, sxth
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w0, #1
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	x1, x8, x1
 -     1.00   1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mov	w0, wzr
```
## `checked_div_ceil_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      10
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    1.10
IPC:               1.10
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w1, #0xff
 1      1     0.25                        cmp	w8, #31
 1      0     0.50                        b.hi	.LBB37_2
 1      1     0.25                        mov	x8, #-1
 1      1     0.50                        lsl	x8, x8, x1
 1      2     1.00                        mvn	x8, x8
 1      2     1.00                        add	x8, x8, w0, sxtw
 1      1     0.25                        mov	w0, #1
 1      1     0.50                        asr	x1, x8, x1
 1      0     1.00                  U     ret
 1      2     1.00                        mov	w0, wzr

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   1.00   1.00    -      -     3.00   3.00   4.00   4.00    -      -     4.00   4.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w1, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w8, #31
1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.hi	.LBB37_2
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x1
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	x8, x8
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     add	x8, x8, w0, sxtw
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w0, #1
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	x1, x8, x1
 -     1.00   1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mov	w0, wzr
```
## `checked_div_ceil_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      9
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    0.78
IPC:               0.78
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        asr	x9, x0, x1
 1      1     0.25                        tst	w1, #0xc0
 1      1     0.25                        cset	w8, eq
 1      1     0.50                        lsl	x10, x9, x1
 1      2     1.00                        cmp	x0, x10
 1      2     1.00                        mov	x0, x8
 1      1     0.25                        cinc	x1, x9, ne

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   2.00   3.00   3.00    -      -     3.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	x9, x0, x1
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w1, #0xc0
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w8, eq
 -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x10, x9, x1
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     cmp	x0, x10
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mov	x0, x8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cinc	x1, x9, ne
```
## `checked_div_ceil_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      29
Total Cycles:      26
Total uOps:        32

Dispatch Width:    6
uOps Per Cycle:    1.23
IPC:               1.12
Block RThroughput: 9.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.50                        tbnz	w2, #7, .LBB35_2
 1      1     0.50                        lsl	x9, x1, #1
 1      2     1.00                        mvn	w10, w2
 1      1     0.50                        lsr	x11, x0, x2
 1      1     0.50                        asr	x12, x1, x2
 1      1     0.25                        and	x13, x2, #0xff
 1      1     0.50                        lsl	x9, x9, x10
 1      1     0.25                        tst	x13, #0x40
 1      2     1.00                        orr	x9, x9, x11
 1      1     0.50                        asr	x11, x1, #63
 1      1     0.25                        csel	x9, x12, x9, ne
 1      1     0.50                        lsr	x13, x9, #1
 1      1     0.25                        csel	x11, x11, x12, ne
 1      1     0.50                        lsl	x12, x11, x2
 1      1     0.50                        lsr	x10, x13, x10
 1      1     0.50                        lsl	x13, x9, x2
 1      2     1.00                        orr	x10, x12, x10
 1      1     0.25                        csel	x12, xzr, x13, ne
 1      1     0.25                        csel	x10, x13, x10, ne
 1      2     1.00                        cmp	x0, x12
 1      1     0.25                        ccmp	x1, x10, #0, eq
 1      1     0.25                        cset	w10, ne
 1      2     1.00                        adds	x9, x9, x10
 1      1     0.25                        cinc	x10, x11, hs
 2      6     0.50           *            stp	x9, x10, [x8, #16]
 1      1     0.25                        mov	w9, #1
 2      6     0.50           *            stp	x9, xzr, [x8]
 1      0     1.00                  U     ret
 2      6     0.50           *            stp	xzr, xzr, [x8]

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   1.00   1.00    -      -     6.00   8.00   9.00   8.00    -      -     11.00  8.00    -     1.00   2.00   1.00   1.00   1.00    -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tbnz	w2, #7, .LBB35_2
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x9, x1, #1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	w10, w2
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x11, x0, x2
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	x12, x1, x2
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x13, x2, #0xff
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x9, x9, x10
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x13, #0x40
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x9, x9, x11
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	x11, x1, #63
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x9, x12, x9, ne
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x13, x9, #1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x11, x11, x12, ne
 -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x12, x11, x2
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x10, x13, x10
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x13, x9, x2
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     orr	x10, x12, x10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x12, xzr, x13, ne
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x10, x13, x10, ne
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     cmp	x0, x12
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ccmp	x1, x10, #0, eq
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w10, ne
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     adds	x9, x9, x10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cinc	x10, x11, hs
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -     stp	x9, x10, [x8, #16]
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w9, #1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     stp	x9, xzr, [x8]
1.00    -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00   1.00    -      -      -      -      -      -     stp	xzr, xzr, [x8]
```
## `checked_div_ceil_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      10
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    0.70
IPC:               0.70
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	x8, #-1
 1      1     0.25                        tst	w1, #0xf8
 1      1     0.50                        lsl	x8, x8, x1
 1      2     1.00                        mvn	x8, x8
 1      2     1.00                        add	x8, x8, w0, uxtb
 1      1     0.25                        cset	w0, eq
 1      1     0.50                        lsr	x1, x8, x1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   2.00   3.00   3.00    -      -     3.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w1, #0xf8
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x1
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	x8, x8
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     add	x8, x8, w0, uxtb
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, eq
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	x1, x8, x1
```
## `checked_div_ceil_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      10
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    0.70
IPC:               0.70
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	x8, #-1
 1      1     0.25                        tst	w1, #0xf0
 1      1     0.50                        lsl	x8, x8, x1
 1      2     1.00                        mvn	x8, x8
 1      2     1.00                        add	x8, x8, w0, uxth
 1      1     0.25                        cset	w0, eq
 1      1     0.50                        lsr	x1, x8, x1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   2.00   3.00   3.00    -      -     3.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w1, #0xf0
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x1
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	x8, x8
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     add	x8, x8, w0, uxth
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, eq
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	x1, x8, x1
```
## `checked_div_ceil_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      10
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    0.70
IPC:               0.70
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	x8, #-1
 1      1     0.25                        tst	w1, #0xe0
 1      1     0.50                        lsl	x8, x8, x1
 1      2     1.00                        mvn	x8, x8
 1      2     1.00                        add	x8, x8, w0, uxtw
 1      1     0.25                        cset	w0, eq
 1      1     0.50                        lsr	x1, x8, x1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   2.00   3.00   3.00    -      -     3.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w1, #0xe0
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x1
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	x8, x8
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     add	x8, x8, w0, uxtw
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, eq
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	x1, x8, x1
```
## `checked_div_ceil_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      9
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    0.78
IPC:               0.78
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsr	x9, x0, x1
 1      1     0.25                        tst	w1, #0xc0
 1      1     0.25                        cset	w8, eq
 1      1     0.50                        lsl	x10, x9, x1
 1      2     1.00                        cmp	x0, x10
 1      2     1.00                        mov	x0, x8
 1      1     0.25                        cinc	x1, x9, ne

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   2.00   3.00   3.00    -      -     3.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x9, x0, x1
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w1, #0xc0
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w8, eq
 -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x10, x9, x1
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     cmp	x0, x10
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mov	x0, x8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cinc	x1, x9, ne
```
## `checked_div_ceil_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      28
Total Cycles:      26
Total uOps:        31

Dispatch Width:    6
uOps Per Cycle:    1.19
IPC:               1.08
Block RThroughput: 9.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.50                        tbnz	w2, #7, .LBB40_2
 1      1     0.50                        lsl	x9, x1, #1
 1      2     1.00                        mvn	w10, w2
 1      1     0.50                        lsr	x11, x0, x2
 1      1     0.50                        lsr	x12, x1, x2
 1      1     0.25                        and	x13, x2, #0xff
 1      1     0.50                        lsl	x9, x9, x10
 1      1     0.25                        tst	x13, #0x40
 1      2     1.00                        orr	x9, x9, x11
 1      1     0.25                        csel	x9, x12, x9, ne
 1      1     0.25                        csel	x12, xzr, x12, ne
 1      1     0.50                        lsr	x11, x9, #1
 1      1     0.50                        lsl	x13, x12, x2
 1      1     0.50                        lsr	x10, x11, x10
 1      1     0.50                        lsl	x11, x9, x2
 1      2     1.00                        orr	x10, x13, x10
 1      1     0.25                        csel	x13, xzr, x11, ne
 1      1     0.25                        csel	x10, x11, x10, ne
 1      2     1.00                        cmp	x0, x13
 1      1     0.25                        ccmp	x1, x10, #0, eq
 1      1     0.25                        cset	w10, ne
 1      2     1.00                        adds	x9, x9, x10
 1      1     0.25                        cinc	x10, x12, hs
 2      6     0.50           *            stp	x9, x10, [x8, #16]
 1      1     0.25                        mov	w9, #1
 2      6     0.50           *            stp	x9, xzr, [x8]
 1      0     1.00                  U     ret
 2      6     0.50           *            stp	xzr, xzr, [x8]

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   1.00   1.00    -      -     7.00   7.00   7.00   9.00    -      -     9.00   9.00    -     1.00   2.00   1.00   1.00   1.00    -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tbnz	w2, #7, .LBB40_2
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x9, x1, #1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	w10, w2
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x11, x0, x2
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	x12, x1, x2
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x13, x2, #0xff
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x9, x9, x10
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x13, #0x40
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x9, x9, x11
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x9, x12, x9, ne
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x12, xzr, x12, ne
 -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	x11, x9, #1
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x13, x12, x2
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	x10, x11, x10
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x11, x9, x2
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x10, x13, x10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x13, xzr, x11, ne
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x10, x11, x10, ne
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     cmp	x0, x13
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ccmp	x1, x10, #0, eq
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w10, ne
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     adds	x9, x9, x10
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cinc	x10, x12, hs
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -     stp	x9, x10, [x8, #16]
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w9, #1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     stp	x9, xzr, [x8]
1.00    -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00   1.00    -      -      -      -      -      -     stp	xzr, xzr, [x8]
```
## `checked_div_floor_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      5
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.80
IPC:               0.80
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        sxtb	w8, w0
 1      1     0.25                        tst	w1, #0xf8
 1      1     0.25                        cset	w0, eq
 1      1     0.50                        asr	w1, w8, w1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   1.00   1.00   1.00    -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     sxtb	w8, w0
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w1, #0xf8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, eq
 -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	w1, w8, w1
```
## `checked_div_floor_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      5
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.80
IPC:               0.80
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        sxth	w8, w0
 1      1     0.25                        tst	w1, #0xf0
 1      1     0.25                        cset	w0, eq
 1      1     0.50                        asr	w1, w8, w1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   1.00   1.00   1.00    -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     sxth	w8, w0
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w1, #0xf0
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, eq
 -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	w1, w8, w1
```
## `checked_div_floor_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      5
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.60
IPC:               0.60
Block RThroughput: 0.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        tst	w1, #0xe0
 1      1     0.50                        asr	w1, w0, w1
 1      1     0.25                        cset	w0, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -     1.00   1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w1, #0xe0
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	w1, w0, w1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, eq
```
## `checked_div_floor_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      5
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.60
IPC:               0.60
Block RThroughput: 0.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        tst	w1, #0xc0
 1      1     0.50                        asr	x1, x0, x1
 1      1     0.25                        cset	w0, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -     1.00   1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w1, #0xc0
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	x1, x0, x1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, eq
```
## `checked_div_floor_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      17
Total Cycles:      16
Total uOps:        20

Dispatch Width:    6
uOps Per Cycle:    1.25
IPC:               1.06
Block RThroughput: 4.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.50                        tbnz	w2, #7, .LBB45_2
 1      1     0.50                        lsl	x9, x1, #1
 1      2     1.00                        mvn	w10, w2
 1      1     0.50                        lsr	x11, x0, x2
 1      1     0.25                        and	x12, x2, #0xff
 1      1     0.50                        asr	x13, x1, #63
 1      1     0.50                        lsl	x9, x9, x10
 1      1     0.50                        asr	x10, x1, x2
 1      1     0.25                        tst	x12, #0x40
 1      2     1.00                        orr	x9, x9, x11
 1      1     0.25                        csel	x11, x13, x10, ne
 1      1     0.25                        csel	x9, x10, x9, ne
 2      6     0.50           *            stp	x9, x11, [x8, #16]
 1      1     0.25                        mov	w9, #1
 2      6     0.50           *            stp	x9, xzr, [x8]
 1      0     1.00                  U     ret
 2      6     0.50           *            stp	xzr, xzr, [x8]

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   1.00   1.00    -      -     4.00   4.00   4.00   4.00    -      -     4.00   5.00    -     1.00   2.00   1.00   1.00   1.00    -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tbnz	w2, #7, .LBB45_2
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x9, x1, #1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	w10, w2
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x11, x0, x2
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x12, x2, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	x13, x1, #63
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x9, x9, x10
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	x10, x1, x2
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x12, #0x40
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     orr	x9, x9, x11
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x11, x13, x10, ne
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x9, x10, x9, ne
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -     stp	x9, x11, [x8, #16]
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w9, #1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     stp	x9, xzr, [x8]
1.00    -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00   1.00    -      -      -      -      -      -     stp	xzr, xzr, [x8]
```
## `checked_div_floor_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      5
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.80
IPC:               0.80
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w0, #0xff
 1      1     0.25                        tst	w1, #0xf8
 1      1     0.50                        lsr	w1, w8, w1
 1      1     0.25                        cset	w0, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   1.00   1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w0, #0xff
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w1, #0xf8
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	w1, w8, w1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, eq
```
## `checked_div_floor_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      5
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.80
IPC:               0.80
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w0, #0xffff
 1      1     0.25                        tst	w1, #0xf0
 1      1     0.50                        lsr	w1, w8, w1
 1      1     0.25                        cset	w0, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   1.00   1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w0, #0xffff
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w1, #0xf0
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	w1, w8, w1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, eq
```
## `checked_div_floor_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      5
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.60
IPC:               0.60
Block RThroughput: 0.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        tst	w1, #0xe0
 1      1     0.50                        lsr	w1, w0, w1
 1      1     0.25                        cset	w0, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -     1.00   1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w1, #0xe0
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	w1, w0, w1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, eq
```
## `checked_div_floor_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      5
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.60
IPC:               0.60
Block RThroughput: 0.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        tst	w1, #0xc0
 1      1     0.50                        lsr	x1, x0, x1
 1      1     0.25                        cset	w0, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -     1.00   1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w1, #0xc0
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x1, x0, x1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, eq
```
## `checked_div_floor_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      16
Total Cycles:      16
Total uOps:        19

Dispatch Width:    6
uOps Per Cycle:    1.19
IPC:               1.00
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.50                        tbnz	w2, #7, .LBB50_2
 1      1     0.50                        lsl	x9, x1, #1
 1      2     1.00                        mvn	w11, w2
 1      1     0.50                        lsr	x10, x0, x2
 1      1     0.25                        and	x12, x2, #0xff
 1      1     0.50                        lsl	x9, x9, x11
 1      1     0.50                        lsr	x11, x1, x2
 1      1     0.25                        tst	x12, #0x40
 1      2     1.00                        orr	x9, x9, x10
 1      1     0.25                        csel	x10, xzr, x11, ne
 1      1     0.25                        csel	x9, x11, x9, ne
 2      6     0.50           *            stp	x9, x10, [x8, #16]
 1      1     0.25                        mov	w9, #1
 2      6     0.50           *            stp	x9, xzr, [x8]
 1      0     1.00                  U     ret
 2      6     0.50           *            stp	xzr, xzr, [x8]

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   1.00   1.00    -      -     3.00   5.00   3.00   4.00    -      -     5.00   3.00    -     1.00   2.00   1.00   1.00   1.00    -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tbnz	w2, #7, .LBB50_2
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x9, x1, #1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	w11, w2
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x10, x0, x2
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x12, x2, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x9, x9, x11
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x11, x1, x2
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x12, #0x40
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x9, x9, x10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x10, xzr, x11, ne
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x9, x11, x9, ne
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -     stp	x9, x10, [x8, #16]
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w9, #1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     stp	x9, xzr, [x8]
1.00    -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00   1.00    -      -      -      -      -      -     stp	xzr, xzr, [x8]
```
## `checked_div_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      19
Total Cycles:      16
Total uOps:        19

Dispatch Width:    6
uOps Per Cycle:    1.19
IPC:               1.19
Block RThroughput: 6.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w1, #0xff
 1      1     0.25                        cmp	w8, #7
 1      0     0.50                        b.hi	.LBB59_3
 1      0     0.50                        tbnz	w0, #7, .LBB59_4
 1      1     0.25                        and	w8, w0, #0xff
 1      1     0.50                        lsr	w8, w8, w1
 1      1     0.25                        tst	w1, #0xf8
 1      2     1.00                        mov	w1, w8
 1      1     0.25                        cset	w0, eq
 1      0     1.00                  U     ret
 1      1     0.25                        mov	w8, #-1
 1      1     0.50                        lsl	w8, w8, w1
 1      2     1.00                        mvn	w8, w8
 1      2     1.00                        add	w8, w0, w8
 1      1     0.50                        sxtb	w8, w8
 1      1     0.50                        asr	w8, w8, w1
 1      1     0.25                        tst	w1, #0xf8
 1      2     1.00                        mov	w1, w8
 1      1     0.25                        cset	w0, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   2.00   1.00    -      -     5.00   6.00   6.00   6.00    -      -     7.00   5.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w1, #0xff
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w8, #7
 -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.hi	.LBB59_3
 -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tbnz	w0, #7, .LBB59_4
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w0, #0xff
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	w8, w8, w1
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w1, #0xf8
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mov	w1, w8
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, eq
1.00    -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w1
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	w8, w8
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     add	w8, w0, w8
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     sxtb	w8, w8
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	w8, w8, w1
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w1, #0xf8
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mov	w1, w8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, eq
```
## `checked_div_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      17
Total Cycles:      12
Total uOps:        17

Dispatch Width:    6
uOps Per Cycle:    1.42
IPC:               1.42
Block RThroughput: 5.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w1, #0xff
 1      1     0.25                        cmp	w8, #15
 1      0     0.50                        b.hi	.LBB56_3
 1      0     0.50                        tbnz	w0, #15, .LBB56_4
 1      1     0.25                        and	w9, w0, #0xffff
 1      1     0.25                        mov	w0, #1
 1      1     0.50                        lsr	w1, w9, w8
 1      0     1.00                  U     ret
 1      2     1.00                        mov	w0, wzr
 1      0     1.00                  U     ret
 1      1     0.25                        mov	w9, #-1
 1      1     0.50                        lsl	w9, w9, w8
 1      2     1.00                        mvn	w9, w9
 1      2     1.00                        add	w9, w0, w9
 1      1     0.25                        mov	w0, #1
 1      1     0.50                        sxth	w9, w9
 1      1     0.50                        asr	w1, w9, w8

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
2.00   2.00   2.00    -      -     5.00   4.00   5.00   6.00    -      -     5.00   5.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w1, #0xff
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w8, #15
 -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.hi	.LBB56_3
 -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tbnz	w0, #15, .LBB56_4
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w9, w0, #0xffff
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w0, #1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	w1, w9, w8
1.00    -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mov	w0, wzr
1.00    -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w9, #-1
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w9, w9, w8
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	w9, w9
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     add	w9, w0, w9
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w0, #1
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     sxth	w9, w9
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	w1, w9, w8
```
## `checked_div_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      11
Total uOps:        15

Dispatch Width:    6
uOps Per Cycle:    1.36
IPC:               1.36
Block RThroughput: 4.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w1, #0xff
 1      1     0.25                        cmp	w8, #31
 1      0     0.50                        b.hi	.LBB57_3
 1      0     0.50                        tbnz	w0, #31, .LBB57_4
 1      1     0.50                        lsr	w1, w0, w8
 1      1     0.25                        mov	w0, #1
 1      0     1.00                  U     ret
 1      2     1.00                        mov	w0, wzr
 1      0     1.00                  U     ret
 1      1     0.25                        mov	w9, #-1
 1      1     0.50                        lsl	w9, w9, w8
 1      2     1.00                        mvn	w9, w9
 1      2     1.00                        add	w9, w0, w9
 1      1     0.25                        mov	w0, #1
 1      1     0.50                        asr	w1, w9, w8

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
2.00   2.00   2.00    -      -     5.00   5.00   4.00   4.00    -      -     4.00   5.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w1, #0xff
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w8, #31
 -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.hi	.LBB57_3
 -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tbnz	w0, #31, .LBB57_4
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	w1, w0, w8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w0, #1
1.00    -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mov	w0, wzr
1.00    -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w9, #-1
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w9, w9, w8
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	w9, w9
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     add	w9, w0, w9
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w0, #1
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	w1, w9, w8
```
## `checked_div_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      16
Total Cycles:      12
Total uOps:        16

Dispatch Width:    6
uOps Per Cycle:    1.33
IPC:               1.33
Block RThroughput: 4.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w1, #0xff
 1      1     0.25                        cmp	w8, #63
 1      0     0.50                        b.hi	.LBB58_3
 1      1     0.25                        and	x8, x1, #0xff
 1      0     0.50                        tbnz	x0, #63, .LBB58_4
 1      1     0.50                        lsr	x1, x0, x8
 1      1     0.25                        mov	w0, #1
 1      0     1.00                  U     ret
 1      2     1.00                        mov	x0, xzr
 1      0     1.00                  U     ret
 1      1     0.25                        mov	x9, #-1
 1      1     0.50                        lsl	x9, x9, x8
 1      2     1.00                        mvn	x9, x9
 1      2     1.00                        add	x9, x0, x9
 1      1     0.25                        mov	w0, #1
 1      1     0.50                        asr	x1, x9, x8

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
2.00   2.00   2.00    -      -     5.00   5.00   4.00   5.00    -      -     5.00   4.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w1, #0xff
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w8, #63
 -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.hi	.LBB58_3
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x8, x1, #0xff
 -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tbnz	x0, #63, .LBB58_4
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x1, x0, x8
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w0, #1
1.00    -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mov	x0, xzr
1.00    -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x9, #-1
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x9, x9, x8
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	x9, x9
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     add	x9, x0, x9
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w0, #1
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	x1, x9, x8
```
## `checked_div_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      45
Total Cycles:      28
Total uOps:        50

Dispatch Width:    6
uOps Per Cycle:    1.79
IPC:               1.61
Block RThroughput: 14.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.50                        tbnz	w2, #7, .LBB55_3
 1      0     0.50                        tbnz	x1, #63, .LBB55_4
 1      1     0.50                        lsl	x9, x1, #1
 1      2     1.00                        mov	w10, w2
 1      2     1.00                        mvn	w11, w2
 1      1     0.50                        lsr	x12, x0, x10
 1      1     0.50                        lsl	x9, x9, x11
 1      1     0.50                        lsr	x11, x1, x10
 1      1     0.25                        and	x10, x10, #0xff
 1      1     0.25                        tst	x10, #0x40
 1      2     1.00                        orr	x10, x9, x12
 1      1     0.25                        csel	x9, xzr, x11, ne
 1      1     0.25                        csel	x10, x11, x10, ne
 2      6     0.50           *            stp	x10, x9, [x8, #16]
 1      1     0.25                        mov	w10, #1
 2      6     0.50           *            stp	x10, xzr, [x8]
 1      0     1.00                  U     ret
 2      6     0.50           *            stp	xzr, xzr, [x8]
 1      0     1.00                  U     ret
 1      1     0.25                        mov	x9, #-1
 1      2     1.00                        mvn	w10, w2
 1      1     0.25                        mov	x11, #9223372036854775807
 1      1     0.50                        lsl	x9, x9, x2
 1      1     0.25                        and	x12, x2, #0xff
 1      1     0.50                        lsr	x11, x11, x10
 1      1     0.25                        tst	x12, #0x40
 1      1     0.25                        csel	x13, xzr, x9, ne
 1      2     1.00                        orr	x11, x9, x11
 1      1     0.25                        csel	x9, x9, x11, ne
 1      2     1.00                        mvn	x11, x13
 1      2     1.00                        mvn	x9, x9
 1      2     1.00                        adds	x11, x0, x11
 1      1     0.25                        adc	x9, x1, x9
 1      1     0.50                        lsr	x11, x11, x2
 1      1     0.25                        tst	x12, #0x40
 1      1     0.50                        lsl	x13, x9, #1
 1      1     0.50                        lsl	x10, x13, x10
 1      1     0.50                        asr	x13, x9, x2
 1      1     0.50                        asr	x9, x9, #63
 1      2     1.00                        orr	x10, x10, x11
 1      1     0.25                        csel	x9, x9, x13, ne
 1      1     0.25                        csel	x10, x13, x10, ne
 2      6     0.50           *            stp	x10, x9, [x8, #16]
 1      1     0.25                        mov	w10, #1
 2      6     0.50           *            stp	x10, xzr, [x8]

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
2.00   2.00   2.00    -      -     13.00  10.00  12.00  14.00   -      -     15.00  14.00   -     2.00   3.00   1.00   2.00   2.00    -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tbnz	w2, #7, .LBB55_3
1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tbnz	x1, #63, .LBB55_4
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x9, x1, #1
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mov	w10, w2
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	w11, w2
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	x12, x0, x10
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x9, x9, x11
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	x11, x1, x10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x10, x10, #0xff
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x10, #0x40
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     orr	x10, x9, x12
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x9, xzr, x11, ne
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x10, x11, x10, ne
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -     stp	x10, x9, [x8, #16]
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w10, #1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     stp	x10, xzr, [x8]
 -     1.00   1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00   1.00    -      -      -      -      -      -     stp	xzr, xzr, [x8]
1.00    -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x9, #-1
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	w10, w2
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x11, #9223372036854775807
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x9, x9, x2
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x12, x2, #0xff
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	x11, x11, x10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x12, #0x40
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x13, xzr, x9, ne
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     orr	x11, x9, x11
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x9, x9, x11, ne
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	x11, x13
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	x9, x9
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     adds	x11, x0, x11
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     adc	x9, x1, x9
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x11, x11, x2
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x12, #0x40
 -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x13, x9, #1
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x10, x13, x10
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	x13, x9, x2
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	x9, x9, #63
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x10, x10, x11
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x9, x9, x13, ne
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x10, x13, x10, ne
 -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -     stp	x10, x9, [x8, #16]
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w10, #1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -     1.00    -      -      -      -      -     stp	x10, xzr, [x8]
```
## `checked_floor_to_multiple_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      7
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.86
IPC:               0.86
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	w8, #-1
 1      1     0.25                        tst	w1, #0xf8
 1      1     0.50                        lsl	w9, w8, w1
 1      1     0.25                        cset	w8, eq
 1      2     1.00                        and	w1, w9, w0
 1      2     1.00                        mov	w0, w8

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   1.00   3.00   3.00    -      -     2.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w1, #0xf8
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w9, w8, w1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w8, eq
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     and	w1, w9, w0
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mov	w0, w8
```
## `checked_floor_to_multiple_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      7
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.86
IPC:               0.86
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	w8, #-1
 1      1     0.25                        tst	w1, #0xf0
 1      1     0.50                        lsl	w9, w8, w1
 1      1     0.25                        cset	w8, eq
 1      2     1.00                        and	w1, w9, w0
 1      2     1.00                        mov	w0, w8

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   1.00   3.00   3.00    -      -     2.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w1, #0xf0
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w9, w8, w1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w8, eq
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     and	w1, w9, w0
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mov	w0, w8
```
## `checked_floor_to_multiple_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      7
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.86
IPC:               0.86
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	w8, #-1
 1      1     0.25                        tst	w1, #0xe0
 1      1     0.50                        lsl	w9, w8, w1
 1      1     0.25                        cset	w8, eq
 1      2     1.00                        and	w1, w9, w0
 1      2     1.00                        mov	w0, w8

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   1.00   3.00   3.00    -      -     2.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w1, #0xe0
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w9, w8, w1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w8, eq
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     and	w1, w9, w0
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mov	w0, w8
```
## `checked_floor_to_multiple_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      7
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.86
IPC:               0.86
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	x8, #-1
 1      1     0.25                        tst	w1, #0xc0
 1      1     0.50                        lsl	x9, x8, x1
 1      1     0.25                        cset	w8, eq
 1      2     1.00                        and	x1, x9, x0
 1      2     1.00                        mov	x0, x8

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   1.00   3.00   3.00    -      -     2.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w1, #0xc0
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x9, x8, x1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w8, eq
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     and	x1, x9, x0
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mov	x0, x8
```
## `checked_floor_to_multiple_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      18
Total Cycles:      18
Total uOps:        21

Dispatch Width:    6
uOps Per Cycle:    1.17
IPC:               1.00
Block RThroughput: 5.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.50                        tbnz	w2, #7, .LBB60_2
 1      1     0.25                        mov	x9, #-1
 1      2     1.00                        mvn	w10, w2
 1      1     0.25                        mov	x11, #9223372036854775807
 1      1     0.50                        lsl	x9, x9, x2
 1      1     0.50                        lsr	x10, x11, x10
 1      1     0.25                        and	x12, x2, #0xff
 1      1     0.25                        tst	x12, #0x40
 1      2     1.00                        orr	x10, x9, x10
 1      1     0.25                        csel	x11, xzr, x9, ne
 1      1     0.25                        csel	x9, x9, x10, ne
 1      2     1.00                        and	x10, x11, x0
 1      2     1.00                        and	x9, x9, x1
 2      6     0.50           *            stp	x10, x9, [x8, #16]
 1      1     0.25                        mov	w9, #1
 2      6     0.50           *            stp	x9, xzr, [x8]
 1      0     1.00                  U     ret
 2      6     0.50           *            stp	xzr, xzr, [x8]

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   1.00   1.00    -      -     3.00   6.00   5.00   5.00    -      -     5.00   5.00    -     1.00   2.00   1.00   1.00   1.00    -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tbnz	w2, #7, .LBB60_2
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x9, #-1
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	w10, w2
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x11, #9223372036854775807
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x9, x9, x2
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x10, x11, x10
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x12, x2, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x12, #0x40
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x10, x9, x10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x11, xzr, x9, ne
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x9, x9, x10, ne
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     and	x10, x11, x0
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     and	x9, x9, x1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -     stp	x10, x9, [x8, #16]
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w9, #1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     stp	x9, xzr, [x8]
1.00    -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00   1.00    -      -      -      -      -      -     stp	xzr, xzr, [x8]
```
## `checked_mod_floor_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      7
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.86
IPC:               0.86
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	w8, #-1
 1      1     0.25                        tst	w1, #0xf8
 1      1     0.50                        lsl	w9, w8, w1
 1      1     0.25                        cset	w8, eq
 1      2     1.00                        bic	w1, w0, w9
 1      2     1.00                        mov	w0, w8

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   1.00   3.00   3.00    -      -     2.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w1, #0xf8
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w9, w8, w1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w8, eq
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     bic	w1, w0, w9
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mov	w0, w8
```
## `checked_mod_floor_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      7
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.86
IPC:               0.86
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	w8, #-1
 1      1     0.25                        tst	w1, #0xf0
 1      1     0.50                        lsl	w9, w8, w1
 1      1     0.25                        cset	w8, eq
 1      2     1.00                        bic	w1, w0, w9
 1      2     1.00                        mov	w0, w8

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   1.00   3.00   3.00    -      -     2.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w1, #0xf0
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w9, w8, w1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w8, eq
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     bic	w1, w0, w9
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mov	w0, w8
```
## `checked_mod_floor_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      7
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.86
IPC:               0.86
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	w8, #-1
 1      1     0.25                        tst	w1, #0xe0
 1      1     0.50                        lsl	w9, w8, w1
 1      1     0.25                        cset	w8, eq
 1      2     1.00                        bic	w1, w0, w9
 1      2     1.00                        mov	w0, w8

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   1.00   3.00   3.00    -      -     2.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w1, #0xe0
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w9, w8, w1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w8, eq
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     bic	w1, w0, w9
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mov	w0, w8
```
## `checked_mod_floor_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      7
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.86
IPC:               0.86
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	x8, #-1
 1      1     0.25                        tst	w1, #0xc0
 1      1     0.50                        lsl	x9, x8, x1
 1      1     0.25                        cset	w8, eq
 1      2     1.00                        bic	x1, x0, x9
 1      2     1.00                        mov	x0, x8

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   1.00   3.00   3.00    -      -     2.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w1, #0xc0
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x9, x8, x1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w8, eq
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     bic	x1, x0, x9
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mov	x0, x8
```
## `checked_mod_floor_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      18
Total Cycles:      18
Total uOps:        21

Dispatch Width:    6
uOps Per Cycle:    1.17
IPC:               1.00
Block RThroughput: 5.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.50                        tbnz	w2, #7, .LBB65_2
 1      1     0.25                        mov	x9, #-1
 1      2     1.00                        mvn	w10, w2
 1      1     0.25                        mov	x11, #9223372036854775807
 1      1     0.50                        lsl	x9, x9, x2
 1      1     0.50                        lsr	x10, x11, x10
 1      1     0.25                        and	x12, x2, #0xff
 1      1     0.25                        tst	x12, #0x40
 1      2     1.00                        orr	x10, x9, x10
 1      1     0.25                        csel	x11, xzr, x9, ne
 1      1     0.25                        csel	x9, x9, x10, ne
 1      2     1.00                        bic	x10, x0, x11
 1      2     1.00                        bic	x9, x1, x9
 2      6     0.50           *            stp	x10, x9, [x8, #16]
 1      1     0.25                        mov	w9, #1
 2      6     0.50           *            stp	x9, xzr, [x8]
 1      0     1.00                  U     ret
 2      6     0.50           *            stp	xzr, xzr, [x8]

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   1.00   1.00    -      -     3.00   6.00   5.00   5.00    -      -     5.00   5.00    -     1.00   2.00   1.00   1.00   1.00    -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tbnz	w2, #7, .LBB65_2
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x9, #-1
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	w10, w2
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x11, #9223372036854775807
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x9, x9, x2
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x10, x11, x10
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x12, x2, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x12, #0x40
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x10, x9, x10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x11, xzr, x9, ne
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x9, x9, x10, ne
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     bic	x10, x0, x11
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     bic	x9, x1, x9
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -     stp	x10, x9, [x8, #16]
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w9, #1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     stp	x9, xzr, [x8]
1.00    -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00   1.00    -      -      -      -      -      -     stp	xzr, xzr, [x8]
```
## `checked_mul_i8_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      9
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    0.78
IPC:               0.78
Block RThroughput: 3.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsl	w8, w0, w1
 1      1     0.25                        and	w10, w0, #0xff
 1      1     0.50                        sxtb	w9, w8
 1      1     0.50                        asr	w9, w9, w1
 1      2     1.00                        mov	w1, w8
 1      2     1.00                        cmp	w10, w9, uxtb
 1      1     0.25                        cset	w0, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   2.00   3.00   2.00    -      -     2.00   5.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w0, w1
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w10, w0, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     sxtb	w9, w8
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	w9, w9, w1
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mov	w1, w8
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     cmp	w10, w9, uxtb
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, eq
```
## `checked_mul_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      9
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.11
IPC:               1.11
Block RThroughput: 2.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsl	w8, w0, w1
 1      1     0.25                        and	x10, x1, #0x7
 1      1     0.25                        tst	w1, #0xf8
 1      1     0.50                        sxtb	w9, w8
 1      2     1.00                        mov	w1, w8
 1      1     0.50                        asr	w9, w9, w10
 1      1     0.25                        and	w10, w0, #0xff
 1      1     0.25                        and	w9, w9, #0xff
 1      1     0.25                        ccmp	w10, w9, #0, eq
 1      1     0.25                        cset	w0, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   2.00   3.00   4.00    -      -     2.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w0, w1
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x10, x1, #0x7
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w1, #0xf8
 -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     sxtb	w9, w8
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mov	w1, w8
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	w9, w9, w10
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w10, w0, #0xff
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w9, w9, #0xff
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     ccmp	w10, w9, #0, eq
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, eq
```
## `checked_mul_i16_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      9
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    0.78
IPC:               0.78
Block RThroughput: 3.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsl	w8, w0, w1
 1      1     0.25                        and	w10, w0, #0xffff
 1      1     0.50                        sxth	w9, w8
 1      1     0.50                        asr	w9, w9, w1
 1      2     1.00                        mov	w1, w8
 1      2     1.00                        cmp	w10, w9, uxth
 1      1     0.25                        cset	w0, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   2.00   3.00   2.00    -      -     2.00   5.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w0, w1
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w10, w0, #0xffff
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     sxth	w9, w8
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	w9, w9, w1
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mov	w1, w8
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     cmp	w10, w9, uxth
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, eq
```
## `checked_mul_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      12
Total Cycles:      12
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 4.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w1, #0xff
 1      1     0.25                        cmp	w8, #15
 1      0     0.50                        b.hi	.LBB73_2
 1      2     1.00                        mov	w8, w1
 1      1     0.50                        lsl	w1, w0, w1
 1      1     0.50                        sxth	w9, w1
 1      1     0.50                        asr	w8, w9, w8
 1      1     0.25                        and	w9, w0, #0xffff
 1      2     1.00                        cmp	w9, w8, uxth
 1      1     0.25                        cset	w0, eq
 1      0     1.00                  U     ret
 1      2     1.00                        mov	w0, wzr

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   1.00   1.00    -      -     5.00   3.00   4.00   3.00    -      -     4.00   5.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w1, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w8, #15
1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.hi	.LBB73_2
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mov	w8, w1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w1, w0, w1
 -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     sxth	w9, w1
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	w8, w9, w8
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w9, w0, #0xffff
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     cmp	w9, w8, uxth
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, eq
 -     1.00   1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mov	w0, wzr
```
## `checked_mul_i32_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      8
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.63
IPC:               0.63
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsl	w8, w0, w1
 1      1     0.50                        asr	w9, w8, w1
 1      2     1.00                        mov	w1, w8
 1      2     1.00                        cmp	w0, w9
 1      1     0.25                        cset	w0, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   2.00   1.00   2.00    -      -     3.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w0, w1
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	w9, w8, w1
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mov	w1, w8
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     cmp	w0, w9
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, eq
```
## `checked_mul_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      12
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    0.83
IPC:               0.83
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w1, #0xff
 1      1     0.25                        cmp	w8, #31
 1      0     0.50                        b.hi	.LBB75_2
 1      2     1.00                        mov	w8, w1
 1      1     0.50                        lsl	w1, w0, w1
 1      1     0.50                        asr	w8, w1, w8
 1      2     1.00                        cmp	w0, w8
 1      1     0.25                        cset	w0, eq
 1      0     1.00                  U     ret
 1      2     1.00                        mov	w0, wzr

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   1.00   1.00    -      -     3.00   2.00   5.00   3.00    -      -     4.00   4.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w1, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w8, #31
1.00    -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.hi	.LBB75_2
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mov	w8, w1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w1, w0, w1
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	w8, w1, w8
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     cmp	w0, w8
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, eq
 -     1.00   1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mov	w0, wzr
```
## `checked_mul_i64_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      8
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.63
IPC:               0.63
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsl	x8, x0, x1
 1      1     0.50                        asr	x9, x8, x1
 1      2     1.00                        mov	x1, x8
 1      2     1.00                        cmp	x0, x9
 1      1     0.25                        cset	w0, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   2.00   1.00   2.00    -      -     3.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x0, x1
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	x9, x8, x1
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mov	x1, x8
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     cmp	x0, x9
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, eq
```
## `checked_mul_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      7
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.86
IPC:               0.86
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsl	x8, x0, x1
 1      1     0.25                        tst	w1, #0xc0
 1      1     0.50                        asr	x9, x8, x1
 1      2     1.00                        mov	x1, x8
 1      1     0.25                        ccmp	x0, x9, #0, eq
 1      1     0.25                        cset	w0, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   1.00   2.00   2.00    -      -     1.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x0, x1
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w1, #0xc0
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	x9, x8, x1
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mov	x1, x8
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     ccmp	x0, x9, #0, eq
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, eq
```
## `checked_mul_i128_pow2`
```asm
Iterations:        1
Instructions:      25
Total Cycles:      22
Total uOps:        27

Dispatch Width:    6
uOps Per Cycle:    1.23
IPC:               1.14
Block RThroughput: 9.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsr	x9, x0, #1
 1      2     1.00                        mvn	w11, w2
 1      1     0.50                        lsl	x10, x1, x2
 1      1     0.50                        lsl	x12, x0, x2
 1      1     0.25                        and	x13, x2, #0xff
 1      1     0.50                        lsr	x9, x9, x11
 1      1     0.25                        tst	x13, #0x40
 1      2     1.00                        orr	x9, x10, x9
 1      1     0.25                        csel	x10, xzr, x12, ne
 1      1     0.25                        csel	x9, x12, x9, ne
 1      1     0.50                        lsr	x13, x10, x2
 1      1     0.50                        lsl	x12, x9, #1
 1      1     0.50                        asr	x14, x9, #63
 1      1     0.50                        lsl	x11, x12, x11
 1      1     0.50                        asr	x12, x9, x2
 1      2     1.00                        orr	x11, x11, x13
 1      1     0.25                        csel	x13, x14, x12, ne
 1      1     0.25                        csel	x11, x12, x11, ne
 1      2     1.00                        cmp	x1, x13
 1      1     0.25                        ccmp	x0, x11, #0, eq
 1      2     1.00                        mov	x11, xzr
 1      0     0.50                        b.ne	.LBB70_2
 1      1     0.25                        mov	w11, #1
 2      6     0.50           *            stp	x10, x9, [x8, #16]
 2      6     0.50           *            stp	x11, xzr, [x8]

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -     1.00    -      -      -     6.00   7.00   8.00   7.00    -      -     11.00  8.00    -     1.00   1.00    -     1.00   1.00    -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x9, x0, #1
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	w11, w2
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x10, x1, x2
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x12, x0, x2
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x13, x2, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x9, x9, x11
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x13, #0x40
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x9, x10, x9
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x10, xzr, x12, ne
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x9, x12, x9, ne
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x13, x10, x2
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x12, x9, #1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	x14, x9, #63
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x11, x12, x11
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	x12, x9, x2
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x11, x11, x13
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x13, x14, x12, ne
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x11, x12, x11, ne
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     cmp	x1, x13
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     ccmp	x0, x11, #0, eq
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mov	x11, xzr
 -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.ne	.LBB70_2
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w11, #1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -     stp	x10, x9, [x8, #16]
 -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     stp	x11, xzr, [x8]
```
## `checked_mul_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      27
Total Cycles:      20
Total uOps:        30

Dispatch Width:    6
uOps Per Cycle:    1.50
IPC:               1.35
Block RThroughput: 8.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.50                        tbnz	w2, #7, .LBB71_3
 1      1     0.50                        lsr	x9, x0, #1
 1      2     1.00                        mvn	w11, w2
 1      1     0.50                        lsl	x10, x1, x2
 1      1     0.50                        lsl	x12, x0, x2
 1      1     0.25                        and	x13, x2, #0xff
 1      1     0.50                        lsr	x9, x9, x11
 1      1     0.25                        tst	x13, #0x40
 1      2     1.00                        orr	x9, x10, x9
 1      1     0.25                        csel	x10, xzr, x12, ne
 1      1     0.25                        csel	x9, x12, x9, ne
 1      1     0.50                        lsr	x13, x10, x2
 1      1     0.50                        lsl	x12, x9, #1
 1      1     0.50                        asr	x14, x9, #63
 1      1     0.50                        lsl	x11, x12, x11
 1      1     0.50                        asr	x12, x9, x2
 1      2     1.00                        orr	x11, x11, x13
 1      1     0.25                        csel	x13, x14, x12, ne
 1      1     0.25                        csel	x11, x12, x11, ne
 1      2     1.00                        cmp	x1, x13
 1      1     0.25                        ccmp	x0, x11, #0, eq
 1      0     0.50                        b.ne	.LBB71_3
 2      6     0.50           *            stp	x10, x9, [x8, #16]
 1      1     0.25                        mov	w10, #1
 2      6     0.50           *            stp	x10, xzr, [x8]
 1      0     1.00                  U     ret
 2      6     0.50           *            stp	xzr, xzr, [x8]

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   2.00   1.00    -      -     7.00   7.00   7.00   7.00    -      -     9.00   8.00    -     1.00   2.00   1.00   1.00   1.00    -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tbnz	w2, #7, .LBB71_3
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x9, x0, #1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	w11, w2
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x10, x1, x2
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x12, x0, x2
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x13, x2, #0xff
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x9, x9, x11
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x13, #0x40
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x9, x10, x9
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x10, xzr, x12, ne
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x9, x12, x9, ne
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x13, x10, x2
 -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x12, x9, #1
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	x14, x9, #63
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x11, x12, x11
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	x12, x9, x2
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x11, x11, x13
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x13, x14, x12, ne
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x11, x12, x11, ne
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     cmp	x1, x13
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ccmp	x0, x11, #0, eq
 -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.ne	.LBB71_3
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -     stp	x10, x9, [x8, #16]
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w10, #1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     stp	x10, xzr, [x8]
1.00    -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00   1.00    -      -      -      -      -      -     stp	xzr, xzr, [x8]
```
## `checked_mul_u8_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      9
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.67
IPC:               0.67
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsl	w8, w0, w1
 1      1     0.25                        and	w9, w8, #0xff
 1      1     0.50                        lsr	w9, w9, w1
 1      2     1.00                        mov	w1, w8
 1      2     1.00                        cmp	w9, w0, uxtb
 1      1     0.25                        cset	w0, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   2.00   2.00   3.00    -      -     4.00   2.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w0, w1
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w9, w8, #0xff
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	w9, w9, w1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mov	w1, w8
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     cmp	w9, w0, uxtb
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, eq
```
## `checked_mul_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      8
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    1.13
IPC:               1.13
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsl	w8, w0, w1
 1      1     0.25                        and	x10, x1, #0x7
 1      1     0.25                        tst	w1, #0xf8
 1      1     0.25                        and	w9, w8, #0xff
 1      2     1.00                        mov	w1, w8
 1      1     0.50                        lsr	w9, w9, w10
 1      1     0.25                        and	w10, w0, #0xff
 1      1     0.25                        ccmp	w9, w10, #0, eq
 1      1     0.25                        cset	w0, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   2.00   2.00   4.00    -      -     2.00   2.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w0, w1
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x10, x1, #0x7
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w1, #0xf8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w9, w8, #0xff
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mov	w1, w8
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	w9, w9, w10
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w10, w0, #0xff
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ccmp	w9, w10, #0, eq
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, eq
```
## `checked_mul_u16_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      9
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.67
IPC:               0.67
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsl	w8, w0, w1
 1      1     0.25                        and	w9, w8, #0xffff
 1      1     0.50                        lsr	w9, w9, w1
 1      2     1.00                        mov	w1, w8
 1      2     1.00                        cmp	w9, w0, uxth
 1      1     0.25                        cset	w0, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   2.00   2.00   3.00    -      -     4.00   2.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w0, w1
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w9, w8, #0xffff
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	w9, w9, w1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mov	w1, w8
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     cmp	w9, w0, uxth
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, eq
```
## `checked_mul_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      12
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    0.92
IPC:               0.92
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w1, #0xff
 1      1     0.25                        cmp	w8, #15
 1      0     0.50                        b.hi	.LBB83_2
 1      2     1.00                        mov	w8, w1
 1      1     0.50                        lsl	w1, w0, w1
 1      1     0.25                        and	w9, w1, #0xffff
 1      1     0.50                        lsr	w8, w9, w8
 1      2     1.00                        cmp	w8, w0, uxth
 1      1     0.25                        cset	w0, eq
 1      0     1.00                  U     ret
 1      2     1.00                        mov	w0, wzr

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   1.00   1.00    -      -     2.00   5.00   3.00   4.00    -      -     4.00   4.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w1, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w8, #15
1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.hi	.LBB83_2
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mov	w8, w1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w1, w0, w1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w9, w1, #0xffff
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	w8, w9, w8
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     cmp	w8, w0, uxth
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, eq
 -     1.00   1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mov	w0, wzr
```
## `checked_mul_u32_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      8
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.63
IPC:               0.63
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsl	w8, w0, w1
 1      1     0.50                        lsr	w9, w8, w1
 1      2     1.00                        mov	w1, w8
 1      2     1.00                        cmp	w0, w9
 1      1     0.25                        cset	w0, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   2.00   1.00   2.00    -      -     3.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w0, w1
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	w9, w8, w1
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mov	w1, w8
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     cmp	w0, w9
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, eq
```
## `checked_mul_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      12
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    0.83
IPC:               0.83
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w1, #0xff
 1      1     0.25                        cmp	w8, #31
 1      0     0.50                        b.hi	.LBB85_2
 1      2     1.00                        mov	w8, w1
 1      1     0.50                        lsl	w1, w0, w1
 1      1     0.50                        lsr	w8, w1, w8
 1      2     1.00                        cmp	w0, w8
 1      1     0.25                        cset	w0, eq
 1      0     1.00                  U     ret
 1      2     1.00                        mov	w0, wzr

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   1.00   1.00    -      -     3.00   2.00   5.00   3.00    -      -     4.00   4.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w1, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w8, #31
1.00    -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.hi	.LBB85_2
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mov	w8, w1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w1, w0, w1
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	w8, w1, w8
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     cmp	w0, w8
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, eq
 -     1.00   1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mov	w0, wzr
```
## `checked_mul_u64_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      8
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.63
IPC:               0.63
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsl	x8, x0, x1
 1      1     0.50                        lsr	x9, x8, x1
 1      2     1.00                        mov	x1, x8
 1      2     1.00                        cmp	x0, x9
 1      1     0.25                        cset	w0, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   2.00   1.00   2.00    -      -     3.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x0, x1
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	x9, x8, x1
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mov	x1, x8
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     cmp	x0, x9
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, eq
```
## `checked_mul_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      7
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.86
IPC:               0.86
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsl	x8, x0, x1
 1      1     0.25                        tst	w1, #0xc0
 1      1     0.50                        lsr	x9, x8, x1
 1      2     1.00                        mov	x1, x8
 1      1     0.25                        ccmp	x0, x9, #0, eq
 1      1     0.25                        cset	w0, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   1.00   2.00   2.00    -      -     1.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x0, x1
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w1, #0xc0
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	x9, x8, x1
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mov	x1, x8
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     ccmp	x0, x9, #0, eq
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, eq
```
## `checked_mul_u128_pow2`
```asm
Iterations:        1
Instructions:      24
Total Cycles:      21
Total uOps:        26

Dispatch Width:    6
uOps Per Cycle:    1.24
IPC:               1.14
Block RThroughput: 9.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsr	x9, x0, #1
 1      2     1.00                        mvn	w11, w2
 1      1     0.50                        lsl	x10, x1, x2
 1      1     0.50                        lsl	x12, x0, x2
 1      1     0.25                        and	x13, x2, #0xff
 1      1     0.50                        lsr	x9, x9, x11
 1      1     0.25                        tst	x13, #0x40
 1      2     1.00                        orr	x9, x10, x9
 1      1     0.25                        csel	x10, xzr, x12, ne
 1      1     0.25                        csel	x9, x12, x9, ne
 1      1     0.50                        lsr	x12, x10, x2
 1      1     0.50                        lsl	x13, x9, #1
 1      1     0.50                        lsl	x11, x13, x11
 1      1     0.50                        lsr	x13, x9, x2
 1      2     1.00                        orr	x11, x11, x12
 1      1     0.25                        csel	x12, xzr, x13, ne
 1      1     0.25                        csel	x11, x13, x11, ne
 1      2     1.00                        cmp	x1, x12
 1      1     0.25                        ccmp	x0, x11, #0, eq
 1      2     1.00                        mov	x11, xzr
 1      0     0.50                        b.ne	.LBB80_2
 1      1     0.25                        mov	w11, #1
 2      6     0.50           *            stp	x10, x9, [x8, #16]
 2      6     0.50           *            stp	x11, xzr, [x8]

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -     1.00    -      -      -     7.00   5.00   8.00   7.00    -      -     9.00   9.00    -     1.00   1.00    -     1.00   1.00    -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x9, x0, #1
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	w11, w2
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x10, x1, x2
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x12, x0, x2
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x13, x2, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x9, x9, x11
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x13, #0x40
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x9, x10, x9
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x10, xzr, x12, ne
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x9, x12, x9, ne
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x12, x10, x2
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x13, x9, #1
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x11, x13, x11
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x13, x9, x2
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     orr	x11, x11, x12
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x12, xzr, x13, ne
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x11, x13, x11, ne
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     cmp	x1, x12
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     ccmp	x0, x11, #0, eq
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mov	x11, xzr
 -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.ne	.LBB80_2
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w11, #1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -     stp	x10, x9, [x8, #16]
 -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     stp	x11, xzr, [x8]
```
## `checked_mul_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      26
Total Cycles:      19
Total uOps:        29

Dispatch Width:    6
uOps Per Cycle:    1.53
IPC:               1.37
Block RThroughput: 8.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.50                        tbnz	w2, #7, .LBB81_3
 1      1     0.50                        lsr	x9, x0, #1
 1      2     1.00                        mvn	w11, w2
 1      1     0.50                        lsl	x10, x1, x2
 1      1     0.50                        lsl	x12, x0, x2
 1      1     0.25                        and	x13, x2, #0xff
 1      1     0.50                        lsr	x9, x9, x11
 1      1     0.25                        tst	x13, #0x40
 1      2     1.00                        orr	x9, x10, x9
 1      1     0.25                        csel	x10, xzr, x12, ne
 1      1     0.25                        csel	x9, x12, x9, ne
 1      1     0.50                        lsr	x12, x10, x2
 1      1     0.50                        lsl	x13, x9, #1
 1      1     0.50                        lsl	x11, x13, x11
 1      1     0.50                        lsr	x13, x9, x2
 1      2     1.00                        orr	x11, x11, x12
 1      1     0.25                        csel	x12, xzr, x13, ne
 1      1     0.25                        csel	x11, x13, x11, ne
 1      2     1.00                        cmp	x1, x12
 1      1     0.25                        ccmp	x0, x11, #0, eq
 1      0     0.50                        b.ne	.LBB81_3
 2      6     0.50           *            stp	x10, x9, [x8, #16]
 1      1     0.25                        mov	w10, #1
 2      6     0.50           *            stp	x10, xzr, [x8]
 1      0     1.00                  U     ret
 2      6     0.50           *            stp	xzr, xzr, [x8]

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   2.00   1.00    -      -     7.00   7.00   6.00   7.00    -      -     9.00   7.00    -     1.00   2.00   1.00   1.00   1.00    -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tbnz	w2, #7, .LBB81_3
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x9, x0, #1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	w11, w2
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x10, x1, x2
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x12, x0, x2
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x13, x2, #0xff
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x9, x9, x11
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x13, #0x40
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x9, x10, x9
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x10, xzr, x12, ne
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x9, x12, x9, ne
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x12, x10, x2
 -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x13, x9, #1
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x11, x13, x11
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x13, x9, x2
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     orr	x11, x11, x12
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x12, xzr, x13, ne
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x11, x13, x11, ne
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     cmp	x1, x12
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ccmp	x0, x11, #0, eq
 -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.ne	.LBB81_3
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -     stp	x10, x9, [x8, #16]
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w10, #1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     stp	x10, xzr, [x8]
1.00    -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00   1.00    -      -      -      -      -      -     stp	xzr, xzr, [x8]
```
## `div_ceil_i8_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      10
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.50
IPC:               0.50
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	x8, #-1
 1      1     0.50                        lsl	x8, x8, x1
 1      2     1.00                        mvn	x8, x8
 1      2     1.00                        add	x8, x8, w0, sxtb
 1      1     0.50                        asr	x0, x8, x1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   2.00   1.00   2.00    -      -     3.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	x8, x8
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     add	x8, x8, w0, sxtb
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	x0, x8, x1
```
## `div_ceil_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      10
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.50
IPC:               0.50
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	x8, #-1
 1      1     0.50                        lsl	x8, x8, x1
 1      2     1.00                        mvn	x8, x8
 1      2     1.00                        add	x8, x8, w0, sxtb
 1      1     0.50                        asr	x0, x8, x1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   2.00   1.00   2.00    -      -     3.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	x8, x8
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     add	x8, x8, w0, sxtb
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	x0, x8, x1
```
## `div_ceil_i16_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      10
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.50
IPC:               0.50
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	x8, #-1
 1      1     0.50                        lsl	x8, x8, x1
 1      2     1.00                        mvn	x8, x8
 1      2     1.00                        add	x8, x8, w0, sxth
 1      1     0.50                        asr	x0, x8, x1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   2.00   1.00   2.00    -      -     3.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	x8, x8
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     add	x8, x8, w0, sxth
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	x0, x8, x1
```
## `div_ceil_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      10
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.50
IPC:               0.50
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	x8, #-1
 1      1     0.50                        lsl	x8, x8, x1
 1      2     1.00                        mvn	x8, x8
 1      2     1.00                        add	x8, x8, w0, sxth
 1      1     0.50                        asr	x0, x8, x1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   2.00   1.00   2.00    -      -     3.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	x8, x8
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     add	x8, x8, w0, sxth
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	x0, x8, x1
```
## `div_ceil_i32_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      10
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.50
IPC:               0.50
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	x8, #-1
 1      1     0.50                        lsl	x8, x8, x1
 1      2     1.00                        mvn	x8, x8
 1      2     1.00                        add	x8, x8, w0, sxtw
 1      1     0.50                        asr	x0, x8, x1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   2.00   1.00   2.00    -      -     3.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	x8, x8
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     add	x8, x8, w0, sxtw
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	x0, x8, x1
```
## `div_ceil_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      10
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.50
IPC:               0.50
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	x8, #-1
 1      1     0.50                        lsl	x8, x8, x1
 1      2     1.00                        mvn	x8, x8
 1      2     1.00                        add	x8, x8, w0, sxtw
 1      1     0.50                        asr	x0, x8, x1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   2.00   1.00   2.00    -      -     3.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	x8, x8
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     add	x8, x8, w0, sxtw
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	x0, x8, x1
```
## `div_ceil_i64_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      8
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.50
IPC:               0.50
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        asr	x8, x0, x1
 1      1     0.50                        lsl	x9, x8, x1
 1      2     1.00                        cmp	x0, x9
 1      1     0.25                        cinc	x0, x8, ne

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   2.00   1.00   1.00    -      -     1.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	x8, x0, x1
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x9, x8, x1
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     cmp	x0, x9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cinc	x0, x8, ne
```
## `div_ceil_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      8
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.50
IPC:               0.50
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        asr	x8, x0, x1
 1      1     0.50                        lsl	x9, x8, x1
 1      2     1.00                        cmp	x0, x9
 1      1     0.25                        cinc	x0, x8, ne

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   2.00   1.00   1.00    -      -     1.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	x8, x0, x1
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x9, x8, x1
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     cmp	x0, x9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cinc	x0, x8, ne
```
## `div_ceil_i128_pow2`
```asm
Iterations:        1
Instructions:      23
Total Cycles:      19
Total uOps:        23

Dispatch Width:    6
uOps Per Cycle:    1.21
IPC:               1.21
Block RThroughput: 9.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsl	x8, x1, #1
 1      2     1.00                        mvn	w9, w2
 1      1     0.50                        lsr	x10, x0, x2
 1      1     0.50                        asr	x11, x1, x2
 1      1     0.25                        and	x12, x2, #0xff
 1      1     0.50                        lsl	x8, x8, x9
 1      1     0.25                        tst	x12, #0x40
 1      2     1.00                        orr	x8, x8, x10
 1      1     0.50                        asr	x10, x1, #63
 1      1     0.25                        csel	x8, x11, x8, ne
 1      1     0.50                        lsr	x12, x8, #1
 1      1     0.25                        csel	x10, x10, x11, ne
 1      1     0.50                        lsl	x11, x10, x2
 1      1     0.50                        lsr	x9, x12, x9
 1      1     0.50                        lsl	x12, x8, x2
 1      2     1.00                        orr	x9, x11, x9
 1      1     0.25                        csel	x11, xzr, x12, ne
 1      1     0.25                        csel	x9, x12, x9, ne
 1      2     1.00                        cmp	x0, x11
 1      1     0.25                        ccmp	x1, x9, #0, eq
 1      1     0.25                        cset	w9, ne
 1      2     1.00                        adds	x0, x8, x9
 1      1     0.25                        cinc	x1, x10, hs

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     6.00   6.00   9.00   7.00    -      -     11.00  8.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x1, #1
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	w9, w2
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x10, x0, x2
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	x11, x1, x2
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x12, x2, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x9
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x12, #0x40
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x8, x8, x10
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	x10, x1, #63
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x8, x11, x8, ne
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x12, x8, #1
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x10, x10, x11, ne
 -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x11, x10, x2
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x9, x12, x9
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x12, x8, x2
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x9, x11, x9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x11, xzr, x12, ne
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x9, x12, x9, ne
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     cmp	x0, x11
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ccmp	x1, x9, #0, eq
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w9, ne
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     adds	x0, x8, x9
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cinc	x1, x10, hs
```
## `div_ceil_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      23
Total Cycles:      19
Total uOps:        23

Dispatch Width:    6
uOps Per Cycle:    1.21
IPC:               1.21
Block RThroughput: 9.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsl	x8, x1, #1
 1      2     1.00                        mvn	w9, w2
 1      1     0.50                        lsr	x10, x0, x2
 1      1     0.50                        asr	x11, x1, x2
 1      1     0.25                        and	x12, x2, #0x7f
 1      1     0.50                        lsl	x8, x8, x9
 1      1     0.25                        tst	x12, #0x40
 1      2     1.00                        orr	x8, x8, x10
 1      1     0.50                        asr	x10, x1, #63
 1      1     0.25                        csel	x8, x11, x8, ne
 1      1     0.50                        lsr	x12, x8, #1
 1      1     0.25                        csel	x10, x10, x11, ne
 1      1     0.50                        lsl	x11, x10, x2
 1      1     0.50                        lsr	x9, x12, x9
 1      1     0.50                        lsl	x12, x8, x2
 1      2     1.00                        orr	x9, x11, x9
 1      1     0.25                        csel	x11, xzr, x12, ne
 1      1     0.25                        csel	x9, x12, x9, ne
 1      2     1.00                        cmp	x0, x11
 1      1     0.25                        ccmp	x1, x9, #0, eq
 1      1     0.25                        cset	w9, ne
 1      2     1.00                        adds	x0, x8, x9
 1      1     0.25                        cinc	x1, x10, hs

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     6.00   6.00   9.00   7.00    -      -     11.00  8.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x1, #1
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	w9, w2
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x10, x0, x2
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	x11, x1, x2
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x12, x2, #0x7f
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x9
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x12, #0x40
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x8, x8, x10
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	x10, x1, #63
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x8, x11, x8, ne
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x12, x8, #1
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x10, x10, x11, ne
 -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x11, x10, x2
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x9, x12, x9
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x12, x8, x2
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x9, x11, x9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x11, xzr, x12, ne
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x9, x12, x9, ne
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     cmp	x0, x11
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ccmp	x1, x9, #0, eq
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w9, ne
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     adds	x0, x8, x9
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cinc	x1, x10, hs
```
## `div_ceil_u8_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      10
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.50
IPC:               0.50
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	x8, #-1
 1      1     0.50                        lsl	x8, x8, x1
 1      2     1.00                        mvn	x8, x8
 1      2     1.00                        add	x8, x8, w0, uxtb
 1      1     0.50                        asr	x0, x8, x1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   2.00   1.00   2.00    -      -     3.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	x8, x8
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     add	x8, x8, w0, uxtb
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	x0, x8, x1
```
## `div_ceil_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      10
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.50
IPC:               0.50
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	x8, #-1
 1      1     0.50                        lsl	x8, x8, x1
 1      2     1.00                        mvn	x8, x8
 1      2     1.00                        add	x8, x8, w0, uxtb
 1      1     0.50                        asr	x0, x8, x1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   2.00   1.00   2.00    -      -     3.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	x8, x8
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     add	x8, x8, w0, uxtb
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	x0, x8, x1
```
## `div_ceil_u16_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      10
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.50
IPC:               0.50
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	x8, #-1
 1      1     0.50                        lsl	x8, x8, x1
 1      2     1.00                        mvn	x8, x8
 1      2     1.00                        add	x8, x8, w0, uxth
 1      1     0.50                        asr	x0, x8, x1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   2.00   1.00   2.00    -      -     3.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	x8, x8
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     add	x8, x8, w0, uxth
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	x0, x8, x1
```
## `div_ceil_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      10
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.50
IPC:               0.50
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	x8, #-1
 1      1     0.50                        lsl	x8, x8, x1
 1      2     1.00                        mvn	x8, x8
 1      2     1.00                        add	x8, x8, w0, uxth
 1      1     0.50                        asr	x0, x8, x1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   2.00   1.00   2.00    -      -     3.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	x8, x8
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     add	x8, x8, w0, uxth
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	x0, x8, x1
```
## `div_ceil_u32_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      10
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.50
IPC:               0.50
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	x8, #-1
 1      1     0.50                        lsl	x8, x8, x1
 1      2     1.00                        mvn	x8, x8
 1      2     1.00                        add	x8, x8, w0, uxtw
 1      1     0.50                        asr	x0, x8, x1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   2.00   1.00   2.00    -      -     3.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	x8, x8
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     add	x8, x8, w0, uxtw
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	x0, x8, x1
```
## `div_ceil_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      10
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.50
IPC:               0.50
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	x8, #-1
 1      1     0.50                        lsl	x8, x8, x1
 1      2     1.00                        mvn	x8, x8
 1      2     1.00                        add	x8, x8, w0, uxtw
 1      1     0.50                        asr	x0, x8, x1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   2.00   1.00   2.00    -      -     3.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	x8, x8
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     add	x8, x8, w0, uxtw
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	x0, x8, x1
```
## `div_ceil_u64_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      8
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.50
IPC:               0.50
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsr	x8, x0, x1
 1      1     0.50                        lsl	x9, x8, x1
 1      2     1.00                        cmp	x0, x9
 1      1     0.25                        cinc	x0, x8, ne

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   2.00   1.00   1.00    -      -     1.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x8, x0, x1
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x9, x8, x1
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     cmp	x0, x9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cinc	x0, x8, ne
```
## `div_ceil_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      8
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.50
IPC:               0.50
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsr	x8, x0, x1
 1      1     0.50                        lsl	x9, x8, x1
 1      2     1.00                        cmp	x0, x9
 1      1     0.25                        cinc	x0, x8, ne

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   2.00   1.00   1.00    -      -     1.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x8, x0, x1
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x9, x8, x1
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     cmp	x0, x9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cinc	x0, x8, ne
```
## `div_ceil_u128_pow2`
```asm
Iterations:        1
Instructions:      22
Total Cycles:      19
Total uOps:        22

Dispatch Width:    6
uOps Per Cycle:    1.16
IPC:               1.16
Block RThroughput: 9.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsl	x8, x1, #1
 1      2     1.00                        mvn	w9, w2
 1      1     0.50                        lsr	x10, x0, x2
 1      1     0.50                        lsr	x11, x1, x2
 1      1     0.25                        and	x12, x2, #0xff
 1      1     0.50                        lsl	x8, x8, x9
 1      1     0.25                        tst	x12, #0x40
 1      2     1.00                        orr	x8, x8, x10
 1      1     0.25                        csel	x8, x11, x8, ne
 1      1     0.25                        csel	x11, xzr, x11, ne
 1      1     0.50                        lsr	x10, x8, #1
 1      1     0.50                        lsl	x12, x11, x2
 1      1     0.50                        lsr	x9, x10, x9
 1      1     0.50                        lsl	x10, x8, x2
 1      2     1.00                        orr	x9, x12, x9
 1      1     0.25                        csel	x12, xzr, x10, ne
 1      1     0.25                        csel	x9, x10, x9, ne
 1      2     1.00                        cmp	x0, x12
 1      1     0.25                        ccmp	x1, x9, #0, eq
 1      1     0.25                        cset	w9, ne
 1      2     1.00                        adds	x0, x8, x9
 1      1     0.25                        cinc	x1, x11, hs

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     7.00   5.00   7.00   8.00    -      -     9.00   9.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x1, #1
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	w9, w2
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x10, x0, x2
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	x11, x1, x2
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x12, x2, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x9
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x12, #0x40
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x8, x8, x10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x8, x11, x8, ne
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x11, xzr, x11, ne
 -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	x10, x8, #1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x12, x11, x2
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	x9, x10, x9
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x10, x8, x2
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     orr	x9, x12, x9
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x12, xzr, x10, ne
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x9, x10, x9, ne
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     cmp	x0, x12
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ccmp	x1, x9, #0, eq
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w9, ne
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     adds	x0, x8, x9
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cinc	x1, x11, hs
```
## `div_ceil_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      22
Total Cycles:      19
Total uOps:        22

Dispatch Width:    6
uOps Per Cycle:    1.16
IPC:               1.16
Block RThroughput: 9.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsl	x8, x1, #1
 1      2     1.00                        mvn	w9, w2
 1      1     0.50                        lsr	x10, x0, x2
 1      1     0.50                        lsr	x11, x1, x2
 1      1     0.25                        and	x12, x2, #0x7f
 1      1     0.50                        lsl	x8, x8, x9
 1      1     0.25                        tst	x12, #0x40
 1      2     1.00                        orr	x8, x8, x10
 1      1     0.25                        csel	x8, x11, x8, ne
 1      1     0.25                        csel	x11, xzr, x11, ne
 1      1     0.50                        lsr	x10, x8, #1
 1      1     0.50                        lsl	x12, x11, x2
 1      1     0.50                        lsr	x9, x10, x9
 1      1     0.50                        lsl	x10, x8, x2
 1      2     1.00                        orr	x9, x12, x9
 1      1     0.25                        csel	x12, xzr, x10, ne
 1      1     0.25                        csel	x9, x10, x9, ne
 1      2     1.00                        cmp	x0, x12
 1      1     0.25                        ccmp	x1, x9, #0, eq
 1      1     0.25                        cset	w9, ne
 1      2     1.00                        adds	x0, x8, x9
 1      1     0.25                        cinc	x1, x11, hs

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     7.00   5.00   7.00   8.00    -      -     9.00   9.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x1, #1
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	w9, w2
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x10, x0, x2
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	x11, x1, x2
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x12, x2, #0x7f
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x9
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x12, #0x40
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x8, x8, x10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x8, x11, x8, ne
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x11, xzr, x11, ne
 -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	x10, x8, #1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x12, x11, x2
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	x9, x10, x9
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x10, x8, x2
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     orr	x9, x12, x9
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x12, xzr, x10, ne
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x9, x10, x9, ne
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     cmp	x0, x12
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ccmp	x1, x9, #0, eq
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w9, ne
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     adds	x0, x8, x9
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cinc	x1, x11, hs
```
## `div_floor_i8_pow2`
```asm
Iterations:        1
Instructions:      2
Total Cycles:      5
Total uOps:        2

Dispatch Width:    6
uOps Per Cycle:    0.40
IPC:               0.40
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        sxtb	w8, w0
 1      1     0.50                        asr	w0, w8, w1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -      -     1.00   1.00    -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     sxtb	w8, w0
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	w0, w8, w1
```
## `div_floor_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      5
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.60
IPC:               0.60
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        sxtb	w8, w0
 1      1     0.25                        and	x9, x1, #0x7
 1      1     0.50                        asr	w0, w8, w9

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -     1.00   1.00   1.00    -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     sxtb	w8, w0
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x9, x1, #0x7
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	w0, w8, w9
```
## `div_floor_i16_pow2`
```asm
Iterations:        1
Instructions:      2
Total Cycles:      5
Total uOps:        2

Dispatch Width:    6
uOps Per Cycle:    0.40
IPC:               0.40
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        sxth	w8, w0
 1      1     0.50                        asr	w0, w8, w1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -      -     1.00   1.00    -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     sxth	w8, w0
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	w0, w8, w1
```
## `div_floor_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      5
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.60
IPC:               0.60
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        sxth	w8, w0
 1      1     0.25                        and	x9, x1, #0xf
 1      1     0.50                        asr	w0, w8, w9

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -     1.00   1.00   1.00    -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     sxth	w8, w0
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x9, x1, #0xf
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	w0, w8, w9
```
## `div_floor_i32_pow2`
```asm
Iterations:        1
Instructions:      1
Total Cycles:      4
Total uOps:        1

Dispatch Width:    6
uOps Per Cycle:    0.25
IPC:               0.25
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        asr	w0, w0, w1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	w0, w0, w1
```
## `div_floor_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      1
Total Cycles:      4
Total uOps:        1

Dispatch Width:    6
uOps Per Cycle:    0.25
IPC:               0.25
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        asr	w0, w0, w1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	w0, w0, w1
```
## `div_floor_i64_pow2`
```asm
Iterations:        1
Instructions:      1
Total Cycles:      4
Total uOps:        1

Dispatch Width:    6
uOps Per Cycle:    0.25
IPC:               0.25
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        asr	x0, x0, x1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	x0, x0, x1
```
## `div_floor_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      1
Total Cycles:      4
Total uOps:        1

Dispatch Width:    6
uOps Per Cycle:    0.25
IPC:               0.25
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        asr	x0, x0, x1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	x0, x0, x1
```
## `div_floor_i128_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      9
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    1.22
IPC:               1.22
Block RThroughput: 4.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsl	x8, x1, #1
 1      2     1.00                        mvn	w9, w2
 1      1     0.50                        lsr	x10, x0, x2
 1      1     0.25                        and	x11, x2, #0xff
 1      1     0.50                        asr	x12, x1, #63
 1      1     0.50                        lsl	x8, x8, x9
 1      1     0.50                        asr	x9, x1, x2
 1      1     0.25                        tst	x11, #0x40
 1      2     1.00                        orr	x8, x8, x10
 1      1     0.25                        csel	x1, x12, x9, ne
 1      1     0.25                        csel	x0, x9, x8, ne

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   3.00   4.00   4.00    -      -     4.00   5.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x1, #1
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	w9, w2
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x10, x0, x2
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x11, x2, #0xff
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	x12, x1, #63
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x9
 -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	x9, x1, x2
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x11, #0x40
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     orr	x8, x8, x10
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x1, x12, x9, ne
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x0, x9, x8, ne
```
## `div_floor_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      9
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    1.22
IPC:               1.22
Block RThroughput: 4.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsl	x8, x1, #1
 1      2     1.00                        mvn	w9, w2
 1      1     0.50                        lsr	x10, x0, x2
 1      1     0.25                        and	x11, x2, #0x7f
 1      1     0.50                        asr	x12, x1, #63
 1      1     0.50                        lsl	x8, x8, x9
 1      1     0.50                        asr	x9, x1, x2
 1      1     0.25                        tst	x11, #0x40
 1      2     1.00                        orr	x8, x8, x10
 1      1     0.25                        csel	x1, x12, x9, ne
 1      1     0.25                        csel	x0, x9, x8, ne

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   3.00   4.00   4.00    -      -     4.00   5.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x1, #1
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	w9, w2
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x10, x0, x2
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x11, x2, #0x7f
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	x12, x1, #63
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x9
 -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	x9, x1, x2
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x11, #0x40
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     orr	x8, x8, x10
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x1, x12, x9, ne
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x0, x9, x8, ne
```
## `div_floor_u8_pow2`
```asm
Iterations:        1
Instructions:      2
Total Cycles:      5
Total uOps:        2

Dispatch Width:    6
uOps Per Cycle:    0.40
IPC:               0.40
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w0, #0xff
 1      1     0.50                        lsr	w0, w8, w1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -      -     1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w0, #0xff
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	w0, w8, w1
```
## `div_floor_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      5
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.60
IPC:               0.60
Block RThroughput: 0.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w0, #0xff
 1      1     0.25                        and	x9, x1, #0x7
 1      1     0.50                        lsr	w0, w8, w9

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -     1.00   1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w0, #0xff
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x9, x1, #0x7
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	w0, w8, w9
```
## `div_floor_u16_pow2`
```asm
Iterations:        1
Instructions:      2
Total Cycles:      5
Total uOps:        2

Dispatch Width:    6
uOps Per Cycle:    0.40
IPC:               0.40
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w0, #0xffff
 1      1     0.50                        lsr	w0, w8, w1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -      -     1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w0, #0xffff
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	w0, w8, w1
```
## `div_floor_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      5
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.60
IPC:               0.60
Block RThroughput: 0.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w0, #0xffff
 1      1     0.25                        and	x9, x1, #0xf
 1      1     0.50                        lsr	w0, w8, w9

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -     1.00   1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w0, #0xffff
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x9, x1, #0xf
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	w0, w8, w9
```
## `div_floor_u32_pow2`
```asm
Iterations:        1
Instructions:      1
Total Cycles:      4
Total uOps:        1

Dispatch Width:    6
uOps Per Cycle:    0.25
IPC:               0.25
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsr	w0, w0, w1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	w0, w0, w1
```
## `div_floor_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      1
Total Cycles:      4
Total uOps:        1

Dispatch Width:    6
uOps Per Cycle:    0.25
IPC:               0.25
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsr	w0, w0, w1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	w0, w0, w1
```
## `div_floor_u64_pow2`
```asm
Iterations:        1
Instructions:      1
Total Cycles:      4
Total uOps:        1

Dispatch Width:    6
uOps Per Cycle:    0.25
IPC:               0.25
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsr	x0, x0, x1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x0, x0, x1
```
## `div_floor_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      1
Total Cycles:      4
Total uOps:        1

Dispatch Width:    6
uOps Per Cycle:    0.25
IPC:               0.25
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsr	x0, x0, x1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x0, x0, x1
```
## `div_floor_u128_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      9
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.11
IPC:               1.11
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsl	x8, x1, #1
 1      2     1.00                        mvn	w9, w2
 1      1     0.50                        lsr	x10, x0, x2
 1      1     0.25                        and	x11, x2, #0xff
 1      1     0.50                        lsl	x8, x8, x9
 1      1     0.50                        lsr	x9, x1, x2
 1      1     0.25                        tst	x11, #0x40
 1      2     1.00                        orr	x8, x8, x10
 1      1     0.25                        csel	x1, xzr, x9, ne
 1      1     0.25                        csel	x0, x9, x8, ne

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     3.00   2.00   4.00   3.00    -      -     5.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x1, #1
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	w9, w2
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x10, x0, x2
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x11, x2, #0xff
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x9
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x9, x1, x2
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x11, #0x40
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x8, x8, x10
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x1, xzr, x9, ne
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x0, x9, x8, ne
```
## `div_floor_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      9
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.11
IPC:               1.11
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsl	x8, x1, #1
 1      2     1.00                        mvn	w9, w2
 1      1     0.50                        lsr	x10, x0, x2
 1      1     0.25                        and	x11, x2, #0x7f
 1      1     0.50                        lsl	x8, x8, x9
 1      1     0.50                        lsr	x9, x1, x2
 1      1     0.25                        tst	x11, #0x40
 1      2     1.00                        orr	x8, x8, x10
 1      1     0.25                        csel	x1, xzr, x9, ne
 1      1     0.25                        csel	x0, x9, x8, ne

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     3.00   2.00   4.00   3.00    -      -     5.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x1, #1
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	w9, w2
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x10, x0, x2
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x11, x2, #0x7f
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x9
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x9, x1, x2
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x11, #0x40
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x8, x8, x10
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x1, xzr, x9, ne
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x0, x9, x8, ne
```
## `div_i8_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      12
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    0.83
IPC:               0.83
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	w8, #-1
 1      1     0.25                        and	w9, w0, #0xff
 1      1     0.25                        tst	w0, #0x80
 1      1     0.50                        lsl	w8, w8, w1
 1      1     0.50                        lsr	w9, w9, w1
 1      2     1.00                        mvn	w8, w8
 1      2     1.00                        add	w8, w0, w8
 1      1     0.50                        sxtb	w8, w8
 1      1     0.50                        asr	w8, w8, w1
 1      1     0.25                        csel	w0, w9, w8, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   3.00   4.00   3.00    -      -     4.00   4.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w9, w0, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w0, #0x80
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w1
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	w9, w9, w1
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	w8, w8
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     add	w8, w0, w8
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     sxtb	w8, w8
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	w8, w8, w1
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	w0, w9, w8, eq
```
## `div_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      12
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    0.92
IPC:               0.92
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	w8, #-1
 1      1     0.25                        and	w9, w1, #0x7
 1      1     0.25                        and	w10, w0, #0xff
 1      1     0.50                        lsl	w8, w8, w9
 1      1     0.50                        lsr	w10, w10, w9
 1      1     0.25                        tst	w0, #0x80
 1      2     1.00                        mvn	w8, w8
 1      2     1.00                        add	w8, w0, w8
 1      1     0.50                        sxtb	w8, w8
 1      1     0.50                        asr	w8, w8, w9
 1      1     0.25                        csel	w0, w10, w8, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     3.00   4.00   3.00   3.00    -      -     4.00   4.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w9, w1, #0x7
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w10, w0, #0xff
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w9
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	w10, w10, w9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w0, #0x80
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	w8, w8
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     add	w8, w0, w8
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     sxtb	w8, w8
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	w8, w8, w9
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	w0, w10, w8, eq
```
## `div_i16_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      12
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    0.92
IPC:               0.92
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	w8, #-1
 1      1     0.25                        and	w9, w1, #0xff
 1      1     0.25                        and	w10, w0, #0xffff
 1      1     0.50                        lsl	w8, w8, w9
 1      1     0.50                        lsr	w10, w10, w9
 1      1     0.25                        tst	w0, #0x8000
 1      2     1.00                        mvn	w8, w8
 1      2     1.00                        add	w8, w0, w8
 1      1     0.50                        sxth	w8, w8
 1      1     0.50                        asr	w8, w8, w9
 1      1     0.25                        csel	w0, w10, w8, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     3.00   4.00   3.00   3.00    -      -     4.00   4.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w9, w1, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w10, w0, #0xffff
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w9
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	w10, w10, w9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w0, #0x8000
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	w8, w8
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     add	w8, w0, w8
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     sxth	w8, w8
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	w8, w8, w9
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	w0, w10, w8, eq
```
## `div_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      12
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    0.92
IPC:               0.92
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	w8, #-1
 1      1     0.25                        and	w9, w1, #0xf
 1      1     0.25                        and	w10, w0, #0xffff
 1      1     0.50                        lsl	w8, w8, w9
 1      1     0.50                        lsr	w10, w10, w9
 1      1     0.25                        tst	w0, #0x8000
 1      2     1.00                        mvn	w8, w8
 1      2     1.00                        add	w8, w0, w8
 1      1     0.50                        sxth	w8, w8
 1      1     0.50                        asr	w8, w8, w9
 1      1     0.25                        csel	w0, w10, w8, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     3.00   4.00   3.00   3.00    -      -     4.00   4.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w9, w1, #0xf
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w10, w0, #0xffff
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w9
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	w10, w10, w9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w0, #0x8000
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	w8, w8
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     add	w8, w0, w8
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     sxth	w8, w8
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	w8, w8, w9
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	w0, w10, w8, eq
```
## `div_i32_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      11
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    0.82
IPC:               0.82
Block RThroughput: 3.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	w8, #-1
 1      1     0.25                        and	w9, w1, #0xff
 1      1     0.25                        tst	w0, #0x80000000
 1      1     0.50                        lsl	w8, w8, w9
 1      1     0.50                        lsr	w10, w0, w9
 1      2     1.00                        mvn	w8, w8
 1      2     1.00                        add	w8, w0, w8
 1      1     0.50                        asr	w8, w8, w9
 1      1     0.25                        csel	w0, w10, w8, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   3.00   3.00   3.00    -      -     3.00   4.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w9, w1, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w0, #0x80000000
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w9
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	w10, w0, w9
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	w8, w8
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     add	w8, w0, w8
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	w8, w8, w9
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	w0, w10, w8, eq
```
## `div_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      11
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    0.82
IPC:               0.82
Block RThroughput: 3.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	w8, #-1
 1      1     0.25                        and	w9, w1, #0x1f
 1      1     0.25                        tst	w0, #0x80000000
 1      1     0.50                        lsl	w8, w8, w9
 1      1     0.50                        lsr	w10, w0, w9
 1      2     1.00                        mvn	w8, w8
 1      2     1.00                        add	w8, w0, w8
 1      1     0.50                        asr	w8, w8, w9
 1      1     0.25                        csel	w0, w10, w8, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   3.00   3.00   3.00    -      -     3.00   4.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w9, w1, #0x1f
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w0, #0x80000000
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w9
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	w10, w0, w9
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	w8, w8
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     add	w8, w0, w8
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	w8, w8, w9
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	w0, w10, w8, eq
```
## `div_i64_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      11
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    0.82
IPC:               0.82
Block RThroughput: 3.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	x8, #-1
 1      1     0.25                        and	x9, x1, #0xff
 1      1     0.25                        tst	x0, #0x8000000000000000
 1      1     0.50                        lsl	x8, x8, x9
 1      1     0.50                        lsr	x10, x0, x9
 1      2     1.00                        mvn	x8, x8
 1      2     1.00                        add	x8, x0, x8
 1      1     0.50                        asr	x8, x8, x9
 1      1     0.25                        csel	x0, x10, x8, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   3.00   3.00   3.00    -      -     3.00   4.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x9, x1, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x0, #0x8000000000000000
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x9
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	x10, x0, x9
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	x8, x8
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     add	x8, x0, x8
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	x8, x8, x9
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x0, x10, x8, eq
```
## `div_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      11
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    0.82
IPC:               0.82
Block RThroughput: 3.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	x8, #-1
 1      1     0.25                        and	x9, x1, #0x3f
 1      1     0.25                        tst	x0, #0x8000000000000000
 1      1     0.50                        lsl	x8, x8, x9
 1      1     0.50                        lsr	x10, x0, x9
 1      2     1.00                        mvn	x8, x8
 1      2     1.00                        add	x8, x0, x8
 1      1     0.50                        asr	x8, x8, x9
 1      1     0.25                        csel	x0, x10, x8, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   3.00   3.00   3.00    -      -     3.00   4.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x9, x1, #0x3f
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x0, #0x8000000000000000
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x9
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	x10, x0, x9
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	x8, x8
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     add	x8, x0, x8
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	x8, x8, x9
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x0, x10, x8, eq
```
## `div_i128_pow2`
```asm
Iterations:        1
Instructions:      36
Total Cycles:      21
Total uOps:        36

Dispatch Width:    6
uOps Per Cycle:    1.71
IPC:               1.71
Block RThroughput: 14.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.50                        tbnz	x1, #63, .LBB130_2
 1      1     0.50                        lsl	x8, x1, #1
 1      2     1.00                        mov	w9, w2
 1      2     1.00                        mvn	w10, w2
 1      1     0.50                        lsr	x11, x0, x9
 1      1     0.50                        lsl	x8, x8, x10
 1      1     0.50                        lsr	x10, x1, x9
 1      1     0.25                        and	x9, x9, #0xff
 1      1     0.25                        tst	x9, #0x40
 1      2     1.00                        orr	x8, x8, x11
 1      1     0.25                        csel	x1, xzr, x10, ne
 1      1     0.25                        csel	x0, x10, x8, ne
 1      0     1.00                  U     ret
 1      1     0.25                        mov	x8, #-1
 1      2     1.00                        mvn	w9, w2
 1      1     0.25                        mov	x10, #9223372036854775807
 1      1     0.50                        lsl	x8, x8, x2
 1      1     0.25                        and	x11, x2, #0xff
 1      1     0.50                        lsr	x10, x10, x9
 1      1     0.25                        tst	x11, #0x40
 1      1     0.25                        csel	x12, xzr, x8, ne
 1      2     1.00                        orr	x10, x8, x10
 1      1     0.25                        csel	x8, x8, x10, ne
 1      2     1.00                        mvn	x10, x12
 1      2     1.00                        mvn	x8, x8
 1      2     1.00                        adds	x10, x0, x10
 1      1     0.25                        adc	x8, x1, x8
 1      1     0.50                        lsr	x10, x10, x2
 1      1     0.25                        tst	x11, #0x40
 1      1     0.50                        lsl	x12, x8, #1
 1      1     0.50                        lsl	x9, x12, x9
 1      1     0.50                        asr	x12, x8, x2
 1      1     0.50                        asr	x8, x8, #63
 1      2     1.00                        orr	x9, x9, x10
 1      1     0.25                        csel	x1, x8, x12, ne
 1      1     0.25                        csel	x0, x12, x9, ne

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   1.00   1.00    -      -     11.00  13.00  10.00  11.00   -      -     15.00  14.00   -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tbnz	x1, #63, .LBB130_2
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x1, #1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mov	w9, w2
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	w10, w2
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	x11, x0, x9
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x10
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	x10, x1, x9
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x9, x9, #0xff
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x9, #0x40
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     orr	x8, x8, x11
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x1, xzr, x10, ne
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x0, x10, x8, ne
1.00    -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	w9, w2
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x10, #9223372036854775807
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x2
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x11, x2, #0xff
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	x10, x10, x9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x11, #0x40
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x12, xzr, x8, ne
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     orr	x10, x8, x10
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x8, x8, x10, ne
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	x10, x12
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	x8, x8
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     adds	x10, x0, x10
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     adc	x8, x1, x8
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x10, x10, x2
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x11, #0x40
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x12, x8, #1
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x9, x12, x9
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	x12, x8, x2
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	x8, x8, #63
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x9, x9, x10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x1, x8, x12, ne
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x0, x12, x9, ne
```
## `div_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      34
Total Cycles:      21
Total uOps:        34

Dispatch Width:    6
uOps Per Cycle:    1.62
IPC:               1.62
Block RThroughput: 13.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w2, #0x7f
 1      0     0.50                        tbnz	x1, #63, .LBB131_2
 1      1     0.50                        lsl	x9, x1, #1
 1      2     1.00                        mvn	w10, w8
 1      1     0.50                        lsr	x11, x0, x8
 1      1     0.25                        tst	x8, #0x40
 1      1     0.50                        lsl	x9, x9, x10
 1      1     0.50                        lsr	x10, x1, x8
 1      2     1.00                        orr	x9, x9, x11
 1      1     0.25                        csel	x1, xzr, x10, ne
 1      1     0.25                        csel	x0, x10, x9, ne
 1      0     1.00                  U     ret
 1      2     1.00                        mvn	w9, w8
 1      1     0.25                        mov	x10, #9223372036854775807
 1      1     0.25                        mov	x11, #-1
 1      1     0.50                        lsr	x10, x10, x9
 1      1     0.50                        lsl	x11, x11, x8
 1      1     0.25                        tst	x8, #0x40
 1      2     1.00                        orr	x10, x11, x10
 1      1     0.25                        csel	x12, xzr, x11, ne
 1      1     0.25                        csel	x10, x11, x10, ne
 1      2     1.00                        mvn	x11, x12
 1      2     1.00                        mvn	x10, x10
 1      2     1.00                        adds	x11, x0, x11
 1      1     0.25                        adc	x10, x1, x10
 1      1     0.50                        lsr	x11, x11, x8
 1      1     0.25                        tst	x8, #0x40
 1      1     0.50                        lsl	x12, x10, #1
 1      1     0.50                        lsl	x9, x12, x9
 1      1     0.50                        asr	x12, x10, x8
 1      1     0.50                        asr	x10, x10, #63
 1      2     1.00                        orr	x8, x9, x11
 1      1     0.25                        csel	x1, x10, x12, ne
 1      1     0.25                        csel	x0, x12, x8, ne

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   1.00   1.00    -      -     9.00   10.00  12.00  11.00   -      -     14.00  13.00   -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w2, #0x7f
 -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tbnz	x1, #63, .LBB131_2
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x9, x1, #1
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	w10, w8
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x11, x0, x8
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x8, #0x40
 -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x9, x9, x10
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x10, x1, x8
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x9, x9, x11
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x1, xzr, x10, ne
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x0, x10, x9, ne
1.00    -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	w9, w8
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x10, #9223372036854775807
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x11, #-1
 -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	x10, x10, x9
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x11, x11, x8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x8, #0x40
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     orr	x10, x11, x10
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x12, xzr, x11, ne
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x10, x11, x10, ne
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	x11, x12
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	x10, x10
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     adds	x11, x0, x11
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     adc	x10, x1, x10
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x11, x11, x8
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x8, #0x40
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x12, x10, #1
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x9, x12, x9
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	x12, x10, x8
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	x10, x10, #63
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x8, x9, x11
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x1, x10, x12, ne
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x0, x12, x8, ne
```
## `floor_to_multiple_i8_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      6
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.50
IPC:               0.50
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w0, #0xff
 1      1     0.50                        lsr	w8, w8, w1
 1      1     0.50                        lsl	w0, w8, w1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -     1.00   1.00   1.00    -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w0, #0xff
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	w8, w8, w1
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	w0, w8, w1
```
## `floor_to_multiple_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      6
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.67
IPC:               0.67
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w0, #0xff
 1      1     0.25                        and	x9, x1, #0x7
 1      1     0.50                        lsr	w8, w8, w9
 1      1     0.50                        lsl	w0, w8, w9

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   1.00   1.00   1.00    -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w0, #0xff
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x9, x1, #0x7
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	w8, w8, w9
 -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	w0, w8, w9
```
## `floor_to_multiple_i16_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      6
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.50
IPC:               0.50
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w0, #0xffff
 1      1     0.50                        lsr	w8, w8, w1
 1      1     0.50                        lsl	w0, w8, w1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -     1.00   1.00   1.00    -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w0, #0xffff
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	w8, w8, w1
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	w0, w8, w1
```
## `floor_to_multiple_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      6
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.67
IPC:               0.67
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w0, #0xffff
 1      1     0.25                        and	x9, x1, #0xf
 1      1     0.50                        lsr	w8, w8, w9
 1      1     0.50                        lsl	w0, w8, w9

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   1.00   1.00   1.00    -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w0, #0xffff
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x9, x1, #0xf
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	w8, w8, w9
 -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	w0, w8, w9
```
## `floor_to_multiple_i32_pow2`
```asm
Iterations:        1
Instructions:      2
Total Cycles:      5
Total uOps:        2

Dispatch Width:    6
uOps Per Cycle:    0.40
IPC:               0.40
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsr	w8, w0, w1
 1      1     0.50                        lsl	w0, w8, w1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -      -     1.00   1.00    -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	w8, w0, w1
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	w0, w8, w1
```
## `floor_to_multiple_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      2
Total Cycles:      5
Total uOps:        2

Dispatch Width:    6
uOps Per Cycle:    0.40
IPC:               0.40
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsr	w8, w0, w1
 1      1     0.50                        lsl	w0, w8, w1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -      -     1.00   1.00    -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	w8, w0, w1
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	w0, w8, w1
```
## `floor_to_multiple_i64_pow2`
```asm
Iterations:        1
Instructions:      2
Total Cycles:      5
Total uOps:        2

Dispatch Width:    6
uOps Per Cycle:    0.40
IPC:               0.40
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsr	x8, x0, x1
 1      1     0.50                        lsl	x0, x8, x1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -      -     1.00   1.00    -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x8, x0, x1
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x0, x8, x1
```
## `floor_to_multiple_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      2
Total Cycles:      5
Total uOps:        2

Dispatch Width:    6
uOps Per Cycle:    0.40
IPC:               0.40
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsr	x8, x0, x1
 1      1     0.50                        lsl	x0, x8, x1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -      -     1.00   1.00    -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x8, x0, x1
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x0, x8, x1
```
## `floor_to_multiple_i128_pow2`
```asm
Iterations:        1
Instructions:      12
Total Cycles:      11
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.09
IPC:               1.09
Block RThroughput: 5.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	x8, #-1
 1      2     1.00                        mvn	w9, w2
 1      1     0.25                        mov	x10, #9223372036854775807
 1      1     0.50                        lsl	x8, x8, x2
 1      1     0.50                        lsr	x9, x10, x9
 1      1     0.25                        and	x10, x2, #0xff
 1      1     0.25                        tst	x10, #0x40
 1      2     1.00                        orr	x9, x8, x9
 1      1     0.25                        csel	x9, x8, x9, ne
 1      1     0.25                        csel	x8, xzr, x8, ne
 1      2     1.00                        and	x0, x8, x0
 1      2     1.00                        and	x1, x9, x1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     3.00   4.00   4.00   5.00    -      -     5.00   5.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	w9, w2
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x10, #9223372036854775807
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x2
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x9, x10, x9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x10, x2, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x10, #0x40
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x9, x8, x9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x9, x8, x9, ne
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x8, xzr, x8, ne
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     and	x0, x8, x0
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     and	x1, x9, x1
```
## `floor_to_multiple_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      12
Total Cycles:      11
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.09
IPC:               1.09
Block RThroughput: 5.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	x8, #-1
 1      2     1.00                        mvn	w9, w2
 1      1     0.25                        mov	x10, #9223372036854775807
 1      1     0.50                        lsl	x8, x8, x2
 1      1     0.50                        lsr	x9, x10, x9
 1      1     0.25                        and	x10, x2, #0x7f
 1      1     0.25                        tst	x10, #0x40
 1      2     1.00                        orr	x9, x8, x9
 1      1     0.25                        csel	x9, x8, x9, ne
 1      1     0.25                        csel	x8, xzr, x8, ne
 1      2     1.00                        and	x0, x8, x0
 1      2     1.00                        and	x1, x9, x1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     3.00   4.00   4.00   5.00    -      -     5.00   5.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	w9, w2
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x10, #9223372036854775807
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x2
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x9, x10, x9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x10, x2, #0x7f
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x10, #0x40
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x9, x8, x9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x9, x8, x9, ne
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x8, xzr, x8, ne
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     and	x0, x8, x0
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     and	x1, x9, x1
```
## `is_multiple_of_i8_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      9
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.56
IPC:               0.56
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        orr	w8, w0, #0x100
 1      1     0.25                        rbit	w8, w8
 1      1     0.25                        clz	w8, w8
 1      2     1.00                        cmp	w8, w1, uxtb
 1      1     0.25                        cset	w0, hs

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   1.00   1.00   2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     orr	w8, w0, #0x100
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     rbit	w8, w8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     clz	w8, w8
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     cmp	w8, w1, uxtb
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, hs
```
## `is_multiple_of_i16_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      9
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.56
IPC:               0.56
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        orr	w8, w0, #0x10000
 1      1     0.25                        rbit	w8, w8
 1      1     0.25                        clz	w8, w8
 1      2     1.00                        cmp	w8, w1, uxtb
 1      1     0.25                        cset	w0, hs

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   1.00   1.00   2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     orr	w8, w0, #0x10000
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     rbit	w8, w8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     clz	w8, w8
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     cmp	w8, w1, uxtb
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, hs
```
## `is_multiple_of_i32_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      8
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.50
IPC:               0.50
Block RThroughput: 1.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        rbit	w8, w0
 1      1     0.25                        clz	w8, w8
 1      2     1.00                        cmp	w8, w1, uxtb
 1      1     0.25                        cset	w0, hs

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   2.00   1.00   1.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     rbit	w8, w0
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     clz	w8, w8
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     cmp	w8, w1, uxtb
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, hs
```
## `is_multiple_of_i64_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      8
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.50
IPC:               0.50
Block RThroughput: 1.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        rbit	x8, x0
 1      1     0.25                        clz	x8, x8
 1      2     1.00                        cmp	x8, w1, uxtb
 1      1     0.25                        cset	w0, hs

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   2.00   1.00   1.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     rbit	x8, x0
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     clz	x8, x8
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     cmp	x8, w1, uxtb
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, hs
```
## `is_multiple_of_i128_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      10
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    0.90
IPC:               0.90
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        rbit	x8, x1
 1      1     0.25                        rbit	x9, x0
 1      1     0.25                        cmp	x0, #0
 1      1     0.25                        clz	x8, x8
 1      1     0.25                        clz	x9, x9
 1      1     0.25                        add	w8, w8, #64
 1      1     0.25                        csel	w8, w9, w8, ne
 1      2     1.00                        cmp	w8, w2, uxtb
 1      1     0.25                        cset	w0, hs

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     3.00   2.00   2.00   3.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     rbit	x8, x1
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     rbit	x9, x0
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	x0, #0
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     clz	x8, x8
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     clz	x9, x9
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	w8, w8, #64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	w8, w9, w8, ne
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     cmp	w8, w2, uxtb
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, hs
```
## `mod_floor_i8_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      7
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.43
IPC:               0.43
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	w8, #-1
 1      1     0.50                        lsl	w8, w8, w1
 1      2     1.00                        bic	w0, w0, w8

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -     2.00   1.00   1.00    -      -     2.00   1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     bic	w0, w0, w8
```
## `mod_floor_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      7
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.57
IPC:               0.57
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	w8, #-1
 1      1     0.25                        and	x9, x1, #0x7
 1      1     0.50                        lsl	w8, w8, w9
 1      2     1.00                        bic	w0, w0, w8

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   1.00   1.00   1.00    -      -     2.00   1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x9, x1, #0x7
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w9
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     bic	w0, w0, w8
```
## `mod_floor_i16_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      7
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.43
IPC:               0.43
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	w8, #-1
 1      1     0.50                        lsl	w8, w8, w1
 1      2     1.00                        bic	w0, w0, w8

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -     2.00   1.00   1.00    -      -     2.00   1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     bic	w0, w0, w8
```
## `mod_floor_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      7
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.57
IPC:               0.57
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	w8, #-1
 1      1     0.25                        and	x9, x1, #0xf
 1      1     0.50                        lsl	w8, w8, w9
 1      2     1.00                        bic	w0, w0, w8

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   1.00   1.00   1.00    -      -     2.00   1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x9, x1, #0xf
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w9
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     bic	w0, w0, w8
```
## `mod_floor_i32_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      7
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.43
IPC:               0.43
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	w8, #-1
 1      1     0.50                        lsl	w8, w8, w1
 1      2     1.00                        bic	w0, w0, w8

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -     2.00   1.00   1.00    -      -     2.00   1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     bic	w0, w0, w8
```
## `mod_floor_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      7
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.43
IPC:               0.43
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	w8, #-1
 1      1     0.50                        lsl	w8, w8, w1
 1      2     1.00                        bic	w0, w0, w8

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -     2.00   1.00   1.00    -      -     2.00   1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     bic	w0, w0, w8
```
## `mod_floor_i64_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      7
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.43
IPC:               0.43
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	x8, #-1
 1      1     0.50                        lsl	x8, x8, x1
 1      2     1.00                        bic	x0, x0, x8

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -     2.00   1.00   1.00    -      -     2.00   1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     bic	x0, x0, x8
```
## `mod_floor_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      7
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.43
IPC:               0.43
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	x8, #-1
 1      1     0.50                        lsl	x8, x8, x1
 1      2     1.00                        bic	x0, x0, x8

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -     2.00   1.00   1.00    -      -     2.00   1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     bic	x0, x0, x8
```
## `mod_floor_i128_pow2`
```asm
Iterations:        1
Instructions:      12
Total Cycles:      11
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.09
IPC:               1.09
Block RThroughput: 5.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	x8, #-1
 1      2     1.00                        mvn	w9, w2
 1      1     0.25                        mov	x10, #9223372036854775807
 1      1     0.50                        lsl	x8, x8, x2
 1      1     0.50                        lsr	x9, x10, x9
 1      1     0.25                        and	x10, x2, #0xff
 1      1     0.25                        tst	x10, #0x40
 1      2     1.00                        orr	x9, x8, x9
 1      1     0.25                        csel	x9, x8, x9, ne
 1      1     0.25                        csel	x8, xzr, x8, ne
 1      2     1.00                        bic	x0, x0, x8
 1      2     1.00                        bic	x1, x1, x9

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     3.00   4.00   4.00   5.00    -      -     5.00   5.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	w9, w2
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x10, #9223372036854775807
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x2
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x9, x10, x9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x10, x2, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x10, #0x40
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x9, x8, x9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x9, x8, x9, ne
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x8, xzr, x8, ne
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     bic	x0, x0, x8
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     bic	x1, x1, x9
```
## `mod_floor_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      12
Total Cycles:      11
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.09
IPC:               1.09
Block RThroughput: 5.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	x8, #-1
 1      2     1.00                        mvn	w9, w2
 1      1     0.25                        mov	x10, #9223372036854775807
 1      1     0.50                        lsl	x8, x8, x2
 1      1     0.50                        lsr	x9, x10, x9
 1      1     0.25                        and	x10, x2, #0x7f
 1      1     0.25                        tst	x10, #0x40
 1      2     1.00                        orr	x9, x8, x9
 1      1     0.25                        csel	x9, x8, x9, ne
 1      1     0.25                        csel	x8, xzr, x8, ne
 1      2     1.00                        bic	x0, x0, x8
 1      2     1.00                        bic	x1, x1, x9

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     3.00   4.00   4.00   5.00    -      -     5.00   5.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	w9, w2
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x10, #9223372036854775807
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x2
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x9, x10, x9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x10, x2, #0x7f
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x10, #0x40
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x9, x8, x9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x9, x8, x9, ne
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x8, xzr, x8, ne
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     bic	x0, x0, x8
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     bic	x1, x1, x9
```
## `mul_i8_pow2`
```asm
Iterations:        1
Instructions:      1
Total Cycles:      4
Total uOps:        1

Dispatch Width:    6
uOps Per Cycle:    0.25
IPC:               0.25
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsl	w0, w0, w1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w0, w0, w1
```
## `mul_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      2
Total Cycles:      5
Total uOps:        2

Dispatch Width:    6
uOps Per Cycle:    0.40
IPC:               0.40
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	x8, x1, #0x7
 1      1     0.50                        lsl	w0, w0, w8

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -      -     1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x8, x1, #0x7
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w0, w0, w8
```
## `mul_i16_pow2`
```asm
Iterations:        1
Instructions:      1
Total Cycles:      4
Total uOps:        1

Dispatch Width:    6
uOps Per Cycle:    0.25
IPC:               0.25
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsl	w0, w0, w1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w0, w0, w1
```
## `mul_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      2
Total Cycles:      5
Total uOps:        2

Dispatch Width:    6
uOps Per Cycle:    0.40
IPC:               0.40
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	x8, x1, #0xf
 1      1     0.50                        lsl	w0, w0, w8

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -      -     1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x8, x1, #0xf
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w0, w0, w8
```
## `mul_i32_pow2`
```asm
Iterations:        1
Instructions:      1
Total Cycles:      4
Total uOps:        1

Dispatch Width:    6
uOps Per Cycle:    0.25
IPC:               0.25
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsl	w0, w0, w1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w0, w0, w1
```
## `mul_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      1
Total Cycles:      4
Total uOps:        1

Dispatch Width:    6
uOps Per Cycle:    0.25
IPC:               0.25
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsl	w0, w0, w1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w0, w0, w1
```
## `mul_i64_pow2`
```asm
Iterations:        1
Instructions:      1
Total Cycles:      4
Total uOps:        1

Dispatch Width:    6
uOps Per Cycle:    0.25
IPC:               0.25
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsl	x0, x0, x1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x0, x0, x1
```
## `mul_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      1
Total Cycles:      4
Total uOps:        1

Dispatch Width:    6
uOps Per Cycle:    0.25
IPC:               0.25
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsl	x0, x0, x1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x0, x0, x1
```
## `mul_i128_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      9
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.11
IPC:               1.11
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsr	x8, x0, #1
 1      2     1.00                        mvn	w9, w2
 1      1     0.50                        lsl	x10, x1, x2
 1      1     0.25                        and	x11, x2, #0xff
 1      1     0.50                        lsr	x8, x8, x9
 1      1     0.50                        lsl	x9, x0, x2
 1      1     0.25                        tst	x11, #0x40
 1      2     1.00                        orr	x8, x10, x8
 1      1     0.25                        csel	x0, xzr, x9, ne
 1      1     0.25                        csel	x1, x9, x8, ne

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     3.00   2.00   4.00   3.00    -      -     5.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x8, x0, #1
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	w9, w2
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x10, x1, x2
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x11, x2, #0xff
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	x8, x8, x9
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x9, x0, x2
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x11, #0x40
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x8, x10, x8
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x0, xzr, x9, ne
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x1, x9, x8, ne
```
## `mul_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      9
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.11
IPC:               1.11
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsr	x8, x0, #1
 1      2     1.00                        mvn	w9, w2
 1      1     0.50                        lsl	x10, x1, x2
 1      1     0.25                        and	x11, x2, #0x7f
 1      1     0.50                        lsr	x8, x8, x9
 1      1     0.50                        lsl	x9, x0, x2
 1      1     0.25                        tst	x11, #0x40
 1      2     1.00                        orr	x8, x10, x8
 1      1     0.25                        csel	x0, xzr, x9, ne
 1      1     0.25                        csel	x1, x9, x8, ne

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     3.00   2.00   4.00   3.00    -      -     5.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x8, x0, #1
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	w9, w2
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x10, x1, x2
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x11, x2, #0x7f
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	x8, x8, x9
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x9, x0, x2
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x11, #0x40
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x8, x10, x8
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x0, xzr, x9, ne
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x1, x9, x8, ne
```
## `unbounded_ceil_to_multiple_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      13
Total uOps:        14

Dispatch Width:    6
uOps Per Cycle:    1.08
IPC:               1.08
Block RThroughput: 5.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	w8, #-1
 1      1     0.50                        sxtb	w9, w0
 1      1     0.25                        and	w11, w1, #0xff
 1      1     0.50                        lsl	w8, w8, w1
 1      2     1.00                        mvn	w10, w8
 1      2     1.00                        add	w10, w9, w10, sxtb
 1      2     1.00                        cmp	w10, w10, sxtb
 1      2     1.00                        and	w8, w10, w8
 1      1     0.25                        cset	w12, eq
 1      1     0.25                        cmp	w9, #1
 1      1     0.25                        cset	w9, lt
 1      1     0.25                        cmp	w11, #8
 1      1     0.25                        csel	w1, w8, wzr, lo
 1      1     0.25                        csel	w0, w12, w9, lo

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     3.00   5.00   5.00   5.00    -      -     5.00   5.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     sxtb	w9, w0
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w11, w1, #0xff
 -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w1
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	w10, w8
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     add	w10, w9, w10, sxtb
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     cmp	w10, w10, sxtb
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     and	w8, w10, w8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w12, eq
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w9, #1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w9, lt
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w11, #8
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	w1, w8, wzr, lo
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	w0, w12, w9, lo
```
## `unbounded_ceil_to_multiple_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      13
Total uOps:        14

Dispatch Width:    6
uOps Per Cycle:    1.08
IPC:               1.08
Block RThroughput: 5.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	w8, #-1
 1      1     0.50                        sxth	w9, w0
 1      1     0.25                        and	w11, w1, #0xff
 1      1     0.50                        lsl	w8, w8, w1
 1      2     1.00                        mvn	w10, w8
 1      2     1.00                        add	w10, w9, w10, sxth
 1      2     1.00                        cmp	w10, w10, sxth
 1      2     1.00                        and	w8, w10, w8
 1      1     0.25                        cset	w12, eq
 1      1     0.25                        cmp	w9, #1
 1      1     0.25                        cset	w9, lt
 1      1     0.25                        cmp	w11, #16
 1      1     0.25                        csel	w1, w8, wzr, lo
 1      1     0.25                        csel	w0, w12, w9, lo

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     3.00   5.00   5.00   5.00    -      -     5.00   5.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     sxth	w9, w0
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w11, w1, #0xff
 -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w1
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	w10, w8
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     add	w10, w9, w10, sxth
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     cmp	w10, w10, sxth
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     and	w8, w10, w8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w12, eq
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w9, #1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w9, lt
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w11, #16
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	w1, w8, wzr, lo
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	w0, w12, w9, lo
```
## `unbounded_ceil_to_multiple_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      12
Total Cycles:      12
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 3.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	w8, #-1
 1      1     0.25                        and	w10, w1, #0xff
 1      1     0.50                        lsl	w8, w8, w1
 1      2     1.00                        mvn	w9, w8
 1      2     1.00                        adds	w9, w0, w9
 1      1     0.25                        cset	w11, vc
 1      1     0.25                        cmp	w0, #1
 1      2     1.00                        and	w8, w9, w8
 1      1     0.25                        cset	w9, lt
 1      1     0.25                        cmp	w10, #32
 1      1     0.25                        csel	w1, w8, wzr, lo
 1      1     0.25                        csel	w0, w11, w9, lo

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     4.00   3.00   5.00   3.00    -      -     4.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w10, w1, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w1
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	w9, w8
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     adds	w9, w0, w9
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w11, vc
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w0, #1
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     and	w8, w9, w8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w9, lt
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w10, #32
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	w1, w8, wzr, lo
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	w0, w11, w9, lo
```
## `unbounded_ceil_to_multiple_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      12
Total Cycles:      12
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 3.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	x8, #-1
 1      1     0.25                        and	w10, w1, #0xff
 1      1     0.50                        lsl	x8, x8, x1
 1      2     1.00                        mvn	x9, x8
 1      2     1.00                        adds	x9, x0, x9
 1      1     0.25                        cset	w11, vc
 1      1     0.25                        cmp	x0, #1
 1      2     1.00                        and	x8, x9, x8
 1      1     0.25                        cset	w9, lt
 1      1     0.25                        cmp	w10, #64
 1      1     0.25                        csel	x1, x8, xzr, lo
 1      1     0.25                        csel	w0, w11, w9, lo

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     4.00   3.00   5.00   3.00    -      -     4.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w10, w1, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x1
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	x9, x8
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     adds	x9, x0, x9
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w11, vc
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	x0, #1
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     and	x8, x9, x8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w9, lt
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w10, #64
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x1, x8, xzr, lo
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	w0, w11, w9, lo
```
## `unbounded_ceil_to_multiple_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      31
Total Cycles:      22
Total uOps:        36

Dispatch Width:    6
uOps Per Cycle:    1.64
IPC:               1.41
Block RThroughput: 8.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.50                        tbnz	w2, #7, .LBB175_3
 1      1     0.25                        mov	x9, #-1
 1      2     1.00                        mvn	w10, w2
 1      1     0.25                        mov	x11, #9223372036854775807
 1      1     0.50                        lsl	x12, x9, x2
 1      1     0.25                        and	x9, x2, #0xff
 1      1     0.50                        lsr	x10, x11, x10
 1      1     0.25                        tst	x9, #0x40
 1      1     0.25                        csel	x9, xzr, x12, ne
 1      2     1.00                        orr	x10, x12, x10
 1      1     0.25                        csel	x11, x12, x10, ne
 1      2     1.00                        mvn	x10, x9
 1      2     1.00                        mvn	x12, x11
 1      2     1.00                        adds	x10, x0, x10
 1      1     0.25                        adcs	x12, x1, x12
 1      1     0.25                        cset	w13, vs
 1      0     0.50                        tbnz	w13, #0, .LBB175_4
 1      2     1.00                        and	x10, x10, x9
 1      2     1.00                        and	x9, x12, x11
 2      6     0.50           *            stp	x10, x9, [x8, #16]
 1      1     0.25                        mov	w10, #1
 2      6     0.50           *            stp	x10, xzr, [x8]
 1      0     1.00                  U     ret
 1      1     0.25                        cmp	x0, #1
 1      1     0.25                        sbcs	xzr, x1, xzr
 1      0     0.50                        b.lt	.LBB175_5
 2      6     0.50           *            stp	xzr, xzr, [x8]
 1      0     1.00                  U     ret
 2      6     0.50           *            stp	xzr, xzr, [x8, #16]
 1      1     0.25                        mov	w10, #1
 2      6     0.50           *            stp	x10, xzr, [x8]

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
2.00   3.00   2.00    -      -     8.00   9.00   7.00   9.00    -      -     7.00   9.00    -     2.00   3.00   1.00   2.00   2.00    -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tbnz	w2, #7, .LBB175_3
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x9, #-1
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	w10, w2
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x11, #9223372036854775807
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x12, x9, x2
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x9, x2, #0xff
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x10, x11, x10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x9, #0x40
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x9, xzr, x12, ne
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x10, x12, x10
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x11, x12, x10, ne
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	x10, x9
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	x12, x11
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     adds	x10, x0, x10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     adcs	x12, x1, x12
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w13, vs
 -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tbnz	w13, #0, .LBB175_4
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     and	x10, x10, x9
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     and	x9, x12, x11
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -     stp	x10, x9, [x8, #16]
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w10, #1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     stp	x10, xzr, [x8]
1.00    -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	x0, #1
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sbcs	xzr, x1, xzr
1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.lt	.LBB175_5
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00   1.00    -      -      -      -      -      -     stp	xzr, xzr, [x8]
 -     1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -     stp	xzr, xzr, [x8, #16]
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w10, #1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -     1.00    -      -      -      -      -     stp	x10, xzr, [x8]
```
## `unbounded_ceil_to_multiple_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      15
Total uOps:        15

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 5.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w1, #0xff
 1      1     0.25                        cmp	w8, #8
 1      0     0.50                        b.hs	.LBB184_2
 1      1     0.25                        mov	w8, #-1
 1      1     0.25                        mov	w10, #255
 1      1     0.50                        lsl	w8, w8, w1
 1      2     1.00                        sub	w9, w8, w0
 1      2     1.00                        bic	w10, w10, w9
 1      2     1.00                        bic	w1, w8, w9
 1      2     1.00                        cmp	w10, w0, uxtb
 1      1     0.25                        cset	w0, hs
 1      0     1.00                  U     ret
 1      1     0.25                        tst	w0, #0xff
 1      2     1.00                        mov	w1, wzr
 1      1     0.25                        cset	w0, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   1.00   1.00    -      -     4.00   5.00   6.00   5.00    -      -     6.00   5.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w1, #0xff
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w8, #8
1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.hs	.LBB184_2
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w10, #255
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w1
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     sub	w9, w8, w0
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     bic	w10, w10, w9
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     bic	w1, w8, w9
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     cmp	w10, w0, uxtb
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, hs
 -     1.00   1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w0, #0xff
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mov	w1, wzr
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, eq
```
## `unbounded_ceil_to_multiple_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      15
Total uOps:        15

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 5.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w1, #0xff
 1      1     0.25                        cmp	w8, #16
 1      0     0.50                        b.hs	.LBB181_2
 1      1     0.25                        mov	w8, #-1
 1      1     0.25                        mov	w10, #65535
 1      1     0.50                        lsl	w8, w8, w1
 1      2     1.00                        sub	w9, w8, w0
 1      2     1.00                        bic	w10, w10, w9
 1      2     1.00                        bic	w1, w8, w9
 1      2     1.00                        cmp	w10, w0, uxth
 1      1     0.25                        cset	w0, hs
 1      0     1.00                  U     ret
 1      1     0.25                        tst	w0, #0xffff
 1      2     1.00                        mov	w1, wzr
 1      1     0.25                        cset	w0, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   1.00   1.00    -      -     4.00   5.00   6.00   5.00    -      -     6.00   5.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w1, #0xff
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w8, #16
1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.hs	.LBB181_2
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w10, #65535
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w1
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     sub	w9, w8, w0
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     bic	w10, w10, w9
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     bic	w1, w8, w9
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     cmp	w10, w0, uxth
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, hs
 -     1.00   1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w0, #0xffff
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mov	w1, wzr
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, eq
```
## `unbounded_ceil_to_multiple_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      15
Total uOps:        15

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 6.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w1, #0xff
 1      1     0.25                        cmp	w8, #32
 1      0     0.50                        b.hs	.LBB182_2
 1      1     0.25                        mov	w8, #-1
 1      1     0.50                        lsl	w8, w8, w1
 1      2     1.00                        mvn	w9, w8
 1      2     1.00                        add	w9, w0, w9
 1      2     1.00                        cmp	w9, w0
 1      2     1.00                        sub	w9, w8, w0
 1      1     0.25                        cset	w0, hs
 1      2     1.00                        bic	w1, w8, w9
 1      0     1.00                  U     ret
 1      1     0.25                        cmp	w0, #0
 1      2     1.00                        mov	w1, wzr
 1      1     0.25                        cset	w0, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   1.00   1.00    -      -     4.00   6.00   6.00   5.00    -      -     6.00   7.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w1, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w8, #32
1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.hs	.LBB182_2
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	w9, w8
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     add	w9, w0, w9
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     cmp	w9, w0
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     sub	w9, w8, w0
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, hs
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     bic	w1, w8, w9
 -     1.00   1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w0, #0
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mov	w1, wzr
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, eq
```
## `unbounded_ceil_to_multiple_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      17
Total Cycles:      18
Total uOps:        17

Dispatch Width:    6
uOps Per Cycle:    0.94
IPC:               0.94
Block RThroughput: 8.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w1, #0xff
 1      1     0.25                        cmp	w8, #64
 1      0     0.50                        b.hs	.LBB183_2
 1      1     0.25                        mov	x8, #-1
 1      1     0.50                        lsl	x9, x8, x1
 1      2     1.00                        mvn	x8, x9
 1      2     1.00                        sub	x10, x9, x0
 1      2     1.00                        add	x8, x0, x8
 1      2     1.00                        bic	x1, x9, x10
 1      2     1.00                        cmp	x8, x0
 1      1     0.25                        cset	w8, hs
 1      2     1.00                        mov	w0, w8
 1      0     1.00                  U     ret
 1      1     0.25                        cmp	x0, #0
 1      2     1.00                        mov	x1, xzr
 1      1     0.25                        cset	w8, eq
 1      2     1.00                        mov	w0, w8

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   1.00   1.00    -      -     4.00   7.00   6.00   8.00    -      -     8.00   9.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w1, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w8, #64
 -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.hs	.LBB183_2
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x9, x8, x1
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	x8, x9
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     sub	x10, x9, x0
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     add	x8, x0, x8
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     bic	x1, x9, x10
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     cmp	x8, x0
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w8, hs
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mov	w0, w8
1.00    -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	x0, #0
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mov	x1, xzr
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w8, eq
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mov	w0, w8
```
## `unbounded_ceil_to_multiple_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      28
Total Cycles:      22
Total uOps:        31

Dispatch Width:    6
uOps Per Cycle:    1.41
IPC:               1.27
Block RThroughput: 10.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.50                        tbnz	w2, #7, .LBB180_5
 1      1     0.25                        mov	x9, #-1
 1      2     1.00                        mvn	w10, w2
 1      1     0.25                        mov	x11, #9223372036854775807
 1      1     0.50                        lsl	x12, x9, x2
 1      1     0.25                        and	x9, x2, #0xff
 1      1     0.50                        lsr	x10, x11, x10
 1      1     0.25                        tst	x9, #0x40
 1      1     0.25                        csel	x9, xzr, x12, ne
 1      2     1.00                        orr	x10, x12, x10
 1      1     0.25                        csel	x11, x12, x10, ne
 1      2     1.00                        mvn	x10, x9
 1      2     1.00                        mvn	x12, x11
 1      2     1.00                        adds	x10, x0, x10
 1      1     0.25                        adcs	x12, x1, x12
 1      1     0.25                        cset	w13, hs
 1      0     0.50                        tbnz	w13, #0, .LBB180_6
 1      2     1.00                        and	x10, x10, x9
 1      2     1.00                        and	x9, x12, x11
 2      6     0.50           *            stp	x10, x9, [x8, #16]
 1      1     0.25                        mov	w10, #1
 2      6     0.50           *            stp	x10, xzr, [x8]
 1      0     1.00                  U     ret
 1      2     1.00                        mov	x10, xzr
 1      2     1.00                        orr	x9, x0, x1
 1      0     0.50                        cbnz	x9, .LBB180_4
 1      0     0.50                        b	.LBB180_3
 2      6     0.50           *            stp	xzr, xzr, [x8]

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
2.00   3.00   1.00    -      -     7.00   10.00  8.00   9.00    -      -     9.00   11.00   -     1.00   2.00   1.00   1.00   1.00    -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tbnz	w2, #7, .LBB180_5
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x9, #-1
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	w10, w2
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x11, #9223372036854775807
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x12, x9, x2
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x9, x2, #0xff
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x10, x11, x10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x9, #0x40
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x9, xzr, x12, ne
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x10, x12, x10
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x11, x12, x10, ne
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	x10, x9
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	x12, x11
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     adds	x10, x0, x10
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     adcs	x12, x1, x12
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w13, hs
 -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tbnz	w13, #0, .LBB180_6
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     and	x10, x10, x9
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     and	x9, x12, x11
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -     stp	x10, x9, [x8, #16]
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w10, #1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     stp	x10, xzr, [x8]
1.00    -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mov	x10, xzr
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     orr	x9, x0, x1
1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cbnz	x9, .LBB180_4
 -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     b	.LBB180_3
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00   1.00    -      -      -      -      -      -     stp	xzr, xzr, [x8]
```
## `unbounded_div_ceil_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      11
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 3.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	x8, #-1
 1      1     0.50                        sxtb	w9, w0
 1      1     0.25                        and	w10, w1, #0xff
 1      1     0.50                        lsl	x8, x8, x1
 1      1     0.25                        cmp	w9, #0
 1      2     1.00                        mvn	x8, x8
 1      1     0.25                        cset	w9, gt
 1      1     0.25                        cmp	w10, #8
 1      2     1.00                        add	x8, x8, w0, sxtb
 1      1     0.50                        asr	x8, x8, x1
 1      1     0.25                        csel	w0, w8, w9, lo

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   4.00   3.00   4.00    -      -     3.00   4.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     sxtb	w9, w0
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w10, w1, #0xff
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w9, #0
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	x8, x8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w9, gt
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w10, #8
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     add	x8, x8, w0, sxtb
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	x8, x8, x1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	w0, w8, w9, lo
```
## `unbounded_div_ceil_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      11
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 3.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	x8, #-1
 1      1     0.50                        sxth	w9, w0
 1      1     0.25                        and	w10, w1, #0xff
 1      1     0.50                        lsl	x8, x8, x1
 1      1     0.25                        cmp	w9, #0
 1      2     1.00                        mvn	x8, x8
 1      1     0.25                        cset	w9, gt
 1      1     0.25                        cmp	w10, #16
 1      2     1.00                        add	x8, x8, w0, sxth
 1      1     0.50                        asr	x8, x8, x1
 1      1     0.25                        csel	w0, w8, w9, lo

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   4.00   3.00   4.00    -      -     3.00   4.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     sxth	w9, w0
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w10, w1, #0xff
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w9, #0
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	x8, x8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w9, gt
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w10, #16
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     add	x8, x8, w0, sxth
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	x8, x8, x1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	w0, w8, w9, lo
```
## `unbounded_div_ceil_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      12
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    0.92
IPC:               0.92
Block RThroughput: 3.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w1, #0xff
 1      1     0.25                        cmp	w8, #32
 1      0     0.50                        b.hs	.LBB187_2
 1      1     0.25                        mov	x8, #-1
 1      1     0.50                        lsl	x8, x8, x1
 1      2     1.00                        mvn	x8, x8
 1      2     1.00                        add	x8, x8, w0, sxtw
 1      1     0.50                        asr	x0, x8, x1
 1      0     1.00                  U     ret
 1      1     0.25                        cmp	w0, #0
 1      1     0.25                        cset	w0, gt

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   1.00   1.00    -      -     3.00   4.00   3.00   3.00    -      -     3.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w1, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w8, #32
1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.hs	.LBB187_2
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	x8, x8
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     add	x8, x8, w0, sxtw
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	x0, x8, x1
 -     1.00   1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w0, #0
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, gt
```
## `unbounded_div_ceil_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      9
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        asr	x8, x0, x1
 1      1     0.50                        lsl	x9, x8, x1
 1      2     1.00                        cmp	x0, x9
 1      1     0.25                        and	w9, w1, #0xff
 1      1     0.25                        cinc	x8, x8, ne
 1      1     0.25                        cmp	x0, #0
 1      1     0.25                        cset	w10, gt
 1      1     0.25                        cmp	w9, #64
 1      1     0.25                        csel	x0, x8, x10, lo

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   3.00   2.00   3.00    -      -     1.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	x8, x0, x1
 -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x9, x8, x1
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     cmp	x0, x9
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w9, w1, #0xff
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cinc	x8, x8, ne
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	x0, #0
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w10, gt
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w9, #64
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x0, x8, x10, lo
```
## `unbounded_div_ceil_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      29
Total Cycles:      23
Total uOps:        29

Dispatch Width:    6
uOps Per Cycle:    1.26
IPC:               1.26
Block RThroughput: 11.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.50                        tbnz	w2, #7, .LBB185_2
 1      1     0.50                        lsl	x8, x1, #1
 1      2     1.00                        mvn	w9, w2
 1      1     0.50                        lsr	x10, x0, x2
 1      1     0.50                        asr	x11, x1, x2
 1      1     0.25                        and	x12, x2, #0xff
 1      1     0.50                        lsl	x8, x8, x9
 1      1     0.25                        tst	x12, #0x40
 1      2     1.00                        orr	x8, x8, x10
 1      1     0.50                        asr	x10, x1, #63
 1      1     0.25                        csel	x8, x11, x8, ne
 1      1     0.50                        lsr	x12, x8, #1
 1      1     0.25                        csel	x10, x10, x11, ne
 1      1     0.50                        lsl	x11, x10, x2
 1      1     0.50                        lsr	x9, x12, x9
 1      1     0.50                        lsl	x12, x8, x2
 1      2     1.00                        orr	x9, x11, x9
 1      1     0.25                        csel	x11, xzr, x12, ne
 1      1     0.25                        csel	x9, x12, x9, ne
 1      2     1.00                        cmp	x0, x11
 1      1     0.25                        ccmp	x1, x9, #0, eq
 1      1     0.25                        cset	w9, ne
 1      2     1.00                        adds	x0, x8, x9
 1      1     0.25                        cinc	x1, x10, hs
 1      0     1.00                  U     ret
 1      2     1.00                        cmp	xzr, x0
 1      1     0.25                        ngcs	xzr, x1
 1      2     1.00                        mov	x1, xzr
 1      1     0.25                        cset	w0, lt

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   1.00   1.00    -      -     10.00  9.00   8.00   9.00    -      -     13.00  10.00   -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tbnz	w2, #7, .LBB185_2
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x1, #1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	w9, w2
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x10, x0, x2
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	x11, x1, x2
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x12, x2, #0xff
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x9
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x12, #0x40
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x8, x8, x10
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	x10, x1, #63
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x8, x11, x8, ne
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x12, x8, #1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x10, x10, x11, ne
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x11, x10, x2
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x9, x12, x9
 -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x12, x8, x2
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x9, x11, x9
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x11, xzr, x12, ne
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x9, x12, x9, ne
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     cmp	x0, x11
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ccmp	x1, x9, #0, eq
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w9, ne
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     adds	x0, x8, x9
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     cinc	x1, x10, hs
1.00    -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     cmp	xzr, x0
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ngcs	xzr, x1
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mov	x1, xzr
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, lt
```
## `unbounded_div_ceil_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      12
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    0.92
IPC:               0.92
Block RThroughput: 3.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w1, #0xff
 1      1     0.25                        cmp	w8, #8
 1      0     0.50                        b.hs	.LBB194_2
 1      1     0.25                        mov	x8, #-1
 1      1     0.50                        lsl	x8, x8, x1
 1      2     1.00                        mvn	x8, x8
 1      2     1.00                        add	x8, x8, w0, uxtb
 1      1     0.50                        lsr	x0, x8, x1
 1      0     1.00                  U     ret
 1      1     0.25                        tst	w0, #0xff
 1      1     0.25                        cset	w0, ne

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   1.00   1.00    -      -     3.00   4.00   3.00   3.00    -      -     3.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w1, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w8, #8
1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.hs	.LBB194_2
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	x8, x8
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     add	x8, x8, w0, uxtb
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	x0, x8, x1
 -     1.00   1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w0, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, ne
```
## `unbounded_div_ceil_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      12
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    0.92
IPC:               0.92
Block RThroughput: 3.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w1, #0xff
 1      1     0.25                        cmp	w8, #16
 1      0     0.50                        b.hs	.LBB191_2
 1      1     0.25                        mov	x8, #-1
 1      1     0.50                        lsl	x8, x8, x1
 1      2     1.00                        mvn	x8, x8
 1      2     1.00                        add	x8, x8, w0, uxth
 1      1     0.50                        lsr	x0, x8, x1
 1      0     1.00                  U     ret
 1      1     0.25                        tst	w0, #0xffff
 1      1     0.25                        cset	w0, ne

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   1.00   1.00    -      -     3.00   4.00   3.00   3.00    -      -     3.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w1, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w8, #16
1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.hs	.LBB191_2
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	x8, x8
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     add	x8, x8, w0, uxth
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	x0, x8, x1
 -     1.00   1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w0, #0xffff
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, ne
```
## `unbounded_div_ceil_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      12
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    0.92
IPC:               0.92
Block RThroughput: 3.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w1, #0xff
 1      1     0.25                        cmp	w8, #32
 1      0     0.50                        b.hs	.LBB192_2
 1      1     0.25                        mov	x8, #-1
 1      1     0.50                        lsl	x8, x8, x1
 1      2     1.00                        mvn	x8, x8
 1      2     1.00                        add	x8, x8, w0, uxtw
 1      1     0.50                        lsr	x0, x8, x1
 1      0     1.00                  U     ret
 1      1     0.25                        cmp	w0, #0
 1      1     0.25                        cset	w0, ne

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   1.00   1.00    -      -     3.00   4.00   3.00   3.00    -      -     3.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w1, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w8, #32
1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.hs	.LBB192_2
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	x8, x8
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     add	x8, x8, w0, uxtw
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	x0, x8, x1
 -     1.00   1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w0, #0
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, ne
```
## `unbounded_div_ceil_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      9
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsr	x8, x0, x1
 1      1     0.50                        lsl	x9, x8, x1
 1      2     1.00                        cmp	x0, x9
 1      1     0.25                        and	w9, w1, #0xff
 1      1     0.25                        cinc	x8, x8, ne
 1      1     0.25                        cmp	x0, #0
 1      1     0.25                        cset	w10, ne
 1      1     0.25                        cmp	w9, #64
 1      1     0.25                        csel	x0, x8, x10, lo

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   3.00   2.00   3.00    -      -     1.00   3.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x8, x0, x1
 -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x9, x8, x1
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     cmp	x0, x9
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w9, w1, #0xff
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cinc	x8, x8, ne
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	x0, #0
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w10, ne
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w9, #64
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x0, x8, x10, lo
```
## `unbounded_div_ceil_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      28
Total Cycles:      23
Total uOps:        28

Dispatch Width:    6
uOps Per Cycle:    1.22
IPC:               1.22
Block RThroughput: 11.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.50                        tbnz	w2, #7, .LBB190_2
 1      1     0.50                        lsl	x8, x1, #1
 1      2     1.00                        mvn	w9, w2
 1      1     0.50                        lsr	x10, x0, x2
 1      1     0.50                        lsr	x11, x1, x2
 1      1     0.25                        and	x12, x2, #0xff
 1      1     0.50                        lsl	x8, x8, x9
 1      1     0.25                        tst	x12, #0x40
 1      2     1.00                        orr	x8, x8, x10
 1      1     0.25                        csel	x8, x11, x8, ne
 1      1     0.25                        csel	x11, xzr, x11, ne
 1      1     0.50                        lsr	x10, x8, #1
 1      1     0.50                        lsl	x12, x11, x2
 1      1     0.50                        lsr	x9, x10, x9
 1      1     0.50                        lsl	x10, x8, x2
 1      2     1.00                        orr	x9, x12, x9
 1      1     0.25                        csel	x12, xzr, x10, ne
 1      1     0.25                        csel	x9, x10, x9, ne
 1      2     1.00                        cmp	x0, x12
 1      1     0.25                        ccmp	x1, x9, #0, eq
 1      1     0.25                        cset	w9, ne
 1      2     1.00                        adds	x0, x8, x9
 1      1     0.25                        cinc	x1, x11, hs
 1      0     1.00                  U     ret
 1      2     1.00                        orr	x9, x0, x1
 1      2     1.00                        mov	x1, xzr
 1      1     0.25                        cmp	x9, #0
 1      1     0.25                        cset	w0, ne

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
1.00   1.00   1.00    -      -     8.00   9.00   9.00   9.00    -      -     11.00  11.00   -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tbnz	w2, #7, .LBB190_2
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x1, #1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	w9, w2
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x10, x0, x2
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	x11, x1, x2
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x12, x2, #0xff
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x9
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x12, #0x40
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x8, x8, x10
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x8, x11, x8, ne
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x11, xzr, x11, ne
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	x10, x8, #1
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x12, x11, x2
 -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	x9, x10, x9
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x10, x8, x2
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x9, x12, x9
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x12, xzr, x10, ne
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x9, x10, x9, ne
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     cmp	x0, x12
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ccmp	x1, x9, #0, eq
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w9, ne
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     adds	x0, x8, x9
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     cinc	x1, x11, hs
1.00    -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     orr	x9, x0, x1
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mov	x1, xzr
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	x9, #0
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, ne
```
## `unbounded_div_floor_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      7
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.86
IPC:               0.86
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w9, w1, #0xff
 1      1     0.25                        mov	w8, #7
 1      1     0.50                        sxtb	w10, w0
 1      1     0.25                        cmp	w9, #7
 1      1     0.25                        csel	w8, w9, w8, lo
 1      1     0.50                        asr	w0, w10, w8

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   1.00   2.00   2.00    -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w9, w1, #0xff
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #7
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     sxtb	w10, w0
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w9, #7
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	w8, w9, w8, lo
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	w0, w10, w8
```
## `unbounded_div_floor_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      7
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.86
IPC:               0.86
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w9, w1, #0xff
 1      1     0.25                        mov	w8, #15
 1      1     0.50                        sxth	w10, w0
 1      1     0.25                        cmp	w9, #15
 1      1     0.25                        csel	w8, w9, w8, lo
 1      1     0.50                        asr	w0, w10, w8

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   1.00   2.00   2.00    -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w9, w1, #0xff
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #15
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     sxth	w10, w0
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w9, #15
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	w8, w9, w8, lo
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	w0, w10, w8
```
## `unbounded_div_floor_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      7
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.71
IPC:               0.71
Block RThroughput: 1.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w9, w1, #0xff
 1      1     0.25                        mov	w8, #31
 1      1     0.25                        cmp	w9, #31
 1      1     0.25                        csel	w8, w9, w8, lo
 1      1     0.50                        asr	w0, w0, w8

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   1.00   1.00   2.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w9, w1, #0xff
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #31
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w9, #31
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	w8, w9, w8, lo
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	w0, w0, w8
```
## `unbounded_div_floor_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      7
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.71
IPC:               0.71
Block RThroughput: 1.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w9, w1, #0xff
 1      1     0.25                        mov	w8, #63
 1      1     0.25                        cmp	w9, #63
 1      1     0.25                        csel	w8, w9, w8, lo
 1      1     0.50                        asr	x0, x0, x8

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   1.00   1.00   2.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w9, w1, #0xff
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #63
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w9, #63
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	w8, w9, w8, lo
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	x0, x0, x8
```
## `unbounded_div_floor_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      12
Total uOps:        15

Dispatch Width:    6
uOps Per Cycle:    1.25
IPC:               1.25
Block RThroughput: 5.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        sxtb	w8, w2
 1      1     0.50                        lsl	x9, x1, #1
 1      1     0.50                        asr	x12, x1, #63
 1      1     0.25                        cmn	w8, #1
 1      1     0.25                        mov	w8, #127
 1      1     0.25                        csel	w8, w2, w8, gt
 1      2     1.00                        mvn	w10, w8
 1      1     0.50                        lsr	x11, x0, x8
 1      1     0.50                        lsl	x9, x9, x10
 1      1     0.50                        asr	x10, x1, x8
 1      1     0.25                        and	x8, x8, #0xff
 1      1     0.25                        tst	x8, #0x40
 1      2     1.00                        orr	x8, x9, x11
 1      1     0.25                        csel	x1, x12, x10, ne
 1      1     0.25                        csel	x0, x10, x8, ne

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     4.00   5.00   4.00   4.00    -      -     4.00   6.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     sxtb	w8, w2
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x9, x1, #1
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	x12, x1, #63
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmn	w8, #1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #127
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	w8, w2, w8, gt
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	w10, w8
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x11, x0, x8
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x9, x9, x10
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	x10, x1, x8
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x8, x8, #0xff
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x8, #0x40
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     orr	x8, x9, x11
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x1, x12, x10, ne
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x0, x10, x8, ne
```
## `unbounded_div_floor_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      6
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.67
IPC:               0.67
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w0, #0xff
 1      1     0.25                        tst	w1, #0xf8
 1      1     0.50                        lsr	w8, w8, w1
 1      1     0.25                        csel	w0, w8, wzr, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   1.00   1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w0, #0xff
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w1, #0xf8
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	w8, w8, w1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	w0, w8, wzr, eq
```
## `unbounded_div_floor_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      6
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.67
IPC:               0.67
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w0, #0xffff
 1      1     0.25                        tst	w1, #0xf0
 1      1     0.50                        lsr	w8, w8, w1
 1      1     0.25                        csel	w0, w8, wzr, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   1.00   1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w0, #0xffff
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w1, #0xf0
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	w8, w8, w1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	w0, w8, wzr, eq
```
## `unbounded_div_floor_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      5
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.60
IPC:               0.60
Block RThroughput: 0.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsr	w8, w0, w1
 1      1     0.25                        tst	w1, #0xe0
 1      1     0.25                        csel	w0, w8, wzr, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -     1.00   1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	w8, w0, w1
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w1, #0xe0
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	w0, w8, wzr, eq
```
## `unbounded_div_floor_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      5
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.60
IPC:               0.60
Block RThroughput: 0.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsr	x8, x0, x1
 1      1     0.25                        tst	w1, #0xc0
 1      1     0.25                        csel	x0, x8, xzr, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -      -     1.00   1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x8, x0, x1
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w1, #0xc0
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x0, x8, xzr, eq
```
## `unbounded_div_floor_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      10
Total uOps:        14

Dispatch Width:    6
uOps Per Cycle:    1.40
IPC:               1.40
Block RThroughput: 4.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsl	x8, x1, #1
 1      2     1.00                        mvn	w9, w2
 1      1     0.50                        lsr	x10, x0, x2
 1      1     0.50                        lsr	x11, x1, x2
 1      1     0.25                        and	x12, x2, #0xff
 1      1     0.50                        lsl	x8, x8, x9
 1      1     0.50                        sxtb	w9, w2
 1      1     0.25                        tst	x12, #0x40
 1      2     1.00                        orr	x8, x8, x10
 1      1     0.25                        csel	x10, xzr, x11, ne
 1      1     0.25                        csel	x8, x11, x8, ne
 1      1     0.25                        cmn	w9, #1
 1      1     0.25                        csel	x1, x10, xzr, gt
 1      1     0.25                        csel	x0, x8, xzr, gt

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     3.00   3.00   5.00   5.00    -      -     4.00   5.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x1, #1
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	w9, w2
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x10, x0, x2
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	x11, x1, x2
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x12, x2, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x9
 -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     sxtb	w9, w2
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x12, #0x40
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     orr	x8, x8, x10
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x10, xzr, x11, ne
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x8, x11, x8, ne
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmn	w9, #1
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x1, x10, xzr, gt
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x0, x8, xzr, gt
```
## `unbounded_div_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      12
Total uOps:        15

Dispatch Width:    6
uOps Per Cycle:    1.25
IPC:               1.25
Block RThroughput: 5.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w1, #0xff
 1      1     0.25                        cmp	w8, #7
 1      0     0.50                        b.hi	.LBB209_3
 1      0     0.50                        tbnz	w0, #7, .LBB209_4
 1      1     0.25                        and	w8, w0, #0xff
 1      1     0.50                        lsr	w0, w8, w1
 1      0     1.00                  U     ret
 1      2     1.00                        mov	w0, wzr
 1      0     1.00                  U     ret
 1      1     0.25                        mov	w8, #-1
 1      1     0.50                        lsl	w8, w8, w1
 1      2     1.00                        mvn	w8, w8
 1      2     1.00                        add	w8, w0, w8
 1      1     0.50                        sxtb	w8, w8
 1      1     0.50                        asr	w0, w8, w1

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
2.00   2.00   2.00    -      -     4.00   4.00   4.00   6.00    -      -     5.00   5.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w1, #0xff
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w8, #7
 -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.hi	.LBB209_3
 -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tbnz	w0, #7, .LBB209_4
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w0, #0xff
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	w0, w8, w1
1.00    -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mov	w0, wzr
1.00    -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w8, #-1
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w1
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	w8, w8
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     add	w8, w0, w8
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     sxtb	w8, w8
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	w0, w8, w1
```
## `unbounded_div_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      12
Total uOps:        15

Dispatch Width:    6
uOps Per Cycle:    1.25
IPC:               1.25
Block RThroughput: 5.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w1, #0xff
 1      1     0.25                        cmp	w8, #15
 1      0     0.50                        b.hi	.LBB206_3
 1      0     0.50                        tbnz	w0, #15, .LBB206_4
 1      1     0.25                        and	w9, w0, #0xffff
 1      1     0.50                        lsr	w0, w9, w8
 1      0     1.00                  U     ret
 1      2     1.00                        mov	w0, wzr
 1      0     1.00                  U     ret
 1      1     0.25                        mov	w9, #-1
 1      1     0.50                        lsl	w9, w9, w8
 1      2     1.00                        mvn	w9, w9
 1      2     1.00                        add	w9, w0, w9
 1      1     0.50                        sxth	w9, w9
 1      1     0.50                        asr	w0, w9, w8

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
2.00   2.00   2.00    -      -     4.00   4.00   4.00   6.00    -      -     5.00   5.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w1, #0xff
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w8, #15
 -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.hi	.LBB206_3
 -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tbnz	w0, #15, .LBB206_4
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w9, w0, #0xffff
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	w0, w9, w8
1.00    -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mov	w0, wzr
1.00    -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w9, #-1
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w9, w9, w8
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	w9, w9
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     add	w9, w0, w9
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     sxth	w9, w9
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     asr	w0, w9, w8
```
## `unbounded_div_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      13
Total Cycles:      11
Total uOps:        13

Dispatch Width:    6
uOps Per Cycle:    1.18
IPC:               1.18
Block RThroughput: 4.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w1, #0xff
 1      1     0.25                        cmp	w8, #31
 1      0     0.50                        b.hi	.LBB207_3
 1      0     0.50                        tbnz	w0, #31, .LBB207_4
 1      1     0.50                        lsr	w0, w0, w8
 1      0     1.00                  U     ret
 1      2     1.00                        mov	w0, wzr
 1      0     1.00                  U     ret
 1      1     0.25                        mov	w9, #-1
 1      1     0.50                        lsl	w9, w9, w8
 1      2     1.00                        mvn	w9, w9
 1      2     1.00                        add	w9, w0, w9
 1      1     0.50                        asr	w0, w9, w8

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
2.00   2.00   2.00    -      -     4.00   4.00   3.00   5.00    -      -     4.00   5.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w1, #0xff
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w8, #31
1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.hi	.LBB207_3
 -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tbnz	w0, #31, .LBB207_4
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	w0, w0, w8
1.00    -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mov	w0, wzr
 -     1.00   1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w9, #-1
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w9, w9, w8
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	w9, w9
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     add	w9, w0, w9
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	w0, w9, w8
```
## `unbounded_div_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      11
Total uOps:        14

Dispatch Width:    6
uOps Per Cycle:    1.27
IPC:               1.27
Block RThroughput: 4.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w1, #0xff
 1      1     0.25                        cmp	w8, #63
 1      0     0.50                        b.hi	.LBB208_3
 1      1     0.25                        and	x8, x1, #0xff
 1      0     0.50                        tbnz	x0, #63, .LBB208_4
 1      1     0.50                        lsr	x0, x0, x8
 1      0     1.00                  U     ret
 1      2     1.00                        mov	x0, xzr
 1      0     1.00                  U     ret
 1      1     0.25                        mov	x9, #-1
 1      1     0.50                        lsl	x9, x9, x8
 1      2     1.00                        mvn	x9, x9
 1      2     1.00                        add	x9, x0, x9
 1      1     0.50                        asr	x0, x9, x8

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
2.00   2.00   2.00    -      -     4.00   3.00   4.00   6.00    -      -     4.00   5.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w1, #0xff
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w8, #63
 -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     b.hi	.LBB208_3
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x8, x1, #0xff
 -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tbnz	x0, #63, .LBB208_4
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x0, x0, x8
1.00    -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mov	x0, xzr
1.00    -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x9, #-1
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x9, x9, x8
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	x9, x9
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     add	x9, x0, x9
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	x0, x9, x8
```
## `unbounded_div_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      40
Total Cycles:      23
Total uOps:        40

Dispatch Width:    6
uOps Per Cycle:    1.74
IPC:               1.74
Block RThroughput: 16.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.50                        tbnz	w2, #7, .LBB205_3
 1      0     0.50                        tbnz	x1, #63, .LBB205_4
 1      1     0.50                        lsl	x8, x1, #1
 1      2     1.00                        mov	w9, w2
 1      2     1.00                        mvn	w10, w2
 1      1     0.50                        lsr	x11, x0, x9
 1      1     0.50                        lsl	x8, x8, x10
 1      1     0.50                        lsr	x10, x1, x9
 1      1     0.25                        and	x9, x9, #0xff
 1      1     0.25                        tst	x9, #0x40
 1      2     1.00                        orr	x8, x8, x11
 1      1     0.25                        csel	x1, xzr, x10, ne
 1      1     0.25                        csel	x0, x10, x8, ne
 1      0     1.00                  U     ret
 1      2     1.00                        mov	x0, xzr
 1      2     1.00                        mov	x1, xzr
 1      0     1.00                  U     ret
 1      1     0.25                        mov	x8, #-1
 1      2     1.00                        mvn	w9, w2
 1      1     0.25                        mov	x10, #9223372036854775807
 1      1     0.50                        lsl	x8, x8, x2
 1      1     0.25                        and	x11, x2, #0xff
 1      1     0.50                        lsr	x10, x10, x9
 1      1     0.25                        tst	x11, #0x40
 1      1     0.25                        csel	x12, xzr, x8, ne
 1      2     1.00                        orr	x10, x8, x10
 1      1     0.25                        csel	x8, x8, x10, ne
 1      2     1.00                        mvn	x10, x12
 1      2     1.00                        mvn	x8, x8
 1      2     1.00                        adds	x10, x0, x10
 1      1     0.25                        adc	x8, x1, x8
 1      1     0.50                        lsr	x10, x10, x2
 1      1     0.25                        tst	x11, #0x40
 1      1     0.50                        lsl	x12, x8, #1
 1      1     0.50                        lsl	x9, x12, x9
 1      1     0.50                        asr	x12, x8, x2
 1      1     0.50                        asr	x8, x8, #63
 1      2     1.00                        orr	x9, x9, x10
 1      1     0.25                        csel	x1, x8, x12, ne
 1      1     0.25                        csel	x0, x12, x9, ne

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
2.00   2.00   2.00    -      -     13.00  11.00  13.00  14.00   -      -     17.00  16.00   -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tbnz	w2, #7, .LBB205_3
1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tbnz	x1, #63, .LBB205_4
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x1, #1
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mov	w9, w2
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	w10, w2
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	x11, x0, x9
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x10
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	x10, x1, x9
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x9, x9, #0xff
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x9, #0x40
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     orr	x8, x8, x11
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x1, xzr, x10, ne
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x0, x10, x8, ne
 -     1.00   1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mov	x0, xzr
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mov	x1, xzr
1.00    -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	w9, w2
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x10, #9223372036854775807
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x2
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x11, x2, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	x10, x10, x9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x11, #0x40
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x12, xzr, x8, ne
 -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     orr	x10, x8, x10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x8, x8, x10, ne
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	x10, x12
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	x8, x8
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     adds	x10, x0, x10
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     adc	x8, x1, x8
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x10, x10, x2
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x11, #0x40
 -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x12, x8, #1
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x9, x12, x9
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	x12, x8, x2
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     asr	x8, x8, #63
 -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x9, x9, x10
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x1, x8, x12, ne
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x0, x12, x9, ne
```
## `unbounded_floor_to_multiple_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      7
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.43
IPC:               1.43
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w0, #0xff
 1      1     0.50                        sxtb	w9, w0
 1      1     0.25                        and	w10, w1, #0xff
 1      1     0.50                        lsr	w8, w8, w1
 1      1     0.25                        cmn	w9, #1
 1      1     0.50                        lsl	w8, w8, w1
 1      1     0.25                        cset	w9, gt
 1      1     0.25                        cmp	w10, #8
 1      1     0.25                        csinc	w0, w9, wzr, hs
 1      1     0.25                        csel	w1, w8, wzr, lo

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   2.00   3.00   3.00    -      -     1.00   2.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w0, #0xff
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     sxtb	w9, w0
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w10, w1, #0xff
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	w8, w8, w1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmn	w9, #1
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w9, gt
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w10, #8
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csinc	w0, w9, wzr, hs
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	w1, w8, wzr, lo
```
## `unbounded_floor_to_multiple_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      7
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    1.29
IPC:               1.29
Block RThroughput: 2.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w0, #0xffff
 1      1     0.50                        ubfx	w9, w0, #15, #1
 1      1     0.25                        and	w10, w1, #0xff
 1      1     0.50                        lsr	w8, w8, w1
 1      1     0.25                        cmp	w10, #16
 1      1     0.25                        eor	w9, w9, #0x1
 1      1     0.50                        lsl	w8, w8, w1
 1      1     0.25                        csinc	w0, w9, wzr, hs
 1      1     0.25                        csel	w1, w8, wzr, lo

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   2.00   2.00   3.00    -      -     1.00   2.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w0, #0xffff
 -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     ubfx	w9, w0, #15, #1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w10, w1, #0xff
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	w8, w8, w1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w10, #16
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     eor	w9, w9, #0x1
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csinc	w0, w9, wzr, hs
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	w1, w8, wzr, lo
```
## `unbounded_floor_to_multiple_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      7
Total uOps:        8

Dispatch Width:    6
uOps Per Cycle:    1.14
IPC:               1.14
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsr	w8, w0, w1
 1      2     1.00                        mvn	w9, w0
 1      1     0.25                        and	w10, w1, #0xff
 1      1     0.50                        lsr	w9, w9, #31
 1      1     0.25                        cmp	w10, #32
 1      1     0.50                        lsl	w8, w8, w1
 1      1     0.25                        csinc	w0, w9, wzr, hs
 1      1     0.25                        csel	w1, w8, wzr, lo

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   2.00   3.00   2.00    -      -     3.00   2.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	w8, w0, w1
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     mvn	w9, w0
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w10, w1, #0xff
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsr	w9, w9, #31
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w10, #32
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csinc	w0, w9, wzr, hs
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	w1, w8, wzr, lo
```
## `unbounded_floor_to_multiple_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      6
Total uOps:        8

Dispatch Width:    6
uOps Per Cycle:    1.33
IPC:               1.33
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsr	x8, x0, x1
 1      1     0.25                        and	w9, w1, #0xff
 1      1     0.25                        cmn	x0, #1
 1      1     0.25                        cset	w10, gt
 1      1     0.25                        cmp	w9, #64
 1      1     0.50                        lsl	x8, x8, x1
 1      1     0.25                        csinc	w0, w10, wzr, hs
 1      1     0.25                        csel	x1, x8, xzr, lo

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     2.00   2.00   2.00   2.00    -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x8, x0, x1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w9, w1, #0xff
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmn	x0, #1
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w10, gt
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w9, #64
 -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csinc	w0, w10, wzr, hs
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x1, x8, xzr, lo
```
## `unbounded_floor_to_multiple_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      23
Total Cycles:      19
Total uOps:        28

Dispatch Width:    6
uOps Per Cycle:    1.47
IPC:               1.21
Block RThroughput: 5.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.50                        tbnz	w2, #7, .LBB210_2
 1      1     0.25                        mov	x9, #-1
 1      2     1.00                        mvn	w10, w2
 1      1     0.25                        mov	x11, #9223372036854775807
 1      1     0.50                        lsl	x9, x9, x2
 1      1     0.50                        lsr	x10, x11, x10
 1      1     0.25                        and	x12, x2, #0xff
 1      1     0.25                        tst	x12, #0x40
 1      2     1.00                        orr	x10, x9, x10
 1      1     0.25                        csel	x11, xzr, x9, ne
 1      1     0.25                        csel	x9, x9, x10, ne
 1      2     1.00                        and	x10, x11, x0
 1      2     1.00                        and	x9, x9, x1
 2      6     0.50           *            stp	x10, x9, [x8, #16]
 1      1     0.25                        mov	w9, #1
 2      6     0.50           *            stp	x9, xzr, [x8]
 1      0     1.00                  U     ret
 1      0     0.50                        tbnz	x1, #63, .LBB210_4
 1      1     0.25                        mov	w9, #1
 2      6     0.50           *            stp	xzr, xzr, [x8, #16]
 2      6     0.50           *            stp	x9, xzr, [x8]
 1      0     1.00                  U     ret
 2      6     0.50           *            stp	xzr, xzr, [x8]

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
2.00   2.00   2.00    -      -     4.00   8.00   5.00   5.00    -      -     5.00   5.00    -     2.00   3.00   1.00   2.00   2.00    -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tbnz	w2, #7, .LBB210_2
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x9, #-1
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	w10, w2
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x11, #9223372036854775807
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x9, x9, x2
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x10, x11, x10
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x12, x2, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x12, #0x40
 -      -      -      -      -      -      -     2.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     orr	x10, x9, x10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x11, xzr, x9, ne
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x9, x9, x10, ne
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     and	x10, x11, x0
 -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     and	x9, x9, x1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -     stp	x10, x9, [x8, #16]
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w9, #1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     stp	x9, xzr, [x8]
1.00    -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     tbnz	x1, #63, .LBB210_4
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	w9, #1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00   1.00    -      -      -      -      -      -     stp	xzr, xzr, [x8, #16]
 -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -     stp	x9, xzr, [x8]
1.00    -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -     1.00    -      -      -      -      -     stp	xzr, xzr, [x8]
```
## `unbounded_floor_to_multiple_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      7
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.71
IPC:               0.71
Block RThroughput: 1.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w0, #0xff
 1      1     0.25                        tst	w1, #0xf8
 1      1     0.50                        lsr	w8, w8, w1
 1      1     0.50                        lsl	w8, w8, w1
 1      1     0.25                        csel	w0, w8, wzr, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   1.00   1.00   2.00    -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w0, #0xff
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w1, #0xf8
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	w8, w8, w1
 -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w1
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	w0, w8, wzr, eq
```
## `unbounded_floor_to_multiple_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      7
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.71
IPC:               0.71
Block RThroughput: 1.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        and	w8, w0, #0xffff
 1      1     0.25                        tst	w1, #0xf0
 1      1     0.50                        lsr	w8, w8, w1
 1      1     0.50                        lsl	w8, w8, w1
 1      1     0.25                        csel	w0, w8, wzr, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   1.00   1.00   2.00    -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w8, w0, #0xffff
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w1, #0xf0
 -      -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	w8, w8, w1
 -      -      -      -      -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w1
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	w0, w8, wzr, eq
```
## `unbounded_floor_to_multiple_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      6
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.67
IPC:               0.67
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsr	w8, w0, w1
 1      1     0.25                        tst	w1, #0xe0
 1      1     0.50                        lsl	w8, w8, w1
 1      1     0.25                        csel	w0, w8, wzr, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   1.00   1.00   1.00    -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	w8, w0, w1
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w1, #0xe0
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	w8, w8, w1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	w0, w8, wzr, eq
```
## `unbounded_floor_to_multiple_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      6
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.67
IPC:               0.67
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        lsr	x8, x0, x1
 1      1     0.25                        tst	w1, #0xc0
 1      1     0.50                        lsl	x8, x8, x1
 1      1     0.25                        csel	x0, x8, xzr, eq

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   1.00   1.00   1.00    -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x8, x0, x1
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w1, #0xc0
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x0, x8, xzr, eq
```
## `unbounded_floor_to_multiple_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      16
Total Cycles:      12
Total uOps:        16

Dispatch Width:    6
uOps Per Cycle:    1.33
IPC:               1.33
Block RThroughput: 5.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        mov	x8, #-1
 1      2     1.00                        mvn	w9, w2
 1      1     0.25                        mov	x10, #9223372036854775807
 1      1     0.50                        lsl	x8, x8, x2
 1      1     0.50                        lsr	x9, x10, x9
 1      1     0.25                        and	x10, x2, #0xff
 1      1     0.50                        sxtb	w11, w2
 1      1     0.25                        tst	x10, #0x40
 1      2     1.00                        orr	x9, x8, x9
 1      1     0.25                        csel	x10, xzr, x8, ne
 1      1     0.25                        csel	x8, x8, x9, ne
 1      2     1.00                        and	x9, x10, x0
 1      1     0.25                        cmn	w11, #1
 1      2     1.00                        and	x8, x8, x1
 1      1     0.25                        csel	x0, x9, xzr, gt
 1      1     0.25                        csel	x1, x8, xzr, gt

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     5.00   5.00   4.00   6.00    -      -     4.00   7.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x8, #-1
 -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     mvn	w9, w2
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	x10, #9223372036854775807
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     lsl	x8, x8, x2
 -      -      -      -      -     1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -     lsr	x9, x10, x9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x10, x2, #0xff
 -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     sxtb	w11, w2
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	x10, #0x40
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     orr	x9, x8, x9
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x10, xzr, x8, ne
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x8, x8, x9, ne
 -      -      -      -      -      -      -      -     2.00    -      -     2.00    -      -      -      -      -      -      -      -      -      -      -     and	x9, x10, x0
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmn	w11, #1
 -      -      -      -      -     2.00    -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     and	x8, x8, x1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x0, x9, xzr, gt
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	x1, x8, xzr, gt
```
## `unbounded_is_multiple_of_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      8
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    0.88
IPC:               0.88
Block RThroughput: 1.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        orr	w8, w0, #0x100
 1      1     0.25                        and	w9, w1, #0xff
 1      1     0.25                        tst	w0, #0xff
 1      1     0.25                        rbit	w8, w8
 1      1     0.25                        clz	w8, w8
 1      1     0.25                        ccmp	w8, w9, #2, ne
 1      1     0.25                        cset	w0, hs

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   2.00   2.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     orr	w8, w0, #0x100
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w9, w1, #0xff
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w0, #0xff
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     rbit	w8, w8
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     clz	w8, w8
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ccmp	w8, w9, #2, ne
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, hs
```
## `unbounded_is_multiple_of_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      8
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    0.88
IPC:               0.88
Block RThroughput: 1.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        orr	w8, w0, #0x10000
 1      1     0.25                        tst	w0, #0xffff
 1      1     0.25                        and	w9, w1, #0xff
 1      1     0.25                        rbit	w8, w8
 1      1     0.25                        clz	w8, w8
 1      1     0.25                        ccmp	w8, w9, #2, ne
 1      1     0.25                        cset	w0, hs

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   2.00   2.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     orr	w8, w0, #0x10000
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     tst	w0, #0xffff
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w9, w1, #0xff
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     rbit	w8, w8
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     clz	w8, w8
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ccmp	w8, w9, #2, ne
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, hs
```
## `unbounded_is_multiple_of_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      7
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.86
IPC:               0.86
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        rbit	w8, w0
 1      1     0.25                        cmp	w0, #0
 1      1     0.25                        and	w9, w1, #0xff
 1      1     0.25                        clz	w8, w8
 1      1     0.25                        ccmp	w8, w9, #2, ne
 1      1     0.25                        cset	w0, hs

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   1.00   2.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     rbit	w8, w0
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	w0, #0
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w9, w1, #0xff
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     clz	w8, w8
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     ccmp	w8, w9, #2, ne
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, hs
```
## `unbounded_is_multiple_of_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      7
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.86
IPC:               0.86
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        rbit	x8, x0
 1      1     0.25                        cmp	x0, #0
 1      1     0.25                        and	x9, x1, #0xff
 1      1     0.25                        clz	x8, x8
 1      1     0.25                        ccmp	x8, x9, #2, ne
 1      1     0.25                        cset	w0, hs

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     1.00   1.00   2.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     rbit	x8, x0
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	x0, #0
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	x9, x1, #0xff
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     clz	x8, x8
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     ccmp	x8, x9, #2, ne
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, hs
```
## `unbounded_is_multiple_of_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      12
Total Cycles:      9
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.33
IPC:               1.33
Block RThroughput: 3.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        rbit	x8, x1
 1      1     0.25                        rbit	x9, x0
 1      2     1.00                        orr	x10, x0, x1
 1      1     0.25                        cmp	x0, #0
 1      1     0.25                        clz	x8, x8
 1      1     0.25                        clz	x9, x9
 1      1     0.25                        add	w8, w8, #64
 1      1     0.25                        csel	w8, w9, w8, ne
 1      1     0.25                        cmp	x10, #0
 1      1     0.25                        and	w9, w2, #0xff
 1      1     0.25                        ccmp	w8, w9, #2, ne
 1      1     0.25                        cset	w0, hs

Resources:

Resource pressure per iteration:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] 
 -      -      -      -      -     3.00   3.00   3.00   4.00    -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0.0]  [0.1]  [1]    [2.0]  [2.1]  [3.0]  [3.1]  [3.2]  [3.3]  [4]    [5]    [6.0]  [6.1]  [7]    [8.0]  [8.1]  [9.0]  [9.1]  [9.2]  [10]   [11]   [12.0] [12.1] Instructions:
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     rbit	x8, x1
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     rbit	x9, x0
 -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -     orr	x10, x0, x1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	x0, #0
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     clz	x8, x8
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     clz	x9, x9
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	w8, w8, #64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     csel	w8, w9, w8, ne
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	x10, #0
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	w9, w2, #0xff
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ccmp	w8, w9, #2, ne
 -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     cset	w0, hs
```
