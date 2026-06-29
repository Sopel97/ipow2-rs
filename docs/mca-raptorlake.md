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
[0]   - ADLPPort00
[1]   - ADLPPort01
[2]   - ADLPPort02
[3]   - ADLPPort03
[4]   - ADLPPort04
[5]   - ADLPPort05
[6]   - ADLPPort06
[7]   - ADLPPort07
[8]   - ADLPPort08
[9]   - ADLPPort09
[10]  - ADLPPort10
[11]  - ADLPPort11
[12]  - ADLPPortInvalid
```
# Functions:
## `ceil_to_multiple_i8_pow2`, `ceil_to_multiple_u8_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      10
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    0.90
IPC:               0.80
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        mov	r9b, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r9b, cl
 1      0     0.20                        mov	eax, r9d
 1      1     0.20                        not	al
 1      1     0.20                        add	al, r8b
 1      2     0.20                        and	al, r9b

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     2.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r9b, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r9b, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	eax, r9d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	al
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     add	al, r8b
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	al, r9b
```
## `ceil_to_multiple_i8_unb_pow2`, `ceil_to_multiple_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      11
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    0.91
IPC:               0.82
Block RThroughput: 1.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, ecx
 1      2     0.20                        and	dl, 7
 1      1     0.20                        mov	r9b, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r9b, cl
 1      0     0.20                        mov	eax, r9d
 1      1     0.20                        not	al
 1      1     0.20                        add	al, r8b
 1      2     0.20                        and	al, r9b

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   2.00    -      -      -     2.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 7
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r9b, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r9b, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, r9d
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     not	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	al, r8b
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	al, r9b
```
## `ceil_to_multiple_i16_pow2`, `ceil_to_multiple_i32_pow2`, `ceil_to_multiple_i32_unb_pow2`, `ceil_to_multiple_u16_pow2`, `ceil_to_multiple_u32_pow2`, `ceil_to_multiple_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      10
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.50
IPC:               0.50
Block RThroughput: 0.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        mov	eax, -1
 1      3     0.50                        shlx	eax, eax, edx
 1      0     0.20                        mov	edx, eax
 1      1     0.20                        sub	edx, ecx
 1      2     0.33                        andn	eax, edx, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
 -     1.00    -      -      -     1.00   1.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	eax, eax, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     sub	edx, ecx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     andn	eax, edx, eax
```
## `ceil_to_multiple_i16_unb_pow2`, `ceil_to_multiple_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      11
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.55
IPC:               0.55
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      2     0.20                        and	dl, 15
 1      1     0.20                        mov	eax, -1
 1      3     0.50                        shlx	eax, eax, edx
 1      0     0.20                        mov	edx, eax
 1      1     0.20                        sub	edx, ecx
 1      2     0.33                        andn	eax, edx, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -     1.00   1.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	dl, 15
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	eax, eax, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     sub	edx, ecx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     andn	eax, edx, eax
```
## `ceil_to_multiple_i64_pow2`, `ceil_to_multiple_i64_unb_pow2`, `ceil_to_multiple_u64_pow2`, `ceil_to_multiple_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      11
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.36
IPC:               0.45
Block RThroughput: 0.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        mov	rax, -1
 1      3     0.50                        shlx	rax, rax, rdx
 0      1     0.00                        mov	rdx, rax
 1      1     0.20                        sub	rdx, rcx
 1      2     0.33                        andn	rax, rdx, rax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
 -     1.00    -      -      -     1.00   1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	rax, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	rax, rax, rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, rax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     sub	rdx, rcx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     andn	rax, rdx, rax
```
## `ceil_to_multiple_i128_pow2`, `ceil_to_multiple_i128_unb_pow2`, `ceil_to_multiple_u128_pow2`, `ceil_to_multiple_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      14
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    0.86
IPC:               1.00
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        mov	r9, -1
 1      3     0.50                        shlx	rax, r9, r8
 1      2     0.20                        xor	r10d, r10d
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmove	r10, rax
 1      1     0.50                        cmovne	r9, rax
 0      1     0.00                        mov	r8, r9
 1      1     0.20                        not	r8
 0      1     0.00                        mov	rax, r10
 1      1     0.20                        not	rax
 1      1     0.20                        add	rax, rcx
 1      1     0.50                        adc	rdx, r8
 1      2     0.20                        and	rax, r10
 1      2     0.20                        and	rdx, r9

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   2.00    -      -      -     2.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r9, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	rax, r9, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	r10d, r10d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	r10, rax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r9, rax
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, r9
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	r8
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, r10
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	rax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     adc	rdx, r8
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	rax, r10
1.00    -      -      -      -      -      -      -      -      -      -      -      -     and	rdx, r9
```
## `checked_ceil_to_multiple_i8_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      10
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    1.10
IPC:               0.90
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      1     0.20                        mov	r8b, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r8b, cl
 1      0     0.20                        mov	edx, r8d
 1      1     0.20                        not	dl
 1      1     0.20                        add	dl, al
 2      2     1.00                        setno	al
 1      2     0.20                        and	dl, r8b

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   1.00    -      -      -     2.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8b, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r8b, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	dl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     add	dl, al
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setno	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	dl, r8b
```
## `checked_ceil_to_multiple_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      12
Total uOps:        18

Dispatch Width:    6
uOps Per Cycle:    1.50
IPC:               1.25
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 7
 1      1     0.50                        ja	.LBB34_1
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        mov	r9b, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r9b, cl
 1      0     0.20                        mov	eax, r9d
 1      1     0.20                        not	al
 1      1     0.20                        add	r8b, al
 2      2     1.00                        setno	al
 1      2     0.20                        and	r8b, r9b
 1      0     0.20                        mov	edx, r8d
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax
 1      0     0.20                        mov	edx, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   3.00    -      -      -     3.00   5.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 7
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB34_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r9b, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r9b, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, r9d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	r8b, al
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setno	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	r8b, r9b
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, r8d
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
```
## `checked_ceil_to_multiple_i16_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      11
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    0.82
IPC:               0.73
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        mov	eax, -1
 1      3     0.50                        shlx	r8d, eax, edx
 1      0     0.20                        mov	edx, r8d
 1      1     0.20                        not	edx
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        add	dx, cx
 2      2     1.00                        setno	al
 1      2     0.20                        and	edx, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -     2.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	r8d, eax, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
1.00    -      -      -      -      -      -      -      -      -      -      -      -     not	edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	dx, cx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setno	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	edx, r8d
```
## `checked_ceil_to_multiple_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      13
Total uOps:        16

Dispatch Width:    6
uOps Per Cycle:    1.23
IPC:               1.08
Block RThroughput: 2.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 15
 1      1     0.50                        ja	.LBB28_1
 1      1     0.20                        mov	eax, -1
 1      3     0.50                        shlx	edx, eax, edx
 1      0     0.20                        mov	r8d, edx
 1      1     0.20                        not	r8d
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        add	cx, r8w
 2      2     1.00                        setno	al
 1      2     0.20                        and	ecx, edx
 1      0     0.20                        mov	edx, ecx
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax
 1      0     0.20                        mov	edx, ecx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   2.00    -      -      -     3.00   4.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 15
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB28_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	edx, eax, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r8d, edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	r8d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	cx, r8w
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setno	al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	ecx, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, ecx
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, ecx
```
## `checked_ceil_to_multiple_i32_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      11
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    0.82
IPC:               0.73
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        mov	eax, -1
 1      3     0.50                        shlx	r8d, eax, edx
 1      0     0.20                        mov	edx, r8d
 1      1     0.20                        not	edx
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        add	edx, ecx
 2      2     1.00                        setno	al
 1      2     0.20                        and	edx, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -     2.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	r8d, eax, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
1.00    -      -      -      -      -      -      -      -      -      -      -      -     not	edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	edx, ecx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setno	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	edx, r8d
```
## `checked_ceil_to_multiple_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      13
Total uOps:        16

Dispatch Width:    6
uOps Per Cycle:    1.23
IPC:               1.08
Block RThroughput: 2.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 31
 1      1     0.50                        ja	.LBB30_1
 1      1     0.20                        mov	eax, -1
 1      3     0.50                        shlx	edx, eax, edx
 1      0     0.20                        mov	r8d, edx
 1      1     0.20                        not	r8d
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        add	ecx, r8d
 2      2     1.00                        setno	al
 1      2     0.20                        and	ecx, edx
 1      0     0.20                        mov	edx, ecx
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax
 1      0     0.20                        mov	edx, ecx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   2.00    -      -      -     3.00   4.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 31
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB30_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	edx, eax, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r8d, edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	r8d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	ecx, r8d
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setno	al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	ecx, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, ecx
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, ecx
```
## `checked_ceil_to_multiple_i64_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      12
Total uOps:        8

Dispatch Width:    6
uOps Per Cycle:    0.67
IPC:               0.67
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        mov	rax, -1
 1      3     0.50                        shlx	r8, rax, rdx
 0      1     0.00                        mov	rdx, r8
 1      1     0.20                        not	rdx
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        add	rdx, rcx
 2      2     1.00                        setno	al
 1      2     0.20                        and	rdx, r8

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -     1.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	rax, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	r8, rax, rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r8
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     add	rdx, rcx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setno	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	rdx, r8
```
## `checked_ceil_to_multiple_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      14
Total uOps:        13

Dispatch Width:    6
uOps Per Cycle:    0.93
IPC:               1.00
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 63
 1      1     0.50                        ja	.LBB32_1
 1      1     0.20                        mov	rax, -1
 1      3     0.50                        shlx	rdx, rax, rdx
 0      1     0.00                        mov	r8, rdx
 1      1     0.20                        not	r8
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        add	rcx, r8
 2      2     1.00                        setno	al
 1      2     0.20                        and	rcx, rdx
 0      1     0.00                        mov	rdx, rcx
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax
 0      1     0.00                        mov	rdx, rcx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   2.00    -      -      -     2.00   4.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 63
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB32_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	rax, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	rdx, rax, rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rdx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	rcx, r8
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setno	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	rcx, rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, rcx
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, rcx
```
## `checked_ceil_to_multiple_i128_pow2`
```asm
Iterations:        1
Instructions:      24
Total Cycles:      28
Total uOps:        25

Dispatch Width:    6
uOps Per Cycle:    0.89
IPC:               0.86
Block RThroughput: 4.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	rsi
 0      1     0.00                        mov	rax, rcx
 1      1     0.20                        mov	r10, -1
 1      3     0.50                        shlx	r11, r10, r9
 1      2     0.20                        xor	ecx, ecx
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	r10, r11
 0      1     0.00                        mov	r9, r11
 1      1     0.50                        cmovne	r9, rcx
 0      1     0.00                        mov	r11, r10
 1      1     0.20                        not	r11
 0      1     0.00                        mov	rsi, r9
 1      1     0.20                        not	rsi
 1      1     0.20                        add	rsi, rdx
 1      1     0.50                        adc	r11, r8
 1      1     0.50                        jo	.LBB25_2
 1      2     0.20                        and	r11, r10
 1      2     0.20                        and	rsi, r9
 2      12    0.50           *            mov	qword ptr [rax + 16], rsi
 2      12    0.50           *            mov	qword ptr [rax + 24], r11
 1      1     0.20                        mov	ecx, 1
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   2.00    -      -     2.00   3.00   3.00   2.00   3.00   3.00   3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r10, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	r11, r10, r9
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r10, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, r11
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r9, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, r10
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rsi, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	rsi
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	rsi, rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     adc	r11, r8
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jo	.LBB25_2
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	r11, r10
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	rsi, r9
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 16], rsi
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 24], r11
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, 1
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 8], 0
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rsi
```
## `checked_ceil_to_multiple_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      31
Total Cycles:      29
Total uOps:        35

Dispatch Width:    6
uOps Per Cycle:    1.21
IPC:               1.07
Block RThroughput: 5.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	rsi
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB26_4
 1      1     0.20                        mov	r10, -1
 1      3     0.50                        shlx	r11, r10, r9
 1      2     0.20                        xor	ecx, ecx
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	r10, r11
 0      1     0.00                        mov	r9, r11
 1      1     0.50                        cmovne	r9, rcx
 0      1     0.00                        mov	r11, r10
 1      1     0.20                        not	r11
 0      1     0.00                        mov	rsi, r9
 1      1     0.20                        not	rsi
 1      1     0.20                        add	rdx, rsi
 1      1     0.50                        adc	r8, r11
 1      1     0.50                        jo	.LBB26_3
 1      2     0.20                        and	r8, r10
 1      2     0.20                        and	rdx, r9
 2      12    0.50           *            mov	qword ptr [rax + 16], rdx
 2      12    0.50           *            mov	qword ptr [rax + 24], r8
 1      1     0.20                        mov	ecx, 1
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rsi
 2      7     0.50                  U     ret
 1      2     0.20                        xor	ecx, ecx
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   3.00   1.00   1.00   3.00   3.00   4.00   3.00   4.00   4.00   4.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, r9b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB26_4
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r10, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	r11, r10, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	ecx, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r10, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, r11
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r9, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, r10
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rsi, r9
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	rsi
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	rdx, rsi
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     adc	r8, r11
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jo	.LBB26_3
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	r8, r10
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	rdx, r9
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 16], rdx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 24], r8
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, 1
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 8], 0
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rsi
1.00    -      -     1.00    -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 8], 0
 -      -     1.00    -      -      -      -      -      -      -      -      -      -     pop	rsi
```
## `checked_ceil_to_multiple_u8_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      11
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.09
IPC:               0.91
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      1     0.20                        mov	r8b, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r8b, cl
 1      0     0.20                        mov	edx, r8d
 1      1     0.20                        not	dl
 1      1     0.20                        add	dl, al
 1      1     0.20                        cmp	dl, al
 2      2     1.00                        setae	al
 1      2     0.20                        and	dl, r8b

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   2.00    -      -      -     2.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8b, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r8b, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	dl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     add	dl, al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	dl, al
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setae	al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	dl, r8b
```
## `checked_ceil_to_multiple_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      13
Total uOps:        17

Dispatch Width:    6
uOps Per Cycle:    1.31
IPC:               1.08
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 7
 1      1     0.50                        ja	.LBB44_1
 1      0     0.20                        mov	eax, ecx
 1      1     0.20                        mov	r8b, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r8b, cl
 1      0     0.20                        mov	edx, r8d
 1      1     0.20                        not	dl
 1      1     0.20                        add	dl, al
 1      1     0.20                        cmp	dl, al
 2      2     1.00                        setae	al
 1      2     0.20                        and	dl, r8b
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   2.00    -      -      -     3.00   4.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 7
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB44_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8b, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r8b, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	edx, r8d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	dl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	dl, al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, al
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setae	al
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, r8b
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
```
## `checked_ceil_to_multiple_u16_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      12
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.92
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        mov	eax, -1
 1      3     0.50                        shlx	edx, eax, edx
 1      0     0.20                        mov	r8d, edx
 1      1     0.20                        not	r8d
 1      1     0.20                        add	r8d, ecx
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	r8w, cx
 2      2     1.00                        setae	al
 1      0     0.20                        mov	r8d, edx
 1      1     0.20                        sub	r8d, ecx
 1      2     0.33                        andn	edx, r8d, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   2.00    -      -      -     2.00   4.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	edx, eax, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r8d, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     not	r8d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	r8d, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmp	r8w, cx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setae	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sub	r8d, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     andn	edx, r8d, edx
```
## `checked_ceil_to_multiple_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      14
Total uOps:        17

Dispatch Width:    6
uOps Per Cycle:    1.21
IPC:               1.07
Block RThroughput: 2.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 15
 1      1     0.50                        ja	.LBB38_1
 1      1     0.20                        mov	eax, -1
 1      3     0.50                        shlx	edx, eax, edx
 1      0     0.20                        mov	r8d, edx
 1      1     0.20                        not	r8d
 1      1     0.20                        add	r8d, ecx
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	r8w, cx
 2      2     1.00                        setae	al
 1      0     0.20                        mov	r8d, edx
 1      1     0.20                        sub	r8d, ecx
 1      2     0.33                        andn	edx, r8d, edx
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   2.00    -      -      -     3.00   5.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 15
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB38_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	edx, eax, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r8d, edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	r8d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	r8d, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	r8w, cx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setae	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8d, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sub	r8d, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     andn	edx, r8d, edx
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
```
## `checked_ceil_to_multiple_u32_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      12
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.92
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        mov	eax, -1
 1      3     0.50                        shlx	edx, eax, edx
 1      0     0.20                        mov	r8d, edx
 1      1     0.20                        not	r8d
 1      1     0.20                        add	r8d, ecx
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	r8d, ecx
 2      2     1.00                        setae	al
 1      0     0.20                        mov	r8d, edx
 1      1     0.20                        sub	r8d, ecx
 1      2     0.33                        andn	edx, r8d, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   2.00    -      -      -     2.00   4.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	edx, eax, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r8d, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     not	r8d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	r8d, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmp	r8d, ecx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setae	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sub	r8d, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     andn	edx, r8d, edx
```
## `checked_ceil_to_multiple_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      14
Total uOps:        17

Dispatch Width:    6
uOps Per Cycle:    1.21
IPC:               1.07
Block RThroughput: 2.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 31
 1      1     0.50                        ja	.LBB40_1
 1      1     0.20                        mov	eax, -1
 1      3     0.50                        shlx	edx, eax, edx
 1      0     0.20                        mov	r8d, edx
 1      1     0.20                        not	r8d
 1      1     0.20                        add	r8d, ecx
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	r8d, ecx
 2      2     1.00                        setae	al
 1      0     0.20                        mov	r8d, edx
 1      1     0.20                        sub	r8d, ecx
 1      2     0.33                        andn	edx, r8d, edx
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   2.00    -      -      -     3.00   5.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 31
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB40_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	edx, eax, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r8d, edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	r8d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	r8d, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	r8d, ecx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setae	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8d, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sub	r8d, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     andn	edx, r8d, edx
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
```
## `checked_ceil_to_multiple_u64_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      13
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    0.77
IPC:               0.85
Block RThroughput: 1.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        mov	rax, -1
 1      3     0.50                        shlx	rdx, rax, rdx
 0      1     0.00                        mov	r8, rdx
 1      1     0.20                        not	r8
 1      1     0.20                        add	r8, rcx
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	r8, rcx
 2      2     1.00                        setae	al
 0      1     0.00                        mov	r8, rdx
 1      1     0.20                        sub	r8, rcx
 1      2     0.33                        andn	rdx, r8, rdx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   1.00    -      -      -     2.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	rax, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	rdx, rax, rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rdx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	r8
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	r8, rcx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	r8, rcx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setae	al
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rdx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sub	r8, rcx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     andn	rdx, r8, rdx
```
## `checked_ceil_to_multiple_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      15
Total uOps:        15

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 63
 1      1     0.50                        ja	.LBB42_1
 1      1     0.20                        mov	rax, -1
 1      3     0.50                        shlx	rdx, rax, rdx
 0      1     0.00                        mov	r8, rdx
 1      1     0.20                        not	r8
 1      1     0.20                        add	r8, rcx
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	r8, rcx
 2      2     1.00                        setae	al
 0      1     0.00                        mov	r8, rdx
 1      1     0.20                        sub	r8, rcx
 1      2     0.33                        andn	rdx, r8, rdx
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   2.00    -      -      -     2.00   4.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 63
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB42_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	rax, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	rdx, rax, rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rdx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	r8, rcx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmp	r8, rcx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setae	al
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rdx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     sub	r8, rcx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     andn	rdx, r8, rdx
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
```
## `checked_ceil_to_multiple_u128_pow2`
```asm
Iterations:        1
Instructions:      24
Total Cycles:      28
Total uOps:        25

Dispatch Width:    6
uOps Per Cycle:    0.89
IPC:               0.86
Block RThroughput: 4.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	rsi
 0      1     0.00                        mov	rax, rcx
 1      1     0.20                        mov	r10, -1
 1      3     0.50                        shlx	r11, r10, r9
 1      2     0.20                        xor	ecx, ecx
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	r10, r11
 0      1     0.00                        mov	r9, r11
 1      1     0.50                        cmovne	r9, rcx
 0      1     0.00                        mov	r11, r10
 1      1     0.20                        not	r11
 0      1     0.00                        mov	rsi, r9
 1      1     0.20                        not	rsi
 1      1     0.20                        add	rsi, rdx
 1      1     0.50                        adc	r11, r8
 1      1     0.50                        jb	.LBB35_2
 1      2     0.20                        and	r11, r10
 1      2     0.20                        and	rsi, r9
 2      12    0.50           *            mov	qword ptr [rax + 16], rsi
 2      12    0.50           *            mov	qword ptr [rax + 24], r11
 1      1     0.20                        mov	ecx, 1
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   2.00    -      -     2.00   3.00   3.00   2.00   3.00   3.00   3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r10, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	r11, r10, r9
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r10, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, r11
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r9, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, r10
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rsi, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	rsi
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	rsi, rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     adc	r11, r8
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jb	.LBB35_2
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	r11, r10
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	rsi, r9
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 16], rsi
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 24], r11
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, 1
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 8], 0
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rsi
```
## `checked_ceil_to_multiple_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      31
Total Cycles:      29
Total uOps:        35

Dispatch Width:    6
uOps Per Cycle:    1.21
IPC:               1.07
Block RThroughput: 5.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	rsi
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB36_4
 1      1     0.20                        mov	r10, -1
 1      3     0.50                        shlx	r11, r10, r9
 1      2     0.20                        xor	ecx, ecx
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	r10, r11
 0      1     0.00                        mov	r9, r11
 1      1     0.50                        cmovne	r9, rcx
 0      1     0.00                        mov	r11, r10
 1      1     0.20                        not	r11
 0      1     0.00                        mov	rsi, r9
 1      1     0.20                        not	rsi
 1      1     0.20                        add	rdx, rsi
 1      1     0.50                        adc	r8, r11
 1      1     0.50                        jb	.LBB36_3
 1      2     0.20                        and	r8, r10
 1      2     0.20                        and	rdx, r9
 2      12    0.50           *            mov	qword ptr [rax + 16], rdx
 2      12    0.50           *            mov	qword ptr [rax + 24], r8
 1      1     0.20                        mov	ecx, 1
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rsi
 2      7     0.50                  U     ret
 1      2     0.20                        xor	ecx, ecx
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   3.00   1.00   1.00   3.00   3.00   4.00   3.00   4.00   4.00   4.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, r9b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB36_4
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r10, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	r11, r10, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	ecx, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r10, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, r11
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r9, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, r10
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rsi, r9
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	rsi
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	rdx, rsi
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     adc	r8, r11
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jb	.LBB36_3
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	r8, r10
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	rdx, r9
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 16], rdx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 24], r8
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, 1
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 8], 0
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rsi
1.00    -      -     1.00    -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 8], 0
 -      -     1.00    -      -      -      -      -      -      -      -      -      -     pop	rsi
```
## `checked_div_ceil_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      11
Total uOps:        18

Dispatch Width:    6
uOps Per Cycle:    1.64
IPC:               1.36
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, edx
 1      1     0.20                        cmp	r8b, 7
 1      1     0.50                        ja	.LBB49_2
 1      0     0.20                        mov	eax, ecx
 1      1     0.20                        mov	r9b, -1
 1      0     0.20                        mov	ecx, r8d
 2      2     1.00                        shl	r9b, cl
 1      1     0.20                        not	r9b
 1      0     0.20                        mov	edx, eax
 2      2     1.00                        sar	dl, cl
 1      2     0.20                        and	r9b, al
 1      1     0.20                        cmp	r9b, 1
 1      1     0.50                        sbb	dl, -1
 1      1     0.20                        cmp	r8b, 8
 2      2     1.00                        setb	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   2.00    -      -      -     3.00   5.00    -      -      -     3.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	r8b, 7
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB49_2
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	eax, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r9b, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r9b, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	r9b
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	edx, eax
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sar	dl, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	r9b, al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	r9b, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sbb	dl, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	r8b, 8
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setb	al
```
## `checked_div_ceil_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      12
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.92
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 15
 1      1     0.50                        ja	.LBB46_1
 1      1     0.33                        movsx	eax, cx
 1      0     0.20                        mov	ecx, edx
 1      3     0.50                        sarx	edx, eax, edx
 1      3     1.00                        bzhi	eax, eax, ecx
 1      1     0.20                        cmp	ax, 1
 1      1     0.50                        sbb	dx, -1
 1      1     0.20                        mov	ax, 1
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   2.00    -      -      -     2.00   3.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 15
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB46_1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movsx	eax, cx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	edx, eax, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	eax, eax, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	ax, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sbb	dx, -1
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ax, 1
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
```
## `checked_div_ceil_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      11
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.91
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 31
 1      1     0.50                        ja	.LBB47_1
 1      0     0.20                        mov	eax, edx
 1      3     0.50                        sarx	edx, ecx, edx
 1      3     1.00                        bzhi	eax, ecx, eax
 1      1     0.20                        cmp	eax, 1
 1      1     0.50                        sbb	edx, -1
 1      1     0.20                        mov	eax, 1
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     2.00   3.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 31
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB47_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sarx	edx, ecx, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	eax, ecx, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	eax, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sbb	edx, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	eax, 1
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
```
## `checked_div_ceil_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      11
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    0.91
IPC:               0.91
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 63
 1      1     0.50                        ja	.LBB48_1
 0      1     0.00                        mov	rax, rdx
 1      3     0.50                        sarx	rdx, rcx, rdx
 1      3     1.00                        bzhi	rax, rcx, rax
 1      1     0.20                        cmp	rax, 1
 1      1     0.50                        sbb	rdx, -1
 1      1     0.20                        mov	eax, 1
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     2.00   2.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 63
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB48_1
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	rdx, rcx, rdx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	rax, rcx, rax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	rax, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sbb	rdx, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	eax, 1
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
```
## `checked_div_ceil_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      35
Total Cycles:      30
Total uOps:        40

Dispatch Width:    6
uOps Per Cycle:    1.33
IPC:               1.17
Block RThroughput: 6.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	rsi
 2      1     0.50           *            push	rdi
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB45_1
 1      1     0.20                        mov	r10, -1
 1      3     0.50                        shlx	r11, r10, r9
 0      1     0.00                        mov	rsi, rdx
 1      0     0.20                        mov	ecx, r9d
 3      5     1.00                        shrd	rsi, r8, cl
 0      1     0.00                        mov	rcx, r8
 1      1     0.50                        sar	rcx, 63
 1      2     0.20                        xor	edi, edi
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmove	rdi, r11
 1      1     0.50                        cmovne	r10, r11
 1      3     0.50                        sarx	r9, r8, r9
 1      1     0.50                        cmovne	rsi, r9
 1      1     0.50                        cmove	rcx, r9
 1      2     0.33                        andn	r8, r10, r8
 1      2     0.33                        andn	rdx, rdi, rdx
 1      2     0.20                        xor	r9d, r9d
 1      2     0.20                        or	rdx, r8
 2      2     1.00                        setne	r9b
 1      1     0.20                        add	r9, rsi
 1      1     0.50                        adc	rcx, 0
 2      12    0.50           *            mov	qword ptr [rax + 16], r9
 2      12    0.50           *            mov	qword ptr [rax + 24], rcx
 1      1     0.20                        mov	ecx, 1
 0      0     0.00                        jmp	.LBB45_3
 1      2     0.20                        xor	ecx, ecx
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rdi
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
6.00   4.00    -     1.00   3.00   4.00   7.00   3.00   3.00   3.00   5.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rsi
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     push	rdi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, r9b
1.00    -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB45_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r10, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	r11, r10, r9
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rsi, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, r9d
1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -     shrd	rsi, r8, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rcx, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sar	rcx, 63
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	edi, edi
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	rdi, r11
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r10, r11
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sarx	r9, r8, r9
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rsi, r9
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	rcx, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     andn	r8, r10, r8
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     andn	rdx, rdi, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	r9d, r9d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     or	rdx, r8
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setne	r9b
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	r9, rsi
1.00    -      -      -      -      -      -      -      -      -      -      -      -     adc	rcx, 0
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 16], r9
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 24], rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, 1
 -      -      -      -      -      -      -      -      -      -      -      -      -     jmp	.LBB45_3
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	ecx, ecx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 8], 0
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rdi
 -      -      -     1.00    -      -      -      -      -      -      -      -      -     pop	rsi
```
## `checked_div_ceil_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      11
Total uOps:        18

Dispatch Width:    6
uOps Per Cycle:    1.64
IPC:               1.36
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, edx
 1      1     0.20                        cmp	r8b, 7
 1      1     0.50                        ja	.LBB54_2
 1      0     0.20                        mov	eax, ecx
 1      1     0.20                        mov	r9b, -1
 1      0     0.20                        mov	ecx, r8d
 2      2     1.00                        shl	r9b, cl
 1      1     0.20                        not	r9b
 1      0     0.20                        mov	edx, eax
 2      2     1.00                        shr	dl, cl
 1      2     0.20                        and	r9b, al
 1      1     0.20                        cmp	r9b, 1
 1      1     0.50                        sbb	dl, -1
 1      1     0.20                        cmp	r8b, 8
 2      2     1.00                        setb	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   2.00    -      -      -     3.00   5.00    -      -      -     3.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	r8b, 7
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB54_2
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	eax, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r9b, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r9b, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	r9b
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	edx, eax
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	dl, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	r9b, al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	r9b, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sbb	dl, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	r8b, 8
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setb	al
```
## `checked_div_ceil_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      12
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.92
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 15
 1      1     0.50                        ja	.LBB51_1
 1      1     0.20                        movzx	eax, cx
 1      0     0.20                        mov	ecx, edx
 1      3     0.50                        shrx	edx, eax, edx
 1      3     1.00                        bzhi	eax, eax, ecx
 1      1     0.20                        cmp	ax, 1
 1      1     0.50                        sbb	dx, -1
 1      1     0.20                        mov	ax, 1
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   2.00    -      -      -     2.00   3.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 15
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB51_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     movzx	eax, cx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shrx	edx, eax, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	eax, eax, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	ax, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sbb	dx, -1
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ax, 1
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
```
## `checked_div_ceil_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      11
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.91
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 31
 1      1     0.50                        ja	.LBB52_1
 1      0     0.20                        mov	eax, edx
 1      3     0.50                        shrx	edx, ecx, edx
 1      3     1.00                        bzhi	eax, ecx, eax
 1      1     0.20                        cmp	eax, 1
 1      1     0.50                        sbb	edx, -1
 1      1     0.20                        mov	eax, 1
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     2.00   3.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 31
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB52_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shrx	edx, ecx, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	eax, ecx, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	eax, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sbb	edx, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	eax, 1
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
```
## `checked_div_ceil_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      11
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    0.91
IPC:               0.91
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 63
 1      1     0.50                        ja	.LBB53_1
 0      1     0.00                        mov	rax, rdx
 1      3     0.50                        shrx	rdx, rcx, rdx
 1      3     1.00                        bzhi	rax, rcx, rax
 1      1     0.20                        cmp	rax, 1
 1      1     0.50                        sbb	rdx, -1
 1      1     0.20                        mov	eax, 1
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     2.00   2.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 63
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB53_1
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shrx	rdx, rcx, rdx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	rax, rcx, rax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	rax, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sbb	rdx, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	eax, 1
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
```
## `checked_div_ceil_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      34
Total Cycles:      30
Total uOps:        43

Dispatch Width:    6
uOps Per Cycle:    1.43
IPC:               1.13
Block RThroughput: 7.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	rsi
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB50_1
 1      1     0.20                        mov	r10, -1
 1      3     0.50                        shlx	r11, r10, r9
 0      1     0.00                        mov	rsi, rdx
 1      0     0.20                        mov	ecx, r9d
 3      5     1.00                        shrd	rsi, r8, cl
 1      2     0.20                        xor	ecx, ecx
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	r10, r11
 1      1     0.50                        cmovne	r11, rcx
 1      3     0.50                        shrx	r9, r8, r9
 1      1     0.50                        cmovne	rsi, r9
 1      1     0.50                        cmovne	r9, rcx
 1      2     0.33                        andn	rcx, r10, r8
 1      2     0.33                        andn	rdx, r11, rdx
 1      2     0.20                        xor	r8d, r8d
 1      2     0.20                        or	rdx, rcx
 2      2     1.00                        setne	r8b
 1      1     0.20                        add	r8, rsi
 1      1     0.50                        adc	r9, 0
 2      12    0.50           *            mov	qword ptr [rax + 16], r8
 2      12    0.50           *            mov	qword ptr [rax + 24], r9
 1      1     0.20                        mov	ecx, 1
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rsi
 2      7     0.50                  U     ret
 1      2     0.20                        xor	ecx, ecx
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
6.00   4.00   1.00   1.00   3.00   4.00   7.00   3.00   4.00   4.00   5.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, r9b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB50_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r10, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	r11, r10, r9
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rsi, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, r9d
1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -     shrd	rsi, r8, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r10, r11
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r11, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shrx	r9, r8, r9
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rsi, r9
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r9, rcx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     andn	rcx, r10, r8
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     andn	rdx, r11, rdx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	r8d, r8d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     or	rdx, rcx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setne	r8b
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	r8, rsi
1.00    -      -      -      -      -      -      -      -      -      -      -      -     adc	r9, 0
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 16], r8
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 24], r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, 1
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 8], 0
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rsi
 -      -      -     1.00    -      -     1.00    -      -      -      -      -      -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 8], 0
 -      -     1.00    -      -      -      -      -      -      -      -      -      -     pop	rsi
```
## `checked_div_floor_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      6
Total uOps:        8

Dispatch Width:    6
uOps Per Cycle:    1.33
IPC:               1.00
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        cmp	dl, 8
 2      2     1.00                        setb	al
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        sar	r8b, cl
 1      0     0.20                        mov	edx, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   3.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 8
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sar	r8b, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
```
## `checked_div_floor_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      7
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.86
IPC:               0.71
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.33                        movsx	ecx, cx
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 16
 2      2     1.00                        setb	al
 1      3     0.50                        sarx	edx, ecx, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00    -      -      -      -     1.00   2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movsx	ecx, cx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	dl, 16
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	edx, ecx, edx
```
## `checked_div_floor_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      6
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.83
IPC:               0.67
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 32
 2      2     1.00                        setb	al
 1      3     0.50                        sarx	edx, ecx, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00    -      -      -      -      -     3.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 32
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setb	al
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sarx	edx, ecx, edx
```
## `checked_div_floor_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      6
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.83
IPC:               0.67
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 64
 2      2     1.00                        setb	al
 1      3     0.50                        sarx	rdx, rcx, rdx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00    -      -      -      -      -     3.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 64
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setb	al
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sarx	rdx, rcx, rdx
```
## `checked_div_floor_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      19
Total Cycles:      23
Total uOps:        27

Dispatch Width:    6
uOps Per Cycle:    1.17
IPC:               0.83
Block RThroughput: 4.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB55_1
 1      0     0.20                        mov	ecx, r9d
 3      5     1.00                        shrd	rdx, r8, cl
 1      3     0.50                        sarx	rcx, r8, r9
 1      1     0.50                        sar	r8, 63
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	rdx, rcx
 1      1     0.50                        cmove	r8, rcx
 2      12    0.50           *            mov	qword ptr [rax + 24], r8
 2      12    0.50           *            mov	qword ptr [rax + 16], rdx
 1      1     0.20                        mov	ecx, 1
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 2      7     0.50                  U     ret
 1      2     0.20                        xor	ecx, ecx
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   2.00    -      -     3.00   2.00   4.00   3.00   3.00   3.00   2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, r9b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB55_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, r9d
1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -     shrd	rdx, r8, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	rcx, r8, r9
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	r8, 63
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rdx, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	r8, rcx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 24], r8
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 16], rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, 1
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 8], 0
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 8], 0
```
## `checked_div_floor_u8_unb_pow2`, `checked_div_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      6
Total uOps:        8

Dispatch Width:    6
uOps Per Cycle:    1.33
IPC:               1.00
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        cmp	dl, 8
 2      2     1.00                        setb	al
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shr	r8b, cl
 1      0     0.20                        mov	edx, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   3.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 8
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	r8b, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
```
## `checked_div_floor_u16_unb_pow2`, `checked_div_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      7
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.86
IPC:               0.71
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        movzx	ecx, cx
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 16
 2      2     1.00                        setb	al
 1      3     0.50                        shrx	edx, ecx, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00    -      -      -      -     1.00   2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	ecx, cx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	dl, 16
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shrx	edx, ecx, edx
```
## `checked_div_floor_u32_unb_pow2`, `checked_div_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      6
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.83
IPC:               0.67
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 32
 2      2     1.00                        setb	al
 1      3     0.50                        shrx	edx, ecx, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00    -      -      -      -      -     3.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 32
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setb	al
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shrx	edx, ecx, edx
```
## `checked_div_floor_u64_unb_pow2`, `checked_div_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      6
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.83
IPC:               0.67
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 64
 2      2     1.00                        setb	al
 1      3     0.50                        shrx	rdx, rcx, rdx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00    -      -      -      -      -     3.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 64
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setb	al
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shrx	rdx, rcx, rdx
```
## `checked_div_floor_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      19
Total Cycles:      23
Total uOps:        27

Dispatch Width:    6
uOps Per Cycle:    1.17
IPC:               0.83
Block RThroughput: 4.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB60_1
 1      0     0.20                        mov	ecx, r9d
 3      5     1.00                        shrd	rdx, r8, cl
 1      3     0.50                        shrx	rcx, r8, r9
 1      2     0.20                        xor	r8d, r8d
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	rdx, rcx
 1      1     0.50                        cmove	r8, rcx
 2      12    0.50           *            mov	qword ptr [rax + 24], r8
 2      12    0.50           *            mov	qword ptr [rax + 16], rdx
 1      1     0.20                        mov	ecx, 1
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 2      7     0.50                  U     ret
 1      2     0.20                        xor	ecx, ecx
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   2.00    -      -     3.00   2.00   4.00   3.00   3.00   3.00   3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, r9b
1.00    -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB60_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, r9d
1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -     shrd	rdx, r8, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shrx	rcx, r8, r9
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	r8d, r8d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rdx, rcx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	r8, rcx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 24], r8
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 16], rdx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, 1
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 8], 0
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	ecx, ecx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 8], 0
```
## `checked_div_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      12
Total uOps:        18

Dispatch Width:    6
uOps Per Cycle:    1.50
IPC:               1.25
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, edx
 1      1     0.20                        cmp	al, 7
 1      1     0.50                        ja	.LBB69_2
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        mov	r9b, -1
 1      0     0.20                        mov	ecx, eax
 2      2     1.00                        shl	r9b, cl
 1      1     0.20                        not	r9b
 1      0     0.20                        mov	edx, r8d
 1      1     0.50                        sar	dl, 7
 1      2     0.20                        and	dl, r9b
 1      1     0.20                        add	dl, r8b
 2      2     1.00                        sar	dl, cl
 1      1     0.20                        cmp	al, 8
 2      2     1.00                        setb	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   2.00    -      -      -     3.00   5.00    -      -      -     3.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	al, 7
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB69_2
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8d, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r9b, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, eax
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r9b, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	r9b
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	edx, r8d
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sar	dl, 7
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	dl, r9b
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	dl, r8b
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sar	dl, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	al, 8
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setb	al
```
## `checked_div_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      13
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    0.92
IPC:               0.85
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 15
 1      1     0.50                        ja	.LBB66_1
 1      1     0.33                        movsx	eax, cx
 1      1     0.50                        sar	eax, 15
 1      3     1.00                        bzhi	eax, eax, edx
 1      1     0.20                        add	eax, ecx
 1      1     0.50                        cwde
 1      3     0.50                        sarx	edx, eax, edx
 1      1     0.20                        mov	ax, 1
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   2.00    -      -      -     2.00   2.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 15
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB66_1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movsx	eax, cx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	eax, 15
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	eax, eax, edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	eax, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cwde
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sarx	edx, eax, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ax, 1
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
```
## `checked_div_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      11
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.91
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 31
 1      1     0.50                        ja	.LBB67_1
 1      0     0.20                        mov	eax, ecx
 1      1     0.50                        sar	eax, 31
 1      3     1.00                        bzhi	eax, eax, edx
 1      1     0.20                        add	eax, ecx
 1      3     0.50                        sarx	edx, eax, edx
 1      1     0.20                        mov	eax, 1
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     2.00   3.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 31
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB67_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, ecx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	eax, 31
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	eax, eax, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	edx, eax, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	eax, 1
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
```
## `checked_div_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      12
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    0.83
IPC:               0.83
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 63
 1      1     0.50                        ja	.LBB68_1
 0      1     0.00                        mov	rax, rcx
 1      1     0.50                        sar	rax, 63
 1      3     1.00                        bzhi	rax, rax, rdx
 1      1     0.20                        add	rax, rcx
 1      3     0.50                        sarx	rdx, rax, rdx
 1      1     0.20                        mov	eax, 1
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     2.00   2.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 63
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB68_1
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, 63
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	rax, rax, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	rax, rcx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sarx	rdx, rax, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	eax, 1
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
```
## `checked_div_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      36
Total Cycles:      32
Total uOps:        43

Dispatch Width:    6
uOps Per Cycle:    1.34
IPC:               1.13
Block RThroughput: 7.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	rsi
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB65_1
 1      1     0.20                        mov	rcx, -1
 1      3     0.50                        shlx	r10, rcx, r9
 1      2     0.20                        xor	r11d, r11d
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmove	r11, r10
 1      1     0.50                        cmovne	rcx, r10
 0      1     0.00                        mov	r10, r8
 1      1     0.50                        sar	r10, 63
 1      2     0.33                        andn	rsi, rcx, r10
 1      2     0.33                        andn	r10, r11, r10
 1      1     0.20                        add	r10, rdx
 1      1     0.50                        adc	rsi, r8
 0      1     0.00                        mov	rdx, rsi
 1      1     0.50                        sar	rdx, 63
 1      2     0.20                        test	r9b, 64
 1      3     0.50                        sarx	r8, rsi, r9
 1      1     0.50                        cmove	rdx, r8
 1      0     0.20                        mov	ecx, r9d
 3      5     1.00                        shrd	r10, rsi, cl
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	r10, r8
 2      12    0.50           *            mov	qword ptr [rax + 24], rdx
 2      12    0.50           *            mov	qword ptr [rax + 16], r10
 1      1     0.20                        mov	ecx, 1
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rsi
 2      7     0.50                  U     ret
 1      2     0.20                        xor	ecx, ecx
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
6.00   4.00   1.00   1.00   3.00   4.00   7.00   3.00   4.00   4.00   5.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, r9b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB65_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	rcx, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	r10, rcx, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	r11d, r11d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	r11, r10
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rcx, r10
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, r8
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	r10, 63
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     andn	rsi, rcx, r10
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     andn	r10, r11, r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	r10, rdx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     adc	rsi, r8
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, rsi
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sar	rdx, 63
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	r8, rsi, r9
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	rdx, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, r9d
1.00   1.00    -      -      -      -      -      -      -      -     1.00    -      -     shrd	r10, rsi, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r10, r8
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 24], rdx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 16], r10
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, 1
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 8], 0
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rsi
 -      -      -     1.00    -      -     1.00    -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	ecx, ecx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 8], 0
 -      -     1.00    -      -      -      -      -      -      -      -      -      -     pop	rsi
```
## `checked_div_round_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      23
Total Cycles:      14
Total uOps:        26

Dispatch Width:    6
uOps Per Cycle:    1.86
IPC:               1.64
Block RThroughput: 5.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, edx
 1      1     0.20                        cmp	al, 7
 1      1     0.50                        ja	.LBB74_2
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        mov	r9b, -1
 1      0     0.20                        mov	ecx, eax
 2      2     1.00                        shl	r9b, cl
 1      0     0.20                        mov	edx, r8d
 2      2     1.00                        sar	dl, cl
 1      1     0.20                        not	r9b
 1      2     0.20                        and	r9b, r8b
 1      1     0.50                        shr	r8b, 7
 1      2     0.20                        xor	ecx, ecx
 1      1     0.20                        sub	r9b, r8b
 1      0     0.20                        movzx	r8d, r9b
 1      1     0.50                        cmovb	r8d, ecx
 1      1     0.20                        add	r8b, r8b
 1      0     0.20                        movzx	ecx, r8b
 1      0     0.20                        movzx	r8d, al
 1      1     1.00                        bt	ecx, r8d
 1      1     0.50                        adc	dl, 0
 1      1     0.20                        cmp	al, 8
 2      2     1.00                        setb	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   5.00    -      -      -     4.00   7.00    -      -      -     5.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	al, 7
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB74_2
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8d, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r9b, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, eax
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r9b, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	edx, r8d
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sar	dl, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	r9b
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	r9b, r8b
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r8b, 7
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     sub	r9b, r8b
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movzx	r8d, r9b
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovb	r8d, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	r8b, r8b
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	ecx, r8b
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	r8d, al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bt	ecx, r8d
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     adc	dl, 0
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	al, 8
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setb	al
```
## `checked_div_round_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      17
Total Cycles:      12
Total uOps:        18

Dispatch Width:    6
uOps Per Cycle:    1.50
IPC:               1.42
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 15
 1      1     0.50                        ja	.LBB71_1
 1      1     0.33                        movsx	ecx, cx
 1      0     0.20                        mov	r8d, edx
 1      3     0.50                        sarx	edx, ecx, edx
 1      1     0.20                        movzx	r9d, cx
 1      3     1.00                        bzhi	ecx, ecx, r8d
 1      1     0.50                        shr	r9d, 15
 1      1     0.20                        sub	cx, r9w
 1      1     0.50                        cmovb	ecx, eax
 1      1     0.20                        add	ecx, ecx
 1      0     0.20                        movzx	eax, r8b
 1      1     1.00                        bt	ecx, eax
 1      1     0.50                        adc	dx, 0
 1      1     0.20                        mov	ax, 1
 2      7     0.50                  U     ret

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   4.00    -      -      -     3.00   4.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 15
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB71_1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movsx	ecx, cx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r8d, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	edx, ecx, edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	r9d, cx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	ecx, ecx, r8d
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r9d, 15
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     sub	cx, r9w
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovb	ecx, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	ecx, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movzx	eax, r8b
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bt	ecx, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     adc	dx, 0
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ax, 1
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
```
## `checked_div_round_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      16
Total Cycles:      12
Total uOps:        17

Dispatch Width:    6
uOps Per Cycle:    1.42
IPC:               1.33
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 31
 1      1     0.50                        ja	.LBB72_1
 1      0     0.20                        mov	eax, edx
 1      3     0.50                        sarx	edx, ecx, edx
 1      3     1.00                        bzhi	r8d, ecx, eax
 1      1     0.50                        shr	ecx, 31
 1      2     0.20                        xor	r9d, r9d
 1      1     0.20                        sub	r8d, ecx
 1      1     0.50                        cmovb	r8d, r9d
 1      1     0.20                        add	r8d, r8d
 1      0     0.20                        movzx	eax, al
 1      1     1.00                        bt	r8d, eax
 1      1     0.50                        adc	edx, 0
 1      1     0.20                        mov	eax, 1
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   3.00    -      -      -     2.00   4.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 31
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB72_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sarx	edx, ecx, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	r8d, ecx, eax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	ecx, 31
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	r9d, r9d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     sub	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovb	r8d, r9d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	r8d, r8d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	eax, al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bt	r8d, eax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     adc	edx, 0
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	eax, 1
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
1.00    -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
```
## `checked_div_round_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      16
Total Cycles:      14
Total uOps:        16

Dispatch Width:    6
uOps Per Cycle:    1.14
IPC:               1.14
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 63
 1      1     0.50                        ja	.LBB73_1
 0      1     0.00                        mov	rax, rdx
 1      3     0.50                        sarx	rdx, rcx, rdx
 1      3     1.00                        bzhi	r8, rcx, rax
 1      1     0.50                        shr	rcx, 63
 1      2     0.20                        xor	r9d, r9d
 1      1     0.20                        sub	r8, rcx
 1      1     0.50                        cmovae	r9, r8
 1      1     0.20                        add	r9, r9
 1      0     0.20                        movzx	eax, al
 1      3     1.00                        bt	r9, rax
 1      1     0.50                        adc	rdx, 0
 1      1     0.20                        mov	eax, 1
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   3.00    -      -      -     3.00   3.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 63
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB73_1
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	rdx, rcx, rdx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	r8, rcx, rax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	rcx, 63
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	r9d, r9d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     sub	r8, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovae	r9, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	r9, r9
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	eax, al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bt	r9, rax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     adc	rdx, 0
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	eax, 1
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
```
## `checked_div_round_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      47
Total Cycles:      40
Total uOps:        56

Dispatch Width:    6
uOps Per Cycle:    1.40
IPC:               1.18
Block RThroughput: 9.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	rsi
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB70_1
 1      1     0.20                        mov	rcx, -1
 1      3     0.50                        shlx	r10, rcx, r9
 1      2     0.20                        xor	r11d, r11d
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	rcx, r10
 1      1     0.50                        cmovne	r10, r11
 1      2     0.33                        andn	rsi, rcx, r8
 1      2     0.33                        andn	r10, r10, rdx
 0      1     0.00                        mov	rcx, r8
 1      1     0.50                        shr	rcx, 63
 1      1     0.20                        sub	r10, rcx
 1      1     0.50                        sbb	rsi, 0
 1      1     0.50                        cmovb	rsi, r11
 1      1     0.50                        cmovb	r10, r11
 1      3     1.00                        shld	rsi, r10, 1
 1      1     0.20                        add	r10, r10
 1      0     0.20                        mov	ecx, r9d
 3      5     1.00                        shrd	r10, rsi, cl
 1      2     0.20                        test	r9b, 64
 1      3     0.50                        shrx	r11, rsi, r9
 1      1     0.50                        cmove	r11, r10
 0      1     0.00                        mov	r10, r8
 1      1     0.50                        sar	r10, 63
 1      2     0.20                        test	r9b, 64
 1      3     0.50                        sarx	rsi, r8, r9
 1      1     0.50                        cmove	r10, rsi
 3      5     1.00                        shrd	rdx, r8, cl
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	rdx, rsi
 1      2     0.20                        and	r11d, 1
 1      1     0.20                        add	r11, rdx
 1      1     0.50                        adc	r10, 0
 2      12    0.50           *            mov	qword ptr [rax + 16], r11
 2      12    0.50           *            mov	qword ptr [rax + 24], r10
 1      1     0.20                        mov	ecx, 1
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rsi
 2      7     0.50                  U     ret
 1      2     0.20                        xor	ecx, ecx
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
10.00  6.00   1.00   1.00   3.00   6.00   10.00  3.00   4.00   4.00   7.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, r9b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB70_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	rcx, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	r10, rcx, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	r11d, r11d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rcx, r10
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r10, r11
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     andn	rsi, rcx, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     andn	r10, r10, rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rcx, r8
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	rcx, 63
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     sub	r10, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sbb	rsi, 0
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovb	rsi, r11
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovb	r10, r11
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     shld	rsi, r10, 1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	r10, r10
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, r9d
 -     1.00    -      -      -      -     1.00    -      -      -     1.00    -      -     shrd	r10, rsi, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shrx	r11, rsi, r9
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	r11, r10
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sar	r10, 63
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sarx	rsi, r8, r9
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	r10, rsi
 -     1.00    -      -      -      -     1.00    -      -      -     1.00    -      -     shrd	rdx, r8, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rdx, rsi
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	r11d, 1
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	r11, rdx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     adc	r10, 0
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 16], r11
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 24], r10
1.00    -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, 1
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 8], 0
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rsi
 -      -      -     1.00    -      -     1.00    -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	ecx, ecx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 8], 0
 -      -     1.00    -      -      -      -      -      -      -      -      -      -     pop	rsi
```
## `checked_div_round_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      11
Total uOps:        17

Dispatch Width:    6
uOps Per Cycle:    1.55
IPC:               1.27
Block RThroughput: 4.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, edx
 1      0     0.20                        mov	edx, ecx
 1      1     0.20                        cmp	al, 7
 1      1     0.50                        ja	.LBB79_2
 1      1     0.20                        mov	r8b, 1
 1      0     0.20                        mov	ecx, eax
 2      2     1.00                        shl	r8b, cl
 1      1     0.50                        shr	r8b
 1      2     0.20                        and	r8b, dl
 2      2     1.00                        shr	dl, cl
 1      1     0.20                        cmp	r8b, 1
 1      1     0.50                        sbb	dl, -1
 1      1     0.20                        cmp	al, 8
 2      2     1.00                        setb	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   2.00    -      -      -     2.00   6.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	edx, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	al, 7
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB79_2
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r8b, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, eax
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r8b, cl
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r8b
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	r8b, dl
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	dl, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	r8b, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sbb	dl, -1
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	al, 8
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setb	al
```
## `checked_div_round_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      13
Total uOps:        15

Dispatch Width:    6
uOps Per Cycle:    1.15
IPC:               1.08
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 15
 1      1     0.50                        ja	.LBB76_1
 1      1     0.20                        mov	eax, 1
 1      3     0.50                        shlx	eax, eax, edx
 1      1     0.20                        movzx	eax, ax
 1      1     0.50                        shr	eax
 1      1     0.20                        movzx	ecx, cx
 1      3     0.50                        shrx	edx, ecx, edx
 1      2     0.20                        and	ecx, eax
 1      1     0.20                        cmp	cx, 1
 1      1     0.50                        sbb	dx, -1
 1      1     0.20                        mov	ax, 1
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   2.00    -      -      -     2.00   4.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 15
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB76_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	eax, eax, edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	eax, ax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movzx	ecx, cx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shrx	edx, ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	ecx, eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	cx, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sbb	dx, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ax, 1
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
```
## `checked_div_round_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      12
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.92
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 31
 1      1     0.50                        ja	.LBB77_1
 1      1     0.20                        mov	eax, 1
 1      3     0.50                        shlx	r8d, eax, edx
 1      1     0.50                        shr	r8d
 1      3     0.50                        shrx	edx, ecx, edx
 1      2     0.20                        and	r8d, ecx
 1      1     0.20                        cmp	r8d, 1
 1      1     0.50                        sbb	edx, -1
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   1.00    -      -      -     1.00   4.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 31
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB77_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	r8d, eax, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r8d
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shrx	edx, ecx, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	r8d, ecx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	r8d, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sbb	edx, -1
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
```
## `checked_div_round_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      12
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.92
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 63
 1      1     0.50                        ja	.LBB78_1
 1      1     0.20                        mov	eax, 1
 1      3     0.50                        shlx	r8, rax, rdx
 1      1     0.50                        shr	r8
 1      3     0.50                        shrx	rdx, rcx, rdx
 1      2     0.20                        and	r8, rcx
 1      1     0.20                        cmp	r8, 1
 1      1     0.50                        sbb	rdx, -1
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   1.00    -      -      -     1.00   4.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 63
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB78_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	r8, rax, rdx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r8
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shrx	rdx, rcx, rdx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	r8, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	r8, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sbb	rdx, -1
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
```
## `checked_div_round_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      45
Total Cycles:      38
Total uOps:        54

Dispatch Width:    6
uOps Per Cycle:    1.42
IPC:               1.18
Block RThroughput: 9.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	r15
 2      1     0.50           *            push	r14
 2      1     0.50           *            push	rsi
 2      1     0.50           *            push	rdi
 2      1     0.50           *            push	rbx
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB75_1
 1      3     0.50                        shrx	rbx, r8, r9
 1      2     0.20                        xor	edi, edi
 1      2     0.20                        test	r9b, 64
 0      1     0.00                        mov	r11, rbx
 1      1     0.50                        cmovne	r11, rdi
 0      1     0.00                        mov	rsi, rdx
 1      0     0.20                        mov	ecx, r9d
 3      5     1.00                        shrd	rsi, r8, cl
 1      2     0.20                        test	r9b, 64
 1      1     0.20                        mov	r10d, 1
 1      3     0.50                        shlx	r14, r10, r9
 0      1     0.00                        mov	r15, r14
 1      1     0.50                        cmovne	r15, rdi
 1      1     0.50                        cmovne	rsi, rbx
 3      5     1.00                        shld	rdi, r10, cl
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	rdi, r14
 1      3     1.00                        shrd	r15, rdi, 1
 1      1     0.50                        shr	rdi
 1      2     0.20                        and	rdi, r8
 1      2     0.20                        and	r15, rdx
 1      2     0.20                        xor	ecx, ecx
 1      2     0.20                        or	r15, rdi
 2      2     1.00                        setne	cl
 1      1     0.20                        add	rcx, rsi
 1      1     0.50                        adc	r11, 0
 2      12    0.50           *            mov	qword ptr [rax + 16], rcx
 2      12    0.50           *            mov	qword ptr [rax + 24], r11
 0      0     0.00                        jmp	.LBB75_3
 1      2     0.20                        xor	r10d, r10d
 2      12    0.50           *            mov	qword ptr [rax], r10
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rbx
 1      5     0.33    *                   pop	rdi
 1      5     0.33    *                   pop	rsi
 1      5     0.33    *                   pop	r14
 1      5     0.33    *                   pop	r15

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
7.00   5.00   1.00   2.00   4.00   5.00   8.00   4.00   5.00   5.00   6.00   2.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	r15
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     push	r14
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rsi
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     push	rdi
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rbx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, r9b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB75_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shrx	rbx, r8, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	edi, edi
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, rbx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r11, rdi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rsi, rdx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
1.00   1.00    -      -      -      -      -      -      -      -     1.00    -      -     shrd	rsi, r8, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r10d, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	r14, r10, r9
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r15, r14
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r15, rdi
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rsi, rbx
 -     1.00    -      -      -     1.00   1.00    -      -      -      -      -      -     shld	rdi, r10, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rdi, r14
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     shrd	r15, rdi, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	rdi
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	rdi, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	r15, rdx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	ecx, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     or	r15, rdi
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setne	cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	rcx, rsi
1.00    -      -      -      -      -      -      -      -      -      -      -      -     adc	r11, 0
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 16], rcx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 24], r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     jmp	.LBB75_3
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	r10d, r10d
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax], r10
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 8], 0
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rbx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -     pop	rdi
 -      -     1.00    -      -      -      -      -      -      -      -      -      -     pop	rsi
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	r14
 -      -      -     1.00    -      -      -      -      -      -      -      -      -     pop	r15
```
## `checked_div_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      19
Total Cycles:      23
Total uOps:        27

Dispatch Width:    6
uOps Per Cycle:    1.17
IPC:               0.83
Block RThroughput: 4.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB80_1
 1      0     0.20                        mov	ecx, r9d
 3      5     1.00                        shrd	rdx, r8, cl
 1      3     0.50                        shrx	rcx, r8, r9
 1      2     0.20                        xor	r8d, r8d
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	rdx, rcx
 1      1     0.50                        cmove	r8, rcx
 2      12    0.50           *            mov	qword ptr [rax + 24], r8
 2      12    0.50           *            mov	qword ptr [rax + 16], rdx
 1      1     0.20                        mov	ecx, 1
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 2      7     0.50                  U     ret
 1      2     0.20                        xor	ecx, ecx
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   2.00    -      -     3.00   2.00   4.00   3.00   3.00   3.00   3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, r9b
1.00    -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB80_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, r9d
1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -     shrd	rdx, r8, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shrx	rcx, r8, r9
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	r8d, r8d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rdx, rcx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	r8, rcx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 24], r8
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 16], rdx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, 1
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 8], 0
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	ecx, ecx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 8], 0
```
## `checked_floor_to_multiple_i8_unb_pow2`, `checked_floor_to_multiple_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      8
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.25
IPC:               1.00
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r9d, edx
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        cmp	dl, 8
 2      2     1.00                        setb	al
 1      1     0.20                        mov	dl, -1
 1      0     0.20                        mov	ecx, r9d
 2      2     1.00                        shl	dl, cl
 1      2     0.20                        and	dl, r8b

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     2.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r9d, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	dl, 8
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	dl, -1
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, r9d
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	dl, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	dl, r8b
```
## `checked_floor_to_multiple_i16_unb_pow2`, `checked_floor_to_multiple_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      9
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    0.78
IPC:               0.67
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 16
 2      2     1.00                        setb	al
 1      1     0.20                        mov	r8d, -1
 1      3     0.50                        shlx	edx, r8d, edx
 1      2     0.20                        and	edx, ecx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 16
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8d, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	edx, r8d, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	edx, ecx
```
## `checked_floor_to_multiple_i32_unb_pow2`, `checked_floor_to_multiple_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      9
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    0.78
IPC:               0.67
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 32
 2      2     1.00                        setb	al
 1      1     0.20                        mov	r8d, -1
 1      3     0.50                        shlx	edx, r8d, edx
 1      2     0.20                        and	edx, ecx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 32
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8d, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	edx, r8d, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	edx, ecx
```
## `checked_floor_to_multiple_i64_unb_pow2`, `checked_floor_to_multiple_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      9
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    0.78
IPC:               0.67
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 64
 2      2     1.00                        setb	al
 1      1     0.20                        mov	r8, -1
 1      3     0.50                        shlx	rdx, r8, rdx
 1      2     0.20                        and	rdx, rcx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 64
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	rdx, r8, rdx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	rdx, rcx
```
## `checked_floor_to_multiple_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      20
Total Cycles:      24
Total uOps:        26

Dispatch Width:    6
uOps Per Cycle:    1.08
IPC:               0.83
Block RThroughput: 4.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB85_1
 1      1     0.20                        mov	rcx, -1
 1      3     0.50                        shlx	r10, rcx, r9
 1      2     0.20                        xor	r11d, r11d
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	rcx, r10
 1      1     0.50                        cmove	r11, r10
 1      2     0.20                        and	r11, rdx
 1      2     0.20                        and	rcx, r8
 2      12    0.50           *            mov	qword ptr [rax + 24], rcx
 2      12    0.50           *            mov	qword ptr [rax + 16], r11
 1      1     0.20                        mov	ecx, 1
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 2      7     0.50                  U     ret
 1      2     0.20                        xor	ecx, ecx
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   2.00    -      -     3.00   2.00   3.00   3.00   3.00   3.00   3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, r9b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB85_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	rcx, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	r10, rcx, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	r11d, r11d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rcx, r10
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	r11, r10
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	r11, rdx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	rcx, r8
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 24], rcx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 16], r11
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, 1
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 8], 0
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 8], 0
```
## `checked_floor_to_multiple_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      20
Total Cycles:      24
Total uOps:        26

Dispatch Width:    6
uOps Per Cycle:    1.08
IPC:               0.83
Block RThroughput: 4.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB90_1
 1      1     0.20                        mov	rcx, -1
 1      3     0.50                        shlx	r10, rcx, r9
 1      2     0.20                        xor	r11d, r11d
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	rcx, r10
 1      1     0.50                        cmove	r11, r10
 1      2     0.20                        and	r11, rdx
 1      2     0.20                        and	rcx, r8
 2      12    0.50           *            mov	qword ptr [rax + 24], rcx
 2      12    0.50           *            mov	qword ptr [rax + 16], r11
 1      1     0.20                        mov	ecx, 1
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 2      7     0.50                  U     ret
 1      2     0.20                        xor	ecx, ecx
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   2.00    -      -     3.00   2.00   3.00   3.00   3.00   3.00   3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, r9b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB90_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	rcx, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	r10, rcx, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	r11d, r11d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rcx, r10
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	r11, r10
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	r11, rdx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	rcx, r8
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 24], rcx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 16], r11
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, 1
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 8], 0
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 8], 0
```
## `checked_mul_i8_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      10
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.20
IPC:               0.90
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, edx
 1      0     0.20                        mov	eax, ecx
 1      0     0.20                        mov	edx, ecx
 1      0     0.20                        mov	ecx, r8d
 2      2     1.00                        shl	dl, cl
 1      0     0.20                        mov	r9d, edx
 2      2     1.00                        sar	r9b, cl
 1      1     0.20                        cmp	al, r9b
 2      2     1.00                        sete	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   1.00    -      -      -     2.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	dl, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r9d, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sar	r9b, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	al, r9b
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sete	al
```
## `checked_mul_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      13
Total uOps:        18

Dispatch Width:    6
uOps Per Cycle:    1.38
IPC:               1.08
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, edx
 1      0     0.20                        mov	eax, ecx
 1      1     0.20                        cmp	dl, 8
 2      2     1.00                        setb	r9b
 1      0     0.20                        mov	edx, ecx
 1      0     0.20                        mov	ecx, r8d
 2      2     1.00                        shl	dl, cl
 1      2     0.20                        and	r8b, 7
 1      0     0.20                        mov	r10d, edx
 1      0     0.20                        mov	ecx, r8d
 2      2     1.00                        sar	r10b, cl
 1      1     0.20                        cmp	al, r10b
 2      2     1.00                        sete	al
 1      2     0.20                        and	al, r9b

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   2.00    -      -      -     3.00   5.00    -      -      -     3.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	dl, 8
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setb	r9b
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	edx, ecx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, r8d
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	dl, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	r8b, 7
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r10d, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sar	r10b, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	al, r10b
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sete	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	al, r9b
```
## `checked_mul_i16_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      13
Total uOps:        8

Dispatch Width:    6
uOps Per Cycle:    0.62
IPC:               0.54
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      3     0.50                        shlx	eax, ecx, edx
 1      1     0.33                        movsx	r8d, ax
 1      3     0.50                        sarx	edx, r8d, edx
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	cx, dx
 2      2     1.00                        sete	al
 1      0     0.20                        mov	edx, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -     1.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	eax, ecx, edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movsx	r8d, ax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sarx	edx, r8d, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	cx, dx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sete	al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
```
## `checked_mul_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      15
Total uOps:        13

Dispatch Width:    6
uOps Per Cycle:    0.87
IPC:               0.73
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 15
 1      1     0.50                        ja	.LBB98_1
 1      0     0.20                        mov	eax, edx
 1      3     0.50                        shlx	edx, ecx, edx
 1      1     0.33                        movsx	r8d, dx
 1      3     0.50                        sarx	r8d, r8d, eax
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	cx, r8w
 2      2     1.00                        sete	al
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   1.00    -      -      -     2.00   3.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 15
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB98_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	edx, ecx, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movsx	r8d, dx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	r8d, r8d, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	cx, r8w
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sete	al
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
```
## `checked_mul_i32_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      12
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    0.58
IPC:               0.50
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      3     0.50                        shlx	r8d, ecx, edx
 1      3     0.50                        sarx	edx, r8d, edx
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	ecx, edx
 2      2     1.00                        sete	al
 1      0     0.20                        mov	edx, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -     1.00   3.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	r8d, ecx, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sarx	edx, r8d, edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sete	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, r8d
```
## `checked_mul_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      14
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    0.86
IPC:               0.71
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 31
 1      1     0.50                        ja	.LBB100_1
 1      0     0.20                        mov	eax, edx
 1      3     0.50                        shlx	edx, ecx, edx
 1      3     0.50                        sarx	r8d, edx, eax
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	ecx, r8d
 2      2     1.00                        sete	al
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   1.00    -      -      -     1.00   3.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 31
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB100_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	edx, ecx, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	r8d, edx, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	ecx, r8d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sete	al
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
```
## `checked_mul_i64_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      12
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.50
IPC:               0.50
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      3     0.50                        shlx	r8, rcx, rdx
 1      3     0.50                        sarx	rdx, r8, rdx
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	rcx, rdx
 2      2     1.00                        sete	al
 0      1     0.00                        mov	rdx, r8

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00    -      -      -      -     1.00   3.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	r8, rcx, rdx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sarx	rdx, r8, rdx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	rcx, rdx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sete	al
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r8
```
## `checked_mul_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      14
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    0.79
IPC:               0.71
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 63
 1      1     0.50                        ja	.LBB102_1
 0      1     0.00                        mov	rax, rdx
 1      3     0.50                        shlx	rdx, rcx, rdx
 1      3     0.50                        sarx	r8, rdx, rax
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	rcx, r8
 2      2     1.00                        sete	al
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   4.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 63
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB102_1
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	rdx, rcx, rdx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sarx	r8, rdx, rax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	rcx, r8
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sete	al
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
```
## `checked_mul_i128_pow2`
```asm
Iterations:        1
Instructions:      35
Total Cycles:      29
Total uOps:        43

Dispatch Width:    6
uOps Per Cycle:    1.48
IPC:               1.21
Block RThroughput: 7.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	r14
 2      1     0.50           *            push	rsi
 2      1     0.50           *            push	rdi
 2      1     0.50           *            push	rbx
 0      1     0.00                        mov	rax, rcx
 0      1     0.00                        mov	r10, r8
 1      0     0.20                        mov	ecx, r9d
 3      5     1.00                        shld	r10, rdx, cl
 1      3     0.50                        shlx	r11, rdx, r9
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	r10, r11
 0      1     0.00                        mov	rdi, r10
 1      1     0.50                        sar	rdi, 63
 1      2     0.20                        xor	esi, esi
 1      2     0.20                        test	r9b, 64
 1      3     0.50                        sarx	rbx, r10, r9
 1      1     0.50                        cmove	rdi, rbx
 1      1     0.50                        cmovne	r11, rsi
 0      1     0.00                        mov	r14, r11
 3      5     1.00                        shrd	r14, r10, cl
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	r14, rbx
 1      2     0.20                        xor	rdi, r8
 1      2     0.20                        xor	r14, rdx
 1      2     0.20                        or	r14, rdi
 1      1     0.50                        jne	.LBB95_2
 2      12    0.50           *            mov	qword ptr [rax + 16], r11
 2      12    0.50           *            mov	qword ptr [rax + 24], r10
 1      1     0.20                        mov	esi, 1
 2      12    0.50           *            mov	qword ptr [rax], rsi
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rbx
 1      5     0.33    *                   pop	rdi
 1      5     0.33    *                   pop	rsi
 1      5     0.33    *                   pop	r14

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   4.00   1.00   1.00   4.00   4.00   6.00   4.00   4.00   4.00   4.00   2.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	r14
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     push	rsi
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rdi
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     push	rbx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, r8
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, r9d
 -     1.00    -      -      -     1.00   1.00    -      -      -      -      -      -     shld	r10, rdx, cl
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	r11, rdx, r9
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r10, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdi, r10
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	rdi, 63
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	esi, esi
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sarx	rbx, r10, r9
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	rdi, rbx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r11, rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r14, r11
 -     1.00    -      -      -     1.00   1.00    -      -      -      -      -      -     shrd	r14, r10, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r14, rbx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	rdi, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	r14, rdx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     or	r14, rdi
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     jne	.LBB95_2
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 16], r11
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 24], r10
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	esi, 1
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax], rsi
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 8], 0
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rbx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -     pop	rdi
 -      -     1.00    -      -      -      -      -      -      -      -      -      -     pop	rsi
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	r14
```
## `checked_mul_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      39
Total Cycles:      30
Total uOps:        46

Dispatch Width:    6
uOps Per Cycle:    1.53
IPC:               1.30
Block RThroughput: 7.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	r14
 2      1     0.50           *            push	rsi
 2      1     0.50           *            push	rdi
 2      1     0.50           *            push	rbx
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB96_1
 0      1     0.00                        mov	r10, r8
 1      0     0.20                        mov	ecx, r9d
 3      5     1.00                        shld	r10, rdx, cl
 1      3     0.50                        shlx	rsi, rdx, r9
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	r10, rsi
 0      1     0.00                        mov	rdi, r10
 1      1     0.50                        sar	rdi, 63
 1      2     0.20                        xor	r11d, r11d
 1      2     0.20                        test	r9b, 64
 1      3     0.50                        sarx	rbx, r10, r9
 1      1     0.50                        cmove	rdi, rbx
 1      1     0.50                        cmovne	rsi, r11
 0      1     0.00                        mov	r14, rsi
 3      5     1.00                        shrd	r14, r10, cl
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	r14, rbx
 1      2     0.20                        xor	rdi, r8
 1      2     0.20                        xor	r14, rdx
 1      2     0.20                        or	r14, rdi
 1      1     0.50                        jne	.LBB96_4
 2      12    0.50           *            mov	qword ptr [rax + 16], rsi
 2      12    0.50           *            mov	qword ptr [rax + 24], r10
 1      1     0.20                        mov	r11d, 1
 0      0     0.00                        jmp	.LBB96_4
 1      2     0.20                        xor	r11d, r11d
 2      12    0.50           *            mov	qword ptr [rax], r11
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rbx
 1      5     0.33    *                   pop	rdi
 1      5     0.33    *                   pop	rsi
 1      5     0.33    *                   pop	r14

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
6.00   4.00   1.00   1.00   4.00   5.00   6.00   4.00   4.00   4.00   5.00   2.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	r14
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     push	rsi
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rdi
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     push	rbx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, r9b
1.00    -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB96_1
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, r9d
1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -     shld	r10, rdx, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	rsi, rdx, r9
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r10, rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdi, r10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sar	rdi, 63
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	r11d, r11d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	rbx, r10, r9
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	rdi, rbx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rsi, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r14, rsi
1.00   1.00    -      -      -      -      -      -      -      -     1.00    -      -     shrd	r14, r10, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r14, rbx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	rdi, r8
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	r14, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     or	r14, rdi
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jne	.LBB96_4
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 16], rsi
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 24], r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r11d, 1
 -      -      -      -      -      -      -      -      -      -      -      -      -     jmp	.LBB96_4
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	r11d, r11d
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax], r11
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 8], 0
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rbx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -     pop	rdi
 -      -     1.00    -      -      -      -      -      -      -      -      -      -     pop	rsi
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	r14
```
## `checked_mul_u8_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      10
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.20
IPC:               0.90
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, edx
 1      0     0.20                        mov	eax, ecx
 1      0     0.20                        mov	edx, ecx
 1      0     0.20                        mov	ecx, r8d
 2      2     1.00                        shl	dl, cl
 1      0     0.20                        mov	r9d, edx
 2      2     1.00                        shr	r9b, cl
 1      1     0.20                        cmp	al, r9b
 2      2     1.00                        sete	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   1.00    -      -      -     2.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	dl, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r9d, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	r9b, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	al, r9b
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sete	al
```
## `checked_mul_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      13
Total uOps:        18

Dispatch Width:    6
uOps Per Cycle:    1.38
IPC:               1.08
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, edx
 1      0     0.20                        mov	eax, ecx
 1      1     0.20                        cmp	dl, 8
 2      2     1.00                        setb	r9b
 1      0     0.20                        mov	edx, ecx
 1      0     0.20                        mov	ecx, r8d
 2      2     1.00                        shl	dl, cl
 1      2     0.20                        and	r8b, 7
 1      0     0.20                        mov	r10d, edx
 1      0     0.20                        mov	ecx, r8d
 2      2     1.00                        shr	r10b, cl
 1      1     0.20                        cmp	al, r10b
 2      2     1.00                        sete	al
 1      2     0.20                        and	al, r9b

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   2.00    -      -      -     3.00   5.00    -      -      -     3.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	dl, 8
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setb	r9b
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	edx, ecx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, r8d
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	dl, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	r8b, 7
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r10d, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	r10b, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	al, r10b
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sete	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	al, r9b
```
## `checked_mul_u16_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      13
Total uOps:        8

Dispatch Width:    6
uOps Per Cycle:    0.62
IPC:               0.54
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      3     0.50                        shlx	eax, ecx, edx
 1      1     0.20                        movzx	r8d, ax
 1      3     0.50                        shrx	edx, r8d, edx
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	cx, dx
 2      2     1.00                        sete	al
 1      0     0.20                        mov	edx, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -     1.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	eax, ecx, edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	r8d, ax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shrx	edx, r8d, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	cx, dx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sete	al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
```
## `checked_mul_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      15
Total uOps:        13

Dispatch Width:    6
uOps Per Cycle:    0.87
IPC:               0.73
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 15
 1      1     0.50                        ja	.LBB108_1
 1      0     0.20                        mov	eax, edx
 1      3     0.50                        shlx	edx, ecx, edx
 1      1     0.20                        movzx	r8d, dx
 1      3     0.50                        shrx	r8d, r8d, eax
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	cx, r8w
 2      2     1.00                        sete	al
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   1.00    -      -      -     2.00   3.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 15
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB108_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	edx, ecx, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movzx	r8d, dx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shrx	r8d, r8d, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	cx, r8w
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sete	al
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
```
## `checked_mul_u32_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      12
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    0.58
IPC:               0.50
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      3     0.50                        shlx	r8d, ecx, edx
 1      3     0.50                        shrx	edx, r8d, edx
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	ecx, edx
 2      2     1.00                        sete	al
 1      0     0.20                        mov	edx, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -     1.00   3.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	r8d, ecx, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shrx	edx, r8d, edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sete	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, r8d
```
## `checked_mul_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      14
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    0.86
IPC:               0.71
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 31
 1      1     0.50                        ja	.LBB110_1
 1      0     0.20                        mov	eax, edx
 1      3     0.50                        shlx	edx, ecx, edx
 1      3     0.50                        shrx	r8d, edx, eax
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	ecx, r8d
 2      2     1.00                        sete	al
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   1.00    -      -      -     1.00   3.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 31
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB110_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	edx, ecx, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shrx	r8d, edx, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	ecx, r8d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sete	al
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
```
## `checked_mul_u64_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      12
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.50
IPC:               0.50
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      3     0.50                        shlx	r8, rcx, rdx
 1      3     0.50                        shrx	rdx, r8, rdx
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	rcx, rdx
 2      2     1.00                        sete	al
 0      1     0.00                        mov	rdx, r8

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00    -      -      -      -     1.00   3.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	r8, rcx, rdx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shrx	rdx, r8, rdx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	rcx, rdx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sete	al
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r8
```
## `checked_mul_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      14
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    0.79
IPC:               0.71
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 63
 1      1     0.50                        ja	.LBB112_1
 0      1     0.00                        mov	rax, rdx
 1      3     0.50                        shlx	rdx, rcx, rdx
 1      3     0.50                        shrx	r8, rdx, rax
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	rcx, r8
 2      2     1.00                        sete	al
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   4.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 63
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB112_1
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	rdx, rcx, rdx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shrx	r8, rdx, rax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	rcx, r8
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sete	al
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
```
## `checked_mul_u128_pow2`
```asm
Iterations:        1
Instructions:      33
Total Cycles:      29
Total uOps:        41

Dispatch Width:    6
uOps Per Cycle:    1.41
IPC:               1.14
Block RThroughput: 6.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	r14
 2      1     0.50           *            push	rsi
 2      1     0.50           *            push	rdi
 2      1     0.50           *            push	rbx
 0      1     0.00                        mov	rax, rcx
 0      1     0.00                        mov	r10, r8
 1      0     0.20                        mov	ecx, r9d
 3      5     1.00                        shld	r10, rdx, cl
 1      3     0.50                        shlx	rsi, rdx, r9
 1      2     0.20                        xor	r11d, r11d
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	r10, rsi
 1      1     0.50                        cmovne	rsi, r11
 1      3     0.50                        shrx	rdi, r10, r9
 0      1     0.00                        mov	rbx, rdi
 1      1     0.50                        cmovne	rbx, r11
 0      1     0.00                        mov	r14, rsi
 3      5     1.00                        shrd	r14, r10, cl
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	r14, rdi
 1      2     0.20                        xor	r14, rdx
 1      2     0.20                        xor	rbx, r8
 1      2     0.20                        or	rbx, r14
 1      1     0.50                        jne	.LBB105_2
 2      12    0.50           *            mov	qword ptr [rax + 16], rsi
 2      12    0.50           *            mov	qword ptr [rax + 24], r10
 1      1     0.20                        mov	r11d, 1
 2      12    0.50           *            mov	qword ptr [rax], r11
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rbx
 1      5     0.33    *                   pop	rdi
 1      5     0.33    *                   pop	rsi
 1      5     0.33    *                   pop	r14

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   3.00   1.00   1.00   4.00   4.00   5.00   4.00   4.00   4.00   4.00   2.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	r14
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     push	rsi
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rdi
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     push	rbx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, r8
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, r9d
 -     1.00    -      -      -     1.00   1.00    -      -      -      -      -      -     shld	r10, rdx, cl
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	rsi, rdx, r9
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	r11d, r11d
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r10, rsi
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rsi, r11
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shrx	rdi, r10, r9
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rbx, rdi
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rbx, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r14, rsi
 -     1.00    -      -      -      -     1.00    -      -      -     1.00    -      -     shrd	r14, r10, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r14, rdi
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	r14, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	rbx, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     or	rbx, r14
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jne	.LBB105_2
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 16], rsi
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 24], r10
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r11d, 1
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax], r11
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 8], 0
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rbx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -     pop	rdi
 -      -     1.00    -      -      -      -      -      -      -      -      -      -     pop	rsi
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	r14
```
## `checked_mul_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      37
Total Cycles:      30
Total uOps:        44

Dispatch Width:    6
uOps Per Cycle:    1.47
IPC:               1.23
Block RThroughput: 7.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	r14
 2      1     0.50           *            push	rsi
 2      1     0.50           *            push	rdi
 2      1     0.50           *            push	rbx
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB106_1
 0      1     0.00                        mov	r11, r8
 1      0     0.20                        mov	ecx, r9d
 3      5     1.00                        shld	r11, rdx, cl
 1      3     0.50                        shlx	rsi, rdx, r9
 1      2     0.20                        xor	r10d, r10d
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	r11, rsi
 1      1     0.50                        cmovne	rsi, r10
 1      3     0.50                        shrx	rdi, r11, r9
 0      1     0.00                        mov	rbx, rdi
 1      1     0.50                        cmovne	rbx, r10
 0      1     0.00                        mov	r14, rsi
 3      5     1.00                        shrd	r14, r11, cl
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	r14, rdi
 1      2     0.20                        xor	r14, rdx
 1      2     0.20                        xor	rbx, r8
 1      2     0.20                        or	rbx, r14
 1      1     0.50                        jne	.LBB106_4
 2      12    0.50           *            mov	qword ptr [rax + 16], rsi
 2      12    0.50           *            mov	qword ptr [rax + 24], r11
 1      1     0.20                        mov	r10d, 1
 0      0     0.00                        jmp	.LBB106_4
 1      2     0.20                        xor	r10d, r10d
 2      12    0.50           *            mov	qword ptr [rax], r10
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rbx
 1      5     0.33    *                   pop	rdi
 1      5     0.33    *                   pop	rsi
 1      5     0.33    *                   pop	r14

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   4.00   1.00   1.00   4.00   4.00   6.00   4.00   4.00   4.00   5.00   2.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	r14
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     push	rsi
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rdi
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     push	rbx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, r9b
1.00    -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB106_1
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, r9d
1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -     shld	r11, rdx, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	rsi, rdx, r9
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	r10d, r10d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r11, rsi
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rsi, r10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shrx	rdi, r11, r9
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rbx, rdi
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rbx, r10
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r14, rsi
1.00   1.00    -      -      -      -      -      -      -      -     1.00    -      -     shrd	r14, r11, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r14, rdi
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	r14, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	rbx, r8
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     or	rbx, r14
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     jne	.LBB106_4
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 16], rsi
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 24], r11
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r10d, 1
 -      -      -      -      -      -      -      -      -      -      -      -      -     jmp	.LBB106_4
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	r10d, r10d
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax], r10
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 8], 0
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rbx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -     pop	rdi
 -      -     1.00    -      -      -      -      -      -      -      -      -      -     pop	rsi
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	r14
```
## `checked_rem_floor_i8_unb_pow2`, `checked_rem_floor_u8_unb_pow2`, `checked_rem_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      9
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    1.22
IPC:               1.00
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r9d, edx
 1      1     0.20                        cmp	dl, 8
 2      2     1.00                        setb	al
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        mov	dl, -1
 1      0     0.20                        mov	ecx, r9d
 2      2     1.00                        shl	dl, cl
 1      1     0.20                        not	dl
 1      2     0.20                        and	dl, r8b

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   2.00    -      -      -     2.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r9d, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 8
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8d, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	dl, -1
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, r9d
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	dl, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	dl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	dl, r8b
```
## `checked_rem_floor_i16_unb_pow2`, `checked_rem_floor_u16_unb_pow2`, `checked_rem_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      9
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    0.78
IPC:               0.67
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 16
 2      2     1.00                        setb	al
 1      1     0.20                        mov	r8d, -1
 1      3     0.50                        shlx	edx, r8d, edx
 1      2     0.33                        andn	edx, edx, ecx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 16
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8d, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	edx, r8d, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     andn	edx, edx, ecx
```
## `checked_rem_floor_i32_unb_pow2`, `checked_rem_floor_u32_unb_pow2`, `checked_rem_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      9
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    0.78
IPC:               0.67
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 32
 2      2     1.00                        setb	al
 1      1     0.20                        mov	r8d, -1
 1      3     0.50                        shlx	edx, r8d, edx
 1      2     0.33                        andn	edx, edx, ecx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 32
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8d, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	edx, r8d, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     andn	edx, edx, ecx
```
## `checked_rem_floor_i64_unb_pow2`, `checked_rem_floor_u64_unb_pow2`, `checked_rem_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      9
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    0.78
IPC:               0.67
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 64
 2      2     1.00                        setb	al
 1      1     0.20                        mov	r8, -1
 1      3     0.50                        shlx	rdx, r8, rdx
 1      2     0.33                        andn	rdx, rdx, rcx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 64
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	rdx, r8, rdx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     andn	rdx, rdx, rcx
```
## `checked_rem_floor_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      20
Total Cycles:      24
Total uOps:        26

Dispatch Width:    6
uOps Per Cycle:    1.08
IPC:               0.83
Block RThroughput: 4.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB115_1
 1      1     0.20                        mov	rcx, -1
 1      3     0.50                        shlx	r10, rcx, r9
 1      2     0.20                        xor	r11d, r11d
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	rcx, r10
 1      1     0.50                        cmove	r11, r10
 1      2     0.33                        andn	rdx, r11, rdx
 1      2     0.33                        andn	rcx, rcx, r8
 2      12    0.50           *            mov	qword ptr [rax + 24], rcx
 2      12    0.50           *            mov	qword ptr [rax + 16], rdx
 1      1     0.20                        mov	ecx, 1
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 2      7     0.50                  U     ret
 1      2     0.20                        xor	ecx, ecx
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   2.00    -      -     3.00   2.00   3.00   3.00   3.00   3.00   3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, r9b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB115_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	rcx, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	r10, rcx, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	r11d, r11d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rcx, r10
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	r11, r10
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     andn	rdx, r11, rdx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     andn	rcx, rcx, r8
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 24], rcx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 16], rdx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, 1
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 8], 0
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 8], 0
```
## `checked_rem_floor_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      20
Total Cycles:      24
Total uOps:        26

Dispatch Width:    6
uOps Per Cycle:    1.08
IPC:               0.83
Block RThroughput: 4.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB120_1
 1      1     0.20                        mov	rcx, -1
 1      3     0.50                        shlx	r10, rcx, r9
 1      2     0.20                        xor	r11d, r11d
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	rcx, r10
 1      1     0.50                        cmove	r11, r10
 1      2     0.33                        andn	rdx, r11, rdx
 1      2     0.33                        andn	rcx, rcx, r8
 2      12    0.50           *            mov	qword ptr [rax + 24], rcx
 2      12    0.50           *            mov	qword ptr [rax + 16], rdx
 1      1     0.20                        mov	ecx, 1
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 2      7     0.50                  U     ret
 1      2     0.20                        xor	ecx, ecx
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   2.00    -      -     3.00   2.00   3.00   3.00   3.00   3.00   3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, r9b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB120_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	rcx, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	r10, rcx, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	r11d, r11d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rcx, r10
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	r11, r10
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     andn	rdx, r11, rdx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     andn	rcx, rcx, r8
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 24], rcx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 16], rdx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, 1
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 8], 0
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 8], 0
```
## `checked_rem_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      17
Total Cycles:      13
Total uOps:        19

Dispatch Width:    6
uOps Per Cycle:    1.46
IPC:               1.31
Block RThroughput: 3.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        cmp	dl, 7
 1      1     0.50                        ja	.LBB129_2
 1      1     0.20                        mov	al, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	al, cl
 1      0     0.20                        mov	ecx, eax
 1      1     0.20                        not	cl
 1      0     0.20                        mov	r9d, r8d
 1      1     0.50                        sar	r9b, 7
 1      2     0.20                        and	r9b, cl
 1      1     0.20                        add	r9b, r8b
 1      2     0.20                        and	r9b, al
 1      1     0.20                        sub	r8b, r9b
 1      1     0.20                        cmp	dl, 8
 2      2     1.00                        setb	al
 1      0     0.20                        mov	edx, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   3.00    -      -      -     3.00   6.00    -      -      -     4.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 7
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB129_2
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	al, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	al, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r9d, r8d
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	r9b, 7
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	r9b, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	r9b, r8b
1.00    -      -      -      -      -      -      -      -      -      -      -      -     and	r9b, al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     sub	r8b, r9b
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	dl, 8
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setb	al
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	edx, r8d
```
## `checked_rem_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      13
Total uOps:        16

Dispatch Width:    6
uOps Per Cycle:    1.23
IPC:               1.15
Block RThroughput: 2.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 15
 1      1     0.50                        ja	.LBB126_1
 1      1     0.20                        mov	eax, -1
 1      3     0.50                        shlx	eax, eax, edx
 1      1     0.33                        movsx	r8d, cx
 1      1     0.50                        sar	r8d, 15
 1      3     1.00                        bzhi	edx, r8d, edx
 1      1     0.20                        add	edx, ecx
 1      2     0.20                        and	edx, eax
 1      1     0.20                        sub	ecx, edx
 1      1     0.20                        mov	ax, 1
 1      0     0.20                        mov	edx, ecx
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax
 1      0     0.20                        mov	edx, ecx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   3.00    -      -      -     3.00   4.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 15
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB126_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	eax, eax, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movsx	r8d, cx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	r8d, 15
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	edx, r8d, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	edx, ecx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	edx, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sub	ecx, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ax, 1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, ecx
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	edx, ecx
```
## `checked_rem_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      12
Total uOps:        16

Dispatch Width:    6
uOps Per Cycle:    1.33
IPC:               1.25
Block RThroughput: 2.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 31
 1      1     0.50                        ja	.LBB127_1
 1      1     0.20                        mov	eax, -1
 1      3     0.50                        shlx	eax, eax, edx
 1      0     0.20                        mov	r8d, ecx
 1      1     0.50                        sar	r8d, 31
 1      3     1.00                        bzhi	edx, r8d, edx
 1      1     0.20                        add	edx, ecx
 1      2     0.20                        and	edx, eax
 1      1     0.20                        sub	ecx, edx
 1      1     0.20                        mov	eax, 1
 1      0     0.20                        mov	edx, ecx
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax
 1      0     0.20                        mov	edx, ecx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   3.00    -      -      -     3.00   3.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 31
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB127_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	eax, eax, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8d, ecx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	r8d, 31
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	edx, r8d, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	edx, ecx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     and	edx, eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     sub	ecx, edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, 1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, ecx
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	edx, ecx
```
## `checked_rem_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      14
Total uOps:        13

Dispatch Width:    6
uOps Per Cycle:    0.93
IPC:               1.07
Block RThroughput: 2.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 63
 1      1     0.50                        ja	.LBB128_1
 1      1     0.20                        mov	rax, -1
 1      3     0.50                        shlx	rax, rax, rdx
 0      1     0.00                        mov	r8, rcx
 1      1     0.50                        sar	r8, 63
 1      3     1.00                        bzhi	rdx, r8, rdx
 1      1     0.20                        add	rdx, rcx
 1      2     0.20                        and	rdx, rax
 1      1     0.20                        sub	rcx, rdx
 1      1     0.20                        mov	eax, 1
 0      1     0.00                        mov	rdx, rcx
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax
 0      1     0.00                        mov	rdx, rcx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   2.00    -      -      -     2.00   3.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 63
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB128_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	rax, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	rax, rax, rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	r8, 63
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	rdx, r8, rdx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	rdx, rcx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	rdx, rax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sub	rcx, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	eax, 1
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, rcx
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, rcx
```
## `checked_rem_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      28
Total Cycles:      29
Total uOps:        33

Dispatch Width:    6
uOps Per Cycle:    1.14
IPC:               0.97
Block RThroughput: 5.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB125_1
 1      1     0.20                        mov	rcx, -1
 1      3     0.50                        shlx	r10, rcx, r9
 1      2     0.20                        xor	r11d, r11d
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmove	r11, r10
 1      1     0.50                        cmovne	rcx, r10
 0      1     0.00                        mov	r9, r8
 1      1     0.50                        sar	r9, 63
 1      2     0.33                        andn	r10, rcx, r9
 1      2     0.33                        andn	r9, r11, r9
 1      1     0.20                        add	r9, rdx
 1      1     0.50                        adc	r10, r8
 1      2     0.20                        and	r10, rcx
 1      2     0.20                        and	r9, r11
 1      1     0.20                        sub	rdx, r9
 1      1     0.50                        sbb	r8, r10
 2      12    0.50           *            mov	qword ptr [rax + 16], rdx
 2      12    0.50           *            mov	qword ptr [rax + 24], r8
 1      1     0.20                        mov	ecx, 1
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 2      7     0.50                  U     ret
 1      2     0.20                        xor	ecx, ecx
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   3.00    -      -     3.00   3.00   5.00   3.00   3.00   3.00   4.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, r9b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB125_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	rcx, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	r10, rcx, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	r11d, r11d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	r11, r10
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rcx, r10
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, r8
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	r9, 63
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     andn	r10, rcx, r9
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     andn	r9, r11, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	r9, rdx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     adc	r10, r8
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	r10, rcx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	r9, r11
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sub	rdx, r9
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sbb	r8, r10
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 16], rdx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 24], r8
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, 1
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 8], 0
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	ecx, ecx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 8], 0
```
## `checked_rem_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      20
Total Cycles:      24
Total uOps:        26

Dispatch Width:    6
uOps Per Cycle:    1.08
IPC:               0.83
Block RThroughput: 4.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB130_1
 1      1     0.20                        mov	rcx, -1
 1      3     0.50                        shlx	r10, rcx, r9
 1      2     0.20                        xor	r11d, r11d
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	rcx, r10
 1      1     0.50                        cmove	r11, r10
 1      2     0.33                        andn	rdx, r11, rdx
 1      2     0.33                        andn	rcx, rcx, r8
 2      12    0.50           *            mov	qword ptr [rax + 24], rcx
 2      12    0.50           *            mov	qword ptr [rax + 16], rdx
 1      1     0.20                        mov	ecx, 1
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 2      7     0.50                  U     ret
 1      2     0.20                        xor	ecx, ecx
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   2.00    -      -     3.00   2.00   3.00   3.00   3.00   3.00   3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, r9b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB130_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	rcx, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	r10, rcx, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	r11d, r11d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rcx, r10
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	r11, r10
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     andn	rdx, r11, rdx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     andn	rcx, rcx, r8
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 24], rcx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 16], rdx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, 1
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 8], 0
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 8], 0
```
## `checked_round_to_multiple_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      21
Total Cycles:      13
Total uOps:        23

Dispatch Width:    6
uOps Per Cycle:    1.77
IPC:               1.62
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      2     0.20                        xor	r10d, r10d
 1      1     0.20                        cmp	dl, 7
 1      1     0.50                        ja	.LBB139_1
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        mov	al, 1
 1      1     0.20                        mov	r9b, 1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r9b, cl
 1      0     0.20                        mov	ecx, r9d
 1      1     0.50                        shr	cl
 1      0     0.20                        mov	edx, r8d
 1      1     0.50                        shr	dl, 7
 1      1     0.20                        sub	cl, dl
 1      0     0.20                        movzx	edx, cl
 1      1     0.50                        cmovb	edx, r10d
 1      1     0.20                        add	dl, r8b
 1      1     0.50                        jo	.LBB139_1
 1      1     0.20                        neg	r9b
 1      2     0.20                        and	dl, r9b
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   4.00    -      -      -     4.00   5.00    -      -      -     4.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	r10d, r10d
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 7
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB139_1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8d, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	al, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     mov	r9b, 1
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r9b, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, r8d
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shr	dl, 7
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     sub	cl, dl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	edx, cl
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovb	edx, r10d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	dl, r8b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     jo	.LBB139_1
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     neg	r9b
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	dl, r9b
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
```
## `checked_round_to_multiple_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      18
Total Cycles:      14
Total uOps:        19

Dispatch Width:    6
uOps Per Cycle:    1.36
IPC:               1.29
Block RThroughput: 3.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 15
 1      1     0.50                        ja	.LBB136_4
 1      1     0.20                        mov	r8d, 1
 1      3     0.50                        shlx	edx, r8d, edx
 1      1     0.20                        movzx	r8d, dx
 1      1     0.50                        shr	r8d
 1      1     0.20                        movzx	ecx, cx
 1      0     0.20                        mov	r9d, ecx
 1      1     0.50                        shr	r9d, 15
 1      1     0.20                        sub	r8w, r9w
 1      1     0.50                        cmovb	r8d, eax
 1      1     0.20                        add	cx, r8w
 1      1     0.50                        jo	.LBB136_4
 1      1     0.20                        neg	edx
 1      2     0.20                        and	edx, ecx
 1      1     0.20                        mov	ax, 1
 2      7     0.50                  U     ret

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   3.00    -      -      -     3.00   4.00    -      -      -     4.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 15
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB136_4
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8d, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	edx, r8d, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movzx	r8d, dx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r8d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movzx	ecx, cx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r9d, ecx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r9d, 15
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     sub	r8w, r9w
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovb	r8d, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	cx, r8w
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jo	.LBB136_4
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     neg	edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	edx, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ax, 1
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
```
## `checked_round_to_multiple_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      18
Total Cycles:      13
Total uOps:        19

Dispatch Width:    6
uOps Per Cycle:    1.46
IPC:               1.38
Block RThroughput: 3.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 31
 1      1     0.50                        ja	.LBB137_3
 1      1     0.20                        mov	eax, 1
 1      3     0.50                        shlx	r8d, eax, edx
 1      0     0.20                        mov	edx, r8d
 1      1     0.50                        shr	edx
 1      0     0.20                        mov	r9d, ecx
 1      1     0.50                        shr	r9d, 31
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        sub	edx, r9d
 1      1     0.50                        cmovb	edx, eax
 1      1     0.20                        add	edx, ecx
 1      1     0.50                        jo	.LBB137_4
 1      1     0.20                        neg	r8d
 1      2     0.20                        and	edx, r8d
 1      1     0.20                        mov	eax, 1
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   3.00    -      -      -     3.00   4.00    -      -      -     4.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 31
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB137_3
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	r8d, eax, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r9d, ecx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r9d, 31
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     sub	edx, r9d
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovb	edx, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	edx, ecx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jo	.LBB137_4
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     neg	r8d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	edx, r8d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, 1
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
```
## `checked_round_to_multiple_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      18
Total Cycles:      14
Total uOps:        17

Dispatch Width:    6
uOps Per Cycle:    1.21
IPC:               1.29
Block RThroughput: 3.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 63
 1      1     0.50                        ja	.LBB138_3
 1      1     0.20                        mov	eax, 1
 1      3     0.50                        shlx	r8, rax, rdx
 0      1     0.00                        mov	rdx, r8
 1      1     0.50                        shr	rdx
 0      1     0.00                        mov	r9, rcx
 1      1     0.50                        shr	r9, 63
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        sub	rdx, r9
 1      1     0.50                        cmovb	rdx, rax
 1      1     0.20                        add	rdx, rcx
 1      1     0.50                        jo	.LBB138_4
 1      1     0.20                        neg	r8
 1      2     0.20                        and	rdx, r8
 1      1     0.20                        mov	eax, 1
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   2.00    -      -      -     3.00   4.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 63
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB138_3
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	r8, rax, rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r8
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r9, 63
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     sub	rdx, r9
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovb	rdx, rax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	rdx, rcx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jo	.LBB138_4
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     neg	r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	rdx, r8
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	eax, 1
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
```
## `checked_round_to_multiple_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      42
Total Cycles:      36
Total uOps:        46

Dispatch Width:    6
uOps Per Cycle:    1.28
IPC:               1.17
Block RThroughput: 7.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	rsi
 2      1     0.50           *            push	rdi
 2      1     0.50           *            push	rbx
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB135_3
 1      2     0.20                        xor	esi, esi
 1      1     0.20                        mov	r10d, 1
 1      2     0.20                        xor	r11d, r11d
 1      0     0.20                        mov	ecx, r9d
 3      5     1.00                        shld	r11, r10, cl
 1      3     0.50                        shlx	rcx, r10, r9
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	r11, rcx
 1      1     0.50                        cmovne	rcx, rsi
 0      1     0.00                        mov	r9, r11
 1      1     0.50                        shr	r9
 0      1     0.00                        mov	rdi, r11
 1      3     1.00                        shld	rdi, rcx, 63
 0      1     0.00                        mov	rbx, r8
 1      1     0.50                        shr	rbx, 63
 1      1     0.20                        sub	rdi, rbx
 1      1     0.50                        sbb	r9, 0
 1      1     0.50                        cmovb	r9, rsi
 1      1     0.50                        cmovb	rdi, rsi
 1      1     0.20                        add	rdx, rdi
 1      1     0.50                        adc	r9, r8
 1      1     0.50                        jo	.LBB135_3
 1      2     0.20                        xor	r8d, r8d
 1      1     0.20                        neg	rcx
 1      1     0.50                        sbb	r8, r11
 1      2     0.20                        and	r9, r8
 1      2     0.20                        and	rdx, rcx
 2      12    0.50           *            mov	qword ptr [rax + 16], rdx
 2      12    0.50           *            mov	qword ptr [rax + 24], r9
 0      0     0.00                        jmp	.LBB135_4
 1      2     0.20                        xor	r10d, r10d
 2      12    0.50           *            mov	qword ptr [rax], r10
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rbx
 1      5     0.33    *                   pop	rdi
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
7.00   4.00   1.00   1.00   3.00   5.00   8.00   3.00   4.00   4.00   5.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rsi
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     push	rdi
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rbx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, r9b
1.00    -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB135_3
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	esi, esi
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r10d, 1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	r11d, r11d
1.00    -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
 -     1.00    -      -      -     1.00   1.00    -      -      -      -      -      -     shld	r11, r10, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	rcx, r10, r9
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r11, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rcx, rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, r11
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r9
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdi, r11
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     shld	rdi, rcx, 63
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rbx, r8
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	rbx, 63
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     sub	rdi, rbx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sbb	r9, 0
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovb	r9, rsi
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovb	rdi, rsi
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	rdx, rdi
1.00    -      -      -      -      -      -      -      -      -      -      -      -     adc	r9, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     jo	.LBB135_3
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	r8d, r8d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     neg	rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sbb	r8, r11
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	r9, r8
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	rdx, rcx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 16], rdx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 24], r9
 -      -      -      -      -      -      -      -      -      -      -      -      -     jmp	.LBB135_4
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	r10d, r10d
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax], r10
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 8], 0
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rbx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -     pop	rdi
 -      -     1.00    -      -      -      -      -      -      -      -      -      -     pop	rsi
```
## `checked_round_to_multiple_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      12
Total uOps:        17

Dispatch Width:    6
uOps Per Cycle:    1.42
IPC:               1.25
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 7
 1      1     0.50                        ja	.LBB144_2
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        mov	al, 1
 1      1     0.20                        mov	r9b, 1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r9b, cl
 1      0     0.20                        mov	edx, r9d
 1      1     0.50                        shr	dl
 1      1     0.20                        add	dl, r8b
 1      1     0.50                        jb	.LBB144_2
 1      1     0.20                        neg	r9b
 1      2     0.20                        and	dl, r9b
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   2.00    -      -      -     3.00   4.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 7
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB144_2
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	al, 1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r9b, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r9b, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, r9d
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	dl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	dl, r8b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     jb	.LBB144_2
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     neg	r9b
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	dl, r9b
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
```
## `checked_round_to_multiple_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      13
Total Cycles:      12
Total uOps:        14

Dispatch Width:    6
uOps Per Cycle:    1.17
IPC:               1.08
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 15
 1      1     0.50                        ja	.LBB141_2
 1      1     0.20                        mov	eax, 1
 1      3     0.50                        shlx	eax, eax, edx
 1      1     0.20                        movzx	edx, ax
 1      1     0.50                        shr	edx
 1      1     0.20                        add	dx, cx
 1      1     0.50                        jb	.LBB141_2
 1      1     0.20                        neg	eax
 1      2     0.20                        and	edx, eax
 1      1     0.20                        mov	ax, 1
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   2.00    -      -      -     2.00   3.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 15
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB141_2
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	eax, eax, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movzx	edx, ax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shr	edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	dx, cx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jb	.LBB141_2
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     neg	eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	edx, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ax, 1
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
```
## `checked_round_to_multiple_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      13
Total Cycles:      12
Total uOps:        14

Dispatch Width:    6
uOps Per Cycle:    1.17
IPC:               1.08
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 31
 1      1     0.50                        ja	.LBB142_2
 1      1     0.20                        mov	r8d, 1
 1      3     0.50                        shlx	r8d, r8d, edx
 1      0     0.20                        mov	edx, r8d
 1      1     0.50                        shr	edx
 1      1     0.20                        add	edx, ecx
 1      1     0.50                        jb	.LBB142_2
 1      1     0.20                        neg	r8d
 1      2     0.20                        and	edx, r8d
 1      1     0.20                        mov	eax, 1
 2      7     0.50                  U     ret

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   2.00    -      -      -     2.00   3.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 31
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB142_2
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8d, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	r8d, r8d, edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	edx, r8d
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shr	edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	edx, ecx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jb	.LBB142_2
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     neg	r8d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	edx, r8d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	eax, 1
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
```
## `checked_round_to_multiple_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      12
Total Cycles:      12
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 63
 1      1     0.50                        ja	.LBB143_2
 1      1     0.20                        mov	eax, 1
 1      3     0.50                        shlx	r8, rax, rdx
 0      1     0.00                        mov	rdx, r8
 1      1     0.50                        shr	rdx
 1      1     0.20                        add	rdx, rcx
 1      1     0.50                        jb	.LBB143_2
 1      1     0.20                        neg	r8
 1      2     0.20                        and	rdx, r8
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   1.00    -      -      -     2.00   3.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 63
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB143_2
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	r8, rax, rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shr	rdx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	rdx, rcx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jb	.LBB143_2
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     neg	r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	rdx, r8
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
```
## `checked_round_to_multiple_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      35
Total Cycles:      34
Total uOps:        42

Dispatch Width:    6
uOps Per Cycle:    1.24
IPC:               1.03
Block RThroughput: 7.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	rsi
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB140_3
 1      2     0.20                        xor	esi, esi
 1      1     0.20                        mov	r10d, 1
 1      2     0.20                        xor	r11d, r11d
 1      0     0.20                        mov	ecx, r9d
 3      5     1.00                        shld	r11, r10, cl
 1      3     0.50                        shlx	rcx, r10, r9
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	r11, rcx
 1      1     0.50                        cmovne	rcx, rsi
 0      1     0.00                        mov	r9, r11
 1      1     0.50                        shr	r9
 0      1     0.00                        mov	rsi, r11
 1      3     1.00                        shld	rsi, rcx, 63
 1      1     0.20                        add	rsi, rdx
 1      1     0.50                        adc	r9, r8
 1      1     0.50                        jb	.LBB140_3
 1      2     0.20                        xor	edx, edx
 1      1     0.20                        neg	rcx
 1      1     0.50                        sbb	rdx, r11
 1      2     0.20                        and	r9, rdx
 1      2     0.20                        and	rsi, rcx
 2      12    0.50           *            mov	qword ptr [rax + 16], rsi
 2      12    0.50           *            mov	qword ptr [rax + 24], r9
 2      12    0.50           *            mov	qword ptr [rax], r10
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rsi
 2      7     0.50                  U     ret
 1      2     0.20                        xor	r10d, r10d
 2      12    0.50           *            mov	qword ptr [rax], r10
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
6.00   4.00   1.00   1.00   3.00   4.00   6.00   3.00   4.00   4.00   5.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, r9b
1.00    -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB140_3
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	esi, esi
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r10d, 1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	r11d, r11d
1.00    -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
 -     1.00    -      -      -     1.00   1.00    -      -      -      -      -      -     shld	r11, r10, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	rcx, r10, r9
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r11, rcx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rcx, rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, r11
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r9
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rsi, r11
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     shld	rsi, rcx, 63
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	rsi, rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     adc	r9, r8
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jb	.LBB140_3
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	edx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     neg	rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sbb	rdx, r11
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	r9, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	rsi, rcx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 16], rsi
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 24], r9
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax], r10
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 8], 0
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rsi
 -      -      -     1.00    -      -     1.00    -      -      -      -      -      -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	r10d, r10d
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax], r10
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 8], 0
 -      -     1.00    -      -      -      -      -      -      -      -      -      -     pop	rsi
```
## `div_ceil_i8_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      11
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.82
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      1     0.20                        mov	r8b, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r8b, cl
 1      1     0.20                        not	r8b
 1      2     0.20                        and	r8b, al
 2      2     1.00                        sar	al, cl
 1      1     0.20                        cmp	r8b, 1
 1      1     0.50                        sbb	al, -1

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   1.00    -      -      -     2.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8b, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r8b, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	r8b
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	r8b, al
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sar	al, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	r8b, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sbb	al, -1
```
## `div_ceil_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      12
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.83
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      2     0.20                        and	dl, 7
 1      1     0.20                        mov	r8b, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r8b, cl
 1      1     0.20                        not	r8b
 1      2     0.20                        and	r8b, al
 2      2     1.00                        sar	al, cl
 1      1     0.20                        cmp	r8b, 1
 1      1     0.50                        sbb	al, -1

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   2.00    -      -      -     2.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 7
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8b, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r8b, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	r8b
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	r8b, al
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sar	al, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	r8b, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sbb	al, -1
```
## `div_ceil_i16_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      8
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.63
IPC:               0.63
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.33                        movsx	eax, cx
 1      3     0.50                        sarx	eax, eax, edx
 1      3     1.00                        bzhi	ecx, ecx, edx
 1      1     0.20                        cmp	cx, 1
 1      1     0.50                        sbb	ax, -1

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -     1.00   1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movsx	eax, cx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	eax, eax, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	ecx, ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	cx, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sbb	ax, -1
```
## `div_ceil_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      10
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.60
IPC:               0.60
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.33                        movsx	eax, cx
 1      2     0.20                        and	dl, 15
 1      3     0.50                        sarx	eax, eax, edx
 1      3     1.00                        bzhi	ecx, ecx, edx
 1      1     0.20                        cmp	cx, 1
 1      1     0.50                        sbb	ax, -1

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -     1.00   2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movsx	eax, cx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 15
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sarx	eax, eax, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	ecx, ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	cx, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sbb	ax, -1
```
## `div_ceil_i32_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      8
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.50
IPC:               0.50
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      3     0.50                        sarx	eax, ecx, edx
 1      3     1.00                        bzhi	ecx, ecx, edx
 1      1     0.20                        cmp	ecx, 1
 1      1     0.50                        sbb	eax, -1

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -      -     1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	eax, ecx, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	ecx, ecx, edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	ecx, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sbb	eax, -1
```
## `div_ceil_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      10
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.60
IPC:               0.60
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, edx
 1      2     0.20                        and	dl, 31
 1      3     0.50                        sarx	eax, ecx, eax
 1      3     1.00                        bzhi	ecx, ecx, edx
 1      1     0.20                        cmp	ecx, 1
 1      1     0.50                        sbb	eax, -1

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -     1.00   2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 31
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sarx	eax, ecx, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	ecx, ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	ecx, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sbb	eax, -1
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
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      3     0.50                        sarx	rax, rcx, rdx
 1      3     1.00                        bzhi	rcx, rcx, rdx
 1      1     0.20                        cmp	rcx, 1
 1      1     0.50                        sbb	rax, -1

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -      -     1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	rax, rcx, rdx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	rcx, rcx, rdx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	rcx, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sbb	rax, -1
```
## `div_ceil_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      10
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.60
IPC:               0.60
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, edx
 1      2     0.20                        and	dl, 63
 1      3     0.50                        sarx	rax, rcx, rax
 1      3     1.00                        bzhi	rcx, rcx, rdx
 1      1     0.20                        cmp	rcx, 1
 1      1     0.50                        sbb	rax, -1

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -     1.00   2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 63
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sarx	rax, rcx, rax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	rcx, rcx, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	rcx, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sbb	rax, -1
```
## `div_ceil_i128_pow2`, `div_ceil_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      25
Total Cycles:      18
Total uOps:        25

Dispatch Width:    6
uOps Per Cycle:    1.39
IPC:               1.39
Block RThroughput: 5.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	rsi
 0      1     0.00                        mov	rax, rcx
 0      1     0.00                        mov	r9, rcx
 1      0     0.20                        mov	ecx, r8d
 3      5     1.00                        shrd	r9, rdx, cl
 1      1     0.20                        mov	r10, -1
 1      3     0.50                        shlx	r11, r10, r8
 0      1     0.00                        mov	rcx, rdx
 1      1     0.50                        sar	rcx, 63
 1      2     0.20                        xor	esi, esi
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmove	rsi, r11
 1      1     0.50                        cmovne	r10, r11
 1      3     0.50                        sarx	r8, rdx, r8
 1      1     0.50                        cmovne	r9, r8
 1      1     0.50                        cmove	rcx, r8
 1      2     0.33                        andn	rdx, r10, rdx
 1      2     0.33                        andn	r8, rsi, rax
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        or	r8, rdx
 2      2     1.00                        setne	al
 1      1     0.20                        add	rax, r9
 1      1     0.50                        adc	rcx, 0
 0      1     0.00                        mov	rdx, rcx
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
6.00   3.00    -      -      -     3.00   6.00    -     1.00   1.00   4.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, r8d
 -     1.00    -      -      -     1.00   1.00    -      -      -      -      -      -     shrd	r9, rdx, cl
1.00    -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	r11, r10, r8
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rcx, rdx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	rcx, 63
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	esi, esi
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r8b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	rsi, r11
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r10, r11
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	r8, rdx, r8
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r9, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	rcx, r8
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     andn	rdx, r10, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     andn	r8, rsi, rax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     or	r8, rdx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setne	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	rax, r9
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     adc	rcx, 0
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, rcx
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rsi
```
## `div_ceil_u8_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      11
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.82
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      1     0.20                        mov	r8b, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r8b, cl
 1      1     0.20                        not	r8b
 1      2     0.20                        and	r8b, al
 2      2     1.00                        shr	al, cl
 1      1     0.20                        cmp	r8b, 1
 1      1     0.50                        sbb	al, -1

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   1.00    -      -      -     2.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8b, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r8b, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	r8b
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	r8b, al
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	al, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	r8b, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sbb	al, -1
```
## `div_ceil_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      12
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.83
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      2     0.20                        and	dl, 7
 1      1     0.20                        mov	r8b, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r8b, cl
 1      1     0.20                        not	r8b
 1      2     0.20                        and	r8b, al
 2      2     1.00                        shr	al, cl
 1      1     0.20                        cmp	r8b, 1
 1      1     0.50                        sbb	al, -1

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   2.00    -      -      -     2.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 7
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8b, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r8b, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	r8b
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	r8b, al
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	al, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	r8b, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sbb	al, -1
```
## `div_ceil_u16_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      8
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.63
IPC:               0.63
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        movzx	eax, cx
 1      3     0.50                        shrx	eax, eax, edx
 1      3     1.00                        bzhi	ecx, ecx, edx
 1      1     0.20                        cmp	cx, 1
 1      1     0.50                        sbb	ax, -1

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -     1.00   1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	eax, cx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shrx	eax, eax, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	ecx, ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	cx, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sbb	ax, -1
```
## `div_ceil_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      10
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.60
IPC:               0.60
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        movzx	eax, cx
 1      2     0.20                        and	dl, 15
 1      3     0.50                        shrx	eax, eax, edx
 1      3     1.00                        bzhi	ecx, ecx, edx
 1      1     0.20                        cmp	cx, 1
 1      1     0.50                        sbb	ax, -1

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -     1.00   2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	eax, cx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 15
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shrx	eax, eax, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	ecx, ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	cx, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sbb	ax, -1
```
## `div_ceil_u32_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      8
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.50
IPC:               0.50
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      3     0.50                        shrx	eax, ecx, edx
 1      3     1.00                        bzhi	ecx, ecx, edx
 1      1     0.20                        cmp	ecx, 1
 1      1     0.50                        sbb	eax, -1

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -      -     1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shrx	eax, ecx, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	ecx, ecx, edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	ecx, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sbb	eax, -1
```
## `div_ceil_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      10
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.60
IPC:               0.60
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, edx
 1      2     0.20                        and	dl, 31
 1      3     0.50                        shrx	eax, ecx, eax
 1      3     1.00                        bzhi	ecx, ecx, edx
 1      1     0.20                        cmp	ecx, 1
 1      1     0.50                        sbb	eax, -1

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -     1.00   2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 31
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shrx	eax, ecx, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	ecx, ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	ecx, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sbb	eax, -1
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
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      3     0.50                        shrx	rax, rcx, rdx
 1      3     1.00                        bzhi	rcx, rcx, rdx
 1      1     0.20                        cmp	rcx, 1
 1      1     0.50                        sbb	rax, -1

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -      -     1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shrx	rax, rcx, rdx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	rcx, rcx, rdx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	rcx, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sbb	rax, -1
```
## `div_ceil_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      10
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.60
IPC:               0.60
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, edx
 1      2     0.20                        and	dl, 63
 1      3     0.50                        shrx	rax, rcx, rax
 1      3     1.00                        bzhi	rcx, rcx, rdx
 1      1     0.20                        cmp	rcx, 1
 1      1     0.50                        sbb	rax, -1

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -     1.00   2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 63
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shrx	rax, rcx, rax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	rcx, rcx, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	rcx, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sbb	rax, -1
```
## `div_ceil_u128_pow2`, `div_ceil_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      23
Total Cycles:      17
Total uOps:        24

Dispatch Width:    6
uOps Per Cycle:    1.41
IPC:               1.35
Block RThroughput: 5.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	rsi
 0      1     0.00                        mov	rax, rcx
 1      1     0.20                        mov	r9, -1
 1      3     0.50                        shlx	r10, r9, r8
 0      1     0.00                        mov	r11, rcx
 1      0     0.20                        mov	ecx, r8d
 3      5     1.00                        shrd	r11, rdx, cl
 1      2     0.20                        xor	esi, esi
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	r9, r10
 1      1     0.50                        cmovne	r10, rsi
 1      3     0.50                        shrx	rcx, rdx, r8
 1      1     0.50                        cmovne	r11, rcx
 1      1     0.50                        cmovne	rcx, rsi
 1      2     0.33                        andn	rdx, r9, rdx
 1      2     0.33                        andn	r8, r10, rax
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        or	r8, rdx
 2      2     1.00                        setne	al
 1      1     0.20                        add	rax, r11
 1      1     0.50                        adc	rcx, 0
 0      1     0.00                        mov	rdx, rcx
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   3.00    -      -      -     3.00   6.00    -     1.00   1.00   4.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r9, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	r10, r9, r8
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, r8d
 -     1.00    -      -      -     1.00   1.00    -      -      -      -      -      -     shrd	r11, rdx, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	esi, esi
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r9, r10
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r10, rsi
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shrx	rcx, rdx, r8
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r11, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rcx, rsi
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     andn	rdx, r9, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     andn	r8, r10, rax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     or	r8, rdx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setne	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	rax, r11
1.00    -      -      -      -      -      -      -      -      -      -      -      -     adc	rcx, 0
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, rcx
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rsi
```
## `div_floor_i8_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      5
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.80
IPC:               0.60
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        sar	al, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00    -      -      -      -      -     1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	al, cl
```
## `div_floor_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      7
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.71
IPC:               0.57
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      2     0.20                        and	dl, 7
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        sar	al, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00    -      -      -      -     1.00   1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 7
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	al, cl
```
## `div_floor_i16_pow2`
```asm
Iterations:        1
Instructions:      2
Total Cycles:      7
Total uOps:        2

Dispatch Width:    6
uOps Per Cycle:    0.29
IPC:               0.29
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.33                        movsx	eax, cx
 1      3     0.50                        sarx	eax, eax, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
 -      -      -      -      -      -     1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movsx	eax, cx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	eax, eax, edx
```
## `div_floor_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      8
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.38
IPC:               0.38
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.33                        movsx	eax, cx
 1      2     0.20                        and	dl, 15
 1      3     0.50                        sarx	eax, eax, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00    -      -      -      -      -     1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movsx	eax, cx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 15
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sarx	eax, eax, edx
```
## `div_floor_i32_pow2`, `div_floor_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      1
Total Cycles:      6
Total uOps:        1

Dispatch Width:    6
uOps Per Cycle:    0.17
IPC:               0.17
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      3     0.50                        sarx	eax, ecx, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	eax, ecx, edx
```
## `div_floor_i64_pow2`, `div_floor_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      1
Total Cycles:      6
Total uOps:        1

Dispatch Width:    6
uOps Per Cycle:    0.17
IPC:               0.17
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      3     0.50                        sarx	rax, rcx, rdx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	rax, rcx, rdx
```
## `div_floor_i128_pow2`, `div_floor_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      10
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    0.90
IPC:               0.80
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      0     0.20                        mov	ecx, r8d
 3      5     1.00                        shrd	rax, rdx, cl
 1      3     0.50                        sarx	rcx, rdx, r8
 1      1     0.50                        sar	rdx, 63
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	rax, rcx
 1      1     0.50                        cmove	rdx, rcx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, r8d
 -     1.00    -      -      -     1.00   1.00    -      -      -      -      -      -     shrd	rax, rdx, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	rcx, rdx, r8
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	rdx, 63
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rax, rcx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	rdx, rcx
```
## `div_floor_u8_pow2`, `div_u8_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      5
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.80
IPC:               0.60
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shr	al, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00    -      -      -      -      -     1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	al, cl
```
## `div_floor_u8_unb_pow2`, `div_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      7
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.71
IPC:               0.57
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      2     0.20                        and	dl, 7
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shr	al, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00    -      -      -      -     1.00   1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 7
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	al, cl
```
## `div_floor_u16_pow2`, `div_u16_pow2`
```asm
Iterations:        1
Instructions:      2
Total Cycles:      7
Total uOps:        2

Dispatch Width:    6
uOps Per Cycle:    0.29
IPC:               0.29
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        movzx	eax, cx
 1      3     0.50                        shrx	eax, eax, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
 -      -      -      -      -      -     1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	eax, cx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shrx	eax, eax, edx
```
## `div_floor_u16_unb_pow2`, `div_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      8
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.38
IPC:               0.38
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        movzx	eax, cx
 1      2     0.20                        and	dl, 15
 1      3     0.50                        shrx	eax, eax, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00    -      -      -      -      -     1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	eax, cx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 15
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shrx	eax, eax, edx
```
## `div_floor_u32_pow2`, `div_floor_u32_unb_pow2`, `div_u32_pow2`, `div_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      1
Total Cycles:      6
Total uOps:        1

Dispatch Width:    6
uOps Per Cycle:    0.17
IPC:               0.17
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      3     0.50                        shrx	eax, ecx, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shrx	eax, ecx, edx
```
## `div_floor_u64_pow2`, `div_floor_u64_unb_pow2`, `div_u64_pow2`, `div_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      1
Total Cycles:      6
Total uOps:        1

Dispatch Width:    6
uOps Per Cycle:    0.17
IPC:               0.17
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      3     0.50                        shrx	rax, rcx, rdx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shrx	rax, rcx, rdx
```
## `div_floor_u128_pow2`, `div_floor_u128_unb_pow2`, `div_u128_pow2`, `div_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      10
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    0.90
IPC:               0.80
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      0     0.20                        mov	ecx, r8d
 3      5     1.00                        shrd	rax, rdx, cl
 1      3     0.50                        shrx	rcx, rdx, r8
 1      2     0.20                        xor	edx, edx
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	rax, rcx
 1      1     0.50                        cmove	rdx, rcx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, r8d
1.00   1.00    -      -      -      -      -      -      -      -     1.00    -      -     shrd	rax, rdx, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shrx	rcx, rdx, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	edx, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rax, rcx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	rdx, rcx
```
## `div_i8_pow2`
```asm
Iterations:        1
Instructions:      12
Total Cycles:      12
Total uOps:        14

Dispatch Width:    6
uOps Per Cycle:    1.17
IPC:               1.00
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        mov	r9b, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r9b, cl
 1      1     0.20                        not	r9b
 1      0     0.20                        mov	eax, r8d
 1      1     0.50                        sar	al, 7
 1      2     0.20                        and	al, r9b
 1      1     0.20                        add	al, r8b
 1      2     0.20                        and	dl, 7
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        sar	al, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   2.00    -      -      -     2.00   4.00    -      -      -     3.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r9b, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r9b, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	r9b
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	eax, r8d
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sar	al, 7
1.00    -      -      -      -      -      -      -      -      -      -      -      -     and	al, r9b
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	al, r8b
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	dl, 7
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sar	al, cl
```
## `div_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      13
Total uOps:        13

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.85
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, ecx
 1      2     0.20                        and	dl, 7
 1      1     0.20                        mov	r9b, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r9b, cl
 1      1     0.20                        not	r9b
 1      0     0.20                        mov	eax, r8d
 1      1     0.50                        sar	al, 7
 1      2     0.20                        and	al, r9b
 1      1     0.20                        add	al, r8b
 2      2     1.00                        sar	al, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   2.00    -      -      -     2.00   5.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 7
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r9b, -1
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r9b, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	r9b
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	eax, r8d
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	al, 7
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	al, r9b
1.00    -      -      -      -      -      -      -      -      -      -      -      -     add	al, r8b
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sar	al, cl
```
## `div_i16_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      13
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    0.54
IPC:               0.54
Block RThroughput: 1.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.33                        movsx	eax, cx
 1      1     0.50                        sar	eax, 15
 1      3     1.00                        bzhi	eax, eax, edx
 1      1     0.20                        add	eax, ecx
 1      1     0.50                        cwde
 1      2     0.20                        and	dl, 15
 1      3     0.50                        sarx	eax, eax, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -     2.00   2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movsx	eax, cx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	eax, 15
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	eax, eax, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	eax, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cwde
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 15
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	eax, eax, edx
```
## `div_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      13
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    0.54
IPC:               0.54
Block RThroughput: 1.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.33                        movsx	eax, cx
 1      2     0.20                        and	dl, 15
 1      1     0.50                        sar	eax, 15
 1      3     1.00                        bzhi	eax, eax, edx
 1      1     0.20                        add	eax, ecx
 1      1     0.50                        cwde
 1      3     0.50                        sarx	eax, eax, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -     2.00   2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movsx	eax, cx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 15
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	eax, 15
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	eax, eax, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	eax, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cwde
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	eax, eax, edx
```
## `div_i32_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      11
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.45
IPC:               0.45
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      1     0.50                        sar	eax, 31
 1      3     1.00                        bzhi	eax, eax, edx
 1      1     0.20                        add	eax, ecx
 1      3     0.50                        sarx	eax, eax, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -     1.00   1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sar	eax, 31
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	eax, eax, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	eax, ecx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sarx	eax, eax, edx
```
## `div_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      12
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    0.58
IPC:               0.58
Block RThroughput: 1.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, edx
 1      2     0.20                        and	dl, 31
 1      0     0.20                        mov	r8d, ecx
 1      1     0.50                        sar	r8d, 31
 1      3     1.00                        bzhi	edx, r8d, edx
 1      1     0.20                        add	edx, ecx
 1      3     0.50                        sarx	eax, edx, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -     1.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 31
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8d, ecx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	r8d, 31
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	edx, r8d, edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	edx, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	eax, edx, eax
```
## `div_i64_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      12
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.33
IPC:               0.42
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      1     0.50                        sar	rax, 63
 1      3     1.00                        bzhi	rax, rax, rdx
 1      1     0.20                        add	rax, rcx
 1      3     0.50                        sarx	rax, rax, rdx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -      -     1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sar	rax, 63
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	rax, rax, rdx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	rax, rcx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sarx	rax, rax, rdx
```
## `div_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      12
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.50
IPC:               0.58
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, edx
 1      2     0.20                        and	dl, 63
 0      1     0.00                        mov	r8, rcx
 1      1     0.50                        sar	r8, 63
 1      3     1.00                        bzhi	rdx, r8, rdx
 1      1     0.20                        add	rdx, rcx
 1      3     0.50                        sarx	rax, rdx, rax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -     1.00   2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 63
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	r8, 63
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	rdx, r8, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	rdx, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	rax, rdx, rax
```
## `div_i128_pow2`, `div_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      21
Total Cycles:      18
Total uOps:        21

Dispatch Width:    6
uOps Per Cycle:    1.17
IPC:               1.17
Block RThroughput: 5.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        mov	rax, -1
 1      3     0.50                        shlx	r9, rax, r8
 1      2     0.20                        xor	r10d, r10d
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmove	r10, r9
 1      1     0.50                        cmovne	rax, r9
 0      1     0.00                        mov	r9, rdx
 1      1     0.50                        sar	r9, 63
 1      2     0.33                        andn	r11, rax, r9
 1      2     0.33                        andn	rax, r10, r9
 1      1     0.20                        add	rax, rcx
 1      1     0.50                        adc	r11, rdx
 0      1     0.00                        mov	rdx, r11
 1      1     0.50                        sar	rdx, 63
 1      2     0.20                        test	r8b, 64
 1      3     0.50                        sarx	r9, r11, r8
 1      1     0.50                        cmove	rdx, r9
 1      0     0.20                        mov	ecx, r8d
 3      5     1.00                        shrd	rax, r11, cl
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	rax, r9

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   3.00    -      -      -     3.00   6.00    -      -      -     4.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	rax, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	r9, rax, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	r10d, r10d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r8b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	r10, r9
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rax, r9
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sar	r9, 63
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     andn	r11, rax, r9
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     andn	rax, r10, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	rax, rcx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     adc	r11, rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r11
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sar	rdx, 63
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	r9, r11, r8
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	rdx, r9
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, r8d
1.00   1.00    -      -      -      -      -      -      -      -     1.00    -      -     shrd	rax, r11, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rax, r9
```
## `div_round_i8_pow2`
```asm
Iterations:        1
Instructions:      18
Total Cycles:      14
Total uOps:        20

Dispatch Width:    6
uOps Per Cycle:    1.43
IPC:               1.29
Block RThroughput: 3.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        mov	r9b, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r9b, cl
 1      0     0.20                        mov	eax, r8d
 2      2     1.00                        sar	al, cl
 1      1     0.20                        not	r9b
 1      2     0.20                        and	r9b, r8b
 1      1     0.50                        shr	r8b, 7
 1      2     0.20                        xor	ecx, ecx
 1      1     0.20                        sub	r9b, r8b
 1      0     0.20                        movzx	r8d, r9b
 1      1     0.50                        cmovb	r8d, ecx
 1      1     0.20                        add	r8b, r8b
 1      0     0.20                        movzx	ecx, r8b
 1      0     0.20                        movzx	edx, dl
 1      1     1.00                        bt	ecx, edx
 1      1     0.50                        adc	al, 0

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   4.00    -      -      -     3.00   5.00    -      -      -     4.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r9b, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r9b, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	eax, r8d
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sar	al, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	r9b
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	r9b, r8b
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r8b, 7
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	ecx, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sub	r9b, r8b
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movzx	r8d, r9b
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovb	r8d, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	r8b, r8b
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	ecx, r8b
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movzx	edx, dl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bt	ecx, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     adc	al, 0
```
## `div_round_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      19
Total Cycles:      15
Total uOps:        21

Dispatch Width:    6
uOps Per Cycle:    1.40
IPC:               1.27
Block RThroughput: 3.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, ecx
 1      2     0.20                        and	dl, 7
 1      1     0.20                        mov	r9b, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r9b, cl
 1      0     0.20                        mov	eax, r8d
 2      2     1.00                        sar	al, cl
 1      1     0.20                        not	r9b
 1      2     0.20                        and	r9b, r8b
 1      1     0.50                        shr	r8b, 7
 1      2     0.20                        xor	ecx, ecx
 1      1     0.20                        sub	r9b, r8b
 1      0     0.20                        movzx	r8d, r9b
 1      1     0.50                        cmovb	r8d, ecx
 1      1     0.20                        add	r8b, r8b
 1      0     0.20                        movzx	ecx, r8b
 1      0     0.20                        movzx	edx, dl
 1      1     1.00                        bt	ecx, edx
 1      1     0.50                        adc	al, 0

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   4.00    -      -      -     3.00   5.00    -      -      -     4.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 7
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r9b, -1
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r9b, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	eax, r8d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	al, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	r9b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	r9b, r8b
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r8b, 7
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     sub	r9b, r8b
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movzx	r8d, r9b
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovb	r8d, ecx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	r8b, r8b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     movzx	ecx, r8b
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movzx	edx, dl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bt	ecx, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     adc	al, 0
```
## `div_round_i16_pow2`
```asm
Iterations:        1
Instructions:      12
Total Cycles:      11
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.09
IPC:               1.09
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        movzx	r8d, cx
 1      1     0.33                        movsx	eax, r8w
 1      3     0.50                        sarx	eax, eax, edx
 1      3     1.00                        bzhi	ecx, ecx, edx
 1      1     0.50                        shr	r8d, 15
 1      2     0.20                        xor	r9d, r9d
 1      1     0.20                        sub	cx, r8w
 1      1     0.50                        cmovb	ecx, r9d
 1      1     0.20                        add	ecx, ecx
 1      0     0.20                        movzx	edx, dl
 1      1     1.00                        bt	ecx, edx
 1      1     0.50                        adc	ax, 0

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   3.00    -      -      -     2.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	r8d, cx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movsx	eax, r8w
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	eax, eax, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	ecx, ecx, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r8d, 15
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	r9d, r9d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     sub	cx, r8w
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovb	ecx, r9d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	ecx, ecx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	edx, dl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bt	ecx, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     adc	ax, 0
```
## `div_round_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      13
Total Cycles:      13
Total uOps:        13

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 2.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        movzx	r8d, cx
 1      1     0.33                        movsx	eax, r8w
 1      2     0.20                        and	dl, 15
 1      3     0.50                        sarx	eax, eax, edx
 1      3     1.00                        bzhi	ecx, ecx, edx
 1      1     0.50                        shr	r8d, 15
 1      2     0.20                        xor	r9d, r9d
 1      1     0.20                        sub	cx, r8w
 1      1     0.50                        cmovb	ecx, r9d
 1      1     0.20                        add	ecx, ecx
 1      0     0.20                        movzx	edx, dl
 1      1     1.00                        bt	ecx, edx
 1      1     0.50                        adc	ax, 0

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   3.00    -      -      -     2.00   3.00    -      -      -     3.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	r8d, cx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movsx	eax, r8w
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 15
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	eax, eax, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	ecx, ecx, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r8d, 15
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	r9d, r9d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     sub	cx, r8w
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovb	ecx, r9d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	ecx, ecx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	edx, dl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bt	ecx, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     adc	ax, 0
```
## `div_round_i32_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      11
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    0.91
IPC:               0.91
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      3     0.50                        sarx	eax, ecx, edx
 1      3     1.00                        bzhi	r8d, ecx, edx
 1      1     0.50                        shr	ecx, 31
 1      2     0.20                        xor	r9d, r9d
 1      1     0.20                        sub	r8d, ecx
 1      1     0.50                        cmovb	r8d, r9d
 1      1     0.20                        add	r8d, r8d
 1      0     0.20                        movzx	ecx, dl
 1      1     1.00                        bt	r8d, ecx
 1      1     0.50                        adc	eax, 0

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   2.00    -      -      -     2.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	eax, ecx, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	r8d, ecx, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	ecx, 31
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	r9d, r9d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     sub	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovb	r8d, r9d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	r8d, r8d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movzx	ecx, dl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bt	r8d, ecx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     adc	eax, 0
```
## `div_round_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      13
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    0.85
IPC:               0.85
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      3     0.50                        sarx	eax, ecx, edx
 1      0     0.20                        movzx	r8d, dl
 1      2     0.20                        and	dl, 31
 1      3     1.00                        bzhi	edx, ecx, edx
 1      1     0.50                        shr	ecx, 31
 1      2     0.20                        xor	r9d, r9d
 1      1     0.20                        sub	edx, ecx
 1      1     0.50                        cmovb	edx, r9d
 1      1     0.20                        add	edx, edx
 1      1     1.00                        bt	edx, r8d
 1      1     0.50                        adc	eax, 0

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   3.00    -      -      -     2.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	eax, ecx, edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	r8d, dl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	dl, 31
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	edx, ecx, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	ecx, 31
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	r9d, r9d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     sub	edx, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovb	edx, r9d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	edx, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bt	edx, r8d
1.00    -      -      -      -      -      -      -      -      -      -      -      -     adc	eax, 0
```
## `div_round_i64_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      13
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    0.77
IPC:               0.77
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      3     0.50                        sarx	rax, rcx, rdx
 1      3     1.00                        bzhi	r8, rcx, rdx
 1      1     0.50                        shr	rcx, 63
 1      2     0.20                        xor	r9d, r9d
 1      1     0.20                        sub	r8, rcx
 1      1     0.50                        cmovae	r9, r8
 1      1     0.20                        add	r9, r9
 1      0     0.20                        movzx	ecx, dl
 1      3     1.00                        bt	r9, rcx
 1      1     0.50                        adc	rax, 0

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   2.00    -      -      -     2.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	rax, rcx, rdx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	r8, rcx, rdx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	rcx, 63
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	r9d, r9d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     sub	r8, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovae	r9, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	r9, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movzx	ecx, dl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bt	r9, rcx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     adc	rax, 0
```
## `div_round_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      15
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    0.73
IPC:               0.73
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      3     0.50                        sarx	rax, rcx, rdx
 1      0     0.20                        movzx	r8d, dl
 1      2     0.20                        and	dl, 63
 1      3     1.00                        bzhi	rdx, rcx, rdx
 1      1     0.50                        shr	rcx, 63
 1      2     0.20                        xor	r9d, r9d
 1      1     0.20                        sub	rdx, rcx
 1      1     0.50                        cmovae	r9, rdx
 1      1     0.20                        add	r9, r9
 1      3     1.00                        bt	r9, r8
 1      1     0.50                        adc	rax, 0

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   3.00    -      -      -     2.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	rax, rcx, rdx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	r8d, dl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	dl, 63
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	rdx, rcx, rdx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	rcx, 63
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	r9d, r9d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     sub	rdx, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovae	r9, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	r9, r9
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bt	r9, r8
1.00    -      -      -      -      -      -      -      -      -      -      -      -     adc	rax, 0
```
## `div_round_i128_pow2`, `div_round_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      34
Total Cycles:      27
Total uOps:        34

Dispatch Width:    6
uOps Per Cycle:    1.26
IPC:               1.26
Block RThroughput: 8.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	r9, rcx
 1      1     0.20                        mov	rax, -1
 1      3     0.50                        shlx	rcx, rax, r8
 1      2     0.20                        xor	r10d, r10d
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	rax, rcx
 1      1     0.50                        cmovne	rcx, r10
 1      2     0.33                        andn	rax, rax, rdx
 1      2     0.33                        andn	r11, rcx, r9
 0      1     0.00                        mov	rcx, rdx
 1      1     0.50                        shr	rcx, 63
 1      1     0.20                        sub	r11, rcx
 1      1     0.50                        sbb	rax, 0
 1      1     0.50                        cmovb	rax, r10
 1      1     0.50                        cmovb	r11, r10
 1      3     1.00                        shld	rax, r11, 1
 1      1     0.20                        add	r11, r11
 1      0     0.20                        mov	ecx, r8d
 3      5     1.00                        shrd	r11, rax, cl
 1      2     0.20                        test	r8b, 64
 1      3     0.50                        shrx	rax, rax, r8
 1      1     0.50                        cmove	rax, r11
 0      1     0.00                        mov	r10, rdx
 1      1     0.50                        sar	r10, 63
 1      2     0.20                        test	r8b, 64
 1      3     0.50                        sarx	r11, rdx, r8
 1      1     0.50                        cmove	r10, r11
 3      5     1.00                        shrd	r9, rdx, cl
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	r9, r11
 1      2     0.20                        and	eax, 1
 1      1     0.20                        add	rax, r9
 1      1     0.50                        adc	r10, 0
 0      1     0.00                        mov	rdx, r10

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
8.00   5.00    -      -      -     6.00   9.00    -      -      -     6.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	rax, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	rcx, rax, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	r10d, r10d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r8b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rcx, r10
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     andn	rax, rax, rdx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     andn	r11, rcx, r9
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rcx, rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shr	rcx, 63
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     sub	r11, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sbb	rax, 0
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovb	rax, r10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovb	r11, r10
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     shld	rax, r11, 1
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	r11, r11
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -     shrd	r11, rax, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shrx	rax, rax, r8
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	rax, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, rdx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	r10, 63
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	r11, rdx, r8
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	r10, r11
1.00   1.00    -      -      -      -      -      -      -      -     1.00    -      -     shrd	r9, rdx, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r9, r11
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	eax, 1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	rax, r9
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     adc	r10, 0
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r10
```
## `div_round_u8_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      11
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.82
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      1     0.20                        mov	r8b, 1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r8b, cl
 1      1     0.50                        shr	r8b
 1      2     0.20                        and	r8b, al
 2      2     1.00                        shr	al, cl
 1      1     0.20                        cmp	r8b, 1
 1      1     0.50                        sbb	al, -1

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   1.00    -      -      -     1.00   4.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8b, 1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r8b, cl
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r8b
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	r8b, al
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	al, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	r8b, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sbb	al, -1
```
## `div_round_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      12
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.83
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      2     0.20                        and	dl, 7
 1      1     0.20                        mov	r8b, 1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r8b, cl
 1      1     0.50                        shr	r8b
 1      2     0.20                        and	r8b, al
 2      2     1.00                        shr	al, cl
 1      1     0.20                        cmp	r8b, 1
 1      1     0.50                        sbb	al, -1

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   1.00    -      -      -     2.00   4.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 7
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8b, 1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r8b, cl
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r8b
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	r8b, al
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	al, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	r8b, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sbb	al, -1
```
## `div_round_u16_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      13
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    0.69
IPC:               0.69
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        movzx	eax, cx
 1      1     0.20                        mov	r8d, 1
 1      3     0.50                        shlx	r8d, r8d, edx
 1      1     0.20                        movzx	r8d, r8w
 1      1     0.50                        shr	r8d
 1      2     0.20                        and	r8d, ecx
 1      3     0.50                        shrx	eax, eax, edx
 1      1     0.20                        cmp	r8w, 1
 1      1     0.50                        sbb	ax, -1

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	eax, cx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8d, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	r8d, r8d, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movzx	r8d, r8w
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r8d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shrx	eax, eax, edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	r8w, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sbb	ax, -1
```
## `div_round_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      13
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    0.69
IPC:               0.69
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        movzx	eax, cx
 1      2     0.20                        and	dl, 15
 1      1     0.20                        mov	r8d, 1
 1      3     0.50                        shlx	r8d, r8d, edx
 1      1     0.50                        shr	r8d
 1      2     0.20                        and	r8d, ecx
 1      3     0.50                        shrx	eax, eax, edx
 1      1     0.20                        cmp	r8w, 1
 1      1     0.50                        sbb	ax, -1

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	eax, cx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 15
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8d, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	r8d, r8d, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r8d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shrx	eax, eax, edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	r8w, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sbb	ax, -1
```
## `div_round_u32_pow2`, `div_round_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      12
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    0.58
IPC:               0.58
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        mov	eax, 1
 1      3     0.50                        shlx	r8d, eax, edx
 1      1     0.50                        shr	r8d
 1      3     0.50                        shrx	eax, ecx, edx
 1      2     0.20                        and	r8d, ecx
 1      1     0.20                        cmp	r8d, 1
 1      1     0.50                        sbb	eax, -1

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	r8d, eax, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shr	r8d
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shrx	eax, ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	r8d, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	r8d, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sbb	eax, -1
```
## `div_round_u64_pow2`, `div_round_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      12
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    0.58
IPC:               0.58
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        mov	eax, 1
 1      3     0.50                        shlx	r8, rax, rdx
 1      1     0.50                        shr	r8
 1      3     0.50                        shrx	rax, rcx, rdx
 1      2     0.20                        and	r8, rcx
 1      1     0.20                        cmp	r8, 1
 1      1     0.50                        sbb	rax, -1

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	r8, rax, rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shr	r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shrx	rax, rcx, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	r8, rcx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	r8, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sbb	rax, -1
```
## `div_round_u128_pow2`, `div_round_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      37
Total Cycles:      29
Total uOps:        41

Dispatch Width:    6
uOps Per Cycle:    1.41
IPC:               1.28
Block RThroughput: 6.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	r14
 2      1     0.50           *            push	rsi
 2      1     0.50           *            push	rdi
 2      1     0.50           *            push	rbx
 1      0     0.20                        mov	r9d, r8d
 0      1     0.00                        mov	r8, rdx
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        xor	r10d, r10d
 1      2     0.20                        test	r9b, 64
 1      3     0.50                        shrx	rsi, rdx, r9
 0      1     0.00                        mov	rdx, rsi
 1      1     0.50                        cmovne	rdx, r10
 0      1     0.00                        mov	r11, rcx
 1      0     0.20                        mov	ecx, r9d
 3      5     1.00                        shrd	r11, r8, cl
 1      2     0.20                        test	r9b, 64
 1      1     0.20                        mov	edi, 1
 1      3     0.50                        shlx	rbx, rdi, r9
 0      1     0.00                        mov	r14, rbx
 1      1     0.50                        cmovne	r14, r10
 1      1     0.50                        cmovne	r11, rsi
 3      5     1.00                        shld	r10, rdi, cl
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	r10, rbx
 1      3     1.00                        shrd	r14, r10, 1
 1      1     0.50                        shr	r10
 1      2     0.20                        and	r10, r8
 1      2     0.20                        and	r14, rax
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        or	r14, r10
 2      2     1.00                        setne	al
 1      1     0.20                        add	rax, r11
 1      1     0.50                        adc	rdx, 0
 1      5     0.33    *                   pop	rbx
 1      5     0.33    *                   pop	rdi
 1      5     0.33    *                   pop	rsi
 1      5     0.33    *                   pop	r14

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
7.00   5.00   1.00   1.00   2.00   5.00   7.00   2.00   2.00   2.00   5.00   2.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	r14
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     push	rsi
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rdi
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     push	rbx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r9d, r8d
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	r10d, r10d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shrx	rsi, rdx, r9
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, rsi
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rdx, r10
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, rcx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
 -     1.00    -      -      -     1.00   1.00    -      -      -      -      -      -     shrd	r11, r8, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     mov	edi, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	rbx, rdi, r9
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r14, rbx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r14, r10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r11, rsi
1.00   1.00    -      -      -      -      -      -      -      -     1.00    -      -     shld	r10, rdi, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r10, rbx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     shrd	r14, r10, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shr	r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	r10, r8
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	r14, rax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     or	r14, r10
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setne	al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	rax, r11
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     adc	rdx, 0
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rbx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -     pop	rdi
 -      -     1.00    -      -      -      -      -      -      -      -      -      -     pop	rsi
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	r14
```
## `floor_to_multiple_i8_pow2`, `floor_to_multiple_u8_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      7
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.86
IPC:               0.57
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shr	al, cl
 2      2     1.00                        shl	al, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00    -      -      -      -      -     3.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	al, cl
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	al, cl
```
## `floor_to_multiple_i8_unb_pow2`, `floor_to_multiple_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      9
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    0.78
IPC:               0.56
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      2     0.20                        and	dl, 7
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shr	al, cl
 2      2     1.00                        shl	al, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00    -      -      -      -     1.00   3.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 7
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	al, cl
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	al, cl
```
## `floor_to_multiple_i16_pow2`, `floor_to_multiple_u16_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      10
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.30
IPC:               0.30
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        movzx	eax, cx
 1      3     0.50                        shrx	eax, eax, edx
 1      3     0.50                        shlx	eax, eax, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00    -      -      -      -      -     1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	eax, cx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shrx	eax, eax, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	eax, eax, edx
```
## `floor_to_multiple_i16_unb_pow2`, `floor_to_multiple_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      11
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.36
IPC:               0.36
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        movzx	eax, cx
 1      2     0.20                        and	dl, 15
 1      3     0.50                        shrx	eax, eax, edx
 1      3     0.50                        shlx	eax, eax, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00    -      -      -      -      -     2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	eax, cx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 15
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shrx	eax, eax, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	eax, eax, edx
```
## `floor_to_multiple_i32_pow2`, `floor_to_multiple_i32_unb_pow2`, `floor_to_multiple_u32_pow2`, `floor_to_multiple_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      2
Total Cycles:      9
Total uOps:        2

Dispatch Width:    6
uOps Per Cycle:    0.22
IPC:               0.22
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      3     0.50                        shrx	eax, ecx, edx
 1      3     0.50                        shlx	eax, eax, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00    -      -      -      -      -     1.00    -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shrx	eax, ecx, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	eax, eax, edx
```
## `floor_to_multiple_i64_pow2`, `floor_to_multiple_i64_unb_pow2`, `floor_to_multiple_u64_pow2`, `floor_to_multiple_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      2
Total Cycles:      9
Total uOps:        2

Dispatch Width:    6
uOps Per Cycle:    0.22
IPC:               0.22
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      3     0.50                        shrx	rax, rcx, rdx
 1      3     0.50                        shlx	rax, rax, rdx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00    -      -      -      -      -     1.00    -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shrx	rax, rcx, rdx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	rax, rax, rdx
```
## `floor_to_multiple_i128_pow2`, `floor_to_multiple_i128_unb_pow2`, `floor_to_multiple_u128_pow2`, `floor_to_multiple_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      10
Total uOps:        8

Dispatch Width:    6
uOps Per Cycle:    0.80
IPC:               0.80
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        mov	r9, -1
 1      3     0.50                        shlx	r10, r9, r8
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	r9, r10
 1      1     0.50                        cmove	rax, r10
 1      2     0.20                        and	rax, rcx
 1      2     0.20                        and	rdx, r9

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r9, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	r10, r9, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r9, r10
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	rax, r10
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	rdx, r9
```
## `is_multiple_of_i8_pow2`, `is_multiple_of_i8_unb_pow2`, `is_multiple_of_u8_pow2`, `is_multiple_of_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      11
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.55
IPC:               0.45
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        movzx	eax, cl
 1      2     0.20                        or	eax, 256
 1      3     1.00                        tzcnt	eax, eax
 1      1     0.20                        cmp	al, dl
 2      2     1.00                        setae	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	eax, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     or	eax, 256
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     tzcnt	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	al, dl
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setae	al
```
## `is_multiple_of_i16_pow2`, `is_multiple_of_i16_unb_pow2`, `is_multiple_of_u16_pow2`, `is_multiple_of_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      11
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.55
IPC:               0.45
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      2     0.20                        or	ecx, 65536
 1      3     1.00                        tzcnt	eax, ecx
 1      0     0.20                        movzx	ecx, dl
 1      1     0.20                        cmp	ax, cx
 2      2     1.00                        setae	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     or	ecx, 65536
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     tzcnt	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     movzx	ecx, dl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	ax, cx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setae	al
```
## `is_multiple_of_i32_pow2`, `is_multiple_of_i32_unb_pow2`, `is_multiple_of_u32_pow2`, `is_multiple_of_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      9
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.56
IPC:               0.44
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      3     1.00                        tzcnt	eax, ecx
 1      0     0.20                        movzx	ecx, dl
 1      1     0.20                        cmp	eax, ecx
 2      2     1.00                        setae	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -      -     1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     tzcnt	eax, ecx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	ecx, dl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	eax, ecx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setae	al
```
## `is_multiple_of_i64_pow2`, `is_multiple_of_i64_unb_pow2`, `is_multiple_of_u64_pow2`, `is_multiple_of_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      9
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.56
IPC:               0.44
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      3     1.00                        tzcnt	rax, rcx
 1      0     0.20                        movzx	ecx, dl
 1      1     0.20                        cmp	eax, ecx
 2      2     1.00                        setae	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -      -     1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     tzcnt	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	ecx, dl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	eax, ecx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setae	al
```
## `is_multiple_of_i128_pow2`, `is_multiple_of_i128_unb_pow2`, `is_multiple_of_u128_pow2`, `is_multiple_of_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      12
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    0.75
IPC:               0.67
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      3     1.00                        tzcnt	rax, rcx
 1      3     1.00                        tzcnt	rdx, rdx
 1      1     0.20                        add	edx, 64
 1      2     0.20                        test	rcx, rcx
 1      1     0.50                        cmovne	edx, eax
 1      0     0.20                        movzx	eax, r8b
 1      1     0.20                        cmp	edx, eax
 2      2     1.00                        setae	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   2.00    -      -      -     1.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     tzcnt	rax, rcx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     tzcnt	rdx, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	edx, 64
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	rcx, rcx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	edx, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     movzx	eax, r8b
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	edx, eax
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setae	al
```
## `mul_i8_pow2`, `mul_u8_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      5
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.80
IPC:               0.60
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	al, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00    -      -      -      -      -     1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	al, cl
```
## `mul_i8_unb_pow2`, `mul_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      7
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.71
IPC:               0.57
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      2     0.20                        and	dl, 7
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	al, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00    -      -      -      -     1.00   1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 7
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	al, cl
```
## `mul_i16_pow2`, `mul_i32_pow2`, `mul_i32_unb_pow2`, `mul_u16_pow2`, `mul_u32_pow2`, `mul_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      1
Total Cycles:      6
Total uOps:        1

Dispatch Width:    6
uOps Per Cycle:    0.17
IPC:               0.17
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      3     0.50                        shlx	eax, ecx, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	eax, ecx, edx
```
## `mul_i16_unb_pow2`, `mul_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      2
Total Cycles:      8
Total uOps:        2

Dispatch Width:    6
uOps Per Cycle:    0.25
IPC:               0.25
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      2     0.20                        and	dl, 15
 1      3     0.50                        shlx	eax, ecx, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
 -      -      -      -      -      -     1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	dl, 15
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	eax, ecx, edx
```
## `mul_i64_pow2`, `mul_i64_unb_pow2`, `mul_u64_pow2`, `mul_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      1
Total Cycles:      6
Total uOps:        1

Dispatch Width:    6
uOps Per Cycle:    0.17
IPC:               0.17
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      3     0.50                        shlx	rax, rcx, rdx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	rax, rcx, rdx
```
## `mul_i128_pow2`, `mul_i128_unb_pow2`, `mul_u128_pow2`, `mul_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      10
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    0.90
IPC:               0.80
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      0     0.20                        mov	ecx, r8d
 3      5     1.00                        shld	rdx, rax, cl
 1      3     0.50                        shlx	rcx, rax, r8
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	rdx, rcx
 1      1     0.50                        cmove	rax, rcx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, r8d
 -     1.00    -      -      -     1.00   1.00    -      -      -      -      -      -     shld	rdx, rax, cl
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	rcx, rax, r8
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rdx, rcx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	rax, rcx
```
## `rem_floor_i8_pow2`, `rem_floor_u8_pow2`, `rem_u8_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      9
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    0.78
IPC:               0.67
Block RThroughput: 1.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        mov	al, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	al, cl
 1      1     0.20                        not	al
 1      2     0.20                        and	al, r8b

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   1.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	al, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	al, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	al, r8b
```
## `rem_floor_i8_unb_pow2`, `rem_floor_u8_unb_pow2`, `rem_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      10
Total uOps:        8

Dispatch Width:    6
uOps Per Cycle:    0.80
IPC:               0.70
Block RThroughput: 1.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, ecx
 1      2     0.20                        and	dl, 7
 1      1     0.20                        mov	al, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	al, cl
 1      1     0.20                        not	al
 1      2     0.20                        and	al, r8b

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 7
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	al, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	al, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	al
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	al, r8b
```
## `rem_floor_i16_pow2`, `rem_floor_i32_pow2`, `rem_floor_u16_pow2`, `rem_floor_u32_pow2`, `rem_u16_pow2`, `rem_u32_pow2`
```asm
Iterations:        1
Instructions:      1
Total Cycles:      6
Total uOps:        1

Dispatch Width:    6
uOps Per Cycle:    0.17
IPC:               0.17
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      3     1.00                        bzhi	eax, ecx, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	eax, ecx, edx
```
## `rem_floor_i16_unb_pow2`, `rem_floor_u16_unb_pow2`, `rem_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      2
Total Cycles:      8
Total uOps:        2

Dispatch Width:    6
uOps Per Cycle:    0.25
IPC:               0.25
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      2     0.20                        and	dl, 15
 1      3     1.00                        bzhi	eax, ecx, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
 -     1.00    -      -      -      -      -      -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	dl, 15
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	eax, ecx, edx
```
## `rem_floor_i32_unb_pow2`, `rem_floor_u32_unb_pow2`, `rem_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      2
Total Cycles:      8
Total uOps:        2

Dispatch Width:    6
uOps Per Cycle:    0.25
IPC:               0.25
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      2     0.20                        and	dl, 31
 1      3     1.00                        bzhi	eax, ecx, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
 -     1.00    -      -      -      -      -      -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	dl, 31
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	eax, ecx, edx
```
## `rem_floor_i64_pow2`, `rem_floor_u64_pow2`, `rem_u64_pow2`
```asm
Iterations:        1
Instructions:      1
Total Cycles:      6
Total uOps:        1

Dispatch Width:    6
uOps Per Cycle:    0.17
IPC:               0.17
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      3     1.00                        bzhi	rax, rcx, rdx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	rax, rcx, rdx
```
## `rem_floor_i64_unb_pow2`, `rem_floor_u64_unb_pow2`, `rem_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      2
Total Cycles:      8
Total uOps:        2

Dispatch Width:    6
uOps Per Cycle:    0.25
IPC:               0.25
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      2     0.20                        and	dl, 63
 1      3     1.00                        bzhi	rax, rcx, rdx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
 -     1.00    -      -      -      -      -      -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	dl, 63
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	rax, rcx, rdx
```
## `rem_floor_i128_pow2`, `rem_floor_i128_unb_pow2`, `rem_floor_u128_pow2`, `rem_floor_u128_unb_pow2`, `rem_u128_pow2`, `rem_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      10
Total uOps:        8

Dispatch Width:    6
uOps Per Cycle:    0.80
IPC:               0.80
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        mov	r9, -1
 1      3     0.50                        shlx	rax, r9, r8
 1      2     0.20                        xor	r10d, r10d
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	r9, rax
 1      1     0.50                        cmove	r10, rax
 1      2     0.33                        andn	rax, r10, rcx
 1      2     0.33                        andn	rdx, r9, rdx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r9, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	rax, r9, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	r10d, r10d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r9, rax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	r10, rax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     andn	rax, r10, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     andn	rdx, r9, rdx
```
## `rem_i8_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      15
Total uOps:        17

Dispatch Width:    6
uOps Per Cycle:    1.13
IPC:               0.93
Block RThroughput: 3.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      1     0.20                        mov	r8b, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r8b, cl
 1      1     0.20                        not	r8b
 1      0     0.20                        mov	r9d, eax
 1      1     0.50                        sar	r9b, 7
 1      2     0.20                        and	r9b, r8b
 1      1     0.20                        add	r9b, al
 1      2     0.20                        and	cl, 7
 2      2     1.00                        sar	r9b, cl
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r9b, cl
 1      1     0.20                        sub	al, r9b

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   2.00    -      -      -     3.00   4.00    -      -      -     3.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8b, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r8b, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	r8b
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r9d, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sar	r9b, 7
1.00    -      -      -      -      -      -      -      -      -      -      -      -     and	r9b, r8b
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	r9b, al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	cl, 7
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sar	r9b, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r9b, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     sub	al, r9b
```
## `rem_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      13
Total Cycles:      14
Total uOps:        14

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.93
Block RThroughput: 2.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      2     0.20                        and	dl, 7
 1      1     0.20                        mov	r8b, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r8b, cl
 1      0     0.20                        mov	ecx, r8d
 1      1     0.20                        not	cl
 1      0     0.20                        mov	edx, eax
 1      1     0.50                        sar	dl, 7
 1      2     0.20                        and	dl, cl
 1      1     0.20                        add	dl, al
 1      2     0.20                        and	dl, r8b
 1      1     0.20                        sub	al, dl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   2.00    -      -      -     3.00   4.00    -      -      -     3.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 7
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8b, -1
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r8b, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, r8d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	edx, eax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	dl, 7
1.00    -      -      -      -      -      -      -      -      -      -      -      -     and	dl, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	dl, al
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, r8b
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     sub	al, dl
```
## `rem_i16_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      17
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    0.65
IPC:               0.65
Block RThroughput: 1.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      1     0.33                        movsx	ecx, cx
 1      0     0.20                        mov	r8d, edx
 1      1     0.50                        sar	ecx, 15
 1      3     1.00                        bzhi	ecx, ecx, r8d
 1      1     0.20                        add	ecx, eax
 1      1     0.33                        movsx	ecx, cx
 1      2     0.20                        and	dl, 15
 1      3     0.50                        sarx	ecx, ecx, edx
 1      3     0.50                        shlx	ecx, ecx, r8d
 1      1     0.20                        sub	eax, ecx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   2.00    -      -      -     2.00   2.00    -      -      -     3.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movsx	ecx, cx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8d, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	ecx, 15
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	ecx, ecx, r8d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	ecx, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movsx	ecx, cx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	dl, 15
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	ecx, ecx, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	ecx, ecx, r8d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     sub	eax, ecx
```
## `rem_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      12
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    0.83
IPC:               0.83
Block RThroughput: 1.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      1     0.33                        movsx	ecx, cx
 1      2     0.20                        and	dl, 15
 1      1     0.20                        mov	r8d, -1
 1      3     0.50                        shlx	r8d, r8d, edx
 1      1     0.50                        sar	ecx, 15
 1      3     1.00                        bzhi	ecx, ecx, edx
 1      1     0.20                        add	ecx, eax
 1      2     0.20                        and	ecx, r8d
 1      1     0.20                        sub	eax, ecx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   2.00    -      -      -     2.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movsx	ecx, cx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 15
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r8d, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	r8d, r8d, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	ecx, 15
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	ecx, ecx, edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	ecx, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	ecx, r8d
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sub	eax, ecx
```
## `rem_i32_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      15
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    0.47
IPC:               0.47
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      1     0.50                        sar	ecx, 31
 1      3     1.00                        bzhi	ecx, ecx, edx
 1      1     0.20                        add	ecx, eax
 1      3     0.50                        sarx	ecx, ecx, edx
 1      3     0.50                        shlx	ecx, ecx, edx
 1      1     0.20                        sub	eax, ecx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -     1.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sar	ecx, 31
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	ecx, ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	ecx, eax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sarx	ecx, ecx, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	ecx, ecx, edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     sub	eax, ecx
```
## `rem_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      12
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    0.92
IPC:               0.92
Block RThroughput: 1.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      0     0.20                        mov	ecx, edx
 1      2     0.20                        and	dl, 31
 1      1     0.20                        mov	r8d, -1
 1      3     0.50                        shlx	ecx, r8d, ecx
 1      0     0.20                        mov	r8d, eax
 1      1     0.50                        sar	r8d, 31
 1      3     1.00                        bzhi	edx, r8d, edx
 1      1     0.20                        add	edx, eax
 1      2     0.20                        and	edx, ecx
 1      1     0.20                        sub	eax, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   2.00    -      -      -     2.00   2.00    -      -      -     3.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	dl, 31
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r8d, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	ecx, r8d, ecx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, eax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	r8d, 31
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	edx, r8d, edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	edx, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	edx, ecx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     sub	eax, edx
```
## `rem_i64_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      15
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.40
IPC:               0.47
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      1     0.50                        sar	rcx, 63
 1      3     1.00                        bzhi	rcx, rcx, rdx
 1      1     0.20                        add	rcx, rax
 1      3     0.50                        sarx	rcx, rcx, rdx
 1      3     0.50                        shlx	rcx, rcx, rdx
 1      1     0.20                        sub	rax, rcx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -     1.00   2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sar	rcx, 63
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	rcx, rcx, rdx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	rcx, rax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sarx	rcx, rcx, rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	rcx, rcx, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     sub	rax, rcx
```
## `rem_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      13
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    0.69
IPC:               0.85
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      0     0.20                        mov	ecx, edx
 1      2     0.20                        and	dl, 63
 1      1     0.20                        mov	r8, -1
 1      3     0.50                        shlx	rcx, r8, rcx
 0      1     0.00                        mov	r8, rax
 1      1     0.50                        sar	r8, 63
 1      3     1.00                        bzhi	rdx, r8, rdx
 1      1     0.20                        add	rdx, rax
 1      2     0.20                        and	rdx, rcx
 1      1     0.20                        sub	rax, rdx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   2.00    -      -      -     2.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 63
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	rcx, r8, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sar	r8, 63
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	rdx, r8, rdx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	rdx, rax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	rdx, rcx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     sub	rax, rdx
```
## `rem_i128_pow2`
```asm
Iterations:        1
Instructions:      31
Total Cycles:      25
Total uOps:        34

Dispatch Width:    6
uOps Per Cycle:    1.36
IPC:               1.24
Block RThroughput: 7.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	rsi
 0      1     0.00                        mov	rax, rcx
 1      1     0.20                        mov	rcx, -1
 1      3     0.50                        shlx	r10, rcx, r8
 1      2     0.20                        xor	r9d, r9d
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	rcx, r10
 1      1     0.50                        cmovne	r10, r9
 0      1     0.00                        mov	r11, rdx
 1      1     0.50                        sar	r11, 63
 1      2     0.33                        andn	rsi, rcx, r11
 1      2     0.33                        andn	r10, r10, r11
 1      1     0.20                        add	r10, rax
 1      1     0.50                        adc	rsi, rdx
 1      0     0.20                        mov	ecx, r8d
 3      5     1.00                        shrd	r10, rsi, cl
 1      2     0.20                        test	r8b, 64
 1      3     0.50                        sarx	rcx, rsi, r8
 1      1     0.50                        cmovne	r10, rcx
 1      3     0.50                        shlx	r11, r10, r8
 1      1     0.50                        cmove	r9, r11
 1      1     0.50                        sar	rsi, 63
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmove	rsi, rcx
 1      0     0.20                        mov	ecx, r8d
 3      5     1.00                        shld	rsi, r10, cl
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	rsi, r11
 1      1     0.20                        sub	rax, r9
 1      1     0.50                        sbb	rdx, rsi
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
8.00   5.00    -      -      -     4.00   9.00    -     1.00   1.00   5.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	rcx, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	r10, rcx, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	r9d, r9d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r8b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rcx, r10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r10, r9
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sar	r11, 63
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     andn	rsi, rcx, r11
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     andn	r10, r10, r11
1.00    -      -      -      -      -      -      -      -      -      -      -      -     add	r10, rax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     adc	rsi, rdx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
1.00   1.00    -      -      -      -      -      -      -      -     1.00    -      -     shrd	r10, rsi, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	rcx, rsi, r8
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r10, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	r11, r10, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	r9, r11
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	rsi, 63
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	rsi, rcx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -     shld	rsi, r10, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r8b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rsi, r11
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     sub	rax, r9
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sbb	rdx, rsi
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rsi
```
## `rem_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      17
Total Cycles:      15
Total uOps:        15

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.13
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      1     0.20                        mov	rcx, -1
 1      3     0.50                        shlx	r9, rcx, r8
 1      2     0.20                        xor	r10d, r10d
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmove	r10, r9
 1      1     0.50                        cmovne	rcx, r9
 0      1     0.00                        mov	r8, rdx
 1      1     0.50                        sar	r8, 63
 1      2     0.33                        andn	r9, rcx, r8
 1      2     0.33                        andn	r8, r10, r8
 1      1     0.20                        add	r8, rax
 1      1     0.50                        adc	r9, rdx
 1      2     0.20                        and	r9, rcx
 1      2     0.20                        and	r8, r10
 1      1     0.20                        sub	rax, r8
 1      1     0.50                        sbb	rdx, r9

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   2.00    -      -      -     2.00   4.00    -      -      -     3.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	rcx, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	r9, rcx, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	r10d, r10d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r8b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	r10, r9
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rcx, r9
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sar	r8, 63
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     andn	r9, rcx, r8
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     andn	r8, r10, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	r8, rax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     adc	r9, rdx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	r9, rcx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	r8, r10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sub	rax, r8
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sbb	rdx, r9
```
## `round_to_multiple_i8_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      12
Total uOps:        16

Dispatch Width:    6
uOps Per Cycle:    1.33
IPC:               1.25
Block RThroughput: 2.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        mov	al, 1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	al, cl
 1      0     0.20                        mov	ecx, eax
 1      1     0.50                        shr	cl
 1      0     0.20                        mov	edx, r8d
 1      1     0.50                        shr	dl, 7
 1      2     0.20                        xor	r9d, r9d
 1      1     0.20                        sub	cl, dl
 1      0     0.20                        movzx	ecx, cl
 1      1     0.50                        cmovb	ecx, r9d
 1      1     0.20                        add	cl, r8b
 1      1     0.20                        neg	al
 1      2     0.20                        and	al, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   3.00    -      -      -     3.00   3.00    -      -      -     3.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	al, 1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	al, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, eax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shr	dl, 7
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	r9d, r9d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     sub	cl, dl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     movzx	ecx, cl
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovb	ecx, r9d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	cl, r8b
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     neg	al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	al, cl
```
## `round_to_multiple_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      16
Total Cycles:      13
Total uOps:        17

Dispatch Width:    6
uOps Per Cycle:    1.31
IPC:               1.23
Block RThroughput: 2.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, ecx
 1      2     0.20                        and	dl, 7
 1      1     0.20                        mov	al, 1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	al, cl
 1      0     0.20                        mov	ecx, eax
 1      1     0.50                        shr	cl
 1      0     0.20                        mov	edx, r8d
 1      1     0.50                        shr	dl, 7
 1      2     0.20                        xor	r9d, r9d
 1      1     0.20                        sub	cl, dl
 1      0     0.20                        movzx	ecx, cl
 1      1     0.50                        cmovb	ecx, r9d
 1      1     0.20                        add	cl, r8b
 1      1     0.20                        neg	al
 1      2     0.20                        and	al, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   3.00    -      -      -     3.00   3.00    -      -      -     4.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 7
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	al, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	al, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shr	cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	dl, 7
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	r9d, r9d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     sub	cl, dl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movzx	ecx, cl
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovb	ecx, r9d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	cl, r8b
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     neg	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	al, cl
```
## `round_to_multiple_i16_pow2`
```asm
Iterations:        1
Instructions:      12
Total Cycles:      14
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    0.86
IPC:               0.86
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        movzx	r8d, cx
 1      1     0.20                        mov	eax, 1
 1      3     0.50                        shlx	eax, eax, edx
 1      1     0.20                        movzx	edx, ax
 1      1     0.50                        shr	edx
 1      1     0.50                        shr	r8d, 15
 1      2     0.20                        xor	r9d, r9d
 1      1     0.20                        sub	dx, r8w
 1      1     0.50                        cmovb	edx, r9d
 1      1     0.20                        add	edx, ecx
 1      1     0.20                        neg	eax
 1      2     0.20                        and	eax, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   2.00    -      -      -     2.00   3.00    -      -      -     3.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	r8d, cx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	eax, eax, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movzx	edx, ax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shr	r8d, 15
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	r9d, r9d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     sub	dx, r8w
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovb	edx, r9d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	edx, ecx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     neg	eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	eax, edx
```
## `round_to_multiple_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      13
Total Cycles:      14
Total uOps:        13

Dispatch Width:    6
uOps Per Cycle:    0.93
IPC:               0.93
Block RThroughput: 2.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        movzx	r8d, cx
 1      2     0.20                        and	dl, 15
 1      1     0.20                        mov	eax, 1
 1      3     0.50                        shlx	eax, eax, edx
 1      0     0.20                        mov	edx, eax
 1      1     0.50                        shr	edx
 1      1     0.50                        shr	r8d, 15
 1      2     0.20                        xor	r9d, r9d
 1      1     0.20                        sub	dx, r8w
 1      1     0.50                        cmovb	edx, r9d
 1      1     0.20                        add	edx, ecx
 1      1     0.20                        neg	eax
 1      2     0.20                        and	eax, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   2.00    -      -      -     3.00   3.00    -      -      -     3.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	r8d, cx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 15
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	eax, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	eax, eax, edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	edx, eax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r8d, 15
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	r9d, r9d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     sub	dx, r8w
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovb	edx, r9d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	edx, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     neg	eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	eax, edx
```
## `round_to_multiple_i32_pow2`, `round_to_multiple_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      12
Total Cycles:      13
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    0.92
IPC:               0.92
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        mov	eax, 1
 1      3     0.50                        shlx	eax, eax, edx
 1      0     0.20                        mov	edx, eax
 1      1     0.50                        shr	edx
 1      0     0.20                        mov	r8d, ecx
 1      1     0.50                        shr	r8d, 31
 1      2     0.20                        xor	r9d, r9d
 1      1     0.20                        sub	edx, r8d
 1      1     0.50                        cmovb	edx, r9d
 1      1     0.20                        add	edx, ecx
 1      1     0.20                        neg	eax
 1      2     0.20                        and	eax, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   2.00    -      -      -     2.00   3.00    -      -      -     3.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	eax, eax, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	edx, eax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8d, ecx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r8d, 31
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	r9d, r9d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     sub	edx, r8d
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovb	edx, r9d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	edx, ecx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     neg	eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	eax, edx
```
## `round_to_multiple_i64_pow2`, `round_to_multiple_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      12
Total Cycles:      14
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    0.71
IPC:               0.86
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        mov	eax, 1
 1      3     0.50                        shlx	rax, rax, rdx
 0      1     0.00                        mov	rdx, rax
 1      1     0.50                        shr	rdx
 0      1     0.00                        mov	r8, rcx
 1      1     0.50                        shr	r8, 63
 1      2     0.20                        xor	r9d, r9d
 1      1     0.20                        sub	rdx, r8
 1      1     0.50                        cmovae	r9, rdx
 1      1     0.20                        add	r9, rcx
 1      1     0.20                        neg	rax
 1      2     0.20                        and	rax, r9

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     2.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	rax, rax, rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, rax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shr	r8, 63
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	r9d, r9d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     sub	rdx, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovae	r9, rdx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	r9, rcx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     neg	rax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	rax, r9
```
## `round_to_multiple_i128_pow2`, `round_to_multiple_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      29
Total Cycles:      22
Total uOps:        28

Dispatch Width:    6
uOps Per Cycle:    1.27
IPC:               1.32
Block RThroughput: 5.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	rsi
 1      0     0.20                        mov	r9d, r8d
 0      1     0.00                        mov	r8, rcx
 1      1     0.20                        mov	eax, 1
 1      2     0.20                        xor	r10d, r10d
 1      0     0.20                        mov	ecx, r9d
 3      5     1.00                        shld	r10, rax, cl
 1      2     0.20                        xor	ecx, ecx
 1      3     0.50                        shlx	rax, rax, r9
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	r10, rax
 1      1     0.50                        cmovne	rax, rcx
 0      1     0.00                        mov	r9, r10
 1      1     0.50                        shr	r9
 0      1     0.00                        mov	r11, r10
 1      3     1.00                        shld	r11, rax, 63
 0      1     0.00                        mov	rsi, rdx
 1      1     0.50                        shr	rsi, 63
 1      1     0.20                        sub	r11, rsi
 1      1     0.50                        sbb	r9, 0
 1      1     0.50                        cmovb	r9, rcx
 1      1     0.50                        cmovb	r11, rcx
 1      1     0.20                        add	r11, r8
 1      1     0.50                        adc	rdx, r9
 1      1     0.20                        neg	rax
 1      1     0.50                        sbb	rcx, r10
 1      2     0.20                        and	rdx, rcx
 1      2     0.20                        and	rax, r11
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
6.00   4.00    -      -      -     4.00   7.00    -     1.00   1.00   4.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rsi
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r9d, r8d
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, 1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	r10d, r10d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -     shld	r10, rax, cl
1.00    -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	rax, rax, r9
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r10, rax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rax, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, r10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shr	r9
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, r10
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     shld	r11, rax, 63
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rsi, rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shr	rsi, 63
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     sub	r11, rsi
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sbb	r9, 0
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovb	r9, rcx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovb	r11, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	r11, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     adc	rdx, r9
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     neg	rax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sbb	rcx, r10
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	rdx, rcx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	rax, r11
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rsi
```
## `round_to_multiple_u8_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      10
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.90
Block RThroughput: 1.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        mov	al, 1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	al, cl
 1      0     0.20                        mov	ecx, eax
 1      1     0.50                        shr	cl
 1      1     0.20                        add	cl, r8b
 1      1     0.20                        neg	al
 1      2     0.20                        and	al, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   2.00    -      -      -     2.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	al, 1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	al, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shr	cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	cl, r8b
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     neg	al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	al, cl
```
## `round_to_multiple_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      11
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.91
Block RThroughput: 1.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, ecx
 1      2     0.20                        and	dl, 7
 1      1     0.20                        mov	al, 1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	al, cl
 1      0     0.20                        mov	ecx, eax
 1      1     0.50                        shr	cl
 1      1     0.20                        add	cl, r8b
 1      1     0.20                        neg	al
 1      2     0.20                        and	al, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   2.00    -      -      -     2.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 7
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	al, 1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	al, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shr	cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	cl, r8b
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     neg	al
1.00    -      -      -      -      -      -      -      -      -      -      -      -     and	al, cl
```
## `round_to_multiple_u16_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      12
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    0.58
IPC:               0.58
Block RThroughput: 1.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        mov	eax, 1
 1      3     0.50                        shlx	eax, eax, edx
 1      1     0.20                        movzx	edx, ax
 1      1     0.50                        shr	edx
 1      1     0.20                        add	edx, ecx
 1      1     0.20                        neg	eax
 1      2     0.20                        and	eax, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -     1.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	eax, eax, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movzx	edx, ax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	edx, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     neg	eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	eax, edx
```
## `round_to_multiple_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      12
Total uOps:        8

Dispatch Width:    6
uOps Per Cycle:    0.67
IPC:               0.67
Block RThroughput: 1.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      2     0.20                        and	dl, 15
 1      1     0.20                        mov	eax, 1
 1      3     0.50                        shlx	eax, eax, edx
 1      0     0.20                        mov	edx, eax
 1      1     0.50                        shr	edx
 1      1     0.20                        add	edx, ecx
 1      1     0.20                        neg	eax
 1      2     0.20                        and	eax, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -     2.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	dl, 15
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	eax, eax, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shr	edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	edx, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     neg	eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	eax, edx
```
## `round_to_multiple_u32_pow2`, `round_to_multiple_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      11
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    0.64
IPC:               0.64
Block RThroughput: 1.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        mov	eax, 1
 1      3     0.50                        shlx	eax, eax, edx
 1      0     0.20                        mov	edx, eax
 1      1     0.50                        shr	edx
 1      1     0.20                        add	edx, ecx
 1      1     0.20                        neg	eax
 1      2     0.20                        and	eax, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -     1.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	eax, eax, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, eax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	edx, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     neg	eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	eax, edx
```
## `round_to_multiple_u64_pow2`, `round_to_multiple_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      12
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.50
IPC:               0.58
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        mov	eax, 1
 1      3     0.50                        shlx	rax, rax, rdx
 0      1     0.00                        mov	rdx, rax
 1      1     0.50                        shr	rdx
 1      1     0.20                        add	rdx, rcx
 1      1     0.20                        neg	rax
 1      2     0.20                        and	rax, rdx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -     1.00   1.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	rax, rax, rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, rax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	rdx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	rdx, rcx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     neg	rax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	rax, rdx
```
## `round_to_multiple_u128_pow2`, `round_to_multiple_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      20
Total Cycles:      19
Total uOps:        19

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.05
Block RThroughput: 3.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	r9, rcx
 1      2     0.20                        xor	r10d, r10d
 1      1     0.20                        mov	eax, 1
 1      2     0.20                        xor	r11d, r11d
 1      0     0.20                        mov	ecx, r8d
 3      5     1.00                        shld	r11, rax, cl
 1      3     0.50                        shlx	rax, rax, r8
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	r11, rax
 1      1     0.50                        cmovne	rax, r10
 0      1     0.00                        mov	rcx, r11
 1      1     0.50                        shr	rcx
 0      1     0.00                        mov	r8, r11
 1      3     1.00                        shld	r8, rax, 63
 1      1     0.20                        add	r8, r9
 1      1     0.50                        adc	rdx, rcx
 1      1     0.20                        neg	rax
 1      1     0.50                        sbb	r10, r11
 1      2     0.20                        and	rdx, r10
 1      2     0.20                        and	rax, r8

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   3.00    -      -      -     4.00   4.00    -      -      -     4.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	r10d, r10d
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, 1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	r11d, r11d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -     1.00    -      -      -     1.00   1.00    -      -      -      -      -      -     shld	r11, rax, cl
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	rax, rax, r8
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r11, rax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rax, r10
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rcx, r11
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shr	rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, r11
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     shld	r8, rax, 63
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	r8, r9
1.00    -      -      -      -      -      -      -      -      -      -      -      -     adc	rdx, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     neg	rax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sbb	r10, r11
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	rdx, r10
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	rax, r8
```
## `unbounded_ceil_to_multiple_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      17
Total Cycles:      14
Total uOps:        21

Dispatch Width:    6
uOps Per Cycle:    1.50
IPC:               1.21
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        cmp	dl, 8
 1      1     0.50                        jae	.LBB349_1
 1      1     0.20                        mov	r9b, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r9b, cl
 1      0     0.20                        mov	eax, r9d
 1      1     0.20                        not	al
 1      1     0.20                        add	r8b, al
 2      2     1.00                        setno	al
 1      2     0.20                        and	r8b, r9b
 1      0     0.20                        mov	edx, r8d
 2      7     0.50                  U     ret
 1      2     0.20                        test	r8b, r8b
 2      2     1.00                        setle	al
 1      2     0.20                        xor	r8d, r8d
 1      0     0.20                        mov	edx, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   3.00    -      -      -     3.00   7.00    -      -      -     4.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 8
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB349_1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r9b, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r9b, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, r9d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	r8b, al
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setno	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	r8b, r9b
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, r8d
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r8b, r8b
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setle	al
1.00    -      -      -      -      -      -      -      -      -      -      -      -     xor	r8d, r8d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	edx, r8d
```
## `unbounded_ceil_to_multiple_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      17
Total Cycles:      15
Total uOps:        20

Dispatch Width:    6
uOps Per Cycle:    1.33
IPC:               1.13
Block RThroughput: 3.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 16
 1      1     0.50                        jae	.LBB346_1
 1      1     0.20                        mov	eax, -1
 1      3     0.50                        shlx	edx, eax, edx
 1      0     0.20                        mov	eax, edx
 1      1     0.20                        not	eax
 1      1     0.20                        add	cx, ax
 2      2     1.00                        setno	al
 1      2     0.20                        and	ecx, edx
 1      0     0.20                        movzx	eax, al
 1      0     0.20                        mov	edx, ecx
 2      7     0.50                  U     ret
 1      2     0.20                        test	cx, cx
 2      2     1.00                        setle	al
 1      2     0.20                        xor	ecx, ecx
 1      0     0.20                        movzx	eax, al
 1      0     0.20                        mov	edx, ecx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   3.00    -      -      -     3.00   6.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 16
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB346_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	edx, eax, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	eax, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	cx, ax
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setno	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	ecx, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
1.00    -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, ecx
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	cx, cx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setle	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	ecx, ecx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	edx, ecx
```
## `unbounded_ceil_to_multiple_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      17
Total Cycles:      15
Total uOps:        20

Dispatch Width:    6
uOps Per Cycle:    1.33
IPC:               1.13
Block RThroughput: 3.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 32
 1      1     0.50                        jae	.LBB347_1
 1      1     0.20                        mov	eax, -1
 1      3     0.50                        shlx	edx, eax, edx
 1      0     0.20                        mov	eax, edx
 1      1     0.20                        not	eax
 1      1     0.20                        add	ecx, eax
 2      2     1.00                        setno	al
 1      2     0.20                        and	ecx, edx
 1      0     0.20                        movzx	eax, al
 1      0     0.20                        mov	edx, ecx
 2      7     0.50                  U     ret
 1      2     0.20                        test	ecx, ecx
 2      2     1.00                        setle	al
 1      2     0.20                        xor	ecx, ecx
 1      0     0.20                        movzx	eax, al
 1      0     0.20                        mov	edx, ecx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   3.00    -      -      -     3.00   6.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 32
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB347_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	edx, eax, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	eax, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	ecx, eax
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setno	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	ecx, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
1.00    -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, ecx
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	ecx, ecx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setle	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	ecx, ecx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	edx, ecx
```
## `unbounded_ceil_to_multiple_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      17
Total Cycles:      16
Total uOps:        17

Dispatch Width:    6
uOps Per Cycle:    1.06
IPC:               1.06
Block RThroughput: 3.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 64
 1      1     0.50                        jae	.LBB348_1
 1      1     0.20                        mov	rax, -1
 1      3     0.50                        shlx	rdx, rax, rdx
 0      1     0.00                        mov	rax, rdx
 1      1     0.20                        not	rax
 1      1     0.20                        add	rcx, rax
 2      2     1.00                        setno	al
 1      2     0.20                        and	rcx, rdx
 1      0     0.20                        movzx	eax, al
 0      1     0.00                        mov	rdx, rcx
 2      7     0.50                  U     ret
 1      2     0.20                        test	rcx, rcx
 2      2     1.00                        setle	al
 1      2     0.20                        xor	ecx, ecx
 1      0     0.20                        movzx	eax, al
 0      1     0.00                        mov	rdx, rcx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   2.00    -      -      -     2.00   6.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB348_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	rax, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	rdx, rax, rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	rax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	rcx, rax
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setno	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	rcx, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movzx	eax, al
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, rcx
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	rcx, rcx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setle	al
1.00    -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	eax, al
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, rcx
```
## `unbounded_ceil_to_multiple_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      36
Total Cycles:      29
Total uOps:        39

Dispatch Width:    6
uOps Per Cycle:    1.34
IPC:               1.24
Block RThroughput: 6.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	rsi
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB345_4
 1      1     0.20                        mov	r10, -1
 1      3     0.50                        shlx	r11, r10, r9
 1      2     0.20                        xor	ecx, ecx
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	r10, r11
 0      1     0.00                        mov	r9, r11
 1      1     0.50                        cmovne	r9, rcx
 0      1     0.00                        mov	r11, r10
 1      1     0.20                        not	r11
 0      1     0.00                        mov	rsi, r9
 1      1     0.20                        not	rsi
 1      1     0.20                        add	rdx, rsi
 1      1     0.50                        adc	r8, r11
 1      1     0.50                        jo	.LBB345_5
 1      2     0.20                        and	rdx, r9
 1      2     0.20                        and	r8, r10
 0      1     0.00                        mov	rcx, rdx
 2      12    0.50           *            mov	qword ptr [rax + 16], rcx
 2      12    0.50           *            mov	qword ptr [rax + 24], r8
 1      1     0.20                        mov	ecx, 1
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rsi
 2      7     0.50                  U     ret
 1      2     0.20                        xor	ecx, ecx
 1      1     0.20                        cmp	rdx, 1
 1      1     0.50                        sbb	r8, 0
 1      1     0.20                        mov	r8d, 0
 1      1     0.50                        jl	.LBB345_3
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   3.00   1.00   1.00   3.00   4.00   6.00   3.00   4.00   4.00   4.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, r9b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB345_4
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r10, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	r11, r10, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	ecx, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r10, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, r11
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r9, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, r10
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rsi, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	rsi
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	rdx, rsi
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     adc	r8, r11
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     jo	.LBB345_5
1.00    -      -      -      -      -      -      -      -      -      -      -      -     and	rdx, r9
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	r8, r10
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rcx, rdx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 16], rcx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 24], r8
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, 1
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 8], 0
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rsi
1.00    -      -     1.00    -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	rdx, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sbb	r8, 0
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r8d, 0
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     jl	.LBB345_3
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 8], 0
 -      -     1.00    -      -      -      -      -      -      -      -      -      -     pop	rsi
```
## `unbounded_ceil_to_multiple_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      16
Total Cycles:      15
Total uOps:        20

Dispatch Width:    6
uOps Per Cycle:    1.33
IPC:               1.07
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      1     0.20                        cmp	dl, 8
 1      1     0.50                        jae	.LBB354_1
 1      1     0.20                        mov	r8b, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r8b, cl
 1      0     0.20                        mov	edx, r8d
 1      1     0.20                        not	dl
 1      1     0.20                        add	dl, al
 1      1     0.20                        cmp	dl, al
 2      2     1.00                        setae	al
 1      2     0.20                        and	dl, r8b
 2      7     0.50                  U     ret
 1      2     0.20                        test	al, al
 2      2     1.00                        sete	al
 1      2     0.20                        xor	edx, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   3.00    -      -      -     3.00   6.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 8
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB354_1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8b, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r8b, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	edx, r8d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	dl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	dl, al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, al
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setae	al
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, r8b
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	al, al
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sete	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	edx, edx
```
## `unbounded_ceil_to_multiple_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      18
Total Cycles:      12
Total uOps:        21

Dispatch Width:    6
uOps Per Cycle:    1.75
IPC:               1.50
Block RThroughput: 3.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 16
 1      1     0.50                        jae	.LBB351_1
 1      1     0.20                        mov	eax, -1
 1      3     0.50                        shlx	edx, eax, edx
 1      0     0.20                        mov	eax, edx
 1      1     0.20                        not	eax
 1      1     0.20                        add	eax, ecx
 1      1     0.20                        cmp	ax, cx
 2      2     1.00                        setae	al
 1      0     0.20                        mov	r8d, edx
 1      1     0.20                        sub	r8d, ecx
 1      2     0.33                        andn	edx, r8d, edx
 1      0     0.20                        movzx	eax, al
 2      7     0.50                  U     ret
 1      2     0.20                        test	cx, cx
 2      2     1.00                        sete	al
 1      2     0.20                        xor	edx, edx
 1      0     0.20                        movzx	eax, al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   2.00    -      -      -     4.00   6.00    -      -      -     4.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 16
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB351_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	edx, eax, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	eax, edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	eax, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	ax, cx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setae	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8d, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sub	r8d, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     andn	edx, r8d, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movzx	eax, al
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	cx, cx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sete	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	edx, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
```
## `unbounded_ceil_to_multiple_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      18
Total Cycles:      12
Total uOps:        21

Dispatch Width:    6
uOps Per Cycle:    1.75
IPC:               1.50
Block RThroughput: 3.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 32
 1      1     0.50                        jae	.LBB352_1
 1      1     0.20                        mov	eax, -1
 1      3     0.50                        shlx	edx, eax, edx
 1      0     0.20                        mov	eax, edx
 1      1     0.20                        not	eax
 1      1     0.20                        add	eax, ecx
 1      1     0.20                        cmp	eax, ecx
 2      2     1.00                        setae	al
 1      0     0.20                        mov	r8d, edx
 1      1     0.20                        sub	r8d, ecx
 1      2     0.33                        andn	edx, r8d, edx
 1      0     0.20                        movzx	eax, al
 2      7     0.50                  U     ret
 1      2     0.20                        test	ecx, ecx
 2      2     1.00                        sete	al
 1      2     0.20                        xor	edx, edx
 1      0     0.20                        movzx	eax, al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   2.00    -      -      -     4.00   6.00    -      -      -     4.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 32
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB352_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	edx, eax, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	eax, edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	eax, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	eax, ecx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setae	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8d, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sub	r8d, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     andn	edx, r8d, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movzx	eax, al
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	ecx, ecx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sete	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	edx, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
```
## `unbounded_ceil_to_multiple_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      18
Total Cycles:      13
Total uOps:        19

Dispatch Width:    6
uOps Per Cycle:    1.46
IPC:               1.38
Block RThroughput: 3.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 64
 1      1     0.50                        jae	.LBB353_1
 1      1     0.20                        mov	rax, -1
 1      3     0.50                        shlx	rdx, rax, rdx
 0      1     0.00                        mov	rax, rdx
 1      1     0.20                        not	rax
 1      1     0.20                        add	rax, rcx
 1      1     0.20                        cmp	rax, rcx
 2      2     1.00                        setae	al
 0      1     0.00                        mov	r8, rdx
 1      1     0.20                        sub	r8, rcx
 1      2     0.33                        andn	rdx, r8, rdx
 1      0     0.20                        movzx	eax, al
 2      7     0.50                  U     ret
 1      2     0.20                        test	rcx, rcx
 2      2     1.00                        sete	al
 1      2     0.20                        xor	edx, edx
 1      0     0.20                        movzx	eax, al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   3.00    -      -      -     3.00   6.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     jae	.LBB353_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	rax, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	rdx, rax, rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rdx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	rax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	rax, rcx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setae	al
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rdx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     sub	r8, rcx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     andn	rdx, r8, rdx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	rcx, rcx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sete	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	edx, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
```
## `unbounded_ceil_to_multiple_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      35
Total Cycles:      29
Total uOps:        38

Dispatch Width:    6
uOps Per Cycle:    1.31
IPC:               1.21
Block RThroughput: 6.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	rsi
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB350_4
 1      1     0.20                        mov	r10, -1
 1      3     0.50                        shlx	r11, r10, r9
 1      2     0.20                        xor	ecx, ecx
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	r10, r11
 0      1     0.00                        mov	r9, r11
 1      1     0.50                        cmovne	r9, rcx
 0      1     0.00                        mov	r11, r10
 1      1     0.20                        not	r11
 0      1     0.00                        mov	rsi, r9
 1      1     0.20                        not	rsi
 1      1     0.20                        add	rdx, rsi
 1      1     0.50                        adc	r8, r11
 1      1     0.50                        jb	.LBB350_5
 1      2     0.20                        and	rdx, r9
 1      2     0.20                        and	r8, r10
 0      1     0.00                        mov	rcx, rdx
 2      12    0.50           *            mov	qword ptr [rax + 16], rcx
 2      12    0.50           *            mov	qword ptr [rax + 24], r8
 1      1     0.20                        mov	ecx, 1
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rsi
 2      7     0.50                  U     ret
 1      2     0.20                        xor	ecx, ecx
 1      2     0.20                        or	rdx, r8
 1      1     0.20                        mov	r8d, 0
 1      1     0.50                        je	.LBB350_3
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   3.00   1.00   1.00   3.00   4.00   5.00   3.00   4.00   4.00   4.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, r9b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB350_4
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r10, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	r11, r10, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	ecx, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r10, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, r11
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r9, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, r10
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rsi, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	rsi
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	rdx, rsi
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     adc	r8, r11
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     jb	.LBB350_5
1.00    -      -      -      -      -      -      -      -      -      -      -      -     and	rdx, r9
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	r8, r10
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rcx, rdx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 16], rcx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 24], r8
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, 1
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 8], 0
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rsi
1.00    -      -     1.00    -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     or	rdx, r8
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r8d, 0
1.00    -      -      -      -      -      -      -      -      -      -      -      -     je	.LBB350_3
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 8], 0
 -      -     1.00    -      -      -      -      -      -      -      -      -      -     pop	rsi
```
## `unbounded_div_ceil_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      13
Total uOps:        19

Dispatch Width:    6
uOps Per Cycle:    1.46
IPC:               1.15
Block RThroughput: 4.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        cmp	dl, 8
 1      1     0.50                        jae	.LBB359_1
 1      1     0.20                        mov	r9b, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r9b, cl
 1      1     0.20                        not	r9b
 1      0     0.20                        mov	eax, r8d
 2      2     1.00                        sar	al, cl
 1      2     0.20                        and	r9b, r8b
 1      1     0.20                        cmp	r9b, 1
 1      1     0.50                        sbb	al, -1
 2      7     0.50                  U     ret
 1      2     0.20                        test	r8b, r8b
 2      2     1.00                        setg	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   2.00    -      -      -     3.00   5.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 8
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB359_1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r9b, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r9b, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	r9b
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, r8d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	al, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	r9b, r8b
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	r9b, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sbb	al, -1
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r8b, r8b
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setg	al
```
## `unbounded_div_ceil_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      12
Total uOps:        13

Dispatch Width:    6
uOps Per Cycle:    1.08
IPC:               0.92
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 16
 1      1     0.50                        jae	.LBB356_1
 1      1     0.33                        movsx	eax, cx
 1      3     0.50                        sarx	eax, eax, edx
 1      3     1.00                        bzhi	ecx, ecx, edx
 1      1     0.20                        cmp	cx, 1
 1      1     0.50                        sbb	ax, -1
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        test	cx, cx
 2      2     1.00                        setg	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   2.00    -      -      -     2.00   4.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 16
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB356_1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movsx	eax, cx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	eax, eax, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	ecx, ecx, edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	cx, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sbb	ax, -1
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	cx, cx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setg	al
```
## `unbounded_div_ceil_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      11
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.09
IPC:               0.91
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 32
 1      1     0.50                        jae	.LBB357_1
 1      3     0.50                        sarx	eax, ecx, edx
 1      3     1.00                        bzhi	ecx, ecx, edx
 1      1     0.20                        cmp	ecx, 1
 1      1     0.50                        sbb	eax, -1
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        test	ecx, ecx
 2      2     1.00                        setg	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     2.00   4.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 32
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB357_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	eax, ecx, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	ecx, ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	ecx, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sbb	eax, -1
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	ecx, ecx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setg	al
```
## `unbounded_div_ceil_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      11
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.09
IPC:               0.91
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 64
 1      1     0.50                        jae	.LBB358_1
 1      3     0.50                        sarx	rax, rcx, rdx
 1      3     1.00                        bzhi	rcx, rcx, rdx
 1      1     0.20                        cmp	rcx, 1
 1      1     0.50                        sbb	rax, -1
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        test	rcx, rcx
 2      2     1.00                        setg	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     2.00   4.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB358_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	rax, rcx, rdx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	rcx, rcx, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	rcx, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sbb	rax, -1
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	rcx, rcx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setg	al
```
## `unbounded_div_ceil_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      35
Total Cycles:      19
Total uOps:        37

Dispatch Width:    6
uOps Per Cycle:    1.95
IPC:               1.84
Block RThroughput: 8.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	rsi
 0      1     0.00                        mov	rax, rdx
 1      2     0.20                        test	r8b, r8b
 1      1     0.50                        js	.LBB355_1
 0      1     0.00                        mov	r9, rcx
 0      1     0.00                        mov	r10, rcx
 1      0     0.20                        mov	ecx, r8d
 3      5     1.00                        shrd	r9, rax, cl
 1      1     0.20                        mov	rcx, -1
 1      3     0.50                        shlx	r11, rcx, r8
 0      1     0.00                        mov	rdx, rax
 1      1     0.50                        sar	rdx, 63
 1      2     0.20                        xor	esi, esi
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmove	rsi, r11
 1      1     0.50                        cmovne	rcx, r11
 1      3     0.50                        sarx	r8, rax, r8
 1      1     0.50                        cmovne	r9, r8
 1      1     0.50                        cmove	rdx, r8
 1      2     0.33                        andn	rcx, rcx, rax
 1      2     0.33                        andn	r8, rsi, r10
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        or	r8, rcx
 2      2     1.00                        setne	al
 1      1     0.20                        add	rax, r9
 1      1     0.50                        adc	rdx, 0
 1      5     0.33    *                   pop	rsi
 2      7     0.50                  U     ret
 1      2     0.20                        xor	edx, edx
 1      1     0.20                        neg	rcx
 1      1     0.20                        mov	ecx, 0
 1      1     0.50                        sbb	rcx, rax
 2      2     1.00                        setl	al
 1      0     0.20                        movzx	eax, al
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
8.00   5.00   1.00   1.00    -     5.00   9.00    -     1.00   1.00   5.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rdx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r8b, r8b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB355_1
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, r8d
1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -     shrd	r9, rax, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	rcx, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	r11, rcx, r8
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, rax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sar	rdx, 63
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	esi, esi
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	rsi, r11
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rcx, r11
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sarx	r8, rax, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r9, r8
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	rdx, r8
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     andn	rcx, rcx, rax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     andn	r8, rsi, r10
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     or	r8, rcx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setne	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	rax, r9
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     adc	rdx, 0
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rsi
 -      -      -     1.00    -      -     1.00    -      -      -      -      -      -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	edx, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     neg	rcx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, 0
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sbb	rcx, rax
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setl	al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
 -      -     1.00    -      -      -      -      -      -      -      -      -      -     pop	rsi
```
## `unbounded_div_ceil_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      13
Total uOps:        19

Dispatch Width:    6
uOps Per Cycle:    1.46
IPC:               1.15
Block RThroughput: 4.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        cmp	dl, 8
 1      1     0.50                        jae	.LBB364_1
 1      1     0.20                        mov	r9b, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r9b, cl
 1      1     0.20                        not	r9b
 1      0     0.20                        mov	eax, r8d
 2      2     1.00                        shr	al, cl
 1      2     0.20                        and	r9b, r8b
 1      1     0.20                        cmp	r9b, 1
 1      1     0.50                        sbb	al, -1
 2      7     0.50                  U     ret
 1      2     0.20                        test	r8b, r8b
 2      2     1.00                        setne	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   2.00    -      -      -     3.00   5.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 8
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB364_1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r9b, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r9b, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	r9b
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, r8d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	al, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	r9b, r8b
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	r9b, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sbb	al, -1
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r8b, r8b
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setne	al
```
## `unbounded_div_ceil_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      12
Total uOps:        13

Dispatch Width:    6
uOps Per Cycle:    1.08
IPC:               0.92
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 16
 1      1     0.50                        jae	.LBB361_1
 1      1     0.20                        movzx	eax, cx
 1      3     0.50                        shrx	eax, eax, edx
 1      3     1.00                        bzhi	ecx, ecx, edx
 1      1     0.20                        cmp	cx, 1
 1      1     0.50                        sbb	ax, -1
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        test	cx, cx
 2      2     1.00                        setne	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   1.00    -      -      -     2.00   3.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 16
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     jae	.LBB361_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     movzx	eax, cx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shrx	eax, eax, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	ecx, ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	cx, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sbb	ax, -1
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	cx, cx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setne	al
```
## `unbounded_div_ceil_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      11
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.09
IPC:               0.91
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 32
 1      1     0.50                        jae	.LBB362_1
 1      3     0.50                        shrx	eax, ecx, edx
 1      3     1.00                        bzhi	ecx, ecx, edx
 1      1     0.20                        cmp	ecx, 1
 1      1     0.50                        sbb	eax, -1
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        test	ecx, ecx
 2      2     1.00                        setne	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     2.00   4.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 32
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB362_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shrx	eax, ecx, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	ecx, ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	ecx, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sbb	eax, -1
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	ecx, ecx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setne	al
```
## `unbounded_div_ceil_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      11
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.09
IPC:               0.91
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 64
 1      1     0.50                        jae	.LBB363_1
 1      3     0.50                        shrx	rax, rcx, rdx
 1      3     1.00                        bzhi	rcx, rcx, rdx
 1      1     0.20                        cmp	rcx, 1
 1      1     0.50                        sbb	rax, -1
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        test	rcx, rcx
 2      2     1.00                        setne	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     2.00   4.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB363_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shrx	rax, rcx, rdx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	rcx, rcx, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	rcx, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sbb	rax, -1
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	rcx, rcx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setne	al
```
## `unbounded_div_ceil_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      31
Total Cycles:      20
Total uOps:        34

Dispatch Width:    6
uOps Per Cycle:    1.70
IPC:               1.55
Block RThroughput: 7.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	rsi
 1      2     0.20                        test	r8b, r8b
 1      1     0.50                        js	.LBB360_1
 1      1     0.20                        mov	rax, -1
 1      3     0.50                        shlx	r9, rax, r8
 0      1     0.00                        mov	r10, rcx
 0      1     0.00                        mov	r11, rcx
 1      0     0.20                        mov	ecx, r8d
 3      5     1.00                        shrd	r10, rdx, cl
 1      2     0.20                        xor	ecx, ecx
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	rax, r9
 1      1     0.50                        cmovne	r9, rcx
 0      1     0.00                        mov	rsi, rdx
 1      3     0.50                        shrx	rdx, rdx, r8
 1      1     0.50                        cmovne	r10, rdx
 1      1     0.50                        cmovne	rdx, rcx
 1      2     0.33                        andn	rcx, rax, rsi
 1      2     0.33                        andn	r8, r9, r11
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        or	r8, rcx
 2      2     1.00                        setne	al
 1      1     0.20                        add	rax, r10
 1      1     0.50                        adc	rdx, 0
 1      5     0.33    *                   pop	rsi
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        or	rcx, rdx
 2      2     1.00                        setne	al
 1      2     0.20                        xor	edx, edx
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
6.00   4.00   1.00   1.00    -     5.00   9.00    -     1.00   1.00   5.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rsi
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r8b, r8b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB360_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	rax, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	r9, rax, r8
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, rcx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, r8d
1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -     shrd	r10, rdx, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r8b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rax, r9
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r9, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rsi, rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shrx	rdx, rdx, r8
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r10, rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rdx, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     andn	rcx, rax, rsi
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     andn	r8, r9, r11
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     or	r8, rcx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setne	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	rax, r10
1.00    -      -      -      -      -      -      -      -      -      -      -      -     adc	rdx, 0
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rsi
1.00    -      -     1.00    -      -      -      -      -      -      -      -      -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     or	rcx, rdx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setne	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	edx, edx
 -      -     1.00    -      -      -      -      -      -      -      -      -      -     pop	rsi
```
## `unbounded_div_floor_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      7
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.86
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      0     0.20                        movzx	edx, dl
 1      1     0.20                        cmp	dl, 7
 1      1     0.20                        mov	ecx, 7
 1      1     0.50                        cmovb	ecx, edx
 2      2     1.00                        sar	al, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -     1.00   3.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	edx, dl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	dl, 7
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, 7
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovb	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sar	al, cl
```
## `unbounded_div_floor_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      8
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.75
IPC:               0.75
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.33                        movsx	eax, cx
 1      0     0.20                        movzx	ecx, dl
 1      1     0.20                        cmp	cl, 15
 1      1     0.20                        mov	edx, 15
 1      1     0.50                        cmovb	edx, ecx
 1      3     0.50                        sarx	eax, eax, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -     1.00   2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movsx	eax, cx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     movzx	ecx, dl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	cl, 15
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	edx, 15
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovb	edx, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	eax, eax, edx
```
## `unbounded_div_floor_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      8
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.63
IPC:               0.63
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        movzx	eax, dl
 1      1     0.20                        cmp	al, 31
 1      1     0.20                        mov	edx, 31
 1      1     0.50                        cmovb	edx, eax
 1      3     0.50                        sarx	eax, ecx, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00    -      -      -      -     1.00   2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	eax, dl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	al, 31
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, 31
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovb	edx, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	eax, ecx, edx
```
## `unbounded_div_floor_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      8
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.63
IPC:               0.63
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        movzx	eax, dl
 1      1     0.20                        cmp	al, 63
 1      1     0.20                        mov	edx, 63
 1      1     0.50                        cmovb	edx, eax
 1      3     0.50                        sarx	rax, rcx, rdx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00    -      -      -      -     1.00   2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	eax, dl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	al, 63
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, 63
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovb	edx, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	rax, rcx, rdx
```
## `unbounded_div_floor_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      12
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.92
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r8b, r8b
 1      0     0.20                        movzx	r8d, r8b
 1      1     0.20                        mov	ecx, 127
 1      1     0.50                        cmovns	ecx, r8d
 3      5     1.00                        shrd	rax, rdx, cl
 1      3     0.50                        sarx	r8, rdx, rcx
 1      1     0.50                        sar	rdx, 63
 1      2     0.20                        test	cl, 64
 1      1     0.50                        cmovne	rax, r8
 1      1     0.50                        cmove	rdx, r8

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   1.00    -      -      -     2.00   4.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r8b, r8b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     movzx	r8d, r8b
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, 127
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovns	ecx, r8d
1.00   1.00    -      -      -      -      -      -      -      -     1.00    -      -     shrd	rax, rdx, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	r8, rdx, rcx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	rdx, 63
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	cl, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rax, r8
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	rdx, r8
```
## `unbounded_div_floor_u8_unb_pow2`, `unbounded_div_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      6
Total uOps:        8

Dispatch Width:    6
uOps Per Cycle:    1.33
IPC:               1.17
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shr	al, cl
 1      2     0.20                        xor	ecx, ecx
 1      1     0.20                        cmp	dl, 8
 1      0     0.20                        movzx	eax, al
 1      1     0.50                        cmovae	eax, ecx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	al, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	ecx, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 8
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	eax, al
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovae	eax, ecx
```
## `unbounded_div_floor_u16_unb_pow2`, `unbounded_div_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      8
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.63
IPC:               0.63
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        movzx	eax, cx
 1      2     0.20                        xor	ecx, ecx
 1      1     0.20                        cmp	dl, 16
 1      3     0.50                        shrx	eax, eax, edx
 1      1     0.50                        cmovae	eax, ecx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00    -      -      -      -     1.00   2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	eax, cx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	dl, 16
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shrx	eax, eax, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovae	eax, ecx
```
## `unbounded_div_floor_u32_unb_pow2`, `unbounded_div_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      7
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.57
IPC:               0.57
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      2     0.20                        xor	r8d, r8d
 1      1     0.20                        cmp	dl, 32
 1      3     0.50                        shrx	eax, ecx, edx
 1      1     0.50                        cmovae	eax, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00    -      -      -      -      -     2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	r8d, r8d
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 32
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shrx	eax, ecx, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovae	eax, r8d
```
## `unbounded_div_floor_u64_unb_pow2`, `unbounded_div_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      7
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.57
IPC:               0.57
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 64
 1      3     0.50                        shrx	rcx, rcx, rdx
 1      1     0.50                        cmovb	rax, rcx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00    -      -      -      -      -     2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shrx	rcx, rcx, rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovb	rax, rcx
```
## `unbounded_div_floor_u128_unb_pow2`, `unbounded_div_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      11
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.09
IPC:               1.00
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      0     0.20                        mov	ecx, r8d
 3      5     1.00                        shrd	rax, rdx, cl
 1      3     0.50                        shrx	rdx, rdx, r8
 1      2     0.20                        xor	ecx, ecx
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	rax, rdx
 1      1     0.50                        cmovne	rdx, rcx
 1      2     0.20                        test	r8b, r8b
 1      1     0.50                        cmovs	rax, rcx
 1      1     0.50                        cmovs	rdx, rcx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   1.00    -      -      -     2.00   4.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, r8d
1.00   1.00    -      -      -      -      -      -      -      -     1.00    -      -     shrd	rax, rdx, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shrx	rdx, rdx, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     test	r8b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rax, rdx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rdx, rcx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r8b, r8b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovs	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovs	rdx, rcx
```
## `unbounded_div_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      14
Total uOps:        17

Dispatch Width:    6
uOps Per Cycle:    1.21
IPC:               1.00
Block RThroughput: 3.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 7
 1      1     0.50                        ja	.LBB379_1
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        mov	r9b, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r9b, cl
 1      1     0.20                        not	r9b
 1      0     0.20                        mov	eax, r8d
 1      1     0.50                        sar	al, 7
 1      2     0.20                        and	al, r9b
 1      1     0.20                        add	al, r8b
 2      2     1.00                        sar	al, cl
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   2.00    -      -      -     3.00   4.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 7
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB379_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r9b, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r9b, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	r9b
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, r8d
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	al, 7
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	al, r9b
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	al, r8b
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	al, cl
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
```
## `unbounded_div_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      15
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    0.73
IPC:               0.67
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 15
 1      1     0.50                        ja	.LBB376_1
 1      1     0.33                        movsx	eax, cx
 1      1     0.50                        sar	eax, 15
 1      3     1.00                        bzhi	eax, eax, edx
 1      1     0.20                        add	eax, ecx
 1      1     0.50                        cwde
 1      3     0.50                        sarx	eax, eax, edx
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   2.00    -      -      -     2.00   2.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 15
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB376_1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movsx	eax, cx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	eax, 15
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	eax, eax, edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	eax, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cwde
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sarx	eax, eax, edx
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
```
## `unbounded_div_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      11
Total uOps:        8

Dispatch Width:    6
uOps Per Cycle:    0.73
IPC:               0.73
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 31
 1      1     0.50                        ja	.LBB377_2
 1      0     0.20                        mov	eax, ecx
 1      1     0.50                        sar	eax, 31
 1      3     1.00                        bzhi	eax, eax, edx
 1      1     0.20                        add	eax, ecx
 1      3     0.50                        sarx	eax, eax, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 31
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB377_2
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	eax, ecx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	eax, 31
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	eax, eax, edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	eax, ecx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sarx	eax, eax, edx
```
## `unbounded_div_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      14
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    0.64
IPC:               0.64
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 63
 1      1     0.50                        ja	.LBB378_1
 0      1     0.00                        mov	rax, rcx
 1      1     0.50                        sar	rax, 63
 1      3     1.00                        bzhi	rax, rax, rdx
 1      1     0.20                        add	rax, rcx
 1      3     0.50                        sarx	rax, rax, rdx
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   2.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 63
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB378_1
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, 63
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	rax, rax, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	rax, rcx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sarx	rax, rax, rdx
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
```
## `unbounded_div_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      26
Total Cycles:      20
Total uOps:        27

Dispatch Width:    6
uOps Per Cycle:    1.35
IPC:               1.30
Block RThroughput: 6.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      2     0.20                        test	r8b, r8b
 1      1     0.50                        js	.LBB375_1
 1      1     0.20                        mov	rax, -1
 1      3     0.50                        shlx	r9, rax, r8
 1      2     0.20                        xor	r10d, r10d
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmove	r10, r9
 1      1     0.50                        cmovne	rax, r9
 0      1     0.00                        mov	r9, rdx
 1      1     0.50                        sar	r9, 63
 0      1     0.00                        mov	r11, rdx
 1      2     0.33                        andn	rdx, rax, r9
 1      2     0.33                        andn	rax, r10, r9
 1      1     0.20                        add	rax, rcx
 1      1     0.50                        adc	rdx, r11
 1      0     0.20                        mov	ecx, r8d
 3      5     1.00                        shrd	rax, rdx, cl
 1      2     0.20                        test	r8b, 64
 1      3     0.50                        sarx	rcx, rdx, r8
 1      1     0.50                        cmovne	rax, rcx
 1      1     0.50                        sar	rdx, 63
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmove	rdx, rcx
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        xor	edx, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
6.00   4.00    -      -      -     4.00   7.00    -      -      -     5.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r8b, r8b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB375_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	rax, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	r9, rax, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	r10d, r10d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	r10, r9
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rax, r9
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rdx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	r9, 63
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, rdx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     andn	rdx, rax, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     andn	rax, r10, r9
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	rax, rcx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     adc	rdx, r11
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, r8d
 -     1.00    -      -      -      -     1.00    -      -      -     1.00    -      -     shrd	rax, rdx, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r8b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sarx	rcx, rdx, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sar	rdx, 63
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	rdx, rcx
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	edx, edx
```
## `unbounded_div_round_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      28
Total Cycles:      15
Total uOps:        33

Dispatch Width:    6
uOps Per Cycle:    2.20
IPC:               1.87
Block RThroughput: 6.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        cmp	dl, 7
 1      1     0.50                        ja	.LBB384_2
 1      1     0.20                        mov	r9b, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r9b, cl
 1      0     0.20                        mov	eax, r8d
 2      2     1.00                        sar	al, cl
 1      1     0.20                        not	r9b
 1      2     0.20                        and	r9b, r8b
 1      1     0.50                        shr	r8b, 7
 1      2     0.20                        xor	ecx, ecx
 1      1     0.20                        sub	r9b, r8b
 1      0     0.20                        movzx	r8d, r9b
 1      1     0.50                        cmovb	r8d, ecx
 1      1     0.20                        add	r8b, r8b
 1      0     0.20                        movzx	ecx, r8b
 1      0     0.20                        movzx	edx, dl
 1      1     1.00                        bt	ecx, edx
 1      1     0.50                        adc	al, 0
 2      7     0.50                  U     ret
 1      1     0.20                        neg	r8b
 1      1     0.50                        jno	.LBB384_3
 1      1     0.20                        cmp	dl, 8
 2      2     1.00                        sete	al
 1      1     0.20                        neg	al
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
7.00   6.00    -     1.00    -     5.00   8.00    -      -      -     5.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 7
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB384_2
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r9b, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r9b, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, r8d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	al, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	r9b
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	r9b, r8b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shr	r8b, 7
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     sub	r9b, r8b
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movzx	r8d, r9b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovb	r8d, ecx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	r8b, r8b
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movzx	ecx, r8b
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	edx, dl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bt	ecx, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     adc	al, 0
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
1.00    -      -      -      -      -      -      -      -      -      -      -      -     neg	r8b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     jno	.LBB384_3
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 8
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sete	al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     neg	al
1.00    -      -     1.00    -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
```
## `unbounded_div_round_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      21
Total Cycles:      14
Total uOps:        23

Dispatch Width:    6
uOps Per Cycle:    1.64
IPC:               1.50
Block RThroughput: 4.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 15
 1      1     0.50                        ja	.LBB381_2
 1      1     0.33                        movsx	eax, cx
 1      3     0.50                        sarx	eax, eax, edx
 1      1     0.20                        movzx	r8d, cx
 1      3     1.00                        bzhi	ecx, ecx, edx
 1      1     0.50                        shr	r8d, 15
 1      2     0.20                        xor	r9d, r9d
 1      1     0.20                        sub	cx, r8w
 1      1     0.50                        cmovb	ecx, r9d
 1      1     0.20                        add	ecx, ecx
 1      0     0.20                        movzx	edx, dl
 1      1     1.00                        bt	ecx, edx
 1      1     0.50                        adc	ax, 0
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        neg	cx
 1      1     0.50                        jno	.LBB381_4
 1      1     0.20                        cmp	dl, 16
 2      2     1.00                        sete	al
 1      1     0.20                        neg	eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   4.00    -      -      -     4.00   6.00    -      -      -     4.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 15
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB381_2
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movsx	eax, cx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	eax, eax, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     movzx	r8d, cx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	ecx, ecx, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r8d, 15
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	r9d, r9d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     sub	cx, r8w
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovb	ecx, r9d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	ecx, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movzx	edx, dl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bt	ecx, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     adc	ax, 0
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     neg	cx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     jno	.LBB381_4
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 16
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sete	al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     neg	eax
```
## `unbounded_div_round_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      20
Total Cycles:      16
Total uOps:        22

Dispatch Width:    6
uOps Per Cycle:    1.38
IPC:               1.25
Block RThroughput: 4.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 31
 1      1     0.50                        ja	.LBB382_2
 1      3     0.50                        sarx	eax, ecx, edx
 1      3     1.00                        bzhi	r8d, ecx, edx
 1      1     0.50                        shr	ecx, 31
 1      2     0.20                        xor	r9d, r9d
 1      1     0.20                        sub	r8d, ecx
 1      1     0.50                        cmovb	r8d, r9d
 1      1     0.20                        add	r8d, r8d
 1      0     0.20                        movzx	ecx, dl
 1      1     1.00                        bt	r8d, ecx
 1      1     0.50                        adc	eax, 0
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        neg	ecx
 1      1     0.50                        jno	.LBB382_4
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 32
 2      2     1.00                        sete	al
 1      1     0.20                        neg	eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   4.00    -      -      -     4.00   5.00    -      -      -     4.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 31
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB382_2
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	eax, ecx, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	r8d, ecx, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	ecx, 31
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	r9d, r9d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     sub	r8d, ecx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovb	r8d, r9d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	r8d, r8d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	ecx, dl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bt	r8d, ecx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     adc	eax, 0
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     neg	ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     jno	.LBB382_4
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 32
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sete	al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     neg	eax
```
## `unbounded_div_round_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      19
Total Cycles:      16
Total uOps:        21

Dispatch Width:    6
uOps Per Cycle:    1.31
IPC:               1.19
Block RThroughput: 4.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 63
 1      1     0.50                        ja	.LBB383_2
 1      3     0.50                        sarx	rax, rcx, rdx
 1      3     1.00                        bzhi	r8, rcx, rdx
 1      1     0.50                        shr	rcx, 63
 1      2     0.20                        xor	r9d, r9d
 1      1     0.20                        sub	r8, rcx
 1      1     0.50                        cmovae	r9, r8
 1      1     0.20                        add	r9, r9
 1      0     0.20                        movzx	ecx, dl
 1      3     1.00                        bt	r9, rcx
 1      1     0.50                        adc	rax, 0
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        neg	rcx
 1      1     0.50                        jno	.LBB383_4
 1      1     0.20                        cmp	dl, 64
 2      2     1.00                        sete	al
 1      1     0.20                        neg	rax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   3.00    -      -      -     4.00   4.00    -      -      -     4.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 63
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB383_2
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sarx	rax, rcx, rdx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	r8, rcx, rdx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	rcx, 63
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	r9d, r9d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     sub	r8, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovae	r9, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	r9, r9
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	ecx, dl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bt	r9, rcx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     adc	rax, 0
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     neg	rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     jno	.LBB383_4
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 64
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sete	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     neg	rax
```
## `unbounded_div_round_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      48
Total Cycles:      31
Total uOps:        49

Dispatch Width:    6
uOps Per Cycle:    1.58
IPC:               1.55
Block RThroughput: 10.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      2     0.20                        test	r8b, r8b
 1      1     0.50                        js	.LBB380_2
 1      1     0.20                        mov	rax, -1
 1      3     0.50                        shlx	r9, rax, r8
 1      2     0.20                        xor	r10d, r10d
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	rax, r9
 1      1     0.50                        cmovne	r9, r10
 1      2     0.33                        andn	rax, rax, rdx
 1      2     0.33                        andn	r9, r9, rcx
 0      1     0.00                        mov	r11, rdx
 1      1     0.50                        shr	r11, 63
 1      1     0.20                        sub	r9, r11
 1      1     0.50                        sbb	rax, 0
 1      1     0.50                        cmovb	rax, r10
 1      1     0.50                        cmovb	r9, r10
 1      3     1.00                        shld	rax, r9, 1
 1      1     0.20                        add	r9, r9
 0      1     0.00                        mov	r10, rcx
 1      0     0.20                        mov	ecx, r8d
 3      5     1.00                        shrd	r9, rax, cl
 1      2     0.20                        test	r8b, 64
 1      3     0.50                        shrx	rax, rax, r8
 1      1     0.50                        cmove	rax, r9
 0      1     0.00                        mov	r9, rdx
 1      1     0.50                        sar	r9, 63
 1      2     0.20                        test	r8b, 64
 1      3     0.50                        sarx	r11, rdx, r8
 1      1     0.50                        cmove	r9, r11
 3      5     1.00                        shrd	r10, rdx, cl
 0      1     0.00                        mov	rdx, r9
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	r10, r11
 1      2     0.20                        and	eax, 1
 1      1     0.20                        add	rax, r10
 1      1     0.50                        adc	rdx, 0
 2      7     0.50                  U     ret
 1      1     0.20                        movabs	rax, -9223372036854775808
 1      2     0.20                        xor	rdx, rax
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        or	rcx, rdx
 1      1     0.20                        mov	edx, 0
 1      1     0.50                        jne	.LBB380_4
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        neg	r8b
 2      2     1.00                        seto	al
 1      1     0.20                        neg	rax
 0      1     0.00                        mov	rdx, rax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
12.00  8.00    -      -      -     8.00   11.00   -      -      -     9.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r8b, r8b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB380_2
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	rax, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	r9, rax, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	r10d, r10d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rax, r9
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r9, r10
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     andn	rax, rax, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     andn	r9, r9, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, rdx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r11, 63
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     sub	r9, r11
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sbb	rax, 0
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovb	rax, r10
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovb	r9, r10
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     shld	rax, r9, 1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	r9, r9
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, r8d
1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -     shrd	r9, rax, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shrx	rax, rax, r8
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	rax, r9
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sar	r9, 63
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sarx	r11, rdx, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	r9, r11
 -     1.00    -      -      -      -     1.00    -      -      -     1.00    -      -     shrd	r10, rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r9
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r10, r11
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	eax, 1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	rax, r10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     adc	rdx, 0
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movabs	rax, -9223372036854775808
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	rdx, rax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     or	rcx, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, 0
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jne	.LBB380_4
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     neg	r8b
2.00    -      -      -      -      -      -      -      -      -      -      -      -     seto	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     neg	rax
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, rax
```
## `unbounded_div_round_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      13
Total Cycles:      14
Total uOps:        16

Dispatch Width:    6
uOps Per Cycle:    1.14
IPC:               0.93
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 7
 1      1     0.50                        ja	.LBB389_1
 1      0     0.20                        mov	eax, ecx
 1      1     0.20                        mov	r8b, 1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r8b, cl
 1      1     0.50                        shr	r8b
 1      2     0.20                        and	r8b, al
 2      2     1.00                        shr	al, cl
 1      1     0.20                        cmp	r8b, 1
 1      1     0.50                        sbb	al, -1
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   2.00    -      -      -     2.00   5.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 7
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB389_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8b, 1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r8b, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shr	r8b
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	r8b, al
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	al, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	r8b, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sbb	al, -1
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
```
## `unbounded_div_round_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      13
Total Cycles:      15
Total uOps:        14

Dispatch Width:    6
uOps Per Cycle:    0.93
IPC:               0.87
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 15
 1      1     0.50                        ja	.LBB386_1
 1      1     0.20                        mov	eax, 1
 1      3     0.50                        shlx	eax, eax, edx
 1      1     0.20                        movzx	r8d, ax
 1      1     0.50                        shr	r8d
 1      1     0.20                        movzx	ecx, cx
 1      3     0.50                        shrx	eax, ecx, edx
 1      2     0.20                        and	ecx, r8d
 1      1     0.20                        cmp	cx, 1
 1      1     0.50                        sbb	ax, -1
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   2.00    -      -      -     2.00   4.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 15
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB386_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	eax, eax, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movzx	r8d, ax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r8d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movzx	ecx, cx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shrx	eax, ecx, edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	ecx, r8d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	cx, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sbb	ax, -1
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
```
## `unbounded_div_round_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      12
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    0.83
IPC:               0.83
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 31
 1      1     0.50                        ja	.LBB387_2
 1      1     0.20                        mov	eax, 1
 1      3     0.50                        shlx	r8d, eax, edx
 1      1     0.50                        shr	r8d
 1      3     0.50                        shrx	eax, ecx, edx
 1      2     0.20                        and	r8d, ecx
 1      1     0.20                        cmp	r8d, 1
 1      1     0.50                        sbb	eax, -1

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   1.00    -      -      -     1.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 31
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB387_2
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	eax, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	r8d, eax, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shr	r8d
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shrx	eax, ecx, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	r8d, ecx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	r8d, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sbb	eax, -1
```
## `unbounded_div_round_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      14
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    0.86
IPC:               0.79
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 63
 1      1     0.50                        ja	.LBB388_1
 1      1     0.20                        mov	eax, 1
 1      3     0.50                        shlx	r8, rax, rdx
 1      1     0.50                        shr	r8
 1      3     0.50                        shrx	rax, rcx, rdx
 1      2     0.20                        and	r8, rcx
 1      1     0.20                        cmp	r8, 1
 1      1     0.50                        sbb	rax, -1
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   1.00    -      -      -     1.00   4.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 63
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB388_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	r8, rax, rdx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r8
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shrx	rax, rcx, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	r8, rcx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	r8, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sbb	rax, -1
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
```
## `unbounded_div_round_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      41
Total Cycles:      30
Total uOps:        44

Dispatch Width:    6
uOps Per Cycle:    1.47
IPC:               1.37
Block RThroughput: 7.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	r14
 2      1     0.50           *            push	rsi
 2      1     0.50           *            push	rdi
 2      1     0.50           *            push	rbx
 1      2     0.20                        test	r8b, r8b
 1      1     0.50                        js	.LBB385_1
 1      3     0.50                        shrx	rax, rdx, r8
 1      2     0.20                        xor	r10d, r10d
 1      2     0.20                        test	r8b, 64
 0      1     0.00                        mov	r11, rdx
 0      1     0.00                        mov	rdx, rax
 1      1     0.50                        cmovne	rdx, r10
 0      1     0.00                        mov	r9, rcx
 0      1     0.00                        mov	rsi, rcx
 1      0     0.20                        mov	ecx, r8d
 3      5     1.00                        shrd	r9, r11, cl
 1      2     0.20                        test	r8b, 64
 1      1     0.20                        mov	edi, 1
 1      3     0.50                        shlx	rbx, rdi, r8
 0      1     0.00                        mov	r14, rbx
 1      1     0.50                        cmovne	r14, r10
 1      1     0.50                        cmovne	r9, rax
 3      5     1.00                        shld	r10, rdi, cl
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	r10, rbx
 1      3     1.00                        shrd	r14, r10, 1
 1      1     0.50                        shr	r10
 1      2     0.20                        and	r10, r11
 1      2     0.20                        and	r14, rsi
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        or	r14, r10
 2      2     1.00                        setne	al
 1      1     0.20                        add	rax, r9
 1      1     0.50                        adc	rdx, 0
 0      0     0.00                        jmp	.LBB385_3
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        xor	edx, edx
 1      5     0.33    *                   pop	rbx
 1      5     0.33    *                   pop	rdi
 1      5     0.33    *                   pop	rsi
 1      5     0.33    *                   pop	r14

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
7.00   5.00   1.00   1.00   2.00   6.00   8.00   2.00   2.00   2.00   6.00   2.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	r14
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     push	rsi
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rdi
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     push	rbx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r8b, r8b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB385_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shrx	rax, rdx, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	r10d, r10d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, rax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rdx, r10
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rsi, rcx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
1.00   1.00    -      -      -      -      -      -      -      -     1.00    -      -     shrd	r9, r11, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	edi, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	rbx, rdi, r8
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r14, rbx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r14, r10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r9, rax
1.00   1.00    -      -      -      -      -      -      -      -     1.00    -      -     shld	r10, rdi, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r10, rbx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     shrd	r14, r10, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	r10, r11
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	r14, rsi
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     or	r14, r10
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setne	al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	rax, r9
1.00    -      -      -      -      -      -      -      -      -      -      -      -     adc	rdx, 0
 -      -      -      -      -      -      -      -      -      -      -      -      -     jmp	.LBB385_3
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	edx, edx
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rbx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -     pop	rdi
 -      -     1.00    -      -      -      -      -      -      -      -      -      -     pop	rsi
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	r14
```
## `unbounded_floor_to_multiple_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      13
Total Cycles:      11
Total uOps:        17

Dispatch Width:    6
uOps Per Cycle:    1.55
IPC:               1.18
Block RThroughput: 4.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      0     0.20                        mov	r8d, ecx
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shr	r8b, cl
 2      2     1.00                        shl	r8b, cl
 1      2     0.20                        test	al, al
 2      2     1.00                        setns	cl
 1      2     0.20                        xor	r9d, r9d
 1      1     0.20                        cmp	dl, 8
 2      2     1.00                        setb	al
 1      0     0.20                        movzx	edx, r8b
 1      1     0.50                        cmovae	edx, r9d
 1      2     0.20                        or	al, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   2.00    -      -      -     2.00   5.00    -      -      -     3.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r8b, cl
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r8b, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	al, al
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setns	cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	r9d, r9d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	dl, 8
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setb	al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movzx	edx, r8b
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovae	edx, r9d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     or	al, cl
```
## `unbounded_floor_to_multiple_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      12
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.92
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 16
 1      1     0.50                        jae	.LBB396_1
 1      1     0.20                        movzx	eax, cx
 1      3     0.50                        shrx	eax, eax, edx
 1      3     0.50                        shlx	edx, eax, edx
 1      1     0.20                        mov	ax, 1
 2      7     0.50                  U     ret
 1      1     0.20                        not	ecx
 1      1     0.20                        movzx	eax, cx
 1      1     0.50                        shr	eax, 15
 1      2     0.20                        xor	edx, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   1.00    -      -      -     2.00   3.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 16
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB396_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     movzx	eax, cx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shrx	eax, eax, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	edx, eax, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ax, 1
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	ecx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	eax, cx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shr	eax, 15
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	edx, edx
```
## `unbounded_floor_to_multiple_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      11
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.91
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      1     0.20                        cmp	dl, 32
 1      1     0.50                        jae	.LBB397_1
 1      3     0.50                        shrx	eax, eax, edx
 1      3     0.50                        shlx	edx, eax, edx
 1      1     0.20                        mov	eax, 1
 2      7     0.50                  U     ret
 1      1     0.20                        not	eax
 1      1     0.50                        shr	eax, 31
 1      2     0.20                        xor	edx, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   1.00    -      -      -     1.00   3.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 32
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     jae	.LBB397_1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shrx	eax, eax, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	edx, eax, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	eax, 1
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shr	eax, 31
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	edx, edx
```
## `unbounded_floor_to_multiple_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      12
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    0.83
IPC:               0.83
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      1     0.20                        cmp	dl, 64
 1      1     0.50                        jae	.LBB398_1
 1      3     0.50                        shrx	rax, rax, rdx
 1      3     0.50                        shlx	rdx, rax, rdx
 1      1     0.20                        mov	eax, 1
 2      7     0.50                  U     ret
 1      1     0.20                        not	rax
 1      1     0.50                        shr	rax, 63
 1      2     0.20                        xor	edx, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   1.00    -      -      -     1.00   3.00    -      -      -     1.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB398_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shrx	rax, rax, rdx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	rdx, rax, rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, 1
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	rax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shr	rax, 63
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	edx, edx
```
## `unbounded_floor_to_multiple_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      24
Total Cycles:      25
Total uOps:        30

Dispatch Width:    6
uOps Per Cycle:    1.20
IPC:               0.96
Block RThroughput: 5.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB395_1
 1      1     0.20                        mov	rcx, -1
 1      3     0.50                        shlx	r10, rcx, r9
 1      2     0.20                        xor	r11d, r11d
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	rcx, r10
 1      1     0.50                        cmove	r11, r10
 1      2     0.20                        and	r11, rdx
 1      2     0.20                        and	rcx, r8
 2      12    0.50           *            mov	qword ptr [rax + 24], rcx
 2      12    0.50           *            mov	qword ptr [rax + 16], r11
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 2      12    0.50           *            mov	qword ptr [rax], 1
 2      7     0.50                  U     ret
 1      2     0.20                        test	r8, r8
 1      1     0.50                        js	.LBB395_2
 1      7     0.33    *                   vmovss	xmm0, dword ptr [rip + __real@00000001]
 2      12    0.50           *            vmovups	ymmword ptr [rax], ymm0
 0      0     0.00                  U     vzeroupper
 2      7     0.50                  U     ret
 1      1     0.33                        vxorps	xmm0, xmm0, xmm0
 2      12    0.50           *            vmovaps	xmmword ptr [rax], xmm0

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   2.00   1.00   1.00   3.00   2.00   4.00   3.00   3.00   3.00   3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, r9b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB395_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	rcx, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	r10, rcx, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	r11d, r11d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rcx, r10
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	r11, r10
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	r11, rdx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	rcx, r8
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 24], rcx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 16], r11
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 8], 0
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax], 1
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r8, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB395_2
 -      -      -     1.00    -      -      -      -      -      -      -      -      -     vmovss	xmm0, dword ptr [rip + __real@00000001]
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     vmovups	ymmword ptr [rax], ymm0
 -      -      -      -      -      -      -      -      -      -      -      -      -     vzeroupper
1.00    -     1.00    -      -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     vxorps	xmm0, xmm0, xmm0
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     vmovaps	xmmword ptr [rax], xmm0
```
## `unbounded_floor_to_multiple_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      8
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.25
IPC:               1.00
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shr	al, cl
 2      2     1.00                        shl	al, cl
 1      2     0.20                        xor	ecx, ecx
 1      1     0.20                        cmp	dl, 8
 1      0     0.20                        movzx	eax, al
 1      1     0.50                        cmovae	eax, ecx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   1.00    -      -      -     1.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	al, cl
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	al, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	ecx, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 8
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	eax, al
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovae	eax, ecx
```
## `unbounded_floor_to_multiple_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      11
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.55
IPC:               0.55
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        movzx	eax, cx
 1      2     0.20                        xor	ecx, ecx
 1      1     0.20                        cmp	dl, 16
 1      3     0.50                        shrx	eax, eax, edx
 1      3     0.50                        shlx	eax, eax, edx
 1      1     0.50                        cmovae	eax, ecx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00    -      -      -      -     1.00   2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	eax, cx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	dl, 16
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shrx	eax, eax, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	eax, eax, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovae	eax, ecx
```
## `unbounded_floor_to_multiple_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      10
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.50
IPC:               0.50
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      2     0.20                        xor	r8d, r8d
 1      1     0.20                        cmp	dl, 32
 1      3     0.50                        shrx	eax, ecx, edx
 1      3     0.50                        shlx	eax, eax, edx
 1      1     0.50                        cmovae	eax, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00    -      -      -      -      -     2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	r8d, r8d
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 32
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shrx	eax, ecx, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	eax, eax, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovae	eax, r8d
```
## `unbounded_floor_to_multiple_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      10
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.50
IPC:               0.50
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 64
 1      3     0.50                        shrx	rcx, rcx, rdx
 1      3     0.50                        shlx	rcx, rcx, rdx
 1      1     0.50                        cmovb	rax, rcx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00    -      -      -      -      -     2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shrx	rcx, rcx, rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	rcx, rcx, rdx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovb	rax, rcx
```
## `unbounded_floor_to_multiple_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      11
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        mov	r9, -1
 1      3     0.50                        shlx	rax, r9, r8
 1      2     0.20                        xor	r10d, r10d
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	r9, rax
 1      1     0.50                        cmovne	rax, r10
 1      2     0.20                        and	rdx, r9
 1      2     0.20                        and	rax, rcx
 1      2     0.20                        test	r8b, r8b
 1      1     0.50                        cmovs	rax, r10
 1      1     0.50                        cmovs	rdx, r10

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   1.00    -      -      -     2.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r9, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	rax, r9, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	r10d, r10d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r9, rax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rax, r10
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	rdx, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	rax, rcx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r8b, r8b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovs	rax, r10
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovs	rdx, r10
```
## `unbounded_is_multiple_of_i8_unb_pow2`, `unbounded_is_multiple_of_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      11
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    0.82
IPC:               0.64
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      2     0.20                        test	cl, cl
 2      2     1.00                        sete	r8b
 1      0     0.20                        movzx	eax, cl
 1      3     1.00                        tzcnt	eax, eax
 1      1     0.20                        cmp	al, dl
 2      2     1.00                        setae	al
 1      2     0.20                        or	al, r8b

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	cl, cl
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sete	r8b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     movzx	eax, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     tzcnt	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	al, dl
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setae	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     or	al, r8b
```
## `unbounded_is_multiple_of_i16_unb_pow2`, `unbounded_is_multiple_of_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      11
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    0.82
IPC:               0.64
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      2     0.20                        test	cx, cx
 2      2     1.00                        sete	r8b
 1      3     1.00                        tzcnt	eax, ecx
 1      0     0.20                        movzx	ecx, dl
 1      1     0.20                        cmp	ax, cx
 2      2     1.00                        setae	al
 1      2     0.20                        or	al, r8b

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	cx, cx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sete	r8b
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     tzcnt	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     movzx	ecx, dl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	ax, cx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setae	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     or	al, r8b
```
## `unbounded_is_multiple_of_i32_unb_pow2`, `unbounded_is_multiple_of_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      11
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    0.82
IPC:               0.64
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      2     0.20                        test	ecx, ecx
 2      2     1.00                        sete	r8b
 1      3     1.00                        tzcnt	eax, ecx
 1      0     0.20                        movzx	ecx, dl
 1      1     0.20                        cmp	eax, ecx
 2      2     1.00                        setae	al
 1      2     0.20                        or	al, r8b

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	ecx, ecx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sete	r8b
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     tzcnt	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     movzx	ecx, dl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	eax, ecx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setae	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     or	al, r8b
```
## `unbounded_is_multiple_of_i64_unb_pow2`, `unbounded_is_multiple_of_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      11
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    0.82
IPC:               0.64
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      2     0.20                        test	rcx, rcx
 2      2     1.00                        sete	r8b
 1      3     1.00                        tzcnt	rax, rcx
 1      0     0.20                        movzx	ecx, dl
 1      1     0.20                        cmp	rax, rcx
 2      2     1.00                        setae	al
 1      2     0.20                        or	al, r8b

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	rcx, rcx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sete	r8b
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     tzcnt	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     movzx	ecx, dl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	rax, rcx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setae	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     or	al, r8b
```
## `unbounded_is_multiple_of_i128_unb_pow2`, `unbounded_is_multiple_of_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      12
Total Cycles:      14
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    0.86
IPC:               0.86
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        or	rax, rdx
 2      2     1.00                        sete	r9b
 1      3     1.00                        tzcnt	rax, rcx
 1      3     1.00                        tzcnt	rdx, rdx
 0      1     0.00                        add	rdx, 64
 1      2     0.20                        test	rcx, rcx
 1      1     0.50                        cmovne	rdx, rax
 1      0     0.20                        movzx	eax, r8b
 1      1     0.20                        cmp	edx, eax
 2      2     1.00                        setae	al
 1      2     0.20                        or	al, r9b

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   2.00    -      -      -     2.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     or	rax, rdx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sete	r9b
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     tzcnt	rax, rcx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     tzcnt	rdx, rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -     add	rdx, 64
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	rcx, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rdx, rax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movzx	eax, r8b
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	edx, eax
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setae	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     or	al, r9b
```
## `unbounded_rem_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      13
Total uOps:        15

Dispatch Width:    6
uOps Per Cycle:    1.15
IPC:               1.08
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      1     0.20                        cmp	dl, 7
 1      1     0.50                        ja	.LBB419_2
 1      1     0.20                        mov	r8b, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r8b, cl
 1      0     0.20                        mov	ecx, r8d
 1      1     0.20                        not	cl
 1      0     0.20                        mov	edx, eax
 1      1     0.50                        sar	dl, 7
 1      2     0.20                        and	dl, cl
 1      1     0.20                        add	dl, al
 1      2     0.20                        and	dl, r8b
 1      1     0.20                        sub	al, dl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   3.00    -      -      -     3.00   4.00    -      -      -     3.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 7
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB419_2
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8b, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r8b, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, r8d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	edx, eax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	dl, 7
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	dl, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     add	dl, al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	dl, r8b
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     sub	al, dl
```
## `unbounded_rem_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      13
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    0.85
IPC:               0.85
Block RThroughput: 1.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      1     0.20                        cmp	dl, 15
 1      1     0.50                        ja	.LBB416_2
 1      1     0.20                        mov	ecx, -1
 1      3     0.50                        shlx	ecx, ecx, edx
 1      1     0.33                        movsx	r8d, ax
 1      1     0.50                        sar	r8d, 15
 1      3     1.00                        bzhi	edx, r8d, edx
 1      1     0.20                        add	edx, eax
 1      2     0.20                        and	edx, ecx
 1      1     0.20                        sub	eax, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   2.00    -      -      -     2.00   2.00    -      -      -     3.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 15
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB416_2
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	ecx, ecx, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movsx	r8d, ax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	r8d, 15
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	edx, r8d, edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	edx, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	edx, ecx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     sub	eax, edx
```
## `unbounded_rem_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      13
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    0.85
IPC:               0.85
Block RThroughput: 1.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      1     0.20                        cmp	dl, 31
 1      1     0.50                        ja	.LBB417_2
 1      1     0.20                        mov	ecx, -1
 1      3     0.50                        shlx	ecx, ecx, edx
 1      0     0.20                        mov	r8d, eax
 1      1     0.50                        sar	r8d, 31
 1      3     1.00                        bzhi	edx, r8d, edx
 1      1     0.20                        add	edx, eax
 1      2     0.20                        and	edx, ecx
 1      1     0.20                        sub	eax, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   2.00    -      -      -     2.00   2.00    -      -      -     3.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 31
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB417_2
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	ecx, ecx, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r8d, eax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	r8d, 31
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	edx, r8d, edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	edx, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	edx, ecx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     sub	eax, edx
```
## `unbounded_rem_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      13
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    0.69
IPC:               0.85
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      1     0.20                        cmp	dl, 63
 1      1     0.50                        ja	.LBB418_2
 1      1     0.20                        mov	rcx, -1
 1      3     0.50                        shlx	rcx, rcx, rdx
 0      1     0.00                        mov	r8, rax
 1      1     0.50                        sar	r8, 63
 1      3     1.00                        bzhi	rdx, r8, rdx
 1      1     0.20                        add	rdx, rax
 1      2     0.20                        and	rdx, rcx
 1      1     0.20                        sub	rax, rdx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     2.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 63
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB418_2
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	rcx, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	rcx, rcx, rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	r8, 63
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     bzhi	rdx, r8, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	rdx, rax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	rdx, rcx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     sub	rax, rdx
```
## `unbounded_rem_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      19
Total Cycles:      15
Total uOps:        17

Dispatch Width:    6
uOps Per Cycle:    1.13
IPC:               1.27
Block RThroughput: 3.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r8b, r8b
 1      1     0.50                        js	.LBB415_2
 1      1     0.20                        mov	rcx, -1
 1      3     0.50                        shlx	r9, rcx, r8
 1      2     0.20                        xor	r10d, r10d
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmove	r10, r9
 1      1     0.50                        cmovne	rcx, r9
 0      1     0.00                        mov	r8, rdx
 1      1     0.50                        sar	r8, 63
 1      2     0.33                        andn	r9, rcx, r8
 1      2     0.33                        andn	r8, r10, r8
 1      1     0.20                        add	r8, rax
 1      1     0.50                        adc	r9, rdx
 1      2     0.20                        and	r9, rcx
 1      2     0.20                        and	r8, r10
 1      1     0.20                        sub	rax, r8
 1      1     0.50                        sbb	rdx, r9

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   3.00    -      -      -     3.00   4.00    -      -      -     3.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r8b, r8b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB415_2
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	rcx, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	r9, rcx, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	r10d, r10d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	r10, r9
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rcx, r9
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rdx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	r8, 63
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     andn	r9, rcx, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     andn	r8, r10, r8
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	r8, rax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     adc	r9, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	r9, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	r8, r10
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     sub	rax, r8
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sbb	rdx, r9
```
## `unbounded_rem_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      10
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    1.10
IPC:               1.00
Block RThroughput: 1.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        mov	al, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	al, cl
 1      1     0.20                        cmp	dl, 8
 1      1     0.20                        not	al
 1      0     0.20                        movzx	ecx, al
 1      1     0.20                        mov	eax, 255
 1      1     0.50                        cmovb	eax, ecx
 1      2     0.20                        and	al, r8b

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   2.00    -      -      -     2.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	al, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	al, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     not	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movzx	ecx, al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, 255
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovb	eax, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	al, r8b
```
## `unbounded_rem_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      11
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    0.64
IPC:               0.64
Block RThroughput: 1.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 16
 1      1     0.20                        mov	eax, -1
 1      3     0.50                        shlx	edx, eax, edx
 1      1     0.20                        not	edx
 1      1     0.20                        mov	eax, 65535
 1      1     0.50                        cmovb	eax, edx
 1      2     0.20                        and	eax, ecx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -     1.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 16
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	edx, eax, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	eax, 65535
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovb	eax, edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	eax, ecx
```
## `unbounded_rem_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      11
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.55
IPC:               0.55
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 32
 1      1     0.20                        mov	r8d, -1
 1      3     0.50                        shlx	eax, r8d, edx
 1      1     0.20                        not	eax
 1      1     0.50                        cmovae	eax, r8d
 1      2     0.20                        and	eax, ecx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -     1.00   2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 32
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8d, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	eax, r8d, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovae	eax, r8d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	eax, ecx
```
## `unbounded_rem_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      11
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.55
IPC:               0.55
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 64
 1      1     0.20                        mov	r8, -1
 1      3     0.50                        shlx	rax, r8, rdx
 1      1     0.20                        not	rax
 1      1     0.50                        cmovae	rax, r8
 1      2     0.20                        and	rax, rcx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -     1.00   2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	rax, r8, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	rax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovae	rax, r8
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	rax, rcx
```
## `unbounded_rem_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      12
Total uOps:        14

Dispatch Width:    6
uOps Per Cycle:    1.17
IPC:               1.17
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        mov	r9, -1
 1      3     0.50                        shlx	r10, r9, r8
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        test	r8b, 64
 1      1     0.20                        mov	r11, -1
 1      1     0.50                        cmovne	r11, r10
 1      1     0.50                        cmove	rax, r10
 1      1     0.20                        not	rax
 1      1     0.20                        not	r11
 1      2     0.20                        test	r8b, r8b
 1      1     0.50                        cmovs	r11, r9
 1      1     0.50                        cmovs	rax, r9
 1      2     0.20                        and	rax, rcx
 1      2     0.20                        and	rdx, r11

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   2.00    -      -      -     3.00   3.00    -      -      -     3.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r9, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	r10, r9, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r8b, 64
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r11, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r11, r10
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	rax, r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	rax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	r11
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r8b, r8b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovs	r11, r9
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovs	rax, r9
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	rax, rcx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	rdx, r11
```
## `unbounded_round_to_multiple_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      29
Total Cycles:      21
Total uOps:        34

Dispatch Width:    6
uOps Per Cycle:    1.62
IPC:               1.38
Block RThroughput: 6.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      1     0.20                        cmp	dl, 7
 1      1     0.50                        ja	.LBB429_5
 1      1     0.20                        mov	r8b, 1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r8b, cl
 1      0     0.20                        mov	ecx, r8d
 1      1     0.50                        shr	cl
 1      0     0.20                        mov	edx, eax
 1      1     0.50                        shr	dl, 7
 1      2     0.20                        xor	r9d, r9d
 1      1     0.20                        sub	cl, dl
 1      0     0.20                        movzx	ecx, cl
 1      1     0.50                        cmovb	ecx, r9d
 1      1     1.00                        lea	edx, [rax + rcx]
 1      1     0.20                        add	al, cl
 2      2     1.00                        seto	al
 1      1     0.50                        jo	.LBB429_2
 1      1     0.20                        neg	r8b
 1      2     0.20                        and	dl, r8b
 1      2     0.20                        xor	al, 1
 2      7     0.50                  U     ret
 1      1     0.20                        add	al, -128
 1      2     0.20                        xor	dl, 8
 1      2     0.20                        or	dl, al
 2      2     1.00                        setne	al
 1      2     0.20                        xor	edx, edx
 2      7     0.50                  U     ret
 1      2     0.20                        xor	al, 1

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
6.00   5.00    -     1.00    -     6.00   9.00    -      -      -     6.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 7
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB429_5
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8b, 1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r8b, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shr	cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	edx, eax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	dl, 7
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	r9d, r9d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     sub	cl, dl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movzx	ecx, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovb	ecx, r9d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     lea	edx, [rax + rcx]
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	al, cl
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     seto	al
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jo	.LBB429_2
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     neg	r8b
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	dl, r8b
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	al, 1
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	al, -128
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	dl, 8
1.00    -      -      -      -      -      -      -      -      -      -      -      -     or	dl, al
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setne	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	edx, edx
1.00    -      -     1.00    -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	al, 1
```
## `unbounded_round_to_multiple_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      30
Total Cycles:      21
Total uOps:        35

Dispatch Width:    6
uOps Per Cycle:    1.67
IPC:               1.43
Block RThroughput: 7.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 15
 1      1     0.50                        ja	.LBB426_3
 1      1     0.20                        mov	eax, 1
 1      3     0.50                        shlx	edx, eax, edx
 1      1     0.20                        movzx	eax, dx
 1      1     0.50                        shr	eax
 1      1     0.20                        movzx	r8d, cx
 1      1     0.50                        shr	r8d, 15
 1      2     0.20                        xor	r9d, r9d
 1      1     0.20                        sub	ax, r8w
 1      1     0.50                        cmovb	eax, r9d
 1      1     1.00                        lea	r8d, [rcx + rax]
 1      1     0.20                        add	cx, ax
 2      2     1.00                        seto	al
 1      1     0.50                        jo	.LBB426_4
 1      1     0.20                        neg	edx
 1      2     0.20                        and	edx, r8d
 1      2     0.20                        xor	al, 1
 1      0     0.20                        movzx	eax, al
 2      7     0.50                  U     ret
 1      1     0.20                        neg	cx
 2      2     1.00                        setno	cl
 1      1     0.20                        cmp	dl, 16
 2      2     1.00                        setne	al
 1      2     0.20                        or	al, cl
 1      2     0.20                        xor	edx, edx
 1      0     0.20                        movzx	eax, al
 2      7     0.50                  U     ret
 1      2     0.20                        xor	al, 1
 1      0     0.20                        movzx	eax, al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
8.00   5.00    -     1.00    -     5.00   9.00    -      -      -     6.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 15
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB426_3
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	edx, eax, edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	eax, dx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movzx	r8d, cx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r8d, 15
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	r9d, r9d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     sub	ax, r8w
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovb	eax, r9d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     lea	r8d, [rcx + rax]
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	cx, ax
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     seto	al
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jo	.LBB426_4
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     neg	edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	edx, r8d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	al, 1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movzx	eax, al
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     neg	cx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setno	cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 16
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setne	al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     or	al, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	edx, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
 -      -      -     1.00    -      -     1.00    -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	al, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     movzx	eax, al
```
## `unbounded_round_to_multiple_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      30
Total Cycles:      20
Total uOps:        35

Dispatch Width:    6
uOps Per Cycle:    1.75
IPC:               1.50
Block RThroughput: 7.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 31
 1      1     0.50                        ja	.LBB427_5
 1      1     0.20                        mov	eax, 1
 1      3     0.50                        shlx	r8d, eax, edx
 1      0     0.20                        mov	eax, r8d
 1      1     0.50                        shr	eax
 1      0     0.20                        mov	edx, ecx
 1      1     0.50                        shr	edx, 31
 1      2     0.20                        xor	r9d, r9d
 1      1     0.20                        sub	eax, edx
 1      1     0.50                        cmovb	eax, r9d
 1      1     1.00                        lea	edx, [rcx + rax]
 1      1     0.20                        add	eax, ecx
 2      2     1.00                        seto	al
 1      1     0.50                        jo	.LBB427_2
 1      1     0.20                        neg	r8d
 1      2     0.20                        and	edx, r8d
 1      2     0.20                        xor	al, 1
 1      0     0.20                        movzx	eax, al
 2      7     0.50                  U     ret
 1      1     0.20                        neg	ecx
 2      2     1.00                        setno	cl
 1      1     0.20                        cmp	dl, 32
 2      2     1.00                        setne	al
 1      2     0.20                        or	al, cl
 1      2     0.20                        xor	edx, edx
 1      0     0.20                        movzx	eax, al
 2      7     0.50                  U     ret
 1      2     0.20                        xor	al, 1
 1      0     0.20                        movzx	eax, al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
8.00   5.00    -     1.00    -     6.00   8.00    -      -      -     6.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 31
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB427_5
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	r8d, eax, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	eax, r8d
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, ecx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	edx, 31
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	r9d, r9d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     sub	eax, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovb	eax, r9d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     lea	edx, [rcx + rax]
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	eax, ecx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     seto	al
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     jo	.LBB427_2
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     neg	r8d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	edx, r8d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	al, 1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movzx	eax, al
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     neg	ecx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setno	cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 32
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setne	al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     or	al, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	edx, edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	eax, al
1.00    -      -     1.00    -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	al, 1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movzx	eax, al
```
## `unbounded_round_to_multiple_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      30
Total Cycles:      21
Total uOps:        33

Dispatch Width:    6
uOps Per Cycle:    1.57
IPC:               1.43
Block RThroughput: 7.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 63
 1      1     0.50                        ja	.LBB428_5
 1      1     0.20                        mov	eax, 1
 1      3     0.50                        shlx	r8, rax, rdx
 0      1     0.00                        mov	rax, r8
 1      1     0.50                        shr	rax
 0      1     0.00                        mov	rdx, rcx
 1      1     0.50                        shr	rdx, 63
 1      2     0.20                        xor	r9d, r9d
 1      1     0.20                        sub	rax, rdx
 1      1     0.50                        cmovae	r9, rax
 1      1     1.00                        lea	rdx, [rcx + r9]
 1      1     0.20                        add	r9, rcx
 2      2     1.00                        seto	al
 1      1     0.50                        jo	.LBB428_2
 1      1     0.20                        neg	r8
 1      2     0.20                        and	rdx, r8
 1      2     0.20                        xor	al, 1
 1      0     0.20                        movzx	eax, al
 2      7     0.50                  U     ret
 1      1     0.20                        neg	rcx
 2      2     1.00                        setno	cl
 1      1     0.20                        cmp	dl, 64
 2      2     1.00                        setne	al
 1      2     0.20                        or	al, cl
 1      2     0.20                        xor	edx, edx
 1      0     0.20                        movzx	eax, al
 2      7     0.50                  U     ret
 1      2     0.20                        xor	al, 1
 1      0     0.20                        movzx	eax, al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
9.00   5.00    -     1.00    -     5.00   7.00    -      -      -     5.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 63
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB428_5
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	r8, rax, rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shr	rax
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, rcx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	rdx, 63
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	r9d, r9d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     sub	rax, rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovae	r9, rax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     lea	rdx, [rcx + r9]
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	r9, rcx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     seto	al
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     jo	.LBB428_2
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     neg	r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	rdx, r8
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	al, 1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     neg	rcx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setno	cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 64
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setne	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     or	al, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	edx, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     movzx	eax, al
1.00    -      -     1.00    -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	al, 1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
```
## `unbounded_round_to_multiple_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      51
Total Cycles:      38
Total uOps:        55

Dispatch Width:    6
uOps Per Cycle:    1.45
IPC:               1.34
Block RThroughput: 9.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	rsi
 2      1     0.50           *            push	rdi
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB425_3
 1      2     0.20                        xor	r10d, r10d
 1      1     0.20                        mov	r11d, 1
 1      2     0.20                        xor	esi, esi
 1      0     0.20                        mov	ecx, r9d
 3      5     1.00                        shld	rsi, r11, cl
 1      3     0.50                        shlx	r11, r11, r9
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	rsi, r11
 1      1     0.50                        cmovne	r11, r10
 0      1     0.00                        mov	rcx, rsi
 1      1     0.50                        shr	rcx
 0      1     0.00                        mov	r9, rsi
 1      3     1.00                        shld	r9, r11, 63
 0      1     0.00                        mov	rdi, r8
 1      1     0.50                        shr	rdi, 63
 1      1     0.20                        sub	r9, rdi
 1      1     0.50                        sbb	rcx, 0
 1      1     0.50                        cmovb	rcx, r10
 1      1     0.50                        cmovb	r9, r10
 1      1     0.20                        add	rdx, r9
 1      1     0.50                        adc	rcx, r8
 1      1     0.50                        jo	.LBB425_5
 1      2     0.20                        xor	r8d, r8d
 1      1     0.20                        neg	r11
 1      1     0.50                        sbb	r8, rsi
 1      2     0.20                        and	rcx, r8
 1      2     0.20                        and	rdx, r11
 0      1     0.00                        mov	r10, rdx
 0      0     0.00                        jmp	.LBB425_4
 1      1     0.20                        movabs	rcx, -9223372036854775808
 1      2     0.20                        xor	r8, rcx
 1      2     0.20                        or	rdx, r8
 2      2     1.00                        sete	dl
 1      1     0.20                        neg	r9b
 2      2     1.00                        seto	r8b
 1      2     0.20                        xor	r10d, r10d
 1      1     0.20                        mov	ecx, 0
 1      2     0.20                        test	dl, r8b
 1      1     0.50                        jne	.LBB425_5
 2      12    0.50           *            mov	qword ptr [rax + 16], r10
 2      12    0.50           *            mov	qword ptr [rax + 24], rcx
 1      1     0.20                        mov	r10d, 1
 2      12    0.50           *            mov	qword ptr [rax], r10
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rdi
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
9.00   7.00    -     1.00   3.00   7.00   11.00  3.00   3.00   3.00   7.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rsi
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     push	rdi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, r9b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB425_3
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	r10d, r10d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r11d, 1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	esi, esi
1.00    -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
 -     1.00    -      -      -     1.00   1.00    -      -      -      -      -      -     shld	rsi, r11, cl
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shlx	r11, r11, r9
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rsi, r11
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r11, r10
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rcx, rsi
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rsi
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     shld	r9, r11, 63
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdi, r8
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	rdi, 63
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     sub	r9, rdi
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sbb	rcx, 0
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovb	rcx, r10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovb	r9, r10
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	rdx, r9
1.00    -      -      -      -      -      -      -      -      -      -      -      -     adc	rcx, r8
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jo	.LBB425_5
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	r8d, r8d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     neg	r11
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sbb	r8, rsi
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	rcx, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	rdx, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -     jmp	.LBB425_4
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movabs	rcx, -9223372036854775808
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	r8, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     or	rdx, r8
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sete	dl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     neg	r9b
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     seto	r8b
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	r10d, r10d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, 0
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	dl, r8b
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jne	.LBB425_5
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 16], r10
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 24], rcx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r10d, 1
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax], r10
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 8], 0
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rdi
 -      -      -     1.00    -      -      -      -      -      -      -      -      -     pop	rsi
```
## `unbounded_round_to_multiple_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      22
Total Cycles:      17
Total uOps:        28

Dispatch Width:    6
uOps Per Cycle:    1.65
IPC:               1.29
Block RThroughput: 6.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      1     0.20                        cmp	dl, 7
 1      1     0.50                        ja	.LBB434_4
 1      1     0.20                        mov	r8b, 1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r8b, cl
 1      0     0.20                        mov	edx, r8d
 1      1     0.50                        shr	dl
 1      1     0.20                        add	dl, al
 1      1     0.20                        cmp	dl, al
 2      2     1.00                        setae	al
 1      1     0.50                        jb	.LBB434_3
 1      1     0.20                        neg	r8b
 1      2     0.20                        and	dl, r8b
 2      7     0.50                  U     ret
 1      2     0.20                        test	al, al
 2      2     1.00                        setns	cl
 1      1     0.20                        cmp	dl, 8
 2      2     1.00                        setne	al
 1      2     0.20                        or	al, cl
 1      2     0.20                        xor	edx, edx
 2      7     0.50                  U     ret

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
6.00   4.00    -     1.00    -     4.00   8.00    -      -      -     4.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 7
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB434_4
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8b, 1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r8b, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	edx, r8d
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	dl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	dl, al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, al
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setae	al
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jb	.LBB434_3
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     neg	r8b
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	dl, r8b
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	al, al
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setns	cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 8
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setne	al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     or	al, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	edx, edx
1.00    -      -     1.00    -      -      -      -      -      -      -      -      -     ret
```
## `unbounded_round_to_multiple_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      23
Total Cycles:      17
Total uOps:        28

Dispatch Width:    6
uOps Per Cycle:    1.65
IPC:               1.35
Block RThroughput: 6.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 15
 1      1     0.50                        ja	.LBB431_4
 1      1     0.20                        mov	eax, 1
 1      3     0.50                        shlx	r8d, eax, edx
 1      1     0.20                        movzx	eax, r8w
 1      1     0.50                        shr	eax
 1      1     1.00                        lea	edx, [rcx + rax]
 1      1     0.20                        cmp	dx, cx
 2      2     1.00                        setae	al
 1      1     0.50                        jb	.LBB431_3
 1      1     0.20                        neg	r8d
 1      2     0.20                        and	edx, r8d
 1      0     0.20                        movzx	eax, al
 2      7     0.50                  U     ret
 1      2     0.20                        test	cx, cx
 2      2     1.00                        setns	cl
 1      1     0.20                        cmp	dl, 16
 2      2     1.00                        setne	al
 1      2     0.20                        or	al, cl
 1      2     0.20                        xor	edx, edx
 1      0     0.20                        movzx	eax, al
 2      7     0.50                  U     ret
 1      0     0.20                        movzx	eax, al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
6.00   4.00    -     1.00    -     4.00   8.00    -      -      -     4.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 15
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB431_4
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	r8d, eax, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movzx	eax, r8w
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     lea	edx, [rcx + rax]
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	dx, cx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setae	al
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jb	.LBB431_3
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     neg	r8d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	edx, r8d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	eax, al
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	cx, cx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setns	cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	dl, 16
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setne	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     or	al, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	edx, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
1.00    -      -     1.00    -      -      -      -      -      -      -      -      -     ret
1.00    -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
```
## `unbounded_round_to_multiple_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      23
Total Cycles:      16
Total uOps:        28

Dispatch Width:    6
uOps Per Cycle:    1.75
IPC:               1.44
Block RThroughput: 6.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 31
 1      1     0.50                        ja	.LBB432_4
 1      1     0.20                        mov	eax, 1
 1      3     0.50                        shlx	r8d, eax, edx
 1      0     0.20                        mov	edx, r8d
 1      1     0.50                        shr	edx
 1      1     0.20                        add	edx, ecx
 1      1     0.20                        cmp	edx, ecx
 2      2     1.00                        setae	al
 1      1     0.50                        jb	.LBB432_3
 1      1     0.20                        neg	r8d
 1      2     0.20                        and	edx, r8d
 1      0     0.20                        movzx	eax, al
 2      7     0.50                  U     ret
 1      2     0.20                        test	ecx, ecx
 2      2     1.00                        setns	cl
 1      1     0.20                        cmp	dl, 32
 2      2     1.00                        setne	al
 1      2     0.20                        or	al, cl
 1      2     0.20                        xor	edx, edx
 1      0     0.20                        movzx	eax, al
 2      7     0.50                  U     ret
 1      0     0.20                        movzx	eax, al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
8.00   4.00    -     1.00    -     4.00   5.00    -      -      -     5.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 31
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB432_4
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	r8d, eax, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shr	edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	edx, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	edx, ecx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setae	al
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     jb	.LBB432_3
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     neg	r8d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	edx, r8d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	eax, al
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	ecx, ecx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setns	cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	dl, 32
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setne	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     or	al, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	edx, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
 -      -      -     1.00    -      -     1.00    -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	eax, al
```
## `unbounded_round_to_multiple_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      23
Total Cycles:      17
Total uOps:        27

Dispatch Width:    6
uOps Per Cycle:    1.59
IPC:               1.35
Block RThroughput: 6.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 63
 1      1     0.50                        ja	.LBB433_4
 1      1     0.20                        mov	eax, 1
 1      3     0.50                        shlx	r8, rax, rdx
 0      1     0.00                        mov	rdx, r8
 1      1     0.50                        shr	rdx
 1      1     0.20                        add	rdx, rcx
 1      1     0.20                        cmp	rdx, rcx
 2      2     1.00                        setae	al
 1      1     0.50                        jb	.LBB433_3
 1      1     0.20                        neg	r8
 1      2     0.20                        and	rdx, r8
 1      0     0.20                        movzx	eax, al
 2      7     0.50                  U     ret
 1      2     0.20                        test	rcx, rcx
 2      2     1.00                        setns	cl
 1      1     0.20                        cmp	dl, 64
 2      2     1.00                        setne	al
 1      2     0.20                        or	al, cl
 1      2     0.20                        xor	edx, edx
 1      0     0.20                        movzx	eax, al
 2      7     0.50                  U     ret
 1      0     0.20                        movzx	eax, al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   4.00    -     1.00    -     4.00   8.00    -      -      -     4.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 63
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB433_4
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	r8, rax, rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r8
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	rdx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	rdx, rcx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	rdx, rcx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setae	al
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jb	.LBB433_3
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     neg	r8
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	rdx, r8
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	rcx, rcx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setns	cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 64
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setne	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     or	al, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	edx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movzx	eax, al
1.00    -      -     1.00    -      -      -      -      -      -      -      -      -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
```
## `unbounded_round_to_multiple_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      41
Total Cycles:      35
Total uOps:        45

Dispatch Width:    6
uOps Per Cycle:    1.29
IPC:               1.17
Block RThroughput: 7.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	rsi
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB430_3
 1      2     0.20                        xor	r10d, r10d
 1      1     0.20                        mov	r11d, 1
 1      2     0.20                        xor	esi, esi
 1      0     0.20                        mov	ecx, r9d
 3      5     1.00                        shld	rsi, r11, cl
 1      3     0.50                        shlx	r11, r11, r9
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	rsi, r11
 1      1     0.50                        cmovne	r11, r10
 0      1     0.00                        mov	rcx, rsi
 1      1     0.50                        shr	rcx
 0      1     0.00                        mov	r9, rsi
 1      3     1.00                        shld	r9, r11, 63
 1      1     0.20                        add	r9, rdx
 1      1     0.50                        adc	rcx, r8
 1      1     0.50                        jb	.LBB430_5
 1      2     0.20                        xor	edx, edx
 1      1     0.20                        neg	r11
 1      1     0.50                        sbb	rdx, rsi
 1      2     0.20                        and	rcx, rdx
 1      2     0.20                        and	r9, r11
 0      1     0.00                        mov	r10, r9
 0      0     0.00                        jmp	.LBB430_4
 1      2     0.20                        test	r8, r8
 2      2     1.00                        sets	dl
 1      1     0.20                        neg	r9b
 2      2     1.00                        seto	r8b
 1      2     0.20                        xor	r10d, r10d
 1      1     0.20                        mov	ecx, 0
 1      2     0.20                        test	dl, r8b
 1      1     0.50                        jne	.LBB430_5
 2      12    0.50           *            mov	qword ptr [rax + 16], r10
 2      12    0.50           *            mov	qword ptr [rax + 24], rcx
 1      1     0.20                        mov	r10d, 1
 2      12    0.50           *            mov	qword ptr [rax], r10
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
7.00   6.00    -      -     2.00   6.00   9.00   2.00   3.00   3.00   6.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, r9b
1.00    -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB430_3
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	r10d, r10d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r11d, 1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	esi, esi
1.00    -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
 -     1.00    -      -      -     1.00   1.00    -      -      -      -      -      -     shld	rsi, r11, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shlx	r11, r11, r9
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rsi, r11
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r11, r10
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rcx, rsi
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rsi
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     shld	r9, r11, 63
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	r9, rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     adc	rcx, r8
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jb	.LBB430_5
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	edx, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     neg	r11
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sbb	rdx, rsi
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	rcx, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	r9, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, r9
 -      -      -      -      -      -      -      -      -      -      -      -      -     jmp	.LBB430_4
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r8, r8
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sets	dl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     neg	r9b
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     seto	r8b
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	r10d, r10d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, 0
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	dl, r8b
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jne	.LBB430_5
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 16], r10
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 24], rcx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r10d, 1
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax], r10
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 8], 0
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rsi
```
