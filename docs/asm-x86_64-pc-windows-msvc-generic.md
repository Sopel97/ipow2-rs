## `ceil_to_multiple_i8_pow2`, `ceil_to_multiple_u8_pow2`
```asm
	mov r8d, ecx
	mov r9b, -1
	mov ecx, edx
	shl r9b, cl
	mov eax, r9d
	not al
	add al, r8b
	and al, r9b
```
## `ceil_to_multiple_i8_unb_pow2`, `ceil_to_multiple_u8_unb_pow2`
```asm
	mov r8d, ecx
	and dl, 7
	mov r9b, -1
	mov ecx, edx
	shl r9b, cl
	mov eax, r9d
	not al
	add al, r8b
	and al, r9b
```
## `ceil_to_multiple_i16_pow2`, `ceil_to_multiple_i32_pow2`, `ceil_to_multiple_i32_unb_pow2`, `ceil_to_multiple_u16_pow2`, `ceil_to_multiple_u32_pow2`, `ceil_to_multiple_u32_unb_pow2`
```asm
	mov r8d, ecx
	mov r9d, -1
	mov ecx, edx
	shl r9d, cl
	mov eax, r9d
	not eax
	add eax, r8d
	and eax, r9d
```
## `ceil_to_multiple_i16_unb_pow2`, `ceil_to_multiple_u16_unb_pow2`
```asm
	mov r8d, ecx
	and dl, 15
	mov r9d, -1
	mov ecx, edx
	shl r9d, cl
	mov eax, r9d
	not eax
	add eax, r8d
	and eax, r9d
```
## `ceil_to_multiple_i64_pow2`, `ceil_to_multiple_i64_unb_pow2`, `ceil_to_multiple_u64_pow2`, `ceil_to_multiple_u64_unb_pow2`
```asm
	mov r8, rcx
	mov r9, -1
	mov ecx, edx
	shl r9, cl
	mov rax, r9
	not rax
	add rax, r8
	and rax, r9
```
## `ceil_to_multiple_i128_pow2`, `ceil_to_multiple_i128_unb_pow2`, `ceil_to_multiple_u128_pow2`, `ceil_to_multiple_u128_unb_pow2`
```asm
	mov r9, rcx
	mov r10, -1
	mov ecx, r8d
	shl r10, cl
	mov rax, -1
	xor ecx, ecx
	test r8b, 64
	cmove rcx, r10
	cmove r10, rax
	mov r8, r10
	not r8
	mov rax, rcx
	not rax
	add rax, r9
	adc rdx, r8
	and rax, rcx
	and rdx, r10
```
## `checked_ceil_to_multiple_i8_pow2`
```asm
	mov eax, ecx
	mov r8b, -1
	mov ecx, edx
	shl r8b, cl
	mov edx, r8d
	not dl
	add dl, al
	setno al
	and dl, r8b
```
## `checked_ceil_to_multiple_i8_unb_pow2`
```asm
	cmp dl, 7
	ja .LBB34_1
	mov r8d, ecx
	mov r9b, -1
	mov ecx, edx
	shl r9b, cl
	mov eax, r9d
	not al
	add r8b, al
	setno al
	and r8b, r9b
	mov edx, r8d
	ret
.LBB34_1:
	xor eax, eax
	mov edx, r8d
```
## `checked_ceil_to_multiple_i16_pow2`
```asm
	mov r8d, ecx
	mov r9d, -1
	mov ecx, edx
	shl r9d, cl
	mov edx, r9d
	not edx
	xor eax, eax
	add dx, r8w
	setno al
	and edx, r9d
```
## `checked_ceil_to_multiple_i16_unb_pow2`
```asm
	cmp dl, 15
	ja .LBB28_1
	mov r8d, ecx
	mov r9d, -1
	mov ecx, edx
	shl r9d, cl
	mov ecx, r9d
	not ecx
	xor eax, eax
	add r8w, cx
	setno al
	and r8d, r9d
	mov edx, r8d
	ret
.LBB28_1:
	xor eax, eax
	mov edx, r8d
```
## `checked_ceil_to_multiple_i32_pow2`
```asm
	mov r8d, ecx
	mov r9d, -1
	mov ecx, edx
	shl r9d, cl
	mov edx, r9d
	not edx
	xor eax, eax
	add edx, r8d
	setno al
	and edx, r9d
```
## `checked_ceil_to_multiple_i32_unb_pow2`
```asm
	cmp dl, 31
	ja .LBB30_1
	mov r8d, ecx
	mov r9d, -1
	mov ecx, edx
	shl r9d, cl
	mov ecx, r9d
	not ecx
	xor eax, eax
	add r8d, ecx
	setno al
	and r8d, r9d
	mov edx, r8d
	ret
.LBB30_1:
	xor eax, eax
	mov edx, r8d
```
## `checked_ceil_to_multiple_i64_pow2`
```asm
	mov r8, rcx
	mov r9, -1
	mov ecx, edx
	shl r9, cl
	mov rdx, r9
	not rdx
	xor eax, eax
	add rdx, r8
	setno al
	and rdx, r9
```
## `checked_ceil_to_multiple_i64_unb_pow2`
```asm
	cmp dl, 63
	ja .LBB32_1
	mov r8, rcx
	mov r9, -1
	mov ecx, edx
	shl r9, cl
	mov rcx, r9
	not rcx
	xor eax, eax
	add r8, rcx
	setno al
	and r8, r9
	mov rdx, r8
	ret
.LBB32_1:
	xor eax, eax
	mov rdx, r8
```
## `checked_ceil_to_multiple_i128_pow2`
```asm
	push rsi
	mov rax, rcx
	mov r10, -1
	mov ecx, r9d
	shl r10, cl
	mov r11, -1
	xor ecx, ecx
	test r9b, 64
	mov r9, r10
	cmovne r9, rcx
	cmove r10, r11
	mov r11, r10
	not r11
	mov rsi, r9
	not rsi
	add rsi, rdx
	adc r11, r8
	jo .LBB25_2
	and r11, r10
	and rsi, r9
	mov qword ptr [rax + 16], rsi
	mov qword ptr [rax + 24], r11
	mov ecx, 1
.LBB25_2:
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	pop rsi
```
## `checked_ceil_to_multiple_i128_unb_pow2`
```asm
	push rsi
	mov rax, rcx
	test r9b, r9b
	js .LBB26_4
	mov r10, -1
	mov ecx, r9d
	shl r10, cl
	mov r11, -1
	xor ecx, ecx
	test r9b, 64
	mov r9, r10
	cmovne r9, rcx
	cmove r10, r11
	mov r11, r10
	not r11
	mov rsi, r9
	not rsi
	add rdx, rsi
	adc r8, r11
	jo .LBB26_3
	and r8, r10
	and rdx, r9
	mov qword ptr [rax + 16], rdx
	mov qword ptr [rax + 24], r8
	mov ecx, 1
.LBB26_3:
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	pop rsi
	ret
.LBB26_4:
	xor ecx, ecx
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	pop rsi
```
## `checked_ceil_to_multiple_u8_pow2`
```asm
	mov eax, ecx
	mov r8b, -1
	mov ecx, edx
	shl r8b, cl
	mov edx, r8d
	not dl
	add dl, al
	cmp dl, al
	setae al
	and dl, r8b
```
## `checked_ceil_to_multiple_u8_unb_pow2`
```asm
	cmp dl, 7
	ja .LBB44_1
	mov eax, ecx
	mov r8b, -1
	mov ecx, edx
	shl r8b, cl
	mov edx, r8d
	not dl
	add dl, al
	cmp dl, al
	setae al
	and dl, r8b
	ret
.LBB44_1:
	xor eax, eax
```
## `checked_ceil_to_multiple_u16_pow2`
```asm
	mov r8d, ecx
	mov r9d, -1
	mov ecx, edx
	shl r9d, cl
	mov edx, r9d
	not edx
	add edx, r8d
	xor eax, eax
	cmp dx, r8w
	setae al
	and edx, r9d
```
## `checked_ceil_to_multiple_u16_unb_pow2`
```asm
	cmp dl, 15
	ja .LBB38_1
	mov r8d, ecx
	mov r9d, -1
	mov ecx, edx
	shl r9d, cl
	mov edx, r9d
	not edx
	add edx, r8d
	xor eax, eax
	cmp dx, r8w
	setae al
	and edx, r9d
	ret
.LBB38_1:
	xor eax, eax
```
## `checked_ceil_to_multiple_u32_pow2`
```asm
	mov r8d, ecx
	mov r9d, -1
	mov ecx, edx
	shl r9d, cl
	mov edx, r9d
	not edx
	add edx, r8d
	xor eax, eax
	cmp edx, r8d
	setae al
	and edx, r9d
```
## `checked_ceil_to_multiple_u32_unb_pow2`
```asm
	cmp dl, 31
	ja .LBB40_1
	mov r8d, ecx
	mov r9d, -1
	mov ecx, edx
	shl r9d, cl
	mov edx, r9d
	not edx
	add edx, r8d
	xor eax, eax
	cmp edx, r8d
	setae al
	and edx, r9d
	ret
.LBB40_1:
	xor eax, eax
```
## `checked_ceil_to_multiple_u64_pow2`
```asm
	mov r8, rcx
	mov r9, -1
	mov ecx, edx
	shl r9, cl
	mov rdx, r9
	not rdx
	add rdx, r8
	xor eax, eax
	cmp rdx, r8
	setae al
	and rdx, r9
```
## `checked_ceil_to_multiple_u64_unb_pow2`
```asm
	cmp dl, 63
	ja .LBB42_1
	mov r8, rcx
	mov r9, -1
	mov ecx, edx
	shl r9, cl
	mov rdx, r9
	not rdx
	add rdx, r8
	xor eax, eax
	cmp rdx, r8
	setae al
	and rdx, r9
	ret
.LBB42_1:
	xor eax, eax
```
## `checked_ceil_to_multiple_u128_pow2`
```asm
	push rsi
	mov rax, rcx
	mov r10, -1
	mov ecx, r9d
	shl r10, cl
	mov r11, -1
	xor ecx, ecx
	test r9b, 64
	mov r9, r10
	cmovne r9, rcx
	cmove r10, r11
	mov r11, r10
	not r11
	mov rsi, r9
	not rsi
	add rsi, rdx
	adc r11, r8
	jb .LBB35_2
	and r11, r10
	and rsi, r9
	mov qword ptr [rax + 16], rsi
	mov qword ptr [rax + 24], r11
	mov ecx, 1
.LBB35_2:
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	pop rsi
```
## `checked_ceil_to_multiple_u128_unb_pow2`
```asm
	push rsi
	mov rax, rcx
	test r9b, r9b
	js .LBB36_4
	mov r10, -1
	mov ecx, r9d
	shl r10, cl
	mov r11, -1
	xor ecx, ecx
	test r9b, 64
	mov r9, r10
	cmovne r9, rcx
	cmove r10, r11
	mov r11, r10
	not r11
	mov rsi, r9
	not rsi
	add rdx, rsi
	adc r8, r11
	jb .LBB36_3
	and r8, r10
	and rdx, r9
	mov qword ptr [rax + 16], rdx
	mov qword ptr [rax + 24], r8
	mov ecx, 1
.LBB36_3:
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	pop rsi
	ret
.LBB36_4:
	xor ecx, ecx
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	pop rsi
```
## `checked_div_ceil_i8_unb_pow2`
```asm
	mov r8d, edx
	cmp r8b, 7
	ja .LBB49_2
	mov eax, ecx
	mov r9b, -1
	mov ecx, r8d
	shl r9b, cl
	not r9b
	mov edx, eax
	sar dl, cl
	and r9b, al
	cmp r9b, 1
	sbb dl, -1
.LBB49_2:
	cmp r8b, 8
	setb al
```
## `checked_div_ceil_i16_unb_pow2`
```asm
	mov eax, edx
	cmp dl, 15
	ja .LBB46_1
	mov edx, ecx
	mov r8d, -1
	mov ecx, eax
	shl r8d, cl
	not r8d
	movsx edx, dx
	mov r9d, edx
	sar edx, cl
	and r8d, r9d
	cmp r8w, 1
	sbb dx, -1
	mov ax, 1
	ret
.LBB46_1:
	xor eax, eax
```
## `checked_div_ceil_i32_unb_pow2`
```asm
	cmp dl, 31
	ja .LBB47_1
	mov eax, ecx
	mov r9d, -1
	mov ecx, edx
	shl r9d, cl
	not r9d
	mov edx, eax
	sar edx, cl
	and r9d, eax
	cmp r9d, 1
	sbb edx, -1
	mov eax, 1
	ret
.LBB47_1:
	xor eax, eax
```
## `checked_div_ceil_i64_unb_pow2`
```asm
	cmp dl, 63
	ja .LBB48_1
	mov rax, rcx
	mov r9, -1
	mov ecx, edx
	shl r9, cl
	not r9
	mov rdx, rax
	sar rdx, cl
	and r9, rax
	cmp r9, 1
	sbb rdx, -1
	mov eax, 1
	ret
.LBB48_1:
	xor eax, eax
```
## `checked_div_ceil_i128_unb_pow2`
```asm
	push rsi
	push rdi
	push rbx
	mov rax, rcx
	test r9b, r9b
	js .LBB45_1
	mov r11, -1
	mov ecx, r9d
	shl r11, cl
	mov r10, rdx
	shrd r10, r8, cl
	mov rsi, r8
	sar rsi, cl
	mov rdi, -1
	mov rcx, r8
	sar rcx, 63
	xor ebx, ebx
	test r9b, 64
	cmovne rdi, r11
	cmove rbx, r11
	not rbx
	not rdi
	cmovne r10, rsi
	cmove rcx, rsi
	and rdi, r8
	and rbx, rdx
	xor edx, edx
	or rbx, rdi
	setne dl
	add rdx, r10
	adc rcx, 0
	mov qword ptr [rax + 16], rdx
	mov qword ptr [rax + 24], rcx
	mov ecx, 1
	jmp .LBB45_3
.LBB45_1:
	xor ecx, ecx
.LBB45_3:
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	pop rbx
	pop rdi
	pop rsi
```
## `checked_div_ceil_u8_unb_pow2`
```asm
	mov r8d, edx
	cmp r8b, 7
	ja .LBB54_2
	mov eax, ecx
	mov r9b, -1
	mov ecx, r8d
	shl r9b, cl
	not r9b
	mov edx, eax
	shr dl, cl
	and r9b, al
	cmp r9b, 1
	sbb dl, -1
.LBB54_2:
	cmp r8b, 8
	setb al
```
## `checked_div_ceil_u16_unb_pow2`
```asm
	mov eax, edx
	cmp dl, 15
	ja .LBB51_1
	mov edx, ecx
	mov r8d, -1
	mov ecx, eax
	shl r8d, cl
	not r8d
	movzx edx, dx
	mov r9d, edx
	shr edx, cl
	and r8d, r9d
	cmp r8w, 1
	sbb dx, -1
	mov ax, 1
	ret
.LBB51_1:
	xor eax, eax
```
## `checked_div_ceil_u32_unb_pow2`
```asm
	cmp dl, 31
	ja .LBB52_1
	mov eax, ecx
	mov r9d, -1
	mov ecx, edx
	shl r9d, cl
	not r9d
	mov edx, eax
	shr edx, cl
	and r9d, eax
	cmp r9d, 1
	sbb edx, -1
	mov eax, 1
	ret
.LBB52_1:
	xor eax, eax
```
## `checked_div_ceil_u64_unb_pow2`
```asm
	cmp dl, 63
	ja .LBB53_1
	mov rax, rcx
	mov r9, -1
	mov ecx, edx
	shl r9, cl
	not r9
	mov rdx, rax
	shr rdx, cl
	and r9, rax
	cmp r9, 1
	sbb rdx, -1
	mov eax, 1
	ret
.LBB53_1:
	xor eax, eax
```
## `checked_div_ceil_u128_unb_pow2`
```asm
	push rsi
	push rdi
	mov rax, rcx
	test r9b, r9b
	js .LBB50_1
	mov rsi, -1
	mov ecx, r9d
	shl rsi, cl
	mov r10, rdx
	shrd r10, r8, cl
	mov rdi, -1
	mov r11, r8
	shr r11, cl
	xor ecx, ecx
	test r9b, 64
	cmovne rdi, rsi
	cmovne rsi, rcx
	not rsi
	not rdi
	cmovne r10, r11
	cmovne r11, rcx
	and rdi, r8
	and rsi, rdx
	xor ecx, ecx
	or rsi, rdi
	setne cl
	add rcx, r10
	adc r11, 0
	mov qword ptr [rax + 16], rcx
	mov qword ptr [rax + 24], r11
	mov ecx, 1
	jmp .LBB50_3
.LBB50_1:
	xor ecx, ecx
.LBB50_3:
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	pop rdi
	pop rsi
```
## `checked_div_floor_i8_unb_pow2`
```asm
	mov r8d, ecx
	cmp dl, 8
	setb al
	mov ecx, edx
	sar r8b, cl
	mov edx, r8d
```
## `checked_div_floor_i16_unb_pow2`
```asm
	mov r8d, edx
	movsx edx, cx
	xor eax, eax
	cmp r8b, 16
	setb al
	mov ecx, r8d
	sar edx, cl
```
## `checked_div_floor_i32_unb_pow2`
```asm
	mov r8d, ecx
	xor eax, eax
	cmp dl, 32
	setb al
	mov ecx, edx
	sar r8d, cl
	mov edx, r8d
```
## `checked_div_floor_i64_unb_pow2`
```asm
	mov r8, rcx
	xor eax, eax
	cmp dl, 64
	setb al
	mov ecx, edx
	sar r8, cl
	mov rdx, r8
```
## `checked_div_floor_i128_unb_pow2`
```asm
	mov rax, rcx
	test r9b, r9b
	js .LBB55_1
	mov ecx, r9d
	shrd rdx, r8, cl
	mov r10, r8
	sar r10, cl
	sar r8, 63
	test r9b, 64
	cmovne rdx, r10
	cmove r8, r10
	mov qword ptr [rax + 24], r8
	mov qword ptr [rax + 16], rdx
	mov ecx, 1
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	ret
.LBB55_1:
	xor ecx, ecx
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
```
## `checked_div_floor_u8_unb_pow2`, `checked_div_u8_unb_pow2`
```asm
	mov r8d, ecx
	cmp dl, 8
	setb al
	mov ecx, edx
	shr r8b, cl
	mov edx, r8d
```
## `checked_div_floor_u16_unb_pow2`, `checked_div_u16_unb_pow2`
```asm
	mov r8d, edx
	movzx edx, cx
	xor eax, eax
	cmp r8b, 16
	setb al
	mov ecx, r8d
	shr edx, cl
```
## `checked_div_floor_u32_unb_pow2`, `checked_div_u32_unb_pow2`
```asm
	mov r8d, ecx
	xor eax, eax
	cmp dl, 32
	setb al
	mov ecx, edx
	shr r8d, cl
	mov edx, r8d
```
## `checked_div_floor_u64_unb_pow2`, `checked_div_u64_unb_pow2`
```asm
	mov r8, rcx
	xor eax, eax
	cmp dl, 64
	setb al
	mov ecx, edx
	shr r8, cl
	mov rdx, r8
```
## `checked_div_floor_u128_unb_pow2`
```asm
	mov rax, rcx
	test r9b, r9b
	js .LBB60_1
	mov ecx, r9d
	shrd rdx, r8, cl
	shr r8, cl
	xor ecx, ecx
	test r9b, 64
	cmovne rdx, r8
	cmove rcx, r8
	mov qword ptr [rax + 24], rcx
	mov qword ptr [rax + 16], rdx
	mov ecx, 1
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	ret
.LBB60_1:
	xor ecx, ecx
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
```
## `checked_div_i8_unb_pow2`
```asm
	mov eax, edx
	cmp al, 7
	ja .LBB69_2
	mov r8d, ecx
	mov r9b, -1
	mov ecx, eax
	shl r9b, cl
	not r9b
	mov edx, r8d
	sar dl, 7
	and dl, r9b
	add dl, r8b
	sar dl, cl
.LBB69_2:
	cmp al, 8
	setb al
```
## `checked_div_i16_unb_pow2`
```asm
	cmp dl, 15
	ja .LBB66_1
	mov r8d, edx
	mov eax, ecx
	mov edx, -1
	mov ecx, r8d
	shl edx, cl
	not edx
	movsx ecx, ax
	sar ecx, 15
	and ecx, edx
	add ecx, eax
	movsx edx, cx
	mov ecx, r8d
	sar edx, cl
	mov ax, 1
	ret
.LBB66_1:
	xor eax, eax
```
## `checked_div_i32_unb_pow2`
```asm
	cmp dl, 31
	ja .LBB67_1
	mov r8d, ecx
	mov r9d, -1
	mov ecx, edx
	shl r9d, cl
	not r9d
	mov edx, r8d
	sar edx, 31
	and edx, r9d
	add edx, r8d
	sar edx, cl
	mov eax, 1
	ret
.LBB67_1:
	xor eax, eax
```
## `checked_div_i64_unb_pow2`
```asm
	cmp dl, 63
	ja .LBB68_1
	mov r8, rcx
	mov r9, -1
	mov ecx, edx
	shl r9, cl
	not r9
	mov rdx, r8
	sar rdx, 63
	and rdx, r9
	add rdx, r8
	sar rdx, cl
	mov eax, 1
	ret
.LBB68_1:
	xor eax, eax
```
## `checked_div_i128_unb_pow2`
```asm
	mov rax, rcx
	test r9b, r9b
	js .LBB65_1
	mov r10, -1
	mov r11, -1
	mov ecx, r9d
	shl r11, cl
	xor ecx, ecx
	test r9b, 64
	cmovne r10, r11
	cmove rcx, r11
	not rcx
	not r10
	mov r11, r8
	sar r11, 63
	and r10, r11
	and r11, rcx
	add r11, rdx
	adc r10, r8
	mov rdx, r10
	mov ecx, r9d
	sar rdx, cl
	mov r8, r10
	sar r8, 63
	test r9b, 64
	cmove r8, rdx
	shrd r11, r10, cl
	test r9b, 64
	cmovne r11, rdx
	mov qword ptr [rax + 24], r8
	mov qword ptr [rax + 16], r11
	mov ecx, 1
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	ret
.LBB65_1:
	xor ecx, ecx
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
```
## `checked_div_round_i8_unb_pow2`
```asm
	mov eax, edx
	cmp al, 7
	ja .LBB74_2
	mov r8d, ecx
	mov r9b, -1
	mov ecx, eax
	shl r9b, cl
	not r9b
	mov edx, r8d
	sar dl, cl
	and r9b, r8b
	shr r8b, 7
	xor ecx, ecx
	sub r9b, r8b
	movzx r8d, r9b
	cmovb r8d, ecx
	add r8b, r8b
	movzx ecx, r8b
	movzx r8d, al
	bt ecx, r8d
	adc dl, 0
.LBB74_2:
	cmp al, 8
	setb al
```
## `checked_div_round_i16_unb_pow2`
```asm
	mov r8d, edx
	xor eax, eax
	cmp dl, 15
	ja .LBB71_1
	mov edx, ecx
	mov r9d, -1
	mov ecx, r8d
	shl r9d, cl
	not r9d
	movsx r10d, dx
	mov edx, r10d
	sar edx, cl
	mov ecx, r10d
	and r10d, r9d
	movzx ecx, cx
	shr ecx, 15
	sub r10w, cx
	cmovb r10d, eax
	add r10d, r10d
	movzx eax, r8b
	bt r10d, eax
	adc dx, 0
	mov ax, 1
	ret
.LBB71_1:
```
## `checked_div_round_i32_unb_pow2`
```asm
	cmp dl, 31
	ja .LBB72_1
	mov eax, edx
	mov r8d, ecx
	mov r9d, -1
	mov ecx, edx
	shl r9d, cl
	not r9d
	mov edx, r8d
	sar edx, cl
	and r9d, r8d
	shr r8d, 31
	xor ecx, ecx
	sub r9d, r8d
	cmovb r9d, ecx
	add r9d, r9d
	movzx eax, al
	bt r9d, eax
	adc edx, 0
	mov eax, 1
	ret
.LBB72_1:
	xor eax, eax
```
## `checked_div_round_i64_unb_pow2`
```asm
	cmp dl, 63
	ja .LBB73_1
	mov eax, edx
	mov r8, rcx
	mov r9, -1
	mov ecx, edx
	shl r9, cl
	not r9
	mov rdx, r8
	sar rdx, cl
	and r9, r8
	shr r8, 63
	xor ecx, ecx
	sub r9, r8
	cmovae rcx, r9
	add rcx, rcx
	movzx eax, al
	bt rcx, rax
	adc rdx, 0
	mov eax, 1
	ret
.LBB73_1:
	xor eax, eax
```
## `checked_div_round_i128_unb_pow2`
```asm
	push rsi
	mov rax, rcx
	test r9b, r9b
	js .LBB70_1
	mov r10, -1
	mov r11, -1
	mov ecx, r9d
	shl r11, cl
	xor ecx, ecx
	test r9b, 64
	cmovne r10, r11
	cmovne r11, rcx
	not r11
	not r10
	and r10, r8
	and r11, rdx
	mov rsi, r8
	shr rsi, 63
	sub r11, rsi
	sbb r10, 0
	cmovb r10, rcx
	cmovb r11, rcx
	shld r10, r11, 1
	add r11, r11
	mov ecx, r9d
	shrd r11, r10, cl
	shr r10, cl
	test r9b, 64
	cmove r10, r11
	mov r11, r8
	sar r11, cl
	mov rsi, r8
	sar rsi, 63
	test r9b, 64
	cmove rsi, r11
	shrd rdx, r8, cl
	test r9b, 64
	cmovne rdx, r11
	and r10d, 1
	add r10, rdx
	adc rsi, 0
	mov qword ptr [rax + 16], r10
	mov qword ptr [rax + 24], rsi
	mov ecx, 1
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	pop rsi
	ret
.LBB70_1:
	xor ecx, ecx
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	pop rsi
```
## `checked_div_round_u8_unb_pow2`
```asm
	mov eax, edx
	mov edx, ecx
	cmp al, 7
	ja .LBB79_2
	mov r8b, 1
	mov ecx, eax
	shl r8b, cl
	shr r8b
	and r8b, dl
	shr dl, cl
	cmp r8b, 1
	sbb dl, -1
.LBB79_2:
	cmp al, 8
	setb al
```
## `checked_div_round_u16_unb_pow2`
```asm
	mov eax, edx
	cmp dl, 15
	ja .LBB76_1
	mov edx, ecx
	mov r8d, 1
	mov ecx, eax
	shl r8d, cl
	movzx r8d, r8w
	shr r8d
	movzx r9d, dx
	mov edx, r9d
	shr edx, cl
	and r9d, r8d
	cmp r9w, 1
	sbb dx, -1
	mov ax, 1
	ret
.LBB76_1:
	xor eax, eax
```
## `checked_div_round_u32_unb_pow2`
```asm
	mov eax, edx
	cmp dl, 31
	ja .LBB77_1
	mov edx, ecx
	mov r8d, 1
	mov ecx, eax
	shl r8d, cl
	shr r8d
	and r8d, edx
	shr edx, cl
	mov eax, 1
	cmp r8d, 1
	sbb edx, -1
	ret
.LBB77_1:
	xor eax, eax
```
## `checked_div_round_u64_unb_pow2`
```asm
	mov eax, edx
	cmp dl, 63
	ja .LBB78_1
	mov rdx, rcx
	mov r8d, 1
	mov ecx, eax
	shl r8, cl
	shr r8
	and r8, rdx
	shr rdx, cl
	mov eax, 1
	cmp r8, 1
	sbb rdx, -1
	ret
.LBB78_1:
	xor eax, eax
```
## `checked_div_round_u128_unb_pow2`
```asm
	push r14
	push rsi
	push rdi
	push rbx
	mov rax, rcx
	test r9b, r9b
	js .LBB75_1
	mov r11, r8
	mov ecx, r9d
	shr r11, cl
	xor edi, edi
	test r9b, 64
	mov r10, r11
	cmovne r10, rdi
	mov rsi, rdx
	shrd rsi, r8, cl
	test r9b, 64
	cmovne rsi, r11
	mov r11d, 1
	mov ebx, 1
	shl rbx, cl
	test r9b, 64
	mov r14, rbx
	cmovne r14, rdi
	shld rdi, r11, cl
	test r9b, 64
	cmovne rdi, rbx
	shrd r14, rdi, 1
	shr rdi
	and rdi, r8
	and r14, rdx
	xor ecx, ecx
	or r14, rdi
	setne cl
	add rcx, rsi
	adc r10, 0
	mov qword ptr [rax + 16], rcx
	mov qword ptr [rax + 24], r10
	jmp .LBB75_3
.LBB75_1:
	xor r11d, r11d
.LBB75_3:
	mov qword ptr [rax], r11
	mov qword ptr [rax + 8], 0
	pop rbx
	pop rdi
	pop rsi
	pop r14
```
## `checked_div_u128_unb_pow2`
```asm
	mov rax, rcx
	test r9b, r9b
	js .LBB80_1
	mov ecx, r9d
	shrd rdx, r8, cl
	shr r8, cl
	xor ecx, ecx
	test r9b, 64
	cmovne rdx, r8
	cmove rcx, r8
	mov qword ptr [rax + 24], rcx
	mov qword ptr [rax + 16], rdx
	mov ecx, 1
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	ret
.LBB80_1:
	xor ecx, ecx
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
```
## `checked_floor_to_multiple_i8_unb_pow2`, `checked_floor_to_multiple_u8_unb_pow2`
```asm
	mov r9d, edx
	mov r8d, ecx
	cmp dl, 8
	setb al
	mov dl, -1
	mov ecx, r9d
	shl dl, cl
	and dl, r8b
```
## `checked_floor_to_multiple_i16_unb_pow2`, `checked_floor_to_multiple_u16_unb_pow2`
```asm
	mov r9d, edx
	mov r8d, ecx
	xor eax, eax
	cmp dl, 16
	setb al
	mov edx, -1
	mov ecx, r9d
	shl edx, cl
	and edx, r8d
```
## `checked_floor_to_multiple_i32_unb_pow2`, `checked_floor_to_multiple_u32_unb_pow2`
```asm
	mov r9d, edx
	mov r8d, ecx
	xor eax, eax
	cmp dl, 32
	setb al
	mov edx, -1
	mov ecx, r9d
	shl edx, cl
	and edx, r8d
```
## `checked_floor_to_multiple_i64_unb_pow2`, `checked_floor_to_multiple_u64_unb_pow2`
```asm
	mov r9d, edx
	mov r8, rcx
	xor eax, eax
	cmp dl, 64
	setb al
	mov rdx, -1
	mov ecx, r9d
	shl rdx, cl
	and rdx, r8
```
## `checked_floor_to_multiple_i128_unb_pow2`
```asm
	mov rax, rcx
	test r9b, r9b
	js .LBB85_1
	mov r10, -1
	mov r11, -1
	mov ecx, r9d
	shl r11, cl
	xor ecx, ecx
	test r9b, 64
	cmovne r10, r11
	cmove rcx, r11
	and rcx, rdx
	and r10, r8
	mov qword ptr [rax + 24], r10
	mov qword ptr [rax + 16], rcx
	mov ecx, 1
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	ret
.LBB85_1:
	xor ecx, ecx
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
```
## `checked_floor_to_multiple_u128_unb_pow2`
```asm
	mov rax, rcx
	test r9b, r9b
	js .LBB90_1
	mov r10, -1
	mov r11, -1
	mov ecx, r9d
	shl r11, cl
	xor ecx, ecx
	test r9b, 64
	cmovne r10, r11
	cmove rcx, r11
	and rcx, rdx
	and r10, r8
	mov qword ptr [rax + 24], r10
	mov qword ptr [rax + 16], rcx
	mov ecx, 1
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	ret
.LBB90_1:
	xor ecx, ecx
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
```
## `checked_mul_i8_pow2`
```asm
	mov r8d, edx
	mov eax, ecx
	mov edx, ecx
	mov ecx, r8d
	shl dl, cl
	mov r9d, edx
	sar r9b, cl
	cmp al, r9b
	sete al
```
## `checked_mul_i8_unb_pow2`
```asm
	mov r8d, edx
	mov eax, ecx
	cmp dl, 8
	setb r9b
	mov edx, ecx
	mov ecx, r8d
	shl dl, cl
	and r8b, 7
	mov r10d, edx
	mov ecx, r8d
	sar r10b, cl
	cmp al, r10b
	sete al
	and al, r9b
```
## `checked_mul_i16_pow2`
```asm
	mov r8d, edx
	mov r9d, ecx
	mov eax, r9d
	mov ecx, edx
	shl eax, cl
	movsx edx, ax
	mov r10d, edx
	mov ecx, r8d
	sar r10d, cl
	xor eax, eax
	cmp r9w, r10w
	sete al
```
## `checked_mul_i16_unb_pow2`
```asm
	cmp dl, 15
	ja .LBB98_1
	mov eax, edx
	mov edx, ecx
	mov r8d, ecx
	mov ecx, eax
	shl edx, cl
	movsx r9d, dx
	sar r9d, cl
	xor eax, eax
	cmp r8w, r9w
	sete al
	ret
.LBB98_1:
	xor eax, eax
```
## `checked_mul_i32_pow2`
```asm
	mov r8d, edx
	mov r9d, ecx
	mov edx, ecx
	mov ecx, r8d
	shl edx, cl
	mov r10d, edx
	sar r10d, cl
	xor eax, eax
	cmp r9d, r10d
	sete al
```
## `checked_mul_i32_unb_pow2`
```asm
	cmp dl, 31
	ja .LBB100_1
	mov eax, edx
	mov edx, ecx
	mov r8d, ecx
	mov ecx, eax
	shl edx, cl
	mov r9d, edx
	sar r9d, cl
	xor eax, eax
	cmp r8d, r9d
	sete al
	ret
.LBB100_1:
	xor eax, eax
```
## `checked_mul_i64_pow2`
```asm
	mov r8d, edx
	mov r9, rcx
	mov rdx, rcx
	mov ecx, r8d
	shl rdx, cl
	mov r10, rdx
	sar r10, cl
	xor eax, eax
	cmp r9, r10
	sete al
```
## `checked_mul_i64_unb_pow2`
```asm
	cmp dl, 63
	ja .LBB102_1
	mov eax, edx
	mov rdx, rcx
	mov r8, rcx
	mov ecx, eax
	shl rdx, cl
	mov r9, rdx
	sar r9, cl
	xor eax, eax
	cmp r8, r9
	sete al
	ret
.LBB102_1:
	xor eax, eax
```
## `checked_mul_i128_pow2`
```asm
	push r14
	push rsi
	push rdi
	push rbx
	mov rax, rcx
	mov r10, r8
	mov ecx, r9d
	shld r10, rdx, cl
	mov r11, rdx
	shl r11, cl
	test r9b, 64
	cmovne r10, r11
	mov rdi, r10
	sar rdi, cl
	mov rbx, r10
	sar rbx, 63
	xor esi, esi
	test r9b, 64
	cmove rbx, rdi
	cmovne r11, rsi
	mov r14, r11
	shrd r14, r10, cl
	test r9b, 64
	cmovne r14, rdi
	xor rbx, r8
	xor r14, rdx
	or r14, rbx
	jne .LBB95_2
	mov qword ptr [rax + 16], r11
	mov qword ptr [rax + 24], r10
	mov esi, 1
.LBB95_2:
	mov qword ptr [rax], rsi
	mov qword ptr [rax + 8], 0
	pop rbx
	pop rdi
	pop rsi
	pop r14
```
## `checked_mul_i128_unb_pow2`
```asm
	push r14
	push rsi
	push rdi
	push rbx
	mov rax, rcx
	test r9b, r9b
	js .LBB96_1
	mov r10, r8
	mov ecx, r9d
	shld r10, rdx, cl
	mov r11, rdx
	shl r11, cl
	test r9b, 64
	cmovne r10, r11
	mov rdi, r10
	sar rdi, cl
	mov rbx, r10
	sar rbx, 63
	xor esi, esi
	test r9b, 64
	cmove rbx, rdi
	cmovne r11, rsi
	mov r14, r11
	shrd r14, r10, cl
	test r9b, 64
	cmovne r14, rdi
	xor rbx, r8
	xor r14, rdx
	or r14, rbx
	jne .LBB96_4
	mov qword ptr [rax + 16], r11
	mov qword ptr [rax + 24], r10
	mov esi, 1
	jmp .LBB96_4
.LBB96_1:
	xor esi, esi
.LBB96_4:
	mov qword ptr [rax], rsi
	mov qword ptr [rax + 8], 0
	pop rbx
	pop rdi
	pop rsi
	pop r14
```
## `checked_mul_u8_pow2`
```asm
	mov r8d, edx
	mov eax, ecx
	mov edx, ecx
	mov ecx, r8d
	shl dl, cl
	mov r9d, edx
	shr r9b, cl
	cmp al, r9b
	sete al
```
## `checked_mul_u8_unb_pow2`
```asm
	mov r8d, edx
	mov eax, ecx
	cmp dl, 8
	setb r9b
	mov edx, ecx
	mov ecx, r8d
	shl dl, cl
	and r8b, 7
	mov r10d, edx
	mov ecx, r8d
	shr r10b, cl
	cmp al, r10b
	sete al
	and al, r9b
```
## `checked_mul_u16_pow2`
```asm
	mov r8d, edx
	mov r9d, ecx
	mov eax, r9d
	mov ecx, edx
	shl eax, cl
	movzx edx, ax
	mov r10d, edx
	mov ecx, r8d
	shr r10d, cl
	xor eax, eax
	cmp r9w, r10w
	sete al
```
## `checked_mul_u16_unb_pow2`
```asm
	cmp dl, 15
	ja .LBB108_1
	mov eax, edx
	mov edx, ecx
	mov r8d, ecx
	mov ecx, eax
	shl edx, cl
	movzx r9d, dx
	shr r9d, cl
	xor eax, eax
	cmp r8w, r9w
	sete al
	ret
.LBB108_1:
	xor eax, eax
```
## `checked_mul_u32_pow2`
```asm
	mov r8d, edx
	mov r9d, ecx
	mov edx, ecx
	mov ecx, r8d
	shl edx, cl
	mov r10d, edx
	shr r10d, cl
	xor eax, eax
	cmp r9d, r10d
	sete al
```
## `checked_mul_u32_unb_pow2`
```asm
	cmp dl, 31
	ja .LBB110_1
	mov eax, edx
	mov edx, ecx
	mov r8d, ecx
	mov ecx, eax
	shl edx, cl
	mov r9d, edx
	shr r9d, cl
	xor eax, eax
	cmp r8d, r9d
	sete al
	ret
.LBB110_1:
	xor eax, eax
```
## `checked_mul_u64_pow2`
```asm
	mov r8d, edx
	mov r9, rcx
	mov rdx, rcx
	mov ecx, r8d
	shl rdx, cl
	mov r10, rdx
	shr r10, cl
	xor eax, eax
	cmp r9, r10
	sete al
```
## `checked_mul_u64_unb_pow2`
```asm
	cmp dl, 63
	ja .LBB112_1
	mov eax, edx
	mov rdx, rcx
	mov r8, rcx
	mov ecx, eax
	shl rdx, cl
	mov r9, rdx
	shr r9, cl
	xor eax, eax
	cmp r8, r9
	sete al
	ret
.LBB112_1:
	xor eax, eax
```
## `checked_mul_u128_pow2`
```asm
	push r14
	push rsi
	push rdi
	push rbx
	mov rax, rcx
	mov r10, r8
	mov ecx, r9d
	shld r10, rdx, cl
	mov r11, rdx
	shl r11, cl
	test r9b, 64
	cmovne r10, r11
	mov rdi, r10
	shr rdi, cl
	xor esi, esi
	test r9b, 64
	cmovne r11, rsi
	mov rbx, rdi
	cmovne rbx, rsi
	mov r14, r11
	shrd r14, r10, cl
	test r9b, 64
	cmovne r14, rdi
	xor r14, rdx
	xor rbx, r8
	or rbx, r14
	jne .LBB105_2
	mov qword ptr [rax + 16], r11
	mov qword ptr [rax + 24], r10
	mov esi, 1
.LBB105_2:
	mov qword ptr [rax], rsi
	mov qword ptr [rax + 8], 0
	pop rbx
	pop rdi
	pop rsi
	pop r14
```
## `checked_mul_u128_unb_pow2`
```asm
	push r14
	push rsi
	push rdi
	push rbx
	mov rax, rcx
	test r9b, r9b
	js .LBB106_1
	mov r10, r8
	mov ecx, r9d
	shld r10, rdx, cl
	mov r11, rdx
	shl r11, cl
	test r9b, 64
	cmovne r10, r11
	mov rdi, r10
	shr rdi, cl
	xor esi, esi
	test r9b, 64
	cmovne r11, rsi
	mov rbx, rdi
	cmovne rbx, rsi
	mov r14, r11
	shrd r14, r10, cl
	test r9b, 64
	cmovne r14, rdi
	xor r14, rdx
	xor rbx, r8
	or rbx, r14
	jne .LBB106_4
	mov qword ptr [rax + 16], r11
	mov qword ptr [rax + 24], r10
	mov esi, 1
	jmp .LBB106_4
.LBB106_1:
	xor esi, esi
.LBB106_4:
	mov qword ptr [rax], rsi
	mov qword ptr [rax + 8], 0
	pop rbx
	pop rdi
	pop rsi
	pop r14
```
## `checked_rem_floor_i8_unb_pow2`, `checked_rem_floor_u8_unb_pow2`, `checked_rem_u8_unb_pow2`
```asm
	mov r9d, edx
	mov r8d, ecx
	cmp dl, 8
	setb al
	mov dl, -1
	mov ecx, r9d
	shl dl, cl
	not dl
	and dl, r8b
```
## `checked_rem_floor_i16_unb_pow2`, `checked_rem_floor_u16_unb_pow2`, `checked_rem_u16_unb_pow2`
```asm
	mov r9d, edx
	mov r8d, ecx
	xor eax, eax
	cmp dl, 16
	setb al
	mov edx, -1
	mov ecx, r9d
	shl edx, cl
	not edx
	and edx, r8d
```
## `checked_rem_floor_i32_unb_pow2`, `checked_rem_floor_u32_unb_pow2`, `checked_rem_u32_unb_pow2`
```asm
	mov r9d, edx
	mov r8d, ecx
	xor eax, eax
	cmp dl, 32
	setb al
	mov edx, -1
	mov ecx, r9d
	shl edx, cl
	not edx
	and edx, r8d
```
## `checked_rem_floor_i64_unb_pow2`, `checked_rem_floor_u64_unb_pow2`, `checked_rem_u64_unb_pow2`
```asm
	mov r9d, edx
	mov r8, rcx
	xor eax, eax
	cmp dl, 64
	setb al
	mov rdx, -1
	mov ecx, r9d
	shl rdx, cl
	not rdx
	and rdx, r8
```
## `checked_rem_floor_i128_unb_pow2`
```asm
	mov rax, rcx
	test r9b, r9b
	js .LBB115_1
	mov r10, -1
	mov ecx, r9d
	shl r10, cl
	mov rcx, -1
	xor r11d, r11d
	test r9b, 64
	cmove r11, r10
	cmove r10, rcx
	not r10
	not r11
	and rdx, r11
	and r8, r10
	mov qword ptr [rax + 24], r8
	mov qword ptr [rax + 16], rdx
	mov ecx, 1
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	ret
.LBB115_1:
	xor ecx, ecx
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
```
## `checked_rem_floor_u128_unb_pow2`
```asm
	mov rax, rcx
	test r9b, r9b
	js .LBB120_1
	mov r10, -1
	mov ecx, r9d
	shl r10, cl
	mov rcx, -1
	xor r11d, r11d
	test r9b, 64
	cmove r11, r10
	cmove r10, rcx
	not r10
	not r11
	and rdx, r11
	and r8, r10
	mov qword ptr [rax + 24], r8
	mov qword ptr [rax + 16], rdx
	mov ecx, 1
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	ret
.LBB120_1:
	xor ecx, ecx
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
```
## `checked_rem_i8_unb_pow2`
```asm
	mov r8d, ecx
	cmp dl, 7
	ja .LBB129_2
	mov al, -1
	mov ecx, edx
	shl al, cl
	mov ecx, eax
	not cl
	mov r9d, r8d
	sar r9b, 7
	and r9b, cl
	add r9b, r8b
	and r9b, al
	sub r8b, r9b
.LBB129_2:
	cmp dl, 8
	setb al
	mov edx, r8d
```
## `checked_rem_i16_unb_pow2`
```asm
	cmp dl, 15
	ja .LBB126_1
	mov r8d, ecx
	mov eax, -1
	mov ecx, edx
	shl eax, cl
	mov ecx, eax
	not ecx
	movsx edx, r8w
	sar edx, 15
	and edx, ecx
	add edx, r8d
	and edx, eax
	sub r8d, edx
	mov ax, 1
	mov edx, r8d
	ret
.LBB126_1:
	xor eax, eax
	mov edx, r8d
```
## `checked_rem_i32_unb_pow2`
```asm
	cmp dl, 31
	ja .LBB127_1
	mov r8d, ecx
	mov eax, -1
	mov ecx, edx
	shl eax, cl
	mov ecx, eax
	not ecx
	mov edx, r8d
	sar edx, 31
	and edx, ecx
	add edx, r8d
	and edx, eax
	sub r8d, edx
	mov eax, 1
	mov edx, r8d
	ret
.LBB127_1:
	xor eax, eax
	mov edx, r8d
```
## `checked_rem_i64_unb_pow2`
```asm
	cmp dl, 63
	ja .LBB128_1
	mov r8, rcx
	mov rax, -1
	mov ecx, edx
	shl rax, cl
	mov rcx, rax
	not rcx
	mov rdx, r8
	sar rdx, 63
	and rdx, rcx
	add rdx, r8
	and rdx, rax
	sub r8, rdx
	mov eax, 1
	mov rdx, r8
	ret
.LBB128_1:
	xor eax, eax
	mov rdx, r8
```
## `checked_rem_i128_unb_pow2`
```asm
	push rsi
	mov rax, rcx
	test r9b, r9b
	js .LBB125_1
	mov r10, -1
	mov ecx, r9d
	shl r10, cl
	mov rcx, -1
	xor r11d, r11d
	test r9b, 64
	cmovne rcx, r10
	cmove r11, r10
	mov r9, r11
	not r9
	mov r10, rcx
	not r10
	mov rsi, r8
	sar rsi, 63
	and r10, rsi
	and rsi, r9
	add rsi, rdx
	adc r10, r8
	and r10, rcx
	and rsi, r11
	sub rdx, rsi
	sbb r8, r10
	mov qword ptr [rax + 16], rdx
	mov qword ptr [rax + 24], r8
	mov ecx, 1
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	pop rsi
	ret
.LBB125_1:
	xor ecx, ecx
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	pop rsi
```
## `checked_rem_u128_unb_pow2`
```asm
	mov rax, rcx
	test r9b, r9b
	js .LBB130_1
	mov r10, -1
	mov ecx, r9d
	shl r10, cl
	mov rcx, -1
	xor r11d, r11d
	test r9b, 64
	cmove r11, r10
	cmove r10, rcx
	not r10
	not r11
	and rdx, r11
	and r8, r10
	mov qword ptr [rax + 24], r8
	mov qword ptr [rax + 16], rdx
	mov ecx, 1
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	ret
.LBB130_1:
	xor ecx, ecx
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
```
## `checked_round_to_multiple_i8_unb_pow2`
```asm
	xor r10d, r10d
	cmp dl, 7
	ja .LBB139_1
	mov r8d, ecx
	mov al, 1
	mov r9b, 1
	mov ecx, edx
	shl r9b, cl
	mov ecx, r9d
	shr cl
	mov edx, r8d
	shr dl, 7
	sub cl, dl
	movzx edx, cl
	cmovb edx, r10d
	add dl, r8b
	jo .LBB139_1
	neg r9b
	and dl, r9b
	ret
.LBB139_1:
	xor eax, eax
```
## `checked_round_to_multiple_i16_unb_pow2`
```asm
	xor eax, eax
	cmp dl, 15
	ja .LBB136_4
	mov r9d, edx
	mov r8d, ecx
	mov edx, 1
	mov ecx, r9d
	shl edx, cl
	movzx r9d, dx
	shr r9d
	movzx ecx, r8w
	mov r8d, ecx
	shr r8d, 15
	sub r9w, r8w
	cmovb r9d, eax
	add cx, r9w
	jo .LBB136_4
	neg edx
	and edx, ecx
	mov ax, 1
	ret
.LBB136_4:
```
## `checked_round_to_multiple_i32_unb_pow2`
```asm
	cmp dl, 31
	ja .LBB137_3
	mov r8d, ecx
	mov r9d, 1
	mov ecx, edx
	shl r9d, cl
	mov edx, r9d
	shr edx
	mov ecx, r8d
	shr ecx, 31
	xor eax, eax
	sub edx, ecx
	cmovb edx, eax
	add edx, r8d
	jo .LBB137_4
	neg r9d
	and edx, r9d
	mov eax, 1
	ret
.LBB137_3:
	xor eax, eax
.LBB137_4:
```
## `checked_round_to_multiple_i64_unb_pow2`
```asm
	cmp dl, 63
	ja .LBB138_3
	mov r8, rcx
	mov r9d, 1
	mov ecx, edx
	shl r9, cl
	mov rdx, r9
	shr rdx
	mov rcx, r8
	shr rcx, 63
	xor eax, eax
	sub rdx, rcx
	cmovb rdx, rax
	add rdx, r8
	jo .LBB138_4
	neg r9
	and rdx, r9
	mov eax, 1
	ret
.LBB138_3:
	xor eax, eax
.LBB138_4:
```
## `checked_round_to_multiple_i128_unb_pow2`
```asm
	push rsi
	push rdi
	push rbx
	mov rax, rcx
	test r9b, r9b
	js .LBB135_3
	mov r10d, 1
	xor esi, esi
	mov ecx, r9d
	shld rsi, r10, cl
	mov r11d, 1
	shl r11, cl
	xor edi, edi
	test r9b, 64
	cmovne rsi, r11
	cmovne r11, rdi
	mov rcx, rsi
	shr rcx
	mov r9, rsi
	shld r9, r11, 63
	mov rbx, r8
	shr rbx, 63
	sub r9, rbx
	sbb rcx, 0
	cmovb rcx, rdi
	cmovb r9, rdi
	add rdx, r9
	adc rcx, r8
	jo .LBB135_3
	xor r8d, r8d
	neg r11
	sbb r8, rsi
	and rcx, r8
	and rdx, r11
	mov qword ptr [rax + 16], rdx
	mov qword ptr [rax + 24], rcx
	jmp .LBB135_4
.LBB135_3:
	xor r10d, r10d
.LBB135_4:
	mov qword ptr [rax], r10
	mov qword ptr [rax + 8], 0
	pop rbx
	pop rdi
	pop rsi
```
## `checked_round_to_multiple_u8_unb_pow2`
```asm
	cmp dl, 7
	ja .LBB144_2
	mov r8d, ecx
	mov r9b, 1
	mov ecx, edx
	shl r9b, cl
	mov al, 1
	mov edx, r9d
	shr dl
	add dl, r8b
	jb .LBB144_2
	neg r9b
	and dl, r9b
	ret
.LBB144_2:
	xor eax, eax
```
## `checked_round_to_multiple_u16_unb_pow2`
```asm
	cmp dl, 15
	ja .LBB141_2
	mov eax, ecx
	mov r8d, 1
	mov ecx, edx
	shl r8d, cl
	movzx edx, r8w
	shr edx
	add dx, ax
	jb .LBB141_2
	neg r8d
	and edx, r8d
	mov ax, 1
	ret
.LBB141_2:
	xor eax, eax
```
## `checked_round_to_multiple_u32_unb_pow2`
```asm
	xor eax, eax
	cmp dl, 31
	ja .LBB142_2
	mov r8d, ecx
	mov r9d, 1
	mov ecx, edx
	shl r9d, cl
	mov edx, r9d
	shr edx
	add edx, r8d
	jb .LBB142_2
	neg r9d
	and edx, r9d
	mov eax, 1
	ret
.LBB142_2:
```
## `checked_round_to_multiple_u64_unb_pow2`
```asm
	cmp dl, 63
	ja .LBB143_2
	mov r8, rcx
	mov r9d, 1
	mov ecx, edx
	shl r9, cl
	mov eax, 1
	mov rdx, r9
	shr rdx
	add rdx, r8
	jb .LBB143_2
	neg r9
	and rdx, r9
	ret
.LBB143_2:
	xor eax, eax
```
## `checked_round_to_multiple_u128_unb_pow2`
```asm
	push rsi
	push rdi
	mov rax, rcx
	test r9b, r9b
	js .LBB140_2
	mov r10d, 1
	xor r11d, r11d
	mov ecx, r9d
	shld r11, r10, cl
	xor edi, edi
	mov esi, 1
	shl rsi, cl
	test r9b, 64
	cmovne r11, rsi
	cmovne rsi, rdi
	mov rcx, r11
	mov r9, r11
	shld r9, rsi, 63
	shr rcx
	add r9, rdx
	adc rcx, r8
	jb .LBB140_2
	xor edx, edx
	neg rsi
	sbb rdx, r11
	and rcx, rdx
	and r9, rsi
	mov qword ptr [rax + 16], r9
	mov qword ptr [rax + 24], rcx
	jmp .LBB140_3
.LBB140_2:
	xor r10d, r10d
.LBB140_3:
	mov qword ptr [rax], r10
	mov qword ptr [rax + 8], 0
	pop rdi
	pop rsi
```
## `div_ceil_i8_pow2`
```asm
	mov eax, ecx
	mov r8b, -1
	mov ecx, edx
	shl r8b, cl
	not r8b
	and r8b, al
	sar al, cl
	cmp r8b, 1
	sbb al, -1
```
## `div_ceil_i8_unb_pow2`
```asm
	mov eax, ecx
	and dl, 7
	mov r8b, -1
	mov ecx, edx
	shl r8b, cl
	not r8b
	and r8b, al
	sar al, cl
	cmp r8b, 1
	sbb al, -1
```
## `div_ceil_i16_pow2`
```asm
	mov r8d, ecx
	mov r9d, -1
	mov ecx, edx
	shl r9d, cl
	movsx eax, r8w
	not r9d
	and r9d, r8d
	sar eax, cl
	cmp r9w, 1
	sbb ax, -1
```
## `div_ceil_i16_unb_pow2`
```asm
	mov r8d, ecx
	and dl, 15
	mov r9d, -1
	mov ecx, edx
	shl r9d, cl
	movsx eax, r8w
	not r9d
	and r9d, r8d
	sar eax, cl
	cmp r9w, 1
	sbb ax, -1
```
## `div_ceil_i32_pow2`, `div_ceil_i32_unb_pow2`
```asm
	mov eax, ecx
	mov r8d, -1
	mov ecx, edx
	shl r8d, cl
	not r8d
	and r8d, eax
	sar eax, cl
	cmp r8d, 1
	sbb eax, -1
```
## `div_ceil_i64_pow2`, `div_ceil_i64_unb_pow2`
```asm
	mov rax, rcx
	mov r8, -1
	mov ecx, edx
	shl r8, cl
	not r8
	and r8, rax
	sar rax, cl
	cmp r8, 1
	sbb rax, -1
```
## `div_ceil_i128_pow2`, `div_ceil_i128_unb_pow2`
```asm
	push rsi
	push rdi
	mov rax, rcx
	mov r11, -1
	mov ecx, r8d
	shl r11, cl
	mov r9, rdx
	mov r10, rax
	shrd r10, rdx, cl
	mov rsi, -1
	mov rdi, rdx
	sar rdi, cl
	sar rdx, 63
	xor ecx, ecx
	test r8b, 64
	cmovne rsi, r11
	cmove rcx, r11
	not rcx
	not rsi
	cmovne r10, rdi
	cmove rdx, rdi
	and rsi, r9
	and rcx, rax
	xor eax, eax
	or rcx, rsi
	setne al
	add rax, r10
	adc rdx, 0
	pop rdi
	pop rsi
```
## `div_ceil_u8_pow2`
```asm
	mov eax, ecx
	mov r8b, -1
	mov ecx, edx
	shl r8b, cl
	not r8b
	and r8b, al
	shr al, cl
	cmp r8b, 1
	sbb al, -1
```
## `div_ceil_u8_unb_pow2`
```asm
	mov eax, ecx
	and dl, 7
	mov r8b, -1
	mov ecx, edx
	shl r8b, cl
	not r8b
	and r8b, al
	shr al, cl
	cmp r8b, 1
	sbb al, -1
```
## `div_ceil_u16_pow2`
```asm
	mov r8d, ecx
	mov r9d, -1
	mov ecx, edx
	shl r9d, cl
	movzx eax, r8w
	not r9d
	and r9d, r8d
	shr eax, cl
	cmp r9w, 1
	sbb ax, -1
```
## `div_ceil_u16_unb_pow2`
```asm
	mov r8d, ecx
	and dl, 15
	mov r9d, -1
	mov ecx, edx
	shl r9d, cl
	movzx eax, r8w
	not r9d
	and r9d, r8d
	shr eax, cl
	cmp r9w, 1
	sbb ax, -1
```
## `div_ceil_u32_pow2`, `div_ceil_u32_unb_pow2`
```asm
	mov eax, ecx
	mov r8d, -1
	mov ecx, edx
	shl r8d, cl
	not r8d
	and r8d, eax
	shr eax, cl
	cmp r8d, 1
	sbb eax, -1
```
## `div_ceil_u64_pow2`, `div_ceil_u64_unb_pow2`
```asm
	mov rax, rcx
	mov r8, -1
	mov ecx, edx
	shl r8, cl
	not r8
	and r8, rax
	shr rax, cl
	cmp r8, 1
	sbb rax, -1
```
## `div_ceil_u128_pow2`, `div_ceil_u128_unb_pow2`
```asm
	push rsi
	mov rax, rcx
	mov r11, -1
	mov ecx, r8d
	shl r11, cl
	mov r9, rdx
	mov r10, rax
	shrd r10, rdx, cl
	shr rdx, cl
	mov rcx, -1
	xor esi, esi
	test r8b, 64
	cmovne rcx, r11
	cmovne r11, rsi
	not r11
	not rcx
	cmovne r10, rdx
	cmovne rdx, rsi
	and rcx, r9
	and r11, rax
	xor eax, eax
	or r11, rcx
	setne al
	add rax, r10
	adc rdx, 0
	pop rsi
```
## `div_floor_i8_pow2`
```asm
	mov eax, ecx
	mov ecx, edx
	sar al, cl
```
## `div_floor_i8_unb_pow2`
```asm
	mov eax, ecx
	and dl, 7
	mov ecx, edx
	sar al, cl
```
## `div_floor_i16_pow2`
```asm
	movsx eax, cx
	mov ecx, edx
	sar eax, cl
```
## `div_floor_i16_unb_pow2`
```asm
	movsx eax, cx
	and dl, 15
	mov ecx, edx
	sar eax, cl
```
## `div_floor_i32_pow2`, `div_floor_i32_unb_pow2`
```asm
	mov eax, ecx
	mov ecx, edx
	sar eax, cl
```
## `div_floor_i64_pow2`, `div_floor_i64_unb_pow2`
```asm
	mov rax, rcx
	mov ecx, edx
	sar rax, cl
```
## `div_floor_i128_pow2`, `div_floor_i128_unb_pow2`
```asm
	mov rax, rcx
	mov ecx, r8d
	shrd rax, rdx, cl
	mov r9, rdx
	sar r9, cl
	sar rdx, 63
	test r8b, 64
	cmovne rax, r9
	cmove rdx, r9
```
## `div_floor_u8_pow2`
```asm
	mov eax, ecx
	mov ecx, edx
	shr al, cl
```
## `div_floor_u8_unb_pow2`, `div_u8_pow2`, `div_u8_unb_pow2`
```asm
	mov eax, ecx
	and dl, 7
	mov ecx, edx
	shr al, cl
```
## `div_floor_u16_pow2`
```asm
	movzx eax, cx
	mov ecx, edx
	shr eax, cl
```
## `div_floor_u16_unb_pow2`, `div_u16_pow2`, `div_u16_unb_pow2`
```asm
	movzx eax, cx
	and dl, 15
	mov ecx, edx
	shr eax, cl
```
## `div_floor_u32_pow2`, `div_floor_u32_unb_pow2`, `div_u32_pow2`, `div_u32_unb_pow2`
```asm
	mov eax, ecx
	mov ecx, edx
	shr eax, cl
```
## `div_floor_u64_pow2`, `div_floor_u64_unb_pow2`, `div_u64_pow2`, `div_u64_unb_pow2`
```asm
	mov rax, rcx
	mov ecx, edx
	shr rax, cl
```
## `div_floor_u128_pow2`, `div_floor_u128_unb_pow2`, `div_u128_pow2`, `div_u128_unb_pow2`
```asm
	mov rax, rcx
	mov ecx, r8d
	shrd rax, rdx, cl
	shr rdx, cl
	xor ecx, ecx
	test r8b, 64
	cmovne rax, rdx
	cmovne rdx, rcx
```
## `div_i8_pow2`
```asm
	mov r8d, ecx
	mov r9b, -1
	mov ecx, edx
	shl r9b, cl
	not r9b
	mov eax, r8d
	sar al, 7
	and al, r9b
	add al, r8b
	and dl, 7
	mov ecx, edx
	sar al, cl
```
## `div_i8_unb_pow2`
```asm
	mov r8d, ecx
	and dl, 7
	mov r9b, -1
	mov ecx, edx
	shl r9b, cl
	not r9b
	mov eax, r8d
	sar al, 7
	and al, r9b
	add al, r8b
	sar al, cl
```
## `div_i16_pow2`
```asm
	mov eax, ecx
	mov r8d, -1
	mov ecx, edx
	shl r8d, cl
	movsx ecx, ax
	not r8d
	sar ecx, 15
	and ecx, r8d
	add ecx, eax
	movsx eax, cx
	and dl, 15
	mov ecx, edx
	sar eax, cl
```
## `div_i16_unb_pow2`
```asm
	mov eax, ecx
	and dl, 15
	mov r8d, -1
	mov ecx, edx
	shl r8d, cl
	movsx ecx, ax
	not r8d
	sar ecx, 15
	and ecx, r8d
	add ecx, eax
	movsx eax, cx
	mov ecx, edx
	sar eax, cl
```
## `div_i32_pow2`, `div_i32_unb_pow2`
```asm
	mov r8d, ecx
	mov r9d, -1
	mov ecx, edx
	shl r9d, cl
	not r9d
	mov eax, r8d
	sar eax, 31
	and eax, r9d
	add eax, r8d
	sar eax, cl
```
## `div_i64_pow2`, `div_i64_unb_pow2`
```asm
	mov r8, rcx
	mov r9, -1
	mov ecx, edx
	shl r9, cl
	not r9
	mov rax, r8
	sar rax, 63
	and rax, r9
	add rax, r8
	sar rax, cl
```
## `div_i128_pow2`, `div_i128_unb_pow2`
```asm
	mov r9, rcx
	mov r10, -1
	mov rax, -1
	mov ecx, r8d
	shl rax, cl
	xor ecx, ecx
	test r8b, 64
	cmovne r10, rax
	cmove rcx, rax
	not rcx
	not r10
	mov rax, rdx
	sar rax, 63
	and r10, rax
	and rax, rcx
	add rax, r9
	adc r10, rdx
	mov r9, r10
	mov ecx, r8d
	sar r9, cl
	mov rdx, r10
	sar rdx, 63
	test r8b, 64
	cmove rdx, r9
	shrd rax, r10, cl
	test r8b, 64
	cmovne rax, r9
```
## `div_round_i8_pow2`
```asm
	mov r8d, ecx
	mov r9b, -1
	mov ecx, edx
	shl r9b, cl
	mov eax, r8d
	sar al, cl
	not r9b
	and r9b, r8b
	shr r8b, 7
	xor ecx, ecx
	sub r9b, r8b
	movzx r8d, r9b
	cmovb r8d, ecx
	add r8b, r8b
	movzx ecx, r8b
	movzx edx, dl
	bt ecx, edx
	adc al, 0
```
## `div_round_i8_unb_pow2`
```asm
	mov r8d, ecx
	and dl, 7
	mov r9b, -1
	mov ecx, edx
	shl r9b, cl
	mov eax, r8d
	sar al, cl
	not r9b
	and r9b, r8b
	shr r8b, 7
	xor ecx, ecx
	sub r9b, r8b
	movzx r8d, r9b
	cmovb r8d, ecx
	add r8b, r8b
	movzx ecx, r8b
	movzx edx, dl
	bt ecx, edx
	adc al, 0
```
## `div_round_i16_pow2`
```asm
	mov r8d, ecx
	movzx r9d, cx
	movsx eax, r9w
	mov r10d, -1
	mov ecx, edx
	shl r10d, cl
	not r10d
	sar eax, cl
	and r10d, r8d
	shr r9d, 15
	xor ecx, ecx
	sub r10w, r9w
	cmovb r10d, ecx
	add r10d, r10d
	movzx ecx, dl
	bt r10d, ecx
	adc ax, 0
```
## `div_round_i16_unb_pow2`
```asm
	mov r8d, ecx
	movzx r9d, cx
	movsx eax, r9w
	and dl, 15
	mov r10d, -1
	mov ecx, edx
	shl r10d, cl
	not r10d
	sar eax, cl
	and r10d, r8d
	shr r9d, 15
	xor ecx, ecx
	sub r10w, r9w
	cmovb r10d, ecx
	add r10d, r10d
	movzx ecx, dl
	bt r10d, ecx
	adc ax, 0
```
## `div_round_i32_pow2`, `div_round_i32_unb_pow2`
```asm
	mov r8d, ecx
	mov r9d, -1
	mov ecx, edx
	shl r9d, cl
	not r9d
	mov eax, r8d
	sar eax, cl
	and r9d, r8d
	shr r8d, 31
	xor ecx, ecx
	sub r9d, r8d
	cmovb r9d, ecx
	add r9d, r9d
	movzx ecx, dl
	bt r9d, ecx
	adc eax, 0
```
## `div_round_i64_pow2`, `div_round_i64_unb_pow2`
```asm
	mov r8, rcx
	mov r9, -1
	mov ecx, edx
	shl r9, cl
	not r9
	mov rax, r8
	sar rax, cl
	and r9, r8
	shr r8, 63
	xor ecx, ecx
	sub r9, r8
	cmovae rcx, r9
	add rcx, rcx
	movzx edx, dl
	bt rcx, rdx
	adc rax, 0
```
## `div_round_i128_pow2`, `div_round_i128_unb_pow2`
```asm
	mov r9d, r8d
	mov r8, rcx
	mov rax, -1
	mov r10, -1
	mov ecx, r9d
	shl r10, cl
	xor ecx, ecx
	test r9b, 64
	cmovne rax, r10
	cmovne r10, rcx
	not r10
	not rax
	and rax, rdx
	and r10, r8
	mov r11, rdx
	shr r11, 63
	sub r10, r11
	sbb rax, 0
	cmovb rax, rcx
	cmovb r10, rcx
	shld rax, r10, 1
	add r10, r10
	mov ecx, r9d
	shrd r10, rax, cl
	shr rax, cl
	test r9b, 64
	cmove rax, r10
	mov r11, rdx
	sar r11, cl
	mov r10, rdx
	sar r10, 63
	test r9b, 64
	cmove r10, r11
	shrd r8, rdx, cl
	test r9b, 64
	cmovne r8, r11
	and eax, 1
	add rax, r8
	adc r10, 0
	mov rdx, r10
```
## `div_round_u8_pow2`
```asm
	mov eax, ecx
	mov r8b, 1
	mov ecx, edx
	shl r8b, cl
	shr r8b
	and r8b, al
	shr al, cl
	cmp r8b, 1
	sbb al, -1
```
## `div_round_u8_unb_pow2`
```asm
	mov eax, ecx
	and dl, 7
	mov r8b, 1
	mov ecx, edx
	shl r8b, cl
	shr r8b
	and r8b, al
	shr al, cl
	cmp r8b, 1
	sbb al, -1
```
## `div_round_u16_pow2`
```asm
	mov r8d, ecx
	mov r9d, 1
	mov ecx, edx
	shl r9d, cl
	movzx eax, r8w
	movzx r9d, r9w
	shr r9d
	and r9d, r8d
	shr eax, cl
	cmp r9w, 1
	sbb ax, -1
```
## `div_round_u16_unb_pow2`
```asm
	mov r8d, ecx
	and dl, 15
	mov r9d, 1
	mov ecx, edx
	shl r9d, cl
	movzx eax, r8w
	shr r9d
	and r9d, r8d
	shr eax, cl
	cmp r9w, 1
	sbb ax, -1
```
## `div_round_u32_pow2`, `div_round_u32_unb_pow2`
```asm
	mov eax, ecx
	mov r8d, 1
	mov ecx, edx
	shl r8d, cl
	shr r8d
	and r8d, eax
	shr eax, cl
	cmp r8d, 1
	sbb eax, -1
```
## `div_round_u64_pow2`, `div_round_u64_unb_pow2`
```asm
	mov rax, rcx
	mov r8d, 1
	mov ecx, edx
	shl r8, cl
	shr r8
	and r8, rax
	shr rax, cl
	cmp r8, 1
	sbb rax, -1
```
## `div_round_u128_pow2`, `div_round_u128_unb_pow2`
```asm
	push rsi
	push rdi
	push rbx
	mov r9d, r8d
	mov r8, rdx
	mov rax, rcx
	mov rsi, rdx
	mov ecx, r9d
	shr rsi, cl
	xor r11d, r11d
	test r9b, 64
	mov rdx, rsi
	cmovne rdx, r11
	mov r10, rax
	shrd r10, r8, cl
	test r9b, 64
	cmovne r10, rsi
	mov esi, 1
	mov edi, 1
	shl rdi, cl
	test r9b, 64
	mov rbx, rdi
	cmovne rbx, r11
	shld r11, rsi, cl
	test r9b, 64
	cmovne r11, rdi
	shrd rbx, r11, 1
	shr r11
	and r11, r8
	and rbx, rax
	xor eax, eax
	or rbx, r11
	setne al
	add rax, r10
	adc rdx, 0
	pop rbx
	pop rdi
	pop rsi
```
## `floor_to_multiple_i8_pow2`, `floor_to_multiple_u8_pow2`
```asm
	mov eax, ecx
	mov ecx, edx
	shr al, cl
	shl al, cl
```
## `floor_to_multiple_i8_unb_pow2`, `floor_to_multiple_u8_unb_pow2`
```asm
	mov eax, ecx
	and dl, 7
	mov ecx, edx
	shr al, cl
	shl al, cl
```
## `floor_to_multiple_i16_pow2`, `floor_to_multiple_u16_pow2`
```asm
	movzx eax, cx
	mov ecx, edx
	shr eax, cl
	shl eax, cl
```
## `floor_to_multiple_i16_unb_pow2`, `floor_to_multiple_u16_unb_pow2`
```asm
	movzx eax, cx
	and dl, 15
	mov ecx, edx
	shr eax, cl
	shl eax, cl
```
## `floor_to_multiple_i32_pow2`, `floor_to_multiple_i32_unb_pow2`, `floor_to_multiple_u32_pow2`, `floor_to_multiple_u32_unb_pow2`
```asm
	mov eax, ecx
	mov ecx, edx
	shr eax, cl
	shl eax, cl
```
## `floor_to_multiple_i64_pow2`, `floor_to_multiple_i64_unb_pow2`, `floor_to_multiple_u64_pow2`, `floor_to_multiple_u64_unb_pow2`
```asm
	mov rax, rcx
	mov ecx, edx
	shr rax, cl
	shl rax, cl
```
## `floor_to_multiple_i128_pow2`, `floor_to_multiple_i128_unb_pow2`, `floor_to_multiple_u128_pow2`, `floor_to_multiple_u128_unb_pow2`
```asm
	mov r9, rcx
	mov r10, -1
	mov ecx, r8d
	shl r10, cl
	mov rcx, -1
	xor eax, eax
	test r8b, 64
	cmovne rcx, r10
	cmove rax, r10
	and rax, r9
	and rdx, rcx
```
## `is_multiple_of_i8_pow2`, `is_multiple_of_i8_unb_pow2`, `is_multiple_of_u8_pow2`, `is_multiple_of_u8_unb_pow2`
```asm
	movzx eax, cl
	or eax, 256
	rep bsf	eax, eax
	cmp al, dl
	setae al
```
## `is_multiple_of_i16_pow2`, `is_multiple_of_i16_unb_pow2`, `is_multiple_of_u16_pow2`, `is_multiple_of_u16_unb_pow2`
```asm
	or ecx, 65536
	rep bsf	eax, ecx
	movzx ecx, dl
	cmp ax, cx
	setae al
```
## `is_multiple_of_i32_pow2`, `is_multiple_of_i32_unb_pow2`, `is_multiple_of_u32_pow2`, `is_multiple_of_u32_unb_pow2`
```asm
	mov eax, 32
	rep bsf	eax, ecx
	movzx ecx, dl
	cmp eax, ecx
	setae al
```
## `is_multiple_of_i64_pow2`, `is_multiple_of_i64_unb_pow2`, `is_multiple_of_u64_pow2`, `is_multiple_of_u64_unb_pow2`
```asm
	mov eax, 64
	rep bsf	rax, rcx
	movzx ecx, dl
	cmp eax, ecx
	setae al
```
## `is_multiple_of_i128_pow2`, `is_multiple_of_i128_unb_pow2`, `is_multiple_of_u128_pow2`, `is_multiple_of_u128_unb_pow2`
```asm
	rep bsf	rax, rcx
	mov r9d, 64
	rep bsf	r9, rdx
	add r9d, 64
	test rcx, rcx
	cmovne r9d, eax
	movzx eax, r8b
	cmp r9d, eax
	setae al
```
## `mul_i8_pow2`, `mul_u8_pow2`
```asm
	mov eax, ecx
	mov ecx, edx
	shl al, cl
```
## `mul_i8_unb_pow2`, `mul_u8_unb_pow2`
```asm
	mov eax, ecx
	and dl, 7
	mov ecx, edx
	shl al, cl
```
## `mul_i16_pow2`, `mul_i32_pow2`, `mul_i32_unb_pow2`, `mul_u16_pow2`, `mul_u32_pow2`, `mul_u32_unb_pow2`
```asm
	mov eax, ecx
	mov ecx, edx
	shl eax, cl
```
## `mul_i16_unb_pow2`, `mul_u16_unb_pow2`
```asm
	mov eax, ecx
	and dl, 15
	mov ecx, edx
	shl eax, cl
```
## `mul_i64_pow2`, `mul_i64_unb_pow2`, `mul_u64_pow2`, `mul_u64_unb_pow2`
```asm
	mov rax, rcx
	mov ecx, edx
	shl rax, cl
```
## `mul_i128_pow2`, `mul_i128_unb_pow2`, `mul_u128_pow2`, `mul_u128_unb_pow2`
```asm
	mov r9, rcx
	mov ecx, r8d
	shld rdx, r9, cl
	shl r9, cl
	xor eax, eax
	test r8b, 64
	cmovne rdx, r9
	cmove rax, r9
```
## `rem_floor_i8_pow2`, `rem_floor_u8_pow2`
```asm
	mov r8d, ecx
	mov al, -1
	mov ecx, edx
	shl al, cl
	not al
	and al, r8b
```
## `rem_floor_i8_unb_pow2`, `rem_floor_u8_unb_pow2`, `rem_u8_unb_pow2`
```asm
	mov r8d, ecx
	and dl, 7
	mov al, -1
	mov ecx, edx
	shl al, cl
	not al
	and al, r8b
```
## `rem_floor_i16_pow2`, `rem_floor_i32_pow2`, `rem_floor_i32_unb_pow2`, `rem_floor_u16_pow2`, `rem_floor_u32_pow2`, `rem_floor_u32_unb_pow2`, `rem_u32_unb_pow2`
```asm
	mov r8d, ecx
	mov eax, -1
	mov ecx, edx
	shl eax, cl
	not eax
	and eax, r8d
```
## `rem_floor_i16_unb_pow2`, `rem_floor_u16_unb_pow2`, `rem_u16_unb_pow2`
```asm
	mov r8d, ecx
	and dl, 15
	mov eax, -1
	mov ecx, edx
	shl eax, cl
	not eax
	and eax, r8d
```
## `rem_floor_i64_pow2`, `rem_floor_i64_unb_pow2`, `rem_floor_u64_pow2`, `rem_floor_u64_unb_pow2`, `rem_u64_unb_pow2`
```asm
	mov r8, rcx
	mov rax, -1
	mov ecx, edx
	shl rax, cl
	not rax
	and rax, r8
```
## `rem_floor_i128_pow2`, `rem_floor_i128_unb_pow2`, `rem_floor_u128_pow2`, `rem_floor_u128_unb_pow2`, `rem_u128_unb_pow2`
```asm
	mov r9, rcx
	mov r10, -1
	mov r11, -1
	mov ecx, r8d
	shl r11, cl
	xor eax, eax
	test r8b, 64
	cmove rax, r11
	cmove r11, r10
	not r11
	not rax
	and rax, r9
	and rdx, r11
```
## `rem_i8_pow2`
```asm
	mov eax, ecx
	mov r8b, -1
	mov ecx, edx
	shl r8b, cl
	not r8b
	mov r9d, eax
	sar r9b, 7
	and r9b, r8b
	add r9b, al
	and cl, 7
	sar r9b, cl
	mov ecx, edx
	shl r9b, cl
	sub al, r9b
```
## `rem_i8_unb_pow2`
```asm
	mov eax, ecx
	and dl, 7
	mov r8b, -1
	mov ecx, edx
	shl r8b, cl
	mov ecx, r8d
	not cl
	mov edx, eax
	sar dl, 7
	and dl, cl
	add dl, al
	and dl, r8b
	sub al, dl
```
## `rem_i16_pow2`
```asm
	mov eax, ecx
	mov r8d, -1
	mov ecx, edx
	shl r8d, cl
	movsx ecx, ax
	not r8d
	sar ecx, 15
	and ecx, r8d
	add ecx, eax
	movsx r8d, cx
	mov ecx, edx
	and cl, 15
	sar r8d, cl
	mov ecx, edx
	shl r8d, cl
	sub eax, r8d
```
## `rem_i16_unb_pow2`
```asm
	mov eax, ecx
	movsx r8d, cx
	and dl, 15
	mov r9d, -1
	mov ecx, edx
	shl r9d, cl
	mov ecx, r9d
	not ecx
	sar r8d, 15
	and r8d, ecx
	add r8d, eax
	and r8d, r9d
	sub eax, r8d
```
## `rem_i32_pow2`
```asm
	mov eax, ecx
	mov r8d, -1
	mov ecx, edx
	shl r8d, cl
	not r8d
	mov r9d, eax
	sar r9d, 31
	and r9d, r8d
	add r9d, eax
	sar r9d, cl
	shl r9d, cl
	sub eax, r9d
```
## `rem_i32_unb_pow2`
```asm
	mov eax, ecx
	mov r8d, -1
	mov ecx, edx
	shl r8d, cl
	mov ecx, r8d
	not ecx
	mov edx, eax
	sar edx, 31
	and edx, ecx
	add edx, eax
	and edx, r8d
	sub eax, edx
```
## `rem_i64_pow2`
```asm
	mov rax, rcx
	mov r8, -1
	mov ecx, edx
	shl r8, cl
	not r8
	mov r9, rax
	sar r9, 63
	and r9, r8
	add r9, rax
	sar r9, cl
	shl r9, cl
	sub rax, r9
```
## `rem_i64_unb_pow2`
```asm
	mov rax, rcx
	mov r8, -1
	mov ecx, edx
	shl r8, cl
	mov rcx, r8
	not rcx
	mov rdx, rax
	sar rdx, 63
	and rdx, rcx
	add rdx, rax
	and rdx, r8
	sub rax, rdx
```
## `rem_i128_pow2`
```asm
	push rsi
	push rdi
	mov rax, rcx
	mov r9, -1
	mov rsi, -1
	mov ecx, r8d
	shl rsi, cl
	xor r10d, r10d
	test r8b, 64
	cmovne r9, rsi
	cmovne rsi, r10
	not rsi
	not r9
	mov r11, rdx
	sar r11, 63
	and r9, r11
	and r11, rsi
	add r11, rax
	adc r9, rdx
	shrd r11, r9, cl
	mov rsi, r9
	sar rsi, cl
	test r8b, 64
	cmovne r11, rsi
	mov rdi, r11
	shl rdi, cl
	test r8b, 64
	cmove r10, rdi
	sar r9, 63
	test r8b, 64
	cmove r9, rsi
	shld r9, r11, cl
	test r8b, 64
	cmovne r9, rdi
	sub rax, r10
	sbb rdx, r9
	pop rdi
	pop rsi
```
## `rem_i128_unb_pow2`
```asm
	mov rax, rcx
	mov r9, -1
	mov ecx, r8d
	shl r9, cl
	mov rcx, -1
	xor r10d, r10d
	test r8b, 64
	cmovne rcx, r9
	cmove r10, r9
	mov r8, r10
	not r8
	mov r9, rcx
	not r9
	mov r11, rdx
	sar r11, 63
	and r9, r11
	and r11, r8
	add r11, rax
	adc r9, rdx
	and r9, rcx
	and r11, r10
	sub rax, r11
	sbb rdx, r9
```
## `rem_u8_pow2`
```asm
	mov eax, ecx
	mov ecx, edx
	and cl, 7
	mov r8d, eax
	shr r8b, cl
	mov ecx, edx
	shl r8b, cl
	sub al, r8b
```
## `rem_u16_pow2`
```asm
	mov eax, ecx
	movzx r8d, cx
	mov ecx, edx
	and cl, 15
	shr r8d, cl
	mov ecx, edx
	shl r8d, cl
	sub eax, r8d
```
## `rem_u32_pow2`
```asm
	mov eax, ecx
	mov r8d, ecx
	mov ecx, edx
	shr r8d, cl
	shl r8d, cl
	sub eax, r8d
```
## `rem_u64_pow2`
```asm
	mov rax, rcx
	mov r8, rcx
	mov ecx, edx
	shr r8, cl
	shl r8, cl
	sub rax, r8
```
## `rem_u128_pow2`
```asm
	push rsi
	mov rax, rcx
	mov r9, rcx
	mov ecx, r8d
	shrd r9, rdx, cl
	mov r10, rdx
	shr r10, cl
	test r8b, 64
	cmovne r9, r10
	mov r11, r9
	shl r11, cl
	xor esi, esi
	test r8b, 64
	cmovne r10, rsi
	cmove rsi, r11
	shld r10, r9, cl
	test r8b, 64
	cmovne r10, r11
	sub rax, rsi
	sbb rdx, r10
	pop rsi
```
## `round_to_multiple_i8_pow2`
```asm
	mov r8d, ecx
	mov al, 1
	mov ecx, edx
	shl al, cl
	mov ecx, eax
	shr cl
	mov edx, r8d
	shr dl, 7
	xor r9d, r9d
	sub cl, dl
	movzx ecx, cl
	cmovb ecx, r9d
	add cl, r8b
	neg al
	and al, cl
```
## `round_to_multiple_i8_unb_pow2`
```asm
	mov r8d, ecx
	and dl, 7
	mov al, 1
	mov ecx, edx
	shl al, cl
	mov ecx, eax
	shr cl
	mov edx, r8d
	shr dl, 7
	xor r9d, r9d
	sub cl, dl
	movzx ecx, cl
	cmovb ecx, r9d
	add cl, r8b
	neg al
	and al, cl
```
## `round_to_multiple_i16_pow2`
```asm
	mov r8d, ecx
	mov eax, 1
	mov ecx, edx
	shl eax, cl
	movzx ecx, r8w
	movzx edx, ax
	shr edx
	shr ecx, 15
	xor r9d, r9d
	sub dx, cx
	cmovb edx, r9d
	add edx, r8d
	neg eax
	and eax, edx
```
## `round_to_multiple_i16_unb_pow2`
```asm
	mov r8d, ecx
	and dl, 15
	mov eax, 1
	mov ecx, edx
	shl eax, cl
	movzx ecx, r8w
	mov edx, eax
	shr edx
	shr ecx, 15
	xor r9d, r9d
	sub dx, cx
	cmovb edx, r9d
	add edx, r8d
	neg eax
	and eax, edx
```
## `round_to_multiple_i32_pow2`, `round_to_multiple_i32_unb_pow2`
```asm
	mov r8d, ecx
	mov eax, 1
	mov ecx, edx
	shl eax, cl
	mov ecx, eax
	shr ecx
	mov edx, r8d
	shr edx, 31
	xor r9d, r9d
	sub ecx, edx
	cmovb ecx, r9d
	add ecx, r8d
	neg eax
	and eax, ecx
```
## `round_to_multiple_i64_pow2`, `round_to_multiple_i64_unb_pow2`
```asm
	mov r8, rcx
	mov eax, 1
	mov ecx, edx
	shl rax, cl
	mov rcx, rax
	shr rcx
	mov rdx, r8
	shr rdx, 63
	xor r9d, r9d
	sub rcx, rdx
	cmovae r9, rcx
	add r9, r8
	neg rax
	and rax, r9
```
## `round_to_multiple_i128_pow2`, `round_to_multiple_i128_unb_pow2`
```asm
	push rsi
	mov r9d, r8d
	mov r8, rcx
	mov eax, 1
	xor r10d, r10d
	mov ecx, r9d
	shld r10, rax, cl
	shl rax, cl
	xor ecx, ecx
	test r9b, 64
	cmovne r10, rax
	cmovne rax, rcx
	mov r9, r10
	shr r9
	mov r11, r10
	shld r11, rax, 63
	mov rsi, rdx
	shr rsi, 63
	sub r11, rsi
	sbb r9, 0
	cmovb r9, rcx
	cmovb r11, rcx
	add r11, r8
	adc rdx, r9
	neg rax
	sbb rcx, r10
	and rdx, rcx
	and rax, r11
	pop rsi
```
## `round_to_multiple_u8_pow2`
```asm
	mov r8d, ecx
	mov al, 1
	mov ecx, edx
	shl al, cl
	mov ecx, eax
	shr cl
	add cl, r8b
	neg al
	and al, cl
```
## `round_to_multiple_u8_unb_pow2`
```asm
	mov r8d, ecx
	and dl, 7
	mov al, 1
	mov ecx, edx
	shl al, cl
	mov ecx, eax
	shr cl
	add cl, r8b
	neg al
	and al, cl
```
## `round_to_multiple_u16_pow2`
```asm
	mov r8d, ecx
	mov eax, 1
	mov ecx, edx
	shl eax, cl
	movzx ecx, ax
	shr ecx
	add ecx, r8d
	neg eax
	and eax, ecx
```
## `round_to_multiple_u16_unb_pow2`
```asm
	mov r8d, ecx
	and dl, 15
	mov eax, 1
	mov ecx, edx
	shl eax, cl
	mov ecx, eax
	shr ecx
	add ecx, r8d
	neg eax
	and eax, ecx
```
## `round_to_multiple_u32_pow2`, `round_to_multiple_u32_unb_pow2`
```asm
	mov r8d, ecx
	mov eax, 1
	mov ecx, edx
	shl eax, cl
	mov ecx, eax
	shr ecx
	add ecx, r8d
	neg eax
	and eax, ecx
```
## `round_to_multiple_u64_pow2`, `round_to_multiple_u64_unb_pow2`
```asm
	mov r8, rcx
	mov eax, 1
	mov ecx, edx
	shl rax, cl
	mov rcx, rax
	shr rcx
	add rcx, r8
	neg rax
	and rax, rcx
```
## `round_to_multiple_u128_pow2`, `round_to_multiple_u128_unb_pow2`
```asm
	mov r9, rcx
	mov eax, 1
	xor r10d, r10d
	mov ecx, r8d
	shld r10, rax, cl
	xor r11d, r11d
	shl rax, cl
	test r8b, 64
	cmovne r10, rax
	cmovne rax, r11
	mov rcx, r10
	mov r8, r10
	shld r8, rax, 63
	shr rcx
	add r8, r9
	adc rdx, rcx
	neg rax
	sbb r11, r10
	and rdx, r11
	and rax, r8
```
## `unbounded_ceil_to_multiple_i8_unb_pow2`
```asm
	mov r8d, ecx
	cmp dl, 8
	jae .LBB349_1
	mov r9b, -1
	mov ecx, edx
	shl r9b, cl
	mov eax, r9d
	not al
	add r8b, al
	setno al
	and r8b, r9b
	mov edx, r8d
	ret
.LBB349_1:
	test r8b, r8b
	setle al
	xor r8d, r8d
	mov edx, r8d
```
## `unbounded_ceil_to_multiple_i16_unb_pow2`
```asm
	mov r8d, ecx
	cmp dl, 16
	jae .LBB346_1
	mov r9d, -1
	mov ecx, edx
	shl r9d, cl
	mov eax, r9d
	not eax
	add r8w, ax
	setno al
	and r8d, r9d
	movzx eax, al
	mov edx, r8d
	ret
.LBB346_1:
	test r8w, r8w
	setle al
	xor r8d, r8d
	movzx eax, al
	mov edx, r8d
```
## `unbounded_ceil_to_multiple_i32_unb_pow2`
```asm
	mov r8d, ecx
	cmp dl, 32
	jae .LBB347_1
	mov r9d, -1
	mov ecx, edx
	shl r9d, cl
	mov eax, r9d
	not eax
	add r8d, eax
	setno al
	and r8d, r9d
	movzx eax, al
	mov edx, r8d
	ret
.LBB347_1:
	test r8d, r8d
	setle al
	xor r8d, r8d
	movzx eax, al
	mov edx, r8d
```
## `unbounded_ceil_to_multiple_i64_unb_pow2`
```asm
	mov r8, rcx
	cmp dl, 64
	jae .LBB348_1
	mov r9, -1
	mov ecx, edx
	shl r9, cl
	mov rax, r9
	not rax
	add r8, rax
	setno al
	and r8, r9
	movzx eax, al
	mov rdx, r8
	ret
.LBB348_1:
	test r8, r8
	setle al
	xor r8d, r8d
	movzx eax, al
	mov rdx, r8
```
## `unbounded_ceil_to_multiple_i128_unb_pow2`
```asm
	push rsi
	mov rax, rcx
	test r9b, r9b
	js .LBB345_4
	mov r10, -1
	mov ecx, r9d
	shl r10, cl
	mov r11, -1
	xor ecx, ecx
	test r9b, 64
	mov r9, r10
	cmovne r9, rcx
	cmove r10, r11
	mov r11, r10
	not r11
	mov rsi, r9
	not rsi
	add rdx, rsi
	adc r8, r11
	jo .LBB345_5
	and rdx, r9
	and r8, r10
	mov rcx, rdx
.LBB345_3:
	mov qword ptr [rax + 16], rcx
	mov qword ptr [rax + 24], r8
	mov ecx, 1
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	pop rsi
	ret
.LBB345_4:
	xor ecx, ecx
	cmp rdx, 1
	sbb r8, 0
	mov r8d, 0
	jl .LBB345_3
.LBB345_5:
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	pop rsi
```
## `unbounded_ceil_to_multiple_u8_unb_pow2`
```asm
	mov eax, ecx
	cmp dl, 8
	jae .LBB354_1
	mov r8b, -1
	mov ecx, edx
	shl r8b, cl
	mov edx, r8d
	not dl
	add dl, al
	cmp dl, al
	setae al
	and dl, r8b
	ret
.LBB354_1:
	test al, al
	sete al
	xor edx, edx
```
## `unbounded_ceil_to_multiple_u16_unb_pow2`
```asm
	mov eax, ecx
	cmp dl, 16
	jae .LBB351_1
	mov r8d, -1
	mov ecx, edx
	shl r8d, cl
	mov edx, r8d
	not edx
	add edx, eax
	cmp dx, ax
	setae al
	and edx, r8d
	movzx eax, al
	ret
.LBB351_1:
	test ax, ax
	sete al
	xor edx, edx
	movzx eax, al
```
## `unbounded_ceil_to_multiple_u32_unb_pow2`
```asm
	mov eax, ecx
	cmp dl, 32
	jae .LBB352_1
	mov r8d, -1
	mov ecx, edx
	shl r8d, cl
	mov edx, r8d
	not edx
	add edx, eax
	cmp edx, eax
	setae al
	and edx, r8d
	movzx eax, al
	ret
.LBB352_1:
	test eax, eax
	sete al
	xor edx, edx
	movzx eax, al
```
## `unbounded_ceil_to_multiple_u64_unb_pow2`
```asm
	mov rax, rcx
	cmp dl, 64
	jae .LBB353_1
	mov r8, -1
	mov ecx, edx
	shl r8, cl
	mov rdx, r8
	not rdx
	add rdx, rax
	cmp rdx, rax
	setae al
	and rdx, r8
	movzx eax, al
	ret
.LBB353_1:
	test rax, rax
	sete al
	xor edx, edx
	movzx eax, al
```
## `unbounded_ceil_to_multiple_u128_unb_pow2`
```asm
	push rsi
	mov rax, rcx
	test r9b, r9b
	js .LBB350_4
	mov r10, -1
	mov ecx, r9d
	shl r10, cl
	mov r11, -1
	xor ecx, ecx
	test r9b, 64
	mov r9, r10
	cmovne r9, rcx
	cmove r10, r11
	mov r11, r10
	not r11
	mov rsi, r9
	not rsi
	add rdx, rsi
	adc r8, r11
	jb .LBB350_5
	and rdx, r9
	and r8, r10
	mov rcx, rdx
.LBB350_3:
	mov qword ptr [rax + 16], rcx
	mov qword ptr [rax + 24], r8
	mov ecx, 1
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	pop rsi
	ret
.LBB350_4:
	xor ecx, ecx
	or rdx, r8
	mov r8d, 0
	je .LBB350_3
.LBB350_5:
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	pop rsi
```
## `unbounded_div_ceil_i8_unb_pow2`
```asm
	mov r8d, ecx
	cmp dl, 8
	jae .LBB359_1
	mov r9b, -1
	mov ecx, edx
	shl r9b, cl
	not r9b
	mov eax, r8d
	sar al, cl
	and r9b, r8b
	cmp r9b, 1
	sbb al, -1
	ret
.LBB359_1:
	test r8b, r8b
	setg al
```
## `unbounded_div_ceil_i16_unb_pow2`
```asm
	mov r8d, edx
	mov edx, ecx
	cmp r8b, 16
	jae .LBB356_1
	mov r9d, -1
	mov ecx, r8d
	shl r9d, cl
	not r9d
	movsx eax, dx
	sar eax, cl
	and edx, r9d
	cmp dx, 1
	sbb ax, -1
	ret
.LBB356_1:
	xor eax, eax
	test dx, dx
	setg al
```
## `unbounded_div_ceil_i32_unb_pow2`
```asm
	mov r8d, edx
	mov edx, ecx
	cmp r8b, 32
	jae .LBB357_1
	mov r9d, -1
	mov ecx, r8d
	shl r9d, cl
	not r9d
	mov eax, edx
	sar eax, cl
	and r9d, edx
	cmp r9d, 1
	sbb eax, -1
	ret
.LBB357_1:
	xor eax, eax
	test edx, edx
	setg al
```
## `unbounded_div_ceil_i64_unb_pow2`
```asm
	mov r8d, edx
	mov rdx, rcx
	cmp r8b, 64
	jae .LBB358_1
	mov r9, -1
	mov ecx, r8d
	shl r9, cl
	not r9
	mov rax, rdx
	sar rax, cl
	and r9, rdx
	cmp r9, 1
	sbb rax, -1
	ret
.LBB358_1:
	xor eax, eax
	test rdx, rdx
	setg al
```
## `unbounded_div_ceil_i128_unb_pow2`
```asm
	push rsi
	push rdi
	mov r9d, r8d
	mov rax, rdx
	mov r8, rcx
	test r9b, r9b
	js .LBB355_1
	mov r11, -1
	mov ecx, r9d
	shl r11, cl
	mov r10, r8
	shrd r10, rax, cl
	mov rsi, rax
	sar rsi, cl
	mov rcx, -1
	mov rdx, rax
	sar rdx, 63
	xor edi, edi
	test r9b, 64
	cmovne rcx, r11
	cmove rdi, r11
	not rdi
	not rcx
	cmovne r10, rsi
	cmove rdx, rsi
	and rcx, rax
	and rdi, r8
	xor eax, eax
	or rdi, rcx
	setne al
	add rax, r10
	adc rdx, 0
	pop rdi
	pop rsi
	ret
.LBB355_1:
	xor edx, edx
	neg r8
	mov ecx, 0
	sbb rcx, rax
	setl al
	movzx eax, al
	pop rdi
	pop rsi
```
## `unbounded_div_ceil_u8_unb_pow2`
```asm
	mov r8d, ecx
	cmp dl, 8
	jae .LBB364_1
	mov r9b, -1
	mov ecx, edx
	shl r9b, cl
	not r9b
	mov eax, r8d
	shr al, cl
	and r9b, r8b
	cmp r9b, 1
	sbb al, -1
	ret
.LBB364_1:
	test r8b, r8b
	setne al
```
## `unbounded_div_ceil_u16_unb_pow2`
```asm
	mov r8d, edx
	mov edx, ecx
	cmp r8b, 16
	jae .LBB361_1
	mov r9d, -1
	mov ecx, r8d
	shl r9d, cl
	not r9d
	movzx eax, dx
	shr eax, cl
	and edx, r9d
	cmp dx, 1
	sbb ax, -1
	ret
.LBB361_1:
	xor eax, eax
	test dx, dx
	setne al
```
## `unbounded_div_ceil_u32_unb_pow2`
```asm
	mov r8d, edx
	mov edx, ecx
	cmp r8b, 32
	jae .LBB362_1
	mov r9d, -1
	mov ecx, r8d
	shl r9d, cl
	not r9d
	mov eax, edx
	shr eax, cl
	and r9d, edx
	cmp r9d, 1
	sbb eax, -1
	ret
.LBB362_1:
	xor eax, eax
	test edx, edx
	setne al
```
## `unbounded_div_ceil_u64_unb_pow2`
```asm
	mov r8d, edx
	mov rdx, rcx
	cmp r8b, 64
	jae .LBB363_1
	mov r9, -1
	mov ecx, r8d
	shl r9, cl
	not r9
	mov rax, rdx
	shr rax, cl
	and r9, rdx
	cmp r9, 1
	sbb rax, -1
	ret
.LBB363_1:
	xor eax, eax
	test rdx, rdx
	setne al
```
## `unbounded_div_ceil_u128_unb_pow2`
```asm
	push rsi
	mov eax, r8d
	mov r8, rcx
	test al, al
	js .LBB360_1
	mov r11, -1
	mov ecx, eax
	shl r11, cl
	mov r10, r8
	shrd r10, rdx, cl
	mov rsi, -1
	mov r9, rdx
	shr rdx, cl
	xor ecx, ecx
	test al, 64
	cmovne rsi, r11
	cmovne r11, rcx
	not r11
	not rsi
	cmovne r10, rdx
	cmovne rdx, rcx
	and rsi, r9
	and r11, r8
	xor eax, eax
	or r11, rsi
	setne al
	add rax, r10
	adc rdx, 0
	pop rsi
	ret
.LBB360_1:
	xor eax, eax
	or r8, rdx
	setne al
	xor edx, edx
	pop rsi
```
## `unbounded_div_floor_i8_unb_pow2`
```asm
	mov eax, ecx
	movzx edx, dl
	cmp dl, 7
	mov ecx, 7
	cmovb ecx, edx
	sar al, cl
```
## `unbounded_div_floor_i16_unb_pow2`
```asm
	movsx eax, cx
	movzx edx, dl
	cmp dl, 15
	mov ecx, 15
	cmovb ecx, edx
	sar eax, cl
```
## `unbounded_div_floor_i32_unb_pow2`
```asm
	mov eax, ecx
	movzx edx, dl
	cmp dl, 31
	mov ecx, 31
	cmovb ecx, edx
	sar eax, cl
```
## `unbounded_div_floor_i64_unb_pow2`
```asm
	mov rax, rcx
	movzx edx, dl
	cmp dl, 63
	mov ecx, 63
	cmovb ecx, edx
	sar rax, cl
```
## `unbounded_div_floor_i128_unb_pow2`
```asm
	mov rax, rcx
	test r8b, r8b
	movzx r8d, r8b
	mov ecx, 127
	cmovns ecx, r8d
	shrd rax, rdx, cl
	mov r8, rdx
	sar r8, cl
	sar rdx, 63
	test cl, 64
	cmovne rax, r8
	cmove rdx, r8
```
## `unbounded_div_floor_u8_unb_pow2`, `unbounded_div_u8_unb_pow2`
```asm
	mov eax, ecx
	mov ecx, edx
	shr al, cl
	xor ecx, ecx
	cmp dl, 8
	movzx eax, al
	cmovae eax, ecx
```
## `unbounded_div_floor_u16_unb_pow2`, `unbounded_div_u16_unb_pow2`
```asm
	movzx r8d, cx
	mov ecx, edx
	shr r8d, cl
	xor eax, eax
	cmp dl, 16
	cmovb eax, r8d
```
## `unbounded_div_floor_u32_unb_pow2`, `unbounded_div_u32_unb_pow2`
```asm
	mov r8d, ecx
	mov ecx, edx
	shr r8d, cl
	xor eax, eax
	cmp dl, 32
	cmovb eax, r8d
```
## `unbounded_div_floor_u64_unb_pow2`, `unbounded_div_u64_unb_pow2`
```asm
	mov r8, rcx
	mov ecx, edx
	shr r8, cl
	xor eax, eax
	cmp dl, 64
	cmovb rax, r8
```
## `unbounded_div_floor_u128_unb_pow2`, `unbounded_div_u128_unb_pow2`
```asm
	mov rax, rcx
	mov ecx, r8d
	shrd rax, rdx, cl
	shr rdx, cl
	xor ecx, ecx
	test r8b, 64
	cmovne rax, rdx
	cmovne rdx, rcx
	test r8b, r8b
	cmovs rax, rcx
	cmovs rdx, rcx
```
## `unbounded_div_i8_unb_pow2`
```asm
	cmp dl, 7
	ja .LBB379_1
	mov r8d, ecx
	mov r9b, -1
	mov ecx, edx
	shl r9b, cl
	not r9b
	mov eax, r8d
	sar al, 7
	and al, r9b
	add al, r8b
	sar al, cl
	ret
.LBB379_1:
	xor eax, eax
```
## `unbounded_div_i16_unb_pow2`
```asm
	cmp dl, 15
	ja .LBB376_1
	mov eax, ecx
	mov r8d, -1
	mov ecx, edx
	shl r8d, cl
	not r8d
	movsx ecx, ax
	sar ecx, 15
	and ecx, r8d
	add ecx, eax
	movsx eax, cx
	mov ecx, edx
	sar eax, cl
	ret
.LBB376_1:
	xor eax, eax
```
## `unbounded_div_i32_unb_pow2`
```asm
	xor eax, eax
	cmp dl, 31
	ja .LBB377_2
	mov r8d, ecx
	mov r9d, -1
	mov ecx, edx
	shl r9d, cl
	not r9d
	mov eax, r8d
	sar eax, 31
	and eax, r9d
	add eax, r8d
	sar eax, cl
.LBB377_2:
```
## `unbounded_div_i64_unb_pow2`
```asm
	cmp dl, 63
	ja .LBB378_1
	mov r8, rcx
	mov r9, -1
	mov ecx, edx
	shl r9, cl
	not r9
	mov rax, r8
	sar rax, 63
	and rax, r9
	add rax, r8
	sar rax, cl
	ret
.LBB378_1:
	xor eax, eax
```
## `unbounded_div_i128_unb_pow2`
```asm
	test r8b, r8b
	js .LBB375_1
	mov r9, rdx
	mov r10, rcx
	mov rdx, -1
	mov rax, -1
	mov ecx, r8d
	shl rax, cl
	xor ecx, ecx
	test r8b, 64
	cmovne rdx, rax
	cmove rcx, rax
	not rcx
	not rdx
	mov rax, r9
	sar rax, 63
	and rdx, rax
	and rax, rcx
	add rax, r10
	adc rdx, r9
	mov ecx, r8d
	shrd rax, rdx, cl
	mov r9, rdx
	sar r9, cl
	test r8b, 64
	cmovne rax, r9
	sar rdx, 63
	test r8b, 64
	cmove rdx, r9
	ret
.LBB375_1:
	xor eax, eax
	xor edx, edx
```
## `unbounded_div_round_i8_unb_pow2`
```asm
	mov r8d, ecx
	cmp dl, 7
	ja .LBB384_2
	mov r9b, -1
	mov ecx, edx
	shl r9b, cl
	not r9b
	mov eax, r8d
	sar al, cl
	and r9b, r8b
	shr r8b, 7
	xor ecx, ecx
	sub r9b, r8b
	movzx r8d, r9b
	cmovb r8d, ecx
	add r8b, r8b
	movzx ecx, r8b
	movzx edx, dl
	bt ecx, edx
	adc al, 0
	ret
.LBB384_2:
	neg r8b
	jno .LBB384_3
	cmp dl, 8
	sete al
	neg al
	ret
.LBB384_3:
	xor eax, eax
```
## `unbounded_div_round_i16_unb_pow2`
```asm
	mov r8d, ecx
	cmp dl, 15
	ja .LBB381_2
	mov r9d, -1
	mov ecx, edx
	shl r9d, cl
	not r9d
	movsx eax, r8w
	sar eax, cl
	and r9d, r8d
	movzx ecx, r8w
	shr ecx, 15
	xor r8d, r8d
	sub r9w, cx
	cmovb r9d, r8d
	add r9d, r9d
	movzx ecx, dl
	bt r9d, ecx
	adc ax, 0
	ret
.LBB381_2:
	xor eax, eax
	neg r8w
	jno .LBB381_4
	cmp dl, 16
	sete al
	neg eax
.LBB381_4:
```
## `unbounded_div_round_i32_unb_pow2`
```asm
	mov r8d, ecx
	cmp dl, 31
	ja .LBB382_2
	mov r9d, -1
	mov ecx, edx
	shl r9d, cl
	mov eax, r8d
	sar eax, cl
	not r9d
	and r9d, r8d
	shr r8d, 31
	xor ecx, ecx
	sub r9d, r8d
	cmovb r9d, ecx
	add r9d, r9d
	movzx ecx, dl
	bt r9d, ecx
	adc eax, 0
	ret
.LBB382_2:
	xor eax, eax
	neg r8d
	jno .LBB382_4
	xor eax, eax
	cmp dl, 32
	sete al
	neg eax
.LBB382_4:
```
## `unbounded_div_round_i64_unb_pow2`
```asm
	mov r8, rcx
	cmp dl, 63
	ja .LBB383_2
	mov r9, -1
	mov ecx, edx
	shl r9, cl
	mov rax, r8
	sar rax, cl
	not r9
	and r9, r8
	shr r8, 63
	xor ecx, ecx
	sub r9, r8
	cmovae rcx, r9
	add rcx, rcx
	movzx edx, dl
	bt rcx, rdx
	adc rax, 0
	ret
.LBB383_2:
	xor eax, eax
	neg r8
	jno .LBB383_4
	cmp dl, 64
	sete al
	neg rax
.LBB383_4:
```
## `unbounded_div_round_i128_unb_pow2`
```asm
	mov r9, rcx
	test r8b, r8b
	js .LBB380_2
	mov rax, -1
	mov r10, -1
	mov ecx, r8d
	shl r10, cl
	xor ecx, ecx
	test r8b, 64
	cmovne rax, r10
	cmovne r10, rcx
	not r10
	not rax
	and rax, rdx
	and r10, r9
	mov r11, rdx
	shr r11, 63
	sub r10, r11
	sbb rax, 0
	cmovb rax, rcx
	cmovb r10, rcx
	shld rax, r10, 1
	add r10, r10
	mov ecx, r8d
	shrd r10, rax, cl
	shr rax, cl
	test r8b, 64
	cmove rax, r10
	mov r11, rdx
	sar r11, cl
	mov r10, rdx
	sar r10, 63
	test r8b, 64
	cmove r10, r11
	shrd r9, rdx, cl
	mov rdx, r10
	test r8b, 64
	cmovne r9, r11
	and eax, 1
	add rax, r9
	adc rdx, 0
	ret
.LBB380_2:
	movabs rax, -9223372036854775808
	xor rdx, rax
	xor eax, eax
	or r9, rdx
	mov edx, 0
	jne .LBB380_4
	xor eax, eax
	neg r8b
	seto al
	neg rax
	mov rdx, rax
.LBB380_4:
```
## `unbounded_div_round_u8_unb_pow2`
```asm
	cmp dl, 7
	ja .LBB389_1
	mov eax, ecx
	mov r8b, 1
	mov ecx, edx
	shl r8b, cl
	shr r8b
	and r8b, al
	shr al, cl
	cmp r8b, 1
	sbb al, -1
	ret
.LBB389_1:
	xor eax, eax
```
## `unbounded_div_round_u16_unb_pow2`
```asm
	cmp dl, 15
	ja .LBB386_1
	mov eax, ecx
	mov r8d, 1
	mov ecx, edx
	shl r8d, cl
	movzx r8d, r8w
	shr r8d
	movzx r9d, ax
	mov eax, r9d
	shr eax, cl
	and r9d, r8d
	cmp r9w, 1
	sbb ax, -1
	ret
.LBB386_1:
	xor eax, eax
```
## `unbounded_div_round_u32_unb_pow2`
```asm
	xor eax, eax
	cmp dl, 31
	ja .LBB387_2
	mov r8d, ecx
	mov eax, 1
	mov ecx, edx
	shl eax, cl
	shr eax
	and eax, r8d
	shr r8d, cl
	cmp eax, 1
	sbb r8d, -1
	mov eax, r8d
.LBB387_2:
```
## `unbounded_div_round_u64_unb_pow2`
```asm
	cmp dl, 63
	ja .LBB388_1
	mov rax, rcx
	mov r8d, 1
	mov ecx, edx
	shl r8, cl
	shr r8
	and r8, rax
	shr rax, cl
	cmp r8, 1
	sbb rax, -1
	ret
.LBB388_1:
	xor eax, eax
```
## `unbounded_div_round_u128_unb_pow2`
```asm
	push rsi
	push rdi
	push rbx
	test r8b, r8b
	js .LBB385_1
	mov rax, rcx
	mov r11, rdx
	mov ecx, r8d
	shr r11, cl
	xor r10d, r10d
	test r8b, 64
	mov rsi, rdx
	mov rdx, r11
	cmovne rdx, r10
	mov r9, rax
	shrd r9, rsi, cl
	test r8b, 64
	cmovne r9, r11
	mov r11d, 1
	mov edi, 1
	shl rdi, cl
	test r8b, 64
	mov rbx, rdi
	cmovne rbx, r10
	shld r10, r11, cl
	test r8b, 64
	cmovne r10, rdi
	shrd rbx, r10, 1
	shr r10
	and r10, rsi
	and rbx, rax
	xor eax, eax
	or rbx, r10
	setne al
	add rax, r9
	adc rdx, 0
	pop rbx
	pop rdi
	pop rsi
	ret
.LBB385_1:
	xor eax, eax
	xor edx, edx
	pop rbx
	pop rdi
	pop rsi
```
## `unbounded_floor_to_multiple_i8_unb_pow2`
```asm
	mov eax, ecx
	mov r8d, ecx
	mov ecx, edx
	shr r8b, cl
	shl r8b, cl
	test al, al
	setns cl
	xor eax, eax
	cmp dl, 8
	movzx edx, r8b
	cmovae edx, eax
	setb al
	or al, cl
```
## `unbounded_floor_to_multiple_i16_unb_pow2`
```asm
	cmp dl, 16
	jae .LBB396_1
	mov eax, edx
	movzx edx, cx
	mov ecx, eax
	shr edx, cl
	shl edx, cl
	mov ax, 1
	ret
.LBB396_1:
	not ecx
	movzx eax, cx
	shr eax, 15
	xor edx, edx
```
## `unbounded_floor_to_multiple_i32_unb_pow2`
```asm
	mov eax, ecx
	cmp dl, 32
	jae .LBB397_1
	mov ecx, edx
	shr eax, cl
	shl eax, cl
	mov edx, eax
	mov eax, 1
	ret
.LBB397_1:
	not eax
	shr eax, 31
	xor edx, edx
```
## `unbounded_floor_to_multiple_i64_unb_pow2`
```asm
	mov rax, rcx
	cmp dl, 64
	jae .LBB398_1
	mov ecx, edx
	shr rax, cl
	shl rax, cl
	mov rdx, rax
	mov eax, 1
	ret
.LBB398_1:
	not rax
	shr rax, 63
	xor edx, edx
```
## `unbounded_floor_to_multiple_i128_unb_pow2`
```asm
	mov rax, rcx
	test r9b, r9b
	js .LBB395_1
	mov r10, -1
	mov r11, -1
	mov ecx, r9d
	shl r11, cl
	xor ecx, ecx
	test r9b, 64
	cmovne r10, r11
	cmove rcx, r11
	and rcx, rdx
	and r10, r8
	mov qword ptr [rax + 24], r10
	mov qword ptr [rax + 16], rcx
	mov qword ptr [rax + 8], 0
	mov qword ptr [rax], 1
	ret
.LBB395_1:
	xorps xmm0, xmm0
	test r8, r8
	js .LBB395_2
	movups xmmword ptr [rax + 8], xmm0
	mov qword ptr [rax], 1
	mov qword ptr [rax + 24], 0
	ret
.LBB395_2:
	movaps xmmword ptr [rax], xmm0
```
## `unbounded_floor_to_multiple_u8_unb_pow2`
```asm
	mov eax, ecx
	mov ecx, edx
	shr al, cl
	shl al, cl
	xor ecx, ecx
	cmp dl, 8
	movzx eax, al
	cmovae eax, ecx
```
## `unbounded_floor_to_multiple_u16_unb_pow2`
```asm
	movzx r8d, cx
	mov ecx, edx
	shr r8d, cl
	shl r8d, cl
	xor eax, eax
	cmp dl, 16
	cmovb eax, r8d
```
## `unbounded_floor_to_multiple_u32_unb_pow2`
```asm
	mov r8d, ecx
	mov ecx, edx
	shr r8d, cl
	shl r8d, cl
	xor eax, eax
	cmp dl, 32
	cmovb eax, r8d
```
## `unbounded_floor_to_multiple_u64_unb_pow2`
```asm
	mov r8, rcx
	mov ecx, edx
	shr r8, cl
	shl r8, cl
	xor eax, eax
	cmp dl, 64
	cmovb rax, r8
```
## `unbounded_floor_to_multiple_u128_unb_pow2`
```asm
	mov r9, rcx
	mov r10, -1
	mov ecx, r8d
	shl r10, cl
	xor ecx, ecx
	test r8b, 64
	mov rax, r10
	cmovne rax, rcx
	mov r11, -1
	cmove r10, r11
	and rdx, r10
	and rax, r9
	test r8b, r8b
	cmovs rax, rcx
	cmovs rdx, rcx
```
## `unbounded_is_multiple_of_i8_unb_pow2`, `unbounded_is_multiple_of_u8_unb_pow2`
```asm
	test cl, cl
	sete r8b
	movzx eax, cl
	rep bsf	eax, eax
	cmp al, dl
	setae al
	or al, r8b
```
## `unbounded_is_multiple_of_i16_unb_pow2`, `unbounded_is_multiple_of_u16_unb_pow2`
```asm
	test cx, cx
	sete r8b
	rep bsf	eax, ecx
	movzx ecx, dl
	cmp ax, cx
	setae al
	or al, r8b
```
## `unbounded_is_multiple_of_i32_unb_pow2`, `unbounded_is_multiple_of_u32_unb_pow2`
```asm
	test ecx, ecx
	sete r8b
	rep bsf	eax, ecx
	movzx ecx, dl
	cmp eax, ecx
	setae al
	or al, r8b
```
## `unbounded_is_multiple_of_i64_unb_pow2`, `unbounded_is_multiple_of_u64_unb_pow2`
```asm
	test rcx, rcx
	sete r8b
	rep bsf	rax, rcx
	movzx ecx, dl
	cmp rax, rcx
	setae al
	or al, r8b
```
## `unbounded_is_multiple_of_i128_unb_pow2`
```asm
	mov rax, rcx
	or rax, rdx
	je .LBB405_1
	rep bsf	rax, rcx
	rep bsf	rdx, rdx
	add edx, 64
	test rcx, rcx
	cmovne edx, eax
	movzx eax, r8b
	cmp edx, eax
	setae al
	ret
.LBB405_1:
	mov al, 1
```
## `unbounded_is_multiple_of_u128_unb_pow2`
```asm
	mov rax, rcx
	or rax, rdx
	je .LBB410_1
	rep bsf	rax, rcx
	rep bsf	rdx, rdx
	add edx, 64
	test rcx, rcx
	cmovne edx, eax
	movzx eax, r8b
	cmp edx, eax
	setae al
	ret
.LBB410_1:
	mov al, 1
```
## `unbounded_rem_i8_unb_pow2`
```asm
	mov eax, ecx
	cmp dl, 7
	ja .LBB419_2
	mov r8b, -1
	mov ecx, edx
	shl r8b, cl
	mov ecx, r8d
	not cl
	mov edx, eax
	sar dl, 7
	and dl, cl
	add dl, al
	and dl, r8b
	sub al, dl
.LBB419_2:
```
## `unbounded_rem_i16_unb_pow2`
```asm
	mov eax, ecx
	cmp dl, 15
	ja .LBB416_2
	mov r8d, -1
	mov ecx, edx
	shl r8d, cl
	mov ecx, r8d
	not ecx
	movsx edx, ax
	sar edx, 15
	and edx, ecx
	add edx, eax
	and edx, r8d
	sub eax, edx
.LBB416_2:
```
## `unbounded_rem_i32_unb_pow2`
```asm
	mov eax, ecx
	cmp dl, 31
	ja .LBB417_2
	mov r8d, -1
	mov ecx, edx
	shl r8d, cl
	mov ecx, r8d
	not ecx
	mov edx, eax
	sar edx, 31
	and edx, ecx
	add edx, eax
	and edx, r8d
	sub eax, edx
.LBB417_2:
```
## `unbounded_rem_i64_unb_pow2`
```asm
	mov rax, rcx
	cmp dl, 63
	ja .LBB418_2
	mov r8, -1
	mov ecx, edx
	shl r8, cl
	mov rcx, r8
	not rcx
	mov rdx, rax
	sar rdx, 63
	and rdx, rcx
	add rdx, rax
	and rdx, r8
	sub rax, rdx
.LBB418_2:
```
## `unbounded_rem_i128_unb_pow2`
```asm
	mov rax, rcx
	test r8b, r8b
	js .LBB415_2
	mov r9, -1
	mov ecx, r8d
	shl r9, cl
	mov rcx, -1
	xor r10d, r10d
	test r8b, 64
	cmovne rcx, r9
	cmove r10, r9
	mov r8, r10
	not r8
	mov r9, rcx
	not r9
	mov r11, rdx
	sar r11, 63
	and r9, r11
	and r11, r8
	add r11, rax
	adc r9, rdx
	and r9, rcx
	and r11, r10
	sub rax, r11
	sbb rdx, r9
.LBB415_2:
```
## `unbounded_rem_u8_unb_pow2`
```asm
	mov r8d, ecx
	mov al, -1
	mov ecx, edx
	shl al, cl
	cmp dl, 8
	not al
	movzx ecx, al
	mov eax, 255
	cmovb eax, ecx
	and al, r8b
```
## `unbounded_rem_u16_unb_pow2`
```asm
	mov r8d, ecx
	mov r9d, -1
	mov ecx, edx
	shl r9d, cl
	cmp dl, 16
	not r9d
	mov eax, 65535
	cmovb eax, r9d
	and eax, r8d
```
## `unbounded_rem_u32_unb_pow2`
```asm
	mov r8d, ecx
	mov eax, -1
	mov ecx, edx
	shl eax, cl
	mov ecx, -1
	cmp dl, 32
	not eax
	cmovae eax, ecx
	and eax, r8d
```
## `unbounded_rem_u64_unb_pow2`
```asm
	mov r8, rcx
	mov rax, -1
	mov ecx, edx
	shl rax, cl
	mov rcx, -1
	cmp dl, 64
	not rax
	cmovae rax, rcx
	and rax, r8
```
## `unbounded_rem_u128_unb_pow2`
```asm
	mov r9, rcx
	mov r10, -1
	mov ecx, r8d
	shl r10, cl
	xor eax, eax
	test r8b, 64
	mov rcx, -1
	cmovne rcx, r10
	cmove rax, r10
	mov r10, -1
	not rax
	not rcx
	test r8b, r8b
	cmovs rcx, r10
	cmovs rax, r10
	and rax, r9
	and rdx, rcx
```
## `unbounded_round_to_multiple_i8_unb_pow2`
```asm
	mov eax, ecx
	cmp dl, 7
	ja .LBB429_5
	mov r8b, 1
	mov ecx, edx
	shl r8b, cl
	mov ecx, r8d
	shr cl
	mov edx, eax
	shr dl, 7
	xor r9d, r9d
	sub cl, dl
	movzx ecx, cl
	cmovb ecx, r9d
	lea edx, [rax + rcx]
	add al, cl
	seto al
	jo .LBB429_2
	neg r8b
	and dl, r8b
	xor al, 1
	ret
.LBB429_5:
	add al, -128
	xor dl, 8
	or dl, al
	setne al
	xor edx, edx
	ret
.LBB429_2:
	xor al, 1
```
## `unbounded_round_to_multiple_i16_unb_pow2`
```asm
	mov r8d, edx
	mov eax, ecx
	cmp dl, 15
	ja .LBB426_3
	mov edx, 1
	mov ecx, r8d
	shl edx, cl
	movzx r8d, dx
	shr r8d
	movzx ecx, ax
	shr ecx, 15
	xor r9d, r9d
	sub r8w, cx
	cmovb r8d, r9d
	lea ecx, [rax + r8]
	add ax, r8w
	seto al
	jo .LBB426_4
	neg edx
	and edx, ecx
	xor al, 1
	movzx eax, al
	ret
.LBB426_3:
	neg ax
	setno cl
	cmp r8b, 16
	setne al
	or al, cl
	xor edx, edx
	movzx eax, al
	ret
.LBB426_4:
	xor al, 1
	movzx eax, al
```
## `unbounded_round_to_multiple_i32_unb_pow2`
```asm
	mov eax, ecx
	cmp dl, 31
	ja .LBB427_5
	mov r8d, 1
	mov ecx, edx
	shl r8d, cl
	mov ecx, r8d
	shr ecx
	mov edx, eax
	shr edx, 31
	xor r9d, r9d
	sub ecx, edx
	cmovb ecx, r9d
	lea edx, [rax + rcx]
	add ecx, eax
	seto al
	jo .LBB427_2
	neg r8d
	and edx, r8d
	xor al, 1
	movzx eax, al
	ret
.LBB427_5:
	neg eax
	setno cl
	cmp dl, 32
	setne al
	or al, cl
	xor edx, edx
	movzx eax, al
	ret
.LBB427_2:
	xor al, 1
	movzx eax, al
```
## `unbounded_round_to_multiple_i64_unb_pow2`
```asm
	mov rax, rcx
	cmp dl, 63
	ja .LBB428_5
	mov r8d, 1
	mov ecx, edx
	shl r8, cl
	mov rcx, r8
	shr rcx
	mov rdx, rax
	shr rdx, 63
	xor r9d, r9d
	sub rcx, rdx
	cmovae r9, rcx
	lea rdx, [rax + r9]
	add r9, rax
	seto al
	jo .LBB428_2
	neg r8
	and rdx, r8
	xor al, 1
	movzx eax, al
	ret
.LBB428_5:
	neg rax
	setno cl
	cmp dl, 64
	setne al
	or al, cl
	xor edx, edx
	movzx eax, al
	ret
.LBB428_2:
	xor al, 1
	movzx eax, al
```
## `unbounded_round_to_multiple_i128_unb_pow2`
```asm
	push rsi
	push rdi
	mov rax, rcx
	test r9b, r9b
	js .LBB425_3
	mov r10d, 1
	xor r11d, r11d
	mov ecx, r9d
	shld r11, r10, cl
	shl r10, cl
	xor ecx, ecx
	test r9b, 64
	cmovne r11, r10
	cmovne r10, rcx
	mov r9, r11
	shr r9
	mov rsi, r11
	shld rsi, r10, 63
	mov rdi, r8
	shr rdi, 63
	sub rsi, rdi
	sbb r9, 0
	cmovb r9, rcx
	cmovb rsi, rcx
	add rdx, rsi
	adc r9, r8
	jo .LBB425_5
	xor ecx, ecx
	neg r10
	sbb rcx, r11
	and r9, rcx
	and rdx, r10
	mov rcx, rdx
	jmp .LBB425_4
.LBB425_3:
	movabs rcx, -9223372036854775808
	xor r8, rcx
	or rdx, r8
	sete dl
	neg r9b
	seto r8b
	xor ecx, ecx
	mov r9d, 0
	test dl, r8b
	jne .LBB425_5
.LBB425_4:
	mov qword ptr [rax + 16], rcx
	mov qword ptr [rax + 24], r9
	mov ecx, 1
.LBB425_5:
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	pop rdi
	pop rsi
```
## `unbounded_round_to_multiple_u8_unb_pow2`
```asm
	mov eax, ecx
	cmp dl, 7
	ja .LBB434_4
	mov r8b, 1
	mov ecx, edx
	shl r8b, cl
	mov edx, r8d
	shr dl
	add dl, al
	cmp dl, al
	setae al
	jb .LBB434_3
	neg r8b
	and dl, r8b
	ret
.LBB434_4:
	test al, al
	setns cl
	cmp dl, 8
	setne al
	or al, cl
	xor edx, edx
	ret
.LBB434_3:
```
## `unbounded_round_to_multiple_u16_unb_pow2`
```asm
	mov eax, ecx
	cmp dl, 15
	ja .LBB431_4
	mov r8d, 1
	mov ecx, edx
	shl r8d, cl
	movzx ecx, r8w
	shr ecx
	lea edx, [rax + rcx]
	cmp dx, ax
	setae al
	jb .LBB431_3
	neg r8d
	and edx, r8d
	movzx eax, al
	ret
.LBB431_4:
	test ax, ax
	setns cl
	cmp dl, 16
	setne al
	or al, cl
	xor edx, edx
	movzx eax, al
	ret
.LBB431_3:
	movzx eax, al
```
## `unbounded_round_to_multiple_u32_unb_pow2`
```asm
	mov eax, ecx
	cmp dl, 31
	ja .LBB432_4
	mov r8d, 1
	mov ecx, edx
	shl r8d, cl
	mov edx, r8d
	shr edx
	add edx, eax
	cmp edx, eax
	setae al
	jb .LBB432_3
	neg r8d
	and edx, r8d
	movzx eax, al
	ret
.LBB432_4:
	test eax, eax
	setns cl
	cmp dl, 32
	setne al
	or al, cl
	xor edx, edx
	movzx eax, al
	ret
.LBB432_3:
	movzx eax, al
```
## `unbounded_round_to_multiple_u64_unb_pow2`
```asm
	mov rax, rcx
	cmp dl, 63
	ja .LBB433_4
	mov r8d, 1
	mov ecx, edx
	shl r8, cl
	mov rdx, r8
	shr rdx
	add rdx, rax
	cmp rdx, rax
	setae al
	jb .LBB433_3
	neg r8
	and rdx, r8
	movzx eax, al
	ret
.LBB433_4:
	test rax, rax
	setns cl
	cmp dl, 64
	setne al
	or al, cl
	xor edx, edx
	movzx eax, al
	ret
.LBB433_3:
	movzx eax, al
```
## `unbounded_round_to_multiple_u128_unb_pow2`
```asm
	push rsi
	mov rax, rcx
	test r9b, r9b
	js .LBB430_3
	mov r11d, 1
	xor esi, esi
	mov ecx, r9d
	shld rsi, r11, cl
	xor r10d, r10d
	shl r11, cl
	test r9b, 64
	cmovne rsi, r11
	cmovne r11, r10
	mov rcx, rsi
	mov r9, rsi
	shld r9, r11, 63
	shr rcx
	add r9, rdx
	adc rcx, r8
	jb .LBB430_5
	xor edx, edx
	neg r11
	sbb rdx, rsi
	and rcx, rdx
	and r9, r11
	mov r10, r9
	jmp .LBB430_4
.LBB430_3:
	test r8, r8
	sets dl
	neg r9b
	seto r8b
	xor r10d, r10d
	mov ecx, 0
	test dl, r8b
	jne .LBB430_5
.LBB430_4:
	mov qword ptr [rax + 16], r10
	mov qword ptr [rax + 24], rcx
	mov r10d, 1
.LBB430_5:
	mov qword ptr [rax], r10
	mov qword ptr [rax + 8], 0
	pop rsi
```
