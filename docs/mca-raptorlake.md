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
## `ceil_to_multiple_i8_pow2`
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
## `ceil_to_multiple_i8_unb_pow2`
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
## `ceil_to_multiple_i16_pow2`
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
 1      1     0.20                        mov	r9d, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r9d, cl
 1      0     0.20                        mov	eax, r9d
 1      1     0.20                        not	eax
 1      1     0.20                        add	eax, r8d
 1      2     0.20                        and	eax, r9d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     2.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r9d, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r9d, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	eax, r9d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     add	eax, r8d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	eax, r9d
```
## `ceil_to_multiple_i16_unb_pow2`
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
 1      2     0.20                        and	dl, 15
 1      1     0.20                        mov	r9d, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r9d, cl
 1      0     0.20                        mov	eax, r9d
 1      1     0.20                        not	eax
 1      1     0.20                        add	eax, r8d
 1      2     0.20                        and	eax, r9d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   2.00    -      -      -     2.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 15
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r9d, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r9d, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, r9d
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     not	eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	eax, r8d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	eax, r9d
```
## `ceil_to_multiple_i32_pow2`
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
 1      1     0.20                        mov	r9d, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r9d, cl
 1      0     0.20                        mov	eax, r9d
 1      1     0.20                        not	eax
 1      1     0.20                        add	eax, r8d
 1      2     0.20                        and	eax, r9d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     2.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r9d, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r9d, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	eax, r9d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     add	eax, r8d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	eax, r9d
```
## `ceil_to_multiple_i32_unb_pow2`
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
 1      1     0.20                        mov	r9d, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r9d, cl
 1      0     0.20                        mov	eax, r9d
 1      1     0.20                        not	eax
 1      1     0.20                        add	eax, r8d
 1      2     0.20                        and	eax, r9d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     2.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r9d, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r9d, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	eax, r9d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     add	eax, r8d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	eax, r9d
```
## `ceil_to_multiple_i64_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      11
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    0.64
IPC:               0.73
Block RThroughput: 1.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	r8, rcx
 1      1     0.20                        mov	r9, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r9, cl
 0      1     0.00                        mov	rax, r9
 1      1     0.20                        not	rax
 1      1     0.20                        add	rax, r8
 1      2     0.20                        and	rax, r9

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   1.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r9, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r9, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	rax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	rax, r8
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	rax, r9
```
## `ceil_to_multiple_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      11
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    0.64
IPC:               0.73
Block RThroughput: 1.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	r8, rcx
 1      1     0.20                        mov	r9, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r9, cl
 0      1     0.00                        mov	rax, r9
 1      1     0.20                        not	rax
 1      1     0.20                        add	rax, r8
 1      2     0.20                        and	rax, r9

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   1.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r9, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r9, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	rax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	rax, r8
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	rax, r9
```
## `ceil_to_multiple_i128_pow2`
```asm
Iterations:        1
Instructions:      17
Total Cycles:      13
Total uOps:        15

Dispatch Width:    6
uOps Per Cycle:    1.15
IPC:               1.31
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	r9, rcx
 1      1     0.20                        mov	r10, -1
 1      0     0.20                        mov	ecx, r8d
 2      2     1.00                        shl	r10, cl
 1      1     0.20                        mov	rax, -1
 1      2     0.20                        xor	ecx, ecx
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmove	rcx, r10
 1      1     0.50                        cmove	r10, rax
 0      1     0.00                        mov	r8, r10
 1      1     0.20                        not	r8
 0      1     0.00                        mov	rax, rcx
 1      1     0.20                        not	rax
 1      1     0.20                        add	rax, r9
 1      1     0.50                        adc	rdx, r8
 1      2     0.20                        and	rax, rcx
 1      2     0.20                        and	rdx, r10

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   3.00    -      -      -     3.00   3.00    -      -      -     3.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r10, -1
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, r8d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r10, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	rax, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	rcx, r10
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	r10, rax
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	r8
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	rax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	rax, r9
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     adc	rdx, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	rax, rcx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	rdx, r10
```
## `ceil_to_multiple_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      17
Total Cycles:      13
Total uOps:        15

Dispatch Width:    6
uOps Per Cycle:    1.15
IPC:               1.31
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	r9, rcx
 1      1     0.20                        mov	r10, -1
 1      0     0.20                        mov	ecx, r8d
 2      2     1.00                        shl	r10, cl
 1      1     0.20                        mov	rax, -1
 1      2     0.20                        xor	ecx, ecx
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmove	rcx, r10
 1      1     0.50                        cmove	r10, rax
 0      1     0.00                        mov	r8, r10
 1      1     0.20                        not	r8
 0      1     0.00                        mov	rax, rcx
 1      1     0.20                        not	rax
 1      1     0.20                        add	rax, r9
 1      1     0.50                        adc	rdx, r8
 1      2     0.20                        and	rax, rcx
 1      2     0.20                        and	rdx, r10

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   3.00    -      -      -     3.00   3.00    -      -      -     3.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r10, -1
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, r8d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r10, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	rax, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	rcx, r10
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	r10, rax
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	r8
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	rax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	rax, r9
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     adc	rdx, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	rax, rcx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	rdx, r10
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
Instructions:      16
Total Cycles:      12
Total uOps:        18

Dispatch Width:    6
uOps Per Cycle:    1.50
IPC:               1.33
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 7
 1      1     0.50                        ja	.LBB24_1
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        mov	al, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	al, cl
 1      0     0.20                        mov	ecx, eax
 1      1     0.20                        not	cl
 1      1     0.20                        add	r8b, cl
 1      1     0.50                        jno	.LBB24_3
 1      2     0.20                        xor	eax, eax
 1      0     0.20                        mov	edx, r8d
 2      7     0.50                  U     ret
 1      2     0.20                        and	r8b, al
 1      1     0.20                        mov	al, 1
 1      0     0.20                        mov	edx, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   3.00    -      -      -     3.00   4.00    -      -      -     4.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 7
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB24_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	al, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	al, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     add	r8b, cl
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jno	.LBB24_3
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, r8d
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	r8b, al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	al, 1
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	edx, r8d
```
## `checked_ceil_to_multiple_i16_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      10
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.20
IPC:               1.00
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        mov	r9d, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r9d, cl
 1      0     0.20                        mov	edx, r9d
 1      1     0.20                        not	edx
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        add	dx, r8w
 2      2     1.00                        setno	al
 1      2     0.20                        and	edx, r9d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   2.00    -      -      -     2.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r9d, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r9d, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	edx, r9d
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     not	edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	dx, r8w
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setno	al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	edx, r9d
```
## `checked_ceil_to_multiple_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      12
Total uOps:        17

Dispatch Width:    6
uOps Per Cycle:    1.42
IPC:               1.25
Block RThroughput: 2.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 15
 1      1     0.50                        ja	.LBB18_1
 1      0     0.20                        mov	r8d, edx
 1      0     0.20                        mov	eax, ecx
 1      1     0.20                        mov	edx, -1
 1      0     0.20                        mov	ecx, r8d
 2      2     1.00                        shl	edx, cl
 1      0     0.20                        mov	ecx, edx
 1      1     0.20                        not	ecx
 1      1     0.20                        add	ax, cx
 1      1     0.50                        jno	.LBB18_3
 1      2     0.20                        xor	eax, eax
 2      7     0.50                  U     ret
 1      2     0.20                        and	edx, eax
 1      1     0.20                        mov	ax, 1

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   2.00    -      -      -     3.00   4.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 15
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB18_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8d, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	eax, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	edx, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	edx, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	ecx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	ax, cx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jno	.LBB18_3
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	edx, eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ax, 1
```
## `checked_ceil_to_multiple_i32_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      10
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.20
IPC:               1.00
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        mov	r9d, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r9d, cl
 1      0     0.20                        mov	edx, r9d
 1      1     0.20                        not	edx
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        add	edx, r8d
 2      2     1.00                        setno	al
 1      2     0.20                        and	edx, r9d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   2.00    -      -      -     2.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r9d, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r9d, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	edx, r9d
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     not	edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	edx, r8d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setno	al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	edx, r9d
```
## `checked_ceil_to_multiple_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      16
Total Cycles:      12
Total uOps:        18

Dispatch Width:    6
uOps Per Cycle:    1.50
IPC:               1.33
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 31
 1      1     0.50                        ja	.LBB20_1
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        mov	r9d, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r9d, cl
 1      0     0.20                        mov	ecx, r9d
 1      1     0.20                        not	ecx
 1      1     0.20                        add	r8d, ecx
 1      1     0.50                        jo	.LBB20_4
 1      2     0.20                        and	r8d, r9d
 1      1     0.20                        mov	eax, 1
 1      0     0.20                        mov	edx, r8d
 2      7     0.50                  U     ret
 1      0     0.20                        mov	edx, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   3.00    -      -      -     3.00   3.00    -      -      -     4.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 31
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB20_1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8d, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r9d, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r9d, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, r9d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	ecx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	r8d, ecx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jo	.LBB20_4
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	r8d, r9d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, 1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	edx, r8d
```
## `checked_ceil_to_multiple_i64_pow2`
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
 0      1     0.00                        mov	r8, rcx
 1      1     0.20                        mov	r9, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r9, cl
 0      1     0.00                        mov	rdx, r9
 1      1     0.20                        not	rdx
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        add	rdx, r8
 2      2     1.00                        setno	al
 1      2     0.20                        and	rdx, r9

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     2.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r9, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r9, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r9
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	rdx, r8
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setno	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	rdx, r9
```
## `checked_ceil_to_multiple_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      16
Total Cycles:      12
Total uOps:        14

Dispatch Width:    6
uOps Per Cycle:    1.17
IPC:               1.33
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 63
 1      1     0.50                        ja	.LBB22_1
 0      1     0.00                        mov	r8, rcx
 1      1     0.20                        mov	rax, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	rax, cl
 0      1     0.00                        mov	rcx, rax
 1      1     0.20                        not	rcx
 1      1     0.20                        add	r8, rcx
 1      1     0.50                        jno	.LBB22_3
 1      2     0.20                        xor	eax, eax
 0      1     0.00                        mov	rdx, r8
 2      7     0.50                  U     ret
 1      2     0.20                        and	r8, rax
 1      1     0.20                        mov	eax, 1
 0      1     0.00                        mov	rdx, r8

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   2.00    -      -      -     2.00   4.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 63
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB22_1
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	rax, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	rax, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rcx, rax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	rcx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	r8, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     jno	.LBB22_3
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r8
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
1.00    -      -      -      -      -      -      -      -      -      -      -      -     and	r8, rax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	eax, 1
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r8
```
## `checked_ceil_to_multiple_i128_pow2`
```asm
Iterations:        1
Instructions:      26
Total Cycles:      27
Total uOps:        28

Dispatch Width:    6
uOps Per Cycle:    1.04
IPC:               0.96
Block RThroughput: 4.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	rsi
 0      1     0.00                        mov	rax, rcx
 1      1     0.20                        mov	r10, -1
 1      0     0.20                        mov	ecx, r9d
 2      2     1.00                        shl	r10, cl
 1      1     0.20                        mov	r11, -1
 1      2     0.20                        xor	ecx, ecx
 1      2     0.20                        test	r9b, 64
 0      1     0.00                        mov	r9, r10
 1      1     0.50                        cmovne	r9, rcx
 1      1     0.50                        cmove	r10, r11
 0      1     0.00                        mov	r11, r10
 1      1     0.20                        not	r11
 0      1     0.00                        mov	rsi, r9
 1      1     0.20                        not	rsi
 1      1     0.20                        add	rsi, rdx
 1      1     0.50                        adc	r11, r8
 1      1     0.50                        jo	.LBB15_2
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
4.00   3.00    -      -     2.00   3.00   4.00   2.00   3.00   3.00   3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r10, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, r9d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r10, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r11, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, 64
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, r10
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r9, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	r10, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, r10
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rsi, r9
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	rsi
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     add	rsi, rdx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     adc	r11, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     jo	.LBB15_2
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	r11, r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	rsi, r9
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 16], rsi
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 24], r11
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, 1
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 8], 0
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rsi
```
## `checked_ceil_to_multiple_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      33
Total Cycles:      28
Total uOps:        38

Dispatch Width:    6
uOps Per Cycle:    1.36
IPC:               1.18
Block RThroughput: 6.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	rsi
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB16_4
 1      1     0.20                        mov	r10, -1
 1      0     0.20                        mov	ecx, r9d
 2      2     1.00                        shl	r10, cl
 1      1     0.20                        mov	r11, -1
 1      2     0.20                        xor	ecx, ecx
 1      2     0.20                        test	r9b, 64
 0      1     0.00                        mov	r9, r10
 1      1     0.50                        cmovne	r9, rcx
 1      1     0.50                        cmove	r10, r11
 0      1     0.00                        mov	r11, r10
 1      1     0.20                        not	r11
 0      1     0.00                        mov	rsi, r9
 1      1     0.20                        not	rsi
 1      1     0.20                        add	rdx, rsi
 1      1     0.50                        adc	r8, r11
 1      1     0.50                        jo	.LBB16_3
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
5.00   3.00   1.00   1.00   3.00   4.00   5.00   3.00   4.00   4.00   4.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, r9b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB16_4
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r10, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, r9d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r10, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r11, -1
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	ecx, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, r10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r9, rcx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	r10, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, r10
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rsi, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	rsi
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	rdx, rsi
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     adc	r8, r11
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jo	.LBB16_3
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	r8, r10
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	rdx, r9
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 16], rdx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 24], r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, 1
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 8], 0
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rsi
1.00    -      -     1.00    -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	ecx, ecx
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
 1      1     0.50                        ja	.LBB34_1
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
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB34_1
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
Total Cycles:      11
Total uOps:        13

Dispatch Width:    6
uOps Per Cycle:    1.18
IPC:               1.00
Block RThroughput: 2.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        mov	r9d, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r9d, cl
 1      0     0.20                        mov	edx, r9d
 1      1     0.20                        not	edx
 1      1     0.20                        add	edx, r8d
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dx, r8w
 2      2     1.00                        setae	al
 1      2     0.20                        and	edx, r9d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   2.00    -      -      -     2.00   4.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r9d, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r9d, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	edx, r9d
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     not	edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	edx, r8d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	dx, r8w
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setae	al
1.00    -      -      -      -      -      -      -      -      -      -      -      -     and	edx, r9d
```
## `checked_ceil_to_multiple_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      13
Total uOps:        18

Dispatch Width:    6
uOps Per Cycle:    1.38
IPC:               1.15
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 15
 1      1     0.50                        ja	.LBB28_1
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        mov	r9d, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r9d, cl
 1      0     0.20                        mov	edx, r9d
 1      1     0.20                        not	edx
 1      1     0.20                        add	edx, r8d
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dx, r8w
 2      2     1.00                        setae	al
 1      2     0.20                        and	edx, r9d
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   3.00    -      -      -     3.00   4.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 15
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB28_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r9d, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r9d, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, r9d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	edx, r8d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dx, r8w
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setae	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	edx, r9d
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
```
## `checked_ceil_to_multiple_u32_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      11
Total uOps:        13

Dispatch Width:    6
uOps Per Cycle:    1.18
IPC:               1.00
Block RThroughput: 2.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        mov	r9d, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r9d, cl
 1      0     0.20                        mov	edx, r9d
 1      1     0.20                        not	edx
 1      1     0.20                        add	edx, r8d
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	edx, r8d
 2      2     1.00                        setae	al
 1      2     0.20                        and	edx, r9d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   2.00    -      -      -     2.00   4.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r9d, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r9d, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	edx, r9d
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     not	edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	edx, r8d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	edx, r8d
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setae	al
1.00    -      -      -      -      -      -      -      -      -      -      -      -     and	edx, r9d
```
## `checked_ceil_to_multiple_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      13
Total uOps:        18

Dispatch Width:    6
uOps Per Cycle:    1.38
IPC:               1.15
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 31
 1      1     0.50                        ja	.LBB30_1
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        mov	r9d, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r9d, cl
 1      0     0.20                        mov	edx, r9d
 1      1     0.20                        not	edx
 1      1     0.20                        add	edx, r8d
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	edx, r8d
 2      2     1.00                        setae	al
 1      2     0.20                        and	edx, r9d
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   3.00    -      -      -     3.00   4.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 31
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB30_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r9d, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r9d, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, r9d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	edx, r8d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	edx, r8d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setae	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	edx, r9d
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
```
## `checked_ceil_to_multiple_u64_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      12
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    0.92
IPC:               0.92
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	r8, rcx
 1      1     0.20                        mov	r9, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r9, cl
 0      1     0.00                        mov	rdx, r9
 1      1     0.20                        not	rdx
 1      1     0.20                        add	rdx, r8
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	rdx, r8
 2      2     1.00                        setae	al
 1      2     0.20                        and	rdx, r9

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   1.00    -      -      -     2.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r9, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r9, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r9
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	rdx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	rdx, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	rdx, r8
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setae	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	rdx, r9
```
## `checked_ceil_to_multiple_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      14
Total uOps:        16

Dispatch Width:    6
uOps Per Cycle:    1.14
IPC:               1.07
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 63
 1      1     0.50                        ja	.LBB32_1
 0      1     0.00                        mov	r8, rcx
 1      1     0.20                        mov	r9, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r9, cl
 0      1     0.00                        mov	rdx, r9
 1      1     0.20                        not	rdx
 1      1     0.20                        add	rdx, r8
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	rdx, r8
 2      2     1.00                        setae	al
 1      2     0.20                        and	rdx, r9
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   2.00    -      -      -     3.00   5.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 63
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB32_1
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r9, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r9, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r9
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	rdx, r8
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	rdx, r8
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setae	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	rdx, r9
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
```
## `checked_ceil_to_multiple_u128_pow2`
```asm
Iterations:        1
Instructions:      26
Total Cycles:      27
Total uOps:        28

Dispatch Width:    6
uOps Per Cycle:    1.04
IPC:               0.96
Block RThroughput: 4.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	rsi
 0      1     0.00                        mov	rax, rcx
 1      1     0.20                        mov	r10, -1
 1      0     0.20                        mov	ecx, r9d
 2      2     1.00                        shl	r10, cl
 1      1     0.20                        mov	r11, -1
 1      2     0.20                        xor	ecx, ecx
 1      2     0.20                        test	r9b, 64
 0      1     0.00                        mov	r9, r10
 1      1     0.50                        cmovne	r9, rcx
 1      1     0.50                        cmove	r10, r11
 0      1     0.00                        mov	r11, r10
 1      1     0.20                        not	r11
 0      1     0.00                        mov	rsi, r9
 1      1     0.20                        not	rsi
 1      1     0.20                        add	rsi, rdx
 1      1     0.50                        adc	r11, r8
 1      1     0.50                        jb	.LBB25_2
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
4.00   3.00    -      -     2.00   3.00   4.00   2.00   3.00   3.00   3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r10, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, r9d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r10, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r11, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, 64
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, r10
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r9, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	r10, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, r10
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rsi, r9
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	rsi
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     add	rsi, rdx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     adc	r11, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     jb	.LBB25_2
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	r11, r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	rsi, r9
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 16], rsi
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 24], r11
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, 1
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 8], 0
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rsi
```
## `checked_ceil_to_multiple_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      33
Total Cycles:      28
Total uOps:        38

Dispatch Width:    6
uOps Per Cycle:    1.36
IPC:               1.18
Block RThroughput: 6.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	rsi
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB26_4
 1      1     0.20                        mov	r10, -1
 1      0     0.20                        mov	ecx, r9d
 2      2     1.00                        shl	r10, cl
 1      1     0.20                        mov	r11, -1
 1      2     0.20                        xor	ecx, ecx
 1      2     0.20                        test	r9b, 64
 0      1     0.00                        mov	r9, r10
 1      1     0.50                        cmovne	r9, rcx
 1      1     0.50                        cmove	r10, r11
 0      1     0.00                        mov	r11, r10
 1      1     0.20                        not	r11
 0      1     0.00                        mov	rsi, r9
 1      1     0.20                        not	rsi
 1      1     0.20                        add	rdx, rsi
 1      1     0.50                        adc	r8, r11
 1      1     0.50                        jb	.LBB26_3
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
5.00   3.00   1.00   1.00   3.00   4.00   5.00   3.00   4.00   4.00   4.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, r9b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB26_4
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r10, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, r9d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r10, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r11, -1
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	ecx, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, r10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r9, rcx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	r10, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, r10
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rsi, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	rsi
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	rdx, rsi
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     adc	r8, r11
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jb	.LBB26_3
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	r8, r10
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	rdx, r9
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 16], rdx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 24], r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, 1
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 8], 0
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rsi
1.00    -      -     1.00    -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	ecx, ecx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 8], 0
 -      -     1.00    -      -      -      -      -      -      -      -      -      -     pop	rsi
```
## `checked_div_ceil_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      13
Total Cycles:      10
Total uOps:        16

Dispatch Width:    6
uOps Per Cycle:    1.60
IPC:               1.30
Block RThroughput: 3.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, edx
 1      1     0.20                        cmp	al, 7
 1      1     0.50                        ja	.LBB39_2
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        mov	r9, -1
 1      0     0.20                        mov	ecx, eax
 2      2     1.00                        shl	r9, cl
 1      1     0.20                        not	r9
 1      1     0.33                        movsx	rdx, r8b
 1      1     0.20                        add	rdx, r9
 2      2     1.00                        sar	rdx, cl
 1      1     0.20                        cmp	al, 8
 2      2     1.00                        setb	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   2.00    -      -      -     2.00   4.00    -      -      -     3.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	al, 7
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB39_2
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8d, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r9, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, eax
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r9, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	r9
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movsx	rdx, r8b
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	rdx, r9
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	rdx, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	al, 8
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setb	al
```
## `checked_div_ceil_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      12
Total uOps:        17

Dispatch Width:    6
uOps Per Cycle:    1.42
IPC:               1.17
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, edx
 1      1     0.20                        cmp	dl, 15
 1      1     0.50                        ja	.LBB36_1
 1      0     0.20                        mov	edx, ecx
 1      1     0.20                        mov	r8, -1
 1      0     0.20                        mov	ecx, eax
 2      2     1.00                        shl	r8, cl
 1      1     0.20                        not	r8
 1      1     0.33                        movsx	rdx, dx
 1      1     0.20                        add	rdx, r8
 2      2     1.00                        sar	rdx, cl
 1      1     0.20                        mov	ax, 1
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   2.00    -      -      -     3.00   3.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 15
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB36_1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r8, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, eax
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r8, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	r8
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movsx	rdx, dx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	rdx, r8
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	rdx, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ax, 1
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
```
## `checked_div_ceil_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      12
Total uOps:        17

Dispatch Width:    6
uOps Per Cycle:    1.42
IPC:               1.17
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, edx
 1      1     0.20                        cmp	dl, 31
 1      1     0.50                        ja	.LBB37_1
 1      0     0.20                        mov	edx, ecx
 1      1     0.20                        mov	r8, -1
 1      0     0.20                        mov	ecx, eax
 2      2     1.00                        shl	r8, cl
 1      1     0.20                        not	r8
 1      1     0.33                        movsxd	rdx, edx
 1      1     0.20                        add	rdx, r8
 2      2     1.00                        sar	rdx, cl
 1      1     0.20                        mov	eax, 1
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   2.00    -      -      -     3.00   3.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 31
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB37_1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r8, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, eax
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r8, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	r8
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movsxd	rdx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	rdx, r8
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	rdx, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	eax, 1
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
```
## `checked_div_ceil_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      13
Total uOps:        16

Dispatch Width:    6
uOps Per Cycle:    1.23
IPC:               1.15
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 63
 1      1     0.50                        ja	.LBB38_1
 0      1     0.00                        mov	rax, rcx
 0      1     0.00                        mov	r8, rcx
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        sar	rax, cl
 0      1     0.00                        mov	r9, rax
 2      2     1.00                        shl	r9, cl
 1      2     0.20                        xor	edx, edx
 1      1     0.20                        cmp	r8, r9
 2      2     1.00                        setne	dl
 1      1     0.20                        add	rdx, rax
 1      1     0.20                        mov	eax, 1
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   2.00    -      -      -     2.00   5.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 63
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB38_1
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sar	rax, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rax
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r9, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	edx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	r8, r9
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setne	dl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	rdx, rax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	eax, 1
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
```
## `checked_div_ceil_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      43
Total Cycles:      38
Total uOps:        50

Dispatch Width:    6
uOps Per Cycle:    1.32
IPC:               1.13
Block RThroughput: 8.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	rsi
 2      1     0.50           *            push	rdi
 2      1     0.50           *            push	rbx
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB35_1
 0      1     0.00                        mov	r10, rdx
 1      0     0.20                        mov	ecx, r9d
 3      5     1.00                        shrd	r10, r8, cl
 0      1     0.00                        mov	r11, r8
 2      2     1.00                        sar	r11, cl
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	r10, r11
 0      1     0.00                        mov	rsi, r10
 2      2     1.00                        shl	rsi, cl
 1      2     0.20                        xor	edi, edi
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmove	rdi, rsi
 0      1     0.00                        mov	rbx, r8
 1      1     0.50                        sar	rbx, 63
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmove	rbx, r11
 0      1     0.00                        mov	r11, rbx
 3      5     1.00                        shld	r11, r10, cl
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	r11, rsi
 1      2     0.20                        xor	r11, r8
 1      2     0.20                        xor	rdi, rdx
 1      2     0.20                        xor	ecx, ecx
 1      2     0.20                        or	rdi, r11
 2      2     1.00                        setne	cl
 1      1     0.20                        add	rcx, r10
 1      1     0.50                        adc	rbx, 0
 2      12    0.50           *            mov	qword ptr [rax + 16], rcx
 2      12    0.50           *            mov	qword ptr [rax + 24], rbx
 1      1     0.20                        mov	ecx, 1
 0      0     0.00                        jmp	.LBB35_3
 1      2     0.20                        xor	ecx, ecx
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rbx
 1      5     0.33    *                   pop	rdi
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
8.00   5.00   1.00   1.00   3.00   6.00   8.00   3.00   4.00   4.00   6.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rsi
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     push	rdi
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rbx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, r9b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB35_1
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, r9d
1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -     shrd	r10, r8, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, r8
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	r11, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r10, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rsi, r10
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	rsi, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	edi, edi
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	rdi, rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rbx, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sar	rbx, 63
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	rbx, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, rbx
1.00   1.00    -      -      -      -      -      -      -      -     1.00    -      -     shld	r11, r10, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r11, rsi
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	r11, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	rdi, rdx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     or	rdi, r11
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setne	cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	rcx, r10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     adc	rbx, 0
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 16], rcx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 24], rbx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, 1
 -      -      -      -      -      -      -      -      -      -      -      -      -     jmp	.LBB35_3
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 8], 0
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rbx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -     pop	rdi
 -      -     1.00    -      -      -      -      -      -      -      -      -      -     pop	rsi
```
## `checked_div_ceil_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      13
Total Cycles:      10
Total uOps:        16

Dispatch Width:    6
uOps Per Cycle:    1.60
IPC:               1.30
Block RThroughput: 3.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, edx
 1      1     0.20                        cmp	al, 7
 1      1     0.50                        ja	.LBB44_2
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        mov	r9, -1
 1      0     0.20                        mov	ecx, eax
 2      2     1.00                        shl	r9, cl
 1      1     0.20                        not	r9
 1      0     0.20                        movzx	edx, r8b
 1      1     0.20                        add	rdx, r9
 2      2     1.00                        shr	rdx, cl
 1      1     0.20                        cmp	al, 8
 2      2     1.00                        setb	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   2.00    -      -      -     2.00   4.00    -      -      -     3.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	al, 7
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB44_2
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8d, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r9, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, eax
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r9, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	r9
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	edx, r8b
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	rdx, r9
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	rdx, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	al, 8
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setb	al
```
## `checked_div_ceil_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      12
Total uOps:        17

Dispatch Width:    6
uOps Per Cycle:    1.42
IPC:               1.17
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, edx
 1      1     0.20                        cmp	dl, 15
 1      1     0.50                        ja	.LBB41_1
 1      0     0.20                        mov	edx, ecx
 1      1     0.20                        mov	r8, -1
 1      0     0.20                        mov	ecx, eax
 2      2     1.00                        shl	r8, cl
 1      1     0.20                        not	r8
 1      1     0.20                        movzx	edx, dx
 1      1     0.20                        add	rdx, r8
 2      2     1.00                        shr	rdx, cl
 1      1     0.20                        mov	ax, 1
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   2.00    -      -      -     3.00   3.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 15
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB41_1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r8, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, eax
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r8, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	r8
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	edx, dx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	rdx, r8
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	rdx, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ax, 1
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
```
## `checked_div_ceil_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      12
Total uOps:        17

Dispatch Width:    6
uOps Per Cycle:    1.42
IPC:               1.17
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, edx
 1      1     0.20                        cmp	dl, 31
 1      1     0.50                        ja	.LBB42_1
 1      0     0.20                        mov	edx, ecx
 1      1     0.20                        mov	r8, -1
 1      0     0.20                        mov	ecx, eax
 2      2     1.00                        shl	r8, cl
 1      1     0.20                        not	r8
 1      0     0.20                        mov	edx, edx
 1      1     0.20                        add	rdx, r8
 2      2     1.00                        shr	rdx, cl
 1      1     0.20                        mov	eax, 1
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   2.00    -      -      -     3.00   3.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 31
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB42_1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r8, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, eax
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r8, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	r8
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	edx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	rdx, r8
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	rdx, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	eax, 1
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
```
## `checked_div_ceil_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      13
Total uOps:        16

Dispatch Width:    6
uOps Per Cycle:    1.23
IPC:               1.15
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 63
 1      1     0.50                        ja	.LBB43_1
 0      1     0.00                        mov	rax, rcx
 0      1     0.00                        mov	r8, rcx
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shr	rax, cl
 0      1     0.00                        mov	r9, rax
 2      2     1.00                        shl	r9, cl
 1      2     0.20                        xor	edx, edx
 1      1     0.20                        cmp	r8, r9
 2      2     1.00                        setne	dl
 1      1     0.20                        add	rdx, rax
 1      1     0.20                        mov	eax, 1
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   2.00    -      -      -     2.00   5.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 63
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB43_1
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	rax, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rax
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r9, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	edx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	r8, r9
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setne	dl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	rdx, rax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	eax, 1
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
```
## `checked_div_ceil_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      40
Total Cycles:      38
Total uOps:        48

Dispatch Width:    6
uOps Per Cycle:    1.26
IPC:               1.05
Block RThroughput: 8.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	rsi
 2      1     0.50           *            push	rdi
 2      1     0.50           *            push	rbx
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB40_1
 0      1     0.00                        mov	r10, rdx
 1      0     0.20                        mov	ecx, r9d
 3      5     1.00                        shrd	r10, r8, cl
 0      1     0.00                        mov	r11, r8
 2      2     1.00                        shr	r11, cl
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	r10, r11
 0      1     0.00                        mov	rsi, r10
 2      2     1.00                        shl	rsi, cl
 1      2     0.20                        xor	edi, edi
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	r11, rdi
 1      1     0.50                        cmove	rdi, rsi
 0      1     0.00                        mov	rbx, r11
 3      5     1.00                        shld	rbx, r10, cl
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	rbx, rsi
 1      2     0.20                        xor	rbx, r8
 1      2     0.20                        xor	rdi, rdx
 1      2     0.20                        xor	ecx, ecx
 1      2     0.20                        or	rdi, rbx
 2      2     1.00                        setne	cl
 1      1     0.20                        add	rcx, r10
 1      1     0.50                        adc	r11, 0
 2      12    0.50           *            mov	qword ptr [rax + 16], rcx
 2      12    0.50           *            mov	qword ptr [rax + 24], r11
 1      1     0.20                        mov	ecx, 1
 0      0     0.00                        jmp	.LBB40_3
 1      2     0.20                        xor	ecx, ecx
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rbx
 1      5     0.33    *                   pop	rdi
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
8.00   5.00   1.00   1.00   3.00   5.00   7.00   3.00   4.00   4.00   6.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rsi
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     push	rdi
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rbx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, r9b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB40_1
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, r9d
1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -     shrd	r10, r8, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, r8
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r11, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r10, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rsi, r10
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	rsi, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	edi, edi
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r11, rdi
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	rdi, rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rbx, r11
 -     1.00    -      -      -     1.00   1.00    -      -      -      -      -      -     shld	rbx, r10, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rbx, rsi
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	rbx, r8
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	rdi, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	ecx, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     or	rdi, rbx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setne	cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	rcx, r10
1.00    -      -      -      -      -      -      -      -      -      -      -      -     adc	r11, 0
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 16], rcx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 24], r11
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, 1
 -      -      -      -      -      -      -      -      -      -      -      -      -     jmp	.LBB40_3
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	ecx, ecx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 8], 0
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rbx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -     pop	rdi
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
Instructions:      7
Total Cycles:      6
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    1.50
IPC:               1.17
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, edx
 1      1     0.33                        movsx	edx, cx
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	r8b, 16
 2      2     1.00                        setb	al
 1      0     0.20                        mov	ecx, r8d
 2      2     1.00                        sar	edx, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movsx	edx, cx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	eax, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	r8b, 16
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, r8d
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sar	edx, cl
```
## `checked_div_floor_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      6
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    1.50
IPC:               1.17
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, ecx
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 32
 2      2     1.00                        setb	al
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        sar	r8d, cl
 1      0     0.20                        mov	edx, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	dl, 32
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sar	r8d, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	edx, r8d
```
## `checked_div_floor_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      7
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	r8, rcx
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 64
 2      2     1.00                        setb	al
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        sar	r8, cl
 0      1     0.00                        mov	rdx, r8

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00    -      -      -      -     1.00   3.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 64
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sar	r8, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r8
```
## `checked_div_floor_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      20
Total Cycles:      23
Total uOps:        28

Dispatch Width:    6
uOps Per Cycle:    1.22
IPC:               0.87
Block RThroughput: 4.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB45_1
 1      0     0.20                        mov	ecx, r9d
 3      5     1.00                        shrd	rdx, r8, cl
 0      1     0.00                        mov	r10, r8
 2      2     1.00                        sar	r10, cl
 1      1     0.50                        sar	r8, 63
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	rdx, r10
 1      1     0.50                        cmove	r8, r10
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
4.00   2.00    -      -     3.00   2.00   5.00   3.00   3.00   3.00   2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, r9b
1.00    -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB45_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, r9d
1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -     shrd	rdx, r8, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, r8
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sar	r10, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sar	r8, 63
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rdx, r10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	r8, r10
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
## `checked_div_floor_u8_unb_pow2`
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
## `checked_div_floor_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      6
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    1.50
IPC:               1.17
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, edx
 1      1     0.20                        movzx	edx, cx
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	r8b, 16
 2      2     1.00                        setb	al
 1      0     0.20                        mov	ecx, r8d
 2      2     1.00                        shr	edx, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     movzx	edx, cx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	r8b, 16
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, r8d
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	edx, cl
```
## `checked_div_floor_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      6
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    1.50
IPC:               1.17
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, ecx
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 32
 2      2     1.00                        setb	al
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shr	r8d, cl
 1      0     0.20                        mov	edx, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	dl, 32
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	r8d, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	edx, r8d
```
## `checked_div_floor_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      7
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	r8, rcx
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 64
 2      2     1.00                        setb	al
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shr	r8, cl
 0      1     0.00                        mov	rdx, r8

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00    -      -      -      -     1.00   3.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 64
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	r8, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r8
```
## `checked_div_floor_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      19
Total Cycles:      23
Total uOps:        28

Dispatch Width:    6
uOps Per Cycle:    1.22
IPC:               0.83
Block RThroughput: 4.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB50_1
 1      0     0.20                        mov	ecx, r9d
 3      5     1.00                        shrd	rdx, r8, cl
 2      2     1.00                        shr	r8, cl
 1      2     0.20                        xor	ecx, ecx
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	rdx, r8
 1      1     0.50                        cmove	rcx, r8
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
3.00   2.00    -      -     3.00   2.00   5.00   3.00   3.00   3.00   3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, r9b
1.00    -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB50_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, r9d
1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -     shrd	rdx, r8, cl
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	r8, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	ecx, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rdx, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	rcx, r8
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 24], rcx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 16], rdx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, 1
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 8], 0
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	ecx, ecx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 8], 0
```
## `checked_div_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      20
Total Cycles:      13
Total uOps:        26

Dispatch Width:    6
uOps Per Cycle:    2.00
IPC:               1.54
Block RThroughput: 6.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        cmp	dl, 7
 1      1     0.50                        ja	.LBB59_3
 1      2     0.20                        test	r8b, r8b
 1      1     0.50                        js	.LBB59_2
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shr	r8b, cl
 1      1     0.20                        cmp	dl, 8
 2      2     1.00                        setb	al
 1      0     0.20                        mov	edx, r8d
 2      7     0.50                  U     ret
 1      1     0.20                        mov	al, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	al, cl
 1      1     0.20                        not	al
 1      1     0.20                        add	r8b, al
 2      2     1.00                        sar	r8b, cl
 1      1     0.20                        cmp	dl, 8
 2      2     1.00                        setb	al
 1      0     0.20                        mov	edx, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
7.00   3.00    -      -      -     4.00   7.00    -      -      -     4.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 7
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB59_3
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r8b, r8b
1.00    -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB59_2
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	r8b, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 8
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setb	al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	al, -1
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	al, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	r8b, al
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	r8b, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	dl, 8
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setb	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, r8d
```
## `checked_div_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      19
Total Cycles:      14
Total uOps:        24

Dispatch Width:    6
uOps Per Cycle:    1.71
IPC:               1.36
Block RThroughput: 5.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 15
 1      1     0.50                        ja	.LBB56_1
 1      0     0.20                        mov	eax, ecx
 1      0     0.20                        movzx	ecx, dl
 1      2     0.20                        test	ax, ax
 1      1     0.50                        js	.LBB56_4
 1      1     0.20                        movzx	edx, ax
 2      2     1.00                        shr	edx, cl
 1      1     0.20                        mov	ax, 1
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax
 2      7     0.50                  U     ret
 1      1     0.20                        mov	edx, -1
 2      2     1.00                        shl	edx, cl
 1      1     0.20                        not	edx
 1      1     0.20                        add	eax, edx
 1      1     0.33                        movsx	edx, ax
 2      2     1.00                        sar	edx, cl
 1      1     0.20                        mov	ax, 1

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   3.00    -     1.00    -     3.00   7.00    -      -      -     4.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 15
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB56_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movzx	ecx, dl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	ax, ax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB56_4
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	edx, ax
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	edx, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ax, 1
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -      -      -     1.00    -      -     1.00    -      -      -      -      -      -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	edx, -1
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	edx, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	eax, edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movsx	edx, ax
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sar	edx, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ax, 1
```
## `checked_div_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      21
Total Cycles:      13
Total uOps:        26

Dispatch Width:    6
uOps Per Cycle:    2.00
IPC:               1.62
Block RThroughput: 5.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 31
 1      1     0.50                        ja	.LBB57_1
 1      0     0.20                        mov	r8d, ecx
 1      2     0.20                        test	ecx, ecx
 1      1     0.50                        js	.LBB57_4
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shr	r8d, cl
 1      1     0.20                        mov	eax, 1
 1      0     0.20                        mov	edx, r8d
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax
 1      0     0.20                        mov	edx, r8d
 2      7     0.50                  U     ret
 1      1     0.20                        mov	eax, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	eax, cl
 1      1     0.20                        not	eax
 1      1     0.20                        add	r8d, eax
 2      2     1.00                        sar	r8d, cl
 1      1     0.20                        mov	eax, 1
 1      0     0.20                        mov	edx, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   4.00    -     1.00    -     4.00   6.00    -      -      -     5.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 31
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB57_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	ecx, ecx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB57_4
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	r8d, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, 1
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	edx, r8d
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, r8d
1.00    -      -     1.00    -      -      -      -      -      -      -      -      -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	eax, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	eax, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	r8d, eax
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	r8d, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, 1
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	edx, r8d
```
## `checked_div_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      20
Total Cycles:      14
Total uOps:        21

Dispatch Width:    6
uOps Per Cycle:    1.50
IPC:               1.43
Block RThroughput: 5.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 63
 1      1     0.50                        ja	.LBB58_1
 0      1     0.00                        mov	r8, rcx
 1      0     0.20                        movzx	ecx, dl
 1      2     0.20                        test	r8, r8
 1      1     0.50                        js	.LBB58_4
 2      2     1.00                        shr	r8, cl
 1      1     0.20                        mov	eax, 1
 0      1     0.00                        mov	rdx, r8
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax
 0      1     0.00                        mov	rdx, r8
 2      7     0.50                  U     ret
 1      1     0.20                        mov	rax, -1
 2      2     1.00                        shl	rax, cl
 1      1     0.20                        not	rax
 1      1     0.20                        add	r8, rax
 2      2     1.00                        sar	r8, cl
 1      1     0.20                        mov	eax, 1
 0      1     0.00                        mov	rdx, r8

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   2.00    -     1.00    -     3.00   6.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 63
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB58_1
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     movzx	ecx, dl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r8, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB58_4
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	r8, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	eax, 1
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r8
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r8
1.00    -      -     1.00    -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	rax, -1
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	rax, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	rax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	r8, rax
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	r8, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	eax, 1
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r8
```
## `checked_div_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      43
Total Cycles:      32
Total uOps:        54

Dispatch Width:    6
uOps Per Cycle:    1.69
IPC:               1.34
Block RThroughput: 9.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB55_1
 1      2     0.20                        test	r8, r8
 1      1     0.50                        js	.LBB55_4
 1      0     0.20                        mov	ecx, r9d
 3      5     1.00                        shrd	rdx, r8, cl
 2      2     1.00                        shr	r8, cl
 1      2     0.20                        xor	ecx, ecx
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	rdx, r8
 1      1     0.50                        cmovne	r8, rcx
 0      0     0.00                        jmp	.LBB55_6
 1      2     0.20                        xor	ecx, ecx
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 2      7     0.50                  U     ret
 1      1     0.20                        mov	r10, -1
 1      1     0.20                        mov	r11, -1
 1      0     0.20                        mov	ecx, r9d
 2      2     1.00                        shl	r11, cl
 1      2     0.20                        xor	ecx, ecx
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmove	rcx, r11
 1      1     0.50                        cmove	r11, r10
 1      1     0.20                        not	r11
 1      1     0.20                        not	rcx
 1      1     0.20                        add	rdx, rcx
 1      1     0.50                        adc	r8, r11
 1      0     0.20                        mov	ecx, r9d
 3      5     1.00                        shrd	rdx, r8, cl
 0      1     0.00                        mov	r10, r8
 2      2     1.00                        sar	r10, cl
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	rdx, r10
 1      1     0.50                        sar	r8, 63
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmove	r8, r10
 2      12    0.50           *            mov	qword ptr [rax + 16], rdx
 2      12    0.50           *            mov	qword ptr [rax + 24], r8
 1      1     0.20                        mov	ecx, 1
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
11.00  7.00    -      -     3.00   6.00   10.00  3.00   3.00   3.00   7.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, r9b
1.00    -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB55_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     test	r8, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB55_4
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, r9d
1.00   1.00    -      -      -      -      -      -      -      -     1.00    -      -     shrd	rdx, r8, cl
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	r8, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	ecx, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rdx, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r8, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -     jmp	.LBB55_6
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	ecx, ecx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 8], 0
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r10, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r11, -1
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, r9d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r11, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	ecx, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	rcx, r11
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	r11, r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	r11
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	rcx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	rdx, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     adc	r8, r11
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, r9d
1.00   1.00    -      -      -      -      -      -      -      -     1.00    -      -     shrd	rdx, r8, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, r8
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	r10, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rdx, r10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sar	r8, 63
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	r8, r10
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 16], rdx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 24], r8
1.00    -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, 1
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 8], 0
```
## `checked_floor_to_multiple_i8_unb_pow2`
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
## `checked_floor_to_multiple_i16_unb_pow2`
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
 1      0     0.20                        mov	r8d, ecx
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 16
 2      2     1.00                        setb	al
 1      1     0.20                        mov	edx, -1
 1      0     0.20                        mov	ecx, r9d
 2      2     1.00                        shl	edx, cl
 1      2     0.20                        and	edx, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     2.00   4.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r9d, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 16
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	edx, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, r9d
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	edx, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	edx, r8d
```
## `checked_floor_to_multiple_i32_unb_pow2`
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
 1      0     0.20                        mov	r8d, ecx
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 32
 2      2     1.00                        setb	al
 1      1     0.20                        mov	edx, -1
 1      0     0.20                        mov	ecx, r9d
 2      2     1.00                        shl	edx, cl
 1      2     0.20                        and	edx, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     2.00   4.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r9d, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 32
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	edx, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, r9d
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	edx, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	edx, r8d
```
## `checked_floor_to_multiple_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      8
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.25
IPC:               1.13
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r9d, edx
 0      1     0.00                        mov	r8, rcx
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 64
 2      2     1.00                        setb	al
 1      1     0.20                        mov	rdx, -1
 1      0     0.20                        mov	ecx, r9d
 2      2     1.00                        shl	rdx, cl
 1      2     0.20                        and	rdx, r8

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     2.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r9d, edx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	dl, 64
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	rdx, -1
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, r9d
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	rdx, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	rdx, r8
```
## `checked_floor_to_multiple_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      22
Total Cycles:      23
Total uOps:        29

Dispatch Width:    6
uOps Per Cycle:    1.26
IPC:               0.96
Block RThroughput: 4.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB60_1
 1      1     0.20                        mov	r10, -1
 1      1     0.20                        mov	r11, -1
 1      0     0.20                        mov	ecx, r9d
 2      2     1.00                        shl	r11, cl
 1      2     0.20                        xor	ecx, ecx
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	r10, r11
 1      1     0.50                        cmove	rcx, r11
 1      2     0.20                        and	rcx, rdx
 1      2     0.20                        and	r10, r8
 2      12    0.50           *            mov	qword ptr [rax + 24], r10
 2      12    0.50           *            mov	qword ptr [rax + 16], rcx
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
4.00   2.00    -      -     3.00   3.00   4.00   3.00   3.00   3.00   3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, r9b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB60_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r10, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r11, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r11, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	ecx, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r10, r11
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	rcx, r11
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	rcx, rdx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	r10, r8
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 24], r10
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 16], rcx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, 1
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 8], 0
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 8], 0
```
## `checked_mod_floor_i8_unb_pow2`
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
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        cmp	dl, 8
 2      2     1.00                        setb	al
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
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	dl, 8
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	dl, -1
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, r9d
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	dl, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	dl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	dl, r8b
```
## `checked_mod_floor_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      10
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.20
IPC:               1.00
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r9d, edx
 1      0     0.20                        mov	r8d, ecx
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 16
 2      2     1.00                        setb	al
 1      1     0.20                        mov	edx, -1
 1      0     0.20                        mov	ecx, r9d
 2      2     1.00                        shl	edx, cl
 1      1     0.20                        not	edx
 1      2     0.20                        and	edx, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   2.00    -      -      -     2.00   4.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r9d, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 16
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	edx, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, r9d
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	edx, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	edx, r8d
```
## `checked_mod_floor_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      10
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.20
IPC:               1.00
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r9d, edx
 1      0     0.20                        mov	r8d, ecx
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 32
 2      2     1.00                        setb	al
 1      1     0.20                        mov	edx, -1
 1      0     0.20                        mov	ecx, r9d
 2      2     1.00                        shl	edx, cl
 1      1     0.20                        not	edx
 1      2     0.20                        and	edx, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   2.00    -      -      -     2.00   4.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r9d, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 32
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	edx, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, r9d
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	edx, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	edx, r8d
```
## `checked_mod_floor_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      9
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    1.22
IPC:               1.11
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r9d, edx
 0      1     0.00                        mov	r8, rcx
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 64
 2      2     1.00                        setb	al
 1      1     0.20                        mov	rdx, -1
 1      0     0.20                        mov	ecx, r9d
 2      2     1.00                        shl	rdx, cl
 1      1     0.20                        not	rdx
 1      2     0.20                        and	rdx, r8

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   2.00    -      -      -     2.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r9d, edx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	dl, 64
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	rdx, -1
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, r9d
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	rdx, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	rdx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	rdx, r8
```
## `checked_mod_floor_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      24
Total Cycles:      24
Total uOps:        31

Dispatch Width:    6
uOps Per Cycle:    1.29
IPC:               1.00
Block RThroughput: 5.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB65_1
 1      1     0.20                        mov	r10, -1
 1      0     0.20                        mov	ecx, r9d
 2      2     1.00                        shl	r10, cl
 1      1     0.20                        mov	rcx, -1
 1      2     0.20                        xor	r11d, r11d
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmove	r11, r10
 1      1     0.50                        cmove	r10, rcx
 1      1     0.20                        not	r10
 1      1     0.20                        not	r11
 1      2     0.20                        and	rdx, r11
 1      2     0.20                        and	r8, r10
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
4.00   3.00    -      -     3.00   3.00   4.00   3.00   3.00   3.00   4.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, r9b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB65_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r10, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, r9d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r10, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	rcx, -1
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	r11d, r11d
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	r11, r10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	r10, rcx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	r10
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	r11
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	rdx, r11
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	r8, r10
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 24], r8
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 16], rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, 1
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
Instructions:      12
Total Cycles:      11
Total uOps:        15

Dispatch Width:    6
uOps Per Cycle:    1.36
IPC:               1.09
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, edx
 1      0     0.20                        mov	r9d, ecx
 1      0     0.20                        mov	eax, r9d
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	eax, cl
 1      1     0.33                        movsx	edx, ax
 1      0     0.20                        mov	r10d, edx
 1      0     0.20                        mov	ecx, r8d
 2      2     1.00                        sar	r10d, cl
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	r9w, r10w
 2      2     1.00                        sete	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   2.00    -      -      -     2.00   4.00    -      -      -     3.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r9d, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	eax, r9d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	eax, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movsx	edx, ax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r10d, edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, r8d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	r10d, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	r9w, r10w
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sete	al
```
## `checked_mul_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      14
Total uOps:        18

Dispatch Width:    6
uOps Per Cycle:    1.29
IPC:               1.00
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 15
 1      1     0.50                        ja	.LBB73_1
 1      0     0.20                        mov	eax, edx
 1      0     0.20                        mov	edx, ecx
 1      0     0.20                        mov	r8d, ecx
 1      0     0.20                        mov	ecx, eax
 2      2     1.00                        shl	edx, cl
 1      1     0.33                        movsx	r9d, dx
 2      2     1.00                        sar	r9d, cl
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	r8w, r9w
 2      2     1.00                        sete	al
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   2.00    -      -      -     2.00   5.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 15
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB73_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, eax
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	edx, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movsx	r9d, dx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	r9d, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	r8w, r9w
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sete	al
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
```
## `checked_mul_i32_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      10
Total uOps:        13

Dispatch Width:    6
uOps Per Cycle:    1.30
IPC:               1.00
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, edx
 1      0     0.20                        mov	r9d, ecx
 1      0     0.20                        mov	edx, ecx
 1      0     0.20                        mov	ecx, r8d
 2      2     1.00                        shl	edx, cl
 1      0     0.20                        mov	r10d, edx
 2      2     1.00                        sar	r10d, cl
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	r9d, r10d
 2      2     1.00                        sete	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   1.00    -      -      -     2.00   4.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r9d, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	edx, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r10d, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	r10d, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	r9d, r10d
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sete	al
```
## `checked_mul_i32_unb_pow2`
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
 1      1     0.20                        cmp	dl, 31
 1      1     0.50                        ja	.LBB75_1
 1      0     0.20                        mov	eax, edx
 1      0     0.20                        mov	edx, ecx
 1      0     0.20                        mov	r8d, ecx
 1      0     0.20                        mov	ecx, eax
 2      2     1.00                        shl	edx, cl
 1      0     0.20                        mov	r9d, edx
 2      2     1.00                        sar	r9d, cl
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	r8d, r9d
 2      2     1.00                        sete	al
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   2.00    -      -      -     2.00   5.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 31
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB75_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, eax
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	edx, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r9d, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	r9d, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	r8d, r9d
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sete	al
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
```
## `checked_mul_i64_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      12
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    0.83
IPC:               0.83
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, edx
 0      1     0.00                        mov	r9, rcx
 0      1     0.00                        mov	rdx, rcx
 1      0     0.20                        mov	ecx, r8d
 2      2     1.00                        shl	rdx, cl
 0      1     0.00                        mov	r10, rdx
 2      2     1.00                        sar	r10, cl
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	r9, r10
 2      2     1.00                        sete	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   1.00    -      -      -     1.00   3.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, edx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, r8d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, rdx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sar	r10, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	r9, r10
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sete	al
```
## `checked_mul_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      14
Total uOps:        15

Dispatch Width:    6
uOps Per Cycle:    1.07
IPC:               1.00
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 63
 1      1     0.50                        ja	.LBB77_1
 1      0     0.20                        mov	eax, edx
 0      1     0.00                        mov	rdx, rcx
 0      1     0.00                        mov	r8, rcx
 1      0     0.20                        mov	ecx, eax
 2      2     1.00                        shl	rdx, cl
 0      1     0.00                        mov	r9, rdx
 2      2     1.00                        sar	r9, cl
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	r8, r9
 2      2     1.00                        sete	al
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   1.00    -      -      -     2.00   5.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 63
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB77_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, edx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, eax
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rdx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sar	r9, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	r8, r9
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sete	al
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
```
## `checked_mul_i128_pow2`
```asm
Iterations:        1
Instructions:      37
Total Cycles:      30
Total uOps:        45

Dispatch Width:    6
uOps Per Cycle:    1.50
IPC:               1.23
Block RThroughput: 7.5

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
 0      1     0.00                        mov	r11, rdx
 2      2     1.00                        shl	r11, cl
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	r10, r11
 0      1     0.00                        mov	rdi, r10
 2      2     1.00                        sar	rdi, cl
 0      1     0.00                        mov	rbx, r10
 1      1     0.50                        sar	rbx, 63
 1      2     0.20                        xor	esi, esi
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmove	rbx, rdi
 1      1     0.50                        cmovne	r11, rsi
 0      1     0.00                        mov	r14, r11
 3      5     1.00                        shrd	r14, r10, cl
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	r14, rdi
 1      2     0.20                        xor	rbx, r8
 1      2     0.20                        xor	r14, rdx
 1      2     0.20                        or	r14, rbx
 1      1     0.50                        jne	.LBB70_2
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
5.00   4.00   1.00   1.00   4.00   4.00   8.00   4.00   4.00   4.00   4.00   2.00    -     

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
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, rdx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r11, cl
1.00    -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r10, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdi, r10
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sar	rdi, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rbx, r10
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	rbx, 63
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	esi, esi
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	rbx, rdi
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r11, rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r14, r11
1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -     shrd	r14, r10, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r14, rdi
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	rbx, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	r14, rdx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     or	r14, rbx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     jne	.LBB70_2
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
Instructions:      41
Total Cycles:      30
Total uOps:        48

Dispatch Width:    6
uOps Per Cycle:    1.60
IPC:               1.37
Block RThroughput: 8.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	r14
 2      1     0.50           *            push	rsi
 2      1     0.50           *            push	rdi
 2      1     0.50           *            push	rbx
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB71_1
 0      1     0.00                        mov	r10, r8
 1      0     0.20                        mov	ecx, r9d
 3      5     1.00                        shld	r10, rdx, cl
 0      1     0.00                        mov	r11, rdx
 2      2     1.00                        shl	r11, cl
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	r10, r11
 0      1     0.00                        mov	rdi, r10
 2      2     1.00                        sar	rdi, cl
 0      1     0.00                        mov	rbx, r10
 1      1     0.50                        sar	rbx, 63
 1      2     0.20                        xor	esi, esi
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmove	rbx, rdi
 1      1     0.50                        cmovne	r11, rsi
 0      1     0.00                        mov	r14, r11
 3      5     1.00                        shrd	r14, r10, cl
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	r14, rdi
 1      2     0.20                        xor	rbx, r8
 1      2     0.20                        xor	r14, rdx
 1      2     0.20                        or	r14, rbx
 1      1     0.50                        jne	.LBB71_4
 2      12    0.50           *            mov	qword ptr [rax + 16], r11
 2      12    0.50           *            mov	qword ptr [rax + 24], r10
 1      1     0.20                        mov	esi, 1
 0      0     0.00                        jmp	.LBB71_4
 1      2     0.20                        xor	esi, esi
 2      12    0.50           *            mov	qword ptr [rax], rsi
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rbx
 1      5     0.33    *                   pop	rdi
 1      5     0.33    *                   pop	rsi
 1      5     0.33    *                   pop	r14

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
8.00   4.00   1.00   1.00   4.00   5.00   6.00   4.00   4.00   4.00   5.00   2.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	r14
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     push	rsi
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rdi
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     push	rbx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, r9b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB71_1
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, r9d
1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -     shld	r10, rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, rdx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r11, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r10, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdi, r10
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	rdi, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rbx, r10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sar	rbx, 63
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	esi, esi
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	rbx, rdi
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r11, rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r14, r11
 -     1.00    -      -      -      -     1.00    -      -      -     1.00    -      -     shrd	r14, r10, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r14, rdi
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	rbx, r8
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	r14, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     or	r14, rbx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jne	.LBB71_4
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 16], r11
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 24], r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	esi, 1
 -      -      -      -      -      -      -      -      -      -      -      -      -     jmp	.LBB71_4
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	esi, esi
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax], rsi
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
Instructions:      12
Total Cycles:      11
Total uOps:        15

Dispatch Width:    6
uOps Per Cycle:    1.36
IPC:               1.09
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, edx
 1      0     0.20                        mov	r9d, ecx
 1      0     0.20                        mov	eax, r9d
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	eax, cl
 1      1     0.20                        movzx	edx, ax
 1      0     0.20                        mov	r10d, edx
 1      0     0.20                        mov	ecx, r8d
 2      2     1.00                        shr	r10d, cl
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	r9w, r10w
 2      2     1.00                        sete	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   2.00    -      -      -     2.00   4.00    -      -      -     3.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r9d, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	eax, r9d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	eax, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     movzx	edx, ax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r10d, edx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, r8d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r10d, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	r9w, r10w
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sete	al
```
## `checked_mul_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      14
Total uOps:        18

Dispatch Width:    6
uOps Per Cycle:    1.29
IPC:               1.00
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 15
 1      1     0.50                        ja	.LBB83_1
 1      0     0.20                        mov	eax, edx
 1      0     0.20                        mov	edx, ecx
 1      0     0.20                        mov	r8d, ecx
 1      0     0.20                        mov	ecx, eax
 2      2     1.00                        shl	edx, cl
 1      1     0.20                        movzx	r9d, dx
 2      2     1.00                        shr	r9d, cl
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	r8w, r9w
 2      2     1.00                        sete	al
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   2.00    -      -      -     2.00   5.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 15
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB83_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, eax
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	edx, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movzx	r9d, dx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r9d, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	r8w, r9w
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sete	al
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
```
## `checked_mul_u32_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      10
Total uOps:        13

Dispatch Width:    6
uOps Per Cycle:    1.30
IPC:               1.00
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, edx
 1      0     0.20                        mov	r9d, ecx
 1      0     0.20                        mov	edx, ecx
 1      0     0.20                        mov	ecx, r8d
 2      2     1.00                        shl	edx, cl
 1      0     0.20                        mov	r10d, edx
 2      2     1.00                        shr	r10d, cl
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	r9d, r10d
 2      2     1.00                        sete	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   1.00    -      -      -     2.00   4.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r9d, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	edx, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r10d, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r10d, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	r9d, r10d
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sete	al
```
## `checked_mul_u32_unb_pow2`
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
 1      1     0.20                        cmp	dl, 31
 1      1     0.50                        ja	.LBB85_1
 1      0     0.20                        mov	eax, edx
 1      0     0.20                        mov	edx, ecx
 1      0     0.20                        mov	r8d, ecx
 1      0     0.20                        mov	ecx, eax
 2      2     1.00                        shl	edx, cl
 1      0     0.20                        mov	r9d, edx
 2      2     1.00                        shr	r9d, cl
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	r8d, r9d
 2      2     1.00                        sete	al
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   2.00    -      -      -     2.00   5.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 31
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     ja	.LBB85_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, eax
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	edx, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r9d, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r9d, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	r8d, r9d
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sete	al
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
```
## `checked_mul_u64_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      12
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    0.83
IPC:               0.83
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, edx
 0      1     0.00                        mov	r9, rcx
 0      1     0.00                        mov	rdx, rcx
 1      0     0.20                        mov	ecx, r8d
 2      2     1.00                        shl	rdx, cl
 0      1     0.00                        mov	r10, rdx
 2      2     1.00                        shr	r10, cl
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	r9, r10
 2      2     1.00                        sete	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   1.00    -      -      -     1.00   3.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, edx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, r8d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, rdx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	r10, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	r9, r10
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sete	al
```
## `checked_mul_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      14
Total uOps:        15

Dispatch Width:    6
uOps Per Cycle:    1.07
IPC:               1.00
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 63
 1      1     0.50                        ja	.LBB87_1
 1      0     0.20                        mov	eax, edx
 0      1     0.00                        mov	rdx, rcx
 0      1     0.00                        mov	r8, rcx
 1      0     0.20                        mov	ecx, eax
 2      2     1.00                        shl	rdx, cl
 0      1     0.00                        mov	r9, rdx
 2      2     1.00                        shr	r9, cl
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	r8, r9
 2      2     1.00                        sete	al
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   1.00    -      -      -     2.00   5.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 63
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB87_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, edx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, eax
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rdx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	r9, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	r8, r9
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sete	al
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
```
## `checked_mul_u128_pow2`
```asm
Iterations:        1
Instructions:      36
Total Cycles:      30
Total uOps:        44

Dispatch Width:    6
uOps Per Cycle:    1.47
IPC:               1.20
Block RThroughput: 7.3

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
 0      1     0.00                        mov	r11, rdx
 2      2     1.00                        shl	r11, cl
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	r10, r11
 0      1     0.00                        mov	rdi, r10
 2      2     1.00                        shr	rdi, cl
 1      2     0.20                        xor	esi, esi
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	r11, rsi
 0      1     0.00                        mov	rbx, rdi
 1      1     0.50                        cmovne	rbx, rsi
 0      1     0.00                        mov	r14, r11
 3      5     1.00                        shrd	r14, r10, cl
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	r14, rdi
 1      2     0.20                        xor	r14, rdx
 1      2     0.20                        xor	rbx, r8
 1      2     0.20                        or	rbx, r14
 1      1     0.50                        jne	.LBB80_2
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
5.00   4.00   1.00   1.00   4.00   4.00   7.00   4.00   4.00   4.00   4.00   2.00    -     

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
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, rdx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r11, cl
1.00    -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r10, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdi, r10
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	rdi, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	esi, esi
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r11, rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rbx, rdi
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rbx, rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r14, r11
1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -     shrd	r14, r10, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r14, rdi
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	r14, rdx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	rbx, r8
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     or	rbx, r14
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jne	.LBB80_2
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
## `checked_mul_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      40
Total Cycles:      30
Total uOps:        47

Dispatch Width:    6
uOps Per Cycle:    1.57
IPC:               1.33
Block RThroughput: 7.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	r14
 2      1     0.50           *            push	rsi
 2      1     0.50           *            push	rdi
 2      1     0.50           *            push	rbx
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB81_1
 0      1     0.00                        mov	r10, r8
 1      0     0.20                        mov	ecx, r9d
 3      5     1.00                        shld	r10, rdx, cl
 0      1     0.00                        mov	r11, rdx
 2      2     1.00                        shl	r11, cl
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	r10, r11
 0      1     0.00                        mov	rdi, r10
 2      2     1.00                        shr	rdi, cl
 1      2     0.20                        xor	esi, esi
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	r11, rsi
 0      1     0.00                        mov	rbx, rdi
 1      1     0.50                        cmovne	rbx, rsi
 0      1     0.00                        mov	r14, r11
 3      5     1.00                        shrd	r14, r10, cl
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	r14, rdi
 1      2     0.20                        xor	r14, rdx
 1      2     0.20                        xor	rbx, r8
 1      2     0.20                        or	rbx, r14
 1      1     0.50                        jne	.LBB81_4
 2      12    0.50           *            mov	qword ptr [rax + 16], r11
 2      12    0.50           *            mov	qword ptr [rax + 24], r10
 1      1     0.20                        mov	esi, 1
 0      0     0.00                        jmp	.LBB81_4
 1      2     0.20                        xor	esi, esi
 2      12    0.50           *            mov	qword ptr [rax], rsi
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rbx
 1      5     0.33    *                   pop	rdi
 1      5     0.33    *                   pop	rsi
 1      5     0.33    *                   pop	r14

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
7.00   4.00   1.00   1.00   4.00   5.00   6.00   4.00   4.00   4.00   5.00   2.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	r14
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     push	rsi
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rdi
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     push	rbx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, r9b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB81_1
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, r9d
1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -     shld	r10, rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, rdx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r11, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r10, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdi, r10
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	rdi, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	esi, esi
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r11, rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rbx, rdi
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rbx, rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r14, r11
 -     1.00    -      -      -      -     1.00    -      -      -     1.00    -      -     shrd	r14, r10, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r14, rdi
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	r14, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	rbx, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     or	rbx, r14
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     jne	.LBB81_4
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 16], r11
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 24], r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	esi, 1
 -      -      -      -      -      -      -      -      -      -      -      -      -     jmp	.LBB81_4
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	esi, esi
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax], rsi
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 8], 0
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rbx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -     pop	rdi
 -      -     1.00    -      -      -      -      -      -      -      -      -      -     pop	rsi
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	r14
```
## `div_ceil_i8_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      10
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.80
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      1     0.20                        mov	r8, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r8, cl
 1      1     0.20                        not	r8
 1      1     0.33                        movsx	rax, al
 1      1     0.20                        add	rax, r8
 2      2     1.00                        sar	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   1.00    -      -      -     1.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r8, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	r8
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movsx	rax, al
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     add	rax, r8
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
```
## `div_ceil_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      10
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.80
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      1     0.20                        mov	r8, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r8, cl
 1      1     0.20                        not	r8
 1      1     0.33                        movsx	rax, al
 1      1     0.20                        add	rax, r8
 2      2     1.00                        sar	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   1.00    -      -      -     1.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r8, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	r8
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movsx	rax, al
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     add	rax, r8
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
```
## `div_ceil_i16_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      10
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.80
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      1     0.20                        mov	r8, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r8, cl
 1      1     0.20                        not	r8
 1      1     0.33                        movsx	rax, ax
 1      1     0.20                        add	rax, r8
 2      2     1.00                        sar	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   1.00    -      -      -     1.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r8, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	r8
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movsx	rax, ax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     add	rax, r8
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
```
## `div_ceil_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      10
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.80
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      1     0.20                        mov	r8, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r8, cl
 1      1     0.20                        not	r8
 1      1     0.33                        movsx	rax, ax
 1      1     0.20                        add	rax, r8
 2      2     1.00                        sar	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   1.00    -      -      -     1.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r8, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	r8
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movsx	rax, ax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     add	rax, r8
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
```
## `div_ceil_i32_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      10
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.80
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      1     0.20                        mov	r8, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r8, cl
 1      1     0.20                        not	r8
 1      1     0.50                        cdqe
 1      1     0.20                        add	rax, r8
 2      2     1.00                        sar	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   1.00    -      -      -     1.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r8, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	r8
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cdqe
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     add	rax, r8
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
```
## `div_ceil_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      10
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.80
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      1     0.20                        mov	r8, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r8, cl
 1      1     0.20                        not	r8
 1      1     0.50                        cdqe
 1      1     0.20                        add	rax, r8
 2      2     1.00                        sar	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   1.00    -      -      -     1.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r8, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	r8
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cdqe
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     add	rax, r8
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
```
## `div_ceil_i64_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      13
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    0.77
IPC:               0.77
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	r8, rcx
 0      1     0.00                        mov	r9, rcx
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        sar	r9, cl
 0      1     0.00                        mov	r10, r9
 2      2     1.00                        shl	r10, cl
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	r8, r10
 2      2     1.00                        setne	al
 1      1     0.20                        add	rax, r9

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   1.00    -      -      -     1.00   3.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	r9, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, r9
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r10, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	r8, r10
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setne	al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	rax, r9
```
## `div_ceil_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      13
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    0.77
IPC:               0.77
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	r8, rcx
 0      1     0.00                        mov	r9, rcx
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        sar	r9, cl
 0      1     0.00                        mov	r10, r9
 2      2     1.00                        shl	r10, cl
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	r8, r10
 2      2     1.00                        setne	al
 1      1     0.20                        add	rax, r9

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   1.00    -      -      -     1.00   3.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	r9, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, r9
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r10, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	r8, r10
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setne	al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	rax, r9
```
## `div_ceil_i128_pow2`
```asm
Iterations:        1
Instructions:      33
Total Cycles:      25
Total uOps:        35

Dispatch Width:    6
uOps Per Cycle:    1.40
IPC:               1.32
Block RThroughput: 7.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	rsi
 2      1     0.50           *            push	rdi
 0      1     0.00                        mov	rax, rcx
 0      1     0.00                        mov	r10, rcx
 1      0     0.20                        mov	ecx, r8d
 3      5     1.00                        shrd	r10, rdx, cl
 0      1     0.00                        mov	r11, rdx
 2      2     1.00                        sar	r11, cl
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	r10, r11
 0      1     0.00                        mov	rsi, r10
 2      2     1.00                        shl	rsi, cl
 1      2     0.20                        xor	edi, edi
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmove	rdi, rsi
 0      1     0.00                        mov	r9, rdx
 1      1     0.50                        sar	r9, 63
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmove	r9, r11
 0      1     0.00                        mov	r11, r9
 3      5     1.00                        shld	r11, r10, cl
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	r11, rsi
 1      2     0.20                        xor	r11, rdx
 1      2     0.20                        xor	rdi, rax
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        or	rdi, r11
 2      2     1.00                        setne	al
 1      1     0.20                        add	rax, r10
 1      1     0.50                        adc	r9, 0
 0      1     0.00                        mov	rdx, r9
 1      5     0.33    *                   pop	rdi
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
7.00   4.00    -     1.00   1.00   4.00   9.00   1.00   1.00   1.00   5.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rsi
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     push	rdi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, r8d
 -     1.00    -      -      -     1.00   1.00    -      -      -      -      -      -     shrd	r10, rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, rdx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sar	r11, cl
1.00    -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r10, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rsi, r10
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	rsi, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	edi, edi
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r8b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	rdi, rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rdx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	r9, 63
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	r9, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, r9
1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -     shld	r11, r10, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r11, rsi
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	r11, rdx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	rdi, rax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	eax, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     or	rdi, r11
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setne	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	rax, r10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     adc	r9, 0
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r9
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rdi
 -      -      -     1.00    -      -      -      -      -      -      -      -      -     pop	rsi
```
## `div_ceil_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      33
Total Cycles:      25
Total uOps:        35

Dispatch Width:    6
uOps Per Cycle:    1.40
IPC:               1.32
Block RThroughput: 7.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	rsi
 2      1     0.50           *            push	rdi
 0      1     0.00                        mov	rax, rcx
 0      1     0.00                        mov	r10, rcx
 1      0     0.20                        mov	ecx, r8d
 3      5     1.00                        shrd	r10, rdx, cl
 0      1     0.00                        mov	r11, rdx
 2      2     1.00                        sar	r11, cl
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	r10, r11
 0      1     0.00                        mov	rsi, r10
 2      2     1.00                        shl	rsi, cl
 1      2     0.20                        xor	edi, edi
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmove	rdi, rsi
 0      1     0.00                        mov	r9, rdx
 1      1     0.50                        sar	r9, 63
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmove	r9, r11
 0      1     0.00                        mov	r11, r9
 3      5     1.00                        shld	r11, r10, cl
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	r11, rsi
 1      2     0.20                        xor	r11, rdx
 1      2     0.20                        xor	rdi, rax
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        or	rdi, r11
 2      2     1.00                        setne	al
 1      1     0.20                        add	rax, r10
 1      1     0.50                        adc	r9, 0
 0      1     0.00                        mov	rdx, r9
 1      5     0.33    *                   pop	rdi
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
7.00   4.00    -     1.00   1.00   4.00   9.00   1.00   1.00   1.00   5.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rsi
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     push	rdi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, r8d
 -     1.00    -      -      -     1.00   1.00    -      -      -      -      -      -     shrd	r10, rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, rdx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sar	r11, cl
1.00    -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r10, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rsi, r10
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	rsi, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	edi, edi
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r8b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	rdi, rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rdx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	r9, 63
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	r9, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, r9
1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -     shld	r11, r10, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r11, rsi
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	r11, rdx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	rdi, rax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	eax, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     or	rdi, r11
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setne	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	rax, r10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     adc	r9, 0
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r9
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rdi
 -      -      -     1.00    -      -      -      -      -      -      -      -      -     pop	rsi
```
## `div_ceil_u8_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      10
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.80
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      1     0.20                        mov	r8, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r8, cl
 1      1     0.20                        not	r8
 1      0     0.20                        movzx	eax, al
 1      1     0.20                        add	rax, r8
 2      2     1.00                        sar	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   1.00    -      -      -     1.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r8, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	r8
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     add	rax, r8
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
```
## `div_ceil_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      10
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.80
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      1     0.20                        mov	r8, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r8, cl
 1      1     0.20                        not	r8
 1      0     0.20                        movzx	eax, al
 1      1     0.20                        add	rax, r8
 2      2     1.00                        sar	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   1.00    -      -      -     1.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r8, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	r8
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     add	rax, r8
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
```
## `div_ceil_u16_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      10
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.80
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      1     0.20                        mov	r8, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r8, cl
 1      1     0.20                        not	r8
 1      1     0.20                        movzx	eax, ax
 1      1     0.20                        add	rax, r8
 2      2     1.00                        sar	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   1.00    -      -      -     1.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r8, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	r8
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movzx	eax, ax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     add	rax, r8
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
```
## `div_ceil_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      10
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.80
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      1     0.20                        mov	r8, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r8, cl
 1      1     0.20                        not	r8
 1      1     0.20                        movzx	eax, ax
 1      1     0.20                        add	rax, r8
 2      2     1.00                        sar	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   1.00    -      -      -     1.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r8, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	r8
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movzx	eax, ax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     add	rax, r8
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
```
## `div_ceil_u32_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      10
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.80
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      1     0.20                        mov	r8, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r8, cl
 1      1     0.20                        not	r8
 1      0     0.20                        mov	eax, eax
 1      1     0.20                        add	rax, r8
 2      2     1.00                        sar	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   1.00    -      -      -     1.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r8, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	r8
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     add	rax, r8
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
```
## `div_ceil_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      10
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.80
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      1     0.20                        mov	r8, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r8, cl
 1      1     0.20                        not	r8
 1      0     0.20                        mov	eax, eax
 1      1     0.20                        add	rax, r8
 2      2     1.00                        sar	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   1.00    -      -      -     1.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r8, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	r8
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     add	rax, r8
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
```
## `div_ceil_u64_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      13
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    0.77
IPC:               0.77
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	r8, rcx
 0      1     0.00                        mov	r9, rcx
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shr	r9, cl
 0      1     0.00                        mov	r10, r9
 2      2     1.00                        shl	r10, cl
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	r8, r10
 2      2     1.00                        setne	al
 1      1     0.20                        add	rax, r9

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   1.00    -      -      -     1.00   3.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r9, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, r9
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r10, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	r8, r10
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setne	al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	rax, r9
```
## `div_ceil_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      13
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    0.77
IPC:               0.77
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	r8, rcx
 0      1     0.00                        mov	r9, rcx
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shr	r9, cl
 0      1     0.00                        mov	r10, r9
 2      2     1.00                        shl	r10, cl
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	r8, r10
 2      2     1.00                        setne	al
 1      1     0.20                        add	rax, r9

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   1.00    -      -      -     1.00   3.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r9, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, r9
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r10, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	r8, r10
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setne	al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	rax, r9
```
## `div_ceil_u128_pow2`
```asm
Iterations:        1
Instructions:      30
Total Cycles:      24
Total uOps:        34

Dispatch Width:    6
uOps Per Cycle:    1.42
IPC:               1.25
Block RThroughput: 6.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	rsi
 2      1     0.50           *            push	rdi
 1      0     0.20                        mov	r9d, r8d
 0      1     0.00                        mov	r8, rdx
 0      1     0.00                        mov	rax, rcx
 0      1     0.00                        mov	r10, rcx
 1      0     0.20                        mov	ecx, r9d
 3      5     1.00                        shrd	r10, rdx, cl
 2      2     1.00                        shr	rdx, cl
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	r10, rdx
 0      1     0.00                        mov	r11, r10
 2      2     1.00                        shl	r11, cl
 1      2     0.20                        xor	esi, esi
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	rdx, rsi
 1      1     0.50                        cmove	rsi, r11
 0      1     0.00                        mov	rdi, rdx
 3      5     1.00                        shld	rdi, r10, cl
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	rdi, r11
 1      2     0.20                        xor	rdi, r8
 1      2     0.20                        xor	rsi, rax
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        or	rsi, rdi
 2      2     1.00                        setne	al
 1      1     0.20                        add	rax, r10
 1      1     0.50                        adc	rdx, 0
 1      5     0.33    *                   pop	rdi
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
7.00   4.00    -     1.00   1.00   4.00   8.00   1.00   1.00   1.00   5.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rsi
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     push	rdi
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r9d, r8d
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, r9d
1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -     shrd	r10, rdx, cl
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	rdx, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r10, rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, r10
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r11, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	esi, esi
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rdx, rsi
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	rsi, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdi, rdx
 -     1.00    -      -      -     1.00   1.00    -      -      -      -      -      -     shld	rdi, r10, cl
1.00    -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rdi, r11
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	rdi, r8
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	rsi, rax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     or	rsi, rdi
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setne	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	rax, r10
1.00    -      -      -      -      -      -      -      -      -      -      -      -     adc	rdx, 0
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rdi
 -      -      -     1.00    -      -      -      -      -      -      -      -      -     pop	rsi
```
## `div_ceil_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      30
Total Cycles:      24
Total uOps:        34

Dispatch Width:    6
uOps Per Cycle:    1.42
IPC:               1.25
Block RThroughput: 6.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	rsi
 2      1     0.50           *            push	rdi
 1      0     0.20                        mov	r9d, r8d
 0      1     0.00                        mov	r8, rdx
 0      1     0.00                        mov	rax, rcx
 0      1     0.00                        mov	r10, rcx
 1      0     0.20                        mov	ecx, r9d
 3      5     1.00                        shrd	r10, rdx, cl
 2      2     1.00                        shr	rdx, cl
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	r10, rdx
 0      1     0.00                        mov	r11, r10
 2      2     1.00                        shl	r11, cl
 1      2     0.20                        xor	esi, esi
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	rdx, rsi
 1      1     0.50                        cmove	rsi, r11
 0      1     0.00                        mov	rdi, rdx
 3      5     1.00                        shld	rdi, r10, cl
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	rdi, r11
 1      2     0.20                        xor	rdi, r8
 1      2     0.20                        xor	rsi, rax
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        or	rsi, rdi
 2      2     1.00                        setne	al
 1      1     0.20                        add	rax, r10
 1      1     0.50                        adc	rdx, 0
 1      5     0.33    *                   pop	rdi
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
7.00   4.00    -     1.00   1.00   4.00   8.00   1.00   1.00   1.00   5.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rsi
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     push	rdi
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r9d, r8d
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, r9d
1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -     shrd	r10, rdx, cl
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	rdx, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r10, rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, r10
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r11, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	esi, esi
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rdx, rsi
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	rsi, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdi, rdx
 -     1.00    -      -      -     1.00   1.00    -      -      -      -      -      -     shld	rdi, r10, cl
1.00    -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rdi, r11
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	rdi, r8
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	rsi, rax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     or	rsi, rdi
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setne	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	rax, r10
1.00    -      -      -      -      -      -      -      -      -      -      -      -     adc	rdx, 0
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rdi
 -      -      -     1.00    -      -      -      -      -      -      -      -      -     pop	rsi
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
Instructions:      3
Total Cycles:      6
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.67
IPC:               0.50
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.33                        movsx	eax, cx
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        sar	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00    -      -      -      -      -     1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movsx	eax, cx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	eax, cl
```
## `div_floor_i16_unb_pow2`
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
 1      1     0.33                        movsx	eax, cx
 1      2     0.20                        and	dl, 15
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        sar	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00    -      -      -      -     1.00   1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movsx	eax, cx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 15
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	eax, cl
```
## `div_floor_i32_pow2`
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
 2      2     1.00                        sar	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00    -      -      -      -      -     1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	eax, cl
```
## `div_floor_i32_unb_pow2`
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
 2      2     1.00                        sar	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00    -      -      -      -      -     1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	eax, cl
```
## `div_floor_i64_pow2`
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
 0      1     0.00                        mov	rax, rcx
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        sar	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
 -      -      -      -      -      -     2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sar	rax, cl
```
## `div_floor_i64_unb_pow2`
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
 0      1     0.00                        mov	rax, rcx
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        sar	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
 -      -      -      -      -      -     2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sar	rax, cl
```
## `div_floor_i128_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      10
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.90
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      0     0.20                        mov	ecx, r8d
 3      5     1.00                        shrd	rax, rdx, cl
 0      1     0.00                        mov	r9, rdx
 2      2     1.00                        sar	r9, cl
 1      1     0.50                        sar	rdx, 63
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	rax, r9
 1      1     0.50                        cmove	rdx, r9

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   1.00    -      -      -     1.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, r8d
 -     1.00    -      -      -     1.00   1.00    -      -      -      -      -      -     shrd	rax, rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rdx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	r9, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sar	rdx, 63
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rax, r9
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	rdx, r9
```
## `div_floor_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      10
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.90
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      0     0.20                        mov	ecx, r8d
 3      5     1.00                        shrd	rax, rdx, cl
 0      1     0.00                        mov	r9, rdx
 2      2     1.00                        sar	r9, cl
 1      1     0.50                        sar	rdx, 63
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	rax, r9
 1      1     0.50                        cmove	rdx, r9

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   1.00    -      -      -     1.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, r8d
 -     1.00    -      -      -     1.00   1.00    -      -      -      -      -      -     shrd	rax, rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rdx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	r9, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sar	rdx, 63
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rax, r9
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	rdx, r9
```
## `div_floor_u8_pow2`
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
## `div_floor_u8_unb_pow2`
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
## `div_floor_u16_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      6
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.67
IPC:               0.50
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        movzx	eax, cx
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shr	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00    -      -      -      -      -     1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	eax, cx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	eax, cl
```
## `div_floor_u16_unb_pow2`
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
 1      1     0.20                        movzx	eax, cx
 1      2     0.20                        and	dl, 15
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shr	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00    -      -      -      -     1.00   1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	eax, cx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 15
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	eax, cl
```
## `div_floor_u32_pow2`
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
 2      2     1.00                        shr	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00    -      -      -      -      -     1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	eax, cl
```
## `div_floor_u32_unb_pow2`
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
 2      2     1.00                        shr	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00    -      -      -      -      -     1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	eax, cl
```
## `div_floor_u64_pow2`
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
 0      1     0.00                        mov	rax, rcx
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shr	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
 -      -      -      -      -      -     2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	rax, cl
```
## `div_floor_u64_unb_pow2`
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
 0      1     0.00                        mov	rax, rcx
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shr	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
 -      -      -      -      -      -     2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	rax, cl
```
## `div_floor_u128_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      10
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.80
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      0     0.20                        mov	ecx, r8d
 3      5     1.00                        shrd	rax, rdx, cl
 2      2     1.00                        shr	rdx, cl
 1      2     0.20                        xor	ecx, ecx
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	rax, rdx
 1      1     0.50                        cmovne	rdx, rcx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   4.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, r8d
1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -     shrd	rax, rdx, cl
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	rdx, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	ecx, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rax, rdx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rdx, rcx
```
## `div_floor_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      10
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.80
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      0     0.20                        mov	ecx, r8d
 3      5     1.00                        shrd	rax, rdx, cl
 2      2     1.00                        shr	rdx, cl
 1      2     0.20                        xor	ecx, ecx
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	rax, rdx
 1      1     0.50                        cmovne	rdx, rcx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   4.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, r8d
1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -     shrd	rax, rdx, cl
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	rdx, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	ecx, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rax, rdx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rdx, rcx
```
## `div_i8_pow2`
```asm
Iterations:        1
Instructions:      12
Total Cycles:      11
Total uOps:        16

Dispatch Width:    6
uOps Per Cycle:    1.45
IPC:               1.09
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      2     0.20                        test	cl, cl
 1      1     0.50                        js	.LBB138_1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shr	al, cl
 2      7     0.50                  U     ret
 1      1     0.20                        mov	r8b, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r8b, cl
 1      1     0.20                        not	r8b
 1      1     0.20                        add	al, r8b
 2      2     1.00                        sar	al, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   2.00    -      -      -     2.00   4.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     test	cl, cl
1.00    -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB138_1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	al, cl
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r8b, -1
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r8b, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	r8b
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	al, r8b
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	al, cl
```
## `div_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      13
Total Cycles:      12
Total uOps:        17

Dispatch Width:    6
uOps Per Cycle:    1.42
IPC:               1.08
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      2     0.20                        and	dl, 7
 1      2     0.20                        test	cl, cl
 1      1     0.50                        js	.LBB139_1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shr	al, cl
 2      7     0.50                  U     ret
 1      1     0.20                        mov	r8b, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r8b, cl
 1      1     0.20                        not	r8b
 1      1     0.20                        add	al, r8b
 2      2     1.00                        sar	al, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   2.00    -      -      -     2.00   4.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 7
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	cl, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB139_1
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	al, cl
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r8b, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r8b, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	r8b
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	al, r8b
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	al, cl
```
## `div_i16_pow2`
```asm
Iterations:        1
Instructions:      13
Total Cycles:      13
Total uOps:        17

Dispatch Width:    6
uOps Per Cycle:    1.31
IPC:               1.00
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      0     0.20                        movzx	ecx, dl
 1      2     0.20                        test	ax, ax
 1      1     0.50                        js	.LBB132_1
 1      1     0.20                        movzx	eax, ax
 2      2     1.00                        shr	eax, cl
 2      7     0.50                  U     ret
 1      1     0.20                        mov	edx, -1
 2      2     1.00                        shl	edx, cl
 1      1     0.20                        not	edx
 1      1     0.20                        add	eax, edx
 1      1     0.50                        cwde
 2      2     1.00                        sar	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   2.00    -      -      -     3.00   5.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     movzx	ecx, dl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	ax, ax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB132_1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movzx	eax, ax
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	eax, cl
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	edx, -1
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	edx, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	eax, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cwde
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sar	eax, cl
```
## `div_i16_unb_pow2`
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
 1      0     0.20                        mov	eax, ecx
 1      2     0.20                        and	dl, 15
 1      0     0.20                        movzx	ecx, dl
 1      2     0.20                        test	ax, ax
 1      1     0.50                        js	.LBB133_1
 1      1     0.20                        movzx	eax, ax
 2      2     1.00                        shr	eax, cl
 2      7     0.50                  U     ret
 1      1     0.20                        mov	edx, -1
 2      2     1.00                        shl	edx, cl
 1      1     0.20                        not	edx
 1      1     0.20                        add	eax, edx
 1      1     0.50                        cwde
 2      2     1.00                        sar	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   2.00    -      -      -     3.00   6.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 15
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     movzx	ecx, dl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	ax, ax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB133_1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movzx	eax, ax
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	eax, cl
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	edx, -1
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	edx, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	eax, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cwde
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sar	eax, cl
```
## `div_i32_pow2`
```asm
Iterations:        1
Instructions:      12
Total Cycles:      11
Total uOps:        16

Dispatch Width:    6
uOps Per Cycle:    1.45
IPC:               1.09
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      2     0.20                        test	ecx, ecx
 1      1     0.50                        js	.LBB134_1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shr	eax, cl
 2      7     0.50                  U     ret
 1      1     0.20                        mov	r8d, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r8d, cl
 1      1     0.20                        not	r8d
 1      1     0.20                        add	eax, r8d
 2      2     1.00                        sar	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   2.00    -      -      -     2.00   4.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     test	ecx, ecx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB134_1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	eax, cl
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r8d, -1
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r8d, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	r8d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	eax, r8d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	eax, cl
```
## `div_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      13
Total Cycles:      12
Total uOps:        17

Dispatch Width:    6
uOps Per Cycle:    1.42
IPC:               1.08
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      2     0.20                        and	dl, 31
 1      2     0.20                        test	ecx, ecx
 1      1     0.50                        js	.LBB135_1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shr	eax, cl
 2      7     0.50                  U     ret
 1      1     0.20                        mov	r8d, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r8d, cl
 1      1     0.20                        not	r8d
 1      1     0.20                        add	eax, r8d
 2      2     1.00                        sar	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   2.00    -      -      -     2.00   4.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 31
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	ecx, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB135_1
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	eax, cl
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r8d, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r8d, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	r8d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	eax, r8d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	eax, cl
```
## `div_i64_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      12
Total uOps:        14

Dispatch Width:    6
uOps Per Cycle:    1.17
IPC:               0.92
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      0     0.20                        movzx	ecx, dl
 1      2     0.20                        test	rax, rax
 1      1     0.50                        js	.LBB136_1
 2      2     1.00                        shr	rax, cl
 2      7     0.50                  U     ret
 1      1     0.20                        mov	rdx, -1
 2      2     1.00                        shl	rdx, cl
 1      1     0.20                        not	rdx
 1      1     0.20                        add	rax, rdx
 2      2     1.00                        sar	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   1.00    -      -      -     1.00   4.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	ecx, dl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     test	rax, rax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB136_1
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	rax, cl
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	rdx, -1
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	rdx, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	rdx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	rax, rdx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
```
## `div_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      12
Total Cycles:      11
Total uOps:        15

Dispatch Width:    6
uOps Per Cycle:    1.36
IPC:               1.09
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        and	dl, 63
 1      0     0.20                        movzx	ecx, dl
 1      2     0.20                        test	rax, rax
 1      1     0.50                        js	.LBB137_1
 2      2     1.00                        shr	rax, cl
 2      7     0.50                  U     ret
 1      1     0.20                        mov	rdx, -1
 2      2     1.00                        shl	rdx, cl
 1      1     0.20                        not	rdx
 1      1     0.20                        add	rax, rdx
 2      2     1.00                        sar	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   1.00    -      -      -     2.00   4.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	dl, 63
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movzx	ecx, dl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     test	rax, rax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB137_1
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	rax, cl
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	rdx, -1
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	rdx, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	rax, rdx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
```
## `div_i128_pow2`
```asm
Iterations:        1
Instructions:      32
Total Cycles:      18
Total uOps:        38

Dispatch Width:    6
uOps Per Cycle:    2.11
IPC:               1.78
Block RThroughput: 9.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	rdx, rdx
 1      1     0.50                        js	.LBB130_1
 1      0     0.20                        mov	ecx, r8d
 3      5     1.00                        shrd	rax, rdx, cl
 2      2     1.00                        shr	rdx, cl
 1      2     0.20                        xor	ecx, ecx
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	rax, rdx
 1      1     0.50                        cmovne	rdx, rcx
 2      7     0.50                  U     ret
 1      1     0.20                        mov	r9, -1
 1      1     0.20                        mov	r10, -1
 1      0     0.20                        mov	ecx, r8d
 2      2     1.00                        shl	r10, cl
 1      2     0.20                        xor	ecx, ecx
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmove	rcx, r10
 1      1     0.50                        cmove	r10, r9
 1      1     0.20                        not	r10
 1      1     0.20                        not	rcx
 1      1     0.20                        add	rax, rcx
 1      1     0.50                        adc	rdx, r10
 1      0     0.20                        mov	ecx, r8d
 3      5     1.00                        shrd	rax, rdx, cl
 0      1     0.00                        mov	r9, rdx
 2      2     1.00                        sar	r9, cl
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	rax, r9
 1      1     0.50                        sar	rdx, 63
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmove	rdx, r9

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
10.00  6.00    -      -      -     6.00   9.00    -      -      -     6.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	rdx, rdx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB130_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, r8d
1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -     shrd	rax, rdx, cl
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	rdx, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	ecx, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r8b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rax, rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rdx, rcx
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r9, -1
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r10, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, r8d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r10, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r8b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	rcx, r10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	r10, r9
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     adc	rdx, r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, r8d
1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -     shrd	rax, rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rdx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	r9, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rax, r9
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sar	rdx, 63
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	rdx, r9
```
## `div_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      33
Total Cycles:      20
Total uOps:        39

Dispatch Width:    6
uOps Per Cycle:    1.95
IPC:               1.65
Block RThroughput: 9.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        and	r8b, 127
 1      2     0.20                        test	rdx, rdx
 1      1     0.50                        js	.LBB131_1
 1      0     0.20                        mov	ecx, r8d
 3      5     1.00                        shrd	rax, rdx, cl
 2      2     1.00                        shr	rdx, cl
 1      2     0.20                        xor	ecx, ecx
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	rax, rdx
 1      1     0.50                        cmovne	rdx, rcx
 2      7     0.50                  U     ret
 1      1     0.20                        mov	r9, -1
 1      1     0.20                        mov	r10, -1
 1      0     0.20                        mov	ecx, r8d
 2      2     1.00                        shl	r10, cl
 1      2     0.20                        xor	ecx, ecx
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmove	rcx, r10
 1      1     0.50                        cmove	r10, r9
 1      1     0.20                        not	r10
 1      1     0.20                        not	rcx
 1      1     0.20                        add	rax, rcx
 1      1     0.50                        adc	rdx, r10
 1      0     0.20                        mov	ecx, r8d
 3      5     1.00                        shrd	rax, rdx, cl
 0      1     0.00                        mov	r9, rdx
 2      2     1.00                        sar	r9, cl
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	rax, r9
 1      1     0.50                        sar	rdx, 63
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmove	rdx, r9

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
10.00  6.00    -      -      -     6.00   10.00   -      -      -     6.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	r8b, 127
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     test	rdx, rdx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB131_1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, r8d
 -     1.00    -      -      -      -     1.00    -      -      -     1.00    -      -     shrd	rax, rdx, cl
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	rdx, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	ecx, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rax, rdx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rdx, rcx
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r9, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r10, -1
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, r8d
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r10, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	ecx, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	rcx, r10
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	r10, r9
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     adc	rdx, r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, r8d
1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -     shrd	rax, rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rdx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	r9, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rax, r9
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sar	rdx, 63
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	rdx, r9
```
## `floor_to_multiple_i8_pow2`
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
## `floor_to_multiple_i8_unb_pow2`
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
## `floor_to_multiple_i16_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      8
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.75
IPC:               0.50
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        movzx	eax, cx
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shr	eax, cl
 2      2     1.00                        shl	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00    -      -      -      -      -     3.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	eax, cx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	eax, cl
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	eax, cl
```
## `floor_to_multiple_i16_unb_pow2`
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
 1      1     0.20                        movzx	eax, cx
 1      2     0.20                        and	dl, 15
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shr	eax, cl
 2      2     1.00                        shl	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00    -      -      -      -     1.00   3.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	eax, cx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 15
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	eax, cl
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	eax, cl
```
## `floor_to_multiple_i32_pow2`
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
 2      2     1.00                        shr	eax, cl
 2      2     1.00                        shl	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00    -      -      -      -      -     3.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	eax, cl
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	eax, cl
```
## `floor_to_multiple_i32_unb_pow2`
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
 2      2     1.00                        shr	eax, cl
 2      2     1.00                        shl	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00    -      -      -      -      -     3.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	eax, cl
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	eax, cl
```
## `floor_to_multiple_i64_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      8
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.63
IPC:               0.50
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shr	rax, cl
 2      2     1.00                        shl	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00    -      -      -      -      -     2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	rax, cl
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	rax, cl
```
## `floor_to_multiple_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      8
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.63
IPC:               0.50
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shr	rax, cl
 2      2     1.00                        shl	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00    -      -      -      -      -     2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	rax, cl
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	rax, cl
```
## `floor_to_multiple_i128_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      9
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    1.22
IPC:               1.22
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	r9, rcx
 1      1     0.20                        mov	r10, -1
 1      0     0.20                        mov	ecx, r8d
 2      2     1.00                        shl	r10, cl
 1      1     0.20                        mov	rcx, -1
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	rcx, r10
 1      1     0.50                        cmove	rax, r10
 1      2     0.20                        and	rax, r9
 1      2     0.20                        and	rdx, rcx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   2.00    -      -      -     2.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r10, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, r8d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r10, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	rcx, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rcx, r10
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	rax, r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	rax, r9
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	rdx, rcx
```
## `floor_to_multiple_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      9
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    1.22
IPC:               1.22
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	r9, rcx
 1      1     0.20                        mov	r10, -1
 1      0     0.20                        mov	ecx, r8d
 2      2     1.00                        shl	r10, cl
 1      1     0.20                        mov	rcx, -1
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	rcx, r10
 1      1     0.50                        cmove	rax, r10
 1      2     0.20                        and	rax, r9
 1      2     0.20                        and	rdx, rcx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   2.00    -      -      -     2.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r10, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, r8d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r10, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	rcx, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rcx, r10
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	rax, r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	rax, r9
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	rdx, rcx
```
## `is_multiple_of_i8_pow2`
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
 1      3     1.00                        rep		bsf	eax, eax
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
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     rep		bsf	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	al, dl
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setae	al
```
## `is_multiple_of_i16_pow2`
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
 1      3     1.00                        rep		bsf	eax, ecx
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
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     rep		bsf	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     movzx	ecx, dl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	ax, cx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setae	al
```
## `is_multiple_of_i32_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      9
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.67
IPC:               0.56
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        mov	eax, 32
 1      3     1.00                        rep		bsf	eax, ecx
 1      0     0.20                        movzx	ecx, dl
 1      1     0.20                        cmp	eax, ecx
 2      2     1.00                        setae	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, 32
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     rep		bsf	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     movzx	ecx, dl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	eax, ecx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setae	al
```
## `is_multiple_of_i64_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      9
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.67
IPC:               0.56
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        mov	eax, 64
 1      3     1.00                        rep		bsf	rax, rcx
 1      0     0.20                        movzx	ecx, dl
 1      1     0.20                        cmp	eax, ecx
 2      2     1.00                        setae	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, 64
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     rep		bsf	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     movzx	ecx, dl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	eax, ecx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setae	al
```
## `is_multiple_of_i128_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      12
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    0.83
IPC:               0.75
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      3     1.00                        rep		bsf	rax, rcx
 1      1     0.20                        mov	r9d, 64
 1      3     1.00                        rep		bsf	r9, rdx
 1      1     0.20                        add	r9d, 64
 1      2     0.20                        test	rcx, rcx
 1      1     0.50                        cmovne	r9d, eax
 1      0     0.20                        movzx	eax, r8b
 1      1     0.20                        cmp	r9d, eax
 2      2     1.00                        setae	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   2.00    -      -      -     1.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     rep		bsf	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r9d, 64
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     rep		bsf	r9, rdx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     add	r9d, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     test	rcx, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r9d, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movzx	eax, r8b
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	r9d, eax
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setae	al
```
## `mod_floor_i8_pow2`
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
## `mod_floor_i8_unb_pow2`
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
## `mod_floor_i16_pow2`
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
 1      1     0.20                        mov	eax, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	eax, cl
 1      1     0.20                        not	eax
 1      2     0.20                        and	eax, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   1.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	eax, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	eax, r8d
```
## `mod_floor_i16_unb_pow2`
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
 1      2     0.20                        and	dl, 15
 1      1     0.20                        mov	eax, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	eax, cl
 1      1     0.20                        not	eax
 1      2     0.20                        and	eax, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   2.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 15
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	eax, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	eax, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	eax, r8d
```
## `mod_floor_i32_pow2`
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
 1      1     0.20                        mov	eax, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	eax, cl
 1      1     0.20                        not	eax
 1      2     0.20                        and	eax, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   1.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	eax, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	eax, r8d
```
## `mod_floor_i32_unb_pow2`
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
 1      1     0.20                        mov	eax, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	eax, cl
 1      1     0.20                        not	eax
 1      2     0.20                        and	eax, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   1.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	eax, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	eax, r8d
```
## `mod_floor_i64_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      9
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.67
IPC:               0.67
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	r8, rcx
 1      1     0.20                        mov	rax, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	rax, cl
 1      1     0.20                        not	rax
 1      2     0.20                        and	rax, r8

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	rax, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	rax, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	rax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	rax, r8
```
## `mod_floor_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      9
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.67
IPC:               0.67
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	r8, rcx
 1      1     0.20                        mov	rax, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	rax, cl
 1      1     0.20                        not	rax
 1      2     0.20                        and	rax, r8

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	rax, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	rax, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	rax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	rax, r8
```
## `mod_floor_i128_pow2`
```asm
Iterations:        1
Instructions:      13
Total Cycles:      10
Total uOps:        13

Dispatch Width:    6
uOps Per Cycle:    1.30
IPC:               1.30
Block RThroughput: 2.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	r9, rcx
 1      1     0.20                        mov	r10, -1
 1      1     0.20                        mov	r11, -1
 1      0     0.20                        mov	ecx, r8d
 2      2     1.00                        shl	r11, cl
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmove	rax, r11
 1      1     0.50                        cmove	r11, r10
 1      1     0.20                        not	r11
 1      1     0.20                        not	rax
 1      2     0.20                        and	rax, r9
 1      2     0.20                        and	rdx, r11

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   2.00    -      -      -     2.00   3.00    -      -      -     3.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r10, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r11, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, r8d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r11, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	rax, r11
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	r11, r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	r11
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	rax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	rax, r9
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	rdx, r11
```
## `mod_floor_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      13
Total Cycles:      10
Total uOps:        13

Dispatch Width:    6
uOps Per Cycle:    1.30
IPC:               1.30
Block RThroughput: 2.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	r9, rcx
 1      1     0.20                        mov	r10, -1
 1      1     0.20                        mov	r11, -1
 1      0     0.20                        mov	ecx, r8d
 2      2     1.00                        shl	r11, cl
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmove	rax, r11
 1      1     0.50                        cmove	r11, r10
 1      1     0.20                        not	r11
 1      1     0.20                        not	rax
 1      2     0.20                        and	rax, r9
 1      2     0.20                        and	rdx, r11

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   2.00    -      -      -     2.00   3.00    -      -      -     3.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r10, -1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r11, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, r8d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r11, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	rax, r11
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	r11, r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	r11
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	rax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	rax, r9
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	rdx, r11
```
## `mul_i8_pow2`
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
## `mul_i8_unb_pow2`
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
## `mul_i16_pow2`
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
 2      2     1.00                        shl	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00    -      -      -      -      -     1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	eax, cl
```
## `mul_i16_unb_pow2`
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
 1      2     0.20                        and	dl, 15
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00    -      -      -      -     1.00   1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	dl, 15
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	eax, cl
```
## `mul_i32_pow2`
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
 2      2     1.00                        shl	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00    -      -      -      -      -     1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	eax, cl
```
## `mul_i32_unb_pow2`
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
 2      2     1.00                        shl	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00    -      -      -      -      -     1.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	eax, cl
```
## `mul_i64_pow2`
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
 0      1     0.00                        mov	rax, rcx
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
 -      -      -      -      -      -     2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	rax, cl
```
## `mul_i64_unb_pow2`
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
 0      1     0.00                        mov	rax, rcx
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
 -      -      -      -      -      -     2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	rax, cl
```
## `mul_i128_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      10
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.80
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	r9, rcx
 1      0     0.20                        mov	ecx, r8d
 3      5     1.00                        shld	rdx, r9, cl
 2      2     1.00                        shl	r9, cl
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	rdx, r9
 1      1     0.50                        cmove	rax, r9

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   1.00    -      -      -     1.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, r8d
 -     1.00    -      -      -     1.00   1.00    -      -      -      -      -      -     shld	rdx, r9, cl
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r9, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rdx, r9
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	rax, r9
```
## `mul_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      10
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.80
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	r9, rcx
 1      0     0.20                        mov	ecx, r8d
 3      5     1.00                        shld	rdx, r9, cl
 2      2     1.00                        shl	r9, cl
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	rdx, r9
 1      1     0.50                        cmove	rax, r9

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   1.00    -      -      -     1.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, r8d
 -     1.00    -      -      -     1.00   1.00    -      -      -      -      -      -     shld	rdx, r9, cl
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r9, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rdx, r9
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	rax, r9
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
 1      1     0.50                        jae	.LBB179_1
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
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB179_1
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
Instructions:      19
Total Cycles:      14
Total uOps:        23

Dispatch Width:    6
uOps Per Cycle:    1.64
IPC:               1.36
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        cmp	dl, 16
 1      1     0.50                        jae	.LBB176_1
 1      1     0.20                        mov	r9d, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r9d, cl
 1      0     0.20                        mov	eax, r9d
 1      1     0.20                        not	eax
 1      1     0.20                        add	r8w, ax
 2      2     1.00                        setno	al
 1      2     0.20                        and	r8d, r9d
 1      0     0.20                        movzx	eax, al
 1      0     0.20                        mov	edx, r8d
 2      7     0.50                  U     ret
 1      2     0.20                        test	r8w, r8w
 2      2     1.00                        setle	al
 1      2     0.20                        xor	r8d, r8d
 1      0     0.20                        movzx	eax, al
 1      0     0.20                        mov	edx, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   4.00    -      -      -     4.00   7.00    -      -      -     4.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 16
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB176_1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r9d, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r9d, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, r9d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	r8w, ax
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setno	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	r8d, r9d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movzx	eax, al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
1.00    -      -      -      -      -      -      -      -      -      -      -      -     test	r8w, r8w
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setle	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	r8d, r8d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, r8d
```
## `unbounded_ceil_to_multiple_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      19
Total Cycles:      14
Total uOps:        23

Dispatch Width:    6
uOps Per Cycle:    1.64
IPC:               1.36
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        cmp	dl, 32
 1      1     0.50                        jae	.LBB177_1
 1      1     0.20                        mov	r9d, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r9d, cl
 1      0     0.20                        mov	eax, r9d
 1      1     0.20                        not	eax
 1      1     0.20                        add	r8d, eax
 2      2     1.00                        setno	al
 1      2     0.20                        and	r8d, r9d
 1      0     0.20                        movzx	eax, al
 1      0     0.20                        mov	edx, r8d
 2      7     0.50                  U     ret
 1      2     0.20                        test	r8d, r8d
 2      2     1.00                        setle	al
 1      2     0.20                        xor	r8d, r8d
 1      0     0.20                        movzx	eax, al
 1      0     0.20                        mov	edx, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   4.00    -      -      -     4.00   7.00    -      -      -     4.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 32
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB177_1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r9d, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r9d, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, r9d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	r8d, eax
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setno	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     and	r8d, r9d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movzx	eax, al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
1.00    -      -      -      -      -      -      -      -      -      -      -      -     test	r8d, r8d
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setle	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	r8d, r8d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, r8d
```
## `unbounded_ceil_to_multiple_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      19
Total Cycles:      15
Total uOps:        19

Dispatch Width:    6
uOps Per Cycle:    1.27
IPC:               1.27
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	r8, rcx
 1      1     0.20                        cmp	dl, 64
 1      1     0.50                        jae	.LBB178_1
 1      1     0.20                        mov	r9, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r9, cl
 0      1     0.00                        mov	rax, r9
 1      1     0.20                        not	rax
 1      1     0.20                        add	r8, rax
 2      2     1.00                        setno	al
 1      2     0.20                        and	r8, r9
 1      0     0.20                        movzx	eax, al
 0      1     0.00                        mov	rdx, r8
 2      7     0.50                  U     ret
 1      2     0.20                        test	r8, r8
 2      2     1.00                        setle	al
 1      2     0.20                        xor	r8d, r8d
 1      0     0.20                        movzx	eax, al
 0      1     0.00                        mov	rdx, r8

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   2.00    -      -      -     3.00   7.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB178_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r9, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r9, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, r9
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	rax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	r8, rax
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setno	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	r8, r9
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r8
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
1.00    -      -      -      -      -      -      -      -      -      -      -      -     test	r8, r8
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setle	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	r8d, r8d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movzx	eax, al
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r8
```
## `unbounded_ceil_to_multiple_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      38
Total Cycles:      28
Total uOps:        42

Dispatch Width:    6
uOps Per Cycle:    1.50
IPC:               1.36
Block RThroughput: 7.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	rsi
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB175_4
 1      1     0.20                        mov	r10, -1
 1      0     0.20                        mov	ecx, r9d
 2      2     1.00                        shl	r10, cl
 1      1     0.20                        mov	r11, -1
 1      2     0.20                        xor	ecx, ecx
 1      2     0.20                        test	r9b, 64
 0      1     0.00                        mov	r9, r10
 1      1     0.50                        cmovne	r9, rcx
 1      1     0.50                        cmove	r10, r11
 0      1     0.00                        mov	r11, r10
 1      1     0.20                        not	r11
 0      1     0.00                        mov	rsi, r9
 1      1     0.20                        not	rsi
 1      1     0.20                        add	rdx, rsi
 1      1     0.50                        adc	r8, r11
 1      1     0.50                        jo	.LBB175_5
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
 1      1     0.50                        jl	.LBB175_3
 2      12    0.50           *            mov	qword ptr [rax], rcx
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
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB175_4
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r10, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, r9d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r10, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r11, -1
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	ecx, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, r10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r9, rcx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	r10, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, r10
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rsi, r9
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	rsi
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	rdx, rsi
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     adc	r8, r11
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jo	.LBB175_5
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	rdx, r9
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	r8, r10
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rcx, rdx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 16], rcx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 24], r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, 1
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 8], 0
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rsi
1.00    -      -     1.00    -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	ecx, ecx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	rdx, 1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sbb	r8, 0
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8d, 0
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jl	.LBB175_3
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
 1      1     0.50                        jae	.LBB184_1
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
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB184_1
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
Total Cycles:      15
Total uOps:        22

Dispatch Width:    6
uOps Per Cycle:    1.47
IPC:               1.20
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      1     0.20                        cmp	dl, 16
 1      1     0.50                        jae	.LBB181_1
 1      1     0.20                        mov	r8d, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r8d, cl
 1      0     0.20                        mov	edx, r8d
 1      1     0.20                        not	edx
 1      1     0.20                        add	edx, eax
 1      1     0.20                        cmp	dx, ax
 2      2     1.00                        setae	al
 1      2     0.20                        and	edx, r8d
 1      0     0.20                        movzx	eax, al
 2      7     0.50                  U     ret
 1      2     0.20                        test	ax, ax
 2      2     1.00                        sete	al
 1      2     0.20                        xor	edx, edx
 1      0     0.20                        movzx	eax, al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   3.00    -      -      -     4.00   6.00    -      -      -     4.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 16
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB181_1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8d, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r8d, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	edx, r8d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	edx, eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dx, ax
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setae	al
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	edx, r8d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	ax, ax
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sete	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	edx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movzx	eax, al
```
## `unbounded_ceil_to_multiple_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      18
Total Cycles:      15
Total uOps:        22

Dispatch Width:    6
uOps Per Cycle:    1.47
IPC:               1.20
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      1     0.20                        cmp	dl, 32
 1      1     0.50                        jae	.LBB182_1
 1      1     0.20                        mov	r8d, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r8d, cl
 1      0     0.20                        mov	edx, r8d
 1      1     0.20                        not	edx
 1      1     0.20                        add	edx, eax
 1      1     0.20                        cmp	edx, eax
 2      2     1.00                        setae	al
 1      2     0.20                        and	edx, r8d
 1      0     0.20                        movzx	eax, al
 2      7     0.50                  U     ret
 1      2     0.20                        test	eax, eax
 2      2     1.00                        sete	al
 1      2     0.20                        xor	edx, edx
 1      0     0.20                        movzx	eax, al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   3.00    -      -      -     4.00   6.00    -      -      -     4.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 32
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB182_1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8d, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r8d, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	edx, r8d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	edx, eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	edx, eax
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setae	al
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     and	edx, r8d
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	eax, eax
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sete	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	edx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movzx	eax, al
```
## `unbounded_ceil_to_multiple_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      18
Total Cycles:      16
Total uOps:        20

Dispatch Width:    6
uOps Per Cycle:    1.25
IPC:               1.13
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      1     0.20                        cmp	dl, 64
 1      1     0.50                        jae	.LBB183_1
 1      1     0.20                        mov	r8, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r8, cl
 0      1     0.00                        mov	rdx, r8
 1      1     0.20                        not	rdx
 1      1     0.20                        add	rdx, rax
 1      1     0.20                        cmp	rdx, rax
 2      2     1.00                        setae	al
 1      2     0.20                        and	rdx, r8
 1      0     0.20                        movzx	eax, al
 2      7     0.50                  U     ret
 1      2     0.20                        test	rax, rax
 2      2     1.00                        sete	al
 1      2     0.20                        xor	edx, edx
 1      0     0.20                        movzx	eax, al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   3.00    -      -      -     3.00   7.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB183_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r8, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r8
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	rdx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	rdx, rax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	rdx, rax
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setae	al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	rdx, r8
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	eax, al
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	rax, rax
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sete	al
1.00    -      -      -      -      -      -      -      -      -      -      -      -     xor	edx, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
```
## `unbounded_ceil_to_multiple_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      37
Total Cycles:      28
Total uOps:        41

Dispatch Width:    6
uOps Per Cycle:    1.46
IPC:               1.32
Block RThroughput: 6.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	rsi
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB180_4
 1      1     0.20                        mov	r10, -1
 1      0     0.20                        mov	ecx, r9d
 2      2     1.00                        shl	r10, cl
 1      1     0.20                        mov	r11, -1
 1      2     0.20                        xor	ecx, ecx
 1      2     0.20                        test	r9b, 64
 0      1     0.00                        mov	r9, r10
 1      1     0.50                        cmovne	r9, rcx
 1      1     0.50                        cmove	r10, r11
 0      1     0.00                        mov	r11, r10
 1      1     0.20                        not	r11
 0      1     0.00                        mov	rsi, r9
 1      1     0.20                        not	rsi
 1      1     0.20                        add	rdx, rsi
 1      1     0.50                        adc	r8, r11
 1      1     0.50                        jb	.LBB180_5
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
 1      1     0.50                        je	.LBB180_3
 2      12    0.50           *            mov	qword ptr [rax], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   4.00   1.00   1.00   3.00   4.00   6.00   3.00   4.00   4.00   5.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, r9b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB180_4
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r10, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, r9d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r10, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	r11, -1
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	ecx, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, r10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r9, rcx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	r10, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, r10
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rsi, r9
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	rsi
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	rdx, rsi
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     adc	r8, r11
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jb	.LBB180_5
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	rdx, r9
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	r8, r10
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rcx, rdx
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 16], rcx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 24], r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, 1
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 8], 0
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rsi
1.00    -      -     1.00    -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	ecx, ecx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     or	rdx, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8d, 0
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     je	.LBB180_3
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax], rcx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 8], 0
 -      -     1.00    -      -      -      -      -      -      -      -      -      -     pop	rsi
```
## `unbounded_div_ceil_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      13
Total Cycles:      14
Total uOps:        17

Dispatch Width:    6
uOps Per Cycle:    1.21
IPC:               0.93
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      1     0.20                        cmp	dl, 8
 1      1     0.50                        jae	.LBB189_1
 1      1     0.20                        mov	r8, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r8, cl
 1      1     0.20                        not	r8
 1      1     0.33                        movsx	rax, al
 1      1     0.20                        add	rax, r8
 2      2     1.00                        sar	rax, cl
 2      7     0.50                  U     ret
 1      2     0.20                        test	al, al
 2      2     1.00                        setg	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   2.00    -      -      -     2.00   5.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 8
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB189_1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r8, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	r8
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movsx	rax, al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	rax, r8
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sar	rax, cl
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	al, al
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setg	al
```
## `unbounded_div_ceil_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      12
Total uOps:        18

Dispatch Width:    6
uOps Per Cycle:    1.50
IPC:               1.17
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        cmp	dl, 16
 1      1     0.50                        jae	.LBB186_1
 1      1     0.20                        mov	r9, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r9, cl
 1      1     0.20                        not	r9
 1      1     0.33                        movsx	rax, r8w
 1      1     0.20                        add	rax, r9
 2      2     1.00                        sar	rax, cl
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        test	r8w, r8w
 2      2     1.00                        setg	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   2.00    -      -      -     3.00   5.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 16
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB186_1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r9, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r9, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	r9
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movsx	rax, r8w
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	rax, r9
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r8w, r8w
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setg	al
```
## `unbounded_div_ceil_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      12
Total uOps:        18

Dispatch Width:    6
uOps Per Cycle:    1.50
IPC:               1.17
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        cmp	dl, 32
 1      1     0.50                        jae	.LBB187_1
 1      1     0.20                        mov	r9, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r9, cl
 1      1     0.20                        not	r9
 1      1     0.33                        movsxd	rax, r8d
 1      1     0.20                        add	rax, r9
 2      2     1.00                        sar	rax, cl
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        test	r8d, r8d
 2      2     1.00                        setg	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   2.00    -      -      -     3.00   5.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 32
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB187_1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r9, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r9, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	r9
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movsxd	rax, r8d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	rax, r9
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r8d, r8d
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setg	al
```
## `unbounded_div_ceil_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      16
Total Cycles:      15
Total uOps:        18

Dispatch Width:    6
uOps Per Cycle:    1.20
IPC:               1.07
Block RThroughput: 5.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 64
 1      1     0.50                        jae	.LBB188_1
 0      1     0.00                        mov	r8, rcx
 0      1     0.00                        mov	r9, rcx
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        sar	r8, cl
 0      1     0.00                        mov	r10, r8
 2      2     1.00                        shl	r10, cl
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	r9, r10
 2      2     1.00                        setne	al
 1      1     0.20                        add	rax, r8
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        test	rcx, rcx
 2      2     1.00                        setg	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   2.00    -      -      -     2.00   7.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB188_1
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sar	r8, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, r8
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r10, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	r9, r10
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setne	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	rax, r8
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	rcx, rcx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setg	al
```
## `unbounded_div_ceil_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      46
Total Cycles:      29
Total uOps:        49

Dispatch Width:    6
uOps Per Cycle:    1.69
IPC:               1.59
Block RThroughput: 9.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	rsi
 2      1     0.50           *            push	rdi
 1      2     0.20                        test	r8b, r8b
 1      1     0.50                        js	.LBB185_1
 1      0     0.20                        mov	eax, r8d
 0      1     0.00                        mov	r9, rcx
 0      1     0.00                        mov	r10, rcx
 1      0     0.20                        mov	ecx, r8d
 3      5     1.00                        shrd	r9, rdx, cl
 0      1     0.00                        mov	r11, rdx
 2      2     1.00                        sar	r11, cl
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	r9, r11
 0      1     0.00                        mov	rsi, r9
 2      2     1.00                        shl	rsi, cl
 1      2     0.20                        xor	edi, edi
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmove	rdi, rsi
 0      1     0.00                        mov	r8, rdx
 1      1     0.50                        sar	r8, 63
 1      2     0.20                        test	al, 64
 1      1     0.50                        cmove	r8, r11
 0      1     0.00                        mov	r11, r8
 3      5     1.00                        shld	r11, r9, cl
 1      2     0.20                        test	al, 64
 1      1     0.50                        cmovne	r11, rsi
 1      2     0.20                        xor	r11, rdx
 1      2     0.20                        xor	rdi, r10
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        or	rdi, r11
 2      2     1.00                        setne	al
 1      1     0.20                        add	rax, r9
 1      1     0.50                        adc	r8, 0
 0      1     0.00                        mov	rdx, r8
 1      5     0.33    *                   pop	rdi
 1      5     0.33    *                   pop	rsi
 2      7     0.50                  U     ret
 1      2     0.20                        xor	r8d, r8d
 1      1     0.20                        neg	rcx
 1      1     0.20                        mov	eax, 0
 1      1     0.50                        sbb	rax, rdx
 2      2     1.00                        setl	al
 1      0     0.20                        movzx	eax, al
 0      1     0.00                        mov	rdx, r8
 1      5     0.33    *                   pop	rdi
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
11.00  6.00   1.00   2.00   1.00   6.00   10.00  1.00   1.00   1.00   7.00   2.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rsi
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     push	rdi
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r8b, r8b
1.00    -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB185_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, r8d
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, rcx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, r8d
 -     1.00    -      -      -      -     1.00    -      -      -     1.00    -      -     shrd	r9, rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, rdx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	r11, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r8b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r9, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rsi, r9
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	rsi, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	edi, edi
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	rdi, rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sar	r8, 63
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     test	al, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	r8, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, r8
 -     1.00    -      -      -      -     1.00    -      -      -     1.00    -      -     shld	r11, r9, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	al, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r11, rsi
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	r11, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	rdi, r10
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     or	rdi, r11
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setne	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	rax, r9
1.00    -      -      -      -      -      -      -      -      -      -      -      -     adc	r8, 0
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r8
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rdi
 -      -     1.00    -      -      -      -      -      -      -      -      -      -     pop	rsi
 -      -      -     1.00    -      -     1.00    -      -      -      -      -      -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	r8d, r8d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     neg	rcx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	eax, 0
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sbb	rax, rdx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setl	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	eax, al
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r8
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rdi
 -      -      -     1.00    -      -      -      -      -      -      -      -      -     pop	rsi
```
## `unbounded_div_ceil_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      13
Total Cycles:      14
Total uOps:        17

Dispatch Width:    6
uOps Per Cycle:    1.21
IPC:               0.93
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      1     0.20                        cmp	dl, 8
 1      1     0.50                        jae	.LBB194_1
 1      1     0.20                        mov	r8, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r8, cl
 1      1     0.20                        not	r8
 1      0     0.20                        movzx	eax, al
 1      1     0.20                        add	rax, r8
 2      2     1.00                        shr	rax, cl
 2      7     0.50                  U     ret
 1      2     0.20                        test	al, al
 2      2     1.00                        setne	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   2.00    -      -      -     2.00   5.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 8
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB194_1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r8, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	r8
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	eax, al
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	rax, r8
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	rax, cl
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	al, al
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setne	al
```
## `unbounded_div_ceil_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      12
Total uOps:        18

Dispatch Width:    6
uOps Per Cycle:    1.50
IPC:               1.17
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        cmp	dl, 16
 1      1     0.50                        jae	.LBB191_1
 1      1     0.20                        mov	r9, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r9, cl
 1      1     0.20                        not	r9
 1      1     0.20                        movzx	eax, r8w
 1      1     0.20                        add	rax, r9
 2      2     1.00                        shr	rax, cl
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        test	r8w, r8w
 2      2     1.00                        setne	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   2.00    -      -      -     3.00   5.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 16
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB191_1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r9, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r9, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	r9
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	eax, r8w
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	rax, r9
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	rax, cl
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r8w, r8w
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setne	al
```
## `unbounded_div_ceil_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      12
Total uOps:        18

Dispatch Width:    6
uOps Per Cycle:    1.50
IPC:               1.17
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, ecx
 1      1     0.20                        cmp	dl, 32
 1      1     0.50                        jae	.LBB192_1
 1      1     0.20                        mov	r9, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r9, cl
 1      1     0.20                        not	r9
 1      0     0.20                        mov	eax, r8d
 1      1     0.20                        add	rax, r9
 2      2     1.00                        shr	rax, cl
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        test	r8d, r8d
 2      2     1.00                        setne	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   2.00    -      -      -     3.00   5.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 32
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB192_1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r9, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r9, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	r9
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, r8d
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	rax, r9
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	rax, cl
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r8d, r8d
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setne	al
```
## `unbounded_div_ceil_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      16
Total Cycles:      15
Total uOps:        18

Dispatch Width:    6
uOps Per Cycle:    1.20
IPC:               1.07
Block RThroughput: 5.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 64
 1      1     0.50                        jae	.LBB193_1
 0      1     0.00                        mov	r8, rcx
 0      1     0.00                        mov	r9, rcx
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shr	r8, cl
 0      1     0.00                        mov	r10, r8
 2      2     1.00                        shl	r10, cl
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	r9, r10
 2      2     1.00                        setne	al
 1      1     0.20                        add	rax, r8
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        test	rcx, rcx
 2      2     1.00                        setne	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   2.00    -      -      -     2.00   7.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB193_1
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	r8, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, r8
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r10, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	r9, r10
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setne	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	rax, r8
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	rcx, rcx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setne	al
```
## `unbounded_div_ceil_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      38
Total Cycles:      30
Total uOps:        44

Dispatch Width:    6
uOps Per Cycle:    1.47
IPC:               1.27
Block RThroughput: 8.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 2      1     0.50           *            push	rsi
 2      1     0.50           *            push	rdi
 1      2     0.20                        test	r8b, r8b
 1      1     0.50                        js	.LBB190_1
 0      1     0.00                        mov	r9, rcx
 0      1     0.00                        mov	r10, rcx
 1      0     0.20                        mov	ecx, r8d
 3      5     1.00                        shrd	r9, rdx, cl
 0      1     0.00                        mov	rax, rdx
 2      2     1.00                        shr	rdx, cl
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	r9, rdx
 0      1     0.00                        mov	r11, r9
 2      2     1.00                        shl	r11, cl
 1      2     0.20                        xor	esi, esi
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	rdx, rsi
 1      1     0.50                        cmove	rsi, r11
 0      1     0.00                        mov	rdi, rdx
 3      5     1.00                        shld	rdi, r9, cl
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	rdi, r11
 1      2     0.20                        xor	rdi, rax
 1      2     0.20                        xor	rsi, r10
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        or	rsi, rdi
 2      2     1.00                        setne	al
 1      1     0.20                        add	rax, r9
 1      1     0.50                        adc	rdx, 0
 1      5     0.33    *                   pop	rdi
 1      5     0.33    *                   pop	rsi
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        or	rcx, rdx
 2      2     1.00                        setne	al
 1      2     0.20                        xor	edx, edx
 1      5     0.33    *                   pop	rdi
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
10.00  5.00   1.00   2.00   1.00   6.00   8.00   1.00   1.00   1.00   6.00   2.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     push	rsi
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     push	rdi
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r8b, r8b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB190_1
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, r8d
1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -     shrd	r9, rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rdx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	rdx, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	r9, rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, r9
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r11, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	esi, esi
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rdx, rsi
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	rsi, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdi, rdx
1.00   1.00    -      -      -      -      -      -      -      -     1.00    -      -     shld	rdi, r9, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rdi, r11
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	rdi, rax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	rsi, r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     or	rsi, rdi
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setne	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	rax, r9
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     adc	rdx, 0
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rdi
 -      -     1.00    -      -      -      -      -      -      -      -      -      -     pop	rsi
1.00    -      -     1.00    -      -      -      -      -      -      -      -      -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     or	rcx, rdx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setne	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	edx, edx
 -      -      -      -      -      -      -      -      -      -      -     1.00    -     pop	rdi
 -      -      -     1.00    -      -      -      -      -      -      -      -      -     pop	rsi
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
Total Cycles:      7
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.86
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.33                        movsx	eax, cx
 1      0     0.20                        movzx	edx, dl
 1      1     0.20                        cmp	dl, 15
 1      1     0.20                        mov	ecx, 15
 1      1     0.50                        cmovb	ecx, edx
 2      2     1.00                        sar	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -     1.00   3.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movsx	eax, cx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	edx, dl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 15
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, 15
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovb	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sar	eax, cl
```
## `unbounded_div_floor_i32_unb_pow2`
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
 1      1     0.20                        cmp	dl, 31
 1      1     0.20                        mov	ecx, 31
 1      1     0.50                        cmovb	ecx, edx
 2      2     1.00                        sar	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00   1.00    -      -      -     1.00   3.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	edx, dl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	dl, 31
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, 31
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovb	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sar	eax, cl
```
## `unbounded_div_floor_i64_unb_pow2`
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
 0      1     0.00                        mov	rax, rcx
 1      0     0.20                        movzx	edx, dl
 1      1     0.20                        cmp	dl, 63
 1      1     0.20                        mov	ecx, 63
 1      1     0.50                        cmovb	ecx, edx
 2      2     1.00                        sar	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
1.00    -      -      -      -     1.00   3.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	edx, dl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 63
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, 63
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovb	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sar	rax, cl
```
## `unbounded_div_floor_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      12
Total Cycles:      12
Total uOps:        13

Dispatch Width:    6
uOps Per Cycle:    1.08
IPC:               1.00
Block RThroughput: 3.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r8b, r8b
 1      0     0.20                        movzx	r8d, r8b
 1      1     0.20                        mov	ecx, 127
 1      1     0.50                        cmovns	ecx, r8d
 3      5     1.00                        shrd	rax, rdx, cl
 0      1     0.00                        mov	r8, rdx
 2      2     1.00                        sar	r8, cl
 1      1     0.50                        sar	rdx, 63
 1      2     0.20                        test	cl, 64
 1      1     0.50                        cmovne	rax, r8
 1      1     0.50                        cmove	rdx, r8

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   1.00    -      -      -     2.00   5.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r8b, r8b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     movzx	r8d, r8b
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, 127
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovns	ecx, r8d
1.00   1.00    -      -      -      -      -      -      -      -     1.00    -      -     shrd	rax, rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rdx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     sar	r8, cl
1.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	rdx, 63
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	cl, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rax, r8
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	rdx, r8
```
## `unbounded_div_floor_u8_unb_pow2`
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
## `unbounded_div_floor_u16_unb_pow2`
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
 1      1     0.20                        movzx	r8d, cx
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shr	r8d, cl
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 16
 1      1     0.50                        cmovb	eax, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	r8d, cx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r8d, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 16
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovb	eax, r8d
```
## `unbounded_div_floor_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      6
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    1.17
IPC:               1.00
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, ecx
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shr	r8d, cl
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 32
 1      1     0.50                        cmovb	eax, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     1.00   2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r8d, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 32
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovb	eax, r8d
```
## `unbounded_div_floor_u64_unb_pow2`
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
 0      1     0.00                        mov	r8, rcx
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shr	r8, cl
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 64
 1      1     0.50                        cmovb	rax, r8

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00    -      -      -      -     1.00   2.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r8, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	dl, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovb	rax, r8
```
## `unbounded_div_floor_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      11
Total uOps:        13

Dispatch Width:    6
uOps Per Cycle:    1.18
IPC:               1.00
Block RThroughput: 3.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      0     0.20                        mov	ecx, r8d
 3      5     1.00                        shrd	rax, rdx, cl
 2      2     1.00                        shr	rdx, cl
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
3.00   1.00    -      -      -     2.00   5.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, r8d
1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -     shrd	rax, rdx, cl
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	rdx, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	ecx, ecx
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
Instructions:      16
Total Cycles:      13
Total uOps:        21

Dispatch Width:    6
uOps Per Cycle:    1.62
IPC:               1.23
Block RThroughput: 5.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 7
 1      1     0.50                        ja	.LBB209_1
 1      0     0.20                        mov	eax, ecx
 1      2     0.20                        test	cl, cl
 1      1     0.50                        js	.LBB209_4
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shr	al, cl
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax
 2      7     0.50                  U     ret
 1      1     0.20                        mov	r8b, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	r8b, cl
 1      1     0.20                        not	r8b
 1      1     0.20                        add	al, r8b
 2      2     1.00                        sar	al, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   2.00    -     1.00    -     3.00   6.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 7
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB209_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	cl, cl
1.00    -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB209_4
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	al, cl
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
1.00    -      -     1.00    -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8b, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r8b, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	r8b
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	al, r8b
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	al, cl
```
## `unbounded_div_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      17
Total Cycles:      14
Total uOps:        22

Dispatch Width:    6
uOps Per Cycle:    1.57
IPC:               1.21
Block RThroughput: 5.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 15
 1      1     0.50                        ja	.LBB206_1
 1      0     0.20                        mov	eax, ecx
 1      0     0.20                        movzx	ecx, dl
 1      2     0.20                        test	ax, ax
 1      1     0.50                        js	.LBB206_4
 1      1     0.20                        movzx	eax, ax
 2      2     1.00                        shr	eax, cl
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax
 2      7     0.50                  U     ret
 1      1     0.20                        mov	edx, -1
 2      2     1.00                        shl	edx, cl
 1      1     0.20                        not	edx
 1      1     0.20                        add	eax, edx
 1      1     0.50                        cwde
 2      2     1.00                        sar	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   3.00    -     1.00    -     3.00   6.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 15
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB206_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movzx	ecx, dl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	ax, ax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB206_4
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	eax, ax
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	eax, cl
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
1.00    -      -     1.00    -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, -1
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	edx, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	eax, edx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cwde
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	eax, cl
```
## `unbounded_div_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      16
Total Cycles:      12
Total uOps:        18

Dispatch Width:    6
uOps Per Cycle:    1.50
IPC:               1.33
Block RThroughput: 4.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 31
 1      1     0.50                        ja	.LBB207_4
 1      0     0.20                        mov	r8d, ecx
 1      2     0.20                        test	ecx, ecx
 1      1     0.50                        js	.LBB207_2
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shr	r8d, cl
 0      0     0.00                        jmp	.LBB207_3
 1      1     0.20                        mov	eax, -1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shl	eax, cl
 1      1     0.20                        not	eax
 1      1     0.20                        add	r8d, eax
 2      2     1.00                        sar	r8d, cl
 1      0     0.20                        mov	eax, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   3.00    -      -      -     3.00   5.00    -      -      -     3.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 31
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB207_4
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r8d, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	ecx, ecx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB207_2
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	r8d, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     jmp	.LBB207_3
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	eax, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	eax, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	r8d, eax
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	r8d, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	eax, r8d
```
## `unbounded_div_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      13
Total uOps:        19

Dispatch Width:    6
uOps Per Cycle:    1.46
IPC:               1.15
Block RThroughput: 5.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 63
 1      1     0.50                        ja	.LBB208_1
 0      1     0.00                        mov	rax, rcx
 1      0     0.20                        movzx	ecx, dl
 1      2     0.20                        test	rax, rax
 1      1     0.50                        js	.LBB208_4
 2      2     1.00                        shr	rax, cl
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax
 2      7     0.50                  U     ret
 1      1     0.20                        mov	rdx, -1
 2      2     1.00                        shl	rdx, cl
 1      1     0.20                        not	rdx
 1      1     0.20                        add	rax, rdx
 2      2     1.00                        sar	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   2.00    -     1.00    -     2.00   6.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 63
1.00    -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB208_1
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     movzx	ecx, dl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	rax, rax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB208_4
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	rax, cl
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
1.00    -      -     1.00    -      -      -      -      -      -      -      -      -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	rdx, -1
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	rdx, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	rdx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     add	rax, rdx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
```
## `unbounded_div_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      37
Total Cycles:      20
Total uOps:        44

Dispatch Width:    6
uOps Per Cycle:    2.20
IPC:               1.85
Block RThroughput: 10.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      2     0.20                        test	r8b, r8b
 1      1     0.50                        js	.LBB205_1
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	rdx, rdx
 1      1     0.50                        js	.LBB205_4
 1      0     0.20                        mov	ecx, r8d
 3      5     1.00                        shrd	rax, rdx, cl
 2      2     1.00                        shr	rdx, cl
 1      2     0.20                        xor	ecx, ecx
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	rax, rdx
 1      1     0.50                        cmovne	rdx, rcx
 2      7     0.50                  U     ret
 1      2     0.20                        xor	eax, eax
 1      2     0.20                        xor	edx, edx
 2      7     0.50                  U     ret
 1      1     0.20                        mov	r9, -1
 1      1     0.20                        mov	r10, -1
 1      0     0.20                        mov	ecx, r8d
 2      2     1.00                        shl	r10, cl
 1      2     0.20                        xor	ecx, ecx
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmove	rcx, r10
 1      1     0.50                        cmove	r10, r9
 1      1     0.20                        not	r10
 1      1     0.20                        not	rcx
 1      1     0.20                        add	rax, rcx
 1      1     0.50                        adc	rdx, r10
 1      0     0.20                        mov	ecx, r8d
 3      5     1.00                        shrd	rax, rdx, cl
 0      1     0.00                        mov	r9, rdx
 2      2     1.00                        sar	r9, cl
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmovne	rax, r9
 1      1     0.50                        sar	rdx, 63
 1      2     0.20                        test	r8b, 64
 1      1     0.50                        cmove	rdx, r9

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
10.00  7.00    -     1.00    -     7.00   11.00   -      -      -     7.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r8b, r8b
1.00    -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB205_1
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     test	rdx, rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB205_4
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, r8d
1.00   1.00    -      -      -      -      -      -      -      -     1.00    -      -     shrd	rax, rdx, cl
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	rdx, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	ecx, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rax, rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rdx, rcx
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	edx, edx
1.00    -      -     1.00    -      -      -      -      -      -      -      -      -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r9, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r10, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r10, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	ecx, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	rcx, r10
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	r10, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	r10
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     add	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     adc	rdx, r10
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, r8d
1.00   1.00    -      -      -     1.00    -      -      -      -      -      -      -     shrd	rax, rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rdx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sar	r9, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     test	r8b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rax, r9
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     sar	rdx, 63
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	rdx, r9
```
## `unbounded_floor_to_multiple_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      13
Total Cycles:      12
Total uOps:        17

Dispatch Width:    6
uOps Per Cycle:    1.42
IPC:               1.08
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
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 8
 1      0     0.20                        movzx	edx, r8b
 1      1     0.50                        cmovae	edx, eax
 2      2     1.00                        setb	al
 1      2     0.20                        or	al, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
6.00   2.00    -      -      -     2.00   4.00    -      -      -     3.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r8b, cl
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r8b, cl
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	al, al
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setns	cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	dl, 8
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movzx	edx, r8b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovae	edx, eax
2.00    -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     or	al, cl
```
## `unbounded_floor_to_multiple_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      13
Total Cycles:      12
Total uOps:        16

Dispatch Width:    6
uOps Per Cycle:    1.33
IPC:               1.08
Block RThroughput: 3.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        cmp	dl, 16
 1      1     0.50                        jae	.LBB211_1
 1      0     0.20                        mov	eax, edx
 1      1     0.20                        movzx	edx, cx
 1      0     0.20                        mov	ecx, eax
 2      2     1.00                        shr	edx, cl
 2      2     1.00                        shl	edx, cl
 1      1     0.20                        mov	ax, 1
 2      7     0.50                  U     ret
 1      1     0.20                        not	ecx
 1      1     0.20                        movzx	eax, cx
 1      1     0.50                        shr	eax, 15
 1      2     0.20                        xor	edx, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   2.00    -      -      -     2.00   5.00    -      -      -     3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 16
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB211_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movzx	edx, cx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, eax
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	edx, cl
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	edx, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ax, 1
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     not	ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     movzx	eax, cx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	eax, 15
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	edx, edx
```
## `unbounded_floor_to_multiple_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      12
Total Cycles:      12
Total uOps:        15

Dispatch Width:    6
uOps Per Cycle:    1.25
IPC:               1.00
Block RThroughput: 3.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	eax, ecx
 1      1     0.20                        cmp	dl, 32
 1      1     0.50                        jae	.LBB212_1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shr	eax, cl
 2      2     1.00                        shl	eax, cl
 1      0     0.20                        mov	edx, eax
 1      1     0.20                        mov	eax, 1
 2      7     0.50                  U     ret
 1      1     0.20                        not	eax
 1      1     0.50                        shr	eax, 31
 1      2     0.20                        xor	edx, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   2.00    -      -      -     2.00   4.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmp	dl, 32
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     jae	.LBB212_1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	eax, cl
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	eax, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	edx, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	eax, 1
 -      -      -      -      -      -     1.00    -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     not	eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     shr	eax, 31
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     xor	edx, edx
```
## `unbounded_floor_to_multiple_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      12
Total Cycles:      12
Total uOps:        13

Dispatch Width:    6
uOps Per Cycle:    1.08
IPC:               1.00
Block RThroughput: 3.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      1     0.20                        cmp	dl, 64
 1      1     0.50                        jae	.LBB213_1
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shr	rax, cl
 2      2     1.00                        shl	rax, cl
 0      1     0.00                        mov	rdx, rax
 1      1     0.20                        mov	eax, 1
 2      7     0.50                  U     ret
 1      1     0.20                        not	rax
 1      1     0.50                        shr	rax, 63
 1      2     0.20                        xor	edx, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   1.00    -      -      -     1.00   5.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	dl, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB213_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shr	rax, cl
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	rax, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, rax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	eax, 1
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     not	rax
1.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	rax, 63
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	edx, edx
```
## `unbounded_floor_to_multiple_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      26
Total Cycles:      24
Total uOps:        36

Dispatch Width:    6
uOps Per Cycle:    1.50
IPC:               1.08
Block RThroughput: 6.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        test	r9b, r9b
 1      1     0.50                        js	.LBB210_1
 1      1     0.20                        mov	r10, -1
 1      1     0.20                        mov	r11, -1
 1      0     0.20                        mov	ecx, r9d
 2      2     1.00                        shl	r11, cl
 1      2     0.20                        xor	ecx, ecx
 1      2     0.20                        test	r9b, 64
 1      1     0.50                        cmovne	r10, r11
 1      1     0.50                        cmove	rcx, r11
 1      2     0.20                        and	rcx, rdx
 1      2     0.20                        and	r10, r8
 2      12    0.50           *            mov	qword ptr [rax + 24], r10
 2      12    0.50           *            mov	qword ptr [rax + 16], rcx
 2      12    0.50           *            mov	qword ptr [rax + 8], 0
 2      12    0.50           *            mov	qword ptr [rax], 1
 2      7     0.50                  U     ret
 1      1     0.33                        xorps	xmm0, xmm0
 1      2     0.20                        test	r8, r8
 1      1     0.50                        js	.LBB210_2
 2      12    0.50           *            movups	xmmword ptr [rax + 8], xmm0
 2      12    0.50           *            mov	qword ptr [rax], 1
 2      12    0.50           *            mov	qword ptr [rax + 24], 0
 2      7     0.50                  U     ret
 2      12    0.50           *            movaps	xmmword ptr [rax], xmm0

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
5.00   2.00    -     1.00   4.00   3.00   5.00   4.00   4.00   4.00   3.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r9b, r9b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     js	.LBB210_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r10, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     mov	r11, -1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r11, cl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     xor	ecx, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     test	r9b, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r10, r11
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmove	rcx, r11
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	rcx, rdx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	r10, r8
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 24], r10
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax + 16], rcx
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 8], 0
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax], 1
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xorps	xmm0, xmm0
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	r8, r8
1.00    -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB210_2
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     movups	xmmword ptr [rax + 8], xmm0
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     mov	qword ptr [rax], 1
 -      -      -      -      -      -      -      -     1.00   1.00    -      -      -     mov	qword ptr [rax + 24], 0
 -      -      -     1.00    -      -     1.00    -      -      -      -      -      -     ret
 -      -      -      -     1.00    -      -     1.00    -      -      -      -      -     movaps	xmmword ptr [rax], xmm0
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
Instructions:      7
Total Cycles:      9
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               0.78
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.20                        movzx	r8d, cx
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shr	r8d, cl
 2      2     1.00                        shl	r8d, cl
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 16
 1      1     0.50                        cmovb	eax, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   1.00    -      -      -     1.00   3.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     movzx	r8d, cx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r8d, cl
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r8d, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 16
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovb	eax, r8d
```
## `unbounded_floor_to_multiple_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      8
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    1.13
IPC:               0.88
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.20                        mov	r8d, ecx
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shr	r8d, cl
 2      2     1.00                        shl	r8d, cl
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 32
 1      1     0.50                        cmovb	eax, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00   1.00    -      -      -     1.00   3.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r8d, cl
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r8d, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	eax, eax
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 32
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovb	eax, r8d
```
## `unbounded_floor_to_multiple_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      9
Total uOps:        8

Dispatch Width:    6
uOps Per Cycle:    0.89
IPC:               0.78
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	r8, rcx
 1      0     0.20                        mov	ecx, edx
 2      2     1.00                        shr	r8, cl
 2      2     1.00                        shl	r8, cl
 1      2     0.20                        xor	eax, eax
 1      1     0.20                        cmp	dl, 64
 1      1     0.50                        cmovb	rax, r8

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
3.00    -      -      -      -     1.00   3.00    -      -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, edx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shr	r8, cl
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     shl	r8, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	dl, 64
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovb	rax, r8
```
## `unbounded_floor_to_multiple_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      11
Total uOps:        14

Dispatch Width:    6
uOps Per Cycle:    1.27
IPC:               1.36
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 0      1     0.00                        mov	r9, rcx
 1      1     0.20                        mov	r10, -1
 1      0     0.20                        mov	ecx, r8d
 2      2     1.00                        shl	r10, cl
 1      2     0.20                        xor	ecx, ecx
 1      2     0.20                        test	r8b, 64
 0      1     0.00                        mov	rax, r10
 1      1     0.50                        cmovne	rax, rcx
 1      1     0.20                        mov	r11, -1
 1      1     0.50                        cmove	r10, r11
 1      2     0.20                        and	rdx, r10
 1      2     0.20                        and	rax, r9
 1      2     0.20                        test	r8b, r8b
 1      1     0.50                        cmovs	rax, rcx
 1      1     0.50                        cmovs	rdx, rcx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
4.00   2.00    -      -      -     2.00   4.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	r10, -1
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	ecx, r8d
2.00    -      -      -      -      -      -      -      -      -      -      -      -     shl	r10, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     xor	ecx, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, r10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovne	rax, rcx
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	r11, -1
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmove	r10, r11
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     and	rdx, r10
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     and	rax, r9
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     test	r8b, r8b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     cmovs	rax, rcx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovs	rdx, rcx
```
## `unbounded_is_multiple_of_i8_unb_pow2`
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
 1      3     1.00                        rep		bsf	eax, eax
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
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     rep		bsf	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	al, dl
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setae	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     or	al, r8b
```
## `unbounded_is_multiple_of_i16_unb_pow2`
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
 1      3     1.00                        rep		bsf	eax, ecx
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
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     rep		bsf	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     movzx	ecx, dl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     cmp	ax, cx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setae	al
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     or	al, r8b
```
## `unbounded_is_multiple_of_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      11
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    0.91
IPC:               0.73
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      2     0.20                        test	ecx, ecx
 2      2     1.00                        sete	r8b
 1      1     0.20                        mov	eax, 32
 1      3     1.00                        rep		bsf	eax, ecx
 1      0     0.20                        movzx	ecx, dl
 1      1     0.20                        cmp	eax, ecx
 2      2     1.00                        setae	al
 1      2     0.20                        or	al, r8b

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     2.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	ecx, ecx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sete	r8b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, 32
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     rep		bsf	eax, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movzx	ecx, dl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	eax, ecx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setae	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     or	al, r8b
```
## `unbounded_is_multiple_of_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      11
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    0.91
IPC:               0.73
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      2     0.20                        test	rcx, rcx
 2      2     1.00                        sete	r8b
 1      1     0.20                        mov	eax, 64
 1      3     1.00                        rep		bsf	rax, rcx
 1      0     0.20                        movzx	ecx, dl
 1      1     0.20                        cmp	eax, ecx
 2      2     1.00                        setae	al
 1      2     0.20                        or	al, r8b

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   1.00    -      -      -     2.00   3.00    -      -      -     2.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	rcx, rcx
2.00    -      -      -      -      -      -      -      -      -      -      -      -     sete	r8b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     mov	eax, 64
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     rep		bsf	rax, rcx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movzx	ecx, dl
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     cmp	eax, ecx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setae	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     or	al, r8b
```
## `unbounded_is_multiple_of_i128_unb_pow2`
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
 0      1     0.00                        mov	rax, rcx
 1      2     0.20                        or	rax, rdx
 1      1     0.50                        je	.LBB220_1
 1      3     1.00                        rep		bsf	rax, rcx
 1      3     1.00                        rep		bsf	rdx, rdx
 1      1     0.20                        add	edx, 64
 1      2     0.20                        test	rcx, rcx
 1      1     0.50                        cmovne	edx, eax
 1      0     0.20                        movzx	eax, r8b
 1      1     0.20                        cmp	edx, eax
 2      2     1.00                        setae	al
 2      7     0.50                  U     ret
 1      1     0.20                        mov	al, 1

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   
2.00   3.00    -      -      -     2.00   4.00    -      -      -     2.00   1.00    -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12]   Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     or	rax, rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -     je	.LBB220_1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     rep		bsf	rax, rcx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     rep		bsf	rdx, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     add	edx, 64
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     test	rcx, rcx
1.00    -      -      -      -      -      -      -      -      -      -      -      -     cmovne	edx, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -     movzx	eax, r8b
 -     1.00    -      -      -      -      -      -      -      -      -      -      -     cmp	edx, eax
 -      -      -      -      -      -     2.00    -      -      -      -      -      -     setae	al
1.00    -      -      -      -      -      -      -      -      -      -     1.00    -     ret
 -      -      -      -      -      -      -      -      -      -     1.00    -      -     mov	al, 1
```
