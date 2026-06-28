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
	and dl, 7
	mov r8d, ecx
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
	mov eax, -1
	shlx eax, eax, edx
	mov edx, eax
	sub edx, ecx
	andn eax, edx, eax
```
## `ceil_to_multiple_i16_unb_pow2`, `ceil_to_multiple_u16_unb_pow2`
```asm
	and dl, 15
	mov eax, -1
	shlx eax, eax, edx
	mov edx, eax
	sub edx, ecx
	andn eax, edx, eax
```
## `ceil_to_multiple_i64_pow2`, `ceil_to_multiple_i64_unb_pow2`, `ceil_to_multiple_u64_pow2`, `ceil_to_multiple_u64_unb_pow2`
```asm
	mov rax, -1
	shlx rax, rax, rdx
	mov rdx, rax
	sub rdx, rcx
	andn rax, rdx, rax
```
## `ceil_to_multiple_i128_pow2`, `ceil_to_multiple_i128_unb_pow2`, `ceil_to_multiple_u128_pow2`, `ceil_to_multiple_u128_unb_pow2`
```asm
	mov r9, -1
	xor r10d, r10d
	test r8b, 64
	shlx rax, r9, r8
	cmove r10, rax
	cmovne r9, rax
	mov rax, r10
	not rax
	mov r8, r9
	not r8
	add rax, rcx
	adc rdx, r8
	and rax, r10
	and rdx, r9
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
	mov edx, r8d
	xor eax, eax
```
## `checked_ceil_to_multiple_i16_pow2`
```asm
	mov eax, -1
	shlx r8d, eax, edx
	xor eax, eax
	mov edx, r8d
	not edx
	add dx, cx
	setno al
	and edx, r8d
```
## `checked_ceil_to_multiple_i16_unb_pow2`
```asm
	cmp dl, 15
	ja .LBB28_1
	mov eax, -1
	shlx edx, eax, edx
	xor eax, eax
	mov r8d, edx
	not r8d
	add cx, r8w
	setno al
	and ecx, edx
	mov edx, ecx
	ret
.LBB28_1:
	mov edx, ecx
	xor eax, eax
```
## `checked_ceil_to_multiple_i32_pow2`
```asm
	mov eax, -1
	shlx r8d, eax, edx
	xor eax, eax
	mov edx, r8d
	not edx
	add edx, ecx
	setno al
	and edx, r8d
```
## `checked_ceil_to_multiple_i32_unb_pow2`
```asm
	cmp dl, 31
	ja .LBB30_1
	mov eax, -1
	shlx edx, eax, edx
	xor eax, eax
	mov r8d, edx
	not r8d
	add ecx, r8d
	setno al
	and ecx, edx
	mov edx, ecx
	ret
.LBB30_1:
	xor eax, eax
	mov edx, ecx
```
## `checked_ceil_to_multiple_i64_pow2`
```asm
	mov rax, -1
	shlx r8, rax, rdx
	xor eax, eax
	mov rdx, r8
	not rdx
	add rdx, rcx
	setno al
	and rdx, r8
```
## `checked_ceil_to_multiple_i64_unb_pow2`
```asm
	cmp dl, 63
	ja .LBB32_1
	mov rax, -1
	shlx rdx, rax, rdx
	xor eax, eax
	mov r8, rdx
	not r8
	add rcx, r8
	setno al
	and rcx, rdx
	mov rdx, rcx
	ret
.LBB32_1:
	xor eax, eax
	mov rdx, rcx
```
## `checked_ceil_to_multiple_i128_pow2`
```asm
	push rsi
	mov r10, -1
	mov rax, rcx
	xor ecx, ecx
	test r9b, 64
	shlx r11, r10, r9
	mov r9, r11
	cmovne r9, rcx
	cmovne r10, r11
	mov rsi, r9
	not rsi
	mov r11, r10
	not r11
	add rsi, rdx
	adc r11, r8
	jo .LBB25_2
	and r11, r10
	and rsi, r9
	mov ecx, 1
	mov qword ptr [rax + 16], rsi
	mov qword ptr [rax + 24], r11
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
	xor ecx, ecx
	test r9b, 64
	shlx r11, r10, r9
	mov r9, r11
	cmovne r9, rcx
	cmovne r10, r11
	mov rsi, r9
	not rsi
	mov r11, r10
	not r11
	add rdx, rsi
	adc r8, r11
	jo .LBB26_3
	and r8, r10
	and rdx, r9
	mov ecx, 1
	mov qword ptr [rax + 16], rdx
	mov qword ptr [rax + 24], r8
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
	mov eax, -1
	shlx edx, eax, edx
	xor eax, eax
	mov r8d, edx
	not r8d
	add r8d, ecx
	cmp r8w, cx
	mov r8d, edx
	setae al
	sub r8d, ecx
	andn edx, r8d, edx
```
## `checked_ceil_to_multiple_u16_unb_pow2`
```asm
	cmp dl, 15
	ja .LBB38_1
	mov eax, -1
	shlx edx, eax, edx
	xor eax, eax
	mov r8d, edx
	not r8d
	add r8d, ecx
	cmp r8w, cx
	mov r8d, edx
	setae al
	sub r8d, ecx
	andn edx, r8d, edx
	ret
.LBB38_1:
	xor eax, eax
```
## `checked_ceil_to_multiple_u32_pow2`
```asm
	mov eax, -1
	shlx edx, eax, edx
	xor eax, eax
	mov r8d, edx
	not r8d
	add r8d, ecx
	cmp r8d, ecx
	mov r8d, edx
	setae al
	sub r8d, ecx
	andn edx, r8d, edx
```
## `checked_ceil_to_multiple_u32_unb_pow2`
```asm
	cmp dl, 31
	ja .LBB40_1
	mov eax, -1
	shlx edx, eax, edx
	xor eax, eax
	mov r8d, edx
	not r8d
	add r8d, ecx
	cmp r8d, ecx
	mov r8d, edx
	setae al
	sub r8d, ecx
	andn edx, r8d, edx
	ret
.LBB40_1:
	xor eax, eax
```
## `checked_ceil_to_multiple_u64_pow2`
```asm
	mov rax, -1
	shlx rdx, rax, rdx
	xor eax, eax
	mov r8, rdx
	not r8
	add r8, rcx
	cmp r8, rcx
	mov r8, rdx
	setae al
	sub r8, rcx
	andn rdx, r8, rdx
```
## `checked_ceil_to_multiple_u64_unb_pow2`
```asm
	cmp dl, 63
	ja .LBB42_1
	mov rax, -1
	shlx rdx, rax, rdx
	xor eax, eax
	mov r8, rdx
	not r8
	add r8, rcx
	cmp r8, rcx
	mov r8, rdx
	setae al
	sub r8, rcx
	andn rdx, r8, rdx
	ret
.LBB42_1:
	xor eax, eax
```
## `checked_ceil_to_multiple_u128_pow2`
```asm
	push rsi
	mov r10, -1
	mov rax, rcx
	xor ecx, ecx
	test r9b, 64
	shlx r11, r10, r9
	mov r9, r11
	cmovne r9, rcx
	cmovne r10, r11
	mov rsi, r9
	not rsi
	mov r11, r10
	not r11
	add rsi, rdx
	adc r11, r8
	jb .LBB35_2
	and r11, r10
	and rsi, r9
	mov ecx, 1
	mov qword ptr [rax + 16], rsi
	mov qword ptr [rax + 24], r11
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
	xor ecx, ecx
	test r9b, 64
	shlx r11, r10, r9
	mov r9, r11
	cmovne r9, rcx
	cmovne r10, r11
	mov rsi, r9
	not rsi
	mov r11, r10
	not r11
	add rdx, rsi
	adc r8, r11
	jb .LBB36_3
	and r8, r10
	and rdx, r9
	mov ecx, 1
	mov qword ptr [rax + 16], rdx
	mov qword ptr [rax + 24], r8
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
	mov edx, eax
	not r9b
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
	cmp dl, 15
	ja .LBB46_1
	movsx eax, cx
	mov ecx, edx
	sarx edx, eax, edx
	bzhi eax, eax, ecx
	cmp ax, 1
	mov ax, 1
	sbb dx, -1
	ret
.LBB46_1:
	xor eax, eax
```
## `checked_div_ceil_i32_unb_pow2`
```asm
	cmp dl, 31
	ja .LBB47_1
	mov eax, edx
	bzhi eax, ecx, eax
	sarx edx, ecx, edx
	cmp eax, 1
	mov eax, 1
	sbb edx, -1
	ret
.LBB47_1:
	xor eax, eax
```
## `checked_div_ceil_i64_unb_pow2`
```asm
	cmp dl, 63
	ja .LBB48_1
	mov rax, rdx
	bzhi rax, rcx, rax
	sarx rdx, rcx, rdx
	cmp rax, 1
	mov eax, 1
	sbb rdx, -1
	ret
.LBB48_1:
	xor eax, eax
```
## `checked_div_ceil_i128_unb_pow2`
```asm
	push rsi
	push rdi
	mov rax, rcx
	test r9b, r9b
	js .LBB45_1
	mov r10, -1
	mov ecx, r9d
	mov rsi, rdx
	xor edi, edi
	shrd rsi, r8, cl
	mov rcx, r8
	sar rcx, 63
	shlx r11, r10, r9
	test r9b, 64
	sarx r9, r8, r9
	cmove rdi, r11
	cmovne r10, r11
	cmove rcx, r9
	cmovne rsi, r9
	andn r8, r10, r8
	andn rdx, rdi, rdx
	xor r9d, r9d
	or rdx, r8
	setne r9b
	add r9, rsi
	adc rcx, 0
	mov qword ptr [rax + 16], r9
	mov qword ptr [rax + 24], rcx
	mov ecx, 1
	jmp .LBB45_3
.LBB45_1:
	xor ecx, ecx
.LBB45_3:
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
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
	mov edx, eax
	not r9b
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
	cmp dl, 15
	ja .LBB51_1
	movzx eax, cx
	mov ecx, edx
	shrx edx, eax, edx
	bzhi eax, eax, ecx
	cmp ax, 1
	mov ax, 1
	sbb dx, -1
	ret
.LBB51_1:
	xor eax, eax
```
## `checked_div_ceil_u32_unb_pow2`
```asm
	cmp dl, 31
	ja .LBB52_1
	mov eax, edx
	bzhi eax, ecx, eax
	shrx edx, ecx, edx
	cmp eax, 1
	mov eax, 1
	sbb edx, -1
	ret
.LBB52_1:
	xor eax, eax
```
## `checked_div_ceil_u64_unb_pow2`
```asm
	cmp dl, 63
	ja .LBB53_1
	mov rax, rdx
	bzhi rax, rcx, rax
	shrx rdx, rcx, rdx
	cmp rax, 1
	mov eax, 1
	sbb rdx, -1
	ret
.LBB53_1:
	xor eax, eax
```
## `checked_div_ceil_u128_unb_pow2`
```asm
	push rsi
	mov rax, rcx
	test r9b, r9b
	js .LBB50_1
	mov r10, -1
	mov ecx, r9d
	mov r11, rdx
	xor esi, esi
	shrd r11, r8, cl
	shlx rcx, r10, r9
	test r9b, 64
	shrx r9, r8, r9
	cmovne r10, rcx
	cmovne rcx, rsi
	cmovne r11, r9
	cmovne r9, rsi
	andn rcx, rcx, rdx
	andn r8, r10, r8
	xor edx, edx
	or rcx, r8
	mov ecx, 1
	setne dl
	add rdx, r11
	adc r9, 0
	mov qword ptr [rax + 16], rdx
	mov qword ptr [rax + 24], r9
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	pop rsi
	ret
.LBB50_1:
	xor ecx, ecx
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	pop rsi
```
## `checked_div_floor_i8_unb_pow2`
```asm
	mov r8d, ecx
	cmp dl, 8
	mov ecx, edx
	setb al
	sar r8b, cl
	mov edx, r8d
```
## `checked_div_floor_i16_unb_pow2`
```asm
	xor eax, eax
	cmp dl, 16
	movsx ecx, cx
	setb al
	sarx edx, ecx, edx
```
## `checked_div_floor_i32_unb_pow2`
```asm
	xor eax, eax
	cmp dl, 32
	sarx edx, ecx, edx
	setb al
```
## `checked_div_floor_i64_unb_pow2`
```asm
	xor eax, eax
	cmp dl, 64
	sarx rdx, rcx, rdx
	setb al
```
## `checked_div_floor_i128_unb_pow2`
```asm
	mov rax, rcx
	test r9b, r9b
	js .LBB55_1
	mov ecx, r9d
	shrd rdx, r8, cl
	sarx rcx, r8, r9
	sar r8, 63
	test r9b, 64
	cmove r8, rcx
	cmovne rdx, rcx
	mov ecx, 1
	mov qword ptr [rax + 24], r8
	mov qword ptr [rax + 16], rdx
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
	mov ecx, edx
	setb al
	shr r8b, cl
	mov edx, r8d
```
## `checked_div_floor_u16_unb_pow2`, `checked_div_u16_unb_pow2`
```asm
	xor eax, eax
	cmp dl, 16
	movzx ecx, cx
	setb al
	shrx edx, ecx, edx
```
## `checked_div_floor_u32_unb_pow2`, `checked_div_u32_unb_pow2`
```asm
	xor eax, eax
	cmp dl, 32
	shrx edx, ecx, edx
	setb al
```
## `checked_div_floor_u64_unb_pow2`, `checked_div_u64_unb_pow2`
```asm
	xor eax, eax
	cmp dl, 64
	shrx rdx, rcx, rdx
	setb al
```
## `checked_div_floor_u128_unb_pow2`
```asm
	mov rax, rcx
	test r9b, r9b
	js .LBB60_1
	mov ecx, r9d
	shrd rdx, r8, cl
	shrx rcx, r8, r9
	xor r8d, r8d
	test r9b, 64
	cmove r8, rcx
	cmovne rdx, rcx
	mov ecx, 1
	mov qword ptr [rax + 24], r8
	mov qword ptr [rax + 16], rdx
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
	mov edx, r8d
	not r9b
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
	movsx eax, cx
	sar eax, 15
	bzhi eax, eax, edx
	add eax, ecx
	cwde
	sarx edx, eax, edx
	mov ax, 1
	ret
.LBB66_1:
	xor eax, eax
```
## `checked_div_i32_unb_pow2`
```asm
	cmp dl, 31
	ja .LBB67_1
	mov eax, ecx
	sar eax, 31
	bzhi eax, eax, edx
	add eax, ecx
	sarx edx, eax, edx
	mov eax, 1
	ret
.LBB67_1:
	xor eax, eax
```
## `checked_div_i64_unb_pow2`
```asm
	cmp dl, 63
	ja .LBB68_1
	mov rax, rcx
	sar rax, 63
	bzhi rax, rax, rdx
	add rax, rcx
	sarx rdx, rax, rdx
	mov eax, 1
	ret
.LBB68_1:
	xor eax, eax
```
## `checked_div_i128_unb_pow2`
```asm
	push rsi
	mov rax, rcx
	test r9b, r9b
	js .LBB65_1
	mov rcx, -1
	xor r11d, r11d
	test r9b, 64
	shlx r10, rcx, r9
	cmove r11, r10
	cmovne rcx, r10
	mov r10, r8
	sar r10, 63
	andn rsi, rcx, r10
	andn r10, r11, r10
	mov ecx, r9d
	add r10, rdx
	adc rsi, r8
	mov rdx, rsi
	sar rdx, 63
	test r9b, 64
	sarx r8, rsi, r9
	cmove rdx, r8
	shrd r10, rsi, cl
	test r9b, 64
	mov ecx, 1
	mov qword ptr [rax + 24], rdx
	cmovne r10, r8
	mov qword ptr [rax + 16], r10
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	pop rsi
	ret
.LBB65_1:
	xor ecx, ecx
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	pop rsi
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
	mov edx, r8d
	not r9b
	sar dl, cl
	xor ecx, ecx
	and r9b, r8b
	shr r8b, 7
	sub r9b, r8b
	movzx r8d, r9b
	movzx r9d, al
	cmovb r8d, ecx
	add r8b, r8b
	movzx ecx, r8b
	bt ecx, r9d
	adc dl, 0
.LBB74_2:
	cmp al, 8
	setb al
```
## `checked_div_round_i16_unb_pow2`
```asm
	xor eax, eax
	cmp dl, 15
	ja .LBB71_1
	movsx ecx, cx
	mov r8d, edx
	movzx r9d, cx
	sarx edx, ecx, edx
	bzhi ecx, ecx, r8d
	shr r9d, 15
	sub cx, r9w
	cmovb ecx, eax
	movzx eax, r8b
	add ecx, ecx
	bt ecx, eax
	mov ax, 1
	adc dx, 0
	ret
.LBB71_1:
```
## `checked_div_round_i32_unb_pow2`
```asm
	cmp dl, 31
	ja .LBB72_1
	mov eax, edx
	bzhi r8d, ecx, eax
	sarx edx, ecx, edx
	shr ecx, 31
	xor r9d, r9d
	movzx eax, al
	sub r8d, ecx
	cmovb r8d, r9d
	add r8d, r8d
	bt r8d, eax
	mov eax, 1
	adc edx, 0
	ret
.LBB72_1:
	xor eax, eax
```
## `checked_div_round_i64_unb_pow2`
```asm
	cmp dl, 63
	ja .LBB73_1
	mov rax, rdx
	bzhi r8, rcx, rax
	sarx rdx, rcx, rdx
	shr rcx, 63
	xor r9d, r9d
	movzx eax, al
	sub r8, rcx
	cmovae r9, r8
	add r9, r9
	bt r9, rax
	mov eax, 1
	adc rdx, 0
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
	mov rcx, -1
	xor r11d, r11d
	test r9b, 64
	shlx r10, rcx, r9
	cmovne rcx, r10
	cmovne r10, r11
	andn rsi, rcx, r8
	mov rcx, r8
	andn r10, r10, rdx
	shr rcx, 63
	sub r10, rcx
	mov ecx, r9d
	sbb rsi, 0
	cmovb rsi, r11
	cmovb r10, r11
	shld rsi, r10, 1
	add r10, r10
	shrd r10, rsi, cl
	shrx r11, rsi, r9
	test r9b, 64
	sarx rsi, r8, r9
	cmove r11, r10
	mov r10, r8
	sar r10, 63
	test r9b, 64
	cmove r10, rsi
	shrd rdx, r8, cl
	test r9b, 64
	mov ecx, 1
	cmovne rdx, rsi
	and r11d, 1
	add r11, rdx
	adc r10, 0
	mov qword ptr [rax + 16], r11
	mov qword ptr [rax + 24], r10
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
	cmp dl, 15
	ja .LBB76_1
	mov eax, 1
	movzx ecx, cx
	shlx eax, eax, edx
	shrx edx, ecx, edx
	movzx eax, ax
	shr eax
	and ecx, eax
	mov ax, 1
	cmp cx, 1
	sbb dx, -1
	ret
.LBB76_1:
	xor eax, eax
```
## `checked_div_round_u32_unb_pow2`
```asm
	cmp dl, 31
	ja .LBB77_1
	mov eax, 1
	shlx r8d, eax, edx
	shrx edx, ecx, edx
	shr r8d
	and r8d, ecx
	cmp r8d, 1
	sbb edx, -1
	ret
.LBB77_1:
	xor eax, eax
```
## `checked_div_round_u64_unb_pow2`
```asm
	cmp dl, 63
	ja .LBB78_1
	mov eax, 1
	shlx r8, rax, rdx
	shrx rdx, rcx, rdx
	shr r8
	and r8, rcx
	cmp r8, 1
	sbb rdx, -1
	ret
.LBB78_1:
	xor eax, eax
```
## `checked_div_round_u128_unb_pow2`
```asm
	push r15
	push r14
	push rsi
	push rdi
	push rbx
	mov rax, rcx
	test r9b, r9b
	js .LBB75_1
	shrx rbx, r8, r9
	xor edi, edi
	test r9b, 64
	mov ecx, r9d
	mov r10d, 1
	mov rsi, rdx
	mov r11, rbx
	cmovne r11, rdi
	shrd rsi, r8, cl
	test r9b, 64
	shlx r14, r10, r9
	mov r15, r14
	cmovne r15, rdi
	cmovne rsi, rbx
	shld rdi, r10, cl
	test r9b, 64
	cmovne rdi, r14
	xor ecx, ecx
	shrd r15, rdi, 1
	shr rdi
	and rdi, r8
	and r15, rdx
	or r15, rdi
	setne cl
	add rcx, rsi
	adc r11, 0
	mov qword ptr [rax + 16], rcx
	mov qword ptr [rax + 24], r11
	jmp .LBB75_3
.LBB75_1:
	xor r10d, r10d
.LBB75_3:
	mov qword ptr [rax], r10
	mov qword ptr [rax + 8], 0
	pop rbx
	pop rdi
	pop rsi
	pop r14
	pop r15
```
## `checked_div_u128_unb_pow2`
```asm
	mov rax, rcx
	test r9b, r9b
	js .LBB80_1
	mov ecx, r9d
	shrd rdx, r8, cl
	shrx rcx, r8, r9
	xor r8d, r8d
	test r9b, 64
	cmove r8, rcx
	cmovne rdx, rcx
	mov ecx, 1
	mov qword ptr [rax + 24], r8
	mov qword ptr [rax + 16], rdx
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
	cmp dl, 8
	mov r8d, ecx
	mov dl, -1
	mov ecx, r9d
	setb al
	shl dl, cl
	and dl, r8b
```
## `checked_floor_to_multiple_i16_unb_pow2`, `checked_floor_to_multiple_u16_unb_pow2`
```asm
	mov r8d, -1
	xor eax, eax
	cmp dl, 16
	shlx edx, r8d, edx
	setb al
	and edx, ecx
```
## `checked_floor_to_multiple_i32_unb_pow2`, `checked_floor_to_multiple_u32_unb_pow2`
```asm
	mov r8d, -1
	xor eax, eax
	cmp dl, 32
	shlx edx, r8d, edx
	setb al
	and edx, ecx
```
## `checked_floor_to_multiple_i64_unb_pow2`, `checked_floor_to_multiple_u64_unb_pow2`
```asm
	mov r8, -1
	xor eax, eax
	cmp dl, 64
	shlx rdx, r8, rdx
	setb al
	and rdx, rcx
```
## `checked_floor_to_multiple_i128_unb_pow2`
```asm
	mov rax, rcx
	test r9b, r9b
	js .LBB85_1
	mov rcx, -1
	xor r11d, r11d
	test r9b, 64
	shlx r10, rcx, r9
	cmovne rcx, r10
	cmove r11, r10
	and rcx, r8
	and r11, rdx
	mov qword ptr [rax + 24], rcx
	mov ecx, 1
	mov qword ptr [rax + 16], r11
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
	mov rcx, -1
	xor r11d, r11d
	test r9b, 64
	shlx r10, rcx, r9
	cmovne rcx, r10
	cmove r11, r10
	and rcx, r8
	and r11, rdx
	mov qword ptr [rax + 24], rcx
	mov ecx, 1
	mov qword ptr [rax + 16], r11
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
	cmp dl, 8
	mov eax, ecx
	mov edx, ecx
	mov ecx, r8d
	setb r9b
	and r8b, 7
	shl dl, cl
	mov ecx, r8d
	mov r10d, edx
	sar r10b, cl
	cmp al, r10b
	sete al
	and al, r9b
```
## `checked_mul_i16_pow2`
```asm
	shlx eax, ecx, edx
	movsx r8d, ax
	xor eax, eax
	sarx edx, r8d, edx
	cmp cx, dx
	mov edx, r8d
	sete al
```
## `checked_mul_i16_unb_pow2`
```asm
	cmp dl, 15
	ja .LBB98_1
	mov eax, edx
	shlx edx, ecx, edx
	movsx r8d, dx
	sarx r8d, r8d, eax
	xor eax, eax
	cmp cx, r8w
	sete al
	ret
.LBB98_1:
	xor eax, eax
```
## `checked_mul_i32_pow2`
```asm
	shlx r8d, ecx, edx
	xor eax, eax
	sarx edx, r8d, edx
	cmp ecx, edx
	mov edx, r8d
	sete al
```
## `checked_mul_i32_unb_pow2`
```asm
	cmp dl, 31
	ja .LBB100_1
	mov eax, edx
	shlx edx, ecx, edx
	sarx r8d, edx, eax
	xor eax, eax
	cmp ecx, r8d
	sete al
	ret
.LBB100_1:
	xor eax, eax
```
## `checked_mul_i64_pow2`
```asm
	shlx r8, rcx, rdx
	xor eax, eax
	sarx rdx, r8, rdx
	cmp rcx, rdx
	mov rdx, r8
	sete al
```
## `checked_mul_i64_unb_pow2`
```asm
	cmp dl, 63
	ja .LBB102_1
	mov rax, rdx
	shlx rdx, rcx, rdx
	sarx r8, rdx, rax
	xor eax, eax
	cmp rcx, r8
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
	mov ecx, r9d
	mov r10, r8
	shlx r11, rdx, r9
	shld r10, rdx, cl
	test r9b, 64
	cmovne r10, r11
	xor esi, esi
	mov rdi, r10
	sar rdi, 63
	test r9b, 64
	sarx rbx, r10, r9
	cmovne r11, rsi
	cmove rdi, rbx
	mov r14, r11
	shrd r14, r10, cl
	test r9b, 64
	cmovne r14, rbx
	xor rdi, r8
	xor r14, rdx
	or r14, rdi
	jne .LBB95_2
	mov esi, 1
	mov qword ptr [rax + 16], r11
	mov qword ptr [rax + 24], r10
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
	mov ecx, r9d
	mov r10, r8
	shlx rsi, rdx, r9
	shld r10, rdx, cl
	test r9b, 64
	cmovne r10, rsi
	xor r11d, r11d
	mov rdi, r10
	sar rdi, 63
	test r9b, 64
	sarx rbx, r10, r9
	cmovne rsi, r11
	cmove rdi, rbx
	mov r14, rsi
	shrd r14, r10, cl
	test r9b, 64
	cmovne r14, rbx
	xor rdi, r8
	xor r14, rdx
	or r14, rdi
	jne .LBB96_4
	mov r11d, 1
	mov qword ptr [rax + 16], rsi
	mov qword ptr [rax + 24], r10
	jmp .LBB96_4
.LBB96_1:
	xor r11d, r11d
.LBB96_4:
	mov qword ptr [rax], r11
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
	cmp dl, 8
	mov eax, ecx
	mov edx, ecx
	mov ecx, r8d
	setb r9b
	and r8b, 7
	shl dl, cl
	mov ecx, r8d
	mov r10d, edx
	shr r10b, cl
	cmp al, r10b
	sete al
	and al, r9b
```
## `checked_mul_u16_pow2`
```asm
	shlx eax, ecx, edx
	movzx r8d, ax
	xor eax, eax
	shrx edx, r8d, edx
	cmp cx, dx
	mov edx, r8d
	sete al
```
## `checked_mul_u16_unb_pow2`
```asm
	cmp dl, 15
	ja .LBB108_1
	mov eax, edx
	shlx edx, ecx, edx
	movzx r8d, dx
	shrx r8d, r8d, eax
	xor eax, eax
	cmp cx, r8w
	sete al
	ret
.LBB108_1:
	xor eax, eax
```
## `checked_mul_u32_pow2`
```asm
	shlx r8d, ecx, edx
	xor eax, eax
	shrx edx, r8d, edx
	cmp ecx, edx
	mov edx, r8d
	sete al
```
## `checked_mul_u32_unb_pow2`
```asm
	cmp dl, 31
	ja .LBB110_1
	mov eax, edx
	shlx edx, ecx, edx
	shrx r8d, edx, eax
	xor eax, eax
	cmp ecx, r8d
	sete al
	ret
.LBB110_1:
	xor eax, eax
```
## `checked_mul_u64_pow2`
```asm
	shlx r8, rcx, rdx
	xor eax, eax
	shrx rdx, r8, rdx
	cmp rcx, rdx
	mov rdx, r8
	sete al
```
## `checked_mul_u64_unb_pow2`
```asm
	cmp dl, 63
	ja .LBB112_1
	mov rax, rdx
	shlx rdx, rcx, rdx
	shrx r8, rdx, rax
	xor eax, eax
	cmp rcx, r8
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
	mov ecx, r9d
	mov r10, r8
	shlx rsi, rdx, r9
	xor r11d, r11d
	shld r10, rdx, cl
	test r9b, 64
	cmovne r10, rsi
	cmovne rsi, r11
	shrx rdi, r10, r9
	mov r14, rsi
	mov rbx, rdi
	cmovne rbx, r11
	shrd r14, r10, cl
	test r9b, 64
	cmovne r14, rdi
	xor rbx, r8
	xor r14, rdx
	or rbx, r14
	jne .LBB105_2
	mov r11d, 1
	mov qword ptr [rax + 16], rsi
	mov qword ptr [rax + 24], r10
.LBB105_2:
	mov qword ptr [rax], r11
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
	mov ecx, r9d
	mov r11, r8
	shlx rsi, rdx, r9
	xor r10d, r10d
	shld r11, rdx, cl
	test r9b, 64
	cmovne r11, rsi
	cmovne rsi, r10
	shrx rdi, r11, r9
	mov r14, rsi
	mov rbx, rdi
	cmovne rbx, r10
	shrd r14, r11, cl
	test r9b, 64
	cmovne r14, rdi
	xor rbx, r8
	xor r14, rdx
	or rbx, r14
	jne .LBB106_4
	mov r10d, 1
	mov qword ptr [rax + 16], rsi
	mov qword ptr [rax + 24], r11
	jmp .LBB106_4
.LBB106_1:
	xor r10d, r10d
.LBB106_4:
	mov qword ptr [rax], r10
	mov qword ptr [rax + 8], 0
	pop rbx
	pop rdi
	pop rsi
	pop r14
```
## `checked_rem_floor_i8_unb_pow2`, `checked_rem_floor_u8_unb_pow2`, `checked_rem_u8_unb_pow2`
```asm
	mov r9d, edx
	cmp dl, 8
	mov r8d, ecx
	mov dl, -1
	mov ecx, r9d
	setb al
	shl dl, cl
	not dl
	and dl, r8b
```
## `checked_rem_floor_i16_unb_pow2`, `checked_rem_floor_u16_unb_pow2`, `checked_rem_u16_unb_pow2`
```asm
	mov r8d, -1
	xor eax, eax
	cmp dl, 16
	shlx edx, r8d, edx
	setb al
	andn edx, edx, ecx
```
## `checked_rem_floor_i32_unb_pow2`, `checked_rem_floor_u32_unb_pow2`, `checked_rem_u32_unb_pow2`
```asm
	mov r8d, -1
	xor eax, eax
	cmp dl, 32
	shlx edx, r8d, edx
	setb al
	andn edx, edx, ecx
```
## `checked_rem_floor_i64_unb_pow2`, `checked_rem_floor_u64_unb_pow2`, `checked_rem_u64_unb_pow2`
```asm
	mov r8, -1
	xor eax, eax
	cmp dl, 64
	shlx rdx, r8, rdx
	setb al
	andn rdx, rdx, rcx
```
## `checked_rem_floor_i128_unb_pow2`
```asm
	mov rax, rcx
	test r9b, r9b
	js .LBB115_1
	mov rcx, -1
	xor r11d, r11d
	test r9b, 64
	shlx r10, rcx, r9
	cmovne rcx, r10
	cmove r11, r10
	andn rcx, rcx, r8
	andn rdx, r11, rdx
	mov qword ptr [rax + 24], rcx
	mov ecx, 1
	mov qword ptr [rax + 16], rdx
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
	mov rcx, -1
	xor r11d, r11d
	test r9b, 64
	shlx r10, rcx, r9
	cmovne rcx, r10
	cmove r11, r10
	andn rcx, rcx, r8
	andn rdx, r11, rdx
	mov qword ptr [rax + 24], rcx
	mov ecx, 1
	mov qword ptr [rax + 16], rdx
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
	mov r9d, r8d
	shl al, cl
	sar r9b, 7
	mov ecx, eax
	not cl
	and r9b, cl
	add r9b, r8b
	and r9b, al
	sub r8b, r9b
.LBB129_2:
	cmp dl, 8
	mov edx, r8d
	setb al
```
## `checked_rem_i16_unb_pow2`
```asm
	cmp dl, 15
	ja .LBB126_1
	movsx r8d, cx
	mov eax, -1
	sar r8d, 15
	shlx eax, eax, edx
	bzhi edx, r8d, edx
	add edx, ecx
	and edx, eax
	mov ax, 1
	sub ecx, edx
	mov edx, ecx
	ret
.LBB126_1:
	mov edx, ecx
	xor eax, eax
```
## `checked_rem_i32_unb_pow2`
```asm
	cmp dl, 31
	ja .LBB127_1
	mov eax, -1
	mov r8d, ecx
	sar r8d, 31
	shlx eax, eax, edx
	bzhi edx, r8d, edx
	add edx, ecx
	and edx, eax
	mov eax, 1
	sub ecx, edx
	mov edx, ecx
	ret
.LBB127_1:
	xor eax, eax
	mov edx, ecx
```
## `checked_rem_i64_unb_pow2`
```asm
	cmp dl, 63
	ja .LBB128_1
	mov rax, -1
	mov r8, rcx
	sar r8, 63
	shlx rax, rax, rdx
	bzhi rdx, r8, rdx
	add rdx, rcx
	and rdx, rax
	mov eax, 1
	sub rcx, rdx
	mov rdx, rcx
	ret
.LBB128_1:
	xor eax, eax
	mov rdx, rcx
```
## `checked_rem_i128_unb_pow2`
```asm
	mov rax, rcx
	test r9b, r9b
	js .LBB125_1
	mov rcx, -1
	xor r11d, r11d
	test r9b, 64
	shlx r10, rcx, r9
	mov r9, r8
	cmove r11, r10
	cmovne rcx, r10
	sar r9, 63
	andn r10, rcx, r9
	andn r9, r11, r9
	add r9, rdx
	adc r10, r8
	and r9, r11
	and r10, rcx
	sub rdx, r9
	mov ecx, 1
	sbb r8, r10
	mov qword ptr [rax + 16], rdx
	mov qword ptr [rax + 24], r8
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
	ret
.LBB125_1:
	xor ecx, ecx
	mov qword ptr [rax], rcx
	mov qword ptr [rax + 8], 0
```
## `checked_rem_u128_unb_pow2`
```asm
	mov rax, rcx
	test r9b, r9b
	js .LBB130_1
	mov rcx, -1
	xor r11d, r11d
	test r9b, 64
	shlx r10, rcx, r9
	cmovne rcx, r10
	cmove r11, r10
	andn rcx, rcx, r8
	andn rdx, r11, rdx
	mov qword ptr [rax + 24], rcx
	mov ecx, 1
	mov qword ptr [rax + 16], rdx
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
	mov ecx, edx
	mov r9b, 1
	mov al, 1
	shl r9b, cl
	mov edx, r8d
	mov ecx, r9d
	shr dl, 7
	shr cl
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
	mov r8d, 1
	movzx ecx, cx
	shlx edx, r8d, edx
	mov r9d, ecx
	shr r9d, 15
	movzx r8d, dx
	shr r8d
	sub r8w, r9w
	cmovb r8d, eax
	add cx, r8w
	jo .LBB136_4
	neg edx
	mov ax, 1
	and edx, ecx
	ret
.LBB136_4:
```
## `checked_round_to_multiple_i32_unb_pow2`
```asm
	cmp dl, 31
	ja .LBB137_3
	mov eax, 1
	mov r9d, ecx
	shr r9d, 31
	shlx r8d, eax, edx
	xor eax, eax
	mov edx, r8d
	shr edx
	sub edx, r9d
	cmovb edx, eax
	add edx, ecx
	jo .LBB137_4
	neg r8d
	mov eax, 1
	and edx, r8d
	ret
.LBB137_3:
	xor eax, eax
.LBB137_4:
```
## `checked_round_to_multiple_i64_unb_pow2`
```asm
	cmp dl, 63
	ja .LBB138_3
	mov eax, 1
	mov r9, rcx
	shr r9, 63
	shlx r8, rax, rdx
	xor eax, eax
	mov rdx, r8
	shr rdx
	sub rdx, r9
	cmovb rdx, rax
	add rdx, rcx
	jo .LBB138_4
	neg r8
	mov eax, 1
	and rdx, r8
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
	mov ecx, r9d
	xor r11d, r11d
	xor esi, esi
	mov rbx, r8
	shld r11, r10, cl
	shlx rcx, r10, r9
	test r9b, 64
	cmovne r11, rcx
	cmovne rcx, rsi
	shr rbx, 63
	mov rdi, r11
	shld rdi, rcx, 63
	mov r9, r11
	shr r9
	sub rdi, rbx
	sbb r9, 0
	cmovb rdi, rsi
	cmovb r9, rsi
	add rdx, rdi
	adc r9, r8
	jo .LBB135_3
	xor r8d, r8d
	neg rcx
	sbb r8, r11
	and rdx, rcx
	and r9, r8
	mov qword ptr [rax + 16], rdx
	mov qword ptr [rax + 24], r9
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
	mov al, 1
	shl r9b, cl
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
	mov eax, 1
	shlx eax, eax, edx
	movzx edx, ax
	shr edx
	add dx, cx
	jb .LBB141_2
	neg eax
	and edx, eax
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
	mov r8d, 1
	shlx r8d, r8d, edx
	mov edx, r8d
	shr edx
	add edx, ecx
	jb .LBB142_2
	neg r8d
	mov eax, 1
	and edx, r8d
	ret
.LBB142_2:
```
## `checked_round_to_multiple_u64_unb_pow2`
```asm
	cmp dl, 63
	ja .LBB143_2
	mov eax, 1
	shlx r8, rax, rdx
	mov rdx, r8
	shr rdx
	add rdx, rcx
	jb .LBB143_2
	neg r8
	and rdx, r8
	ret
.LBB143_2:
	xor eax, eax
```
## `checked_round_to_multiple_u128_unb_pow2`
```asm
	push rsi
	mov rax, rcx
	test r9b, r9b
	js .LBB140_3
	mov r10d, 1
	mov ecx, r9d
	xor r11d, r11d
	xor esi, esi
	shld r11, r10, cl
	shlx rcx, r10, r9
	test r9b, 64
	cmovne r11, rcx
	cmovne rcx, rsi
	mov rsi, r11
	shld rsi, rcx, 63
	mov r9, r11
	shr r9
	add rsi, rdx
	adc r9, r8
	jb .LBB140_3
	xor edx, edx
	neg rcx
	sbb rdx, r11
	and rsi, rcx
	and r9, rdx
	mov qword ptr [rax + 16], rsi
	mov qword ptr [rax + 24], r9
	mov qword ptr [rax], r10
	mov qword ptr [rax + 8], 0
	pop rsi
	ret
.LBB140_3:
	xor r10d, r10d
	mov qword ptr [rax], r10
	mov qword ptr [rax + 8], 0
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
	and dl, 7
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
## `div_ceil_i16_pow2`
```asm
	movsx eax, cx
	bzhi ecx, ecx, edx
	sarx eax, eax, edx
	cmp cx, 1
	sbb ax, -1
```
## `div_ceil_i16_unb_pow2`
```asm
	and dl, 15
	movsx eax, cx
	bzhi ecx, ecx, edx
	sarx eax, eax, edx
	cmp cx, 1
	sbb ax, -1
```
## `div_ceil_i32_pow2`
```asm
	sarx eax, ecx, edx
	bzhi ecx, ecx, edx
	cmp ecx, 1
	sbb eax, -1
```
## `div_ceil_i32_unb_pow2`
```asm
	mov eax, edx
	and dl, 31
	sarx eax, ecx, eax
	bzhi ecx, ecx, edx
	cmp ecx, 1
	sbb eax, -1
```
## `div_ceil_i64_pow2`
```asm
	sarx rax, rcx, rdx
	bzhi rcx, rcx, rdx
	cmp rcx, 1
	sbb rax, -1
```
## `div_ceil_i64_unb_pow2`
```asm
	mov eax, edx
	and dl, 63
	sarx rax, rcx, rax
	bzhi rcx, rcx, rdx
	cmp rcx, 1
	sbb rax, -1
```
## `div_ceil_i128_pow2`, `div_ceil_i128_unb_pow2`
```asm
	push rsi
	mov r9, -1
	mov rax, rcx
	mov r11, rcx
	mov ecx, r8d
	xor esi, esi
	shrd r11, rdx, cl
	mov rcx, rdx
	sar rcx, 63
	shlx r10, r9, r8
	test r8b, 64
	sarx r8, rdx, r8
	cmove rsi, r10
	cmovne r9, r10
	cmove rcx, r8
	cmovne r11, r8
	andn r8, rsi, rax
	andn rdx, r9, rdx
	xor eax, eax
	or r8, rdx
	setne al
	add rax, r11
	adc rcx, 0
	mov rdx, rcx
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
	and dl, 7
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
## `div_ceil_u16_pow2`
```asm
	movzx eax, cx
	bzhi ecx, ecx, edx
	shrx eax, eax, edx
	cmp cx, 1
	sbb ax, -1
```
## `div_ceil_u16_unb_pow2`
```asm
	and dl, 15
	movzx eax, cx
	bzhi ecx, ecx, edx
	shrx eax, eax, edx
	cmp cx, 1
	sbb ax, -1
```
## `div_ceil_u32_pow2`
```asm
	shrx eax, ecx, edx
	bzhi ecx, ecx, edx
	cmp ecx, 1
	sbb eax, -1
```
## `div_ceil_u32_unb_pow2`
```asm
	mov eax, edx
	and dl, 31
	shrx eax, ecx, eax
	bzhi ecx, ecx, edx
	cmp ecx, 1
	sbb eax, -1
```
## `div_ceil_u64_pow2`
```asm
	shrx rax, rcx, rdx
	bzhi rcx, rcx, rdx
	cmp rcx, 1
	sbb rax, -1
```
## `div_ceil_u64_unb_pow2`
```asm
	mov eax, edx
	and dl, 63
	shrx rax, rcx, rax
	bzhi rcx, rcx, rdx
	cmp rcx, 1
	sbb rax, -1
```
## `div_ceil_u128_pow2`, `div_ceil_u128_unb_pow2`
```asm
	push rsi
	mov r10, -1
	mov rax, rcx
	mov r9, rcx
	mov ecx, r8d
	xor esi, esi
	shrd r9, rdx, cl
	shlx r11, r10, r8
	test r8b, 64
	shrx rcx, rdx, r8
	cmovne r10, r11
	cmovne r11, rsi
	cmovne r9, rcx
	cmovne rcx, rsi
	andn r8, r11, rax
	andn rdx, r10, rdx
	xor eax, eax
	or r8, rdx
	setne al
	add rax, r9
	adc rcx, 0
	mov rdx, rcx
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
	and dl, 7
	mov eax, ecx
	mov ecx, edx
	sar al, cl
```
## `div_floor_i16_pow2`
```asm
	movsx eax, cx
	sarx eax, eax, edx
```
## `div_floor_i16_unb_pow2`
```asm
	movsx eax, cx
	and dl, 15
	sarx eax, eax, edx
```
## `div_floor_i32_pow2`, `div_floor_i32_unb_pow2`
```asm
	sarx eax, ecx, edx
```
## `div_floor_i64_pow2`, `div_floor_i64_unb_pow2`
```asm
	sarx rax, rcx, rdx
```
## `div_floor_i128_pow2`, `div_floor_i128_unb_pow2`
```asm
	mov rax, rcx
	mov ecx, r8d
	shrd rax, rdx, cl
	sarx rcx, rdx, r8
	sar rdx, 63
	test r8b, 64
	cmove rdx, rcx
	cmovne rax, rcx
```
## `div_floor_u8_pow2`
```asm
	mov eax, ecx
	mov ecx, edx
	shr al, cl
```
## `div_floor_u8_unb_pow2`, `div_u8_pow2`, `div_u8_unb_pow2`
```asm
	and dl, 7
	mov eax, ecx
	mov ecx, edx
	shr al, cl
```
## `div_floor_u16_pow2`
```asm
	movzx eax, cx
	shrx eax, eax, edx
```
## `div_floor_u16_unb_pow2`, `div_u16_pow2`, `div_u16_unb_pow2`
```asm
	movzx eax, cx
	and dl, 15
	shrx eax, eax, edx
```
## `div_floor_u32_pow2`, `div_floor_u32_unb_pow2`, `div_u32_pow2`, `div_u32_unb_pow2`
```asm
	shrx eax, ecx, edx
```
## `div_floor_u64_pow2`, `div_floor_u64_unb_pow2`, `div_u64_pow2`, `div_u64_unb_pow2`
```asm
	shrx rax, rcx, rdx
```
## `div_floor_u128_pow2`, `div_floor_u128_unb_pow2`, `div_u128_pow2`, `div_u128_unb_pow2`
```asm
	mov rax, rcx
	mov ecx, r8d
	shrd rax, rdx, cl
	shrx rcx, rdx, r8
	xor edx, edx
	test r8b, 64
	cmove rdx, rcx
	cmovne rax, rcx
```
## `div_i8_pow2`
```asm
	mov r8d, ecx
	mov ecx, edx
	mov r9b, -1
	and dl, 7
	shl r9b, cl
	mov eax, r8d
	mov ecx, edx
	not r9b
	sar al, 7
	and al, r9b
	add al, r8b
	sar al, cl
```
## `div_i8_unb_pow2`
```asm
	and dl, 7
	mov r8d, ecx
	mov r9b, -1
	mov ecx, edx
	mov eax, r8d
	shl r9b, cl
	sar al, 7
	not r9b
	and al, r9b
	add al, r8b
	sar al, cl
```
## `div_i16_pow2`
```asm
	movsx eax, cx
	sar eax, 15
	bzhi eax, eax, edx
	and dl, 15
	add eax, ecx
	cwde
	sarx eax, eax, edx
```
## `div_i16_unb_pow2`
```asm
	movsx eax, cx
	and dl, 15
	sar eax, 15
	bzhi eax, eax, edx
	add eax, ecx
	cwde
	sarx eax, eax, edx
```
## `div_i32_pow2`
```asm
	mov eax, ecx
	sar eax, 31
	bzhi eax, eax, edx
	add eax, ecx
	sarx eax, eax, edx
```
## `div_i32_unb_pow2`
```asm
	mov r8d, ecx
	mov eax, edx
	and dl, 31
	sar r8d, 31
	bzhi edx, r8d, edx
	add edx, ecx
	sarx eax, edx, eax
```
## `div_i64_pow2`
```asm
	mov rax, rcx
	sar rax, 63
	bzhi rax, rax, rdx
	add rax, rcx
	sarx rax, rax, rdx
```
## `div_i64_unb_pow2`
```asm
	mov r8, rcx
	mov eax, edx
	and dl, 63
	sar r8, 63
	bzhi rdx, r8, rdx
	add rdx, rcx
	sarx rax, rdx, rax
```
## `div_i128_pow2`, `div_i128_unb_pow2`
```asm
	mov rax, -1
	xor r10d, r10d
	test r8b, 64
	shlx r9, rax, r8
	cmove r10, r9
	cmovne rax, r9
	mov r9, rdx
	sar r9, 63
	andn r11, rax, r9
	andn rax, r10, r9
	add rax, rcx
	mov ecx, r8d
	adc r11, rdx
	mov rdx, r11
	sar rdx, 63
	test r8b, 64
	sarx r9, r11, r8
	cmove rdx, r9
	shrd rax, r11, cl
	test r8b, 64
	cmovne rax, r9
```
## `div_round_i8_pow2`
```asm
	mov r8d, ecx
	mov ecx, edx
	mov r9b, -1
	movzx edx, dl
	shl r9b, cl
	mov eax, r8d
	not r9b
	sar al, cl
	xor ecx, ecx
	and r9b, r8b
	shr r8b, 7
	sub r9b, r8b
	movzx r8d, r9b
	cmovb r8d, ecx
	add r8b, r8b
	movzx ecx, r8b
	bt ecx, edx
	adc al, 0
```
## `div_round_i8_unb_pow2`
```asm
	and dl, 7
	mov r8d, ecx
	mov r9b, -1
	mov ecx, edx
	mov eax, r8d
	movzx edx, dl
	shl r9b, cl
	sar al, cl
	xor ecx, ecx
	not r9b
	and r9b, r8b
	shr r8b, 7
	sub r9b, r8b
	movzx r8d, r9b
	cmovb r8d, ecx
	add r8b, r8b
	movzx ecx, r8b
	bt ecx, edx
	adc al, 0
```
## `div_round_i16_pow2`
```asm
	movzx r8d, cx
	bzhi ecx, ecx, edx
	xor r9d, r9d
	movsx eax, r8w
	shr r8d, 15
	sub cx, r8w
	sarx eax, eax, edx
	movzx edx, dl
	cmovb ecx, r9d
	add ecx, ecx
	bt ecx, edx
	adc ax, 0
```
## `div_round_i16_unb_pow2`
```asm
	movzx r8d, cx
	and dl, 15
	xor r9d, r9d
	movsx eax, r8w
	bzhi ecx, ecx, edx
	shr r8d, 15
	sub cx, r8w
	sarx eax, eax, edx
	movzx edx, dl
	cmovb ecx, r9d
	add ecx, ecx
	bt ecx, edx
	adc ax, 0
```
## `div_round_i32_pow2`
```asm
	sarx eax, ecx, edx
	bzhi r8d, ecx, edx
	shr ecx, 31
	xor r9d, r9d
	sub r8d, ecx
	movzx ecx, dl
	cmovb r8d, r9d
	add r8d, r8d
	bt r8d, ecx
	adc eax, 0
```
## `div_round_i32_unb_pow2`
```asm
	sarx eax, ecx, edx
	movzx r8d, dl
	and dl, 31
	xor r9d, r9d
	bzhi edx, ecx, edx
	shr ecx, 31
	sub edx, ecx
	cmovb edx, r9d
	add edx, edx
	bt edx, r8d
	adc eax, 0
```
## `div_round_i64_pow2`
```asm
	sarx rax, rcx, rdx
	bzhi r8, rcx, rdx
	shr rcx, 63
	xor r9d, r9d
	sub r8, rcx
	movzx ecx, dl
	cmovae r9, r8
	add r9, r9
	bt r9, rcx
	adc rax, 0
```
## `div_round_i64_unb_pow2`
```asm
	sarx rax, rcx, rdx
	movzx r8d, dl
	and dl, 63
	xor r9d, r9d
	bzhi rdx, rcx, rdx
	shr rcx, 63
	sub rdx, rcx
	cmovae r9, rdx
	add r9, r9
	bt r9, r8
	adc rax, 0
```
## `div_round_i128_pow2`, `div_round_i128_unb_pow2`
```asm
	mov rax, -1
	mov r9, rcx
	xor r10d, r10d
	test r8b, 64
	shlx rcx, rax, r8
	cmovne rax, rcx
	cmovne rcx, r10
	andn r11, rcx, r9
	mov rcx, rdx
	shr rcx, 63
	andn rax, rax, rdx
	sub r11, rcx
	mov ecx, r8d
	sbb rax, 0
	cmovb rax, r10
	cmovb r11, r10
	mov r10, rdx
	shld rax, r11, 1
	add r11, r11
	shrd r11, rax, cl
	test r8b, 64
	shrx rax, rax, r8
	cmove rax, r11
	sar r10, 63
	test r8b, 64
	sarx r11, rdx, r8
	cmove r10, r11
	shrd r9, rdx, cl
	test r8b, 64
	cmovne r9, r11
	and eax, 1
	add rax, r9
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
	and dl, 7
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
## `div_round_u16_pow2`
```asm
	mov r8d, 1
	movzx eax, cx
	shlx r8d, r8d, edx
	shrx eax, eax, edx
	movzx r8d, r8w
	shr r8d
	and r8d, ecx
	cmp r8w, 1
	sbb ax, -1
```
## `div_round_u16_unb_pow2`
```asm
	and dl, 15
	mov r8d, 1
	movzx eax, cx
	shlx r8d, r8d, edx
	shrx eax, eax, edx
	shr r8d
	and r8d, ecx
	cmp r8w, 1
	sbb ax, -1
```
## `div_round_u32_pow2`, `div_round_u32_unb_pow2`
```asm
	mov eax, 1
	shlx r8d, eax, edx
	shrx eax, ecx, edx
	shr r8d
	and r8d, ecx
	cmp r8d, 1
	sbb eax, -1
```
## `div_round_u64_pow2`, `div_round_u64_unb_pow2`
```asm
	mov eax, 1
	shlx r8, rax, rdx
	shrx rax, rcx, rdx
	shr r8
	and r8, rcx
	cmp r8, 1
	sbb rax, -1
```
## `div_round_u128_pow2`, `div_round_u128_unb_pow2`
```asm
	push r14
	push rsi
	push rdi
	push rbx
	mov r9d, r8d
	xor r10d, r10d
	mov rax, rcx
	mov r11, rcx
	mov edi, 1
	mov r8, rdx
	shrx rsi, rdx, r9
	test r9b, 64
	mov ecx, r9d
	shlx rbx, rdi, r9
	mov rdx, rsi
	cmovne rdx, r10
	shrd r11, r8, cl
	test r9b, 64
	mov r14, rbx
	cmovne r14, r10
	cmovne r11, rsi
	shld r10, rdi, cl
	test r9b, 64
	cmovne r10, rbx
	shrd r14, r10, 1
	shr r10
	and r10, r8
	and r14, rax
	xor eax, eax
	or r14, r10
	setne al
	add rax, r11
	adc rdx, 0
	pop rbx
	pop rdi
	pop rsi
	pop r14
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
	and dl, 7
	mov eax, ecx
	mov ecx, edx
	shr al, cl
	shl al, cl
```
## `floor_to_multiple_i16_pow2`, `floor_to_multiple_u16_pow2`
```asm
	movzx eax, cx
	shrx eax, eax, edx
	shlx eax, eax, edx
```
## `floor_to_multiple_i16_unb_pow2`, `floor_to_multiple_u16_unb_pow2`
```asm
	movzx eax, cx
	and dl, 15
	shrx eax, eax, edx
	shlx eax, eax, edx
```
## `floor_to_multiple_i32_pow2`, `floor_to_multiple_i32_unb_pow2`, `floor_to_multiple_u32_pow2`, `floor_to_multiple_u32_unb_pow2`
```asm
	shrx eax, ecx, edx
	shlx eax, eax, edx
```
## `floor_to_multiple_i64_pow2`, `floor_to_multiple_i64_unb_pow2`, `floor_to_multiple_u64_pow2`, `floor_to_multiple_u64_unb_pow2`
```asm
	shrx rax, rcx, rdx
	shlx rax, rax, rdx
```
## `floor_to_multiple_i128_pow2`, `floor_to_multiple_i128_unb_pow2`, `floor_to_multiple_u128_pow2`, `floor_to_multiple_u128_unb_pow2`
```asm
	mov r9, -1
	xor eax, eax
	test r8b, 64
	shlx r10, r9, r8
	cmovne r9, r10
	cmove rax, r10
	and rax, rcx
	and rdx, r9
```
## `is_multiple_of_i8_pow2`, `is_multiple_of_i8_unb_pow2`, `is_multiple_of_u8_pow2`, `is_multiple_of_u8_unb_pow2`
```asm
	movzx eax, cl
	or eax, 256
	tzcnt eax, eax
	cmp al, dl
	setae al
```
## `is_multiple_of_i16_pow2`, `is_multiple_of_i16_unb_pow2`, `is_multiple_of_u16_pow2`, `is_multiple_of_u16_unb_pow2`
```asm
	or ecx, 65536
	tzcnt eax, ecx
	movzx ecx, dl
	cmp ax, cx
	setae al
```
## `is_multiple_of_i32_pow2`, `is_multiple_of_i32_unb_pow2`, `is_multiple_of_u32_pow2`, `is_multiple_of_u32_unb_pow2`
```asm
	tzcnt eax, ecx
	movzx edx, dl
	cmp eax, edx
	setae al
```
## `is_multiple_of_i64_pow2`, `is_multiple_of_i64_unb_pow2`, `is_multiple_of_u64_pow2`, `is_multiple_of_u64_unb_pow2`
```asm
	tzcnt rax, rcx
	movzx edx, dl
	cmp eax, edx
	setae al
```
## `is_multiple_of_i128_pow2`, `is_multiple_of_i128_unb_pow2`, `is_multiple_of_u128_pow2`, `is_multiple_of_u128_unb_pow2`
```asm
	tzcnt rdx, rdx
	tzcnt rax, rcx
	add edx, 64
	test rcx, rcx
	cmovne edx, eax
	movzx eax, r8b
	cmp edx, eax
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
	and dl, 7
	mov eax, ecx
	mov ecx, edx
	shl al, cl
```
## `mul_i16_pow2`, `mul_i32_pow2`, `mul_i32_unb_pow2`, `mul_u16_pow2`, `mul_u32_pow2`, `mul_u32_unb_pow2`
```asm
	shlx eax, ecx, edx
```
## `mul_i16_unb_pow2`, `mul_u16_unb_pow2`
```asm
	and dl, 15
	shlx eax, ecx, edx
```
## `mul_i64_pow2`, `mul_i64_unb_pow2`, `mul_u64_pow2`, `mul_u64_unb_pow2`
```asm
	shlx rax, rcx, rdx
```
## `mul_i128_pow2`, `mul_i128_unb_pow2`, `mul_u128_pow2`, `mul_u128_unb_pow2`
```asm
	mov rax, rcx
	mov ecx, r8d
	shld rdx, rax, cl
	shlx rcx, rax, r8
	xor eax, eax
	test r8b, 64
	cmove rax, rcx
	cmovne rdx, rcx
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
	and dl, 7
	mov r8d, ecx
	mov al, -1
	mov ecx, edx
	shl al, cl
	not al
	and al, r8b
```
## `rem_floor_i16_pow2`, `rem_floor_i32_pow2`, `rem_floor_u16_pow2`, `rem_floor_u32_pow2`
```asm
	bzhi eax, ecx, edx
```
## `rem_floor_i16_unb_pow2`, `rem_floor_u16_unb_pow2`, `rem_u16_unb_pow2`
```asm
	and dl, 15
	bzhi eax, ecx, edx
```
## `rem_floor_i32_unb_pow2`, `rem_floor_u32_unb_pow2`, `rem_u32_unb_pow2`
```asm
	and dl, 31
	bzhi eax, ecx, edx
```
## `rem_floor_i64_pow2`, `rem_floor_u64_pow2`
```asm
	bzhi rax, rcx, rdx
```
## `rem_floor_i64_unb_pow2`, `rem_floor_u64_unb_pow2`, `rem_u64_unb_pow2`
```asm
	and dl, 63
	bzhi rax, rcx, rdx
```
## `rem_floor_i128_pow2`, `rem_floor_i128_unb_pow2`, `rem_floor_u128_pow2`, `rem_floor_u128_unb_pow2`, `rem_u128_unb_pow2`
```asm
	mov r9, -1
	xor r10d, r10d
	test r8b, 64
	shlx rax, r9, r8
	cmovne r9, rax
	cmove r10, rax
	andn rax, r10, rcx
	andn rdx, r9, rdx
```
## `rem_i8_pow2`
```asm
	mov eax, ecx
	mov r8b, -1
	mov ecx, edx
	shl r8b, cl
	mov r9d, eax
	and cl, 7
	not r8b
	sar r9b, 7
	and r9b, r8b
	add r9b, al
	sar r9b, cl
	mov ecx, edx
	shl r9b, cl
	sub al, r9b
```
## `rem_i8_unb_pow2`
```asm
	and dl, 7
	mov eax, ecx
	mov r8b, -1
	mov ecx, edx
	mov edx, eax
	shl r8b, cl
	sar dl, 7
	mov ecx, r8d
	not cl
	and dl, cl
	add dl, al
	and dl, r8b
	sub al, dl
```
## `rem_i16_pow2`
```asm
	mov eax, ecx
	movsx ecx, cx
	mov r8d, edx
	and dl, 15
	sar ecx, 15
	bzhi ecx, ecx, r8d
	add ecx, eax
	movsx ecx, cx
	sarx ecx, ecx, edx
	shlx ecx, ecx, r8d
	sub eax, ecx
```
## `rem_i16_unb_pow2`
```asm
	mov eax, ecx
	movsx ecx, cx
	and dl, 15
	mov r8d, -1
	sar ecx, 15
	shlx r8d, r8d, edx
	bzhi ecx, ecx, edx
	add ecx, eax
	and ecx, r8d
	sub eax, ecx
```
## `rem_i32_pow2`
```asm
	mov eax, ecx
	sar ecx, 31
	bzhi ecx, ecx, edx
	add ecx, eax
	sarx ecx, ecx, edx
	shlx ecx, ecx, edx
	sub eax, ecx
```
## `rem_i32_unb_pow2`
```asm
	mov eax, ecx
	mov r9d, eax
	mov ecx, edx
	and dl, 31
	sar r9d, 31
	mov r8d, -1
	bzhi edx, r9d, edx
	shlx ecx, r8d, ecx
	add edx, eax
	and edx, ecx
	sub eax, edx
```
## `rem_i64_pow2`
```asm
	mov rax, rcx
	sar rcx, 63
	bzhi rcx, rcx, rdx
	add rcx, rax
	sarx rcx, rcx, rdx
	shlx rcx, rcx, rdx
	sub rax, rcx
```
## `rem_i64_unb_pow2`
```asm
	mov rax, rcx
	mov r9, rax
	mov ecx, edx
	and dl, 63
	sar r9, 63
	mov r8, -1
	bzhi rdx, r9, rdx
	shlx rcx, r8, rcx
	add rdx, rax
	and rdx, rcx
	sub rax, rdx
```
## `rem_i128_pow2`
```asm
	push rsi
	mov rax, rcx
	mov rcx, -1
	xor r9d, r9d
	test r8b, 64
	mov r11, rdx
	shlx r10, rcx, r8
	cmovne rcx, r10
	cmovne r10, r9
	sar r11, 63
	andn r10, r10, r11
	andn rsi, rcx, r11
	mov ecx, r8d
	add r10, rax
	adc rsi, rdx
	shrd r10, rsi, cl
	test r8b, 64
	sarx rcx, rsi, r8
	cmovne r10, rcx
	shlx r11, r10, r8
	cmove r9, r11
	sar rsi, 63
	test r8b, 64
	cmove rsi, rcx
	mov ecx, r8d
	shld rsi, r10, cl
	test r8b, 64
	cmovne rsi, r11
	sub rax, r9
	sbb rdx, rsi
	pop rsi
```
## `rem_i128_unb_pow2`
```asm
	mov r11, -1
	xor r10d, r10d
	test r8b, 64
	mov rax, rcx
	shlx r9, r11, r8
	mov r8, rdx
	cmove r10, r9
	cmovne r11, r9
	sar r8, 63
	andn r9, r11, r8
	andn r8, r10, r8
	add r8, rax
	adc r9, rdx
	and r8, r10
	and r9, r11
	sub rax, r8
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
	mov r8d, edx
	movzx ecx, cx
	and dl, 15
	shrx ecx, ecx, edx
	shlx ecx, ecx, r8d
	sub eax, ecx
```
## `rem_u32_pow2`
```asm
	mov eax, ecx
	shrx ecx, ecx, edx
	shlx ecx, ecx, edx
	sub eax, ecx
```
## `rem_u64_pow2`
```asm
	mov rax, rcx
	shrx rcx, rcx, rdx
	shlx rcx, rcx, rdx
	sub rax, rcx
```
## `rem_u128_pow2`
```asm
	push rsi
	mov rax, rcx
	mov r9, rcx
	mov ecx, r8d
	shrx r10, rdx, r8
	xor r11d, r11d
	shrd r9, rdx, cl
	test r8b, 64
	cmovne r9, r10
	cmovne r10, r11
	shlx rsi, r9, r8
	cmove r11, rsi
	shld r10, r9, cl
	test r8b, 64
	cmovne r10, rsi
	sub rax, r11
	sbb rdx, r10
	pop rsi
```
## `round_to_multiple_i8_pow2`
```asm
	mov r8d, ecx
	mov ecx, edx
	mov al, 1
	xor r9d, r9d
	shl al, cl
	mov edx, r8d
	mov ecx, eax
	shr dl, 7
	shr cl
	sub cl, dl
	movzx ecx, cl
	cmovb ecx, r9d
	neg al
	add cl, r8b
	and al, cl
```
## `round_to_multiple_i8_unb_pow2`
```asm
	and dl, 7
	mov r8d, ecx
	mov al, 1
	xor r9d, r9d
	mov ecx, edx
	mov edx, r8d
	shl al, cl
	shr dl, 7
	mov ecx, eax
	shr cl
	sub cl, dl
	movzx ecx, cl
	cmovb ecx, r9d
	neg al
	add cl, r8b
	and al, cl
```
## `round_to_multiple_i16_pow2`
```asm
	mov eax, 1
	movzx r8d, cx
	xor r9d, r9d
	shlx eax, eax, edx
	shr r8d, 15
	movzx edx, ax
	shr edx
	sub dx, r8w
	cmovb edx, r9d
	neg eax
	add edx, ecx
	and eax, edx
```
## `round_to_multiple_i16_unb_pow2`
```asm
	and dl, 15
	mov eax, 1
	movzx r8d, cx
	xor r9d, r9d
	shlx eax, eax, edx
	shr r8d, 15
	mov edx, eax
	shr edx
	sub dx, r8w
	cmovb edx, r9d
	neg eax
	add edx, ecx
	and eax, edx
```
## `round_to_multiple_i32_pow2`, `round_to_multiple_i32_unb_pow2`
```asm
	mov eax, 1
	mov r8d, ecx
	shr r8d, 31
	xor r9d, r9d
	shlx eax, eax, edx
	mov edx, eax
	shr edx
	sub edx, r8d
	cmovb edx, r9d
	neg eax
	add edx, ecx
	and eax, edx
```
## `round_to_multiple_i64_pow2`, `round_to_multiple_i64_unb_pow2`
```asm
	mov eax, 1
	mov r8, rcx
	shr r8, 63
	xor r9d, r9d
	shlx rax, rax, rdx
	mov rdx, rax
	shr rdx
	sub rdx, r8
	cmovae r9, rdx
	neg rax
	add r9, rcx
	and rax, r9
```
## `round_to_multiple_i128_pow2`, `round_to_multiple_i128_unb_pow2`
```asm
	push rsi
	mov r9, rcx
	mov eax, 1
	mov ecx, r8d
	xor r11d, r11d
	xor r10d, r10d
	mov rsi, rdx
	shld r11, rax, cl
	shlx rax, rax, r8
	test r8b, 64
	cmovne r11, rax
	cmovne rax, r10
	shr rsi, 63
	mov r8, r11
	shld r8, rax, 63
	mov rcx, r11
	shr rcx
	sub r8, rsi
	sbb rcx, 0
	cmovb r8, r10
	cmovb rcx, r10
	add r8, r9
	adc rdx, rcx
	neg rax
	sbb r10, r11
	and rax, r8
	and rdx, r10
	pop rsi
```
## `round_to_multiple_u8_pow2`
```asm
	mov r8d, ecx
	mov al, 1
	mov ecx, edx
	shl al, cl
	mov ecx, eax
	neg al
	shr cl
	add cl, r8b
	and al, cl
```
## `round_to_multiple_u8_unb_pow2`
```asm
	and dl, 7
	mov r8d, ecx
	mov al, 1
	mov ecx, edx
	shl al, cl
	mov ecx, eax
	neg al
	shr cl
	add cl, r8b
	and al, cl
```
## `round_to_multiple_u16_pow2`
```asm
	mov eax, 1
	shlx eax, eax, edx
	movzx edx, ax
	neg eax
	shr edx
	add edx, ecx
	and eax, edx
```
## `round_to_multiple_u16_unb_pow2`
```asm
	and dl, 15
	mov eax, 1
	shlx eax, eax, edx
	mov edx, eax
	shr edx
	neg eax
	add edx, ecx
	and eax, edx
```
## `round_to_multiple_u32_pow2`, `round_to_multiple_u32_unb_pow2`
```asm
	mov eax, 1
	shlx eax, eax, edx
	mov edx, eax
	shr edx
	neg eax
	add edx, ecx
	and eax, edx
```
## `round_to_multiple_u64_pow2`, `round_to_multiple_u64_unb_pow2`
```asm
	mov eax, 1
	shlx rax, rax, rdx
	mov rdx, rax
	shr rdx
	neg rax
	add rdx, rcx
	and rax, rdx
```
## `round_to_multiple_u128_pow2`, `round_to_multiple_u128_unb_pow2`
```asm
	mov r9, rcx
	mov eax, 1
	mov ecx, r8d
	xor r11d, r11d
	xor r10d, r10d
	shld r11, rax, cl
	shlx rax, rax, r8
	test r8b, 64
	cmovne r11, rax
	cmovne rax, r10
	mov r8, r11
	shld r8, rax, 63
	mov rcx, r11
	shr rcx
	add r8, r9
	adc rdx, rcx
	neg rax
	sbb r10, r11
	and rax, r8
	and rdx, r10
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
	cmp dl, 16
	jae .LBB346_1
	mov eax, -1
	shlx edx, eax, edx
	mov eax, edx
	not eax
	add cx, ax
	setno al
	and ecx, edx
	mov edx, ecx
	movzx eax, al
	ret
.LBB346_1:
	test cx, cx
	setle al
	xor ecx, ecx
	mov edx, ecx
	movzx eax, al
```
## `unbounded_ceil_to_multiple_i32_unb_pow2`
```asm
	cmp dl, 32
	jae .LBB347_1
	mov eax, -1
	shlx edx, eax, edx
	mov eax, edx
	not eax
	add ecx, eax
	setno al
	and ecx, edx
	movzx eax, al
	mov edx, ecx
	ret
.LBB347_1:
	test ecx, ecx
	setle al
	xor ecx, ecx
	movzx eax, al
	mov edx, ecx
```
## `unbounded_ceil_to_multiple_i64_unb_pow2`
```asm
	cmp dl, 64
	jae .LBB348_1
	mov rax, -1
	shlx rdx, rax, rdx
	mov rax, rdx
	not rax
	add rcx, rax
	setno al
	and rcx, rdx
	movzx eax, al
	mov rdx, rcx
	ret
.LBB348_1:
	test rcx, rcx
	setle al
	xor ecx, ecx
	movzx eax, al
	mov rdx, rcx
```
## `unbounded_ceil_to_multiple_i128_unb_pow2`
```asm
	push rsi
	mov rax, rcx
	test r9b, r9b
	js .LBB345_4
	mov r10, -1
	xor ecx, ecx
	test r9b, 64
	shlx r11, r10, r9
	mov r9, r11
	cmovne r9, rcx
	cmovne r10, r11
	mov rsi, r9
	not rsi
	mov r11, r10
	not r11
	add rdx, rsi
	adc r8, r11
	jo .LBB345_5
	and rdx, r9
	and r8, r10
	mov rcx, rdx
.LBB345_3:
	mov qword ptr [rax + 16], rcx
	mov ecx, 1
	mov qword ptr [rax + 24], r8
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
	cmp dl, 16
	jae .LBB351_1
	mov eax, -1
	shlx edx, eax, edx
	mov eax, edx
	not eax
	mov r8d, edx
	add eax, ecx
	cmp ax, cx
	setae al
	sub r8d, ecx
	andn edx, r8d, edx
	movzx eax, al
	ret
.LBB351_1:
	test cx, cx
	sete al
	xor edx, edx
	movzx eax, al
```
## `unbounded_ceil_to_multiple_u32_unb_pow2`
```asm
	cmp dl, 32
	jae .LBB352_1
	mov eax, -1
	shlx edx, eax, edx
	mov eax, edx
	not eax
	mov r8d, edx
	add eax, ecx
	cmp eax, ecx
	setae al
	sub r8d, ecx
	andn edx, r8d, edx
	movzx eax, al
	ret
.LBB352_1:
	test ecx, ecx
	sete al
	xor edx, edx
	movzx eax, al
```
## `unbounded_ceil_to_multiple_u64_unb_pow2`
```asm
	cmp dl, 64
	jae .LBB353_1
	mov rax, -1
	shlx rdx, rax, rdx
	mov rax, rdx
	not rax
	mov r8, rdx
	add rax, rcx
	cmp rax, rcx
	setae al
	sub r8, rcx
	andn rdx, r8, rdx
	movzx eax, al
	ret
.LBB353_1:
	test rcx, rcx
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
	xor ecx, ecx
	test r9b, 64
	shlx r11, r10, r9
	mov r9, r11
	cmovne r9, rcx
	cmovne r10, r11
	mov rsi, r9
	not rsi
	mov r11, r10
	not r11
	add rdx, rsi
	adc r8, r11
	jb .LBB350_5
	and rdx, r9
	and r8, r10
	mov rcx, rdx
.LBB350_3:
	mov qword ptr [rax + 16], rcx
	mov ecx, 1
	mov qword ptr [rax + 24], r8
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
	mov eax, r8d
	shl r9b, cl
	sar al, cl
	not r9b
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
	cmp dl, 16
	jae .LBB356_1
	movsx eax, cx
	bzhi ecx, ecx, edx
	sarx eax, eax, edx
	cmp cx, 1
	sbb ax, -1
	ret
.LBB356_1:
	xor eax, eax
	test cx, cx
	setg al
```
## `unbounded_div_ceil_i32_unb_pow2`
```asm
	cmp dl, 32
	jae .LBB357_1
	sarx eax, ecx, edx
	bzhi ecx, ecx, edx
	cmp ecx, 1
	sbb eax, -1
	ret
.LBB357_1:
	xor eax, eax
	test ecx, ecx
	setg al
```
## `unbounded_div_ceil_i64_unb_pow2`
```asm
	cmp dl, 64
	jae .LBB358_1
	sarx rax, rcx, rdx
	bzhi rcx, rcx, rdx
	cmp rcx, 1
	sbb rax, -1
	ret
.LBB358_1:
	xor eax, eax
	test rcx, rcx
	setg al
```
## `unbounded_div_ceil_i128_unb_pow2`
```asm
	push rsi
	mov rax, rdx
	test r8b, r8b
	js .LBB355_1
	mov r9, rcx
	mov r10, rcx
	mov ecx, r8d
	mov rdx, rax
	sar rdx, 63
	xor esi, esi
	shrd r9, rax, cl
	mov rcx, -1
	test r8b, 64
	shlx r11, rcx, r8
	sarx r8, rax, r8
	cmove rsi, r11
	cmovne rcx, r11
	cmovne r9, r8
	cmove rdx, r8
	andn rcx, rcx, rax
	andn r8, rsi, r10
	xor eax, eax
	or r8, rcx
	setne al
	add rax, r9
	adc rdx, 0
	pop rsi
	ret
.LBB355_1:
	xor edx, edx
	neg rcx
	mov ecx, 0
	sbb rcx, rax
	setl al
	movzx eax, al
	pop rsi
```
## `unbounded_div_ceil_u8_unb_pow2`
```asm
	mov r8d, ecx
	cmp dl, 8
	jae .LBB364_1
	mov r9b, -1
	mov ecx, edx
	mov eax, r8d
	shl r9b, cl
	shr al, cl
	not r9b
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
	cmp dl, 16
	jae .LBB361_1
	movzx eax, cx
	bzhi ecx, ecx, edx
	shrx eax, eax, edx
	cmp cx, 1
	sbb ax, -1
	ret
.LBB361_1:
	xor eax, eax
	test cx, cx
	setne al
```
## `unbounded_div_ceil_u32_unb_pow2`
```asm
	cmp dl, 32
	jae .LBB362_1
	shrx eax, ecx, edx
	bzhi ecx, ecx, edx
	cmp ecx, 1
	sbb eax, -1
	ret
.LBB362_1:
	xor eax, eax
	test ecx, ecx
	setne al
```
## `unbounded_div_ceil_u64_unb_pow2`
```asm
	cmp dl, 64
	jae .LBB363_1
	shrx rax, rcx, rdx
	bzhi rcx, rcx, rdx
	cmp rcx, 1
	sbb rax, -1
	ret
.LBB363_1:
	xor eax, eax
	test rcx, rcx
	setne al
```
## `unbounded_div_ceil_u128_unb_pow2`
```asm
	test r8b, r8b
	js .LBB360_1
	mov r9, rcx
	mov rax, rcx
	mov ecx, r8d
	xor r11d, r11d
	shrd r9, rdx, cl
	mov rcx, -1
	test r8b, 64
	shlx r10, rcx, r8
	shrx r8, rdx, r8
	cmovne rcx, r10
	cmovne r10, r11
	cmovne r9, r8
	cmovne r8, r11
	andn rcx, rcx, rdx
	andn r10, r10, rax
	xor eax, eax
	mov rdx, r8
	or r10, rcx
	setne al
	add rax, r9
	adc rdx, 0
	ret
.LBB360_1:
	xor eax, eax
	or rcx, rdx
	setne al
	xor edx, edx
```
## `unbounded_div_floor_i8_unb_pow2`
```asm
	movzx edx, dl
	mov eax, ecx
	mov ecx, 7
	cmp dl, 7
	cmovb ecx, edx
	sar al, cl
```
## `unbounded_div_floor_i16_unb_pow2`
```asm
	movsx eax, cx
	movzx ecx, dl
	mov edx, 15
	cmp cl, 15
	cmovb edx, ecx
	sarx eax, eax, edx
```
## `unbounded_div_floor_i32_unb_pow2`
```asm
	movzx eax, dl
	mov edx, 31
	cmp al, 31
	cmovb edx, eax
	sarx eax, ecx, edx
```
## `unbounded_div_floor_i64_unb_pow2`
```asm
	movzx eax, dl
	mov edx, 63
	cmp al, 63
	cmovb edx, eax
	sarx rax, rcx, rdx
```
## `unbounded_div_floor_i128_unb_pow2`
```asm
	mov rax, rcx
	test r8b, r8b
	movzx r8d, r8b
	mov ecx, 127
	cmovns ecx, r8d
	shrd rax, rdx, cl
	sarx r8, rdx, rcx
	sar rdx, 63
	test cl, 64
	cmove rdx, r8
	cmovne rax, r8
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
	movzx eax, cx
	xor ecx, ecx
	cmp dl, 16
	shrx eax, eax, edx
	cmovae eax, ecx
```
## `unbounded_div_floor_u32_unb_pow2`, `unbounded_div_u32_unb_pow2`
```asm
	xor r8d, r8d
	cmp dl, 32
	shrx eax, ecx, edx
	cmovae eax, r8d
```
## `unbounded_div_floor_u64_unb_pow2`, `unbounded_div_u64_unb_pow2`
```asm
	xor eax, eax
	cmp dl, 64
	shrx rcx, rcx, rdx
	cmovb rax, rcx
```
## `unbounded_div_floor_u128_unb_pow2`, `unbounded_div_u128_unb_pow2`
```asm
	mov rax, rcx
	mov ecx, r8d
	shrd rax, rdx, cl
	xor ecx, ecx
	shrx rdx, rdx, r8
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
	mov eax, r8d
	shl r9b, cl
	not r9b
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
	movsx eax, cx
	sar eax, 15
	bzhi eax, eax, edx
	add eax, ecx
	cwde
	sarx eax, eax, edx
	ret
.LBB376_1:
	xor eax, eax
```
## `unbounded_div_i32_unb_pow2`
```asm
	xor eax, eax
	cmp dl, 31
	ja .LBB377_2
	mov eax, ecx
	sar eax, 31
	bzhi eax, eax, edx
	add eax, ecx
	sarx eax, eax, edx
.LBB377_2:
```
## `unbounded_div_i64_unb_pow2`
```asm
	cmp dl, 63
	ja .LBB378_1
	mov rax, rcx
	sar rax, 63
	bzhi rax, rax, rdx
	add rax, rcx
	sarx rax, rax, rdx
	ret
.LBB378_1:
	xor eax, eax
```
## `unbounded_div_i128_unb_pow2`
```asm
	test r8b, r8b
	js .LBB375_1
	mov rax, -1
	xor r10d, r10d
	test r8b, 64
	mov r11, rdx
	shlx r9, rax, r8
	cmove r10, r9
	cmovne rax, r9
	mov r9, rdx
	sar r9, 63
	andn rdx, rax, r9
	andn rax, r10, r9
	add rax, rcx
	mov ecx, r8d
	adc rdx, r11
	shrd rax, rdx, cl
	test r8b, 64
	sarx rcx, rdx, r8
	cmovne rax, rcx
	sar rdx, 63
	test r8b, 64
	cmove rdx, rcx
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
	mov ecx, edx
	mov r9b, -1
	mov eax, r8d
	movzx edx, dl
	shl r9b, cl
	sar al, cl
	xor ecx, ecx
	not r9b
	and r9b, r8b
	shr r8b, 7
	sub r9b, r8b
	movzx r8d, r9b
	cmovb r8d, ecx
	add r8b, r8b
	movzx ecx, r8b
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
	cmp dl, 15
	ja .LBB381_2
	movzx r8d, cx
	movsx eax, cx
	bzhi ecx, ecx, edx
	xor r9d, r9d
	shr r8d, 15
	sarx eax, eax, edx
	movzx edx, dl
	sub cx, r8w
	cmovb ecx, r9d
	add ecx, ecx
	bt ecx, edx
	adc ax, 0
	ret
.LBB381_2:
	xor eax, eax
	neg cx
	jno .LBB381_4
	cmp dl, 16
	sete al
	neg eax
.LBB381_4:
```
## `unbounded_div_round_i32_unb_pow2`
```asm
	cmp dl, 31
	ja .LBB382_2
	sarx eax, ecx, edx
	bzhi r8d, ecx, edx
	shr ecx, 31
	xor r9d, r9d
	sub r8d, ecx
	movzx ecx, dl
	cmovb r8d, r9d
	add r8d, r8d
	bt r8d, ecx
	adc eax, 0
	ret
.LBB382_2:
	xor eax, eax
	neg ecx
	jno .LBB382_4
	xor eax, eax
	cmp dl, 32
	sete al
	neg eax
.LBB382_4:
```
## `unbounded_div_round_i64_unb_pow2`
```asm
	cmp dl, 63
	ja .LBB383_2
	sarx rax, rcx, rdx
	bzhi r8, rcx, rdx
	shr rcx, 63
	xor r9d, r9d
	sub r8, rcx
	movzx ecx, dl
	cmovae r9, r8
	add r9, r9
	bt r9, rcx
	adc rax, 0
	ret
.LBB383_2:
	xor eax, eax
	neg rcx
	jno .LBB383_4
	cmp dl, 64
	sete al
	neg rax
.LBB383_4:
```
## `unbounded_div_round_i128_unb_pow2`
```asm
	test r8b, r8b
	js .LBB380_2
	mov rax, -1
	xor r10d, r10d
	test r8b, 64
	mov r11, rdx
	shlx r9, rax, r8
	cmovne rax, r9
	cmovne r9, r10
	shr r11, 63
	andn r9, r9, rcx
	andn rax, rax, rdx
	sub r9, r11
	sarx r11, rdx, r8
	sbb rax, 0
	cmovb rax, r10
	cmovb r9, r10
	mov r10, rcx
	mov ecx, r8d
	shld rax, r9, 1
	add r9, r9
	shrd r9, rax, cl
	test r8b, 64
	shrx rax, rax, r8
	cmove rax, r9
	mov r9, rdx
	sar r9, 63
	test r8b, 64
	cmove r9, r11
	shrd r10, rdx, cl
	test r8b, 64
	mov rdx, r9
	cmovne r10, r11
	and eax, 1
	add rax, r10
	adc rdx, 0
	ret
.LBB380_2:
	movabs rax, -9223372036854775808
	xor rdx, rax
	xor eax, eax
	or rcx, rdx
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
	mov eax, 1
	movzx ecx, cx
	shlx eax, eax, edx
	movzx r8d, ax
	shrx eax, ecx, edx
	shr r8d
	and ecx, r8d
	cmp cx, 1
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
	mov eax, 1
	shlx r8d, eax, edx
	shrx eax, ecx, edx
	shr r8d
	and r8d, ecx
	cmp r8d, 1
	sbb eax, -1
.LBB387_2:
```
## `unbounded_div_round_u64_unb_pow2`
```asm
	cmp dl, 63
	ja .LBB388_1
	mov eax, 1
	shlx r8, rax, rdx
	shrx rax, rcx, rdx
	shr r8
	and r8, rcx
	cmp r8, 1
	sbb rax, -1
	ret
.LBB388_1:
	xor eax, eax
```
## `unbounded_div_round_u128_unb_pow2`
```asm
	push r14
	push rsi
	push rdi
	push rbx
	test r8b, r8b
	js .LBB385_1
	xor r10d, r10d
	shrx rax, rdx, r8
	test r8b, 64
	mov r9, rcx
	mov rsi, rcx
	mov ecx, r8d
	mov r11, rdx
	mov edi, 1
	mov rdx, rax
	cmovne rdx, r10
	shrd r9, r11, cl
	test r8b, 64
	shlx rbx, rdi, r8
	mov r14, rbx
	cmovne r14, r10
	cmovne r9, rax
	shld r10, rdi, cl
	test r8b, 64
	cmovne r10, rbx
	xor eax, eax
	shrd r14, r10, 1
	shr r10
	and r10, r11
	and r14, rsi
	or r14, r10
	setne al
	add rax, r9
	adc rdx, 0
	jmp .LBB385_3
.LBB385_1:
	xor eax, eax
	xor edx, edx
.LBB385_3:
	pop rbx
	pop rdi
	pop rsi
	pop r14
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
	xor r9d, r9d
	cmp dl, 8
	movzx edx, r8b
	setb al
	cmovae edx, r9d
	or al, cl
```
## `unbounded_floor_to_multiple_i16_unb_pow2`
```asm
	cmp dl, 16
	jae .LBB396_1
	movzx eax, cx
	shrx eax, eax, edx
	shlx edx, eax, edx
	mov ax, 1
	ret
.LBB396_1:
	not ecx
	xor edx, edx
	movzx eax, cx
	shr eax, 15
```
## `unbounded_floor_to_multiple_i32_unb_pow2`
```asm
	mov eax, ecx
	cmp dl, 32
	jae .LBB397_1
	shrx eax, eax, edx
	shlx edx, eax, edx
	mov eax, 1
	ret
.LBB397_1:
	not eax
	xor edx, edx
	shr eax, 31
```
## `unbounded_floor_to_multiple_i64_unb_pow2`
```asm
	mov rax, rcx
	cmp dl, 64
	jae .LBB398_1
	shrx rax, rax, rdx
	shlx rdx, rax, rdx
	mov eax, 1
	ret
.LBB398_1:
	not rax
	xor edx, edx
	shr rax, 63
```
## `unbounded_floor_to_multiple_i128_unb_pow2`
```asm
	mov rax, rcx
	test r9b, r9b
	js .LBB395_1
	mov rcx, -1
	xor r11d, r11d
	test r9b, 64
	shlx r10, rcx, r9
	cmovne rcx, r10
	cmove r11, r10
	and r11, rdx
	and rcx, r8
	mov qword ptr [rax + 24], rcx
	mov qword ptr [rax + 16], r11
	mov qword ptr [rax + 8], 0
	mov qword ptr [rax], 1
	ret
.LBB395_1:
	test r8, r8
	js .LBB395_2
	vpmovsxbq xmm0, word ptr [rip + .LCPI395_1]
	vmovups ymmword ptr [rax], ymm0
	vzeroupper
	ret
.LBB395_2:
	vxorps xmm0, xmm0, xmm0
	vmovaps xmmword ptr [rax], xmm0
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
	movzx eax, cx
	xor ecx, ecx
	cmp dl, 16
	shrx eax, eax, edx
	shlx eax, eax, edx
	cmovae eax, ecx
```
## `unbounded_floor_to_multiple_u32_unb_pow2`
```asm
	shrx eax, ecx, edx
	xor r8d, r8d
	cmp dl, 32
	shlx eax, eax, edx
	cmovae eax, r8d
```
## `unbounded_floor_to_multiple_u64_unb_pow2`
```asm
	shrx rcx, rcx, rdx
	xor eax, eax
	cmp dl, 64
	shlx rcx, rcx, rdx
	cmovb rax, rcx
```
## `unbounded_floor_to_multiple_u128_unb_pow2`
```asm
	mov r9, -1
	xor r10d, r10d
	test r8b, 64
	shlx rax, r9, r8
	cmovne r9, rax
	cmovne rax, r10
	and rdx, r9
	and rax, rcx
	test r8b, r8b
	cmovs rax, r10
	cmovs rdx, r10
```
## `unbounded_is_multiple_of_i8_unb_pow2`, `unbounded_is_multiple_of_u8_unb_pow2`
```asm
	test cl, cl
	movzx eax, cl
	sete r8b
	tzcnt eax, eax
	cmp al, dl
	setae al
	or al, r8b
```
## `unbounded_is_multiple_of_i16_unb_pow2`, `unbounded_is_multiple_of_u16_unb_pow2`
```asm
	test cx, cx
	sete r8b
	tzcnt eax, ecx
	movzx ecx, dl
	cmp ax, cx
	setae al
	or al, r8b
```
## `unbounded_is_multiple_of_i32_unb_pow2`, `unbounded_is_multiple_of_u32_unb_pow2`
```asm
	test ecx, ecx
	movzx edx, dl
	sete r8b
	tzcnt eax, ecx
	cmp eax, edx
	setae al
	or al, r8b
```
## `unbounded_is_multiple_of_i64_unb_pow2`, `unbounded_is_multiple_of_u64_unb_pow2`
```asm
	test rcx, rcx
	sete r8b
	tzcnt rax, rcx
	movzx ecx, dl
	cmp rax, rcx
	setae al
	or al, r8b
```
## `unbounded_is_multiple_of_i128_unb_pow2`, `unbounded_is_multiple_of_u128_unb_pow2`
```asm
	mov rax, rcx
	or rax, rdx
	sete r9b
	tzcnt rdx, rdx
	tzcnt rax, rcx
	add rdx, 64
	test rcx, rcx
	cmovne rdx, rax
	movzx eax, r8b
	cmp edx, eax
	setae al
	or al, r9b
```
## `unbounded_rem_i8_unb_pow2`
```asm
	mov eax, ecx
	cmp dl, 7
	ja .LBB419_2
	mov ecx, edx
	mov r8b, -1
	mov edx, eax
	shl r8b, cl
	sar dl, 7
	mov ecx, r8d
	not cl
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
	movsx r8d, ax
	mov ecx, -1
	sar r8d, 15
	shlx ecx, ecx, edx
	bzhi edx, r8d, edx
	add edx, eax
	and edx, ecx
	sub eax, edx
.LBB416_2:
```
## `unbounded_rem_i32_unb_pow2`
```asm
	mov eax, ecx
	cmp dl, 31
	ja .LBB417_2
	mov ecx, -1
	mov r8d, eax
	sar r8d, 31
	shlx ecx, ecx, edx
	bzhi edx, r8d, edx
	add edx, eax
	and edx, ecx
	sub eax, edx
.LBB417_2:
```
## `unbounded_rem_i64_unb_pow2`
```asm
	mov rax, rcx
	cmp dl, 63
	ja .LBB418_2
	mov rcx, -1
	mov r8, rax
	sar r8, 63
	shlx rcx, rcx, rdx
	bzhi rdx, r8, rdx
	add rdx, rax
	and rdx, rcx
	sub rax, rdx
.LBB418_2:
```
## `unbounded_rem_i128_unb_pow2`
```asm
	mov rax, rcx
	test r8b, r8b
	js .LBB415_2
	mov rcx, -1
	xor r10d, r10d
	test r8b, 64
	shlx r9, rcx, r8
	mov r8, rdx
	cmove r10, r9
	cmovne rcx, r9
	sar r8, 63
	andn r9, rcx, r8
	andn r8, r10, r8
	add r8, rax
	adc r9, rdx
	and r8, r10
	and r9, rcx
	sub rax, r8
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
	mov eax, -1
	cmp dl, 16
	shlx edx, eax, edx
	mov eax, 65535
	not edx
	cmovb eax, edx
	and eax, ecx
```
## `unbounded_rem_u32_unb_pow2`
```asm
	mov r8d, -1
	cmp dl, 32
	shlx eax, r8d, edx
	not eax
	cmovae eax, r8d
	and eax, ecx
```
## `unbounded_rem_u64_unb_pow2`
```asm
	mov r8, -1
	cmp dl, 64
	shlx rax, r8, rdx
	not rax
	cmovae rax, r8
	and rax, rcx
```
## `unbounded_rem_u128_unb_pow2`
```asm
	mov r9, -1
	xor eax, eax
	test r8b, 64
	mov r11, -1
	shlx r10, r9, r8
	cmovne r11, r10
	cmove rax, r10
	test r8b, r8b
	not rax
	not r11
	cmovs r11, r9
	cmovs rax, r9
	and rax, rcx
	and rdx, r11
```
## `unbounded_round_to_multiple_i8_unb_pow2`
```asm
	mov eax, ecx
	cmp dl, 7
	ja .LBB429_5
	mov ecx, edx
	mov r8b, 1
	mov edx, eax
	xor r9d, r9d
	shl r8b, cl
	shr dl, 7
	mov ecx, r8d
	shr cl
	sub cl, dl
	movzx ecx, cl
	cmovb ecx, r9d
	lea edx, [rax + rcx]
	add al, cl
	seto al
	jo .LBB429_2
	neg r8b
	xor al, 1
	and dl, r8b
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
	cmp dl, 15
	ja .LBB426_5
	mov eax, 1
	movzx r8d, cx
	xor r9d, r9d
	shlx edx, eax, edx
	shr r8d, 15
	movzx eax, dx
	shr eax
	sub ax, r8w
	cmovb eax, r9d
	lea r8d, [rcx + rax]
	add cx, ax
	seto al
	jo .LBB426_2
	neg edx
	xor al, 1
	and edx, r8d
	movzx eax, al
	ret
.LBB426_5:
	neg cx
	setno cl
	cmp dl, 16
	setne al
	xor edx, edx
	or al, cl
	movzx eax, al
	ret
.LBB426_2:
	xor al, 1
	movzx eax, al
```
## `unbounded_round_to_multiple_i32_unb_pow2`
```asm
	cmp dl, 31
	ja .LBB427_5
	mov eax, 1
	xor r9d, r9d
	shlx r8d, eax, edx
	mov edx, ecx
	shr edx, 31
	mov eax, r8d
	shr eax
	sub eax, edx
	cmovb eax, r9d
	lea edx, [rcx + rax]
	add eax, ecx
	seto al
	jo .LBB427_2
	xor al, 1
	neg r8d
	and edx, r8d
	movzx eax, al
	ret
.LBB427_5:
	neg ecx
	setno cl
	cmp dl, 32
	setne al
	xor edx, edx
	or al, cl
	movzx eax, al
	ret
.LBB427_2:
	xor al, 1
	movzx eax, al
```
## `unbounded_round_to_multiple_i64_unb_pow2`
```asm
	cmp dl, 63
	ja .LBB428_5
	mov eax, 1
	xor r9d, r9d
	shlx r8, rax, rdx
	mov rdx, rcx
	shr rdx, 63
	mov rax, r8
	shr rax
	sub rax, rdx
	cmovae r9, rax
	lea rdx, [rcx + r9]
	add r9, rcx
	seto al
	jo .LBB428_2
	xor al, 1
	neg r8
	and rdx, r8
	movzx eax, al
	ret
.LBB428_5:
	neg rcx
	setno cl
	cmp dl, 64
	setne al
	xor edx, edx
	or al, cl
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
	mov r11d, 1
	mov ecx, r9d
	xor esi, esi
	xor r10d, r10d
	mov rdi, r8
	shld rsi, r11, cl
	shlx r11, r11, r9
	test r9b, 64
	cmovne rsi, r11
	cmovne r11, r10
	shr rdi, 63
	mov r9, rsi
	shld r9, r11, 63
	mov rcx, rsi
	shr rcx
	sub r9, rdi
	sbb rcx, 0
	cmovb r9, r10
	cmovb rcx, r10
	add rdx, r9
	adc rcx, r8
	jo .LBB425_5
	xor r8d, r8d
	neg r11
	sbb r8, rsi
	and rdx, r11
	and rcx, r8
	mov r10, rdx
	jmp .LBB425_4
.LBB425_3:
	movabs rcx, -9223372036854775808
	xor r8, rcx
	mov ecx, 0
	or rdx, r8
	sete dl
	neg r9b
	seto r8b
	xor r10d, r10d
	test dl, r8b
	jne .LBB425_5
.LBB425_4:
	mov qword ptr [rax + 16], r10
	mov r10d, 1
	mov qword ptr [rax + 24], rcx
.LBB425_5:
	mov qword ptr [rax], r10
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
	xor edx, edx
	or al, cl
	ret
.LBB434_3:
```
## `unbounded_round_to_multiple_u16_unb_pow2`
```asm
	cmp dl, 15
	ja .LBB431_4
	mov eax, 1
	shlx r8d, eax, edx
	movzx eax, r8w
	shr eax
	lea edx, [rcx + rax]
	cmp dx, cx
	setae al
	jb .LBB431_3
	neg r8d
	movzx eax, al
	and edx, r8d
	ret
.LBB431_4:
	test cx, cx
	setns cl
	cmp dl, 16
	setne al
	xor edx, edx
	or al, cl
	movzx eax, al
	ret
.LBB431_3:
	movzx eax, al
```
## `unbounded_round_to_multiple_u32_unb_pow2`
```asm
	cmp dl, 31
	ja .LBB432_4
	mov eax, 1
	shlx r8d, eax, edx
	mov edx, r8d
	shr edx
	add edx, ecx
	cmp edx, ecx
	setae al
	jb .LBB432_3
	neg r8d
	movzx eax, al
	and edx, r8d
	ret
.LBB432_4:
	test ecx, ecx
	setns cl
	cmp dl, 32
	setne al
	xor edx, edx
	or al, cl
	movzx eax, al
	ret
.LBB432_3:
	movzx eax, al
```
## `unbounded_round_to_multiple_u64_unb_pow2`
```asm
	cmp dl, 63
	ja .LBB433_4
	mov eax, 1
	shlx r8, rax, rdx
	mov rdx, r8
	shr rdx
	add rdx, rcx
	cmp rdx, rcx
	setae al
	jb .LBB433_3
	neg r8
	movzx eax, al
	and rdx, r8
	ret
.LBB433_4:
	test rcx, rcx
	setns cl
	cmp dl, 64
	setne al
	xor edx, edx
	or al, cl
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
	mov ecx, r9d
	xor esi, esi
	xor r10d, r10d
	shld rsi, r11, cl
	shlx r11, r11, r9
	test r9b, 64
	cmovne rsi, r11
	cmovne r11, r10
	mov r9, rsi
	shld r9, r11, 63
	mov rcx, rsi
	shr rcx
	add r9, rdx
	adc rcx, r8
	jb .LBB430_5
	xor edx, edx
	neg r11
	sbb rdx, rsi
	and r9, r11
	and rcx, rdx
	mov r10, r9
	jmp .LBB430_4
.LBB430_3:
	test r8, r8
	mov ecx, 0
	sets dl
	neg r9b
	seto r8b
	xor r10d, r10d
	test dl, r8b
	jne .LBB430_5
.LBB430_4:
	mov qword ptr [rax + 16], r10
	mov r10d, 1
	mov qword ptr [rax + 24], rcx
.LBB430_5:
	mov qword ptr [rax], r10
	mov qword ptr [rax + 8], 0
	pop rsi
```
