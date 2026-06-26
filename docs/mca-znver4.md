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
[0]   - Zn4AGU0
[1]   - Zn4AGU1
[2]   - Zn4AGU2
[3]   - Zn4ALU0
[4]   - Zn4ALU1
[5]   - Zn4ALU2
[6]   - Zn4ALU3
[7]   - Zn4BRU1
[8]   - Zn4FP0
[9]   - Zn4FP1
[10]  - Zn4FP2
[11]  - Zn4FP3
[12.0] - Zn4FP45
[12.1] - Zn4FP45
[13]  - Zn4FPSt
[14.0] - Zn4LSU
[14.1] - Zn4LSU
[14.2] - Zn4LSU
[15.0] - Zn4Load
[15.1] - Zn4Load
[15.2] - Zn4Load
[16.0] - Zn4Store
[16.1] - Zn4Store
```
# Functions:
## `ceil_to_multiple_i8_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      8
Total uOps:        8

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 1.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, ecx
 1      1     1.00                        mov	r9b, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r9b, cl
 1      0     0.17                        mov	eax, r9d
 1      1     0.25                        not	al
 1      1     0.25                        add	al, r8b
 1      1     0.25                        and	al, r9b

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   1.00   5.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9b, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r9b, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, r9d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	al
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	al, r8b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	al, r9b
```
## `ceil_to_multiple_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      8
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    1.13
IPC:               1.13
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, ecx
 1      1     0.25                        and	dl, 7
 1      1     1.00                        mov	r9b, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r9b, cl
 1      0     0.17                        mov	eax, r9d
 1      1     0.25                        not	al
 1      1     0.25                        add	al, r8b
 1      1     0.25                        and	al, r9b

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   5.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	dl, 7
 -      -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9b, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r9b, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, r9d
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	al
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	al, r8b
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	al, r9b
```
## `ceil_to_multiple_i16_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      8
Total uOps:        8

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 1.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, ecx
 1      1     0.50                        mov	r9d, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r9d, cl
 1      0     0.17                        mov	eax, r9d
 1      1     0.25                        not	eax
 1      1     0.25                        add	eax, r8d
 1      1     0.25                        and	eax, r9d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   1.00   3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9d, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r9d, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, r9d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	eax
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	eax, r8d
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	eax, r9d
```
## `ceil_to_multiple_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      8
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    1.13
IPC:               1.13
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, ecx
 1      1     0.25                        and	dl, 15
 1      1     0.50                        mov	r9d, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r9d, cl
 1      0     0.17                        mov	eax, r9d
 1      1     0.25                        not	eax
 1      1     0.25                        add	eax, r8d
 1      1     0.25                        and	eax, r9d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   3.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	dl, 15
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9d, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r9d, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, r9d
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	eax, r8d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	eax, r9d
```
## `ceil_to_multiple_i32_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      8
Total uOps:        8

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 1.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, ecx
 1      1     0.50                        mov	r9d, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r9d, cl
 1      0     0.17                        mov	eax, r9d
 1      1     0.25                        not	eax
 1      1     0.25                        add	eax, r8d
 1      1     0.25                        and	eax, r9d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   1.00   3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9d, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r9d, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, r9d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	eax
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	eax, r8d
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	eax, r9d
```
## `ceil_to_multiple_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      8
Total uOps:        8

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 1.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, ecx
 1      1     0.50                        mov	r9d, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r9d, cl
 1      0     0.17                        mov	eax, r9d
 1      1     0.25                        not	eax
 1      1     0.25                        add	eax, r8d
 1      1     0.25                        and	eax, r9d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   1.00   3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9d, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r9d, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, r9d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	eax
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	eax, r8d
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	eax, r9d
```
## `ceil_to_multiple_i64_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      8
Total uOps:        8

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 1.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8, rcx
 1      1     0.50                        mov	r9, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r9, cl
 1      0     0.17                        mov	rax, r9
 1      1     0.25                        not	rax
 1      1     0.25                        add	rax, r8
 1      1     0.25                        and	rax, r9

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   1.00   3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r9, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, r9
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	rax
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	rax, r9
```
## `ceil_to_multiple_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      8
Total uOps:        8

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 1.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8, rcx
 1      1     0.50                        mov	r9, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r9, cl
 1      0     0.17                        mov	rax, r9
 1      1     0.25                        not	rax
 1      1     0.25                        add	rax, r8
 1      1     0.25                        and	rax, r9

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   1.00   3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r9, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, r9
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	rax
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	rax, r9
```
## `ceil_to_multiple_i128_pow2`
```asm
Iterations:        1
Instructions:      17
Total Cycles:      10
Total uOps:        17

Dispatch Width:    6
uOps Per Cycle:    1.70
IPC:               1.70
Block RThroughput: 3.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r9, rcx
 1      1     0.50                        mov	r10, -1
 1      0     0.17                        mov	ecx, r8d
 1      1     0.50                        shl	r10, cl
 1      1     0.50                        mov	rax, -1
 1      0     0.17                        xor	ecx, ecx
 1      1     0.25                        test	r8b, 64
 1      1     0.50                        cmove	rcx, r10
 1      1     0.50                        cmove	r10, rax
 1      0     0.17                        mov	r8, r10
 1      1     0.25                        not	r8
 1      0     0.17                        mov	rax, rcx
 1      1     0.25                        not	rax
 1      1     0.25                        add	rax, r9
 1      1     1.00                        adc	rdx, r8
 1      1     0.25                        and	rax, rcx
 1      1     0.25                        and	rdx, r10

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     3.00   3.00   7.00   4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r10, cl
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	rcx, r10
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	r10, rax
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r8
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	rax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, r9
 -      -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     adc	rdx, r8
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	rax, rcx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	rdx, r10
```
## `ceil_to_multiple_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      17
Total Cycles:      10
Total uOps:        17

Dispatch Width:    6
uOps Per Cycle:    1.70
IPC:               1.70
Block RThroughput: 3.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r9, rcx
 1      1     0.50                        mov	r10, -1
 1      0     0.17                        mov	ecx, r8d
 1      1     0.50                        shl	r10, cl
 1      1     0.50                        mov	rax, -1
 1      0     0.17                        xor	ecx, ecx
 1      1     0.25                        test	r8b, 64
 1      1     0.50                        cmove	rcx, r10
 1      1     0.50                        cmove	r10, rax
 1      0     0.17                        mov	r8, r10
 1      1     0.25                        not	r8
 1      0     0.17                        mov	rax, rcx
 1      1     0.25                        not	rax
 1      1     0.25                        add	rax, r9
 1      1     1.00                        adc	rdx, r8
 1      1     0.25                        and	rax, rcx
 1      1     0.25                        and	rdx, r10

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     3.00   3.00   7.00   4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r10, cl
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	rcx, r10
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	r10, rax
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r8
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	rax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, r9
 -      -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     adc	rdx, r8
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	rax, rcx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	rdx, r10
```
## `checked_ceil_to_multiple_i8_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      8
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    1.13
IPC:               1.13
Block RThroughput: 1.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      1     1.00                        mov	r8b, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r8b, cl
 1      0     0.17                        mov	edx, r8d
 1      1     0.25                        not	dl
 1      1     0.25                        add	dl, al
 1      1     1.00                        setno	al
 1      1     0.25                        and	dl, r8b

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   2.00   6.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8b, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r8b, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	dl
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	dl, al
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setno	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	dl, r8b
```
## `checked_ceil_to_multiple_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      16
Total Cycles:      10
Total uOps:        16

Dispatch Width:    6
uOps Per Cycle:    1.60
IPC:               1.60
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        cmp	dl, 7
 1      1     0.50                        ja	.LBB24_1
 1      0     0.17                        mov	r8d, ecx
 1      1     1.00                        mov	al, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	al, cl
 1      0     0.17                        mov	ecx, eax
 1      1     0.25                        not	cl
 1      1     0.25                        add	r8b, cl
 1      1     0.50                        jno	.LBB24_3
 1      0     0.17                        xor	eax, eax
 1      0     0.17                        mov	edx, r8d
 1      5     0.50                  U     ret
 1      1     0.25                        and	r8b, al
 1      1     1.00                        mov	al, 1
 1      0     0.17                        mov	edx, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   2.00   2.00   5.00   5.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 7
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB24_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	al, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	al, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, eax
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	r8b, cl
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jno	.LBB24_3
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	r8b, al
 -      -      -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	al, 1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
```
## `checked_ceil_to_multiple_i16_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      8
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.25
IPC:               1.25
Block RThroughput: 1.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, ecx
 1      1     0.50                        mov	r9d, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r9d, cl
 1      0     0.17                        mov	edx, r9d
 1      1     0.25                        not	edx
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        add	dx, r8w
 1      1     1.00                        setno	al
 1      1     0.25                        and	edx, r9d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   2.00   4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9d, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r9d, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, r9d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	edx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	dx, r8w
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setno	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	edx, r9d
```
## `checked_ceil_to_multiple_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      10
Total uOps:        15

Dispatch Width:    6
uOps Per Cycle:    1.50
IPC:               1.50
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        cmp	dl, 15
 1      1     0.50                        ja	.LBB18_1
 1      0     0.17                        mov	r8d, edx
 1      0     0.17                        mov	eax, ecx
 1      1     0.50                        mov	edx, -1
 1      0     0.17                        mov	ecx, r8d
 1      1     0.50                        shl	edx, cl
 1      0     0.17                        mov	ecx, edx
 1      1     0.25                        not	ecx
 1      1     0.25                        add	ax, cx
 1      1     0.50                        jno	.LBB18_3
 1      0     0.17                        xor	eax, eax
 1      5     0.50                  U     ret
 1      1     0.25                        and	edx, eax
 1      1     1.00                        mov	ax, 1

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   2.00   2.00   6.00   2.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 15
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB18_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, edx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	edx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	ecx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	ax, cx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jno	.LBB18_3
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	edx, eax
 -      -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ax, 1
```
## `checked_ceil_to_multiple_i32_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      8
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.25
IPC:               1.25
Block RThroughput: 1.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, ecx
 1      1     0.50                        mov	r9d, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r9d, cl
 1      0     0.17                        mov	edx, r9d
 1      1     0.25                        not	edx
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        add	edx, r8d
 1      1     1.00                        setno	al
 1      1     0.25                        and	edx, r9d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   2.00   4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9d, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r9d, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, r9d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	edx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	edx, r8d
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setno	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	edx, r9d
```
## `checked_ceil_to_multiple_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      16
Total Cycles:      10
Total uOps:        16

Dispatch Width:    6
uOps Per Cycle:    1.60
IPC:               1.60
Block RThroughput: 2.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	dl, 31
 1      1     0.50                        ja	.LBB20_1
 1      0     0.17                        mov	r8d, ecx
 1      1     0.50                        mov	r9d, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r9d, cl
 1      0     0.17                        mov	ecx, r9d
 1      1     0.25                        not	ecx
 1      1     0.25                        add	r8d, ecx
 1      1     0.50                        jo	.LBB20_4
 1      1     0.25                        and	r8d, r9d
 1      1     0.50                        mov	eax, 1
 1      0     0.17                        mov	edx, r8d
 1      5     0.50                  U     ret
 1      0     0.17                        mov	edx, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   2.00   2.00   3.00   3.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 31
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB20_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9d, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r9d, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	r8d, ecx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jo	.LBB20_4
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	r8d, r9d
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, 1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
```
## `checked_ceil_to_multiple_i64_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      8
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.25
IPC:               1.25
Block RThroughput: 1.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8, rcx
 1      1     0.50                        mov	r9, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r9, cl
 1      0     0.17                        mov	rdx, r9
 1      1     0.25                        not	rdx
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        add	rdx, r8
 1      1     1.00                        setno	al
 1      1     0.25                        and	rdx, r9

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   2.00   4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r9, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r9
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rdx, r8
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setno	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	rdx, r9
```
## `checked_ceil_to_multiple_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      16
Total Cycles:      10
Total uOps:        16

Dispatch Width:    6
uOps Per Cycle:    1.60
IPC:               1.60
Block RThroughput: 2.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        cmp	dl, 63
 1      1     0.50                        ja	.LBB22_1
 1      0     0.17                        mov	r8, rcx
 1      1     0.50                        mov	rax, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	rax, cl
 1      0     0.17                        mov	rcx, rax
 1      1     0.25                        not	rcx
 1      1     0.25                        add	r8, rcx
 1      1     0.50                        jno	.LBB22_3
 1      0     0.17                        xor	eax, eax
 1      0     0.17                        mov	rdx, r8
 1      5     0.50                  U     ret
 1      1     0.25                        and	r8, rax
 1      1     0.50                        mov	eax, 1
 1      0     0.17                        mov	rdx, r8

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   2.00   2.00   3.00   3.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 63
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB22_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	rax, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rcx, rax
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	rcx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	r8, rcx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jno	.LBB22_3
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r8
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	r8, rax
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, 1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r8
```
## `checked_ceil_to_multiple_i128_pow2`
```asm
Iterations:        1
Instructions:      26
Total Cycles:      13
Total uOps:        26

Dispatch Width:    6
uOps Per Cycle:    2.00
IPC:               2.00
Block RThroughput: 5.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     1.00           *            push	rsi
 1      0     0.17                        mov	rax, rcx
 1      1     0.50                        mov	r10, -1
 1      0     0.17                        mov	ecx, r9d
 1      1     0.50                        shl	r10, cl
 1      1     0.50                        mov	r11, -1
 1      0     0.17                        xor	ecx, ecx
 1      1     0.25                        test	r9b, 64
 1      0     0.17                        mov	r9, r10
 1      1     0.50                        cmovne	r9, rcx
 1      1     0.50                        cmove	r10, r11
 1      0     0.17                        mov	r11, r10
 1      1     0.25                        not	r11
 1      0     0.17                        mov	rsi, r9
 1      1     0.25                        not	rsi
 1      1     0.25                        add	rsi, rdx
 1      1     1.00                        adc	r11, r8
 1      1     0.50                        jo	.LBB15_2
 1      1     0.25                        and	r11, r10
 1      1     0.25                        and	rsi, r9
 1      1     1.00           *            mov	qword ptr [rax + 16], rsi
 1      1     1.00           *            mov	qword ptr [rax + 24], r11
 1      1     0.50                        mov	ecx, 1
 1      1     1.00           *            mov	qword ptr [rax], rcx
 1      1     1.00           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
2.00   2.00   2.00   3.00   6.00   4.00   6.00   1.00    -      -      -      -      -      -      -     4.00   3.00   4.00    -      -     1.00   4.00   6.00   

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00   push	rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r10, cl
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, r10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r9, rcx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	r10, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r11
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rsi, r9
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	rsi
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rsi, rdx
 -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     adc	r11, r8
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jo	.LBB15_2
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	r11, r10
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	rsi, r9
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -     mov	qword ptr [rax + 16], rsi
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00   mov	qword ptr [rax + 24], r11
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, 1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -     mov	qword ptr [rax], rcx
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -      -     2.00   mov	qword ptr [rax + 8], 0
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -     pop	rsi
```
## `checked_ceil_to_multiple_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      33
Total Cycles:      17
Total uOps:        33

Dispatch Width:    6
uOps Per Cycle:    1.94
IPC:               1.94
Block RThroughput: 7.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     1.00           *            push	rsi
 1      0     0.17                        mov	rax, rcx
 1      1     0.25                        test	r9b, r9b
 1      1     0.50                        js	.LBB16_4
 1      1     0.50                        mov	r10, -1
 1      0     0.17                        mov	ecx, r9d
 1      1     0.50                        shl	r10, cl
 1      1     0.50                        mov	r11, -1
 1      0     0.17                        xor	ecx, ecx
 1      1     0.25                        test	r9b, 64
 1      0     0.17                        mov	r9, r10
 1      1     0.50                        cmovne	r9, rcx
 1      1     0.50                        cmove	r10, r11
 1      0     0.17                        mov	r11, r10
 1      1     0.25                        not	r11
 1      0     0.17                        mov	rsi, r9
 1      1     0.25                        not	rsi
 1      1     0.25                        add	rdx, rsi
 1      1     1.00                        adc	r8, r11
 1      1     0.50                        jo	.LBB16_3
 1      1     0.25                        and	r8, r10
 1      1     0.25                        and	rdx, r9
 1      1     1.00           *            mov	qword ptr [rax + 16], rdx
 1      1     1.00           *            mov	qword ptr [rax + 24], r8
 1      1     0.50                        mov	ecx, 1
 1      1     1.00           *            mov	qword ptr [rax], rcx
 1      1     1.00           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rsi
 1      5     0.50                  U     ret
 1      0     0.17                        xor	ecx, ecx
 1      1     1.00           *            mov	qword ptr [rax], rcx
 1      1     1.00           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
3.00   3.00   4.00   5.00   3.00   5.00   7.00   3.00    -      -      -      -      -      -      -     5.00   4.00   8.00   1.00   1.00   1.00   6.00   8.00   

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00   push	rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, r9b
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB16_4
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r10, cl
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, r10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r9, rcx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	r10, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, r10
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r11
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rsi, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	rsi
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rdx, rsi
 -      -      -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     adc	r8, r11
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jo	.LBB16_3
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	r8, r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	rdx, r9
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -     mov	qword ptr [rax + 16], rdx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00   mov	qword ptr [rax + 24], r8
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -     mov	qword ptr [rax], rcx
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00   mov	qword ptr [rax + 8], 0
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -     pop	rsi
1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -     mov	qword ptr [rax], rcx
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00   mov	qword ptr [rax + 8], 0
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -     1.00    -      -      -      -     pop	rsi
```
## `checked_ceil_to_multiple_u8_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      9
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.11
IPC:               1.11
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      1     1.00                        mov	r8b, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r8b, cl
 1      0     0.17                        mov	edx, r8d
 1      1     0.25                        not	dl
 1      1     0.25                        add	dl, al
 1      1     0.25                        cmp	dl, al
 1      1     1.00                        setae	al
 1      1     0.25                        and	dl, r8b

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     3.00   1.00   2.00   5.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8b, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r8b, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	dl
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	dl, al
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, al
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setae	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	dl, r8b
```
## `checked_ceil_to_multiple_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      10
Total uOps:        14

Dispatch Width:    6
uOps Per Cycle:    1.40
IPC:               1.40
Block RThroughput: 2.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        cmp	dl, 7
 1      1     0.50                        ja	.LBB34_1
 1      0     0.17                        mov	eax, ecx
 1      1     1.00                        mov	r8b, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r8b, cl
 1      0     0.17                        mov	edx, r8d
 1      1     0.25                        not	dl
 1      1     0.25                        add	dl, al
 1      1     0.25                        cmp	dl, al
 1      1     1.00                        setae	al
 1      1     0.25                        and	dl, r8b
 1      5     0.50                  U     ret
 1      0     0.17                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   3.00   2.00   5.00   2.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 7
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB34_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8b, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r8b, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	dl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	dl, al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, al
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setae	al
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	dl, r8b
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
```
## `checked_ceil_to_multiple_u16_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      9
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    1.22
IPC:               1.22
Block RThroughput: 1.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, ecx
 1      1     0.50                        mov	r9d, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r9d, cl
 1      0     0.17                        mov	edx, r9d
 1      1     0.25                        not	edx
 1      1     0.25                        add	edx, r8d
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	dx, r8w
 1      1     1.00                        setae	al
 1      1     0.25                        and	edx, r9d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     3.00   1.00   2.00   3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9d, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r9d, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, r9d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	edx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	edx, r8d
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dx, r8w
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setae	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	edx, r9d
```
## `checked_ceil_to_multiple_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      10
Total uOps:        15

Dispatch Width:    6
uOps Per Cycle:    1.50
IPC:               1.50
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        cmp	dl, 15
 1      1     0.50                        ja	.LBB28_1
 1      0     0.17                        mov	r8d, ecx
 1      1     0.50                        mov	r9d, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r9d, cl
 1      0     0.17                        mov	edx, r9d
 1      1     0.25                        not	edx
 1      1     0.25                        add	edx, r8d
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	dx, r8w
 1      1     1.00                        setae	al
 1      1     0.25                        and	edx, r9d
 1      5     0.50                  U     ret
 1      0     0.17                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   3.00   2.00   3.00   2.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 15
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB28_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9d, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r9d, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, r9d
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	edx, r8d
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dx, r8w
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setae	al
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	edx, r9d
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
```
## `checked_ceil_to_multiple_u32_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      9
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    1.22
IPC:               1.22
Block RThroughput: 1.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, ecx
 1      1     0.50                        mov	r9d, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r9d, cl
 1      0     0.17                        mov	edx, r9d
 1      1     0.25                        not	edx
 1      1     0.25                        add	edx, r8d
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	edx, r8d
 1      1     1.00                        setae	al
 1      1     0.25                        and	edx, r9d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     3.00   1.00   2.00   3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9d, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r9d, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, r9d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	edx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	edx, r8d
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	edx, r8d
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setae	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	edx, r9d
```
## `checked_ceil_to_multiple_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      10
Total uOps:        15

Dispatch Width:    6
uOps Per Cycle:    1.50
IPC:               1.50
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        cmp	dl, 31
 1      1     0.50                        ja	.LBB30_1
 1      0     0.17                        mov	r8d, ecx
 1      1     0.50                        mov	r9d, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r9d, cl
 1      0     0.17                        mov	edx, r9d
 1      1     0.25                        not	edx
 1      1     0.25                        add	edx, r8d
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	edx, r8d
 1      1     1.00                        setae	al
 1      1     0.25                        and	edx, r9d
 1      5     0.50                  U     ret
 1      0     0.17                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   3.00   2.00   3.00   2.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 31
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB30_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9d, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r9d, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, r9d
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	edx, r8d
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	edx, r8d
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setae	al
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	edx, r9d
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
```
## `checked_ceil_to_multiple_u64_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      9
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    1.22
IPC:               1.22
Block RThroughput: 1.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8, rcx
 1      1     0.50                        mov	r9, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r9, cl
 1      0     0.17                        mov	rdx, r9
 1      1     0.25                        not	rdx
 1      1     0.25                        add	rdx, r8
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	rdx, r8
 1      1     1.00                        setae	al
 1      1     0.25                        and	rdx, r9

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     3.00   1.00   2.00   3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r9, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r9
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	rdx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rdx, r8
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	rdx, r8
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setae	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	rdx, r9
```
## `checked_ceil_to_multiple_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      10
Total uOps:        15

Dispatch Width:    6
uOps Per Cycle:    1.50
IPC:               1.50
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        cmp	dl, 63
 1      1     0.50                        ja	.LBB32_1
 1      0     0.17                        mov	r8, rcx
 1      1     0.50                        mov	r9, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r9, cl
 1      0     0.17                        mov	rdx, r9
 1      1     0.25                        not	rdx
 1      1     0.25                        add	rdx, r8
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	rdx, r8
 1      1     1.00                        setae	al
 1      1     0.25                        and	rdx, r9
 1      5     0.50                  U     ret
 1      0     0.17                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   3.00   2.00   3.00   2.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 63
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB32_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r9, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r9
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rdx, r8
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	rdx, r8
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setae	al
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	rdx, r9
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
```
## `checked_ceil_to_multiple_u128_pow2`
```asm
Iterations:        1
Instructions:      26
Total Cycles:      13
Total uOps:        26

Dispatch Width:    6
uOps Per Cycle:    2.00
IPC:               2.00
Block RThroughput: 5.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     1.00           *            push	rsi
 1      0     0.17                        mov	rax, rcx
 1      1     0.50                        mov	r10, -1
 1      0     0.17                        mov	ecx, r9d
 1      1     0.50                        shl	r10, cl
 1      1     0.50                        mov	r11, -1
 1      0     0.17                        xor	ecx, ecx
 1      1     0.25                        test	r9b, 64
 1      0     0.17                        mov	r9, r10
 1      1     0.50                        cmovne	r9, rcx
 1      1     0.50                        cmove	r10, r11
 1      0     0.17                        mov	r11, r10
 1      1     0.25                        not	r11
 1      0     0.17                        mov	rsi, r9
 1      1     0.25                        not	rsi
 1      1     0.25                        add	rsi, rdx
 1      1     1.00                        adc	r11, r8
 1      1     0.50                        jb	.LBB25_2
 1      1     0.25                        and	r11, r10
 1      1     0.25                        and	rsi, r9
 1      1     1.00           *            mov	qword ptr [rax + 16], rsi
 1      1     1.00           *            mov	qword ptr [rax + 24], r11
 1      1     0.50                        mov	ecx, 1
 1      1     1.00           *            mov	qword ptr [rax], rcx
 1      1     1.00           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
2.00   2.00   2.00   3.00   6.00   4.00   6.00   1.00    -      -      -      -      -      -      -     4.00   3.00   4.00    -      -     1.00   4.00   6.00   

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00   push	rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r10, cl
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, r10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r9, rcx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	r10, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r11
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rsi, r9
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	rsi
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rsi, rdx
 -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     adc	r11, r8
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jb	.LBB25_2
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	r11, r10
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	rsi, r9
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -     mov	qword ptr [rax + 16], rsi
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00   mov	qword ptr [rax + 24], r11
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, 1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -     mov	qword ptr [rax], rcx
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -      -     2.00   mov	qword ptr [rax + 8], 0
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -     pop	rsi
```
## `checked_ceil_to_multiple_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      33
Total Cycles:      17
Total uOps:        33

Dispatch Width:    6
uOps Per Cycle:    1.94
IPC:               1.94
Block RThroughput: 7.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     1.00           *            push	rsi
 1      0     0.17                        mov	rax, rcx
 1      1     0.25                        test	r9b, r9b
 1      1     0.50                        js	.LBB26_4
 1      1     0.50                        mov	r10, -1
 1      0     0.17                        mov	ecx, r9d
 1      1     0.50                        shl	r10, cl
 1      1     0.50                        mov	r11, -1
 1      0     0.17                        xor	ecx, ecx
 1      1     0.25                        test	r9b, 64
 1      0     0.17                        mov	r9, r10
 1      1     0.50                        cmovne	r9, rcx
 1      1     0.50                        cmove	r10, r11
 1      0     0.17                        mov	r11, r10
 1      1     0.25                        not	r11
 1      0     0.17                        mov	rsi, r9
 1      1     0.25                        not	rsi
 1      1     0.25                        add	rdx, rsi
 1      1     1.00                        adc	r8, r11
 1      1     0.50                        jb	.LBB26_3
 1      1     0.25                        and	r8, r10
 1      1     0.25                        and	rdx, r9
 1      1     1.00           *            mov	qword ptr [rax + 16], rdx
 1      1     1.00           *            mov	qword ptr [rax + 24], r8
 1      1     0.50                        mov	ecx, 1
 1      1     1.00           *            mov	qword ptr [rax], rcx
 1      1     1.00           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rsi
 1      5     0.50                  U     ret
 1      0     0.17                        xor	ecx, ecx
 1      1     1.00           *            mov	qword ptr [rax], rcx
 1      1     1.00           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
3.00   3.00   4.00   5.00   3.00   5.00   7.00   3.00    -      -      -      -      -      -      -     5.00   4.00   8.00   1.00   1.00   1.00   6.00   8.00   

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00   push	rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, r9b
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB26_4
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r10, cl
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, r10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r9, rcx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	r10, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, r10
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r11
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rsi, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	rsi
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rdx, rsi
 -      -      -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     adc	r8, r11
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jb	.LBB26_3
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	r8, r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	rdx, r9
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -     mov	qword ptr [rax + 16], rdx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00   mov	qword ptr [rax + 24], r8
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -     mov	qword ptr [rax], rcx
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00   mov	qword ptr [rax + 8], 0
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -     pop	rsi
1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -     mov	qword ptr [rax], rcx
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00   mov	qword ptr [rax + 8], 0
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -     1.00    -      -      -      -     pop	rsi
```
## `checked_div_ceil_i8_unb_pow2`
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
 1      0     0.17                        mov	eax, edx
 1      1     0.25                        cmp	al, 7
 1      1     0.50                        ja	.LBB39_2
 1      0     0.17                        mov	r8d, ecx
 1      1     0.50                        mov	r9, -1
 1      0     0.17                        mov	ecx, eax
 1      1     0.50                        shl	r9, cl
 1      1     0.25                        not	r9
 1      1     0.25                        movsx	rdx, r8b
 1      1     0.25                        add	rdx, r9
 1      1     0.50                        sar	rdx, cl
 1      1     0.25                        cmp	al, 8
 1      1     1.00                        setb	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     2.00   2.00   3.00   4.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	al, 7
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB39_2
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, eax
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r9, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r9
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movsx	rdx, r8b
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rdx, r9
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	rdx, cl
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	al, 8
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setb	al
```
## `checked_div_ceil_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      10
Total uOps:        14

Dispatch Width:    6
uOps Per Cycle:    1.40
IPC:               1.40
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, edx
 1      1     0.25                        cmp	dl, 15
 1      1     0.50                        ja	.LBB36_1
 1      0     0.17                        mov	edx, ecx
 1      1     0.50                        mov	r8, -1
 1      0     0.17                        mov	ecx, eax
 1      1     0.50                        shl	r8, cl
 1      1     0.25                        not	r8
 1      1     0.25                        movsx	rdx, dx
 1      1     0.25                        add	rdx, r8
 1      1     0.50                        sar	rdx, cl
 1      1     1.00                        mov	ax, 1
 1      5     0.50                  U     ret
 1      0     0.17                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   1.00   2.00   4.00   5.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 15
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB36_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, ecx
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, eax
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r8, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r8
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movsx	rdx, dx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rdx, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	rdx, cl
 -      -      -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ax, 1
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
```
## `checked_div_ceil_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      10
Total uOps:        14

Dispatch Width:    6
uOps Per Cycle:    1.40
IPC:               1.40
Block RThroughput: 2.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, edx
 1      1     0.25                        cmp	dl, 31
 1      1     0.50                        ja	.LBB37_1
 1      0     0.17                        mov	edx, ecx
 1      1     0.50                        mov	r8, -1
 1      0     0.17                        mov	ecx, eax
 1      1     0.50                        shl	r8, cl
 1      1     0.25                        not	r8
 1      1     0.25                        movsxd	rdx, edx
 1      1     0.25                        add	rdx, r8
 1      1     0.50                        sar	rdx, cl
 1      1     0.50                        mov	eax, 1
 1      5     0.50                  U     ret
 1      0     0.17                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   1.00   2.00   4.00   3.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 31
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB37_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, ecx
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, eax
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r8, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r8
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movsxd	rdx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rdx, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	rdx, cl
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, 1
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
```
## `checked_div_ceil_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      10
Total uOps:        15

Dispatch Width:    6
uOps Per Cycle:    1.50
IPC:               1.50
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        cmp	dl, 63
 1      1     0.50                        ja	.LBB38_1
 1      0     0.17                        mov	rax, rcx
 1      0     0.17                        mov	r8, rcx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        sar	rax, cl
 1      0     0.17                        mov	r9, rax
 1      1     0.50                        shl	r9, cl
 1      0     0.17                        xor	edx, edx
 1      1     0.25                        cmp	r8, r9
 1      1     1.00                        setne	dl
 1      1     0.25                        add	rdx, rax
 1      1     0.50                        mov	eax, 1
 1      5     0.50                  U     ret
 1      0     0.17                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   3.00   1.00   2.00   3.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 63
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB38_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rax
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r9, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	edx, edx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	r8, r9
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setne	dl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rdx, rax
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, 1
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
```
## `checked_div_ceil_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      43
Total Cycles:      26
Total uOps:        51

Dispatch Width:    6
uOps Per Cycle:    1.96
IPC:               1.65
Block RThroughput: 8.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     1.00           *            push	rsi
 1      1     1.00           *            push	rdi
 1      1     1.00           *            push	rbx
 1      0     0.17                        mov	rax, rcx
 1      1     0.25                        test	r9b, r9b
 1      1     0.50                        js	.LBB35_1
 1      0     0.17                        mov	r10, rdx
 1      0     0.17                        mov	ecx, r9d
 5      2     1.50                        shrd	r10, r8, cl
 1      0     0.17                        mov	r11, r8
 1      1     0.50                        sar	r11, cl
 1      1     0.25                        test	r9b, 64
 1      1     0.50                        cmovne	r10, r11
 1      0     0.17                        mov	rsi, r10
 1      1     0.50                        shl	rsi, cl
 1      0     0.17                        xor	edi, edi
 1      1     0.25                        test	r9b, 64
 1      1     0.50                        cmove	rdi, rsi
 1      0     0.17                        mov	rbx, r8
 1      1     0.50                        sar	rbx, 63
 1      1     0.25                        test	r9b, 64
 1      1     0.50                        cmove	rbx, r11
 1      0     0.17                        mov	r11, rbx
 5      2     1.50                        shld	r11, r10, cl
 1      1     0.25                        test	r9b, 64
 1      1     0.50                        cmovne	r11, rsi
 1      1     0.25                        xor	r11, r8
 1      1     0.25                        xor	rdi, rdx
 1      0     0.17                        xor	ecx, ecx
 1      1     0.25                        or	rdi, r11
 1      1     1.00                        setne	cl
 1      1     0.25                        add	rcx, r10
 1      1     1.00                        adc	rbx, 0
 1      1     1.00           *            mov	qword ptr [rax + 16], rcx
 1      1     1.00           *            mov	qword ptr [rax + 24], rbx
 1      1     0.50                        mov	ecx, 1
 1      1     0.50                        jmp	.LBB35_3
 1      0     0.17                        xor	ecx, ecx
 1      1     1.00           *            mov	qword ptr [rax], rcx
 1      1     1.00           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rbx
 1      5     0.33    *                   pop	rdi
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
3.00   3.00   4.00   6.00   11.00  7.00   6.00   2.00    -      -      -      -      -      -      -     6.00   5.00   6.00   1.00   1.00   1.00   6.00   8.00   

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00   push	rsi
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -     push	rdi
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -      -     2.00   push	rbx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, r9b
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB35_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
 -      -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shrd	r10, r8, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, r8
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	r11, cl
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r10, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rsi, r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	rsi, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	edi, edi
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	rdi, rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rbx, r8
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	rbx, 63
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	rbx, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, rbx
 -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shld	r11, r10, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r11, rsi
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	r11, r8
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	rdi, rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     or	rdi, r11
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setne	cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rcx, r10
 -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     adc	rbx, 0
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -     mov	qword ptr [rax + 16], rcx
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00   mov	qword ptr [rax + 24], rbx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, 1
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jmp	.LBB35_3
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -     mov	qword ptr [rax], rcx
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -      -     2.00   mov	qword ptr [rax + 8], 0
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     pop	rbx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -     pop	rdi
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00   1.00    -      -      -      -     pop	rsi
```
## `checked_div_ceil_u8_unb_pow2`
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
 1      0     0.17                        mov	eax, edx
 1      1     0.25                        cmp	al, 7
 1      1     0.50                        ja	.LBB44_2
 1      0     0.17                        mov	r8d, ecx
 1      1     0.50                        mov	r9, -1
 1      0     0.17                        mov	ecx, eax
 1      1     0.50                        shl	r9, cl
 1      1     0.25                        not	r9
 1      1     0.25                        movzx	edx, r8b
 1      1     0.25                        add	rdx, r9
 1      1     0.50                        shr	rdx, cl
 1      1     0.25                        cmp	al, 8
 1      1     1.00                        setb	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     2.00   2.00   3.00   4.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	al, 7
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB44_2
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, eax
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r9, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r9
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	edx, r8b
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rdx, r9
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	rdx, cl
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	al, 8
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setb	al
```
## `checked_div_ceil_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      10
Total uOps:        14

Dispatch Width:    6
uOps Per Cycle:    1.40
IPC:               1.40
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, edx
 1      1     0.25                        cmp	dl, 15
 1      1     0.50                        ja	.LBB41_1
 1      0     0.17                        mov	edx, ecx
 1      1     0.50                        mov	r8, -1
 1      0     0.17                        mov	ecx, eax
 1      1     0.50                        shl	r8, cl
 1      1     0.25                        not	r8
 1      1     0.25                        movzx	edx, dx
 1      1     0.25                        add	rdx, r8
 1      1     0.50                        shr	rdx, cl
 1      1     1.00                        mov	ax, 1
 1      5     0.50                  U     ret
 1      0     0.17                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   1.00   2.00   4.00   5.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 15
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB41_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, ecx
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, eax
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r8, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r8
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	edx, dx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rdx, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	rdx, cl
 -      -      -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ax, 1
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
```
## `checked_div_ceil_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      10
Total uOps:        14

Dispatch Width:    6
uOps Per Cycle:    1.40
IPC:               1.40
Block RThroughput: 2.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, edx
 1      1     0.25                        cmp	dl, 31
 1      1     0.50                        ja	.LBB42_1
 1      0     0.17                        mov	edx, ecx
 1      1     0.50                        mov	r8, -1
 1      0     0.17                        mov	ecx, eax
 1      1     0.50                        shl	r8, cl
 1      1     0.25                        not	r8
 1      0     0.17                        mov	edx, edx
 1      1     0.25                        add	rdx, r8
 1      1     0.50                        shr	rdx, cl
 1      1     0.50                        mov	eax, 1
 1      5     0.50                  U     ret
 1      0     0.17                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   2.00   2.00   3.00   2.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 31
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB42_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, ecx
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, eax
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r8, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r8
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rdx, r8
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	rdx, cl
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, 1
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
```
## `checked_div_ceil_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      10
Total uOps:        15

Dispatch Width:    6
uOps Per Cycle:    1.50
IPC:               1.50
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        cmp	dl, 63
 1      1     0.50                        ja	.LBB43_1
 1      0     0.17                        mov	rax, rcx
 1      0     0.17                        mov	r8, rcx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	rax, cl
 1      0     0.17                        mov	r9, rax
 1      1     0.50                        shl	r9, cl
 1      0     0.17                        xor	edx, edx
 1      1     0.25                        cmp	r8, r9
 1      1     1.00                        setne	dl
 1      1     0.25                        add	rdx, rax
 1      1     0.50                        mov	eax, 1
 1      5     0.50                  U     ret
 1      0     0.17                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   3.00   1.00   2.00   3.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 63
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB43_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	rax, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rax
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r9, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	edx, edx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	r8, r9
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setne	dl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rdx, rax
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, 1
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
```
## `checked_div_ceil_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      40
Total Cycles:      26
Total uOps:        48

Dispatch Width:    6
uOps Per Cycle:    1.85
IPC:               1.54
Block RThroughput: 8.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     1.00           *            push	rsi
 1      1     1.00           *            push	rdi
 1      1     1.00           *            push	rbx
 1      0     0.17                        mov	rax, rcx
 1      1     0.25                        test	r9b, r9b
 1      1     0.50                        js	.LBB40_1
 1      0     0.17                        mov	r10, rdx
 1      0     0.17                        mov	ecx, r9d
 5      2     1.50                        shrd	r10, r8, cl
 1      0     0.17                        mov	r11, r8
 1      1     0.50                        shr	r11, cl
 1      1     0.25                        test	r9b, 64
 1      1     0.50                        cmovne	r10, r11
 1      0     0.17                        mov	rsi, r10
 1      1     0.50                        shl	rsi, cl
 1      0     0.17                        xor	edi, edi
 1      1     0.25                        test	r9b, 64
 1      1     0.50                        cmovne	r11, rdi
 1      1     0.50                        cmove	rdi, rsi
 1      0     0.17                        mov	rbx, r11
 5      2     1.50                        shld	rbx, r10, cl
 1      1     0.25                        test	r9b, 64
 1      1     0.50                        cmovne	rbx, rsi
 1      1     0.25                        xor	rbx, r8
 1      1     0.25                        xor	rdi, rdx
 1      0     0.17                        xor	ecx, ecx
 1      1     0.25                        or	rdi, rbx
 1      1     1.00                        setne	cl
 1      1     0.25                        add	rcx, r10
 1      1     1.00                        adc	r11, 0
 1      1     1.00           *            mov	qword ptr [rax + 16], rcx
 1      1     1.00           *            mov	qword ptr [rax + 24], r11
 1      1     0.50                        mov	ecx, 1
 1      1     0.50                        jmp	.LBB40_3
 1      0     0.17                        xor	ecx, ecx
 1      1     1.00           *            mov	qword ptr [rax], rcx
 1      1     1.00           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rbx
 1      5     0.33    *                   pop	rdi
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
3.00   3.00   4.00   6.00   8.00   8.00   6.00   2.00    -      -      -      -      -      -      -     5.00   6.00   6.00   1.00   1.00   1.00   6.00   8.00   

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00   push	rsi
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -     push	rdi
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -      -     2.00   push	rbx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, r9b
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB40_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
 -      -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shrd	r10, r8, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, r8
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	r11, cl
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r10, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rsi, r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	rsi, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	edi, edi
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r11, rdi
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	rdi, rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rbx, r11
 -      -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shld	rbx, r10, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rbx, rsi
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	rbx, r8
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	rdi, rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     or	rdi, rbx
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setne	cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rcx, r10
 -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     adc	r11, 0
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -     mov	qword ptr [rax + 16], rcx
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00   mov	qword ptr [rax + 24], r11
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, 1
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jmp	.LBB40_3
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -     mov	qword ptr [rax], rcx
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -      -     2.00   mov	qword ptr [rax + 8], 0
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     pop	rbx
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -     pop	rdi
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00   1.00    -      -      -      -     pop	rsi
```
## `checked_div_floor_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      5
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    1.20
IPC:               1.20
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, ecx
 1      1     0.25                        cmp	dl, 8
 1      1     1.00                        setb	al
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        sar	r8b, cl
 1      0     0.17                        mov	edx, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     2.00    -     1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 8
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	r8b, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
```
## `checked_div_floor_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      6
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    1.17
IPC:               1.17
Block RThroughput: 1.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, edx
 1      1     0.25                        movsx	edx, cx
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	r8b, 16
 1      1     1.00                        setb	al
 1      0     0.17                        mov	ecx, r8d
 1      1     0.50                        sar	edx, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     2.00   1.00   1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movsx	edx, cx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	r8b, 16
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	edx, cl
```
## `checked_div_floor_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      5
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    1.40
IPC:               1.40
Block RThroughput: 1.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, ecx
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	dl, 32
 1      1     1.00                        setb	al
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        sar	r8d, cl
 1      0     0.17                        mov	edx, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     2.00    -     1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 32
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	r8d, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
```
## `checked_div_floor_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      5
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    1.40
IPC:               1.40
Block RThroughput: 1.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8, rcx
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	dl, 64
 1      1     1.00                        setb	al
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        sar	r8, cl
 1      0     0.17                        mov	rdx, r8

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     2.00    -     1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 64
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	r8, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r8
```
## `checked_div_floor_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      20
Total Cycles:      13
Total uOps:        24

Dispatch Width:    6
uOps Per Cycle:    1.85
IPC:               1.54
Block RThroughput: 6.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	rax, rcx
 1      1     0.25                        test	r9b, r9b
 1      1     0.50                        js	.LBB45_1
 1      0     0.17                        mov	ecx, r9d
 5      2     1.50                        shrd	rdx, r8, cl
 1      0     0.17                        mov	r10, r8
 1      1     0.50                        sar	r10, cl
 1      1     0.50                        sar	r8, 63
 1      1     0.25                        test	r9b, 64
 1      1     0.50                        cmovne	rdx, r10
 1      1     0.50                        cmove	r8, r10
 1      1     1.00           *            mov	qword ptr [rax + 24], r8
 1      1     1.00           *            mov	qword ptr [rax + 16], rdx
 1      1     0.50                        mov	ecx, 1
 1      1     1.00           *            mov	qword ptr [rax], rcx
 1      1     1.00           *            mov	qword ptr [rax + 8], 0
 1      5     0.50                  U     ret
 1      0     0.17                        xor	ecx, ecx
 1      1     1.00           *            mov	qword ptr [rax], rcx
 1      1     1.00           *            mov	qword ptr [rax + 8], 0

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
2.00   2.00   3.00   3.00   2.00   3.00   3.00   2.00    -      -      -      -      -      -      -     4.00   4.00   5.00    -      -     1.00   6.00   6.00   

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, r9b
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB45_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
 -      -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shrd	rdx, r8, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, r8
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	r10, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	r8, 63
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rdx, r10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	r8, r10
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00   mov	qword ptr [rax + 24], r8
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -     mov	qword ptr [rax + 16], rdx
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, 1
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00   mov	qword ptr [rax], rcx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -     mov	qword ptr [rax + 8], 0
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -      -     2.00   mov	qword ptr [rax], rcx
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -     mov	qword ptr [rax + 8], 0
```
## `checked_div_floor_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      5
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    1.20
IPC:               1.20
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, ecx
 1      1     0.25                        cmp	dl, 8
 1      1     1.00                        setb	al
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	r8b, cl
 1      0     0.17                        mov	edx, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     2.00    -     1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 8
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	r8b, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
```
## `checked_div_floor_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      6
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    1.17
IPC:               1.17
Block RThroughput: 1.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, edx
 1      1     0.25                        movzx	edx, cx
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	r8b, 16
 1      1     1.00                        setb	al
 1      0     0.17                        mov	ecx, r8d
 1      1     0.50                        shr	edx, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     2.00   1.00   1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	edx, cx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	r8b, 16
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	edx, cl
```
## `checked_div_floor_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      5
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    1.40
IPC:               1.40
Block RThroughput: 1.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, ecx
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	dl, 32
 1      1     1.00                        setb	al
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	r8d, cl
 1      0     0.17                        mov	edx, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     2.00    -     1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 32
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	r8d, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
```
## `checked_div_floor_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      5
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    1.40
IPC:               1.40
Block RThroughput: 1.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8, rcx
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	dl, 64
 1      1     1.00                        setb	al
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	r8, cl
 1      0     0.17                        mov	rdx, r8

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     2.00    -     1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 64
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	r8, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r8
```
## `checked_div_floor_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      19
Total Cycles:      12
Total uOps:        23

Dispatch Width:    6
uOps Per Cycle:    1.92
IPC:               1.58
Block RThroughput: 6.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	rax, rcx
 1      1     0.25                        test	r9b, r9b
 1      1     0.50                        js	.LBB50_1
 1      0     0.17                        mov	ecx, r9d
 5      2     1.50                        shrd	rdx, r8, cl
 1      1     0.50                        shr	r8, cl
 1      0     0.17                        xor	ecx, ecx
 1      1     0.25                        test	r9b, 64
 1      1     0.50                        cmovne	rdx, r8
 1      1     0.50                        cmove	rcx, r8
 1      1     1.00           *            mov	qword ptr [rax + 24], rcx
 1      1     1.00           *            mov	qword ptr [rax + 16], rdx
 1      1     0.50                        mov	ecx, 1
 1      1     1.00           *            mov	qword ptr [rax], rcx
 1      1     1.00           *            mov	qword ptr [rax + 8], 0
 1      5     0.50                  U     ret
 1      0     0.17                        xor	ecx, ecx
 1      1     1.00           *            mov	qword ptr [rax], rcx
 1      1     1.00           *            mov	qword ptr [rax + 8], 0

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
2.00   2.00   3.00   2.00   3.00   3.00   2.00   2.00    -      -      -      -      -      -      -     4.00   4.00   5.00    -      -     1.00   6.00   6.00   

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, r9b
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB50_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
 -      -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shrd	rdx, r8, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	r8, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rdx, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	rcx, r8
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00   mov	qword ptr [rax + 24], rcx
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -     mov	qword ptr [rax + 16], rdx
 -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, 1
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00   mov	qword ptr [rax], rcx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -     mov	qword ptr [rax + 8], 0
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -      -     2.00   mov	qword ptr [rax], rcx
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -     mov	qword ptr [rax + 8], 0
```
## `checked_div_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      20
Total Cycles:      13
Total uOps:        20

Dispatch Width:    6
uOps Per Cycle:    1.54
IPC:               1.54
Block RThroughput: 3.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, ecx
 1      1     0.25                        cmp	dl, 7
 1      1     0.50                        ja	.LBB59_3
 1      1     0.25                        test	r8b, r8b
 1      1     0.50                        js	.LBB59_2
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	r8b, cl
 1      1     0.25                        cmp	dl, 8
 1      1     1.00                        setb	al
 1      0     0.17                        mov	edx, r8d
 1      5     0.50                  U     ret
 1      1     1.00                        mov	al, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	al, cl
 1      1     0.25                        not	al
 1      1     0.25                        add	r8b, al
 1      1     0.50                        sar	r8b, cl
 1      1     0.25                        cmp	dl, 8
 1      1     1.00                        setb	al
 1      0     0.17                        mov	edx, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   4.00   3.00   6.00   5.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 7
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB59_3
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, r8b
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB59_2
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	r8b, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 8
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	al, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	al, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	al
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	r8b, al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	r8b, cl
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 8
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
```
## `checked_div_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      19
Total Cycles:      11
Total uOps:        19

Dispatch Width:    6
uOps Per Cycle:    1.73
IPC:               1.73
Block RThroughput: 4.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        cmp	dl, 15
 1      1     0.50                        ja	.LBB56_1
 1      0     0.17                        mov	eax, ecx
 1      1     0.25                        movzx	ecx, dl
 1      1     0.25                        test	ax, ax
 1      1     0.50                        js	.LBB56_4
 1      1     0.25                        movzx	edx, ax
 1      1     0.50                        shr	edx, cl
 1      1     1.00                        mov	ax, 1
 1      5     0.50                  U     ret
 1      0     0.17                        xor	eax, eax
 1      5     0.50                  U     ret
 1      1     0.50                        mov	edx, -1
 1      1     0.50                        shl	edx, cl
 1      1     0.25                        not	edx
 1      1     0.25                        add	eax, edx
 1      1     0.25                        movsx	edx, ax
 1      1     0.50                        sar	edx, cl
 1      1     1.00                        mov	ax, 1

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -     1.00   1.00   3.00   5.00   7.00   6.00   3.00    -      -      -      -      -      -      -      -     1.00   1.00    -     1.00   1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 15
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB56_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	ecx, dl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	ax, ax
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB56_4
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	edx, ax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	edx, cl
 -      -      -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ax, 1
 -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -     ret
 -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	edx, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	edx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	eax, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movsx	edx, ax
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	edx, cl
 -      -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ax, 1
```
## `checked_div_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      21
Total Cycles:      11
Total uOps:        21

Dispatch Width:    6
uOps Per Cycle:    1.91
IPC:               1.91
Block RThroughput: 3.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        cmp	dl, 31
 1      1     0.50                        ja	.LBB57_1
 1      0     0.17                        mov	r8d, ecx
 1      1     0.25                        test	ecx, ecx
 1      1     0.50                        js	.LBB57_4
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	r8d, cl
 1      1     0.50                        mov	eax, 1
 1      0     0.17                        mov	edx, r8d
 1      5     0.50                  U     ret
 1      0     0.17                        xor	eax, eax
 1      0     0.17                        mov	edx, r8d
 1      5     0.50                  U     ret
 1      1     0.50                        mov	eax, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	eax, cl
 1      1     0.25                        not	eax
 1      1     0.25                        add	r8d, eax
 1      1     0.50                        sar	r8d, cl
 1      1     0.50                        mov	eax, 1
 1      0     0.17                        mov	edx, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -     1.00   1.00   2.00   3.00   5.00   5.00   2.00    -      -      -      -      -      -      -      -     1.00   1.00    -     1.00   1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 31
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB57_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	ecx, ecx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB57_4
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	r8d, cl
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, 1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
 -     1.00    -     1.00    -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -     ret
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	eax, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	eax
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	r8d, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	r8d, cl
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, 1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
```
## `checked_div_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      20
Total Cycles:      12
Total uOps:        20

Dispatch Width:    6
uOps Per Cycle:    1.67
IPC:               1.67
Block RThroughput: 3.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        cmp	dl, 63
 1      1     0.50                        ja	.LBB58_1
 1      0     0.17                        mov	r8, rcx
 1      1     0.25                        movzx	ecx, dl
 1      1     0.25                        test	r8, r8
 1      1     0.50                        js	.LBB58_4
 1      1     0.50                        shr	r8, cl
 1      1     0.50                        mov	eax, 1
 1      0     0.17                        mov	rdx, r8
 1      5     0.50                  U     ret
 1      0     0.17                        xor	eax, eax
 1      0     0.17                        mov	rdx, r8
 1      5     0.50                  U     ret
 1      1     0.50                        mov	rax, -1
 1      1     0.50                        shl	rax, cl
 1      1     0.25                        not	rax
 1      1     0.25                        add	r8, rax
 1      1     0.50                        sar	r8, cl
 1      1     0.50                        mov	eax, 1
 1      0     0.17                        mov	rdx, r8

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -     1.00   1.00   4.00   3.00   5.00   3.00   3.00    -      -      -      -      -      -      -      -     1.00   1.00    -     1.00   1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 63
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB58_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	ecx, dl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8, r8
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB58_4
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	r8, cl
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, 1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r8
 -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r8
 -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -     ret
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, -1
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	rax, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	rax
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	r8, rax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	r8, cl
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, 1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r8
```
## `checked_div_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      43
Total Cycles:      21
Total uOps:        51

Dispatch Width:    6
uOps Per Cycle:    2.43
IPC:               2.05
Block RThroughput: 8.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	rax, rcx
 1      1     0.25                        test	r9b, r9b
 1      1     0.50                        js	.LBB55_1
 1      1     0.25                        test	r8, r8
 1      1     0.50                        js	.LBB55_4
 1      0     0.17                        mov	ecx, r9d
 5      2     1.50                        shrd	rdx, r8, cl
 1      1     0.50                        shr	r8, cl
 1      0     0.17                        xor	ecx, ecx
 1      1     0.25                        test	r9b, 64
 1      1     0.50                        cmovne	rdx, r8
 1      1     0.50                        cmovne	r8, rcx
 1      1     0.50                        jmp	.LBB55_6
 1      0     0.17                        xor	ecx, ecx
 1      1     1.00           *            mov	qword ptr [rax], rcx
 1      1     1.00           *            mov	qword ptr [rax + 8], 0
 1      5     0.50                  U     ret
 1      1     0.50                        mov	r10, -1
 1      1     0.50                        mov	r11, -1
 1      0     0.17                        mov	ecx, r9d
 1      1     0.50                        shl	r11, cl
 1      0     0.17                        xor	ecx, ecx
 1      1     0.25                        test	r9b, 64
 1      1     0.50                        cmove	rcx, r11
 1      1     0.50                        cmove	r11, r10
 1      1     0.25                        not	r11
 1      1     0.25                        not	rcx
 1      1     0.25                        add	rdx, rcx
 1      1     1.00                        adc	r8, r11
 1      0     0.17                        mov	ecx, r9d
 5      2     1.50                        shrd	rdx, r8, cl
 1      0     0.17                        mov	r10, r8
 1      1     0.50                        sar	r10, cl
 1      1     0.25                        test	r9b, 64
 1      1     0.50                        cmovne	rdx, r10
 1      1     0.50                        sar	r8, 63
 1      1     0.25                        test	r9b, 64
 1      1     0.50                        cmove	r8, r10
 1      1     1.00           *            mov	qword ptr [rax + 16], rdx
 1      1     1.00           *            mov	qword ptr [rax + 24], r8
 1      1     0.50                        mov	ecx, 1
 1      1     1.00           *            mov	qword ptr [rax], rcx
 1      1     1.00           *            mov	qword ptr [rax + 8], 0

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
2.00   2.00   3.00   6.00   11.00  12.00  7.00   3.00    -      -      -      -      -      -      -     3.00   4.00   6.00    -      -     1.00   6.00   6.00   

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, r9b
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB55_1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8, r8
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB55_4
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
 -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shrd	rdx, r8, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	r8, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rdx, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r8, rcx
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jmp	.LBB55_6
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00   mov	qword ptr [rax], rcx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -     mov	qword ptr [rax + 8], 0
1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -     ret
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, -1
 -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r11, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	rcx, r11
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	r11, r10
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r11
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rdx, rcx
 -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     adc	r8, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
 -      -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shrd	rdx, r8, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, r8
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	r10, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rdx, r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	r8, 63
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	r8, r10
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00   mov	qword ptr [rax + 16], rdx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -     mov	qword ptr [rax + 24], r8
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -      -     2.00   mov	qword ptr [rax], rcx
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -     mov	qword ptr [rax + 8], 0
```
## `checked_floor_to_multiple_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      6
Total uOps:        8

Dispatch Width:    6
uOps Per Cycle:    1.33
IPC:               1.33
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r9d, edx
 1      0     0.17                        mov	r8d, ecx
 1      1     0.25                        cmp	dl, 8
 1      1     1.00                        setb	al
 1      1     1.00                        mov	dl, -1
 1      0     0.17                        mov	ecx, r9d
 1      1     0.50                        shl	dl, cl
 1      1     0.25                        and	dl, r8b

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     2.00   1.00   4.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9d, edx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 8
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	dl, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	dl, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	dl, r8b
```
## `checked_floor_to_multiple_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      6
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    1.50
IPC:               1.50
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r9d, edx
 1      0     0.17                        mov	r8d, ecx
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	dl, 16
 1      1     1.00                        setb	al
 1      1     0.50                        mov	edx, -1
 1      0     0.17                        mov	ecx, r9d
 1      1     0.50                        shl	edx, cl
 1      1     0.25                        and	edx, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     2.00   1.00   2.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9d, edx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 16
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	edx, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	edx, r8d
```
## `checked_floor_to_multiple_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      6
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    1.50
IPC:               1.50
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r9d, edx
 1      0     0.17                        mov	r8d, ecx
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	dl, 32
 1      1     1.00                        setb	al
 1      1     0.50                        mov	edx, -1
 1      0     0.17                        mov	ecx, r9d
 1      1     0.50                        shl	edx, cl
 1      1     0.25                        and	edx, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     2.00   1.00   2.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9d, edx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 32
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	edx, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	edx, r8d
```
## `checked_floor_to_multiple_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      6
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    1.50
IPC:               1.50
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r9d, edx
 1      0     0.17                        mov	r8, rcx
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	dl, 64
 1      1     1.00                        setb	al
 1      1     0.50                        mov	rdx, -1
 1      0     0.17                        mov	ecx, r9d
 1      1     0.50                        shl	rdx, cl
 1      1     0.25                        and	rdx, r8

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     2.00   1.00   2.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9d, edx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 64
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	rdx, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	rdx, r8
```
## `checked_floor_to_multiple_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      22
Total Cycles:      14
Total uOps:        22

Dispatch Width:    6
uOps Per Cycle:    1.57
IPC:               1.57
Block RThroughput: 6.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	rax, rcx
 1      1     0.25                        test	r9b, r9b
 1      1     0.50                        js	.LBB60_1
 1      1     0.50                        mov	r10, -1
 1      1     0.50                        mov	r11, -1
 1      0     0.17                        mov	ecx, r9d
 1      1     0.50                        shl	r11, cl
 1      0     0.17                        xor	ecx, ecx
 1      1     0.25                        test	r9b, 64
 1      1     0.50                        cmovne	r10, r11
 1      1     0.50                        cmove	rcx, r11
 1      1     0.25                        and	rcx, rdx
 1      1     0.25                        and	r10, r8
 1      1     1.00           *            mov	qword ptr [rax + 24], r10
 1      1     1.00           *            mov	qword ptr [rax + 16], rcx
 1      1     0.50                        mov	ecx, 1
 1      1     1.00           *            mov	qword ptr [rax], rcx
 1      1     1.00           *            mov	qword ptr [rax + 8], 0
 1      5     0.50                  U     ret
 1      0     0.17                        xor	ecx, ecx
 1      1     1.00           *            mov	qword ptr [rax], rcx
 1      1     1.00           *            mov	qword ptr [rax + 8], 0

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
2.00   2.00   3.00   2.00   3.00   4.00   4.00   2.00    -      -      -      -      -      -      -     4.00   4.00   5.00    -      -     1.00   6.00   6.00   

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, r9b
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB60_1
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, -1
 -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r11, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r10, r11
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	rcx, r11
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	rcx, rdx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	r10, r8
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00   mov	qword ptr [rax + 24], r10
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -     mov	qword ptr [rax + 16], rcx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, 1
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00   mov	qword ptr [rax], rcx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -     mov	qword ptr [rax + 8], 0
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -      -     2.00   mov	qword ptr [rax], rcx
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -     mov	qword ptr [rax + 8], 0
```
## `checked_mod_floor_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      7
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    1.29
IPC:               1.29
Block RThroughput: 1.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r9d, edx
 1      0     0.17                        mov	r8d, ecx
 1      1     0.25                        cmp	dl, 8
 1      1     1.00                        setb	al
 1      1     1.00                        mov	dl, -1
 1      0     0.17                        mov	ecx, r9d
 1      1     0.50                        shl	dl, cl
 1      1     0.25                        not	dl
 1      1     0.25                        and	dl, r8b

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     2.00   2.00   4.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9d, edx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 8
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	dl, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	dl, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	dl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	dl, r8b
```
## `checked_mod_floor_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      7
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.43
IPC:               1.43
Block RThroughput: 1.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r9d, edx
 1      0     0.17                        mov	r8d, ecx
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	dl, 16
 1      1     1.00                        setb	al
 1      1     0.50                        mov	edx, -1
 1      0     0.17                        mov	ecx, r9d
 1      1     0.50                        shl	edx, cl
 1      1     0.25                        not	edx
 1      1     0.25                        and	edx, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     2.00   1.00   3.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9d, edx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 16
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	edx, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	edx, r8d
```
## `checked_mod_floor_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      7
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.43
IPC:               1.43
Block RThroughput: 1.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r9d, edx
 1      0     0.17                        mov	r8d, ecx
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	dl, 32
 1      1     1.00                        setb	al
 1      1     0.50                        mov	edx, -1
 1      0     0.17                        mov	ecx, r9d
 1      1     0.50                        shl	edx, cl
 1      1     0.25                        not	edx
 1      1     0.25                        and	edx, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     2.00   1.00   3.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9d, edx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 32
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	edx, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	edx, r8d
```
## `checked_mod_floor_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      7
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.43
IPC:               1.43
Block RThroughput: 1.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r9d, edx
 1      0     0.17                        mov	r8, rcx
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	dl, 64
 1      1     1.00                        setb	al
 1      1     0.50                        mov	rdx, -1
 1      0     0.17                        mov	ecx, r9d
 1      1     0.50                        shl	rdx, cl
 1      1     0.25                        not	rdx
 1      1     0.25                        and	rdx, r8

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     2.00   1.00   3.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9d, edx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 64
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	rdx, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	rdx, r8
```
## `checked_mod_floor_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      24
Total Cycles:      14
Total uOps:        24

Dispatch Width:    6
uOps Per Cycle:    1.71
IPC:               1.71
Block RThroughput: 6.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	rax, rcx
 1      1     0.25                        test	r9b, r9b
 1      1     0.50                        js	.LBB65_1
 1      1     0.50                        mov	r10, -1
 1      0     0.17                        mov	ecx, r9d
 1      1     0.50                        shl	r10, cl
 1      1     0.50                        mov	rcx, -1
 1      0     0.17                        xor	r11d, r11d
 1      1     0.25                        test	r9b, 64
 1      1     0.50                        cmove	r11, r10
 1      1     0.50                        cmove	r10, rcx
 1      1     0.25                        not	r10
 1      1     0.25                        not	r11
 1      1     0.25                        and	rdx, r11
 1      1     0.25                        and	r8, r10
 1      1     1.00           *            mov	qword ptr [rax + 24], r8
 1      1     1.00           *            mov	qword ptr [rax + 16], rdx
 1      1     0.50                        mov	ecx, 1
 1      1     1.00           *            mov	qword ptr [rax], rcx
 1      1     1.00           *            mov	qword ptr [rax + 8], 0
 1      5     0.50                  U     ret
 1      0     0.17                        xor	ecx, ecx
 1      1     1.00           *            mov	qword ptr [rax], rcx
 1      1     1.00           *            mov	qword ptr [rax + 8], 0

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
2.00   2.00   3.00   4.00   3.00   5.00   3.00   2.00    -      -      -      -      -      -      -     4.00   4.00   5.00    -      -     1.00   6.00   6.00   

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, r9b
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB65_1
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r10, cl
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rcx, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	r11d, r11d
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	r11, r10
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	r10, rcx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r10
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r11
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	rdx, r11
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	r8, r10
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00   mov	qword ptr [rax + 24], r8
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -     mov	qword ptr [rax + 16], rdx
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, 1
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00   mov	qword ptr [rax], rcx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -     mov	qword ptr [rax + 8], 0
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -      -     2.00   mov	qword ptr [rax], rcx
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -     mov	qword ptr [rax + 8], 0
```
## `checked_mul_i8_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      7
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    1.29
IPC:               1.29
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, edx
 1      0     0.17                        mov	eax, ecx
 1      0     0.17                        mov	edx, ecx
 1      0     0.17                        mov	ecx, r8d
 1      1     0.50                        shl	dl, cl
 1      0     0.17                        mov	r9d, edx
 1      1     0.50                        sar	r9b, cl
 1      1     0.25                        cmp	al, r9b
 1      1     1.00                        sete	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     2.00   1.00   1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, edx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	dl, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9d, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	r9b, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	al, r9b
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sete	al
```
## `checked_mul_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      10
Total uOps:        14

Dispatch Width:    6
uOps Per Cycle:    1.40
IPC:               1.40
Block RThroughput: 2.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, edx
 1      0     0.17                        mov	eax, ecx
 1      1     0.25                        cmp	dl, 8
 1      1     1.00                        setb	r9b
 1      0     0.17                        mov	edx, ecx
 1      0     0.17                        mov	ecx, r8d
 1      1     0.50                        shl	dl, cl
 1      1     0.25                        and	r8b, 7
 1      0     0.17                        mov	r10d, edx
 1      0     0.17                        mov	ecx, r8d
 1      1     0.50                        sar	r10b, cl
 1      1     0.25                        cmp	al, r10b
 1      1     1.00                        sete	al
 1      1     0.25                        and	al, r9b

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     4.00   2.00   2.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, edx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 8
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setb	r9b
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	dl, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	r8b, 7
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10d, edx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	r10b, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	al, r10b
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sete	al
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	al, r9b
```
## `checked_mul_i16_pow2`
```asm
Iterations:        1
Instructions:      12
Total Cycles:      8
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.50
IPC:               1.50
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, edx
 1      0     0.17                        mov	r9d, ecx
 1      0     0.17                        mov	eax, r9d
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	eax, cl
 1      1     0.25                        movsx	edx, ax
 1      0     0.17                        mov	r10d, edx
 1      0     0.17                        mov	ecx, r8d
 1      1     0.50                        sar	r10d, cl
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	r9w, r10w
 1      1     1.00                        sete	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   1.00   3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, edx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9d, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, r9d
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	eax, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movsx	edx, ax
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10d, edx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	r10d, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	r9w, r10w
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sete	al
```
## `checked_mul_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      10
Total uOps:        14

Dispatch Width:    6
uOps Per Cycle:    1.40
IPC:               1.40
Block RThroughput: 2.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        cmp	dl, 15
 1      1     0.50                        ja	.LBB73_1
 1      0     0.17                        mov	eax, edx
 1      0     0.17                        mov	edx, ecx
 1      0     0.17                        mov	r8d, ecx
 1      0     0.17                        mov	ecx, eax
 1      1     0.50                        shl	edx, cl
 1      1     0.25                        movsx	r9d, dx
 1      1     0.50                        sar	r9d, cl
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	r8w, r9w
 1      1     1.00                        sete	al
 1      5     0.50                  U     ret
 1      0     0.17                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   3.00   1.00   2.00   2.00   1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 15
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB73_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, edx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	edx, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movsx	r9d, dx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	r9d, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	r8w, r9w
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sete	al
 -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
```
## `checked_mul_i32_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      7
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.43
IPC:               1.43
Block RThroughput: 1.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, edx
 1      0     0.17                        mov	r9d, ecx
 1      0     0.17                        mov	edx, ecx
 1      0     0.17                        mov	ecx, r8d
 1      1     0.50                        shl	edx, cl
 1      0     0.17                        mov	r10d, edx
 1      1     0.50                        sar	r10d, cl
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	r9d, r10d
 1      1     1.00                        sete	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     2.00   1.00   1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, edx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9d, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	edx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10d, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	r10d, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	r9d, r10d
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sete	al
```
## `checked_mul_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      10
Total uOps:        14

Dispatch Width:    6
uOps Per Cycle:    1.40
IPC:               1.40
Block RThroughput: 2.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        cmp	dl, 31
 1      1     0.50                        ja	.LBB75_1
 1      0     0.17                        mov	eax, edx
 1      0     0.17                        mov	edx, ecx
 1      0     0.17                        mov	r8d, ecx
 1      0     0.17                        mov	ecx, eax
 1      1     0.50                        shl	edx, cl
 1      0     0.17                        mov	r9d, edx
 1      1     0.50                        sar	r9d, cl
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	r8d, r9d
 1      1     1.00                        sete	al
 1      5     0.50                  U     ret
 1      0     0.17                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   3.00   1.00   1.00   2.00   1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 31
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB75_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, edx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	edx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9d, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	r9d, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	r8d, r9d
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sete	al
 -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
```
## `checked_mul_i64_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      7
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.43
IPC:               1.43
Block RThroughput: 1.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, edx
 1      0     0.17                        mov	r9, rcx
 1      0     0.17                        mov	rdx, rcx
 1      0     0.17                        mov	ecx, r8d
 1      1     0.50                        shl	rdx, cl
 1      0     0.17                        mov	r10, rdx
 1      1     0.50                        sar	r10, cl
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	r9, r10
 1      1     1.00                        sete	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     2.00   1.00   1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, edx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, rdx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	r10, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	r9, r10
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sete	al
```
## `checked_mul_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      10
Total uOps:        14

Dispatch Width:    6
uOps Per Cycle:    1.40
IPC:               1.40
Block RThroughput: 2.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        cmp	dl, 63
 1      1     0.50                        ja	.LBB77_1
 1      0     0.17                        mov	eax, edx
 1      0     0.17                        mov	rdx, rcx
 1      0     0.17                        mov	r8, rcx
 1      0     0.17                        mov	ecx, eax
 1      1     0.50                        shl	rdx, cl
 1      0     0.17                        mov	r9, rdx
 1      1     0.50                        sar	r9, cl
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	r8, r9
 1      1     1.00                        sete	al
 1      5     0.50                  U     ret
 1      0     0.17                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   3.00   1.00   1.00   2.00   1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 63
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB77_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, edx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rdx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	r9, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	r8, r9
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sete	al
 -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
```
## `checked_mul_i128_pow2`
```asm
Iterations:        1
Instructions:      37
Total Cycles:      29
Total uOps:        45

Dispatch Width:    6
uOps Per Cycle:    1.55
IPC:               1.28
Block RThroughput: 8.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     1.00           *            push	r14
 1      1     1.00           *            push	rsi
 1      1     1.00           *            push	rdi
 1      1     1.00           *            push	rbx
 1      0     0.17                        mov	rax, rcx
 1      0     0.17                        mov	r10, r8
 1      0     0.17                        mov	ecx, r9d
 5      2     1.50                        shld	r10, rdx, cl
 1      0     0.17                        mov	r11, rdx
 1      1     0.50                        shl	r11, cl
 1      1     0.25                        test	r9b, 64
 1      1     0.50                        cmovne	r10, r11
 1      0     0.17                        mov	rdi, r10
 1      1     0.50                        sar	rdi, cl
 1      0     0.17                        mov	rbx, r10
 1      1     0.50                        sar	rbx, 63
 1      0     0.17                        xor	esi, esi
 1      1     0.25                        test	r9b, 64
 1      1     0.50                        cmove	rbx, rdi
 1      1     0.50                        cmovne	r11, rsi
 1      0     0.17                        mov	r14, r11
 5      2     1.50                        shrd	r14, r10, cl
 1      1     0.25                        test	r9b, 64
 1      1     0.50                        cmovne	r14, rdi
 1      1     0.25                        xor	rbx, r8
 1      1     0.25                        xor	r14, rdx
 1      1     0.25                        or	r14, rbx
 1      1     0.50                        jne	.LBB70_2
 1      1     1.00           *            mov	qword ptr [rax + 16], r11
 1      1     1.00           *            mov	qword ptr [rax + 24], r10
 1      1     0.50                        mov	esi, 1
 1      1     1.00           *            mov	qword ptr [rax], rsi
 1      1     1.00           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rbx
 1      5     0.33    *                   pop	rdi
 1      5     0.33    *                   pop	rsi
 1      5     0.33    *                   pop	r14

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
4.00   4.00   4.00   4.00   4.00   8.00   5.00   1.00    -      -      -      -      -      -      -     7.00   7.00   6.00   1.00   1.00   2.00   8.00   8.00   

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00   push	r14
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -     push	rsi
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -      -     2.00   push	rdi
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -     push	rbx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, r8
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
 -      -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shld	r10, rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, rdx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r11, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r10, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdi, r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	rdi, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rbx, r10
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	rbx, 63
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	esi, esi
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	rbx, rdi
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r11, rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r14, r11
 -      -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shrd	r14, r10, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r14, rdi
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	rbx, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	r14, rdx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     or	r14, rbx
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jne	.LBB70_2
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00   mov	qword ptr [rax + 16], r11
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -     mov	qword ptr [rax + 24], r10
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	esi, 1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00   mov	qword ptr [rax], rsi
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -     mov	qword ptr [rax + 8], 0
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     pop	rbx
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -     1.00    -      -      -     pop	rdi
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -     1.00    -      -      -      -     pop	rsi
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -     pop	r14
```
## `checked_mul_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      41
Total Cycles:      31
Total uOps:        49

Dispatch Width:    6
uOps Per Cycle:    1.58
IPC:               1.32
Block RThroughput: 8.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     1.00           *            push	r14
 1      1     1.00           *            push	rsi
 1      1     1.00           *            push	rdi
 1      1     1.00           *            push	rbx
 1      0     0.17                        mov	rax, rcx
 1      1     0.25                        test	r9b, r9b
 1      1     0.50                        js	.LBB71_1
 1      0     0.17                        mov	r10, r8
 1      0     0.17                        mov	ecx, r9d
 5      2     1.50                        shld	r10, rdx, cl
 1      0     0.17                        mov	r11, rdx
 1      1     0.50                        shl	r11, cl
 1      1     0.25                        test	r9b, 64
 1      1     0.50                        cmovne	r10, r11
 1      0     0.17                        mov	rdi, r10
 1      1     0.50                        sar	rdi, cl
 1      0     0.17                        mov	rbx, r10
 1      1     0.50                        sar	rbx, 63
 1      0     0.17                        xor	esi, esi
 1      1     0.25                        test	r9b, 64
 1      1     0.50                        cmove	rbx, rdi
 1      1     0.50                        cmovne	r11, rsi
 1      0     0.17                        mov	r14, r11
 5      2     1.50                        shrd	r14, r10, cl
 1      1     0.25                        test	r9b, 64
 1      1     0.50                        cmovne	r14, rdi
 1      1     0.25                        xor	rbx, r8
 1      1     0.25                        xor	r14, rdx
 1      1     0.25                        or	r14, rbx
 1      1     0.50                        jne	.LBB71_4
 1      1     1.00           *            mov	qword ptr [rax + 16], r11
 1      1     1.00           *            mov	qword ptr [rax + 24], r10
 1      1     0.50                        mov	esi, 1
 1      1     0.50                        jmp	.LBB71_4
 1      0     0.17                        xor	esi, esi
 1      1     1.00           *            mov	qword ptr [rax], rsi
 1      1     1.00           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rbx
 1      5     0.33    *                   pop	rdi
 1      5     0.33    *                   pop	rsi
 1      5     0.33    *                   pop	r14

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
4.00   4.00   4.00   4.00   5.00   8.00   5.00   3.00    -      -      -      -      -      -      -     7.00   6.00   7.00   1.00   1.00   2.00   8.00   8.00   

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00   push	r14
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -     push	rsi
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -      -     2.00   push	rdi
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -     push	rbx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, r9b
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB71_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, r8
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
 -      -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shld	r10, rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, rdx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r11, cl
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r10, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdi, r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	rdi, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rbx, r10
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	rbx, 63
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	esi, esi
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	rbx, rdi
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r11, rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r14, r11
 -      -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shrd	r14, r10, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r14, rdi
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	rbx, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	r14, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     or	r14, rbx
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jne	.LBB71_4
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00   mov	qword ptr [rax + 16], r11
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -     mov	qword ptr [rax + 24], r10
 -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	esi, 1
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jmp	.LBB71_4
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	esi, esi
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00   mov	qword ptr [rax], rsi
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -     mov	qword ptr [rax + 8], 0
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -     pop	rbx
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -     1.00    -      -      -     pop	rdi
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -     1.00    -      -      -      -     pop	rsi
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -     pop	r14
```
## `checked_mul_u8_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      7
Total uOps:        9

Dispatch Width:    6
uOps Per Cycle:    1.29
IPC:               1.29
Block RThroughput: 1.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, edx
 1      0     0.17                        mov	eax, ecx
 1      0     0.17                        mov	edx, ecx
 1      0     0.17                        mov	ecx, r8d
 1      1     0.50                        shl	dl, cl
 1      0     0.17                        mov	r9d, edx
 1      1     0.50                        shr	r9b, cl
 1      1     0.25                        cmp	al, r9b
 1      1     1.00                        sete	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     2.00   1.00   1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, edx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	dl, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9d, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	r9b, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	al, r9b
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sete	al
```
## `checked_mul_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      10
Total uOps:        14

Dispatch Width:    6
uOps Per Cycle:    1.40
IPC:               1.40
Block RThroughput: 2.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, edx
 1      0     0.17                        mov	eax, ecx
 1      1     0.25                        cmp	dl, 8
 1      1     1.00                        setb	r9b
 1      0     0.17                        mov	edx, ecx
 1      0     0.17                        mov	ecx, r8d
 1      1     0.50                        shl	dl, cl
 1      1     0.25                        and	r8b, 7
 1      0     0.17                        mov	r10d, edx
 1      0     0.17                        mov	ecx, r8d
 1      1     0.50                        shr	r10b, cl
 1      1     0.25                        cmp	al, r10b
 1      1     1.00                        sete	al
 1      1     0.25                        and	al, r9b

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     4.00   2.00   2.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, edx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 8
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setb	r9b
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	dl, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	r8b, 7
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10d, edx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	r10b, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	al, r10b
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sete	al
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	al, r9b
```
## `checked_mul_u16_pow2`
```asm
Iterations:        1
Instructions:      12
Total Cycles:      8
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.50
IPC:               1.50
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, edx
 1      0     0.17                        mov	r9d, ecx
 1      0     0.17                        mov	eax, r9d
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	eax, cl
 1      1     0.25                        movzx	edx, ax
 1      0     0.17                        mov	r10d, edx
 1      0     0.17                        mov	ecx, r8d
 1      1     0.50                        shr	r10d, cl
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	r9w, r10w
 1      1     1.00                        sete	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   1.00   3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, edx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9d, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, r9d
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	eax, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	edx, ax
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10d, edx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	r10d, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	r9w, r10w
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sete	al
```
## `checked_mul_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      10
Total uOps:        14

Dispatch Width:    6
uOps Per Cycle:    1.40
IPC:               1.40
Block RThroughput: 2.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        cmp	dl, 15
 1      1     0.50                        ja	.LBB83_1
 1      0     0.17                        mov	eax, edx
 1      0     0.17                        mov	edx, ecx
 1      0     0.17                        mov	r8d, ecx
 1      0     0.17                        mov	ecx, eax
 1      1     0.50                        shl	edx, cl
 1      1     0.25                        movzx	r9d, dx
 1      1     0.50                        shr	r9d, cl
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	r8w, r9w
 1      1     1.00                        sete	al
 1      5     0.50                  U     ret
 1      0     0.17                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   3.00   1.00   2.00   2.00   1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 15
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB83_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, edx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	edx, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	r9d, dx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	r9d, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	r8w, r9w
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sete	al
 -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
```
## `checked_mul_u32_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      7
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.43
IPC:               1.43
Block RThroughput: 1.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, edx
 1      0     0.17                        mov	r9d, ecx
 1      0     0.17                        mov	edx, ecx
 1      0     0.17                        mov	ecx, r8d
 1      1     0.50                        shl	edx, cl
 1      0     0.17                        mov	r10d, edx
 1      1     0.50                        shr	r10d, cl
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	r9d, r10d
 1      1     1.00                        sete	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     2.00   1.00   1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, edx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9d, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	edx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10d, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	r10d, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	r9d, r10d
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sete	al
```
## `checked_mul_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      10
Total uOps:        14

Dispatch Width:    6
uOps Per Cycle:    1.40
IPC:               1.40
Block RThroughput: 2.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        cmp	dl, 31
 1      1     0.50                        ja	.LBB85_1
 1      0     0.17                        mov	eax, edx
 1      0     0.17                        mov	edx, ecx
 1      0     0.17                        mov	r8d, ecx
 1      0     0.17                        mov	ecx, eax
 1      1     0.50                        shl	edx, cl
 1      0     0.17                        mov	r9d, edx
 1      1     0.50                        shr	r9d, cl
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	r8d, r9d
 1      1     1.00                        sete	al
 1      5     0.50                  U     ret
 1      0     0.17                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   3.00   1.00   1.00   2.00   1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 31
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB85_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, edx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	edx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9d, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	r9d, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	r8d, r9d
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sete	al
 -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
```
## `checked_mul_u64_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      7
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.43
IPC:               1.43
Block RThroughput: 1.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, edx
 1      0     0.17                        mov	r9, rcx
 1      0     0.17                        mov	rdx, rcx
 1      0     0.17                        mov	ecx, r8d
 1      1     0.50                        shl	rdx, cl
 1      0     0.17                        mov	r10, rdx
 1      1     0.50                        shr	r10, cl
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	r9, r10
 1      1     1.00                        sete	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     2.00   1.00   1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, edx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, rdx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	r10, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	r9, r10
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sete	al
```
## `checked_mul_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      10
Total uOps:        14

Dispatch Width:    6
uOps Per Cycle:    1.40
IPC:               1.40
Block RThroughput: 2.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        cmp	dl, 63
 1      1     0.50                        ja	.LBB87_1
 1      0     0.17                        mov	eax, edx
 1      0     0.17                        mov	rdx, rcx
 1      0     0.17                        mov	r8, rcx
 1      0     0.17                        mov	ecx, eax
 1      1     0.50                        shl	rdx, cl
 1      0     0.17                        mov	r9, rdx
 1      1     0.50                        shr	r9, cl
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	r8, r9
 1      1     1.00                        sete	al
 1      5     0.50                  U     ret
 1      0     0.17                        xor	eax, eax

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   3.00   1.00   1.00   2.00   1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 63
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB87_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, edx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rdx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	r9, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	r8, r9
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sete	al
 -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
```
## `checked_mul_u128_pow2`
```asm
Iterations:        1
Instructions:      36
Total Cycles:      29
Total uOps:        44

Dispatch Width:    6
uOps Per Cycle:    1.52
IPC:               1.24
Block RThroughput: 8.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     1.00           *            push	r14
 1      1     1.00           *            push	rsi
 1      1     1.00           *            push	rdi
 1      1     1.00           *            push	rbx
 1      0     0.17                        mov	rax, rcx
 1      0     0.17                        mov	r10, r8
 1      0     0.17                        mov	ecx, r9d
 5      2     1.50                        shld	r10, rdx, cl
 1      0     0.17                        mov	r11, rdx
 1      1     0.50                        shl	r11, cl
 1      1     0.25                        test	r9b, 64
 1      1     0.50                        cmovne	r10, r11
 1      0     0.17                        mov	rdi, r10
 1      1     0.50                        shr	rdi, cl
 1      0     0.17                        xor	esi, esi
 1      1     0.25                        test	r9b, 64
 1      1     0.50                        cmovne	r11, rsi
 1      0     0.17                        mov	rbx, rdi
 1      1     0.50                        cmovne	rbx, rsi
 1      0     0.17                        mov	r14, r11
 5      2     1.50                        shrd	r14, r10, cl
 1      1     0.25                        test	r9b, 64
 1      1     0.50                        cmovne	r14, rdi
 1      1     0.25                        xor	r14, rdx
 1      1     0.25                        xor	rbx, r8
 1      1     0.25                        or	rbx, r14
 1      1     0.50                        jne	.LBB80_2
 1      1     1.00           *            mov	qword ptr [rax + 16], r11
 1      1     1.00           *            mov	qword ptr [rax + 24], r10
 1      1     0.50                        mov	esi, 1
 1      1     1.00           *            mov	qword ptr [rax], rsi
 1      1     1.00           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rbx
 1      5     0.33    *                   pop	rdi
 1      5     0.33    *                   pop	rsi
 1      5     0.33    *                   pop	r14

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
4.00   4.00   4.00   4.00   4.00   8.00   4.00   1.00    -      -      -      -      -      -      -     7.00   7.00   6.00   1.00   1.00   2.00   8.00   8.00   

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00   push	r14
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -     push	rsi
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -      -     2.00   push	rdi
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -     push	rbx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, r8
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
 -      -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shld	r10, rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, rdx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r11, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r10, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdi, r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	rdi, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	esi, esi
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r11, rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rbx, rdi
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rbx, rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r14, r11
 -      -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shrd	r14, r10, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r14, rdi
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	r14, rdx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	rbx, r8
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     or	rbx, r14
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jne	.LBB80_2
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00   mov	qword ptr [rax + 16], r11
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -     mov	qword ptr [rax + 24], r10
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	esi, 1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00   mov	qword ptr [rax], rsi
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -     mov	qword ptr [rax + 8], 0
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     pop	rbx
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -     1.00    -      -      -     pop	rdi
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -     1.00    -      -      -      -     pop	rsi
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -     pop	r14
```
## `checked_mul_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      40
Total Cycles:      31
Total uOps:        48

Dispatch Width:    6
uOps Per Cycle:    1.55
IPC:               1.29
Block RThroughput: 8.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     1.00           *            push	r14
 1      1     1.00           *            push	rsi
 1      1     1.00           *            push	rdi
 1      1     1.00           *            push	rbx
 1      0     0.17                        mov	rax, rcx
 1      1     0.25                        test	r9b, r9b
 1      1     0.50                        js	.LBB81_1
 1      0     0.17                        mov	r10, r8
 1      0     0.17                        mov	ecx, r9d
 5      2     1.50                        shld	r10, rdx, cl
 1      0     0.17                        mov	r11, rdx
 1      1     0.50                        shl	r11, cl
 1      1     0.25                        test	r9b, 64
 1      1     0.50                        cmovne	r10, r11
 1      0     0.17                        mov	rdi, r10
 1      1     0.50                        shr	rdi, cl
 1      0     0.17                        xor	esi, esi
 1      1     0.25                        test	r9b, 64
 1      1     0.50                        cmovne	r11, rsi
 1      0     0.17                        mov	rbx, rdi
 1      1     0.50                        cmovne	rbx, rsi
 1      0     0.17                        mov	r14, r11
 5      2     1.50                        shrd	r14, r10, cl
 1      1     0.25                        test	r9b, 64
 1      1     0.50                        cmovne	r14, rdi
 1      1     0.25                        xor	r14, rdx
 1      1     0.25                        xor	rbx, r8
 1      1     0.25                        or	rbx, r14
 1      1     0.50                        jne	.LBB81_4
 1      1     1.00           *            mov	qword ptr [rax + 16], r11
 1      1     1.00           *            mov	qword ptr [rax + 24], r10
 1      1     0.50                        mov	esi, 1
 1      1     0.50                        jmp	.LBB81_4
 1      0     0.17                        xor	esi, esi
 1      1     1.00           *            mov	qword ptr [rax], rsi
 1      1     1.00           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rbx
 1      5     0.33    *                   pop	rdi
 1      5     0.33    *                   pop	rsi
 1      5     0.33    *                   pop	r14

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
4.00   4.00   4.00   4.00   4.00   7.00   6.00   3.00    -      -      -      -      -      -      -     7.00   6.00   7.00   1.00   1.00   2.00   8.00   8.00   

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00   push	r14
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -     push	rsi
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -      -     2.00   push	rdi
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -     push	rbx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, r9b
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB81_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, r8
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
 -      -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shld	r10, rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, rdx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r11, cl
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r10, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdi, r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	rdi, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	esi, esi
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r11, rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rbx, rdi
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rbx, rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r14, r11
 -      -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shrd	r14, r10, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r14, rdi
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	r14, rdx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	rbx, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     or	rbx, r14
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jne	.LBB81_4
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00   mov	qword ptr [rax + 16], r11
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -     mov	qword ptr [rax + 24], r10
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	esi, 1
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jmp	.LBB81_4
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	esi, esi
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00   mov	qword ptr [rax], rsi
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -     mov	qword ptr [rax + 8], 0
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -     pop	rbx
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -     1.00    -      -      -     pop	rdi
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -     1.00    -      -      -      -     pop	rsi
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -     pop	r14
```
## `div_ceil_i8_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      8
Total uOps:        8

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 1.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      1     0.50                        mov	r8, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r8, cl
 1      1     0.25                        not	r8
 1      1     0.25                        movsx	rax, al
 1      1     0.25                        add	rax, r8
 1      1     0.50                        sar	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   2.00   3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r8, cl
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movsx	rax, al
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
```
## `div_ceil_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      8
Total uOps:        8

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 1.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      1     0.50                        mov	r8, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r8, cl
 1      1     0.25                        not	r8
 1      1     0.25                        movsx	rax, al
 1      1     0.25                        add	rax, r8
 1      1     0.50                        sar	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   2.00   3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r8, cl
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movsx	rax, al
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
```
## `div_ceil_i16_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      8
Total uOps:        8

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 1.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      1     0.50                        mov	r8, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r8, cl
 1      1     0.25                        not	r8
 1      1     0.25                        movsx	rax, ax
 1      1     0.25                        add	rax, r8
 1      1     0.50                        sar	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   2.00   3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r8, cl
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movsx	rax, ax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
```
## `div_ceil_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      8
Total uOps:        8

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 1.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      1     0.50                        mov	r8, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r8, cl
 1      1     0.25                        not	r8
 1      1     0.25                        movsx	rax, ax
 1      1     0.25                        add	rax, r8
 1      1     0.50                        sar	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   2.00   3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r8, cl
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movsx	rax, ax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
```
## `div_ceil_i32_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      8
Total uOps:        8

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 1.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      1     0.50                        mov	r8, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r8, cl
 1      1     0.25                        not	r8
 1      1     0.25                        cdqe
 1      1     0.25                        add	rax, r8
 1      1     0.50                        sar	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   2.00   3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r8, cl
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cdqe
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
```
## `div_ceil_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      8
Total uOps:        8

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 1.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      1     0.50                        mov	r8, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r8, cl
 1      1     0.25                        not	r8
 1      1     0.25                        cdqe
 1      1     0.25                        add	rax, r8
 1      1     0.50                        sar	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   2.00   3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r8, cl
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cdqe
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
```
## `div_ceil_i64_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      8
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.25
IPC:               1.25
Block RThroughput: 1.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8, rcx
 1      0     0.17                        mov	r9, rcx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        sar	r9, cl
 1      0     0.17                        mov	r10, r9
 1      1     0.50                        shl	r10, cl
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	r8, r10
 1      1     1.00                        setne	al
 1      1     0.25                        add	rax, r9

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     2.00   1.00   1.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	r9, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, r9
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r10, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	r8, r10
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setne	al
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, r9
```
## `div_ceil_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      8
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.25
IPC:               1.25
Block RThroughput: 1.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8, rcx
 1      0     0.17                        mov	r9, rcx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        sar	r9, cl
 1      0     0.17                        mov	r10, r9
 1      1     0.50                        shl	r10, cl
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	r8, r10
 1      1     1.00                        setne	al
 1      1     0.25                        add	rax, r9

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     2.00   1.00   1.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	r9, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, r9
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r10, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	r8, r10
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setne	al
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, r9
```
## `div_ceil_i128_pow2`
```asm
Iterations:        1
Instructions:      33
Total Cycles:      19
Total uOps:        41

Dispatch Width:    6
uOps Per Cycle:    2.16
IPC:               1.74
Block RThroughput: 6.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     1.00           *            push	rsi
 1      1     1.00           *            push	rdi
 1      0     0.17                        mov	rax, rcx
 1      0     0.17                        mov	r10, rcx
 1      0     0.17                        mov	ecx, r8d
 5      2     1.50                        shrd	r10, rdx, cl
 1      0     0.17                        mov	r11, rdx
 1      1     0.50                        sar	r11, cl
 1      1     0.25                        test	r8b, 64
 1      1     0.50                        cmovne	r10, r11
 1      0     0.17                        mov	rsi, r10
 1      1     0.50                        shl	rsi, cl
 1      0     0.17                        xor	edi, edi
 1      1     0.25                        test	r8b, 64
 1      1     0.50                        cmove	rdi, rsi
 1      0     0.17                        mov	r9, rdx
 1      1     0.50                        sar	r9, 63
 1      1     0.25                        test	r8b, 64
 1      1     0.50                        cmove	r9, r11
 1      0     0.17                        mov	r11, r9
 5      2     1.50                        shld	r11, r10, cl
 1      1     0.25                        test	r8b, 64
 1      1     0.50                        cmovne	r11, rsi
 1      1     0.25                        xor	r11, rdx
 1      1     0.25                        xor	rdi, rax
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        or	rdi, r11
 1      1     1.00                        setne	al
 1      1     0.25                        add	rax, r10
 1      1     1.00                        adc	r9, 0
 1      0     0.17                        mov	rdx, r9
 1      5     0.33    *                   pop	rdi
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
1.00   1.00   2.00   4.00   10.00  7.00   6.00    -      -      -      -      -      -      -      -     1.00   2.00   3.00    -     1.00   1.00   2.00   2.00   

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00   push	rsi
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -     push	rdi
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shrd	r10, rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, rdx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	r11, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r10, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rsi, r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	rsi, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	edi, edi
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	rdi, rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rdx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	r9, 63
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	r9, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, r9
 -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shld	r11, r10, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r11, rsi
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	r11, rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	rdi, rax
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     or	rdi, r11
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setne	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, r10
 -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     adc	r9, 0
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r9
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -     pop	rdi
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -     1.00    -      -      -     pop	rsi
```
## `div_ceil_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      33
Total Cycles:      19
Total uOps:        41

Dispatch Width:    6
uOps Per Cycle:    2.16
IPC:               1.74
Block RThroughput: 6.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     1.00           *            push	rsi
 1      1     1.00           *            push	rdi
 1      0     0.17                        mov	rax, rcx
 1      0     0.17                        mov	r10, rcx
 1      0     0.17                        mov	ecx, r8d
 5      2     1.50                        shrd	r10, rdx, cl
 1      0     0.17                        mov	r11, rdx
 1      1     0.50                        sar	r11, cl
 1      1     0.25                        test	r8b, 64
 1      1     0.50                        cmovne	r10, r11
 1      0     0.17                        mov	rsi, r10
 1      1     0.50                        shl	rsi, cl
 1      0     0.17                        xor	edi, edi
 1      1     0.25                        test	r8b, 64
 1      1     0.50                        cmove	rdi, rsi
 1      0     0.17                        mov	r9, rdx
 1      1     0.50                        sar	r9, 63
 1      1     0.25                        test	r8b, 64
 1      1     0.50                        cmove	r9, r11
 1      0     0.17                        mov	r11, r9
 5      2     1.50                        shld	r11, r10, cl
 1      1     0.25                        test	r8b, 64
 1      1     0.50                        cmovne	r11, rsi
 1      1     0.25                        xor	r11, rdx
 1      1     0.25                        xor	rdi, rax
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        or	rdi, r11
 1      1     1.00                        setne	al
 1      1     0.25                        add	rax, r10
 1      1     1.00                        adc	r9, 0
 1      0     0.17                        mov	rdx, r9
 1      5     0.33    *                   pop	rdi
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
1.00   1.00   2.00   4.00   10.00  7.00   6.00    -      -      -      -      -      -      -      -     1.00   2.00   3.00    -     1.00   1.00   2.00   2.00   

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00   push	rsi
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -     push	rdi
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shrd	r10, rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, rdx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	r11, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r10, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rsi, r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	rsi, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	edi, edi
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	rdi, rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rdx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	r9, 63
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	r9, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, r9
 -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shld	r11, r10, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r11, rsi
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	r11, rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	rdi, rax
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     or	rdi, r11
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setne	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, r10
 -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     adc	r9, 0
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r9
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -     pop	rdi
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -     1.00    -      -      -     pop	rsi
```
## `div_ceil_u8_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      8
Total uOps:        8

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 1.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      1     0.50                        mov	r8, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r8, cl
 1      1     0.25                        not	r8
 1      1     0.25                        movzx	eax, al
 1      1     0.25                        add	rax, r8
 1      1     0.50                        sar	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   2.00   3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r8, cl
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
```
## `div_ceil_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      8
Total uOps:        8

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 1.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      1     0.50                        mov	r8, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r8, cl
 1      1     0.25                        not	r8
 1      1     0.25                        movzx	eax, al
 1      1     0.25                        add	rax, r8
 1      1     0.50                        sar	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   2.00   3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r8, cl
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
```
## `div_ceil_u16_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      8
Total uOps:        8

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 1.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      1     0.50                        mov	r8, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r8, cl
 1      1     0.25                        not	r8
 1      1     0.25                        movzx	eax, ax
 1      1     0.25                        add	rax, r8
 1      1     0.50                        sar	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   2.00   3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r8, cl
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, ax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
```
## `div_ceil_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      8
Total uOps:        8

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 1.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      1     0.50                        mov	r8, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r8, cl
 1      1     0.25                        not	r8
 1      1     0.25                        movzx	eax, ax
 1      1     0.25                        add	rax, r8
 1      1     0.50                        sar	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   2.00   3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r8, cl
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, ax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
```
## `div_ceil_u32_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      8
Total uOps:        8

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 1.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      1     0.50                        mov	r8, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r8, cl
 1      1     0.25                        not	r8
 1      0     0.17                        mov	eax, eax
 1      1     0.25                        add	rax, r8
 1      1     0.50                        sar	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   2.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r8, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r8
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, eax
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
```
## `div_ceil_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      8
Total uOps:        8

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 1.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      1     0.50                        mov	r8, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r8, cl
 1      1     0.25                        not	r8
 1      0     0.17                        mov	eax, eax
 1      1     0.25                        add	rax, r8
 1      1     0.50                        sar	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   2.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r8, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r8
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, eax
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, r8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
```
## `div_ceil_u64_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      8
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.25
IPC:               1.25
Block RThroughput: 1.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8, rcx
 1      0     0.17                        mov	r9, rcx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	r9, cl
 1      0     0.17                        mov	r10, r9
 1      1     0.50                        shl	r10, cl
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	r8, r10
 1      1     1.00                        setne	al
 1      1     0.25                        add	rax, r9

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     2.00   1.00   1.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	r9, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, r9
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r10, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	r8, r10
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setne	al
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, r9
```
## `div_ceil_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      10
Total Cycles:      8
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.25
IPC:               1.25
Block RThroughput: 1.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8, rcx
 1      0     0.17                        mov	r9, rcx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	r9, cl
 1      0     0.17                        mov	r10, r9
 1      1     0.50                        shl	r10, cl
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	r8, r10
 1      1     1.00                        setne	al
 1      1     0.25                        add	rax, r9

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     2.00   1.00   1.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	r9, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, r9
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r10, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	r8, r10
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setne	al
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, r9
```
## `div_ceil_u128_pow2`
```asm
Iterations:        1
Instructions:      30
Total Cycles:      19
Total uOps:        38

Dispatch Width:    6
uOps Per Cycle:    2.00
IPC:               1.58
Block RThroughput: 6.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     1.00           *            push	rsi
 1      1     1.00           *            push	rdi
 1      0     0.17                        mov	r9d, r8d
 1      0     0.17                        mov	r8, rdx
 1      0     0.17                        mov	rax, rcx
 1      0     0.17                        mov	r10, rcx
 1      0     0.17                        mov	ecx, r9d
 5      2     1.50                        shrd	r10, rdx, cl
 1      1     0.50                        shr	rdx, cl
 1      1     0.25                        test	r9b, 64
 1      1     0.50                        cmovne	r10, rdx
 1      0     0.17                        mov	r11, r10
 1      1     0.50                        shl	r11, cl
 1      0     0.17                        xor	esi, esi
 1      1     0.25                        test	r9b, 64
 1      1     0.50                        cmovne	rdx, rsi
 1      1     0.50                        cmove	rsi, r11
 1      0     0.17                        mov	rdi, rdx
 5      2     1.50                        shld	rdi, r10, cl
 1      1     0.25                        test	r9b, 64
 1      1     0.50                        cmovne	rdi, r11
 1      1     0.25                        xor	rdi, r8
 1      1     0.25                        xor	rsi, rax
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        or	rsi, rdi
 1      1     1.00                        setne	al
 1      1     0.25                        add	rax, r10
 1      1     1.00                        adc	rdx, 0
 1      5     0.33    *                   pop	rdi
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
1.00   1.00   2.00   5.00   4.00   8.00   8.00    -      -      -      -      -      -      -      -     1.00   2.00   3.00    -     1.00   1.00   2.00   2.00   

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00   push	rsi
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -     push	rdi
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9d, r8d
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
 -      -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shrd	r10, rdx, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	rdx, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r10, rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r11, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	esi, esi
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rdx, rsi
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	rsi, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdi, rdx
 -      -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shld	rdi, r10, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rdi, r11
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	rdi, r8
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	rsi, rax
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     or	rsi, rdi
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setne	al
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, r10
 -      -      -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     adc	rdx, 0
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -     pop	rdi
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -     1.00    -      -      -     pop	rsi
```
## `div_ceil_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      30
Total Cycles:      19
Total uOps:        38

Dispatch Width:    6
uOps Per Cycle:    2.00
IPC:               1.58
Block RThroughput: 6.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     1.00           *            push	rsi
 1      1     1.00           *            push	rdi
 1      0     0.17                        mov	r9d, r8d
 1      0     0.17                        mov	r8, rdx
 1      0     0.17                        mov	rax, rcx
 1      0     0.17                        mov	r10, rcx
 1      0     0.17                        mov	ecx, r9d
 5      2     1.50                        shrd	r10, rdx, cl
 1      1     0.50                        shr	rdx, cl
 1      1     0.25                        test	r9b, 64
 1      1     0.50                        cmovne	r10, rdx
 1      0     0.17                        mov	r11, r10
 1      1     0.50                        shl	r11, cl
 1      0     0.17                        xor	esi, esi
 1      1     0.25                        test	r9b, 64
 1      1     0.50                        cmovne	rdx, rsi
 1      1     0.50                        cmove	rsi, r11
 1      0     0.17                        mov	rdi, rdx
 5      2     1.50                        shld	rdi, r10, cl
 1      1     0.25                        test	r9b, 64
 1      1     0.50                        cmovne	rdi, r11
 1      1     0.25                        xor	rdi, r8
 1      1     0.25                        xor	rsi, rax
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        or	rsi, rdi
 1      1     1.00                        setne	al
 1      1     0.25                        add	rax, r10
 1      1     1.00                        adc	rdx, 0
 1      5     0.33    *                   pop	rdi
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
1.00   1.00   2.00   5.00   4.00   8.00   8.00    -      -      -      -      -      -      -      -     1.00   2.00   3.00    -     1.00   1.00   2.00   2.00   

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00   push	rsi
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -     push	rdi
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9d, r8d
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
 -      -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shrd	r10, rdx, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	rdx, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r10, rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r11, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	esi, esi
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rdx, rsi
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	rsi, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdi, rdx
 -      -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shld	rdi, r10, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rdi, r11
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	rdi, r8
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	rsi, rax
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     or	rsi, rdi
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setne	al
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, r10
 -      -      -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     adc	rdx, 0
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -     pop	rdi
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -     1.00    -      -      -     pop	rsi
```
## `div_floor_i8_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      4
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.75
IPC:               0.75
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        sar	al, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	al, cl
```
## `div_floor_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      5
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.80
IPC:               0.80
Block RThroughput: 0.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      1     0.25                        and	dl, 7
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        sar	al, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -      -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	dl, 7
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	al, cl
```
## `div_floor_i16_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      5
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.60
IPC:               0.60
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        movsx	eax, cx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        sar	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -      -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movsx	eax, cx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	eax, cl
```
## `div_floor_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      5
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.80
IPC:               0.80
Block RThroughput: 0.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        movsx	eax, cx
 1      1     0.25                        and	dl, 15
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        sar	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -      -     1.00   1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movsx	eax, cx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	dl, 15
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	eax, cl
```
## `div_floor_i32_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      4
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.75
IPC:               0.75
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        sar	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	eax, cl
```
## `div_floor_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      4
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.75
IPC:               0.75
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        sar	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	eax, cl
```
## `div_floor_i64_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      4
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.75
IPC:               0.75
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	rax, rcx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        sar	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
```
## `div_floor_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      4
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.75
IPC:               0.75
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	rax, rcx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        sar	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
```
## `div_floor_i128_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      8
Total uOps:        13

Dispatch Width:    6
uOps Per Cycle:    1.63
IPC:               1.13
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	rax, rcx
 1      0     0.17                        mov	ecx, r8d
 5      2     1.50                        shrd	rax, rdx, cl
 1      0     0.17                        mov	r9, rdx
 1      1     0.50                        sar	r9, cl
 1      1     0.50                        sar	rdx, 63
 1      1     0.25                        test	r8b, 64
 1      1     0.50                        cmovne	rax, r9
 1      1     0.50                        cmove	rdx, r9

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   2.00   3.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shrd	rax, rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rdx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	r9, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	rdx, 63
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rax, r9
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	rdx, r9
```
## `div_floor_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      8
Total uOps:        13

Dispatch Width:    6
uOps Per Cycle:    1.63
IPC:               1.13
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	rax, rcx
 1      0     0.17                        mov	ecx, r8d
 5      2     1.50                        shrd	rax, rdx, cl
 1      0     0.17                        mov	r9, rdx
 1      1     0.50                        sar	r9, cl
 1      1     0.50                        sar	rdx, 63
 1      1     0.25                        test	r8b, 64
 1      1     0.50                        cmovne	rax, r9
 1      1     0.50                        cmove	rdx, r9

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   2.00   3.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shrd	rax, rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rdx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	r9, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	rdx, 63
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rax, r9
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	rdx, r9
```
## `div_floor_u8_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      4
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.75
IPC:               0.75
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	al, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	al, cl
```
## `div_floor_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      5
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.80
IPC:               0.80
Block RThroughput: 0.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      1     0.25                        and	dl, 7
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	al, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -      -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	dl, 7
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	al, cl
```
## `div_floor_u16_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      5
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.60
IPC:               0.60
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        movzx	eax, cx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -      -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, cx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	eax, cl
```
## `div_floor_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      5
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.80
IPC:               0.80
Block RThroughput: 0.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        movzx	eax, cx
 1      1     0.25                        and	dl, 15
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -      -     1.00   1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, cx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	dl, 15
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	eax, cl
```
## `div_floor_u32_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      4
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.75
IPC:               0.75
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	eax, cl
```
## `div_floor_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      4
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.75
IPC:               0.75
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	eax, cl
```
## `div_floor_u64_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      4
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.75
IPC:               0.75
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	rax, rcx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	rax, cl
```
## `div_floor_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      4
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.75
IPC:               0.75
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	rax, rcx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	rax, cl
```
## `div_floor_u128_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      7
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.71
IPC:               1.14
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	rax, rcx
 1      0     0.17                        mov	ecx, r8d
 5      2     1.50                        shrd	rax, rdx, cl
 1      1     0.50                        shr	rdx, cl
 1      0     0.17                        xor	ecx, ecx
 1      1     0.25                        test	r8b, 64
 1      1     0.50                        cmovne	rax, rdx
 1      1     0.50                        cmovne	rdx, rcx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   3.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shrd	rax, rdx, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rax, rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rdx, rcx
```
## `div_floor_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      7
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.71
IPC:               1.14
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	rax, rcx
 1      0     0.17                        mov	ecx, r8d
 5      2     1.50                        shrd	rax, rdx, cl
 1      1     0.50                        shr	rdx, cl
 1      0     0.17                        xor	ecx, ecx
 1      1     0.25                        test	r8b, 64
 1      1     0.50                        cmovne	rax, rdx
 1      1     0.50                        cmovne	rdx, rcx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   3.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shrd	rax, rdx, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rax, rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rdx, rcx
```
## `div_i8_pow2`
```asm
Iterations:        1
Instructions:      12
Total Cycles:      9
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.33
IPC:               1.33
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      1     0.25                        test	cl, cl
 1      1     0.50                        js	.LBB138_1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	al, cl
 1      5     0.50                  U     ret
 1      1     1.00                        mov	r8b, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r8b, cl
 1      1     0.25                        not	r8b
 1      1     0.25                        add	al, r8b
 1      1     0.50                        sar	al, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   2.00   5.00   2.00   2.00   1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	cl, cl
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB138_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	al, cl
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8b, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r8b, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r8b
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	al, r8b
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	al, cl
```
## `div_i8_unb_pow2`
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
 1      0     0.17                        mov	eax, ecx
 1      1     0.25                        and	dl, 7
 1      1     0.25                        test	cl, cl
 1      1     0.50                        js	.LBB139_1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	al, cl
 1      5     0.50                  U     ret
 1      1     1.00                        mov	r8b, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r8b, cl
 1      1     0.25                        not	r8b
 1      1     0.25                        add	al, r8b
 1      1     0.50                        sar	al, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   4.00   2.00   3.00   2.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	dl, 7
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	cl, cl
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB139_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	al, cl
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8b, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r8b, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r8b
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	al, r8b
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	al, cl
```
## `div_i16_pow2`
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
 1      0     0.17                        mov	eax, ecx
 1      1     0.25                        movzx	ecx, dl
 1      1     0.25                        test	ax, ax
 1      1     0.50                        js	.LBB132_1
 1      1     0.25                        movzx	eax, ax
 1      1     0.50                        shr	eax, cl
 1      5     0.50                  U     ret
 1      1     0.50                        mov	edx, -1
 1      1     0.50                        shl	edx, cl
 1      1     0.25                        not	edx
 1      1     0.25                        add	eax, edx
 1      1     0.25                        cwde
 1      1     0.50                        sar	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   3.00   2.00   3.00   3.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	ecx, dl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	ax, ax
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB132_1
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, ax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	eax, cl
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, -1
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	edx, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	edx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	eax, edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cwde
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	eax, cl
```
## `div_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      10
Total uOps:        14

Dispatch Width:    6
uOps Per Cycle:    1.40
IPC:               1.40
Block RThroughput: 2.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      1     0.25                        and	dl, 15
 1      1     0.25                        movzx	ecx, dl
 1      1     0.25                        test	ax, ax
 1      1     0.50                        js	.LBB133_1
 1      1     0.25                        movzx	eax, ax
 1      1     0.50                        shr	eax, cl
 1      5     0.50                  U     ret
 1      1     0.50                        mov	edx, -1
 1      1     0.50                        shl	edx, cl
 1      1     0.25                        not	edx
 1      1     0.25                        add	eax, edx
 1      1     0.25                        cwde
 1      1     0.50                        sar	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   2.00   4.00   3.00   3.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	dl, 15
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	ecx, dl
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	ax, ax
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB133_1
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, ax
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	eax, cl
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	edx, cl
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	eax, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cwde
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	eax, cl
```
## `div_i32_pow2`
```asm
Iterations:        1
Instructions:      12
Total Cycles:      9
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.33
IPC:               1.33
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      1     0.25                        test	ecx, ecx
 1      1     0.50                        js	.LBB134_1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	eax, cl
 1      5     0.50                  U     ret
 1      1     0.50                        mov	r8d, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r8d, cl
 1      1     0.25                        not	r8d
 1      1     0.25                        add	eax, r8d
 1      1     0.50                        sar	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   1.00   3.00   3.00   2.00   1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	ecx, ecx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB134_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	eax, cl
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r8d, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r8d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	eax, r8d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	eax, cl
```
## `div_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      13
Total Cycles:      9
Total uOps:        13

Dispatch Width:    6
uOps Per Cycle:    1.44
IPC:               1.44
Block RThroughput: 2.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      1     0.25                        and	dl, 31
 1      1     0.25                        test	ecx, ecx
 1      1     0.50                        js	.LBB135_1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	eax, cl
 1      5     0.50                  U     ret
 1      1     0.50                        mov	r8d, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r8d, cl
 1      1     0.25                        not	r8d
 1      1     0.25                        add	eax, r8d
 1      1     0.50                        sar	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   2.00   2.00   3.00   3.00   1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	dl, 31
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	ecx, ecx
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB135_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	eax, cl
 -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r8d, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r8d
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	eax, r8d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	eax, cl
```
## `div_i64_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      9
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    1.22
IPC:               1.22
Block RThroughput: 1.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	rax, rcx
 1      1     0.25                        movzx	ecx, dl
 1      1     0.25                        test	rax, rax
 1      1     0.50                        js	.LBB136_1
 1      1     0.50                        shr	rax, cl
 1      5     0.50                  U     ret
 1      1     0.50                        mov	rdx, -1
 1      1     0.50                        shl	rdx, cl
 1      1     0.25                        not	rdx
 1      1     0.25                        add	rax, rdx
 1      1     0.50                        sar	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   2.00   2.00   3.00   2.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	ecx, dl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	rax, rax
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB136_1
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	rax, cl
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, -1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	rdx, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	rdx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
```
## `div_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      12
Total Cycles:      10
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.20
IPC:               1.20
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	rax, rcx
 1      1     0.25                        and	dl, 63
 1      1     0.25                        movzx	ecx, dl
 1      1     0.25                        test	rax, rax
 1      1     0.50                        js	.LBB137_1
 1      1     0.50                        shr	rax, cl
 1      5     0.50                  U     ret
 1      1     0.50                        mov	rdx, -1
 1      1     0.50                        shl	rdx, cl
 1      1     0.25                        not	rdx
 1      1     0.25                        add	rax, rdx
 1      1     0.50                        sar	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   2.00   3.00   3.00   2.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	dl, 63
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	ecx, dl
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	rax, rax
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB137_1
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	rax, cl
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, -1
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	rdx, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	rdx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
```
## `div_i128_pow2`
```asm
Iterations:        1
Instructions:      32
Total Cycles:      17
Total uOps:        40

Dispatch Width:    6
uOps Per Cycle:    2.35
IPC:               1.88
Block RThroughput: 6.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	rax, rcx
 1      1     0.25                        test	rdx, rdx
 1      1     0.50                        js	.LBB130_1
 1      0     0.17                        mov	ecx, r8d
 5      2     1.50                        shrd	rax, rdx, cl
 1      1     0.50                        shr	rdx, cl
 1      0     0.17                        xor	ecx, ecx
 1      1     0.25                        test	r8b, 64
 1      1     0.50                        cmovne	rax, rdx
 1      1     0.50                        cmovne	rdx, rcx
 1      5     0.50                  U     ret
 1      1     0.50                        mov	r9, -1
 1      1     0.50                        mov	r10, -1
 1      0     0.17                        mov	ecx, r8d
 1      1     0.50                        shl	r10, cl
 1      0     0.17                        xor	ecx, ecx
 1      1     0.25                        test	r8b, 64
 1      1     0.50                        cmove	rcx, r10
 1      1     0.50                        cmove	r10, r9
 1      1     0.25                        not	r10
 1      1     0.25                        not	rcx
 1      1     0.25                        add	rax, rcx
 1      1     1.00                        adc	rdx, r10
 1      0     0.17                        mov	ecx, r8d
 5      2     1.50                        shrd	rax, rdx, cl
 1      0     0.17                        mov	r9, rdx
 1      1     0.50                        sar	r9, cl
 1      1     0.25                        test	r8b, 64
 1      1     0.50                        cmovne	rax, r9
 1      1     0.50                        sar	rdx, 63
 1      1     0.25                        test	r8b, 64
 1      1     0.50                        cmove	rdx, r9

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   6.00   8.00   10.00  8.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	rdx, rdx
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB130_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shrd	rax, rdx, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rax, rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rdx, rcx
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, -1
 -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r10, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	rcx, r10
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	r10, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r10
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, rcx
 -      -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     adc	rdx, r10
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shrd	rax, rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	r9, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rax, r9
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	rdx, 63
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	rdx, r9
```
## `div_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      33
Total Cycles:      17
Total uOps:        41

Dispatch Width:    6
uOps Per Cycle:    2.41
IPC:               1.94
Block RThroughput: 6.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	rax, rcx
 1      1     0.25                        and	r8b, 127
 1      1     0.25                        test	rdx, rdx
 1      1     0.50                        js	.LBB131_1
 1      0     0.17                        mov	ecx, r8d
 5      2     1.50                        shrd	rax, rdx, cl
 1      1     0.50                        shr	rdx, cl
 1      0     0.17                        xor	ecx, ecx
 1      1     0.25                        test	r8b, 64
 1      1     0.50                        cmovne	rax, rdx
 1      1     0.50                        cmovne	rdx, rcx
 1      5     0.50                  U     ret
 1      1     0.50                        mov	r9, -1
 1      1     0.50                        mov	r10, -1
 1      0     0.17                        mov	ecx, r8d
 1      1     0.50                        shl	r10, cl
 1      0     0.17                        xor	ecx, ecx
 1      1     0.25                        test	r8b, 64
 1      1     0.50                        cmove	rcx, r10
 1      1     0.50                        cmove	r10, r9
 1      1     0.25                        not	r10
 1      1     0.25                        not	rcx
 1      1     0.25                        add	rax, rcx
 1      1     1.00                        adc	rdx, r10
 1      0     0.17                        mov	ecx, r8d
 5      2     1.50                        shrd	rax, rdx, cl
 1      0     0.17                        mov	r9, rdx
 1      1     0.50                        sar	r9, cl
 1      1     0.25                        test	r8b, 64
 1      1     0.50                        cmovne	rax, r9
 1      1     0.50                        sar	rdx, 63
 1      1     0.25                        test	r8b, 64
 1      1     0.50                        cmove	rdx, r9

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   6.00   10.00  10.00  7.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	r8b, 127
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	rdx, rdx
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB131_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shrd	rax, rdx, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rax, rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rdx, rcx
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, -1
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r10, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	rcx, r10
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	r10, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r10
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	rcx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, rcx
 -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     adc	rdx, r10
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shrd	rax, rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rdx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	r9, cl
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rax, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	rdx, 63
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	rdx, r9
```
## `floor_to_multiple_i8_pow2`
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
 1      0     0.17                        mov	eax, ecx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	al, cl
 1      1     0.50                        shl	al, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	al, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	al, cl
```
## `floor_to_multiple_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      6
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.83
IPC:               0.83
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      1     0.25                        and	dl, 7
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	al, cl
 1      1     0.50                        shl	al, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -      -     1.00   1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	dl, 7
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	al, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	al, cl
```
## `floor_to_multiple_i16_pow2`
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
 1      1     0.25                        movzx	eax, cx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	eax, cl
 1      1     0.50                        shl	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -      -     1.00   1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, cx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	eax, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	eax, cl
```
## `floor_to_multiple_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      6
Total uOps:        5

Dispatch Width:    6
uOps Per Cycle:    0.83
IPC:               0.83
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        movzx	eax, cx
 1      1     0.25                        and	dl, 15
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	eax, cl
 1      1     0.50                        shl	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -      -     1.00   2.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, cx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	dl, 15
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	eax, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	eax, cl
```
## `floor_to_multiple_i32_pow2`
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
 1      0     0.17                        mov	eax, ecx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	eax, cl
 1      1     0.50                        shl	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	eax, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	eax, cl
```
## `floor_to_multiple_i32_unb_pow2`
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
 1      0     0.17                        mov	eax, ecx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	eax, cl
 1      1     0.50                        shl	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	eax, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	eax, cl
```
## `floor_to_multiple_i64_pow2`
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
 1      0     0.17                        mov	rax, rcx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	rax, cl
 1      1     0.50                        shl	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	rax, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	rax, cl
```
## `floor_to_multiple_i64_unb_pow2`
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
 1      0     0.17                        mov	rax, rcx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	rax, cl
 1      1     0.50                        shl	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	rax, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	rax, cl
```
## `floor_to_multiple_i128_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      7
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    1.57
IPC:               1.57
Block RThroughput: 1.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r9, rcx
 1      1     0.50                        mov	r10, -1
 1      0     0.17                        mov	ecx, r8d
 1      1     0.50                        shl	r10, cl
 1      1     0.50                        mov	rcx, -1
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        test	r8b, 64
 1      1     0.50                        cmovne	rcx, r10
 1      1     0.50                        cmove	rax, r10
 1      1     0.25                        and	rax, r9
 1      1     0.25                        and	rdx, rcx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     2.00   2.00   3.00   3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r10, cl
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rcx, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rcx, r10
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	rax, r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	rax, r9
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	rdx, rcx
```
## `floor_to_multiple_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      7
Total uOps:        11

Dispatch Width:    6
uOps Per Cycle:    1.57
IPC:               1.57
Block RThroughput: 1.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r9, rcx
 1      1     0.50                        mov	r10, -1
 1      0     0.17                        mov	ecx, r8d
 1      1     0.50                        shl	r10, cl
 1      1     0.50                        mov	rcx, -1
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        test	r8b, 64
 1      1     0.50                        cmovne	rcx, r10
 1      1     0.50                        cmove	rax, r10
 1      1     0.25                        and	rax, r9
 1      1     0.25                        and	rdx, rcx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     2.00   2.00   3.00   3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r10, cl
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rcx, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rcx, r10
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	rax, r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	rax, r9
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	rdx, rcx
```
## `is_multiple_of_i8_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      8
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.25
IPC:               0.63
Block RThroughput: 1.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        movzx	eax, cl
 1      1     1.00                        or	eax, 256
 6      1     1.00                        rep		bsf	eax, eax
 1      1     0.25                        cmp	al, dl
 1      1     1.00                        setae	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   4.00   3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, cl
 -      -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     or	eax, 256
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     rep		bsf	eax, eax
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	al, dl
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setae	al
```
## `is_multiple_of_i16_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      8
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.25
IPC:               0.63
Block RThroughput: 1.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        or	ecx, 65536
 6      1     1.00                        rep		bsf	eax, ecx
 1      1     0.25                        movzx	ecx, dl
 1      1     0.25                        cmp	ax, cx
 1      1     1.00                        setae	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   1.00   3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     or	ecx, 65536
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     rep		bsf	eax, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	ecx, dl
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	ax, cx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setae	al
```
## `is_multiple_of_i32_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      8
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.25
IPC:               0.63
Block RThroughput: 1.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        mov	eax, 32
 6      1     1.00                        rep		bsf	eax, ecx
 1      1     0.25                        movzx	ecx, dl
 1      1     0.25                        cmp	eax, ecx
 1      1     1.00                        setae	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   1.00   4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, 32
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     rep		bsf	eax, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	ecx, dl
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	eax, ecx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setae	al
```
## `is_multiple_of_i64_pow2`
```asm
Iterations:        1
Instructions:      5
Total Cycles:      8
Total uOps:        10

Dispatch Width:    6
uOps Per Cycle:    1.25
IPC:               0.63
Block RThroughput: 1.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.50                        mov	eax, 64
 6      1     1.00                        rep		bsf	rax, rcx
 1      1     0.25                        movzx	ecx, dl
 1      1     0.25                        cmp	eax, ecx
 1      1     1.00                        setae	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   1.00   4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, 64
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     rep		bsf	rax, rcx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	ecx, dl
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	eax, ecx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setae	al
```
## `is_multiple_of_i128_pow2`
```asm
Iterations:        1
Instructions:      9
Total Cycles:      10
Total uOps:        19

Dispatch Width:    6
uOps Per Cycle:    1.90
IPC:               0.90
Block RThroughput: 3.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 6      1     1.00                        rep		bsf	rax, rcx
 1      1     0.50                        mov	r9d, 64
 6      1     1.00                        rep		bsf	r9, rdx
 1      1     0.25                        add	r9d, 64
 1      1     0.25                        test	rcx, rcx
 1      1     0.50                        cmovne	r9d, eax
 1      1     0.25                        movzx	eax, r8b
 1      1     0.25                        cmp	r9d, eax
 1      1     1.00                        setae	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     2.00   2.00   2.00   5.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     rep		bsf	rax, rcx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9d, 64
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     rep		bsf	r9, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	r9d, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	rcx, rcx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r9d, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, r8b
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	r9d, eax
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setae	al
```
## `mod_floor_i8_pow2`
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
 1      0     0.17                        mov	r8d, ecx
 1      1     1.00                        mov	al, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	al, cl
 1      1     0.25                        not	al
 1      1     0.25                        and	al, r8b

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   1.00   4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	al, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	al, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	al
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	al, r8b
```
## `mod_floor_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      7
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 1.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, ecx
 1      1     0.25                        and	dl, 7
 1      1     1.00                        mov	al, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	al, cl
 1      1     0.25                        not	al
 1      1     0.25                        and	al, r8b

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   4.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	dl, 7
 -      -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	al, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	al, cl
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	al
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	al, r8b
```
## `mod_floor_i16_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      7
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.86
IPC:               0.86
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, ecx
 1      1     0.50                        mov	eax, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	eax, cl
 1      1     0.25                        not	eax
 1      1     0.25                        and	eax, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   1.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	eax, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	eax
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	eax, r8d
```
## `mod_floor_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      7
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 1.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, ecx
 1      1     0.25                        and	dl, 15
 1      1     0.50                        mov	eax, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	eax, cl
 1      1     0.25                        not	eax
 1      1     0.25                        and	eax, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   2.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	dl, 15
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	eax, cl
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	eax, r8d
```
## `mod_floor_i32_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      7
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.86
IPC:               0.86
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, ecx
 1      1     0.50                        mov	eax, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	eax, cl
 1      1     0.25                        not	eax
 1      1     0.25                        and	eax, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   1.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	eax, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	eax
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	eax, r8d
```
## `mod_floor_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      7
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.86
IPC:               0.86
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, ecx
 1      1     0.50                        mov	eax, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	eax, cl
 1      1     0.25                        not	eax
 1      1     0.25                        and	eax, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   1.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	eax, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	eax
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	eax, r8d
```
## `mod_floor_i64_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      7
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.86
IPC:               0.86
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8, rcx
 1      1     0.50                        mov	rax, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	rax, cl
 1      1     0.25                        not	rax
 1      1     0.25                        and	rax, r8

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   1.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	rax, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	rax
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	rax, r8
```
## `mod_floor_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      7
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.86
IPC:               0.86
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8, rcx
 1      1     0.50                        mov	rax, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	rax, cl
 1      1     0.25                        not	rax
 1      1     0.25                        and	rax, r8

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   1.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	rax, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	rax
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	rax, r8
```
## `mod_floor_i128_pow2`
```asm
Iterations:        1
Instructions:      13
Total Cycles:      8
Total uOps:        13

Dispatch Width:    6
uOps Per Cycle:    1.63
IPC:               1.63
Block RThroughput: 2.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r9, rcx
 1      1     0.50                        mov	r10, -1
 1      1     0.50                        mov	r11, -1
 1      0     0.17                        mov	ecx, r8d
 1      1     0.50                        shl	r11, cl
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        test	r8b, 64
 1      1     0.50                        cmove	rax, r11
 1      1     0.50                        cmove	r11, r10
 1      1     0.25                        not	r11
 1      1     0.25                        not	rax
 1      1     0.25                        and	rax, r9
 1      1     0.25                        and	rdx, r11

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     2.00   2.00   4.00   4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, -1
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r11, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	rax, r11
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	r11, r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r11
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	rax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	rax, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	rdx, r11
```
## `mod_floor_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      13
Total Cycles:      8
Total uOps:        13

Dispatch Width:    6
uOps Per Cycle:    1.63
IPC:               1.63
Block RThroughput: 2.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r9, rcx
 1      1     0.50                        mov	r10, -1
 1      1     0.50                        mov	r11, -1
 1      0     0.17                        mov	ecx, r8d
 1      1     0.50                        shl	r11, cl
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        test	r8b, 64
 1      1     0.50                        cmove	rax, r11
 1      1     0.50                        cmove	r11, r10
 1      1     0.25                        not	r11
 1      1     0.25                        not	rax
 1      1     0.25                        and	rax, r9
 1      1     0.25                        and	rdx, r11

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     2.00   2.00   4.00   4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, -1
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r11, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	rax, r11
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	r11, r10
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r11
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	rax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	rax, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	rdx, r11
```
## `mul_i8_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      4
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.75
IPC:               0.75
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	al, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	al, cl
```
## `mul_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      5
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.80
IPC:               0.80
Block RThroughput: 0.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      1     0.25                        and	dl, 7
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	al, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -      -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	dl, 7
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	al, cl
```
## `mul_i16_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      4
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.75
IPC:               0.75
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	eax, cl
```
## `mul_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      4
Total Cycles:      5
Total uOps:        4

Dispatch Width:    6
uOps Per Cycle:    0.80
IPC:               0.80
Block RThroughput: 0.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      1     0.25                        and	dl, 15
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -      -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	dl, 15
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	eax, cl
```
## `mul_i32_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      4
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.75
IPC:               0.75
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	eax, cl
```
## `mul_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      4
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.75
IPC:               0.75
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	eax, cl
```
## `mul_i64_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      4
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.75
IPC:               0.75
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	rax, rcx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	rax, cl
```
## `mul_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      3
Total Cycles:      4
Total uOps:        3

Dispatch Width:    6
uOps Per Cycle:    0.75
IPC:               0.75
Block RThroughput: 0.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	rax, rcx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	rax, cl
```
## `mul_i128_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      7
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.71
IPC:               1.14
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r9, rcx
 1      0     0.17                        mov	ecx, r8d
 5      2     1.50                        shld	rdx, r9, cl
 1      1     0.50                        shl	r9, cl
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        test	r8b, 64
 1      1     0.50                        cmovne	rdx, r9
 1      1     0.50                        cmove	rax, r9

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   3.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shld	rdx, r9, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r9, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rdx, r9
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	rax, r9
```
## `mul_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      7
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.71
IPC:               1.14
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r9, rcx
 1      0     0.17                        mov	ecx, r8d
 5      2     1.50                        shld	rdx, r9, cl
 1      1     0.50                        shl	r9, cl
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        test	r8b, 64
 1      1     0.50                        cmovne	rdx, r9
 1      1     0.50                        cmove	rax, r9

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   3.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shld	rdx, r9, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r9, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rdx, r9
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	rax, r9
```
## `unbounded_ceil_to_multiple_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      17
Total Cycles:      10
Total uOps:        17

Dispatch Width:    6
uOps Per Cycle:    1.70
IPC:               1.70
Block RThroughput: 2.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, ecx
 1      1     0.25                        cmp	dl, 8
 1      1     0.50                        jae	.LBB179_1
 1      1     1.00                        mov	r9b, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r9b, cl
 1      0     0.17                        mov	eax, r9d
 1      1     0.25                        not	al
 1      1     0.25                        add	r8b, al
 1      1     1.00                        setno	al
 1      1     0.25                        and	r8b, r9b
 1      0     0.17                        mov	edx, r8d
 1      5     0.50                  U     ret
 1      1     0.25                        test	r8b, r8b
 1      1     1.00                        setle	al
 1      0     0.17                        xor	r8d, r8d
 1      0     0.17                        mov	edx, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   3.00   2.00   5.00   4.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 8
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB179_1
 -      -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9b, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r9b, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, r9d
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	al
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	r8b, al
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setno	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	r8b, r9b
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, r8b
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setle	al
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	r8d, r8d
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
```
## `unbounded_ceil_to_multiple_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      19
Total Cycles:      11
Total uOps:        19

Dispatch Width:    6
uOps Per Cycle:    1.73
IPC:               1.73
Block RThroughput: 3.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, ecx
 1      1     0.25                        cmp	dl, 16
 1      1     0.50                        jae	.LBB176_1
 1      1     0.50                        mov	r9d, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r9d, cl
 1      0     0.17                        mov	eax, r9d
 1      1     0.25                        not	eax
 1      1     0.25                        add	r8w, ax
 1      1     1.00                        setno	al
 1      1     0.25                        and	r8d, r9d
 1      1     0.25                        movzx	eax, al
 1      0     0.17                        mov	edx, r8d
 1      5     0.50                  U     ret
 1      1     0.25                        test	r8w, r8w
 1      1     1.00                        setle	al
 1      0     0.17                        xor	r8d, r8d
 1      1     0.25                        movzx	eax, al
 1      0     0.17                        mov	edx, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   5.00   2.00   4.00   3.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 16
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB176_1
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9d, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r9d, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, r9d
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	r8w, ax
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setno	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	r8d, r9d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8w, r8w
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setle	al
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	r8d, r8d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
```
## `unbounded_ceil_to_multiple_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      19
Total Cycles:      11
Total uOps:        19

Dispatch Width:    6
uOps Per Cycle:    1.73
IPC:               1.73
Block RThroughput: 3.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, ecx
 1      1     0.25                        cmp	dl, 32
 1      1     0.50                        jae	.LBB177_1
 1      1     0.50                        mov	r9d, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r9d, cl
 1      0     0.17                        mov	eax, r9d
 1      1     0.25                        not	eax
 1      1     0.25                        add	r8d, eax
 1      1     1.00                        setno	al
 1      1     0.25                        and	r8d, r9d
 1      1     0.25                        movzx	eax, al
 1      0     0.17                        mov	edx, r8d
 1      5     0.50                  U     ret
 1      1     0.25                        test	r8d, r8d
 1      1     1.00                        setle	al
 1      0     0.17                        xor	r8d, r8d
 1      1     0.25                        movzx	eax, al
 1      0     0.17                        mov	edx, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   5.00   2.00   4.00   3.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 32
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB177_1
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9d, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r9d, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, r9d
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	r8d, eax
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setno	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	r8d, r9d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8d, r8d
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setle	al
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	r8d, r8d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
```
## `unbounded_ceil_to_multiple_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      19
Total Cycles:      11
Total uOps:        19

Dispatch Width:    6
uOps Per Cycle:    1.73
IPC:               1.73
Block RThroughput: 3.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8, rcx
 1      1     0.25                        cmp	dl, 64
 1      1     0.50                        jae	.LBB178_1
 1      1     0.50                        mov	r9, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r9, cl
 1      0     0.17                        mov	rax, r9
 1      1     0.25                        not	rax
 1      1     0.25                        add	r8, rax
 1      1     1.00                        setno	al
 1      1     0.25                        and	r8, r9
 1      1     0.25                        movzx	eax, al
 1      0     0.17                        mov	rdx, r8
 1      5     0.50                  U     ret
 1      1     0.25                        test	r8, r8
 1      1     1.00                        setle	al
 1      0     0.17                        xor	r8d, r8d
 1      1     0.25                        movzx	eax, al
 1      0     0.17                        mov	rdx, r8

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   5.00   2.00   4.00   3.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 64
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB178_1
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r9, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, r9
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	rax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	r8, rax
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setno	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	r8, r9
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r8
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8, r8
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setle	al
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	r8d, r8d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r8
```
## `unbounded_ceil_to_multiple_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      38
Total Cycles:      17
Total uOps:        38

Dispatch Width:    6
uOps Per Cycle:    2.24
IPC:               2.24
Block RThroughput: 7.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     1.00           *            push	rsi
 1      0     0.17                        mov	rax, rcx
 1      1     0.25                        test	r9b, r9b
 1      1     0.50                        js	.LBB175_4
 1      1     0.50                        mov	r10, -1
 1      0     0.17                        mov	ecx, r9d
 1      1     0.50                        shl	r10, cl
 1      1     0.50                        mov	r11, -1
 1      0     0.17                        xor	ecx, ecx
 1      1     0.25                        test	r9b, 64
 1      0     0.17                        mov	r9, r10
 1      1     0.50                        cmovne	r9, rcx
 1      1     0.50                        cmove	r10, r11
 1      0     0.17                        mov	r11, r10
 1      1     0.25                        not	r11
 1      0     0.17                        mov	rsi, r9
 1      1     0.25                        not	rsi
 1      1     0.25                        add	rdx, rsi
 1      1     1.00                        adc	r8, r11
 1      1     0.50                        jo	.LBB175_5
 1      1     0.25                        and	rdx, r9
 1      1     0.25                        and	r8, r10
 1      0     0.17                        mov	rcx, rdx
 1      1     1.00           *            mov	qword ptr [rax + 16], rcx
 1      1     1.00           *            mov	qword ptr [rax + 24], r8
 1      1     0.50                        mov	ecx, 1
 1      1     1.00           *            mov	qword ptr [rax], rcx
 1      1     1.00           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rsi
 1      5     0.50                  U     ret
 1      0     0.17                        xor	ecx, ecx
 1      1     0.25                        cmp	rdx, 1
 1      1     1.00                        sbb	r8, 0
 1      1     0.50                        mov	r8d, 0
 1      1     0.50                        jl	.LBB175_3
 1      1     1.00           *            mov	qword ptr [rax], rcx
 1      1     1.00           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
3.00   3.00   4.00   6.00   8.00   6.00   7.00   4.00    -      -      -      -      -      -      -     5.00   4.00   8.00   1.00   1.00   1.00   6.00   8.00   

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00   push	rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, r9b
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB175_4
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r10, cl
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, r10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r9, rcx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	r10, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, r10
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r11
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rsi, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	rsi
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rdx, rsi
 -      -      -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     adc	r8, r11
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jo	.LBB175_5
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	rdx, r9
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	r8, r10
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rcx, rdx
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -     mov	qword ptr [rax + 16], rcx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00   mov	qword ptr [rax + 24], r8
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -     mov	qword ptr [rax], rcx
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00   mov	qword ptr [rax + 8], 0
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -     pop	rsi
1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	rdx, 1
 -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sbb	r8, 0
 -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, 0
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jl	.LBB175_3
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -     mov	qword ptr [rax], rcx
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00   mov	qword ptr [rax + 8], 0
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -     1.00    -      -      -      -     pop	rsi
```
## `unbounded_ceil_to_multiple_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      16
Total Cycles:      11
Total uOps:        16

Dispatch Width:    6
uOps Per Cycle:    1.45
IPC:               1.45
Block RThroughput: 2.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      1     0.25                        cmp	dl, 8
 1      1     0.50                        jae	.LBB184_1
 1      1     1.00                        mov	r8b, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r8b, cl
 1      0     0.17                        mov	edx, r8d
 1      1     0.25                        not	dl
 1      1     0.25                        add	dl, al
 1      1     0.25                        cmp	dl, al
 1      1     1.00                        setae	al
 1      1     0.25                        and	dl, r8b
 1      5     0.50                  U     ret
 1      1     0.25                        test	al, al
 1      1     1.00                        sete	al
 1      0     0.17                        xor	edx, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   5.00   2.00   5.00   3.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 8
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB184_1
 -      -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8b, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r8b, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	dl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	dl, al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, al
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setae	al
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	dl, r8b
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	al, al
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sete	al
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	edx, edx
```
## `unbounded_ceil_to_multiple_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      18
Total Cycles:      13
Total uOps:        18

Dispatch Width:    6
uOps Per Cycle:    1.38
IPC:               1.38
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      1     0.25                        cmp	dl, 16
 1      1     0.50                        jae	.LBB181_1
 1      1     0.50                        mov	r8d, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r8d, cl
 1      0     0.17                        mov	edx, r8d
 1      1     0.25                        not	edx
 1      1     0.25                        add	edx, eax
 1      1     0.25                        cmp	dx, ax
 1      1     1.00                        setae	al
 1      1     0.25                        and	edx, r8d
 1      1     0.25                        movzx	eax, al
 1      5     0.50                  U     ret
 1      1     0.25                        test	ax, ax
 1      1     1.00                        sete	al
 1      0     0.17                        xor	edx, edx
 1      1     0.25                        movzx	eax, al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   5.00   3.00   4.00   3.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 16
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB181_1
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r8d, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	edx, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dx, ax
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setae	al
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	edx, r8d
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	ax, ax
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sete	al
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	edx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
```
## `unbounded_ceil_to_multiple_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      18
Total Cycles:      13
Total uOps:        18

Dispatch Width:    6
uOps Per Cycle:    1.38
IPC:               1.38
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      1     0.25                        cmp	dl, 32
 1      1     0.50                        jae	.LBB182_1
 1      1     0.50                        mov	r8d, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r8d, cl
 1      0     0.17                        mov	edx, r8d
 1      1     0.25                        not	edx
 1      1     0.25                        add	edx, eax
 1      1     0.25                        cmp	edx, eax
 1      1     1.00                        setae	al
 1      1     0.25                        and	edx, r8d
 1      1     0.25                        movzx	eax, al
 1      5     0.50                  U     ret
 1      1     0.25                        test	eax, eax
 1      1     1.00                        sete	al
 1      0     0.17                        xor	edx, edx
 1      1     0.25                        movzx	eax, al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   5.00   3.00   4.00   3.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 32
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB182_1
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r8d, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, r8d
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	edx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	edx, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	edx, eax
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setae	al
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	edx, r8d
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	eax, eax
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sete	al
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	edx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
```
## `unbounded_ceil_to_multiple_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      18
Total Cycles:      13
Total uOps:        18

Dispatch Width:    6
uOps Per Cycle:    1.38
IPC:               1.38
Block RThroughput: 3.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	rax, rcx
 1      1     0.25                        cmp	dl, 64
 1      1     0.50                        jae	.LBB183_1
 1      1     0.50                        mov	r8, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r8, cl
 1      0     0.17                        mov	rdx, r8
 1      1     0.25                        not	rdx
 1      1     0.25                        add	rdx, rax
 1      1     0.25                        cmp	rdx, rax
 1      1     1.00                        setae	al
 1      1     0.25                        and	rdx, r8
 1      1     0.25                        movzx	eax, al
 1      5     0.50                  U     ret
 1      1     0.25                        test	rax, rax
 1      1     1.00                        sete	al
 1      0     0.17                        xor	edx, edx
 1      1     0.25                        movzx	eax, al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   5.00   3.00   4.00   3.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 64
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB183_1
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r8, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r8
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rdx, rax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	rdx, rax
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setae	al
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	rdx, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	rax, rax
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sete	al
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	edx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
```
## `unbounded_ceil_to_multiple_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      37
Total Cycles:      17
Total uOps:        37

Dispatch Width:    6
uOps Per Cycle:    2.18
IPC:               2.18
Block RThroughput: 7.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     1.00           *            push	rsi
 1      0     0.17                        mov	rax, rcx
 1      1     0.25                        test	r9b, r9b
 1      1     0.50                        js	.LBB180_4
 1      1     0.50                        mov	r10, -1
 1      0     0.17                        mov	ecx, r9d
 1      1     0.50                        shl	r10, cl
 1      1     0.50                        mov	r11, -1
 1      0     0.17                        xor	ecx, ecx
 1      1     0.25                        test	r9b, 64
 1      0     0.17                        mov	r9, r10
 1      1     0.50                        cmovne	r9, rcx
 1      1     0.50                        cmove	r10, r11
 1      0     0.17                        mov	r11, r10
 1      1     0.25                        not	r11
 1      0     0.17                        mov	rsi, r9
 1      1     0.25                        not	rsi
 1      1     0.25                        add	rdx, rsi
 1      1     1.00                        adc	r8, r11
 1      1     0.50                        jb	.LBB180_5
 1      1     0.25                        and	rdx, r9
 1      1     0.25                        and	r8, r10
 1      0     0.17                        mov	rcx, rdx
 1      1     1.00           *            mov	qword ptr [rax + 16], rcx
 1      1     1.00           *            mov	qword ptr [rax + 24], r8
 1      1     0.50                        mov	ecx, 1
 1      1     1.00           *            mov	qword ptr [rax], rcx
 1      1     1.00           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rsi
 1      5     0.50                  U     ret
 1      0     0.17                        xor	ecx, ecx
 1      1     0.25                        or	rdx, r8
 1      1     0.50                        mov	r8d, 0
 1      1     0.50                        je	.LBB180_3
 1      1     1.00           *            mov	qword ptr [rax], rcx
 1      1     1.00           *            mov	qword ptr [rax + 8], 0
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
3.00   3.00   4.00   6.00   4.00   6.00   7.00   4.00    -      -      -      -      -      -      -     5.00   4.00   8.00   1.00   1.00   1.00   6.00   8.00   

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00   push	rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, r9b
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB180_4
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r10, cl
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, r10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r9, rcx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	r10, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, r10
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r11
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rsi, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	rsi
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rdx, rsi
 -      -      -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     adc	r8, r11
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jb	.LBB180_5
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	rdx, r9
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	r8, r10
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rcx, rdx
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -     mov	qword ptr [rax + 16], rcx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00   mov	qword ptr [rax + 24], r8
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, 1
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -     mov	qword ptr [rax], rcx
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00   mov	qword ptr [rax + 8], 0
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -     pop	rsi
1.00    -      -      -      -      -      -     1.00    -      -      -      -      -      -      -     1.00    -      -      -     1.00    -      -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     or	rdx, r8
 -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, 0
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     je	.LBB180_3
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -     mov	qword ptr [rax], rcx
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00   mov	qword ptr [rax + 8], 0
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -     1.00    -      -      -      -     pop	rsi
```
## `unbounded_div_ceil_i8_unb_pow2`
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
 1      0     0.17                        mov	eax, ecx
 1      1     0.25                        cmp	dl, 8
 1      1     0.50                        jae	.LBB189_1
 1      1     0.50                        mov	r8, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r8, cl
 1      1     0.25                        not	r8
 1      1     0.25                        movsx	rax, al
 1      1     0.25                        add	rax, r8
 1      1     0.50                        sar	rax, cl
 1      5     0.50                  U     ret
 1      1     0.25                        test	al, al
 1      1     1.00                        setg	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   2.00   2.00   3.00   4.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 8
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB189_1
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r8, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r8
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movsx	rax, al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, r8
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	al, al
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setg	al
```
## `unbounded_div_ceil_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      10
Total uOps:        14

Dispatch Width:    6
uOps Per Cycle:    1.40
IPC:               1.40
Block RThroughput: 2.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, ecx
 1      1     0.25                        cmp	dl, 16
 1      1     0.50                        jae	.LBB186_1
 1      1     0.50                        mov	r9, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r9, cl
 1      1     0.25                        not	r9
 1      1     0.25                        movsx	rax, r8w
 1      1     0.25                        add	rax, r9
 1      1     0.50                        sar	rax, cl
 1      5     0.50                  U     ret
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        test	r8w, r8w
 1      1     1.00                        setg	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   3.00   2.00   4.00   2.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 16
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB186_1
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r9, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r9
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movsx	rax, r8w
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8w, r8w
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setg	al
```
## `unbounded_div_ceil_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      10
Total uOps:        14

Dispatch Width:    6
uOps Per Cycle:    1.40
IPC:               1.40
Block RThroughput: 2.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, ecx
 1      1     0.25                        cmp	dl, 32
 1      1     0.50                        jae	.LBB187_1
 1      1     0.50                        mov	r9, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r9, cl
 1      1     0.25                        not	r9
 1      1     0.25                        movsxd	rax, r8d
 1      1     0.25                        add	rax, r9
 1      1     0.50                        sar	rax, cl
 1      5     0.50                  U     ret
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        test	r8d, r8d
 1      1     1.00                        setg	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   3.00   2.00   4.00   2.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 32
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB187_1
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r9, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r9
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movsxd	rax, r8d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8d, r8d
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setg	al
```
## `unbounded_div_ceil_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      16
Total Cycles:      10
Total uOps:        16

Dispatch Width:    6
uOps Per Cycle:    1.60
IPC:               1.60
Block RThroughput: 2.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        cmp	dl, 64
 1      1     0.50                        jae	.LBB188_1
 1      0     0.17                        mov	r8, rcx
 1      0     0.17                        mov	r9, rcx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        sar	r8, cl
 1      0     0.17                        mov	r10, r8
 1      1     0.50                        shl	r10, cl
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	r9, r10
 1      1     1.00                        setne	al
 1      1     0.25                        add	rax, r8
 1      5     0.50                  U     ret
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        test	rcx, rcx
 1      1     1.00                        setg	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   3.00   1.00   2.00   4.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 64
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB188_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	r8, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, r8
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r10, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	r9, r10
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setne	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, r8
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	rcx, rcx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setg	al
```
## `unbounded_div_ceil_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      46
Total Cycles:      30
Total uOps:        54

Dispatch Width:    6
uOps Per Cycle:    1.80
IPC:               1.53
Block RThroughput: 9.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     1.00           *            push	rsi
 1      1     1.00           *            push	rdi
 1      1     0.25                        test	r8b, r8b
 1      1     0.50                        js	.LBB185_1
 1      0     0.17                        mov	eax, r8d
 1      0     0.17                        mov	r9, rcx
 1      0     0.17                        mov	r10, rcx
 1      0     0.17                        mov	ecx, r8d
 5      2     1.50                        shrd	r9, rdx, cl
 1      0     0.17                        mov	r11, rdx
 1      1     0.50                        sar	r11, cl
 1      1     0.25                        test	r8b, 64
 1      1     0.50                        cmovne	r9, r11
 1      0     0.17                        mov	rsi, r9
 1      1     0.50                        shl	rsi, cl
 1      0     0.17                        xor	edi, edi
 1      1     0.25                        test	r8b, 64
 1      1     0.50                        cmove	rdi, rsi
 1      0     0.17                        mov	r8, rdx
 1      1     0.50                        sar	r8, 63
 1      1     0.25                        test	al, 64
 1      1     0.50                        cmove	r8, r11
 1      0     0.17                        mov	r11, r8
 5      2     1.50                        shld	r11, r9, cl
 1      1     0.25                        test	al, 64
 1      1     0.50                        cmovne	r11, rsi
 1      1     0.25                        xor	r11, rdx
 1      1     0.25                        xor	rdi, r10
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        or	rdi, r11
 1      1     1.00                        setne	al
 1      1     0.25                        add	rax, r9
 1      1     1.00                        adc	r8, 0
 1      0     0.17                        mov	rdx, r8
 1      5     0.33    *                   pop	rdi
 1      5     0.33    *                   pop	rsi
 1      5     0.50                  U     ret
 1      0     0.17                        xor	r8d, r8d
 1      1     0.25                        neg	rcx
 1      1     0.50                        mov	eax, 0
 1      1     1.00                        sbb	rax, rdx
 1      1     1.00                        setl	al
 1      1     0.25                        movzx	eax, al
 1      0     0.17                        mov	rdx, r8
 1      5     0.33    *                   pop	rdi
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
2.00   2.00   3.00   8.00   12.00  9.00   9.00   2.00    -      -      -      -      -      -      -     2.00   3.00   4.00   1.00   2.00   2.00   2.00   2.00   

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00   push	rsi
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -     push	rdi
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, r8b
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB185_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, r8d
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shrd	r9, rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, rdx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	r11, cl
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r9, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rsi, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	rsi, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	edi, edi
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	rdi, rsi
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rdx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	r8, 63
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	al, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	r8, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, r8
 -      -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shld	r11, r9, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	al, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r11, rsi
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	r11, rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	rdi, r10
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     or	rdi, r11
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setne	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, r9
 -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     adc	r8, 0
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r8
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -     pop	rdi
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -     1.00    -      -      -      -     pop	rsi
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -     1.00    -      -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	r8d, r8d
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     neg	rcx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, 0
 -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sbb	rax, rdx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setl	al
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, r8
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -     pop	rdi
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -     1.00    -      -      -     pop	rsi
```
## `unbounded_div_ceil_u8_unb_pow2`
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
 1      0     0.17                        mov	eax, ecx
 1      1     0.25                        cmp	dl, 8
 1      1     0.50                        jae	.LBB194_1
 1      1     0.50                        mov	r8, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r8, cl
 1      1     0.25                        not	r8
 1      1     0.25                        movzx	eax, al
 1      1     0.25                        add	rax, r8
 1      1     0.50                        shr	rax, cl
 1      5     0.50                  U     ret
 1      1     0.25                        test	al, al
 1      1     1.00                        setne	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   2.00   2.00   3.00   4.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 8
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB194_1
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r8, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r8
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, r8
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	rax, cl
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	al, al
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setne	al
```
## `unbounded_div_ceil_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      10
Total uOps:        14

Dispatch Width:    6
uOps Per Cycle:    1.40
IPC:               1.40
Block RThroughput: 2.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, ecx
 1      1     0.25                        cmp	dl, 16
 1      1     0.50                        jae	.LBB191_1
 1      1     0.50                        mov	r9, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r9, cl
 1      1     0.25                        not	r9
 1      1     0.25                        movzx	eax, r8w
 1      1     0.25                        add	rax, r9
 1      1     0.50                        shr	rax, cl
 1      5     0.50                  U     ret
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        test	r8w, r8w
 1      1     1.00                        setne	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   3.00   2.00   4.00   2.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 16
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB191_1
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r9, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r9
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, r8w
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	rax, cl
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8w, r8w
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setne	al
```
## `unbounded_div_ceil_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      14
Total Cycles:      9
Total uOps:        14

Dispatch Width:    6
uOps Per Cycle:    1.56
IPC:               1.56
Block RThroughput: 2.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, ecx
 1      1     0.25                        cmp	dl, 32
 1      1     0.50                        jae	.LBB192_1
 1      1     0.50                        mov	r9, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r9, cl
 1      1     0.25                        not	r9
 1      0     0.17                        mov	eax, r8d
 1      1     0.25                        add	rax, r9
 1      1     0.50                        shr	rax, cl
 1      5     0.50                  U     ret
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        test	r8d, r8d
 1      1     1.00                        setne	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   3.00   2.00   4.00   2.00   1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 32
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB192_1
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r9, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r9
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, r8d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	rax, cl
 -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8d, r8d
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setne	al
```
## `unbounded_div_ceil_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      16
Total Cycles:      10
Total uOps:        16

Dispatch Width:    6
uOps Per Cycle:    1.60
IPC:               1.60
Block RThroughput: 2.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        cmp	dl, 64
 1      1     0.50                        jae	.LBB193_1
 1      0     0.17                        mov	r8, rcx
 1      0     0.17                        mov	r9, rcx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	r8, cl
 1      0     0.17                        mov	r10, r8
 1      1     0.50                        shl	r10, cl
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	r9, r10
 1      1     1.00                        setne	al
 1      1     0.25                        add	rax, r8
 1      5     0.50                  U     ret
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        test	rcx, rcx
 1      1     1.00                        setne	al

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   3.00   1.00   2.00   4.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 64
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB193_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	r8, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, r8
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r10, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	r9, r10
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setne	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, r8
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	rcx, rcx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setne	al
```
## `unbounded_div_ceil_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      38
Total Cycles:      29
Total uOps:        46

Dispatch Width:    6
uOps Per Cycle:    1.59
IPC:               1.31
Block RThroughput: 7.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     1.00           *            push	rsi
 1      1     1.00           *            push	rdi
 1      1     0.25                        test	r8b, r8b
 1      1     0.50                        js	.LBB190_1
 1      0     0.17                        mov	r9, rcx
 1      0     0.17                        mov	r10, rcx
 1      0     0.17                        mov	ecx, r8d
 5      2     1.50                        shrd	r9, rdx, cl
 1      0     0.17                        mov	rax, rdx
 1      1     0.50                        shr	rdx, cl
 1      1     0.25                        test	r8b, 64
 1      1     0.50                        cmovne	r9, rdx
 1      0     0.17                        mov	r11, r9
 1      1     0.50                        shl	r11, cl
 1      0     0.17                        xor	esi, esi
 1      1     0.25                        test	r8b, 64
 1      1     0.50                        cmovne	rdx, rsi
 1      1     0.50                        cmove	rsi, r11
 1      0     0.17                        mov	rdi, rdx
 5      2     1.50                        shld	rdi, r9, cl
 1      1     0.25                        test	r8b, 64
 1      1     0.50                        cmovne	rdi, r11
 1      1     0.25                        xor	rdi, rax
 1      1     0.25                        xor	rsi, r10
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        or	rsi, rdi
 1      1     1.00                        setne	al
 1      1     0.25                        add	rax, r9
 1      1     1.00                        adc	rdx, 0
 1      5     0.33    *                   pop	rdi
 1      5     0.33    *                   pop	rsi
 1      5     0.50                  U     ret
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        or	rcx, rdx
 1      1     1.00                        setne	al
 1      0     0.17                        xor	edx, edx
 1      5     0.33    *                   pop	rdi
 1      5     0.33    *                   pop	rsi

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
2.00   2.00   3.00   5.00   8.00   8.00   8.00   2.00    -      -      -      -      -      -      -     2.00   3.00   4.00   1.00   2.00   2.00   2.00   2.00   

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00   push	rsi
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -     push	rdi
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, r8b
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB190_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shrd	r9, rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rdx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	rdx, cl
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r9, rdx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r11, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	esi, esi
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rdx, rsi
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	rsi, r11
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdi, rdx
 -      -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shld	rdi, r9, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rdi, r11
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	rdi, rax
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	rsi, r10
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     or	rsi, rdi
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setne	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, r9
 -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     adc	rdx, 0
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -     pop	rdi
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -     1.00    -      -      -      -     pop	rsi
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -     1.00    -      -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     or	rcx, rdx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setne	al
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	edx, edx
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -      -      -     1.00    -      -     pop	rdi
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -     1.00    -      -      -     pop	rsi
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
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      1     0.25                        movzx	edx, dl
 1      1     0.25                        cmp	dl, 7
 1      1     0.50                        mov	ecx, 7
 1      1     0.50                        cmovb	ecx, edx
 1      1     0.50                        sar	al, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   3.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	edx, dl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 7
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, 7
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovb	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	al, cl
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
Block RThroughput: 1.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        movsx	eax, cx
 1      1     0.25                        movzx	edx, dl
 1      1     0.25                        cmp	dl, 15
 1      1     0.50                        mov	ecx, 15
 1      1     0.50                        cmovb	ecx, edx
 1      1     0.50                        sar	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   2.00   2.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movsx	eax, cx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	edx, dl
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 15
 -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, 15
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovb	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	eax, cl
```
## `unbounded_div_floor_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      7
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    0.86
IPC:               0.86
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      1     0.25                        movzx	edx, dl
 1      1     0.25                        cmp	dl, 31
 1      1     0.50                        mov	ecx, 31
 1      1     0.50                        cmovb	ecx, edx
 1      1     0.50                        sar	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   3.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	edx, dl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 31
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, 31
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovb	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	eax, cl
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
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	rax, rcx
 1      1     0.25                        movzx	edx, dl
 1      1     0.25                        cmp	dl, 63
 1      1     0.50                        mov	ecx, 63
 1      1     0.50                        cmovb	ecx, edx
 1      1     0.50                        sar	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   3.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	edx, dl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 63
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, 63
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovb	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
```
## `unbounded_div_floor_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      12
Total Cycles:      8
Total uOps:        16

Dispatch Width:    6
uOps Per Cycle:    2.00
IPC:               1.50
Block RThroughput: 2.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	rax, rcx
 1      1     0.25                        test	r8b, r8b
 1      1     0.25                        movzx	r8d, r8b
 1      1     0.50                        mov	ecx, 127
 1      1     0.50                        cmovns	ecx, r8d
 5      2     1.50                        shrd	rax, rdx, cl
 1      0     0.17                        mov	r8, rdx
 1      1     0.50                        sar	r8, cl
 1      1     0.50                        sar	rdx, 63
 1      1     0.25                        test	cl, 64
 1      1     0.50                        cmovne	rax, r8
 1      1     0.50                        cmove	rdx, r8

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     2.00   4.00   4.00   3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, r8b
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	r8d, r8b
 -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, 127
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovns	ecx, r8d
 -      -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shrd	rax, rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rdx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	r8, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	rdx, 63
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	cl, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rax, r8
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	rdx, r8
```
## `unbounded_div_floor_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      6
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    1.17
IPC:               1.17
Block RThroughput: 1.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	al, cl
 1      0     0.17                        xor	ecx, ecx
 1      1     0.25                        cmp	dl, 8
 1      1     0.25                        movzx	eax, al
 1      1     0.50                        cmovae	eax, ecx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	al, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 8
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovae	eax, ecx
```
## `unbounded_div_floor_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      6
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        movzx	r8d, cx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	r8d, cl
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	dl, 16
 1      1     0.50                        cmovb	eax, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	r8d, cx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	r8d, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 16
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovb	eax, r8d
```
## `unbounded_div_floor_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      5
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    1.20
IPC:               1.20
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, ecx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	r8d, cl
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	dl, 32
 1      1     0.50                        cmovb	eax, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00    -     1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	r8d, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 32
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovb	eax, r8d
```
## `unbounded_div_floor_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      6
Total Cycles:      5
Total uOps:        6

Dispatch Width:    6
uOps Per Cycle:    1.20
IPC:               1.20
Block RThroughput: 1.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8, rcx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	r8, cl
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	dl, 64
 1      1     0.50                        cmovb	rax, r8

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00    -     1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	r8, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovb	rax, r8
```
## `unbounded_div_floor_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      11
Total Cycles:      8
Total uOps:        15

Dispatch Width:    6
uOps Per Cycle:    1.88
IPC:               1.38
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	rax, rcx
 1      0     0.17                        mov	ecx, r8d
 5      2     1.50                        shrd	rax, rdx, cl
 1      1     0.50                        shr	rdx, cl
 1      0     0.17                        xor	ecx, ecx
 1      1     0.25                        test	r8b, 64
 1      1     0.50                        cmovne	rax, rdx
 1      1     0.50                        cmovne	rdx, rcx
 1      1     0.25                        test	r8b, r8b
 1      1     0.50                        cmovs	rax, rcx
 1      1     0.50                        cmovs	rdx, rcx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     3.00   1.00   3.00   3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shrd	rax, rdx, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rax, rdx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rdx, rcx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, r8b
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovs	rax, rcx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovs	rdx, rcx
```
## `unbounded_div_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      16
Total Cycles:      10
Total uOps:        16

Dispatch Width:    6
uOps Per Cycle:    1.60
IPC:               1.60
Block RThroughput: 2.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        cmp	dl, 7
 1      1     0.50                        ja	.LBB209_1
 1      0     0.17                        mov	eax, ecx
 1      1     0.25                        test	cl, cl
 1      1     0.50                        js	.LBB209_4
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	al, cl
 1      5     0.50                  U     ret
 1      0     0.17                        xor	eax, eax
 1      5     0.50                  U     ret
 1      1     1.00                        mov	r8b, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	r8b, cl
 1      1     0.25                        not	r8b
 1      1     0.25                        add	al, r8b
 1      1     0.50                        sar	al, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -     1.00   1.00   2.00   3.00   3.00   5.00   2.00    -      -      -      -      -      -      -      -     1.00   1.00    -     1.00   1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 7
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB209_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	cl, cl
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB209_4
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	al, cl
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -     1.00    -     1.00    -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -     ret
 -      -      -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8b, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r8b, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r8b
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	al, r8b
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	al, cl
```
## `unbounded_div_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      17
Total Cycles:      11
Total uOps:        17

Dispatch Width:    6
uOps Per Cycle:    1.55
IPC:               1.55
Block RThroughput: 2.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        cmp	dl, 15
 1      1     0.50                        ja	.LBB206_1
 1      0     0.17                        mov	eax, ecx
 1      1     0.25                        movzx	ecx, dl
 1      1     0.25                        test	ax, ax
 1      1     0.50                        js	.LBB206_4
 1      1     0.25                        movzx	eax, ax
 1      1     0.50                        shr	eax, cl
 1      5     0.50                  U     ret
 1      0     0.17                        xor	eax, eax
 1      5     0.50                  U     ret
 1      1     0.50                        mov	edx, -1
 1      1     0.50                        shl	edx, cl
 1      1     0.25                        not	edx
 1      1     0.25                        add	eax, edx
 1      1     0.25                        cwde
 1      1     0.50                        sar	eax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -     1.00   1.00   2.00   3.00   4.00   4.00   3.00    -      -      -      -      -      -      -      -     1.00   1.00    -     1.00   1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 15
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB206_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	ecx, dl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	ax, ax
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB206_4
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, ax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	eax, cl
 -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -     ret
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, -1
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	edx, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	eax, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cwde
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	eax, cl
```
## `unbounded_div_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      16
Total Cycles:      9
Total uOps:        16

Dispatch Width:    6
uOps Per Cycle:    1.78
IPC:               1.78
Block RThroughput: 2.7

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	dl, 31
 1      1     0.50                        ja	.LBB207_4
 1      0     0.17                        mov	r8d, ecx
 1      1     0.25                        test	ecx, ecx
 1      1     0.50                        js	.LBB207_2
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	r8d, cl
 1      1     0.50                        jmp	.LBB207_3
 1      1     0.50                        mov	eax, -1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shl	eax, cl
 1      1     0.25                        not	eax
 1      1     0.25                        add	r8d, eax
 1      1     0.50                        sar	r8d, cl
 1      0     0.17                        mov	eax, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     2.00   2.00   3.00   3.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 31
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB207_4
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	ecx, ecx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB207_2
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	r8d, cl
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jmp	.LBB207_3
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	eax, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	eax
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	r8d, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	r8d, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, r8d
```
## `unbounded_div_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      10
Total uOps:        15

Dispatch Width:    6
uOps Per Cycle:    1.50
IPC:               1.50
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        cmp	dl, 63
 1      1     0.50                        ja	.LBB208_1
 1      0     0.17                        mov	rax, rcx
 1      1     0.25                        movzx	ecx, dl
 1      1     0.25                        test	rax, rax
 1      1     0.50                        js	.LBB208_4
 1      1     0.50                        shr	rax, cl
 1      5     0.50                  U     ret
 1      0     0.17                        xor	eax, eax
 1      5     0.50                  U     ret
 1      1     0.50                        mov	rdx, -1
 1      1     0.50                        shl	rdx, cl
 1      1     0.25                        not	rdx
 1      1     0.25                        add	rax, rdx
 1      1     0.50                        sar	rax, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -     1.00   1.00   2.00   3.00   3.00   4.00   2.00    -      -      -      -      -      -      -      -     1.00   1.00    -     1.00   1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 63
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     ja	.LBB208_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	ecx, dl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	rax, rax
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB208_4
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	rax, cl
 -      -     1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -     1.00    -     1.00    -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -     ret
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, -1
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	rdx, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, rdx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	rax, cl
```
## `unbounded_div_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      37
Total Cycles:      18
Total uOps:        45

Dispatch Width:    6
uOps Per Cycle:    2.50
IPC:               2.06
Block RThroughput: 7.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        test	r8b, r8b
 1      1     0.50                        js	.LBB205_1
 1      0     0.17                        mov	rax, rcx
 1      1     0.25                        test	rdx, rdx
 1      1     0.50                        js	.LBB205_4
 1      0     0.17                        mov	ecx, r8d
 5      2     1.50                        shrd	rax, rdx, cl
 1      1     0.50                        shr	rdx, cl
 1      0     0.17                        xor	ecx, ecx
 1      1     0.25                        test	r8b, 64
 1      1     0.50                        cmovne	rax, rdx
 1      1     0.50                        cmovne	rdx, rcx
 1      5     0.50                  U     ret
 1      0     0.17                        xor	eax, eax
 1      0     0.17                        xor	edx, edx
 1      5     0.50                  U     ret
 1      1     0.50                        mov	r9, -1
 1      1     0.50                        mov	r10, -1
 1      0     0.17                        mov	ecx, r8d
 1      1     0.50                        shl	r10, cl
 1      0     0.17                        xor	ecx, ecx
 1      1     0.25                        test	r8b, 64
 1      1     0.50                        cmove	rcx, r10
 1      1     0.50                        cmove	r10, r9
 1      1     0.25                        not	r10
 1      1     0.25                        not	rcx
 1      1     0.25                        add	rax, rcx
 1      1     1.00                        adc	rdx, r10
 1      0     0.17                        mov	ecx, r8d
 5      2     1.50                        shrd	rax, rdx, cl
 1      0     0.17                        mov	r9, rdx
 1      1     0.50                        sar	r9, cl
 1      1     0.25                        test	r8b, 64
 1      1     0.50                        cmovne	rax, r9
 1      1     0.50                        sar	rdx, 63
 1      1     0.25                        test	r8b, 64
 1      1     0.50                        cmove	rdx, r9

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -     1.00   1.00   6.00   10.00  11.00  7.00   3.00    -      -      -      -      -      -      -      -     1.00   1.00    -     1.00   1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, r8b
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB205_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	rdx, rdx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB205_4
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shrd	rax, rdx, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rax, rdx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rdx, rcx
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	edx, edx
 -     1.00    -      -      -      -      -     1.00    -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -     ret
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, -1
 -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r10, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	rcx, r10
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	r10, r9
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	r10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	rax, rcx
 -      -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     adc	rdx, r10
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -     3.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shrd	rax, rdx, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rdx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	r9, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rax, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sar	rdx, 63
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	rdx, r9
```
## `unbounded_floor_to_multiple_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      13
Total Cycles:      7
Total uOps:        13

Dispatch Width:    6
uOps Per Cycle:    1.86
IPC:               1.86
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      0     0.17                        mov	r8d, ecx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	r8b, cl
 1      1     0.50                        shl	r8b, cl
 1      1     0.25                        test	al, al
 1      1     1.00                        setns	cl
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	dl, 8
 1      1     0.25                        movzx	edx, r8b
 1      1     0.50                        cmovae	edx, eax
 1      1     1.00                        setb	al
 1      1     0.25                        or	al, cl

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     3.00   2.00   2.00   4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	r8b, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r8b, cl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	al, al
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setns	cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 8
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	edx, r8b
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovae	edx, eax
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setb	al
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     or	al, cl
```
## `unbounded_floor_to_multiple_i16_unb_pow2`
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
 1      1     0.25                        cmp	dl, 16
 1      1     0.50                        jae	.LBB211_1
 1      0     0.17                        mov	eax, edx
 1      1     0.25                        movzx	edx, cx
 1      0     0.17                        mov	ecx, eax
 1      1     0.50                        shr	edx, cl
 1      1     0.50                        shl	edx, cl
 1      1     1.00                        mov	ax, 1
 1      5     0.50                  U     ret
 1      1     0.25                        not	ecx
 1      1     0.25                        movzx	eax, cx
 1      1     0.50                        shr	eax, 15
 1      0     0.17                        xor	edx, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   4.00   2.00   3.00   2.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 16
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB211_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	edx, cx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, eax
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	edx, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	edx, cl
 -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ax, 1
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	ecx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, cx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	eax, 15
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	edx, edx
```
## `unbounded_floor_to_multiple_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      12
Total Cycles:      10
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.20
IPC:               1.20
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      1     0.25                        cmp	dl, 32
 1      1     0.50                        jae	.LBB212_1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	eax, cl
 1      1     0.50                        shl	eax, cl
 1      0     0.17                        mov	edx, eax
 1      1     0.50                        mov	eax, 1
 1      5     0.50                  U     ret
 1      1     0.25                        not	eax
 1      1     0.50                        shr	eax, 31
 1      0     0.17                        xor	edx, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   2.00   1.00   2.00   2.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 32
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB212_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	eax, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	eax, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	edx, eax
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, 1
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	eax, 31
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	edx, edx
```
## `unbounded_floor_to_multiple_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      12
Total Cycles:      10
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.20
IPC:               1.20
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	rax, rcx
 1      1     0.25                        cmp	dl, 64
 1      1     0.50                        jae	.LBB213_1
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	rax, cl
 1      1     0.50                        shl	rax, cl
 1      0     0.17                        mov	rdx, rax
 1      1     0.50                        mov	eax, 1
 1      5     0.50                  U     ret
 1      1     0.25                        not	rax
 1      1     0.50                        shr	rax, 63
 1      0     0.17                        xor	edx, edx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   2.00   1.00   2.00   2.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 64
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     jae	.LBB213_1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	rax, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	rax, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rdx, rax
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, 1
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     not	rax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	rax, 63
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	edx, edx
```
## `unbounded_floor_to_multiple_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      26
Total Cycles:      15
Total uOps:        26

Dispatch Width:    6
uOps Per Cycle:    1.73
IPC:               1.73
Block RThroughput: 7.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	rax, rcx
 1      1     0.25                        test	r9b, r9b
 1      1     0.50                        js	.LBB210_1
 1      1     0.50                        mov	r10, -1
 1      1     0.50                        mov	r11, -1
 1      0     0.17                        mov	ecx, r9d
 1      1     0.50                        shl	r11, cl
 1      0     0.17                        xor	ecx, ecx
 1      1     0.25                        test	r9b, 64
 1      1     0.50                        cmovne	r10, r11
 1      1     0.50                        cmove	rcx, r11
 1      1     0.25                        and	rcx, rdx
 1      1     0.25                        and	r10, r8
 1      1     1.00           *            mov	qword ptr [rax + 24], r10
 1      1     1.00           *            mov	qword ptr [rax + 16], rcx
 1      1     1.00           *            mov	qword ptr [rax + 8], 0
 1      1     1.00           *            mov	qword ptr [rax], 1
 1      5     0.50                  U     ret
 1      1     0.25                        xorps	xmm0, xmm0
 1      1     0.25                        test	r8, r8
 1      1     0.50                        js	.LBB210_2
 1      1     1.00           *            movups	xmmword ptr [rax + 8], xmm0
 1      1     1.00           *            mov	qword ptr [rax], 1
 1      1     1.00           *            mov	qword ptr [rax + 24], 0
 1      5     0.50                  U     ret
 1      1     1.00           *            movaps	xmmword ptr [rax], xmm0

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
2.00   3.00   3.00   3.00   3.00   4.00   3.00   3.00    -      -      -     1.00   1.00   1.00   2.00   6.00   5.00   5.00    -     1.00   1.00   7.00   7.00   

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, r9b
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB210_1
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, -1
 -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r9d
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r11, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r9b, 64
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	r10, r11
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	rcx, r11
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	rcx, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	r10, r8
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -      -     2.00   mov	qword ptr [rax + 24], r10
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -     2.00    -     mov	qword ptr [rax + 16], rcx
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00   mov	qword ptr [rax + 8], 0
1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -     2.00    -     mov	qword ptr [rax], 1
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -      -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -     xorps	xmm0, xmm0
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8, r8
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     js	.LBB210_2
 -      -      -      -      -      -      -      -      -      -      -      -      -     1.00   1.00    -      -     1.00    -      -      -      -     1.00   movups	xmmword ptr [rax + 8], xmm0
 -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -     2.00    -     mov	qword ptr [rax], 1
 -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -     2.00    -      -      -      -      -      -     2.00   mov	qword ptr [rax + 24], 0
 -     1.00    -     1.00    -      -      -      -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -      -     ret
 -      -      -      -      -      -      -      -      -      -      -      -     1.00    -     1.00    -      -     1.00    -      -      -     1.00    -     movaps	xmmword ptr [rax], xmm0
```
## `unbounded_floor_to_multiple_u8_unb_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      7
Total uOps:        8

Dispatch Width:    6
uOps Per Cycle:    1.14
IPC:               1.14
Block RThroughput: 1.3

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	eax, ecx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	al, cl
 1      1     0.50                        shl	al, cl
 1      0     0.17                        xor	ecx, ecx
 1      1     0.25                        cmp	dl, 8
 1      1     0.25                        movzx	eax, al
 1      1     0.50                        cmovae	eax, ecx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   1.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	al, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	al, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 8
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, al
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovae	eax, ecx
```
## `unbounded_floor_to_multiple_u16_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      7
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    1.00
IPC:               1.00
Block RThroughput: 1.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        movzx	r8d, cx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	r8d, cl
 1      1     0.50                        shl	r8d, cl
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	dl, 16
 1      1     0.50                        cmovb	eax, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   2.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	r8d, cx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	r8d, cl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r8d, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 16
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovb	eax, r8d
```
## `unbounded_floor_to_multiple_u32_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      6
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    1.17
IPC:               1.17
Block RThroughput: 1.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8d, ecx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	r8d, cl
 1      1     0.50                        shl	r8d, cl
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	dl, 32
 1      1     0.50                        cmovb	eax, r8d

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8d, ecx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	r8d, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r8d, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 32
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovb	eax, r8d
```
## `unbounded_floor_to_multiple_u64_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      6
Total uOps:        7

Dispatch Width:    6
uOps Per Cycle:    1.17
IPC:               1.17
Block RThroughput: 1.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r8, rcx
 1      0     0.17                        mov	ecx, edx
 1      1     0.50                        shr	r8, cl
 1      1     0.50                        shl	r8, cl
 1      0     0.17                        xor	eax, eax
 1      1     0.25                        cmp	dl, 64
 1      1     0.50                        cmovb	rax, r8

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     1.00   1.00   1.00   1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r8, rcx
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, edx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shr	r8, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r8, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	dl, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovb	rax, r8
```
## `unbounded_floor_to_multiple_u128_unb_pow2`
```asm
Iterations:        1
Instructions:      15
Total Cycles:      9
Total uOps:        15

Dispatch Width:    6
uOps Per Cycle:    1.67
IPC:               1.67
Block RThroughput: 2.5

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	r9, rcx
 1      1     0.50                        mov	r10, -1
 1      0     0.17                        mov	ecx, r8d
 1      1     0.50                        shl	r10, cl
 1      0     0.17                        xor	ecx, ecx
 1      1     0.25                        test	r8b, 64
 1      0     0.17                        mov	rax, r10
 1      1     0.50                        cmovne	rax, rcx
 1      1     0.50                        mov	r11, -1
 1      1     0.50                        cmove	r10, r11
 1      1     0.25                        and	rdx, r10
 1      1     0.25                        and	rax, r9
 1      1     0.25                        test	r8b, r8b
 1      1     0.50                        cmovs	rax, rcx
 1      1     0.50                        cmovs	rdx, rcx

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     4.00   2.00   2.00   5.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r9, rcx
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r10, -1
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	ecx, r8d
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     shl	r10, cl
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     xor	ecx, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, 64
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, r10
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	rax, rcx
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	r11, -1
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmove	r10, r11
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	rdx, r10
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     and	rax, r9
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	r8b, r8b
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovs	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovs	rdx, rcx
```
## `unbounded_is_multiple_of_i8_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      8
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.50
IPC:               0.88
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        test	cl, cl
 1      1     1.00                        sete	r8b
 1      1     0.25                        movzx	eax, cl
 6      1     1.00                        rep		bsf	eax, eax
 1      1     0.25                        cmp	al, dl
 1      1     1.00                        setae	al
 1      1     0.25                        or	al, r8b

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     4.00   1.00   2.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	cl, cl
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sete	r8b
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, cl
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     rep		bsf	eax, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	al, dl
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setae	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     or	al, r8b
```
## `unbounded_is_multiple_of_i16_unb_pow2`
```asm
Iterations:        1
Instructions:      7
Total Cycles:      9
Total uOps:        12

Dispatch Width:    6
uOps Per Cycle:    1.33
IPC:               0.78
Block RThroughput: 2.0

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        test	cx, cx
 1      1     1.00                        sete	r8b
 6      1     1.00                        rep		bsf	eax, ecx
 1      1     0.25                        movzx	ecx, dl
 1      1     0.25                        cmp	ax, cx
 1      1     1.00                        setae	al
 1      1     0.25                        or	al, r8b

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     4.00   1.00   2.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	cx, cx
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sete	r8b
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     rep		bsf	eax, ecx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	ecx, dl
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	ax, cx
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setae	al
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     or	al, r8b
```
## `unbounded_is_multiple_of_i32_unb_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      9
Total uOps:        13

Dispatch Width:    6
uOps Per Cycle:    1.44
IPC:               0.89
Block RThroughput: 2.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        test	ecx, ecx
 1      1     1.00                        sete	r8b
 1      1     0.50                        mov	eax, 32
 6      1     1.00                        rep		bsf	eax, ecx
 1      1     0.25                        movzx	ecx, dl
 1      1     0.25                        cmp	eax, ecx
 1      1     1.00                        setae	al
 1      1     0.25                        or	al, r8b

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     4.00   2.00   3.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	ecx, ecx
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sete	r8b
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, 32
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     rep		bsf	eax, ecx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	ecx, dl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	eax, ecx
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setae	al
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     or	al, r8b
```
## `unbounded_is_multiple_of_i64_unb_pow2`
```asm
Iterations:        1
Instructions:      8
Total Cycles:      9
Total uOps:        13

Dispatch Width:    6
uOps Per Cycle:    1.44
IPC:               0.89
Block RThroughput: 2.2

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      1     0.25                        test	rcx, rcx
 1      1     1.00                        sete	r8b
 1      1     0.50                        mov	eax, 64
 6      1     1.00                        rep		bsf	rax, rcx
 1      1     0.25                        movzx	ecx, dl
 1      1     0.25                        cmp	eax, ecx
 1      1     1.00                        setae	al
 1      1     0.25                        or	al, r8b

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -      -     4.00   2.00   3.00   2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	rcx, rcx
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     sete	r8b
 -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	eax, 64
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     rep		bsf	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	ecx, dl
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	eax, ecx
 -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setae	al
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     or	al, r8b
```
## `unbounded_is_multiple_of_i128_unb_pow2`
```asm
Iterations:        1
Instructions:      13
Total Cycles:      12
Total uOps:        23

Dispatch Width:    6
uOps Per Cycle:    1.92
IPC:               1.08
Block RThroughput: 3.8

Instruction Info:

[1]    [2]    [3]    [4]    [5]    [6]    Instructions:
 1      0     0.17                        mov	rax, rcx
 1      1     0.25                        or	rax, rdx
 1      1     0.50                        je	.LBB220_1
 6      1     1.00                        rep		bsf	rax, rcx
 6      1     1.00                        rep		bsf	rdx, rdx
 1      1     0.25                        add	edx, 64
 1      1     0.25                        test	rcx, rcx
 1      1     0.50                        cmovne	edx, eax
 1      1     0.25                        movzx	eax, r8b
 1      1     0.25                        cmp	edx, eax
 1      1     1.00                        setae	al
 1      5     0.50                  U     ret
 1      1     1.00                        mov	al, 1

Resources:

Resource pressure per iteration:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] 
 -      -     1.00   2.00   6.00   2.00   4.00   2.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     

Resource pressure by instruction:
[0]    [1]    [2]    [3]    [4]    [5]    [6]    [7]    [8]    [9]    [10]   [11]   [12.0] [12.1] [13]   [14.0] [14.1] [14.2] [15.0] [15.1] [15.2] [16.0] [16.1] Instructions:
 -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	rax, rcx
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     or	rax, rdx
 -      -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     je	.LBB220_1
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     rep		bsf	rax, rcx
 -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     rep		bsf	rdx, rdx
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     add	edx, 64
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     test	rcx, rcx
 -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmovne	edx, eax
 -      -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     movzx	eax, r8b
 -      -      -      -      -     1.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     cmp	edx, eax
 -      -      -      -      -      -     2.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     setae	al
 -      -     1.00    -      -      -      -     1.00    -      -      -      -      -      -      -      -      -     1.00    -      -     1.00    -      -     ret
 -      -      -      -     4.00    -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -      -     mov	al, 1
```
