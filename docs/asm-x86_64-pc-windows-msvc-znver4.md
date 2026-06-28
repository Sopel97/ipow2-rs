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
	ja .LBB78_1
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
.LBB78_1:
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
	ja .LBB80_1
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
.LBB80_1:
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
	ja .LBB82_1
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
.LBB82_1:
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
	jne .LBB75_2
	mov qword ptr [rax + 16], r11
	mov qword ptr [rax + 24], r10
	mov esi, 1
.LBB75_2:
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
	js .LBB76_1
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
	jne .LBB76_4
	mov qword ptr [rax + 16], r11
	mov qword ptr [rax + 24], r10
	mov esi, 1
	jmp .LBB76_4
.LBB76_1:
	xor esi, esi
.LBB76_4:
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
	ja .LBB88_1
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
.LBB88_1:
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
	ja .LBB90_1
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
.LBB90_1:
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
	ja .LBB92_1
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
.LBB92_1:
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
	jne .LBB85_2
	mov qword ptr [rax + 16], r11
	mov qword ptr [rax + 24], r10
	mov esi, 1
.LBB85_2:
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
	js .LBB86_1
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
	jne .LBB86_4
	mov qword ptr [rax + 16], r11
	mov qword ptr [rax + 24], r10
	mov esi, 1
	jmp .LBB86_4
.LBB86_1:
	xor esi, esi
.LBB86_4:
	mov qword ptr [rax], rsi
	mov qword ptr [rax + 8], 0
	pop rbx
	pop rdi
	pop rsi
	pop r14
```
## `checked_rem_floor_i8_unb_pow2`
```asm
checked_rem_floor_i8_unb_pow2:
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
## `checked_rem_floor_i16_unb_pow2`
```asm
checked_rem_floor_i16_unb_pow2:
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
## `checked_rem_floor_i32_unb_pow2`
```asm
checked_rem_floor_i32_unb_pow2:
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
## `checked_rem_floor_i64_unb_pow2`
```asm
checked_rem_floor_i64_unb_pow2:
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
checked_rem_floor_i128_unb_pow2:
	mov rax, rcx
	test r9b, r9b
	js .LBB95_1
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
.LBB95_1:
	xor ecx, ecx
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
```
## `checked_rem_i8_unb_pow2`
```asm
checked_rem_i8_unb_pow2:
	mov r8d, ecx
	cmp dl, 7
	ja .LBB104_3
	mov al, -1
	mov ecx, edx
	shl al, cl
	mov ecx, eax
	not cl
	test r8b, r8b
	js .LBB104_2
	and r8b, cl
.LBB104_3:
	cmp dl, 8
	setb al
	mov edx, r8d
	ret
.LBB104_2:
	add cl, r8b
	and cl, al
	sub r8b, cl
	cmp dl, 8
	setb al
	mov edx, r8d
```
## `checked_rem_i16_unb_pow2`
```asm
checked_rem_i16_unb_pow2:
	cmp dl, 15
	ja .LBB101_1
	mov r8d, ecx
	mov eax, -1
	mov ecx, edx
	shl eax, cl
	mov ecx, eax
	not ecx
	test r8w, r8w
	js .LBB101_4
	and r8d, ecx
	mov ax, 1
	mov edx, r8d
	ret
.LBB101_1:
	xor eax, eax
	mov edx, r8d
	ret
.LBB101_4:
	add ecx, r8d
	and eax, ecx
	sub r8d, eax
	mov ax, 1
	mov edx, r8d
```
## `checked_rem_i32_unb_pow2`
```asm
checked_rem_i32_unb_pow2:
	cmp dl, 31
	ja .LBB102_1
	mov r8d, ecx
	mov eax, -1
	mov ecx, edx
	shl eax, cl
	mov ecx, eax
	not ecx
	test r8d, r8d
	js .LBB102_4
	and r8d, ecx
	mov eax, 1
	mov edx, r8d
	ret
.LBB102_1:
	xor eax, eax
	mov edx, r8d
	ret
.LBB102_4:
	add ecx, r8d
	and ecx, eax
	sub r8d, ecx
	mov eax, 1
	mov edx, r8d
```
## `checked_rem_i64_unb_pow2`
```asm
checked_rem_i64_unb_pow2:
	cmp dl, 63
	ja .LBB103_1
	mov r8, rcx
	mov rax, -1
	mov ecx, edx
	shl rax, cl
	mov rcx, rax
	not rcx
	test r8, r8
	js .LBB103_4
	and r8, rcx
	mov eax, 1
	mov rdx, r8
	ret
.LBB103_1:
	xor eax, eax
	mov rdx, r8
	ret
.LBB103_4:
	add rcx, r8
	and rcx, rax
	sub r8, rcx
	mov eax, 1
	mov rdx, r8
```
## `checked_rem_i128_unb_pow2`
```asm
checked_rem_i128_unb_pow2:
	push rsi
	push rdi
	mov rax, rcx
	test r9b, r9b
	js .LBB100_1
	mov r10, -1
	mov ecx, r9d
	shl r10, cl
	mov rcx, -1
	xor r11d, r11d
	test r9b, 64
	cmove r11, r10
	cmove r10, rcx
	mov rcx, r10
	not rcx
	mov r9, r11
	not r9
	mov rsi, rdx
	and rsi, r9
	mov rdi, r8
	and rdi, rcx
	add r9, rdx
	adc rcx, r8
	and rcx, r10
	and r9, r11
	sub rdx, r9
	mov r9, r8
	sbb r9, rcx
	test r8, r8
	cmovns r9, rdi
	cmovns rdx, rsi
	mov qword ptr [rax + 16], rdx
	mov qword ptr [rax + 24], r9
	mov ecx, 1
	jmp .LBB100_3
.LBB100_1:
	xor ecx, ecx
.LBB100_3:
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	pop rdi
	pop rsi
```
## `checked_rem_u16_unb_pow2`
```asm
checked_rem_u16_unb_pow2:
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
## `checked_rem_u32_unb_pow2`
```asm
checked_rem_u32_unb_pow2:
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
## `checked_rem_u64_unb_pow2`
```asm
checked_rem_u64_unb_pow2:
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
## `checked_round_to_multiple_i8_unb_pow2`
```asm
checked_round_to_multiple_i8_unb_pow2:
	xor r10d, r10d
	cmp dl, 7
	ja .LBB112_1
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
	jo .LBB112_1
	neg r9b
	and dl, r9b
	ret
.LBB112_1:
	xor eax, eax
```
## `checked_round_to_multiple_i16_unb_pow2`
```asm
checked_round_to_multiple_i16_unb_pow2:
	xor eax, eax
	cmp dl, 15
	ja .LBB109_4
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
	jo .LBB109_4
	neg edx
	and edx, ecx
	mov ax, 1
	ret
.LBB109_4:
```
## `checked_round_to_multiple_i32_unb_pow2`
```asm
checked_round_to_multiple_i32_unb_pow2:
	cmp dl, 31
	ja .LBB110_3
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
	jo .LBB110_4
	neg r9d
	and edx, r9d
	mov eax, 1
	ret
.LBB110_3:
	xor eax, eax
.LBB110_4:
```
## `checked_round_to_multiple_i64_unb_pow2`
```asm
checked_round_to_multiple_i64_unb_pow2:
	cmp dl, 63
	ja .LBB111_3
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
	jo .LBB111_4
	neg r9
	and rdx, r9
	mov eax, 1
	ret
.LBB111_3:
	xor eax, eax
.LBB111_4:
```
## `checked_round_to_multiple_i128_unb_pow2`
```asm
checked_round_to_multiple_i128_unb_pow2:
	push rsi
	push rdi
	push rbx
	mov rax, rcx
	test r9b, r9b
	js .LBB108_3
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
	jo .LBB108_3
	xor r8d, r8d
	neg r11
	sbb r8, rsi
	and rcx, r8
	and rdx, r11
	mov qword ptr [rax + 16], rdx
	mov qword ptr [rax + 24], rcx
	jmp .LBB108_4
.LBB108_3:
	xor r10d, r10d
.LBB108_4:
	mov qword ptr [rax], r10
	mov qword ptr [rax + 8], 0
	pop rbx
	pop rdi
	pop rsi
```
## `checked_round_to_multiple_u8_unb_pow2`
```asm
checked_round_to_multiple_u8_unb_pow2:
	cmp dl, 7
	ja .LBB117_2
	mov r8d, ecx
	mov r9b, 1
	mov ecx, edx
	shl r9b, cl
	mov al, 1
	mov edx, r9d
	shr dl
	add dl, r8b
	jb .LBB117_2
	neg r9b
	and dl, r9b
	ret
.LBB117_2:
	xor eax, eax
```
## `checked_round_to_multiple_u16_unb_pow2`
```asm
checked_round_to_multiple_u16_unb_pow2:
	cmp dl, 15
	ja .LBB114_2
	mov eax, ecx
	mov r8d, 1
	mov ecx, edx
	shl r8d, cl
	movzx edx, r8w
	shr edx
	add dx, ax
	jb .LBB114_2
	neg r8d
	and edx, r8d
	mov ax, 1
	ret
.LBB114_2:
	xor eax, eax
```
## `checked_round_to_multiple_u32_unb_pow2`
```asm
checked_round_to_multiple_u32_unb_pow2:
	xor eax, eax
	cmp dl, 31
	ja .LBB115_2
	mov r8d, ecx
	mov r9d, 1
	mov ecx, edx
	shl r9d, cl
	mov edx, r9d
	shr edx
	add edx, r8d
	jb .LBB115_2
	neg r9d
	and edx, r9d
	mov eax, 1
	ret
.LBB115_2:
```
## `checked_round_to_multiple_u64_unb_pow2`
```asm
checked_round_to_multiple_u64_unb_pow2:
	cmp dl, 63
	ja .LBB116_2
	mov r8, rcx
	mov r9d, 1
	mov ecx, edx
	shl r9, cl
	mov eax, 1
	mov rdx, r9
	shr rdx
	add rdx, r8
	jb .LBB116_2
	neg r9
	and rdx, r9
	ret
.LBB116_2:
	xor eax, eax
```
## `checked_round_to_multiple_u128_unb_pow2`
```asm
checked_round_to_multiple_u128_unb_pow2:
	push rsi
	push rdi
	mov rax, rcx
	test r9b, r9b
	js .LBB113_2
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
	jb .LBB113_2
	xor edx, edx
	neg rsi
	sbb rdx, r11
	and rcx, rdx
	and r9, rsi
	mov qword ptr [rax + 16], r9
	mov qword ptr [rax + 24], rcx
	jmp .LBB113_3
.LBB113_2:
	xor r10d, r10d
.LBB113_3:
	mov qword ptr [rax], r10
	mov qword ptr [rax + 8], 0
	pop rdi
	pop rsi
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
	js .LBB166_1
	mov ecx, edx
	shr al, cl
	ret
.LBB166_1:
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
	js .LBB167_1
	mov ecx, edx
	shr al, cl
	ret
.LBB167_1:
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
	js .LBB160_1
	movzx eax, ax
	shr eax, cl
	ret
.LBB160_1:
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
	js .LBB161_1
	movzx eax, ax
	shr eax, cl
	ret
.LBB161_1:
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
	js .LBB162_1
	mov ecx, edx
	shr eax, cl
	ret
.LBB162_1:
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
	js .LBB163_1
	mov ecx, edx
	shr eax, cl
	ret
.LBB163_1:
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
	js .LBB164_1
	shr rax, cl
	ret
.LBB164_1:
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
	js .LBB165_1
	shr rax, cl
	ret
.LBB165_1:
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
	js .LBB158_1
	mov ecx, r8d
	shrd rax, rdx, cl
	shr rdx, cl
	xor ecx, ecx
	test r8b, 64
	cmovne rax, rdx
	cmovne rdx, rcx
	ret
.LBB158_1:
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
	js .LBB159_1
	mov ecx, r8d
	shrd rax, rdx, cl
	shr rdx, cl
	xor ecx, ecx
	test r8b, 64
	cmovne rax, rdx
	cmovne rdx, rcx
	ret
.LBB159_1:
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
## `rem_floor_i8_pow2`
```asm
rem_floor_i8_pow2:
	mov r8d, ecx
	mov al, -1
	mov ecx, edx
	shl al, cl
	not al
	and al, r8b
```
## `rem_floor_i8_unb_pow2`
```asm
rem_floor_i8_unb_pow2:
	mov r8d, ecx
	and dl, 7
	mov al, -1
	mov ecx, edx
	shl al, cl
	not al
	and al, r8b
```
## `rem_floor_i16_pow2`
```asm
rem_floor_i16_pow2:
	mov r8d, ecx
	mov eax, -1
	mov ecx, edx
	shl eax, cl
	not eax
	and eax, r8d
```
## `rem_floor_i16_unb_pow2`
```asm
rem_floor_i16_unb_pow2:
	mov r8d, ecx
	and dl, 15
	mov eax, -1
	mov ecx, edx
	shl eax, cl
	not eax
	and eax, r8d
```
## `rem_floor_i32_pow2`
```asm
rem_floor_i32_pow2:
	mov r8d, ecx
	mov eax, -1
	mov ecx, edx
	shl eax, cl
	not eax
	and eax, r8d
```
## `rem_floor_i32_unb_pow2`
```asm
rem_floor_i32_unb_pow2:
	mov r8d, ecx
	mov eax, -1
	mov ecx, edx
	shl eax, cl
	not eax
	and eax, r8d
```
## `rem_floor_i64_pow2`
```asm
rem_floor_i64_pow2:
	mov r8, rcx
	mov rax, -1
	mov ecx, edx
	shl rax, cl
	not rax
	and rax, r8
```
## `rem_floor_i64_unb_pow2`
```asm
rem_floor_i64_unb_pow2:
	mov r8, rcx
	mov rax, -1
	mov ecx, edx
	shl rax, cl
	not rax
	and rax, r8
```
## `rem_floor_i128_pow2`
```asm
rem_floor_i128_pow2:
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
## `rem_floor_i128_unb_pow2`
```asm
rem_floor_i128_unb_pow2:
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
rem_i8_pow2:
	mov eax, ecx
	mov r8b, -1
	mov ecx, edx
	shl r8b, cl
	mov ecx, r8d
	not cl
	mov edx, eax
	and dl, cl
	add cl, al
	and cl, r8b
	mov r8d, eax
	sub r8b, cl
	test al, al
	movzx ecx, r8b
	movzx eax, dl
	cmovs eax, ecx
```
## `rem_i8_unb_pow2`
```asm
rem_i8_unb_pow2:
	mov eax, ecx
	and dl, 7
	mov r8b, -1
	mov ecx, edx
	shl r8b, cl
	mov ecx, r8d
	not cl
	mov edx, eax
	and dl, cl
	add cl, al
	and cl, r8b
	mov r8d, eax
	sub r8b, cl
	test al, al
	movzx ecx, r8b
	movzx eax, dl
	cmovs eax, ecx
```
## `rem_i16_pow2`
```asm
rem_i16_pow2:
	mov r8d, ecx
	mov eax, -1
	mov ecx, edx
	shl eax, cl
	mov ecx, eax
	not ecx
	mov edx, r8d
	and edx, ecx
	add ecx, r8d
	and ecx, eax
	mov eax, r8d
	sub eax, ecx
	test r8w, r8w
	cmovns eax, edx
```
## `rem_i16_unb_pow2`
```asm
rem_i16_unb_pow2:
	mov r8d, ecx
	and dl, 15
	mov eax, -1
	mov ecx, edx
	shl eax, cl
	mov ecx, eax
	not ecx
	mov edx, r8d
	and edx, ecx
	add ecx, r8d
	and ecx, eax
	mov eax, r8d
	sub eax, ecx
	test r8w, r8w
	cmovns eax, edx
```
## `rem_i32_pow2`
```asm
rem_i32_pow2:
	mov r8d, ecx
	mov eax, -1
	mov ecx, edx
	shl eax, cl
	mov ecx, eax
	not ecx
	mov edx, r8d
	and edx, ecx
	add ecx, r8d
	and ecx, eax
	mov eax, r8d
	sub eax, ecx
	test r8d, r8d
	cmovns eax, edx
```
## `rem_i32_unb_pow2`
```asm
rem_i32_unb_pow2:
	mov r8d, ecx
	mov eax, -1
	mov ecx, edx
	shl eax, cl
	mov ecx, eax
	not ecx
	mov edx, r8d
	and edx, ecx
	add ecx, r8d
	and ecx, eax
	mov eax, r8d
	sub eax, ecx
	test r8d, r8d
	cmovns eax, edx
```
## `rem_i64_pow2`
```asm
rem_i64_pow2:
	mov r8, rcx
	mov rax, -1
	mov ecx, edx
	shl rax, cl
	mov rcx, rax
	not rcx
	mov rdx, r8
	and rdx, rcx
	add rcx, r8
	and rcx, rax
	mov rax, r8
	sub rax, rcx
	test r8, r8
	cmovns rax, rdx
```
## `rem_i64_unb_pow2`
```asm
rem_i64_unb_pow2:
	mov r8, rcx
	mov rax, -1
	mov ecx, edx
	shl rax, cl
	mov rcx, rax
	not rcx
	mov rdx, r8
	and rdx, rcx
	add rcx, r8
	and rcx, rax
	mov rax, r8
	sub rax, rcx
	test r8, r8
	cmovns rax, rdx
```
## `rem_i128_pow2`
```asm
rem_i128_pow2:
	push rsi
	mov rax, rcx
	mov r9, -1
	mov ecx, r8d
	shl r9, cl
	xor ecx, ecx
	test r8b, 64
	cmove rcx, r9
	mov r8, -1
	cmove r9, r8
	mov r8, r9
	not r8
	mov r10, rcx
	not r10
	mov r11, rax
	and r11, r10
	mov rsi, rdx
	and rsi, r8
	add r10, rax
	adc r8, rdx
	and r8, r9
	and r10, rcx
	sub rax, r10
	mov rcx, rdx
	sbb rcx, r8
	test rdx, rdx
	cmovns rcx, rsi
	cmovns rax, r11
	mov rdx, rcx
	pop rsi
```
## `rem_i128_unb_pow2`
```asm
rem_i128_unb_pow2:
	push rsi
	mov rax, rcx
	mov r9, -1
	mov ecx, r8d
	shl r9, cl
	xor ecx, ecx
	test r8b, 64
	cmove rcx, r9
	mov r8, -1
	cmove r9, r8
	mov r8, r9
	not r8
	mov r10, rcx
	not r10
	mov r11, rax
	and r11, r10
	mov rsi, rdx
	and rsi, r8
	add r10, rax
	adc r8, rdx
	and r8, r9
	and r10, rcx
	sub rax, r10
	mov rcx, rdx
	sbb rcx, r8
	test rdx, rdx
	cmovns rcx, rsi
	cmovns rax, r11
	mov rdx, rcx
	pop rsi
```
## `round_to_multiple_i8_pow2`
```asm
round_to_multiple_i8_pow2:
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
round_to_multiple_i8_unb_pow2:
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
round_to_multiple_i16_pow2:
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
round_to_multiple_i16_unb_pow2:
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
## `round_to_multiple_i32_pow2`
```asm
round_to_multiple_i32_pow2:
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
## `round_to_multiple_i32_unb_pow2`
```asm
round_to_multiple_i32_unb_pow2:
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
## `round_to_multiple_i64_pow2`
```asm
round_to_multiple_i64_pow2:
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
## `round_to_multiple_i64_unb_pow2`
```asm
round_to_multiple_i64_unb_pow2:
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
## `round_to_multiple_i128_pow2`
```asm
round_to_multiple_i128_pow2:
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
## `round_to_multiple_i128_unb_pow2`
```asm
round_to_multiple_i128_unb_pow2:
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
round_to_multiple_u8_pow2:
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
round_to_multiple_u8_unb_pow2:
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
round_to_multiple_u16_pow2:
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
round_to_multiple_u16_unb_pow2:
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
## `round_to_multiple_u32_pow2`
```asm
round_to_multiple_u32_pow2:
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
## `round_to_multiple_u32_unb_pow2`
```asm
round_to_multiple_u32_unb_pow2:
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
## `round_to_multiple_u64_pow2`
```asm
round_to_multiple_u64_pow2:
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
## `round_to_multiple_u64_unb_pow2`
```asm
round_to_multiple_u64_unb_pow2:
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
## `round_to_multiple_u128_pow2`
```asm
round_to_multiple_u128_pow2:
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
## `round_to_multiple_u128_unb_pow2`
```asm
round_to_multiple_u128_unb_pow2:
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
unbounded_ceil_to_multiple_i8_unb_pow2:
	mov r8d, ecx
	cmp dl, 8
	jae .LBB257_1
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
.LBB257_1:
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
	jae .LBB254_1
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
.LBB254_1:
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
	jae .LBB255_1
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
.LBB255_1:
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
	jae .LBB256_1
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
.LBB256_1:
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
	js .LBB253_4
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
	jo .LBB253_5
	and rdx, r9
	and r8, r10
	mov rcx, rdx
.LBB253_3:
	mov qword ptr [rax + 16], rcx
	mov qword ptr [rax + 24], r8
	mov ecx, 1
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	pop rsi
	ret
.LBB253_4:
	xor ecx, ecx
	cmp rdx, 1
	sbb r8, 0
	mov r8d, 0
	jl .LBB253_3
.LBB253_5:
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	pop rsi
```
## `unbounded_ceil_to_multiple_u8_unb_pow2`
```asm
unbounded_ceil_to_multiple_u8_unb_pow2:
	mov eax, ecx
	cmp dl, 8
	jae .LBB262_1
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
.LBB262_1:
	test al, al
	sete al
	xor edx, edx
```
## `unbounded_ceil_to_multiple_u16_unb_pow2`
```asm
unbounded_ceil_to_multiple_u16_unb_pow2:
	mov eax, ecx
	cmp dl, 16
	jae .LBB259_1
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
.LBB259_1:
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
	jae .LBB260_1
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
.LBB260_1:
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
	jae .LBB261_1
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
.LBB261_1:
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
	js .LBB258_4
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
	jb .LBB258_5
	and rdx, r9
	and r8, r10
	mov rcx, rdx
.LBB258_3:
	mov qword ptr [rax + 16], rcx
	mov qword ptr [rax + 24], r8
	mov ecx, 1
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	pop rsi
	ret
.LBB258_4:
	xor ecx, ecx
	or rdx, r8
	mov r8d, 0
	je .LBB258_3
.LBB258_5:
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	pop rsi
```
## `unbounded_div_ceil_i8_unb_pow2`
```asm
unbounded_div_ceil_i8_unb_pow2:
	mov r8d, ecx
	cmp dl, 8
	jae .LBB267_1
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
.LBB267_1:
	test r8b, r8b
	setg al
```
## `unbounded_div_ceil_i16_unb_pow2`
```asm
unbounded_div_ceil_i16_unb_pow2:
	mov r8d, edx
	mov edx, ecx
	cmp r8b, 16
	jae .LBB264_1
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
.LBB264_1:
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
	jae .LBB265_1
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
.LBB265_1:
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
	jae .LBB266_1
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
.LBB266_1:
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
	js .LBB263_1
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
.LBB263_1:
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
	jae .LBB272_1
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
.LBB272_1:
	test r8b, r8b
	setne al
```
## `unbounded_div_ceil_u16_unb_pow2`
```asm
unbounded_div_ceil_u16_unb_pow2:
	mov r8d, edx
	mov edx, ecx
	cmp r8b, 16
	jae .LBB269_1
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
.LBB269_1:
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
	jae .LBB270_1
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
.LBB270_1:
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
	jae .LBB271_1
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
.LBB271_1:
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
	js .LBB268_1
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
.LBB268_1:
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
	ja .LBB287_1
	mov eax, ecx
	test cl, cl
	js .LBB287_4
	mov ecx, edx
	shr al, cl
	ret
.LBB287_1:
	xor eax, eax
	ret
.LBB287_4:
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
	ja .LBB284_1
	mov eax, ecx
	movzx ecx, dl
	test ax, ax
	js .LBB284_4
	movzx eax, ax
	shr eax, cl
	ret
.LBB284_1:
	xor eax, eax
	ret
.LBB284_4:
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
	ja .LBB285_4
	mov r8d, ecx
	test ecx, ecx
	js .LBB285_2
	mov ecx, edx
	shr r8d, cl
	jmp .LBB285_3
.LBB285_2:
	mov eax, -1
	mov ecx, edx
	shl eax, cl
	not eax
	add r8d, eax
	sar r8d, cl
.LBB285_3:
	mov eax, r8d
.LBB285_4:
```
## `unbounded_div_i64_unb_pow2`
```asm
unbounded_div_i64_unb_pow2:
	cmp dl, 63
	ja .LBB286_1
	mov rax, rcx
	movzx ecx, dl
	test rax, rax
	js .LBB286_4
	shr rax, cl
	ret
.LBB286_1:
	xor eax, eax
	ret
.LBB286_4:
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
	js .LBB283_1
	mov rax, rcx
	test rdx, rdx
	js .LBB283_4
	mov ecx, r8d
	shrd rax, rdx, cl
	shr rdx, cl
	xor ecx, ecx
	test r8b, 64
	cmovne rax, rdx
	cmovne rdx, rcx
	ret
.LBB283_1:
	xor eax, eax
	xor edx, edx
	ret
.LBB283_4:
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
	ja .LBB292_2
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
.LBB292_2:
	neg r8b
	jno .LBB292_3
	cmp dl, 8
	sete al
	neg al
	ret
.LBB292_3:
	xor eax, eax
```
## `unbounded_div_round_i16_unb_pow2`
```asm
unbounded_div_round_i16_unb_pow2:
	mov r8d, ecx
	cmp dl, 15
	ja .LBB289_2
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
.LBB289_2:
	xor eax, eax
	neg r8w
	jno .LBB289_4
	cmp dl, 16
	sete al
	neg eax
.LBB289_4:
```
## `unbounded_div_round_i32_unb_pow2`
```asm
unbounded_div_round_i32_unb_pow2:
	mov r8d, ecx
	cmp dl, 31
	ja .LBB290_2
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
.LBB290_2:
	xor eax, eax
	neg r8d
	jno .LBB290_4
	xor eax, eax
	cmp dl, 32
	sete al
	neg eax
.LBB290_4:
```
## `unbounded_div_round_i64_unb_pow2`
```asm
unbounded_div_round_i64_unb_pow2:
	mov r8, rcx
	cmp dl, 63
	ja .LBB291_2
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
.LBB291_2:
	xor eax, eax
	neg r8
	jno .LBB291_4
	cmp dl, 64
	sete al
	neg rax
.LBB291_4:
```
## `unbounded_div_round_i128_unb_pow2`
```asm
unbounded_div_round_i128_unb_pow2:
	mov r9, rcx
	test r8b, r8b
	js .LBB288_2
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
.LBB288_2:
	movabs rax, -9223372036854775808
	xor rdx, rax
	xor eax, eax
	or r9, rdx
	mov edx, 0
	jne .LBB288_4
	xor eax, eax
	neg r8b
	seto al
	neg rax
	mov rdx, rax
.LBB288_4:
```
## `unbounded_div_round_u8_unb_pow2`
```asm
unbounded_div_round_u8_unb_pow2:
	cmp dl, 7
	ja .LBB297_1
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
.LBB297_1:
	xor eax, eax
```
## `unbounded_div_round_u16_unb_pow2`
```asm
unbounded_div_round_u16_unb_pow2:
	cmp dl, 15
	ja .LBB294_1
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
.LBB294_1:
	xor eax, eax
```
## `unbounded_div_round_u32_unb_pow2`
```asm
unbounded_div_round_u32_unb_pow2:
	xor eax, eax
	cmp dl, 31
	ja .LBB295_2
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
.LBB295_2:
```
## `unbounded_div_round_u64_unb_pow2`
```asm
unbounded_div_round_u64_unb_pow2:
	cmp dl, 63
	ja .LBB296_1
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
.LBB296_1:
	xor eax, eax
```
## `unbounded_div_round_u128_unb_pow2`
```asm
unbounded_div_round_u128_unb_pow2:
	push rsi
	push rdi
	push rbx
	test r8b, r8b
	js .LBB293_1
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
.LBB293_1:
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
	jae .LBB299_1
	mov eax, edx
	movzx edx, cx
	mov ecx, eax
	shr edx, cl
	shl edx, cl
	mov ax, 1
	ret
.LBB299_1:
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
	jae .LBB300_1
	mov ecx, edx
	shr eax, cl
	shl eax, cl
	mov edx, eax
	mov eax, 1
	ret
.LBB300_1:
	not eax
	shr eax, 31
	xor edx, edx
```
## `unbounded_floor_to_multiple_i64_unb_pow2`
```asm
unbounded_floor_to_multiple_i64_unb_pow2:
	mov rax, rcx
	cmp dl, 64
	jae .LBB301_1
	mov ecx, edx
	shr rax, cl
	shl rax, cl
	mov rdx, rax
	mov eax, 1
	ret
.LBB301_1:
	not rax
	shr rax, 63
	xor edx, edx
```
## `unbounded_floor_to_multiple_i128_unb_pow2`
```asm
unbounded_floor_to_multiple_i128_unb_pow2:
	mov rax, rcx
	test r9b, r9b
	js .LBB298_1
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
.LBB298_1:
	xorps xmm0, xmm0
	test r8, r8
	js .LBB298_2
	movups xmmword ptr [rax + 8], xmm0
	mov qword ptr [rax], 1
	mov qword ptr [rax + 24], 0
	ret
.LBB298_2:
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
	rep bsf	rax, rcx
	movzx ecx, dl
	cmp rax, rcx
	setae al
	or al, r8b
```
## `unbounded_is_multiple_of_i128_unb_pow2`
```asm
unbounded_is_multiple_of_i128_unb_pow2:
	mov rax, rcx
	or rax, rdx
	je .LBB308_1
	rep bsf	rax, rcx
	rep bsf	rdx, rdx
	add edx, 64
	test rcx, rcx
	cmovne edx, eax
	movzx eax, r8b
	cmp edx, eax
	setae al
	ret
.LBB308_1:
	mov al, 1
```
## `unbounded_rem_i8_unb_pow2`
```asm
unbounded_rem_i8_unb_pow2:
	mov eax, ecx
	cmp dl, 7
	ja .LBB317_3
	mov r8b, -1
	mov ecx, edx
	shl r8b, cl
	mov ecx, r8d
	not cl
	test al, al
	js .LBB317_2
	and al, cl
	ret
.LBB317_2:
	add cl, al
	and cl, r8b
	sub al, cl
.LBB317_3:
```
## `unbounded_rem_i16_unb_pow2`
```asm
unbounded_rem_i16_unb_pow2:
	mov eax, ecx
	cmp dl, 15
	ja .LBB314_3
	mov r8d, -1
	mov ecx, edx
	shl r8d, cl
	mov ecx, r8d
	not ecx
	test ax, ax
	js .LBB314_2
	and eax, ecx
	ret
.LBB314_2:
	add ecx, eax
	and r8d, ecx
	sub eax, r8d
.LBB314_3:
```
## `unbounded_rem_i32_unb_pow2`
```asm
unbounded_rem_i32_unb_pow2:
	mov eax, ecx
	cmp dl, 31
	ja .LBB315_3
	mov r8d, -1
	mov ecx, edx
	shl r8d, cl
	mov ecx, r8d
	not ecx
	test eax, eax
	js .LBB315_2
	and eax, ecx
	ret
.LBB315_2:
	add ecx, eax
	and ecx, r8d
	sub eax, ecx
.LBB315_3:
```
## `unbounded_rem_i64_unb_pow2`
```asm
unbounded_rem_i64_unb_pow2:
	mov rax, rcx
	cmp dl, 63
	ja .LBB316_3
	mov r8, -1
	mov ecx, edx
	shl r8, cl
	mov rcx, r8
	not rcx
	test rax, rax
	js .LBB316_2
	and rax, rcx
	ret
.LBB316_2:
	add rcx, rax
	and rcx, r8
	sub rax, rcx
.LBB316_3:
```
## `unbounded_rem_i128_unb_pow2`
```asm
unbounded_rem_i128_unb_pow2:
	mov rax, rcx
	test r8b, r8b
	js .LBB313_3
	mov r9, -1
	mov ecx, r8d
	shl r9, cl
	xor ecx, ecx
	test r8b, 64
	cmove rcx, r9
	mov r8, -1
	cmove r9, r8
	mov r8, r9
	not r8
	mov r10, rcx
	not r10
	test rdx, rdx
	js .LBB313_2
	and rax, r10
	and rdx, r8
	ret
.LBB313_2:
	add r10, rax
	adc r8, rdx
	and r8, r9
	and r10, rcx
	sub rax, r10
	sbb rdx, r8
.LBB313_3:
```
## `unbounded_rem_u8_unb_pow2`
```asm
unbounded_rem_u8_unb_pow2:
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
unbounded_rem_u16_unb_pow2:
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
unbounded_rem_u32_unb_pow2:
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
unbounded_rem_u64_unb_pow2:
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
unbounded_rem_u128_unb_pow2:
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
unbounded_round_to_multiple_i8_unb_pow2:
	mov eax, ecx
	cmp dl, 7
	ja .LBB327_5
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
	jo .LBB327_2
	neg r8b
	and dl, r8b
	xor al, 1
	ret
.LBB327_5:
	add al, -128
	xor dl, 8
	or dl, al
	setne al
	xor edx, edx
	ret
.LBB327_2:
	xor al, 1
```
## `unbounded_round_to_multiple_i16_unb_pow2`
```asm
unbounded_round_to_multiple_i16_unb_pow2:
	mov r8d, edx
	mov eax, ecx
	cmp dl, 15
	ja .LBB324_3
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
	jo .LBB324_4
	neg edx
	and edx, ecx
	xor al, 1
	movzx eax, al
	ret
.LBB324_3:
	neg ax
	setno cl
	cmp r8b, 16
	setne al
	or al, cl
	xor edx, edx
	movzx eax, al
	ret
.LBB324_4:
	xor al, 1
	movzx eax, al
```
## `unbounded_round_to_multiple_i32_unb_pow2`
```asm
unbounded_round_to_multiple_i32_unb_pow2:
	mov eax, ecx
	cmp dl, 31
	ja .LBB325_5
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
	jo .LBB325_2
	neg r8d
	and edx, r8d
	xor al, 1
	movzx eax, al
	ret
.LBB325_5:
	neg eax
	setno cl
	cmp dl, 32
	setne al
	or al, cl
	xor edx, edx
	movzx eax, al
	ret
.LBB325_2:
	xor al, 1
	movzx eax, al
```
## `unbounded_round_to_multiple_i64_unb_pow2`
```asm
unbounded_round_to_multiple_i64_unb_pow2:
	mov rax, rcx
	cmp dl, 63
	ja .LBB326_5
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
	jo .LBB326_2
	neg r8
	and rdx, r8
	xor al, 1
	movzx eax, al
	ret
.LBB326_5:
	neg rax
	setno cl
	cmp dl, 64
	setne al
	or al, cl
	xor edx, edx
	movzx eax, al
	ret
.LBB326_2:
	xor al, 1
	movzx eax, al
```
## `unbounded_round_to_multiple_i128_unb_pow2`
```asm
unbounded_round_to_multiple_i128_unb_pow2:
	push rsi
	push rdi
	mov rax, rcx
	test r9b, r9b
	js .LBB323_3
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
	jo .LBB323_5
	xor ecx, ecx
	neg r10
	sbb rcx, r11
	and r9, rcx
	and rdx, r10
	mov rcx, rdx
	jmp .LBB323_4
.LBB323_3:
	movabs rcx, -9223372036854775808
	xor r8, rcx
	or rdx, r8
	sete dl
	neg r9b
	seto r8b
	xor ecx, ecx
	mov r9d, 0
	test dl, r8b
	jne .LBB323_5
.LBB323_4:
	mov qword ptr [rax + 16], rcx
	mov qword ptr [rax + 24], r9
	mov ecx, 1
.LBB323_5:
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	pop rdi
	pop rsi
```
## `unbounded_round_to_multiple_u8_unb_pow2`
```asm
unbounded_round_to_multiple_u8_unb_pow2:
	mov eax, ecx
	cmp dl, 7
	ja .LBB332_4
	mov r8b, 1
	mov ecx, edx
	shl r8b, cl
	mov edx, r8d
	shr dl
	add dl, al
	cmp dl, al
	setae al
	jb .LBB332_3
	neg r8b
	and dl, r8b
	ret
.LBB332_4:
	test al, al
	setns cl
	cmp dl, 8
	setne al
	or al, cl
	xor edx, edx
	ret
.LBB332_3:
```
## `unbounded_round_to_multiple_u16_unb_pow2`
```asm
unbounded_round_to_multiple_u16_unb_pow2:
	mov eax, ecx
	cmp dl, 15
	ja .LBB329_4
	mov r8d, 1
	mov ecx, edx
	shl r8d, cl
	movzx ecx, r8w
	shr ecx
	lea edx, [rax + rcx]
	cmp dx, ax
	setae al
	jb .LBB329_3
	neg r8d
	and edx, r8d
	movzx eax, al
	ret
.LBB329_4:
	test ax, ax
	setns cl
	cmp dl, 16
	setne al
	or al, cl
	xor edx, edx
	movzx eax, al
	ret
.LBB329_3:
	movzx eax, al
```
## `unbounded_round_to_multiple_u32_unb_pow2`
```asm
unbounded_round_to_multiple_u32_unb_pow2:
	mov eax, ecx
	cmp dl, 31
	ja .LBB330_4
	mov r8d, 1
	mov ecx, edx
	shl r8d, cl
	mov edx, r8d
	shr edx
	add edx, eax
	cmp edx, eax
	setae al
	jb .LBB330_3
	neg r8d
	and edx, r8d
	movzx eax, al
	ret
.LBB330_4:
	test eax, eax
	setns cl
	cmp dl, 32
	setne al
	or al, cl
	xor edx, edx
	movzx eax, al
	ret
.LBB330_3:
	movzx eax, al
```
## `unbounded_round_to_multiple_u64_unb_pow2`
```asm
unbounded_round_to_multiple_u64_unb_pow2:
	mov rax, rcx
	cmp dl, 63
	ja .LBB331_4
	mov r8d, 1
	mov ecx, edx
	shl r8, cl
	mov rdx, r8
	shr rdx
	add rdx, rax
	cmp rdx, rax
	setae al
	jb .LBB331_3
	neg r8
	and rdx, r8
	movzx eax, al
	ret
.LBB331_4:
	test rax, rax
	setns cl
	cmp dl, 64
	setne al
	or al, cl
	xor edx, edx
	movzx eax, al
	ret
.LBB331_3:
	movzx eax, al
```
## `unbounded_round_to_multiple_u128_unb_pow2`
```asm
unbounded_round_to_multiple_u128_unb_pow2:
	push rsi
	mov rax, rcx
	test r9b, r9b
	js .LBB328_3
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
	jb .LBB328_5
	xor edx, edx
	neg r11
	sbb rdx, rsi
	and rcx, rdx
	and r9, r11
	mov r10, r9
	jmp .LBB328_4
.LBB328_3:
	test r8, r8
	sets dl
	neg r9b
	seto r8b
	xor r10d, r10d
	mov ecx, 0
	test dl, r8b
	jne .LBB328_5
.LBB328_4:
	mov qword ptr [rax + 16], r10
	mov qword ptr [rax + 24], rcx
	mov r10d, 1
.LBB328_5:
	mov qword ptr [rax], r10
	mov qword ptr [rax + 8], 0
	pop rsi
```
