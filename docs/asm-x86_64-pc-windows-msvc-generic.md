## `ceil_to_multiple_i8_pow2`
```asm
ceil_to_multiple_i8_pow2:
	mov r8d, ecx
	mov r9b, -1
	mov ecx, edx
	shl r9b, cl
	mov eax, r9d
	not al
	add al, r8b
	and al, r9b
```
## `ceil_to_multiple_i8_unb_pow2`
```asm
ceil_to_multiple_i8_unb_pow2:
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
## `ceil_to_multiple_i16_pow2`
```asm
ceil_to_multiple_i16_pow2:
	mov r8d, ecx
	mov r9d, -1
	mov ecx, edx
	shl r9d, cl
	mov eax, r9d
	not eax
	add eax, r8d
	and eax, r9d
```
## `ceil_to_multiple_i16_unb_pow2`
```asm
ceil_to_multiple_i16_unb_pow2:
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
## `ceil_to_multiple_i32_pow2`
```asm
ceil_to_multiple_i32_pow2:
	mov r8d, ecx
	mov r9d, -1
	mov ecx, edx
	shl r9d, cl
	mov eax, r9d
	not eax
	add eax, r8d
	and eax, r9d
```
## `ceil_to_multiple_i32_unb_pow2`
```asm
ceil_to_multiple_i32_unb_pow2:
	mov r8d, ecx
	mov r9d, -1
	mov ecx, edx
	shl r9d, cl
	mov eax, r9d
	not eax
	add eax, r8d
	and eax, r9d
```
## `ceil_to_multiple_i64_pow2`
```asm
ceil_to_multiple_i64_pow2:
	mov r8, rcx
	mov r9, -1
	mov ecx, edx
	shl r9, cl
	mov rax, r9
	not rax
	add rax, r8
	and rax, r9
```
## `ceil_to_multiple_i64_unb_pow2`
```asm
ceil_to_multiple_i64_unb_pow2:
	mov r8, rcx
	mov r9, -1
	mov ecx, edx
	shl r9, cl
	mov rax, r9
	not rax
	add rax, r8
	and rax, r9
```
## `ceil_to_multiple_i128_pow2`
```asm
ceil_to_multiple_i128_pow2:
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
## `ceil_to_multiple_i128_unb_pow2`
```asm
ceil_to_multiple_i128_unb_pow2:
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
checked_ceil_to_multiple_i8_pow2:
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
checked_ceil_to_multiple_i8_unb_pow2:
	cmp dl, 7
	ja .LBB24_1
	mov r8d, ecx
	mov al, -1
	mov ecx, edx
	shl al, cl
	mov ecx, eax
	not cl
	add r8b, cl
	jno .LBB24_3
.LBB24_1:
	xor eax, eax
	mov edx, r8d
	ret
.LBB24_3:
	and r8b, al
	mov al, 1
	mov edx, r8d
```
## `checked_ceil_to_multiple_i16_pow2`
```asm
checked_ceil_to_multiple_i16_pow2:
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
checked_ceil_to_multiple_i16_unb_pow2:
	cmp dl, 15
	ja .LBB18_1
	mov r8d, edx
	mov eax, ecx
	mov edx, -1
	mov ecx, r8d
	shl edx, cl
	mov ecx, edx
	not ecx
	add ax, cx
	jno .LBB18_3
.LBB18_1:
	xor eax, eax
	ret
.LBB18_3:
	and edx, eax
	mov ax, 1
```
## `checked_ceil_to_multiple_i32_pow2`
```asm
checked_ceil_to_multiple_i32_pow2:
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
checked_ceil_to_multiple_i32_unb_pow2:
	xor eax, eax
	cmp dl, 31
	ja .LBB20_1
	mov r8d, ecx
	mov r9d, -1
	mov ecx, edx
	shl r9d, cl
	mov ecx, r9d
	not ecx
	add r8d, ecx
	jo .LBB20_4
	and r8d, r9d
	mov eax, 1
.LBB20_4:
	mov edx, r8d
	ret
.LBB20_1:
	mov edx, r8d
```
## `checked_ceil_to_multiple_i64_pow2`
```asm
checked_ceil_to_multiple_i64_pow2:
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
checked_ceil_to_multiple_i64_unb_pow2:
	cmp dl, 63
	ja .LBB22_1
	mov r8, rcx
	mov rax, -1
	mov ecx, edx
	shl rax, cl
	mov rcx, rax
	not rcx
	add r8, rcx
	jno .LBB22_3
.LBB22_1:
	xor eax, eax
	mov rdx, r8
	ret
.LBB22_3:
	and r8, rax
	mov eax, 1
	mov rdx, r8
```
## `checked_ceil_to_multiple_i128_pow2`
```asm
checked_ceil_to_multiple_i128_pow2:
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
	jo .LBB15_2
	and r11, r10
	and rsi, r9
	mov qword ptr [rax + 16], rsi
	mov qword ptr [rax + 24], r11
	mov ecx, 1
.LBB15_2:
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	pop rsi
```
## `checked_ceil_to_multiple_i128_unb_pow2`
```asm
checked_ceil_to_multiple_i128_unb_pow2:
	push rsi
	mov rax, rcx
	test r9b, r9b
	js .LBB16_4
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
	jo .LBB16_3
	and r8, r10
	and rdx, r9
	mov qword ptr [rax + 16], rdx
	mov qword ptr [rax + 24], r8
	mov ecx, 1
.LBB16_3:
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	pop rsi
	ret
.LBB16_4:
	xor ecx, ecx
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	pop rsi
```
## `checked_ceil_to_multiple_u8_pow2`
```asm
checked_ceil_to_multiple_u8_pow2:
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
checked_ceil_to_multiple_u8_unb_pow2:
	cmp dl, 7
	ja .LBB34_1
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
.LBB34_1:
	xor eax, eax
```
## `checked_ceil_to_multiple_u16_pow2`
```asm
checked_ceil_to_multiple_u16_pow2:
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
checked_ceil_to_multiple_u16_unb_pow2:
	cmp dl, 15
	ja .LBB28_1
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
.LBB28_1:
	xor eax, eax
```
## `checked_ceil_to_multiple_u32_pow2`
```asm
checked_ceil_to_multiple_u32_pow2:
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
checked_ceil_to_multiple_u32_unb_pow2:
	cmp dl, 31
	ja .LBB30_1
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
.LBB30_1:
	xor eax, eax
```
## `checked_ceil_to_multiple_u64_pow2`
```asm
checked_ceil_to_multiple_u64_pow2:
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
checked_ceil_to_multiple_u64_unb_pow2:
	cmp dl, 63
	ja .LBB32_1
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
.LBB32_1:
	xor eax, eax
```
## `checked_ceil_to_multiple_u128_pow2`
```asm
checked_ceil_to_multiple_u128_pow2:
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
	jb .LBB25_2
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
## `checked_ceil_to_multiple_u128_unb_pow2`
```asm
checked_ceil_to_multiple_u128_unb_pow2:
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
	jb .LBB26_3
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
## `checked_div_ceil_i8_unb_pow2`
```asm
checked_div_ceil_i8_unb_pow2:
	mov r8d, edx
	cmp r8b, 7
	ja .LBB39_2
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
.LBB39_2:
	cmp r8b, 8
	setb al
```
## `checked_div_ceil_i16_unb_pow2`
```asm
checked_div_ceil_i16_unb_pow2:
	mov eax, edx
	cmp dl, 15
	ja .LBB36_1
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
.LBB36_1:
	xor eax, eax
```
## `checked_div_ceil_i32_unb_pow2`
```asm
checked_div_ceil_i32_unb_pow2:
	cmp dl, 31
	ja .LBB37_1
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
.LBB37_1:
	xor eax, eax
```
## `checked_div_ceil_i64_unb_pow2`
```asm
checked_div_ceil_i64_unb_pow2:
	cmp dl, 63
	ja .LBB38_1
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
.LBB38_1:
	xor eax, eax
```
## `checked_div_ceil_i128_unb_pow2`
```asm
checked_div_ceil_i128_unb_pow2:
	push rsi
	push rdi
	push rbx
	mov rax, rcx
	test r9b, r9b
	js .LBB35_1
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
	jmp .LBB35_3
.LBB35_1:
	xor ecx, ecx
.LBB35_3:
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	pop rbx
	pop rdi
	pop rsi
```
## `checked_div_ceil_u8_unb_pow2`
```asm
checked_div_ceil_u8_unb_pow2:
	mov r8d, edx
	cmp r8b, 7
	ja .LBB44_2
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
.LBB44_2:
	cmp r8b, 8
	setb al
```
## `checked_div_ceil_u16_unb_pow2`
```asm
checked_div_ceil_u16_unb_pow2:
	mov eax, edx
	cmp dl, 15
	ja .LBB41_1
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
.LBB41_1:
	xor eax, eax
```
## `checked_div_ceil_u32_unb_pow2`
```asm
checked_div_ceil_u32_unb_pow2:
	cmp dl, 31
	ja .LBB42_1
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
.LBB42_1:
	xor eax, eax
```
## `checked_div_ceil_u64_unb_pow2`
```asm
checked_div_ceil_u64_unb_pow2:
	cmp dl, 63
	ja .LBB43_1
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
.LBB43_1:
	xor eax, eax
```
## `checked_div_ceil_u128_unb_pow2`
```asm
checked_div_ceil_u128_unb_pow2:
	push rsi
	push rdi
	mov rax, rcx
	test r9b, r9b
	js .LBB40_1
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
	jmp .LBB40_3
.LBB40_1:
	xor ecx, ecx
.LBB40_3:
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	pop rdi
	pop rsi
```
## `checked_div_floor_i8_unb_pow2`
```asm
checked_div_floor_i8_unb_pow2:
	mov r8d, ecx
	cmp dl, 8
	setb al
	mov ecx, edx
	sar r8b, cl
	mov edx, r8d
```
## `checked_div_floor_i16_unb_pow2`
```asm
checked_div_floor_i16_unb_pow2:
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
checked_div_floor_i32_unb_pow2:
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
checked_div_floor_i64_unb_pow2:
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
checked_div_floor_i128_unb_pow2:
	mov rax, rcx
	test r9b, r9b
	js .LBB45_1
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
.LBB45_1:
	xor ecx, ecx
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
```
## `checked_div_floor_u8_unb_pow2`
```asm
checked_div_floor_u8_unb_pow2:
	mov r8d, ecx
	cmp dl, 8
	setb al
	mov ecx, edx
	shr r8b, cl
	mov edx, r8d
```
## `checked_div_floor_u16_unb_pow2`
```asm
checked_div_floor_u16_unb_pow2:
	mov r8d, edx
	movzx edx, cx
	xor eax, eax
	cmp r8b, 16
	setb al
	mov ecx, r8d
	shr edx, cl
```
## `checked_div_floor_u32_unb_pow2`
```asm
checked_div_floor_u32_unb_pow2:
	mov r8d, ecx
	xor eax, eax
	cmp dl, 32
	setb al
	mov ecx, edx
	shr r8d, cl
	mov edx, r8d
```
## `checked_div_floor_u64_unb_pow2`
```asm
checked_div_floor_u64_unb_pow2:
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
checked_div_floor_u128_unb_pow2:
	mov rax, rcx
	test r9b, r9b
	js .LBB50_1
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
.LBB50_1:
	xor ecx, ecx
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
```
## `checked_div_i8_unb_pow2`
```asm
checked_div_i8_unb_pow2:
	mov r8d, ecx
	cmp dl, 7
	ja .LBB59_3
	test r8b, r8b
	js .LBB59_2
	mov ecx, edx
	shr r8b, cl
.LBB59_3:
	cmp dl, 8
	setb al
	mov edx, r8d
	ret
.LBB59_2:
	mov al, -1
	mov ecx, edx
	shl al, cl
	not al
	add r8b, al
	sar r8b, cl
	cmp dl, 8
	setb al
	mov edx, r8d
```
## `checked_div_i16_unb_pow2`
```asm
checked_div_i16_unb_pow2:
	cmp dl, 15
	ja .LBB56_1
	mov eax, ecx
	movzx ecx, dl
	test ax, ax
	js .LBB56_4
	movzx edx, ax
	shr edx, cl
	mov ax, 1
	ret
.LBB56_1:
	xor eax, eax
	ret
.LBB56_4:
	mov edx, -1
	shl edx, cl
	not edx
	add eax, edx
	movsx edx, ax
	sar edx, cl
	mov ax, 1
```
## `checked_div_i32_unb_pow2`
```asm
checked_div_i32_unb_pow2:
	cmp dl, 31
	ja .LBB57_1
	mov r8d, ecx
	test ecx, ecx
	js .LBB57_4
	mov ecx, edx
	shr r8d, cl
	mov eax, 1
	mov edx, r8d
	ret
.LBB57_1:
	xor eax, eax
	mov edx, r8d
	ret
.LBB57_4:
	mov eax, -1
	mov ecx, edx
	shl eax, cl
	not eax
	add r8d, eax
	sar r8d, cl
	mov eax, 1
	mov edx, r8d
```
## `checked_div_i64_unb_pow2`
```asm
checked_div_i64_unb_pow2:
	cmp dl, 63
	ja .LBB58_1
	mov r8, rcx
	movzx ecx, dl
	test r8, r8
	js .LBB58_4
	shr r8, cl
	mov eax, 1
	mov rdx, r8
	ret
.LBB58_1:
	xor eax, eax
	mov rdx, r8
	ret
.LBB58_4:
	mov rax, -1
	shl rax, cl
	not rax
	add r8, rax
	sar r8, cl
	mov eax, 1
	mov rdx, r8
```
## `checked_div_i128_unb_pow2`
```asm
checked_div_i128_unb_pow2:
	mov rax, rcx
	test r9b, r9b
	js .LBB55_1
	test r8, r8
	js .LBB55_4
	mov ecx, r9d
	shrd rdx, r8, cl
	shr r8, cl
	xor ecx, ecx
	test r9b, 64
	cmovne rdx, r8
	cmovne r8, rcx
	jmp .LBB55_6
.LBB55_1:
	xor ecx, ecx
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	ret
.LBB55_4:
	mov r10, -1
	mov r11, -1
	mov ecx, r9d
	shl r11, cl
	xor ecx, ecx
	test r9b, 64
	cmove rcx, r11
	cmove r11, r10
	not r11
	not rcx
	add rdx, rcx
	adc r8, r11
	mov ecx, r9d
	shrd rdx, r8, cl
	mov r10, r8
	sar r10, cl
	test r9b, 64
	cmovne rdx, r10
	sar r8, 63
	test r9b, 64
	cmove r8, r10
.LBB55_6:
	mov qword ptr [rax + 16], rdx
	mov qword ptr [rax + 24], r8
	mov ecx, 1
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
```
## `checked_div_round_i8_unb_pow2`
```asm
checked_div_round_i8_unb_pow2:
	mov eax, edx
	cmp al, 7
	ja .LBB64_2
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
.LBB64_2:
	cmp al, 8
	setb al
```
## `checked_div_round_i16_unb_pow2`
```asm
checked_div_round_i16_unb_pow2:
	mov r8d, edx
	xor eax, eax
	cmp dl, 15
	ja .LBB61_1
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
.LBB61_1:
```
## `checked_div_round_i32_unb_pow2`
```asm
checked_div_round_i32_unb_pow2:
	cmp dl, 31
	ja .LBB62_1
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
.LBB62_1:
	xor eax, eax
```
## `checked_div_round_i64_unb_pow2`
```asm
checked_div_round_i64_unb_pow2:
	cmp dl, 63
	ja .LBB63_1
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
.LBB63_1:
	xor eax, eax
```
## `checked_div_round_i128_unb_pow2`
```asm
checked_div_round_i128_unb_pow2:
	push rsi
	mov rax, rcx
	test r9b, r9b
	js .LBB60_1
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
.LBB60_1:
	xor ecx, ecx
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	pop rsi
```
## `checked_div_round_u8_unb_pow2`
```asm
checked_div_round_u8_unb_pow2:
	mov eax, edx
	mov edx, ecx
	cmp al, 7
	ja .LBB69_2
	mov r8b, 1
	mov ecx, eax
	shl r8b, cl
	shr r8b
	and r8b, dl
	shr dl, cl
	cmp r8b, 1
	sbb dl, -1
.LBB69_2:
	cmp al, 8
	setb al
```
## `checked_div_round_u16_unb_pow2`
```asm
checked_div_round_u16_unb_pow2:
	mov eax, edx
	cmp dl, 15
	ja .LBB66_1
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
.LBB66_1:
	xor eax, eax
```
## `checked_div_round_u32_unb_pow2`
```asm
checked_div_round_u32_unb_pow2:
	mov eax, edx
	cmp dl, 31
	ja .LBB67_1
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
.LBB67_1:
	xor eax, eax
```
## `checked_div_round_u64_unb_pow2`
```asm
checked_div_round_u64_unb_pow2:
	mov eax, edx
	cmp dl, 63
	ja .LBB68_1
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
.LBB68_1:
	xor eax, eax
```
## `checked_div_round_u128_unb_pow2`
```asm
checked_div_round_u128_unb_pow2:
	push r14
	push rsi
	push rdi
	push rbx
	mov rax, rcx
	test r9b, r9b
	js .LBB65_1
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
	jmp .LBB65_3
.LBB65_1:
	xor r11d, r11d
.LBB65_3:
	mov qword ptr [rax], r11
	mov qword ptr [rax + 8], 0
	pop rbx
	pop rdi
	pop rsi
	pop r14
```
## `checked_floor_to_multiple_i8_unb_pow2`
```asm
checked_floor_to_multiple_i8_unb_pow2:
	mov r9d, edx
	mov r8d, ecx
	cmp dl, 8
	setb al
	mov dl, -1
	mov ecx, r9d
	shl dl, cl
	and dl, r8b
```
## `checked_floor_to_multiple_i16_unb_pow2`
```asm
checked_floor_to_multiple_i16_unb_pow2:
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
## `checked_floor_to_multiple_i32_unb_pow2`
```asm
checked_floor_to_multiple_i32_unb_pow2:
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
## `checked_floor_to_multiple_i64_unb_pow2`
```asm
checked_floor_to_multiple_i64_unb_pow2:
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
checked_floor_to_multiple_i128_unb_pow2:
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
	cmove rcx, r11
	and rcx, rdx
	and r10, r8
	mov qword ptr [rax + 24], r10
	mov qword ptr [rax + 16], rcx
	mov ecx, 1
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	ret
.LBB70_1:
	xor ecx, ecx
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
```
## `checked_mod_floor_i8_unb_pow2`
```asm
checked_mod_floor_i8_unb_pow2:
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
## `checked_mod_floor_i16_unb_pow2`
```asm
checked_mod_floor_i16_unb_pow2:
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
## `checked_mod_floor_i32_unb_pow2`
```asm
checked_mod_floor_i32_unb_pow2:
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
## `checked_mod_floor_i64_unb_pow2`
```asm
checked_mod_floor_i64_unb_pow2:
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
## `checked_mod_floor_i128_unb_pow2`
```asm
checked_mod_floor_i128_unb_pow2:
	mov rax, rcx
	test r9b, r9b
	js .LBB75_1
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
.LBB75_1:
	xor ecx, ecx
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
```
## `checked_mul_i8_pow2`
```asm
checked_mul_i8_pow2:
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
checked_mul_i8_unb_pow2:
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
checked_mul_i16_pow2:
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
checked_mul_i16_unb_pow2:
	cmp dl, 15
	ja .LBB83_1
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
.LBB83_1:
	xor eax, eax
```
## `checked_mul_i32_pow2`
```asm
checked_mul_i32_pow2:
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
checked_mul_i32_unb_pow2:
	cmp dl, 31
	ja .LBB85_1
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
.LBB85_1:
	xor eax, eax
```
## `checked_mul_i64_pow2`
```asm
checked_mul_i64_pow2:
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
checked_mul_i64_unb_pow2:
	cmp dl, 63
	ja .LBB87_1
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
.LBB87_1:
	xor eax, eax
```
## `checked_mul_i128_pow2`
```asm
checked_mul_i128_pow2:
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
	jne .LBB80_2
	mov qword ptr [rax + 16], r11
	mov qword ptr [rax + 24], r10
	mov esi, 1
.LBB80_2:
	mov qword ptr [rax], rsi
	mov qword ptr [rax + 8], 0
	pop rbx
	pop rdi
	pop rsi
	pop r14
```
## `checked_mul_i128_unb_pow2`
```asm
checked_mul_i128_unb_pow2:
	push r14
	push rsi
	push rdi
	push rbx
	mov rax, rcx
	test r9b, r9b
	js .LBB81_1
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
	jne .LBB81_4
	mov qword ptr [rax + 16], r11
	mov qword ptr [rax + 24], r10
	mov esi, 1
	jmp .LBB81_4
.LBB81_1:
	xor esi, esi
.LBB81_4:
	mov qword ptr [rax], rsi
	mov qword ptr [rax + 8], 0
	pop rbx
	pop rdi
	pop rsi
	pop r14
```
## `checked_mul_u8_pow2`
```asm
checked_mul_u8_pow2:
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
checked_mul_u8_unb_pow2:
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
checked_mul_u16_pow2:
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
checked_mul_u16_unb_pow2:
	cmp dl, 15
	ja .LBB93_1
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
.LBB93_1:
	xor eax, eax
```
## `checked_mul_u32_pow2`
```asm
checked_mul_u32_pow2:
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
checked_mul_u32_unb_pow2:
	cmp dl, 31
	ja .LBB95_1
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
.LBB95_1:
	xor eax, eax
```
## `checked_mul_u64_pow2`
```asm
checked_mul_u64_pow2:
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
checked_mul_u64_unb_pow2:
	cmp dl, 63
	ja .LBB97_1
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
.LBB97_1:
	xor eax, eax
```
## `checked_mul_u128_pow2`
```asm
checked_mul_u128_pow2:
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
	jne .LBB90_2
	mov qword ptr [rax + 16], r11
	mov qword ptr [rax + 24], r10
	mov esi, 1
.LBB90_2:
	mov qword ptr [rax], rsi
	mov qword ptr [rax + 8], 0
	pop rbx
	pop rdi
	pop rsi
	pop r14
```
## `checked_mul_u128_unb_pow2`
```asm
checked_mul_u128_unb_pow2:
	push r14
	push rsi
	push rdi
	push rbx
	mov rax, rcx
	test r9b, r9b
	js .LBB91_1
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
	jne .LBB91_4
	mov qword ptr [rax + 16], r11
	mov qword ptr [rax + 24], r10
	mov esi, 1
	jmp .LBB91_4
.LBB91_1:
	xor esi, esi
.LBB91_4:
	mov qword ptr [rax], rsi
	mov qword ptr [rax + 8], 0
	pop rbx
	pop rdi
	pop rsi
	pop r14
```
## `div_ceil_i8_pow2`
```asm
div_ceil_i8_pow2:
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
div_ceil_i8_unb_pow2:
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
div_ceil_i16_pow2:
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
div_ceil_i16_unb_pow2:
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
## `div_ceil_i32_pow2`
```asm
div_ceil_i32_pow2:
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
## `div_ceil_i32_unb_pow2`
```asm
div_ceil_i32_unb_pow2:
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
## `div_ceil_i64_pow2`
```asm
div_ceil_i64_pow2:
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
## `div_ceil_i64_unb_pow2`
```asm
div_ceil_i64_unb_pow2:
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
## `div_ceil_i128_pow2`
```asm
div_ceil_i128_pow2:
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
## `div_ceil_i128_unb_pow2`
```asm
div_ceil_i128_unb_pow2:
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
div_ceil_u8_pow2:
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
div_ceil_u8_unb_pow2:
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
div_ceil_u16_pow2:
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
div_ceil_u16_unb_pow2:
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
## `div_ceil_u32_pow2`
```asm
div_ceil_u32_pow2:
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
## `div_ceil_u32_unb_pow2`
```asm
div_ceil_u32_unb_pow2:
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
## `div_ceil_u64_pow2`
```asm
div_ceil_u64_pow2:
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
## `div_ceil_u64_unb_pow2`
```asm
div_ceil_u64_unb_pow2:
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
## `div_ceil_u128_pow2`
```asm
div_ceil_u128_pow2:
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
## `div_ceil_u128_unb_pow2`
```asm
div_ceil_u128_unb_pow2:
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
div_floor_i8_pow2:
	mov eax, ecx
	mov ecx, edx
	sar al, cl
```
## `div_floor_i8_unb_pow2`
```asm
div_floor_i8_unb_pow2:
	mov eax, ecx
	and dl, 7
	mov ecx, edx
	sar al, cl
```
## `div_floor_i16_pow2`
```asm
div_floor_i16_pow2:
	movsx eax, cx
	mov ecx, edx
	sar eax, cl
```
## `div_floor_i16_unb_pow2`
```asm
div_floor_i16_unb_pow2:
	movsx eax, cx
	and dl, 15
	mov ecx, edx
	sar eax, cl
```
## `div_floor_i32_pow2`
```asm
div_floor_i32_pow2:
	mov eax, ecx
	mov ecx, edx
	sar eax, cl
```
## `div_floor_i32_unb_pow2`
```asm
div_floor_i32_unb_pow2:
	mov eax, ecx
	mov ecx, edx
	sar eax, cl
```
## `div_floor_i64_pow2`
```asm
div_floor_i64_pow2:
	mov rax, rcx
	mov ecx, edx
	sar rax, cl
```
## `div_floor_i64_unb_pow2`
```asm
div_floor_i64_unb_pow2:
	mov rax, rcx
	mov ecx, edx
	sar rax, cl
```
## `div_floor_i128_pow2`
```asm
div_floor_i128_pow2:
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
## `div_floor_i128_unb_pow2`
```asm
div_floor_i128_unb_pow2:
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
div_floor_u8_pow2:
	mov eax, ecx
	mov ecx, edx
	shr al, cl
```
## `div_floor_u8_unb_pow2`
```asm
div_floor_u8_unb_pow2:
	mov eax, ecx
	and dl, 7
	mov ecx, edx
	shr al, cl
```
## `div_floor_u16_pow2`
```asm
div_floor_u16_pow2:
	movzx eax, cx
	mov ecx, edx
	shr eax, cl
```
## `div_floor_u16_unb_pow2`
```asm
div_floor_u16_unb_pow2:
	movzx eax, cx
	and dl, 15
	mov ecx, edx
	shr eax, cl
```
## `div_floor_u32_pow2`
```asm
div_floor_u32_pow2:
	mov eax, ecx
	mov ecx, edx
	shr eax, cl
```
## `div_floor_u32_unb_pow2`
```asm
div_floor_u32_unb_pow2:
	mov eax, ecx
	mov ecx, edx
	shr eax, cl
```
## `div_floor_u64_pow2`
```asm
div_floor_u64_pow2:
	mov rax, rcx
	mov ecx, edx
	shr rax, cl
```
## `div_floor_u64_unb_pow2`
```asm
div_floor_u64_unb_pow2:
	mov rax, rcx
	mov ecx, edx
	shr rax, cl
```
## `div_floor_u128_pow2`
```asm
div_floor_u128_pow2:
	mov rax, rcx
	mov ecx, r8d
	shrd rax, rdx, cl
	shr rdx, cl
	xor ecx, ecx
	test r8b, 64
	cmovne rax, rdx
	cmovne rdx, rcx
```
## `div_floor_u128_unb_pow2`
```asm
div_floor_u128_unb_pow2:
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
div_i8_pow2:
	mov eax, ecx
	test cl, cl
	js .LBB148_1
	mov ecx, edx
	shr al, cl
	ret
.LBB148_1:
	mov r8b, -1
	mov ecx, edx
	shl r8b, cl
	not r8b
	add al, r8b
	sar al, cl
```
## `div_i8_unb_pow2`
```asm
div_i8_unb_pow2:
	mov eax, ecx
	and dl, 7
	test cl, cl
	js .LBB149_1
	mov ecx, edx
	shr al, cl
	ret
.LBB149_1:
	mov r8b, -1
	mov ecx, edx
	shl r8b, cl
	not r8b
	add al, r8b
	sar al, cl
```
## `div_i16_pow2`
```asm
div_i16_pow2:
	mov eax, ecx
	movzx ecx, dl
	test ax, ax
	js .LBB142_1
	movzx eax, ax
	shr eax, cl
	ret
.LBB142_1:
	mov edx, -1
	shl edx, cl
	not edx
	add eax, edx
	cwde
	sar eax, cl
```
## `div_i16_unb_pow2`
```asm
div_i16_unb_pow2:
	mov eax, ecx
	and dl, 15
	movzx ecx, dl
	test ax, ax
	js .LBB143_1
	movzx eax, ax
	shr eax, cl
	ret
.LBB143_1:
	mov edx, -1
	shl edx, cl
	not edx
	add eax, edx
	cwde
	sar eax, cl
```
## `div_i32_pow2`
```asm
div_i32_pow2:
	mov eax, ecx
	test ecx, ecx
	js .LBB144_1
	mov ecx, edx
	shr eax, cl
	ret
.LBB144_1:
	mov r8d, -1
	mov ecx, edx
	shl r8d, cl
	not r8d
	add eax, r8d
	sar eax, cl
```
## `div_i32_unb_pow2`
```asm
div_i32_unb_pow2:
	mov eax, ecx
	and dl, 31
	test ecx, ecx
	js .LBB145_1
	mov ecx, edx
	shr eax, cl
	ret
.LBB145_1:
	mov r8d, -1
	mov ecx, edx
	shl r8d, cl
	not r8d
	add eax, r8d
	sar eax, cl
```
## `div_i64_pow2`
```asm
div_i64_pow2:
	mov rax, rcx
	movzx ecx, dl
	test rax, rax
	js .LBB146_1
	shr rax, cl
	ret
.LBB146_1:
	mov rdx, -1
	shl rdx, cl
	not rdx
	add rax, rdx
	sar rax, cl
```
## `div_i64_unb_pow2`
```asm
div_i64_unb_pow2:
	mov rax, rcx
	and dl, 63
	movzx ecx, dl
	test rax, rax
	js .LBB147_1
	shr rax, cl
	ret
.LBB147_1:
	mov rdx, -1
	shl rdx, cl
	not rdx
	add rax, rdx
	sar rax, cl
```
## `div_i128_pow2`
```asm
div_i128_pow2:
	mov rax, rcx
	test rdx, rdx
	js .LBB140_1
	mov ecx, r8d
	shrd rax, rdx, cl
	shr rdx, cl
	xor ecx, ecx
	test r8b, 64
	cmovne rax, rdx
	cmovne rdx, rcx
	ret
.LBB140_1:
	mov r9, -1
	mov r10, -1
	mov ecx, r8d
	shl r10, cl
	xor ecx, ecx
	test r8b, 64
	cmove rcx, r10
	cmove r10, r9
	not r10
	not rcx
	add rax, rcx
	adc rdx, r10
	mov ecx, r8d
	shrd rax, rdx, cl
	mov r9, rdx
	sar r9, cl
	test r8b, 64
	cmovne rax, r9
	sar rdx, 63
	test r8b, 64
	cmove rdx, r9
```
## `div_i128_unb_pow2`
```asm
div_i128_unb_pow2:
	mov rax, rcx
	and r8b, 127
	test rdx, rdx
	js .LBB141_1
	mov ecx, r8d
	shrd rax, rdx, cl
	shr rdx, cl
	xor ecx, ecx
	test r8b, 64
	cmovne rax, rdx
	cmovne rdx, rcx
	ret
.LBB141_1:
	mov r9, -1
	mov r10, -1
	mov ecx, r8d
	shl r10, cl
	xor ecx, ecx
	test r8b, 64
	cmove rcx, r10
	cmove r10, r9
	not r10
	not rcx
	add rax, rcx
	adc rdx, r10
	mov ecx, r8d
	shrd rax, rdx, cl
	mov r9, rdx
	sar r9, cl
	test r8b, 64
	cmovne rax, r9
	sar rdx, 63
	test r8b, 64
	cmove rdx, r9
```
## `div_round_i8_pow2`
```asm
div_round_i8_pow2:
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
div_round_i8_unb_pow2:
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
div_round_i16_pow2:
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
div_round_i16_unb_pow2:
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
## `div_round_i32_pow2`
```asm
div_round_i32_pow2:
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
## `div_round_i32_unb_pow2`
```asm
div_round_i32_unb_pow2:
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
## `div_round_i64_pow2`
```asm
div_round_i64_pow2:
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
## `div_round_i64_unb_pow2`
```asm
div_round_i64_unb_pow2:
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
## `div_round_i128_pow2`
```asm
div_round_i128_pow2:
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
## `div_round_i128_unb_pow2`
```asm
div_round_i128_unb_pow2:
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
div_round_u8_pow2:
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
div_round_u8_unb_pow2:
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
div_round_u16_pow2:
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
div_round_u16_unb_pow2:
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
## `div_round_u32_pow2`
```asm
div_round_u32_pow2:
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
## `div_round_u32_unb_pow2`
```asm
div_round_u32_unb_pow2:
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
## `div_round_u64_pow2`
```asm
div_round_u64_pow2:
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
## `div_round_u64_unb_pow2`
```asm
div_round_u64_unb_pow2:
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
## `div_round_u128_pow2`
```asm
div_round_u128_pow2:
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
## `div_round_u128_unb_pow2`
```asm
div_round_u128_unb_pow2:
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
## `floor_to_multiple_i8_pow2`
```asm
floor_to_multiple_i8_pow2:
	mov eax, ecx
	mov ecx, edx
	shr al, cl
	shl al, cl
```
## `floor_to_multiple_i8_unb_pow2`
```asm
floor_to_multiple_i8_unb_pow2:
	mov eax, ecx
	and dl, 7
	mov ecx, edx
	shr al, cl
	shl al, cl
```
## `floor_to_multiple_i16_pow2`
```asm
floor_to_multiple_i16_pow2:
	movzx eax, cx
	mov ecx, edx
	shr eax, cl
	shl eax, cl
```
## `floor_to_multiple_i16_unb_pow2`
```asm
floor_to_multiple_i16_unb_pow2:
	movzx eax, cx
	and dl, 15
	mov ecx, edx
	shr eax, cl
	shl eax, cl
```
## `floor_to_multiple_i32_pow2`
```asm
floor_to_multiple_i32_pow2:
	mov eax, ecx
	mov ecx, edx
	shr eax, cl
	shl eax, cl
```
## `floor_to_multiple_i32_unb_pow2`
```asm
floor_to_multiple_i32_unb_pow2:
	mov eax, ecx
	mov ecx, edx
	shr eax, cl
	shl eax, cl
```
## `floor_to_multiple_i64_pow2`
```asm
floor_to_multiple_i64_pow2:
	mov rax, rcx
	mov ecx, edx
	shr rax, cl
	shl rax, cl
```
## `floor_to_multiple_i64_unb_pow2`
```asm
floor_to_multiple_i64_unb_pow2:
	mov rax, rcx
	mov ecx, edx
	shr rax, cl
	shl rax, cl
```
## `floor_to_multiple_i128_pow2`
```asm
floor_to_multiple_i128_pow2:
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
## `floor_to_multiple_i128_unb_pow2`
```asm
floor_to_multiple_i128_unb_pow2:
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
## `is_multiple_of_i8_pow2`
```asm
is_multiple_of_i8_pow2:
	movzx eax, cl
	or eax, 256
	rep bsf	eax, eax
	cmp al, dl
	setae al
```
## `is_multiple_of_i16_pow2`
```asm
is_multiple_of_i16_pow2:
	or ecx, 65536
	rep bsf	eax, ecx
	movzx ecx, dl
	cmp ax, cx
	setae al
```
## `is_multiple_of_i32_pow2`
```asm
is_multiple_of_i32_pow2:
	mov eax, 32
	rep bsf	eax, ecx
	movzx ecx, dl
	cmp eax, ecx
	setae al
```
## `is_multiple_of_i64_pow2`
```asm
is_multiple_of_i64_pow2:
	mov eax, 64
	rep bsf	rax, rcx
	movzx ecx, dl
	cmp eax, ecx
	setae al
```
## `is_multiple_of_i128_pow2`
```asm
is_multiple_of_i128_pow2:
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
## `mod_floor_i8_pow2`
```asm
mod_floor_i8_pow2:
	mov r8d, ecx
	mov al, -1
	mov ecx, edx
	shl al, cl
	not al
	and al, r8b
```
## `mod_floor_i8_unb_pow2`
```asm
mod_floor_i8_unb_pow2:
	mov r8d, ecx
	and dl, 7
	mov al, -1
	mov ecx, edx
	shl al, cl
	not al
	and al, r8b
```
## `mod_floor_i16_pow2`
```asm
mod_floor_i16_pow2:
	mov r8d, ecx
	mov eax, -1
	mov ecx, edx
	shl eax, cl
	not eax
	and eax, r8d
```
## `mod_floor_i16_unb_pow2`
```asm
mod_floor_i16_unb_pow2:
	mov r8d, ecx
	and dl, 15
	mov eax, -1
	mov ecx, edx
	shl eax, cl
	not eax
	and eax, r8d
```
## `mod_floor_i32_pow2`
```asm
mod_floor_i32_pow2:
	mov r8d, ecx
	mov eax, -1
	mov ecx, edx
	shl eax, cl
	not eax
	and eax, r8d
```
## `mod_floor_i32_unb_pow2`
```asm
mod_floor_i32_unb_pow2:
	mov r8d, ecx
	mov eax, -1
	mov ecx, edx
	shl eax, cl
	not eax
	and eax, r8d
```
## `mod_floor_i64_pow2`
```asm
mod_floor_i64_pow2:
	mov r8, rcx
	mov rax, -1
	mov ecx, edx
	shl rax, cl
	not rax
	and rax, r8
```
## `mod_floor_i64_unb_pow2`
```asm
mod_floor_i64_unb_pow2:
	mov r8, rcx
	mov rax, -1
	mov ecx, edx
	shl rax, cl
	not rax
	and rax, r8
```
## `mod_floor_i128_pow2`
```asm
mod_floor_i128_pow2:
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
## `mod_floor_i128_unb_pow2`
```asm
mod_floor_i128_unb_pow2:
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
## `mul_i8_pow2`
```asm
mul_i8_pow2:
	mov eax, ecx
	mov ecx, edx
	shl al, cl
```
## `mul_i8_unb_pow2`
```asm
mul_i8_unb_pow2:
	mov eax, ecx
	and dl, 7
	mov ecx, edx
	shl al, cl
```
## `mul_i16_pow2`
```asm
mul_i16_pow2:
	mov eax, ecx
	mov ecx, edx
	shl eax, cl
```
## `mul_i16_unb_pow2`
```asm
mul_i16_unb_pow2:
	mov eax, ecx
	and dl, 15
	mov ecx, edx
	shl eax, cl
```
## `mul_i32_pow2`
```asm
mul_i32_pow2:
	mov eax, ecx
	mov ecx, edx
	shl eax, cl
```
## `mul_i32_unb_pow2`
```asm
mul_i32_unb_pow2:
	mov eax, ecx
	mov ecx, edx
	shl eax, cl
```
## `mul_i64_pow2`
```asm
mul_i64_pow2:
	mov rax, rcx
	mov ecx, edx
	shl rax, cl
```
## `mul_i64_unb_pow2`
```asm
mul_i64_unb_pow2:
	mov rax, rcx
	mov ecx, edx
	shl rax, cl
```
## `mul_i128_pow2`
```asm
mul_i128_pow2:
	mov r9, rcx
	mov ecx, r8d
	shld rdx, r9, cl
	shl r9, cl
	xor eax, eax
	test r8b, 64
	cmovne rdx, r9
	cmove rax, r9
```
## `mul_i128_unb_pow2`
```asm
mul_i128_unb_pow2:
	mov r9, rcx
	mov ecx, r8d
	shld rdx, r9, cl
	shl r9, cl
	xor eax, eax
	test r8b, 64
	cmovne rdx, r9
	cmove rax, r9
```
## `unbounded_ceil_to_multiple_i8_unb_pow2`
```asm
unbounded_ceil_to_multiple_i8_unb_pow2:
	mov r8d, ecx
	cmp dl, 8
	jae .LBB209_1
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
.LBB209_1:
	test r8b, r8b
	setle al
	xor r8d, r8d
	mov edx, r8d
```
## `unbounded_ceil_to_multiple_i16_unb_pow2`
```asm
unbounded_ceil_to_multiple_i16_unb_pow2:
	mov r8d, ecx
	cmp dl, 16
	jae .LBB206_1
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
.LBB206_1:
	test r8w, r8w
	setle al
	xor r8d, r8d
	movzx eax, al
	mov edx, r8d
```
## `unbounded_ceil_to_multiple_i32_unb_pow2`
```asm
unbounded_ceil_to_multiple_i32_unb_pow2:
	mov r8d, ecx
	cmp dl, 32
	jae .LBB207_1
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
.LBB207_1:
	test r8d, r8d
	setle al
	xor r8d, r8d
	movzx eax, al
	mov edx, r8d
```
## `unbounded_ceil_to_multiple_i64_unb_pow2`
```asm
unbounded_ceil_to_multiple_i64_unb_pow2:
	mov r8, rcx
	cmp dl, 64
	jae .LBB208_1
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
.LBB208_1:
	test r8, r8
	setle al
	xor r8d, r8d
	movzx eax, al
	mov rdx, r8
```
## `unbounded_ceil_to_multiple_i128_unb_pow2`
```asm
unbounded_ceil_to_multiple_i128_unb_pow2:
	push rsi
	mov rax, rcx
	test r9b, r9b
	js .LBB205_4
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
	jo .LBB205_5
	and rdx, r9
	and r8, r10
	mov rcx, rdx
.LBB205_3:
	mov qword ptr [rax + 16], rcx
	mov qword ptr [rax + 24], r8
	mov ecx, 1
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	pop rsi
	ret
.LBB205_4:
	xor ecx, ecx
	cmp rdx, 1
	sbb r8, 0
	mov r8d, 0
	jl .LBB205_3
.LBB205_5:
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	pop rsi
```
## `unbounded_ceil_to_multiple_u8_unb_pow2`
```asm
unbounded_ceil_to_multiple_u8_unb_pow2:
	mov eax, ecx
	cmp dl, 8
	jae .LBB214_1
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
.LBB214_1:
	test al, al
	sete al
	xor edx, edx
```
## `unbounded_ceil_to_multiple_u16_unb_pow2`
```asm
unbounded_ceil_to_multiple_u16_unb_pow2:
	mov eax, ecx
	cmp dl, 16
	jae .LBB211_1
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
.LBB211_1:
	test ax, ax
	sete al
	xor edx, edx
	movzx eax, al
```
## `unbounded_ceil_to_multiple_u32_unb_pow2`
```asm
unbounded_ceil_to_multiple_u32_unb_pow2:
	mov eax, ecx
	cmp dl, 32
	jae .LBB212_1
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
.LBB212_1:
	test eax, eax
	sete al
	xor edx, edx
	movzx eax, al
```
## `unbounded_ceil_to_multiple_u64_unb_pow2`
```asm
unbounded_ceil_to_multiple_u64_unb_pow2:
	mov rax, rcx
	cmp dl, 64
	jae .LBB213_1
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
.LBB213_1:
	test rax, rax
	sete al
	xor edx, edx
	movzx eax, al
```
## `unbounded_ceil_to_multiple_u128_unb_pow2`
```asm
unbounded_ceil_to_multiple_u128_unb_pow2:
	push rsi
	mov rax, rcx
	test r9b, r9b
	js .LBB210_4
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
	jb .LBB210_5
	and rdx, r9
	and r8, r10
	mov rcx, rdx
.LBB210_3:
	mov qword ptr [rax + 16], rcx
	mov qword ptr [rax + 24], r8
	mov ecx, 1
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	pop rsi
	ret
.LBB210_4:
	xor ecx, ecx
	or rdx, r8
	mov r8d, 0
	je .LBB210_3
.LBB210_5:
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	pop rsi
```
## `unbounded_div_ceil_i8_unb_pow2`
```asm
unbounded_div_ceil_i8_unb_pow2:
	mov r8d, ecx
	cmp dl, 8
	jae .LBB219_1
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
.LBB219_1:
	test r8b, r8b
	setg al
```
## `unbounded_div_ceil_i16_unb_pow2`
```asm
unbounded_div_ceil_i16_unb_pow2:
	mov r8d, edx
	mov edx, ecx
	cmp r8b, 16
	jae .LBB216_1
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
.LBB216_1:
	xor eax, eax
	test dx, dx
	setg al
```
## `unbounded_div_ceil_i32_unb_pow2`
```asm
unbounded_div_ceil_i32_unb_pow2:
	mov r8d, edx
	mov edx, ecx
	cmp r8b, 32
	jae .LBB217_1
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
.LBB217_1:
	xor eax, eax
	test edx, edx
	setg al
```
## `unbounded_div_ceil_i64_unb_pow2`
```asm
unbounded_div_ceil_i64_unb_pow2:
	mov r8d, edx
	mov rdx, rcx
	cmp r8b, 64
	jae .LBB218_1
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
.LBB218_1:
	xor eax, eax
	test rdx, rdx
	setg al
```
## `unbounded_div_ceil_i128_unb_pow2`
```asm
unbounded_div_ceil_i128_unb_pow2:
	push rsi
	push rdi
	mov r9d, r8d
	mov rax, rdx
	mov r8, rcx
	test r9b, r9b
	js .LBB215_1
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
.LBB215_1:
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
unbounded_div_ceil_u8_unb_pow2:
	mov r8d, ecx
	cmp dl, 8
	jae .LBB224_1
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
.LBB224_1:
	test r8b, r8b
	setne al
```
## `unbounded_div_ceil_u16_unb_pow2`
```asm
unbounded_div_ceil_u16_unb_pow2:
	mov r8d, edx
	mov edx, ecx
	cmp r8b, 16
	jae .LBB221_1
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
.LBB221_1:
	xor eax, eax
	test dx, dx
	setne al
```
## `unbounded_div_ceil_u32_unb_pow2`
```asm
unbounded_div_ceil_u32_unb_pow2:
	mov r8d, edx
	mov edx, ecx
	cmp r8b, 32
	jae .LBB222_1
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
.LBB222_1:
	xor eax, eax
	test edx, edx
	setne al
```
## `unbounded_div_ceil_u64_unb_pow2`
```asm
unbounded_div_ceil_u64_unb_pow2:
	mov r8d, edx
	mov rdx, rcx
	cmp r8b, 64
	jae .LBB223_1
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
.LBB223_1:
	xor eax, eax
	test rdx, rdx
	setne al
```
## `unbounded_div_ceil_u128_unb_pow2`
```asm
unbounded_div_ceil_u128_unb_pow2:
	push rsi
	mov eax, r8d
	mov r8, rcx
	test al, al
	js .LBB220_1
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
.LBB220_1:
	xor eax, eax
	or r8, rdx
	setne al
	xor edx, edx
	pop rsi
```
## `unbounded_div_floor_i8_unb_pow2`
```asm
unbounded_div_floor_i8_unb_pow2:
	mov eax, ecx
	movzx edx, dl
	cmp dl, 7
	mov ecx, 7
	cmovb ecx, edx
	sar al, cl
```
## `unbounded_div_floor_i16_unb_pow2`
```asm
unbounded_div_floor_i16_unb_pow2:
	movsx eax, cx
	movzx edx, dl
	cmp dl, 15
	mov ecx, 15
	cmovb ecx, edx
	sar eax, cl
```
## `unbounded_div_floor_i32_unb_pow2`
```asm
unbounded_div_floor_i32_unb_pow2:
	mov eax, ecx
	movzx edx, dl
	cmp dl, 31
	mov ecx, 31
	cmovb ecx, edx
	sar eax, cl
```
## `unbounded_div_floor_i64_unb_pow2`
```asm
unbounded_div_floor_i64_unb_pow2:
	mov rax, rcx
	movzx edx, dl
	cmp dl, 63
	mov ecx, 63
	cmovb ecx, edx
	sar rax, cl
```
## `unbounded_div_floor_i128_unb_pow2`
```asm
unbounded_div_floor_i128_unb_pow2:
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
## `unbounded_div_floor_u8_unb_pow2`
```asm
unbounded_div_floor_u8_unb_pow2:
	mov eax, ecx
	mov ecx, edx
	shr al, cl
	xor ecx, ecx
	cmp dl, 8
	movzx eax, al
	cmovae eax, ecx
```
## `unbounded_div_floor_u16_unb_pow2`
```asm
unbounded_div_floor_u16_unb_pow2:
	movzx r8d, cx
	mov ecx, edx
	shr r8d, cl
	xor eax, eax
	cmp dl, 16
	cmovb eax, r8d
```
## `unbounded_div_floor_u32_unb_pow2`
```asm
unbounded_div_floor_u32_unb_pow2:
	mov r8d, ecx
	mov ecx, edx
	shr r8d, cl
	xor eax, eax
	cmp dl, 32
	cmovb eax, r8d
```
## `unbounded_div_floor_u64_unb_pow2`
```asm
unbounded_div_floor_u64_unb_pow2:
	mov r8, rcx
	mov ecx, edx
	shr r8, cl
	xor eax, eax
	cmp dl, 64
	cmovb rax, r8
```
## `unbounded_div_floor_u128_unb_pow2`
```asm
unbounded_div_floor_u128_unb_pow2:
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
unbounded_div_i8_unb_pow2:
	cmp dl, 7
	ja .LBB239_1
	mov eax, ecx
	test cl, cl
	js .LBB239_4
	mov ecx, edx
	shr al, cl
	ret
.LBB239_1:
	xor eax, eax
	ret
.LBB239_4:
	mov r8b, -1
	mov ecx, edx
	shl r8b, cl
	not r8b
	add al, r8b
	sar al, cl
```
## `unbounded_div_i16_unb_pow2`
```asm
unbounded_div_i16_unb_pow2:
	cmp dl, 15
	ja .LBB236_1
	mov eax, ecx
	movzx ecx, dl
	test ax, ax
	js .LBB236_4
	movzx eax, ax
	shr eax, cl
	ret
.LBB236_1:
	xor eax, eax
	ret
.LBB236_4:
	mov edx, -1
	shl edx, cl
	not edx
	add eax, edx
	cwde
	sar eax, cl
```
## `unbounded_div_i32_unb_pow2`
```asm
unbounded_div_i32_unb_pow2:
	xor eax, eax
	cmp dl, 31
	ja .LBB237_4
	mov r8d, ecx
	test ecx, ecx
	js .LBB237_2
	mov ecx, edx
	shr r8d, cl
	jmp .LBB237_3
.LBB237_2:
	mov eax, -1
	mov ecx, edx
	shl eax, cl
	not eax
	add r8d, eax
	sar r8d, cl
.LBB237_3:
	mov eax, r8d
.LBB237_4:
```
## `unbounded_div_i64_unb_pow2`
```asm
unbounded_div_i64_unb_pow2:
	cmp dl, 63
	ja .LBB238_1
	mov rax, rcx
	movzx ecx, dl
	test rax, rax
	js .LBB238_4
	shr rax, cl
	ret
.LBB238_1:
	xor eax, eax
	ret
.LBB238_4:
	mov rdx, -1
	shl rdx, cl
	not rdx
	add rax, rdx
	sar rax, cl
```
## `unbounded_div_i128_unb_pow2`
```asm
unbounded_div_i128_unb_pow2:
	test r8b, r8b
	js .LBB235_1
	mov rax, rcx
	test rdx, rdx
	js .LBB235_4
	mov ecx, r8d
	shrd rax, rdx, cl
	shr rdx, cl
	xor ecx, ecx
	test r8b, 64
	cmovne rax, rdx
	cmovne rdx, rcx
	ret
.LBB235_1:
	xor eax, eax
	xor edx, edx
	ret
.LBB235_4:
	mov r9, -1
	mov r10, -1
	mov ecx, r8d
	shl r10, cl
	xor ecx, ecx
	test r8b, 64
	cmove rcx, r10
	cmove r10, r9
	not r10
	not rcx
	add rax, rcx
	adc rdx, r10
	mov ecx, r8d
	shrd rax, rdx, cl
	mov r9, rdx
	sar r9, cl
	test r8b, 64
	cmovne rax, r9
	sar rdx, 63
	test r8b, 64
	cmove rdx, r9
```
## `unbounded_div_round_i8_unb_pow2`
```asm
unbounded_div_round_i8_unb_pow2:
	mov r8d, ecx
	cmp dl, 7
	ja .LBB244_2
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
.LBB244_2:
	neg r8b
	jno .LBB244_3
	cmp dl, 8
	sete al
	neg al
	ret
.LBB244_3:
	xor eax, eax
```
## `unbounded_div_round_i16_unb_pow2`
```asm
unbounded_div_round_i16_unb_pow2:
	mov r8d, ecx
	cmp dl, 15
	ja .LBB241_2
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
.LBB241_2:
	xor eax, eax
	neg r8w
	jno .LBB241_4
	cmp dl, 16
	sete al
	neg eax
.LBB241_4:
```
## `unbounded_div_round_i32_unb_pow2`
```asm
unbounded_div_round_i32_unb_pow2:
	mov r8d, ecx
	cmp dl, 31
	ja .LBB242_2
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
.LBB242_2:
	xor eax, eax
	neg r8d
	jno .LBB242_4
	xor eax, eax
	cmp dl, 32
	sete al
	neg eax
.LBB242_4:
```
## `unbounded_div_round_i64_unb_pow2`
```asm
unbounded_div_round_i64_unb_pow2:
	mov r8, rcx
	cmp dl, 63
	ja .LBB243_2
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
.LBB243_2:
	xor eax, eax
	neg r8
	jno .LBB243_4
	cmp dl, 64
	sete al
	neg rax
.LBB243_4:
```
## `unbounded_div_round_i128_unb_pow2`
```asm
unbounded_div_round_i128_unb_pow2:
	mov r9, rcx
	test r8b, r8b
	js .LBB240_2
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
.LBB240_2:
	movabs rax, -9223372036854775808
	xor rdx, rax
	xor eax, eax
	or r9, rdx
	mov edx, 0
	jne .LBB240_4
	xor eax, eax
	neg r8b
	seto al
	neg rax
	mov rdx, rax
.LBB240_4:
```
## `unbounded_div_round_u8_unb_pow2`
```asm
unbounded_div_round_u8_unb_pow2:
	cmp dl, 7
	ja .LBB249_1
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
.LBB249_1:
	xor eax, eax
```
## `unbounded_div_round_u16_unb_pow2`
```asm
unbounded_div_round_u16_unb_pow2:
	cmp dl, 15
	ja .LBB246_1
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
.LBB246_1:
	xor eax, eax
```
## `unbounded_div_round_u32_unb_pow2`
```asm
unbounded_div_round_u32_unb_pow2:
	xor eax, eax
	cmp dl, 31
	ja .LBB247_2
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
.LBB247_2:
```
## `unbounded_div_round_u64_unb_pow2`
```asm
unbounded_div_round_u64_unb_pow2:
	cmp dl, 63
	ja .LBB248_1
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
.LBB248_1:
	xor eax, eax
```
## `unbounded_div_round_u128_unb_pow2`
```asm
unbounded_div_round_u128_unb_pow2:
	push rsi
	push rdi
	push rbx
	test r8b, r8b
	js .LBB245_1
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
.LBB245_1:
	xor eax, eax
	xor edx, edx
	pop rbx
	pop rdi
	pop rsi
```
## `unbounded_floor_to_multiple_i8_unb_pow2`
```asm
unbounded_floor_to_multiple_i8_unb_pow2:
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
unbounded_floor_to_multiple_i16_unb_pow2:
	cmp dl, 16
	jae .LBB251_1
	mov eax, edx
	movzx edx, cx
	mov ecx, eax
	shr edx, cl
	shl edx, cl
	mov ax, 1
	ret
.LBB251_1:
	not ecx
	movzx eax, cx
	shr eax, 15
	xor edx, edx
```
## `unbounded_floor_to_multiple_i32_unb_pow2`
```asm
unbounded_floor_to_multiple_i32_unb_pow2:
	mov eax, ecx
	cmp dl, 32
	jae .LBB252_1
	mov ecx, edx
	shr eax, cl
	shl eax, cl
	mov edx, eax
	mov eax, 1
	ret
.LBB252_1:
	not eax
	shr eax, 31
	xor edx, edx
```
## `unbounded_floor_to_multiple_i64_unb_pow2`
```asm
unbounded_floor_to_multiple_i64_unb_pow2:
	mov rax, rcx
	cmp dl, 64
	jae .LBB253_1
	mov ecx, edx
	shr rax, cl
	shl rax, cl
	mov rdx, rax
	mov eax, 1
	ret
.LBB253_1:
	not rax
	shr rax, 63
	xor edx, edx
```
## `unbounded_floor_to_multiple_i128_unb_pow2`
```asm
unbounded_floor_to_multiple_i128_unb_pow2:
	mov rax, rcx
	test r9b, r9b
	js .LBB250_1
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
.LBB250_1:
	xorps xmm0, xmm0
	test r8, r8
	js .LBB250_2
	movups xmmword ptr [rax + 8], xmm0
	mov qword ptr [rax], 1
	mov qword ptr [rax + 24], 0
	ret
.LBB250_2:
	movaps xmmword ptr [rax], xmm0
```
## `unbounded_floor_to_multiple_u8_unb_pow2`
```asm
unbounded_floor_to_multiple_u8_unb_pow2:
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
unbounded_floor_to_multiple_u16_unb_pow2:
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
unbounded_floor_to_multiple_u32_unb_pow2:
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
unbounded_floor_to_multiple_u64_unb_pow2:
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
unbounded_floor_to_multiple_u128_unb_pow2:
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
## `unbounded_is_multiple_of_i8_unb_pow2`
```asm
unbounded_is_multiple_of_i8_unb_pow2:
	test cl, cl
	sete r8b
	movzx eax, cl
	rep bsf	eax, eax
	cmp al, dl
	setae al
	or al, r8b
```
## `unbounded_is_multiple_of_i16_unb_pow2`
```asm
unbounded_is_multiple_of_i16_unb_pow2:
	test cx, cx
	sete r8b
	rep bsf	eax, ecx
	movzx ecx, dl
	cmp ax, cx
	setae al
	or al, r8b
```
## `unbounded_is_multiple_of_i32_unb_pow2`
```asm
unbounded_is_multiple_of_i32_unb_pow2:
	test ecx, ecx
	sete r8b
	mov eax, 32
	rep bsf	eax, ecx
	movzx ecx, dl
	cmp eax, ecx
	setae al
	or al, r8b
```
## `unbounded_is_multiple_of_i64_unb_pow2`
```asm
unbounded_is_multiple_of_i64_unb_pow2:
	test rcx, rcx
	sete r8b
	mov eax, 64
	rep bsf	rax, rcx
	movzx ecx, dl
	cmp eax, ecx
	setae al
	or al, r8b
```
## `unbounded_is_multiple_of_i128_unb_pow2`
```asm
unbounded_is_multiple_of_i128_unb_pow2:
	mov rax, rcx
	or rax, rdx
	je .LBB260_1
	rep bsf	rax, rcx
	rep bsf	rdx, rdx
	add edx, 64
	test rcx, rcx
	cmovne edx, eax
	movzx eax, r8b
	cmp edx, eax
	setae al
	ret
.LBB260_1:
	mov al, 1
```
