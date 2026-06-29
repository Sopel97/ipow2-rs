## `ceil_to_multiple_i8_pow2`, `ceil_to_multiple_i16_pow2`, `ceil_to_multiple_i32_pow2`, `ceil_to_multiple_i32_unb_pow2`, `ceil_to_multiple_u8_pow2`, `ceil_to_multiple_u16_pow2`, `ceil_to_multiple_u32_pow2`, `ceil_to_multiple_u32_unb_pow2`
```asm
	mov w8, #-1
	lsl w8, w8, w1
	sub w9, w8, w0
	bic w0, w8, w9
```
## `ceil_to_multiple_i8_unb_pow2`, `ceil_to_multiple_u8_unb_pow2`
```asm
	and x8, x1, #0x7
	mov w9, #-1
	lsl w8, w9, w8
	sub w9, w8, w0
	bic w0, w8, w9
```
## `ceil_to_multiple_i16_unb_pow2`, `ceil_to_multiple_u16_unb_pow2`
```asm
	and x8, x1, #0xf
	mov w9, #-1
	lsl w8, w9, w8
	sub w9, w8, w0
	bic w0, w8, w9
```
## `ceil_to_multiple_i64_pow2`, `ceil_to_multiple_i64_unb_pow2`, `ceil_to_multiple_u64_pow2`, `ceil_to_multiple_u64_unb_pow2`
```asm
	mov x8, #-1
	lsl x8, x8, x1
	sub x9, x8, x0
	bic x0, x8, x9
```
## `ceil_to_multiple_i128_pow2`, `ceil_to_multiple_u128_pow2`
```asm
	mov x8, #-1
	lsl x8, x8, x2
	mvn w9, w2
	mov x10, #9223372036854775807
	lsr x9, x10, x9
	orr x9, x8, x9
	and x10, x2, #0xff
	tst x10, #0x40
	csel x9, x8, x9, ne
	csel x8, xzr, x8, ne
	subs x10, x8, x0
	bic x0, x8, x10
	sbc x8, x9, x1
	bic x1, x9, x8
```
## `ceil_to_multiple_i128_unb_pow2`, `ceil_to_multiple_u128_unb_pow2`
```asm
	mov x8, #-1
	lsl x8, x8, x2
	mvn w9, w2
	mov x10, #9223372036854775807
	lsr x9, x10, x9
	orr x9, x8, x9
	and x10, x2, #0x7f
	tst x10, #0x40
	csel x9, x8, x9, ne
	csel x8, xzr, x8, ne
	subs x10, x8, x0
	bic x0, x8, x10
	sbc x8, x9, x1
	bic x1, x9, x8
```
## `checked_ceil_to_multiple_i8_pow2`
```asm
	sxtb w8, w0
	mov w9, #-1
	lsl w9, w9, w1
	mvn w10, w9
	add w8, w8, w10, sxtb
	cmp w8, w8, sxtb
	cset w0, eq
	and w1, w8, w9
```
## `checked_ceil_to_multiple_i8_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-1
	lsl w9, w9, w1
	mvn w10, w9
	sxtb w11, w0
	add w10, w11, w10, sxtb
	cmp w10, w10, sxtb
	cset w11, eq
	and w9, w10, w9
	cmp w8, #7
	csel w1, w8, w9, hi
	csel w0, wzr, w11, hi
```
## `checked_ceil_to_multiple_i16_pow2`
```asm
	sxth w8, w0
	mov w9, #-1
	lsl w9, w9, w1
	mvn w10, w9
	add w8, w8, w10, sxth
	cmp w8, w8, sxth
	cset w0, eq
	and w1, w8, w9
```
## `checked_ceil_to_multiple_i16_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-1
	lsl w9, w9, w1
	mvn w10, w9
	sxth w11, w0
	add w10, w11, w10, sxth
	cmp w10, w10, sxth
	cset w11, eq
	and w9, w10, w9
	cmp w8, #15
	csel w1, w8, w9, hi
	csel w0, wzr, w11, hi
```
## `checked_ceil_to_multiple_i32_pow2`
```asm
	mov w8, #-1
	lsl w8, w8, w1
	mvn w9, w8
	adds w9, w0, w9
	and w1, w9, w8
	cset w0, vc
```
## `checked_ceil_to_multiple_i32_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-1
	lsl w9, w9, w1
	mvn w10, w9
	adds w10, w0, w10
	and w9, w10, w9
	cset w10, vc
	cmp w8, #31
	csel w1, w8, w9, hi
	csel w0, wzr, w10, hi
```
## `checked_ceil_to_multiple_i64_pow2`
```asm
	mov x8, #-1
	lsl x8, x8, x1
	mvn x9, x8
	adds x9, x0, x9
	and x1, x9, x8
	cset w0, vc
```
## `checked_ceil_to_multiple_i64_unb_pow2`
```asm
	and w8, w1, #0xff
	mov x9, #-1
	lsl x9, x9, x1
	mvn x10, x9
	adds x10, x0, x10
	and x9, x10, x9
	cset w10, vc
	cmp w8, #63
	csel x1, x8, x9, hi
	csel x0, xzr, x10, hi
```
## `checked_ceil_to_multiple_i128_pow2`
```asm
	mov x9, #-1
	lsl x10, x9, x2
	and x9, x2, #0xff
	tst x9, #0x40
	csel x9, xzr, x10, ne
	mvn w11, w2
	mov x12, #9223372036854775807
	lsr x11, x12, x11
	orr x11, x10, x11
	csel x10, x10, x11, ne
	mvn x12, x10
	mvn x11, x9
	adds x11, x0, x11
	adcs x12, x1, x12
	cset w13, vs
	tbnz w13, #0, .LBB25_2
	and x10, x12, x10
	and x9, x11, x9
	stp x9, x10, [x8, #16]
	mov w9, #1
	stp x9, xzr, [x8]
	ret
.LBB25_2:
	mov x9, #0
	stp x9, xzr, [x8]
```
## `checked_ceil_to_multiple_i128_unb_pow2`
```asm
	tbnz w2, #7, .LBB26_3
	mov x9, #-1
	lsl x10, x9, x2
	and x9, x2, #0xff
	tst x9, #0x40
	csel x9, xzr, x10, ne
	mvn w11, w2
	mov x12, #9223372036854775807
	lsr x11, x12, x11
	orr x11, x10, x11
	csel x10, x10, x11, ne
	mvn x12, x10
	mvn x11, x9
	adds x11, x0, x11
	adcs x12, x1, x12
	cset w13, vs
	tbnz w13, #0, .LBB26_3
	and x10, x12, x10
	and x9, x11, x9
	stp x9, x10, [x8, #16]
	mov w9, #1
	stp x9, xzr, [x8]
	ret
.LBB26_3:
	mov x9, #0
	stp x9, xzr, [x8]
```
## `checked_ceil_to_multiple_u8_pow2`
```asm
	mov w8, #-1
	lsl w8, w8, w1
	mov w9, #255
	sub w10, w8, w0
	bic w9, w9, w10
	cmp w9, w0, uxtb
	cset w0, hs
	bic w1, w8, w10
```
## `checked_ceil_to_multiple_u8_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-1
	lsl w9, w9, w1
	mov w10, #255
	sub w11, w9, w0
	bic w10, w10, w11
	cmp w10, w0, uxtb
	cset w10, hs
	bic w9, w9, w11
	cmp w8, #7
	csel w1, w8, w9, hi
	csel w0, wzr, w10, hi
```
## `checked_ceil_to_multiple_u16_pow2`
```asm
	mov w8, #-1
	lsl w8, w8, w1
	mov w9, #65535
	sub w10, w8, w0
	bic w9, w9, w10
	cmp w9, w0, uxth
	cset w0, hs
	bic w1, w8, w10
```
## `checked_ceil_to_multiple_u16_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-1
	lsl w9, w9, w1
	mov w10, #65535
	sub w11, w9, w0
	bic w10, w10, w11
	cmp w10, w0, uxth
	cset w10, hs
	bic w9, w9, w11
	cmp w8, #15
	csel w1, w8, w9, hi
	csel w0, wzr, w10, hi
```
## `checked_ceil_to_multiple_u32_pow2`
```asm
	mov w8, #-1
	lsl w9, w8, w1
	mvn w8, w9
	add w8, w0, w8
	cmp w8, w0
	cset w8, hs
	sub w10, w9, w0
	bic w1, w9, w10
	mov x0, x8
```
## `checked_ceil_to_multiple_u32_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-1
	lsl w9, w9, w1
	mvn w10, w9
	add w10, w0, w10
	cmp w10, w0
	cset w10, hs
	sub w11, w9, w0
	bic w9, w9, w11
	cmp w8, #31
	csel w1, w8, w9, hi
	csel w0, wzr, w10, hi
```
## `checked_ceil_to_multiple_u64_pow2`
```asm
	mov x8, #-1
	lsl x9, x8, x1
	mvn x8, x9
	add x8, x0, x8
	cmp x8, x0
	cset w8, hs
	sub x10, x9, x0
	bic x1, x9, x10
	mov x0, x8
```
## `checked_ceil_to_multiple_u64_unb_pow2`
```asm
	and w8, w1, #0xff
	cmp w8, #63
	b.hi .LBB42_2
	mov x8, #-1
	lsl x8, x8, x1
	mvn x9, x8
	add x9, x0, x9
	cmp x9, x0
	mov x9, x0
	cset w0, hs
	sub x9, x8, x9
	bic x1, x8, x9
	ret
.LBB42_2:
	mov x0, #0
```
## `checked_ceil_to_multiple_u128_pow2`
```asm
	mov x9, #-1
	lsl x10, x9, x2
	and x9, x2, #0xff
	tst x9, #0x40
	csel x9, xzr, x10, ne
	mvn w11, w2
	mov x12, #9223372036854775807
	lsr x11, x12, x11
	orr x11, x10, x11
	csel x10, x10, x11, ne
	mvn x12, x10
	mvn x11, x9
	adds x11, x0, x11
	adcs x12, x1, x12
	cset w13, hs
	tbnz w13, #0, .LBB35_2
	and x10, x12, x10
	and x9, x11, x9
	stp x9, x10, [x8, #16]
	mov w9, #1
	stp x9, xzr, [x8]
	ret
.LBB35_2:
	mov x9, #0
	stp x9, xzr, [x8]
```
## `checked_ceil_to_multiple_u128_unb_pow2`
```asm
	tbnz w2, #7, .LBB36_3
	mov x9, #-1
	lsl x10, x9, x2
	and x9, x2, #0xff
	tst x9, #0x40
	csel x9, xzr, x10, ne
	mvn w11, w2
	mov x12, #9223372036854775807
	lsr x11, x12, x11
	orr x11, x10, x11
	csel x10, x10, x11, ne
	mvn x12, x10
	mvn x11, x9
	adds x11, x0, x11
	adcs x12, x1, x12
	cset w13, hs
	tbnz w13, #0, .LBB36_3
	and x10, x12, x10
	and x9, x11, x9
	stp x9, x10, [x8, #16]
	mov w9, #1
	stp x9, xzr, [x8]
	ret
.LBB36_3:
	mov x9, #0
	stp x9, xzr, [x8]
```
## `checked_div_ceil_i8_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-1
	lsl w9, w9, w1
	sxtb w10, w0
	asr w10, w10, w1
	bic w9, w0, w9
	tst w9, #0xff
	cinc w9, w10, ne
	cmp w8, #7
	csel w8, w8, w9, hi
	tst w1, #0xf8
	cset w0, eq
	mov x1, x8
```
## `checked_div_ceil_i16_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-1
	lsl w9, w9, w1
	sxth w10, w0
	asr w10, w10, w1
	bic w9, w0, w9
	tst w9, #0xffff
	cinc w9, w10, ne
	cmp w8, #15
	csel w1, w8, w9, hi
	cset w0, ls
```
## `checked_div_ceil_i32_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-1
	lsl w9, w9, w1
	asr w10, w0, w1
	bics wzr, w0, w9
	cinc w9, w10, ne
	cmp w8, #31
	csel w1, w8, w9, hi
	cset w0, ls
```
## `checked_div_ceil_i64_unb_pow2`
```asm
	and w8, w1, #0xff
	mov x9, #-1
	lsl x9, x9, x1
	asr x10, x0, x1
	bics xzr, x0, x9
	cinc x9, x10, ne
	cmp w8, #63
	csel x1, x8, x9, hi
	cset x0, ls
```
## `checked_div_ceil_i128_unb_pow2`
```asm
	tbnz w2, #7, .LBB45_2
	mov x9, #-1
	lsl x9, x9, x2
	mvn w10, w2
	mov x11, #9223372036854775807
	lsr x11, x11, x10
	orr x11, x9, x11
	and x12, x2, #0xff
	tst x12, #0x40
	csel x11, x9, x11, ne
	csel x9, xzr, x9, ne
	asr x12, x1, x2
	asr x13, x1, #63
	csel x13, x13, x12, ne
	lsr x14, x0, x2
	lsl x15, x1, #1
	lsl x10, x15, x10
	orr x10, x10, x14
	csel x10, x12, x10, ne
	bic x9, x0, x9
	bic x11, x1, x11
	orr x9, x9, x11
	cmp x9, #0
	cset w9, ne
	adds x9, x10, x9
	cinc x10, x13, hs
	stp x9, x10, [x8, #16]
	mov w9, #1
	stp x9, xzr, [x8]
	ret
.LBB45_2:
	mov x9, #0
	stp x9, xzr, [x8]
```
## `checked_div_ceil_u8_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-1
	lsl w9, w9, w1
	and w10, w0, #0xff
	lsr w11, w10, w1
	bics wzr, w10, w9
	cinc w9, w11, ne
	cmp w8, #7
	csel w8, w8, w9, hi
	tst w1, #0xf8
	cset w0, eq
	mov x1, x8
```
## `checked_div_ceil_u16_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-1
	lsl w9, w9, w1
	and w10, w0, #0xffff
	lsr w11, w10, w1
	bics wzr, w10, w9
	cinc w9, w11, ne
	cmp w8, #15
	csel w1, w8, w9, hi
	cset w0, ls
```
## `checked_div_ceil_u32_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-1
	lsl w9, w9, w1
	lsr w10, w0, w1
	bics wzr, w0, w9
	cinc w9, w10, ne
	cmp w8, #31
	csel w1, w8, w9, hi
	cset w0, ls
```
## `checked_div_ceil_u64_unb_pow2`
```asm
	and w8, w1, #0xff
	mov x9, #-1
	lsl x9, x9, x1
	lsr x10, x0, x1
	bics xzr, x0, x9
	cinc x9, x10, ne
	cmp w8, #63
	csel x1, x8, x9, hi
	cset x0, ls
```
## `checked_div_ceil_u128_unb_pow2`
```asm
	tbnz w2, #7, .LBB50_2
	mov x9, #-1
	lsl x9, x9, x2
	mvn w10, w2
	mov x11, #9223372036854775807
	lsr x11, x11, x10
	orr x11, x9, x11
	and x12, x2, #0xff
	tst x12, #0x40
	csel x11, x9, x11, ne
	csel x9, xzr, x9, ne
	lsr x12, x1, x2
	csel x13, xzr, x12, ne
	lsr x14, x0, x2
	lsl x15, x1, #1
	lsl x10, x15, x10
	orr x10, x10, x14
	csel x10, x12, x10, ne
	bic x9, x0, x9
	bic x11, x1, x11
	orr x9, x9, x11
	cmp x9, #0
	cset w9, ne
	adds x9, x10, x9
	cinc x10, x13, hs
	stp x9, x10, [x8, #16]
	mov w9, #1
	stp x9, xzr, [x8]
	ret
.LBB50_2:
	mov x9, #0
	stp x9, xzr, [x8]
```
## `checked_div_floor_i8_unb_pow2`
```asm
	sxtb w8, w0
	tst w1, #0xf8
	cset w0, eq
	asr w1, w8, w1
```
## `checked_div_floor_i16_unb_pow2`
```asm
	sxth w8, w0
	tst w1, #0xf0
	cset w0, eq
	asr w1, w8, w1
```
## `checked_div_floor_i32_unb_pow2`
```asm
	tst w1, #0xe0
	cset w8, eq
	asr w1, w0, w1
	mov x0, x8
```
## `checked_div_floor_i64_unb_pow2`
```asm
	tst w1, #0xc0
	cset w8, eq
	asr x1, x0, x1
	mov x0, x8
```
## `checked_div_floor_i128_unb_pow2`
```asm
	tbnz w2, #7, .LBB55_2
	lsr x9, x0, x2
	mvn w10, w2
	lsl x11, x1, #1
	lsl x10, x11, x10
	orr x9, x10, x9
	asr x10, x1, x2
	and x11, x2, #0xff
	tst x11, #0x40
	csel x9, x10, x9, ne
	asr x11, x1, #63
	csel x10, x11, x10, ne
	stp x9, x10, [x8, #16]
	mov w9, #1
	stp x9, xzr, [x8]
	ret
.LBB55_2:
	mov x9, #0
	stp x9, xzr, [x8]
```
## `checked_div_floor_u8_unb_pow2`, `checked_div_u8_unb_pow2`
```asm
	and w8, w0, #0xff
	tst w1, #0xf8
	cset w0, eq
	lsr w1, w8, w1
```
## `checked_div_floor_u16_unb_pow2`, `checked_div_u16_unb_pow2`
```asm
	and w8, w0, #0xffff
	tst w1, #0xf0
	cset w0, eq
	lsr w1, w8, w1
```
## `checked_div_floor_u32_unb_pow2`, `checked_div_u32_unb_pow2`
```asm
	tst w1, #0xe0
	cset w8, eq
	lsr w1, w0, w1
	mov x0, x8
```
## `checked_div_floor_u64_unb_pow2`, `checked_div_u64_unb_pow2`
```asm
	tst w1, #0xc0
	cset w8, eq
	lsr x1, x0, x1
	mov x0, x8
```
## `checked_div_floor_u128_unb_pow2`
```asm
	tbnz w2, #7, .LBB60_2
	lsr x9, x0, x2
	mvn w10, w2
	lsl x11, x1, #1
	lsl x10, x11, x10
	orr x9, x10, x9
	lsr x10, x1, x2
	and x11, x2, #0xff
	tst x11, #0x40
	csel x9, x10, x9, ne
	csel x10, xzr, x10, ne
	stp x9, x10, [x8, #16]
	mov w9, #1
	stp x9, xzr, [x8]
	ret
.LBB60_2:
	mov x9, #0
	stp x9, xzr, [x8]
```
## `checked_div_i8_unb_pow2`
```asm
	mov w8, #-1
	lsl w8, w8, w1
	sxtb w9, w0
	lsr w9, w9, #7
	bic w8, w9, w8
	and w9, w1, #0xff
	add w8, w8, w0
	sxtb w8, w8
	asr w8, w8, w1
	cmp w9, #7
	csel w8, w8, w8, hi
	tst w1, #0xf8
	cset w0, eq
	mov x1, x8
```
## `checked_div_i16_unb_pow2`
```asm
	mov w8, #-1
	lsl w8, w8, w1
	sxth w9, w0
	lsr w9, w9, #15
	bic w8, w9, w8
	and w9, w1, #0xff
	add w8, w8, w0
	sxth w8, w8
	asr w8, w8, w1
	cmp w9, #15
	csel w1, w8, w8, hi
	cset w0, ls
```
## `checked_div_i32_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-1
	lsl w9, w9, w1
	asr w10, w0, #31
	bic w9, w10, w9
	add w9, w9, w0
	asr w9, w9, w1
	cmp w8, #31
	csel w1, w8, w9, hi
	cset w0, ls
```
## `checked_div_i64_unb_pow2`
```asm
	and w8, w1, #0xff
	cmp w8, #63
	mov x8, #-1
	lsl x8, x8, x1
	asr x9, x0, #63
	bic x8, x9, x8
	add x8, x8, x0
	asr x8, x8, x1
	csel x1, x8, x8, hi
	cset x0, ls
```
## `checked_div_i128_unb_pow2`
```asm
	tbnz w2, #7, .LBB65_2
	mov x9, #-1
	lsl x9, x9, x2
	and x10, x2, #0xff
	tst x10, #0x40
	csel x11, xzr, x9, ne
	mvn w12, w2
	mov x13, #9223372036854775807
	lsr x13, x13, x12
	orr x13, x9, x13
	csel x9, x9, x13, ne
	asr x13, x1, #63
	bic x9, x13, x9
	bic x11, x13, x11
	adds x11, x11, x0
	adc x9, x9, x1
	tst x10, #0x40
	asr x10, x9, x2
	lsl x13, x9, #1
	lsl x12, x13, x12
	lsr x11, x11, x2
	orr x11, x12, x11
	csel x11, x10, x11, ne
	asr x9, x9, #63
	csel x9, x9, x10, ne
	stp x11, x9, [x8, #16]
	mov w9, #1
	stp x9, xzr, [x8]
	ret
.LBB65_2:
	mov x9, #0
	stp x9, xzr, [x8]
```
## `checked_div_round_i8_unb_pow2`
```asm
	and w9, w1, #0xff
	cmp w9, #7
	b.hi .LBB74_2
	mov w8, #-1
	lsl w8, w8, w1
	sxtb w9, w0
	asr w9, w9, w1
	and w10, w0, #0xff
	bic w8, w10, w8
	subs w8, w8, w10, lsr #7
	csel w8, wzr, w8, lo
	ubfiz w8, w8, #1, #7
	lsr w8, w8, w1
	and w8, w8, #0x1
	add w8, w8, w9
.LBB74_2:
	tst w1, #0xf8
	cset w0, eq
	mov x1, x8
```
## `checked_div_round_i16_unb_pow2`
```asm
	and w8, w1, #0xff
	cmp w8, #15
	b.hi .LBB71_2
	mov w8, #-1
	lsl w8, w8, w1
	sxth w9, w0
	asr w9, w9, w1
	and w10, w0, #0xffff
	bic w8, w10, w8
	subs w8, w8, w10, lsr #15
	csel w8, wzr, w8, lo
	ubfiz w8, w8, #1, #15
	lsr w8, w8, w1
	and w8, w8, #0x1
	add w1, w8, w9
	mov w0, #1
	ret
.LBB71_2:
	mov w0, #0
```
## `checked_div_round_i32_unb_pow2`
```asm
	and w8, w1, #0xff
	cmp w8, #31
	b.hi .LBB72_2
	mov w8, #-1
	lsl w8, w8, w1
	asr w9, w0, w1
	bic w8, w0, w8
	subs w8, w8, w0, lsr #31
	csel w8, wzr, w8, lo
	lsl w8, w8, #1
	lsr w8, w8, w1
	and w8, w8, #0x1
	add w1, w8, w9
	mov w0, #1
	ret
.LBB72_2:
	mov w0, #0
```
## `checked_div_round_i64_unb_pow2`
```asm
	and w8, w1, #0xff
	cmp w8, #63
	b.hi .LBB73_2
	mov x8, #-1
	lsl x8, x8, x1
	asr x9, x0, x1
	bic x8, x0, x8
	subs x8, x8, x0, lsr #63
	csel x8, xzr, x8, lo
	lsl x8, x8, #1
	lsr x8, x8, x1
	and x8, x8, #0x1
	add x1, x8, x9
	mov w0, #1
	ret
.LBB73_2:
	mov x0, #0
```
## `checked_div_round_i128_unb_pow2`
```asm
	tbnz w2, #7, .LBB70_2
	mov x9, #-1
	lsl x9, x9, x2
	and x10, x2, #0xff
	tst x10, #0x40
	csel x11, xzr, x9, ne
	mvn w12, w2
	mov x13, #9223372036854775807
	lsr x13, x13, x12
	orr x13, x9, x13
	csel x9, x9, x13, ne
	bic x9, x1, x9
	bic x11, x0, x11
	subs x11, x11, x1, lsr #63
	sbcs x9, x9, xzr
	csel x9, xzr, x9, lo
	csel x11, xzr, x11, lo
	tst x10, #0x40
	asr x10, x1, x2
	asr x13, x1, #63
	csel x13, x13, x10, ne
	lsr x14, x0, x2
	lsl x15, x1, #1
	lsl x15, x15, x12
	orr x14, x15, x14
	csel x10, x10, x14, ne
	extr x9, x9, x11, #63
	lsl x11, x11, #1
	lsr x14, x9, x2
	lsl x9, x9, #1
	lsl x9, x9, x12
	lsr x11, x11, x2
	orr x9, x9, x11
	csel x9, x14, x9, ne
	and x9, x9, #0x1
	adds x9, x9, x10
	cinc x10, x13, hs
	stp x9, x10, [x8, #16]
	mov w9, #1
	stp x9, xzr, [x8]
	ret
.LBB70_2:
	mov x9, #0
	stp x9, xzr, [x8]
```
## `checked_div_round_u8_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #1
	lsl w9, w9, w1
	and w9, w9, #0xfe
	and w10, w0, #0xff
	lsr w10, w10, w1
	tst w0, w9, lsr #1
	cinc w9, w10, ne
	cmp w8, #7
	csel w8, w8, w9, hi
	tst w1, #0xf8
	cset w0, eq
	mov x1, x8
```
## `checked_div_round_u16_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #1
	lsl w9, w9, w1
	and w9, w9, #0xfffe
	and w10, w0, #0xffff
	lsr w10, w10, w1
	tst w0, w9, lsr #1
	cinc w9, w10, ne
	cmp w8, #15
	csel w1, w8, w9, hi
	cset w0, ls
```
## `checked_div_round_u32_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #1
	lsl w9, w9, w1
	lsr w10, w0, w1
	tst w0, w9, lsr #1
	cinc w9, w10, ne
	cmp w8, #31
	csel w1, w8, w9, hi
	cset w0, ls
```
## `checked_div_round_u64_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #1
	lsl x9, x9, x1
	lsr x10, x0, x1
	tst x0, x9, lsr #1
	cinc x9, x10, ne
	cmp w8, #63
	csel x1, x8, x9, hi
	cset x0, ls
```
## `checked_div_round_u128_unb_pow2`
```asm
	tbnz w2, #7, .LBB75_2
	mov w9, #1
	lsl x10, x9, x2
	and x11, x2, #0xff
	tst x11, #0x40
	csel x11, xzr, x10, ne
	csel x10, x10, xzr, ne
	extr x11, x10, x11, #1
	lsr x12, x1, x2
	csel x13, xzr, x12, ne
	lsr x14, x0, x2
	mvn w15, w2
	lsl x16, x1, #1
	lsl x15, x16, x15
	orr x14, x15, x14
	csel x12, x12, x14, ne
	and x11, x11, x0
	and x10, x1, x10, lsr #1
	orr x10, x11, x10
	cmp x10, #0
	cset w10, ne
	adds x10, x12, x10
	cinc x11, x13, hs
	stp x10, x11, [x8, #16]
	stp x9, xzr, [x8]
	ret
.LBB75_2:
	mov x9, #0
	stp x9, xzr, [x8]
```
## `checked_div_u128_unb_pow2`
```asm
	tbnz w2, #7, .LBB80_2
	lsr x9, x0, x2
	mvn w10, w2
	lsl x11, x1, #1
	lsl x10, x11, x10
	orr x9, x10, x9
	lsr x10, x1, x2
	and x11, x2, #0xff
	tst x11, #0x40
	csel x9, x10, x9, ne
	csel x10, xzr, x10, ne
	stp x9, x10, [x8, #16]
	mov w9, #1
	stp x9, xzr, [x8]
	ret
.LBB80_2:
	mov x9, #0
	stp x9, xzr, [x8]
```
## `checked_floor_to_multiple_i8_unb_pow2`, `checked_floor_to_multiple_u8_unb_pow2`
```asm
	tst w1, #0xf8
	cset w8, eq
	mov w9, #-1
	lsl w9, w9, w1
	and w1, w9, w0
	mov x0, x8
```
## `checked_floor_to_multiple_i16_unb_pow2`, `checked_floor_to_multiple_u16_unb_pow2`
```asm
	tst w1, #0xf0
	cset w8, eq
	mov w9, #-1
	lsl w9, w9, w1
	and w1, w9, w0
	mov x0, x8
```
## `checked_floor_to_multiple_i32_unb_pow2`, `checked_floor_to_multiple_u32_unb_pow2`
```asm
	tst w1, #0xe0
	cset w8, eq
	mov w9, #-1
	lsl w9, w9, w1
	and w1, w9, w0
	mov x0, x8
```
## `checked_floor_to_multiple_i64_unb_pow2`, `checked_floor_to_multiple_u64_unb_pow2`
```asm
	tst w1, #0xc0
	cset w8, eq
	mov x9, #-1
	lsl x9, x9, x1
	and x1, x9, x0
	mov x0, x8
```
## `checked_floor_to_multiple_i128_unb_pow2`
```asm
	tbnz w2, #7, .LBB85_2
	mov x9, #-1
	lsl x9, x9, x2
	and x10, x2, #0xff
	tst x10, #0x40
	csel x10, xzr, x9, ne
	mvn w11, w2
	mov x12, #9223372036854775807
	lsr x11, x12, x11
	orr x11, x9, x11
	csel x9, x9, x11, ne
	and x9, x9, x1
	and x10, x10, x0
	stp x10, x9, [x8, #16]
	mov w9, #1
	stp x9, xzr, [x8]
	ret
.LBB85_2:
	mov x9, #0
	stp x9, xzr, [x8]
```
## `checked_floor_to_multiple_u128_unb_pow2`
```asm
	tbnz w2, #7, .LBB90_2
	mov x9, #-1
	lsl x9, x9, x2
	and x10, x2, #0xff
	tst x10, #0x40
	csel x10, xzr, x9, ne
	mvn w11, w2
	mov x12, #9223372036854775807
	lsr x11, x12, x11
	orr x11, x9, x11
	csel x9, x9, x11, ne
	and x9, x9, x1
	and x10, x10, x0
	stp x10, x9, [x8, #16]
	mov w9, #1
	stp x9, xzr, [x8]
	ret
.LBB90_2:
	mov x9, #0
	stp x9, xzr, [x8]
```
## `checked_mul_i8_pow2`
```asm
	and w9, w0, #0xff
	lsl w8, w0, w1
	sxtb w10, w8
	asr w10, w10, w1
	cmp w9, w10, uxtb
	cset w0, eq
	mov x1, x8
```
## `checked_mul_i8_unb_pow2`
```asm
	and w9, w0, #0xff
	tst w1, #0xf8
	lsl w8, w0, w1
	sxtb w10, w8
	and x11, x1, #0x7
	asr w10, w10, w11
	and w10, w10, #0xff
	ccmp w9, w10, #0, eq
	cset w0, eq
	mov x1, x8
```
## `checked_mul_i16_pow2`
```asm
	and w9, w0, #0xffff
	lsl w8, w0, w1
	sxth w10, w8
	asr w10, w10, w1
	cmp w9, w10, uxth
	cset w0, eq
	mov x1, x8
```
## `checked_mul_i16_unb_pow2`
```asm
	and w8, w1, #0xff
	lsl w9, w0, w1
	sxth w10, w9
	and w11, w0, #0xffff
	asr w10, w10, w1
	cmp w11, w10, uxth
	cset w10, eq
	cmp w8, #15
	csel w1, w8, w9, hi
	csel w0, wzr, w10, hi
```
## `checked_mul_i32_pow2`
```asm
	lsl w8, w0, w1
	asr w9, w8, w1
	cmp w0, w9
	cset w0, eq
	mov x1, x8
```
## `checked_mul_i32_unb_pow2`
```asm
	and w8, w1, #0xff
	lsl w9, w0, w1
	asr w10, w9, w1
	cmp w0, w10
	cset w10, eq
	cmp w8, #31
	csel w1, w8, w9, hi
	csel w0, wzr, w10, hi
```
## `checked_mul_i64_pow2`
```asm
	lsl x8, x0, x1
	asr x9, x8, x1
	cmp x0, x9
	cset w0, eq
	mov x1, x8
```
## `checked_mul_i64_unb_pow2`
```asm
	tst w1, #0xc0
	lsl x8, x0, x1
	asr x9, x8, x1
	ccmp x0, x9, #0, eq
	cset w0, eq
	mov x1, x8
```
## `checked_mul_i128_pow2`
```asm
	mov x9, #0
	lsl x10, x1, x2
	mvn w12, w2
	lsr x11, x0, #1
	lsr x11, x11, x12
	orr x10, x10, x11
	lsl x11, x0, x2
	and x13, x2, #0xff
	tst x13, #0x40
	csel x10, x11, x10, ne
	csel x11, xzr, x11, ne
	lsl x13, x10, #1
	lsl x12, x13, x12
	lsr x13, x11, x2
	orr x12, x12, x13
	asr x13, x10, x2
	csel x12, x13, x12, ne
	asr x14, x10, #63
	csel x13, x14, x13, ne
	cmp x1, x13
	ccmp x0, x12, #0, eq
	b.ne .LBB95_2
	stp x11, x10, [x8, #16]
	mov w9, #1
.LBB95_2:
	stp x9, xzr, [x8]
```
## `checked_mul_i128_unb_pow2`
```asm
	tbnz w2, #7, .LBB96_3
	lsl x9, x1, x2
	mvn w11, w2
	lsr x10, x0, #1
	lsr x10, x10, x11
	orr x9, x9, x10
	lsl x10, x0, x2
	and x12, x2, #0xff
	tst x12, #0x40
	csel x9, x10, x9, ne
	csel x10, xzr, x10, ne
	lsr x12, x10, x2
	lsl x13, x9, #1
	lsl x11, x13, x11
	orr x11, x11, x12
	asr x12, x9, x2
	csel x11, x12, x11, ne
	asr x13, x9, #63
	csel x12, x13, x12, ne
	cmp x1, x12
	ccmp x0, x11, #0, eq
	b.ne .LBB96_3
	stp x10, x9, [x8, #16]
	mov w9, #1
	stp x9, xzr, [x8]
	ret
.LBB96_3:
	mov x9, #0
	stp x9, xzr, [x8]
```
## `checked_mul_u8_pow2`
```asm
	lsl w8, w0, w1
	and w9, w8, #0xff
	lsr w9, w9, w1
	cmp w9, w0, uxtb
	cset w0, eq
	mov x1, x8
```
## `checked_mul_u8_unb_pow2`
```asm
	and w9, w0, #0xff
	tst w1, #0xf8
	lsl w8, w0, w1
	and w10, w8, #0xff
	and x11, x1, #0x7
	lsr w10, w10, w11
	ccmp w10, w9, #0, eq
	cset w0, eq
	mov x1, x8
```
## `checked_mul_u16_pow2`
```asm
	lsl w8, w0, w1
	and w9, w8, #0xffff
	lsr w9, w9, w1
	cmp w9, w0, uxth
	cset w0, eq
	mov x1, x8
```
## `checked_mul_u16_unb_pow2`
```asm
	and w8, w1, #0xff
	lsl w9, w0, w1
	and w10, w9, #0xffff
	lsr w10, w10, w1
	cmp w10, w0, uxth
	cset w10, eq
	cmp w8, #15
	csel w1, w8, w9, hi
	csel w0, wzr, w10, hi
```
## `checked_mul_u32_pow2`
```asm
	lsl w8, w0, w1
	lsr w9, w8, w1
	cmp w0, w9
	cset w0, eq
	mov x1, x8
```
## `checked_mul_u32_unb_pow2`
```asm
	and w8, w1, #0xff
	lsl w9, w0, w1
	lsr w10, w9, w1
	cmp w0, w10
	cset w10, eq
	cmp w8, #31
	csel w1, w8, w9, hi
	csel w0, wzr, w10, hi
```
## `checked_mul_u64_pow2`
```asm
	lsl x8, x0, x1
	lsr x9, x8, x1
	cmp x0, x9
	cset w0, eq
	mov x1, x8
```
## `checked_mul_u64_unb_pow2`
```asm
	tst w1, #0xc0
	lsl x8, x0, x1
	lsr x9, x8, x1
	ccmp x0, x9, #0, eq
	cset w0, eq
	mov x1, x8
```
## `checked_mul_u128_pow2`
```asm
	mov x9, #0
	lsl x10, x1, x2
	mvn w12, w2
	lsr x11, x0, #1
	lsr x11, x11, x12
	orr x10, x10, x11
	lsl x11, x0, x2
	and x13, x2, #0xff
	tst x13, #0x40
	csel x10, x11, x10, ne
	csel x11, xzr, x11, ne
	lsl x13, x10, #1
	lsl x12, x13, x12
	lsr x13, x11, x2
	orr x12, x12, x13
	lsr x13, x10, x2
	csel x12, x13, x12, ne
	csel x13, xzr, x13, ne
	cmp x1, x13
	ccmp x0, x12, #0, eq
	b.ne .LBB105_2
	stp x11, x10, [x8, #16]
	mov w9, #1
.LBB105_2:
	stp x9, xzr, [x8]
```
## `checked_mul_u128_unb_pow2`
```asm
	tbnz w2, #7, .LBB106_3
	lsl x9, x1, x2
	mvn w11, w2
	lsr x10, x0, #1
	lsr x10, x10, x11
	orr x9, x9, x10
	lsl x10, x0, x2
	and x12, x2, #0xff
	tst x12, #0x40
	csel x9, x10, x9, ne
	csel x10, xzr, x10, ne
	lsr x12, x10, x2
	lsl x13, x9, #1
	lsl x11, x13, x11
	orr x11, x11, x12
	lsr x12, x9, x2
	csel x11, x12, x11, ne
	csel x12, xzr, x12, ne
	cmp x1, x12
	ccmp x0, x11, #0, eq
	b.ne .LBB106_3
	stp x10, x9, [x8, #16]
	mov w9, #1
	stp x9, xzr, [x8]
	ret
.LBB106_3:
	mov x9, #0
	stp x9, xzr, [x8]
```
## `checked_rem_floor_i8_unb_pow2`, `checked_rem_floor_u8_unb_pow2`, `checked_rem_u8_unb_pow2`
```asm
	tst w1, #0xf8
	cset w8, eq
	mov w9, #-1
	lsl w9, w9, w1
	bic w1, w0, w9
	mov x0, x8
```
## `checked_rem_floor_i16_unb_pow2`, `checked_rem_floor_u16_unb_pow2`, `checked_rem_u16_unb_pow2`
```asm
	tst w1, #0xf0
	cset w8, eq
	mov w9, #-1
	lsl w9, w9, w1
	bic w1, w0, w9
	mov x0, x8
```
## `checked_rem_floor_i32_unb_pow2`, `checked_rem_floor_u32_unb_pow2`, `checked_rem_u32_unb_pow2`
```asm
	tst w1, #0xe0
	cset w8, eq
	mov w9, #-1
	lsl w9, w9, w1
	bic w1, w0, w9
	mov x0, x8
```
## `checked_rem_floor_i64_unb_pow2`, `checked_rem_floor_u64_unb_pow2`, `checked_rem_u64_unb_pow2`
```asm
	tst w1, #0xc0
	cset w8, eq
	mov x9, #-1
	lsl x9, x9, x1
	bic x1, x0, x9
	mov x0, x8
```
## `checked_rem_floor_i128_unb_pow2`
```asm
	tbnz w2, #7, .LBB115_2
	mov x9, #-1
	lsl x9, x9, x2
	and x10, x2, #0xff
	tst x10, #0x40
	csel x10, xzr, x9, ne
	mvn w11, w2
	mov x12, #9223372036854775807
	lsr x11, x12, x11
	orr x11, x9, x11
	csel x9, x9, x11, ne
	bic x9, x1, x9
	bic x10, x0, x10
	stp x10, x9, [x8, #16]
	mov w9, #1
	stp x9, xzr, [x8]
	ret
.LBB115_2:
	mov x9, #0
	stp x9, xzr, [x8]
```
## `checked_rem_floor_u128_unb_pow2`
```asm
	tbnz w2, #7, .LBB120_2
	mov x9, #-1
	lsl x9, x9, x2
	and x10, x2, #0xff
	tst x10, #0x40
	csel x10, xzr, x9, ne
	mvn w11, w2
	mov x12, #9223372036854775807
	lsr x11, x12, x11
	orr x11, x9, x11
	csel x9, x9, x11, ne
	bic x9, x1, x9
	bic x10, x0, x10
	stp x10, x9, [x8, #16]
	mov w9, #1
	stp x9, xzr, [x8]
	ret
.LBB120_2:
	mov x9, #0
	stp x9, xzr, [x8]
```
## `checked_rem_i8_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-1
	lsl w9, w9, w1
	sbfx w10, w0, #7, #1
	bic w10, w10, w9
	add w10, w10, w0
	and w9, w10, w9
	sub w9, w0, w9
	cmp w8, #7
	csel w8, w8, w9, hi
	tst w1, #0xf8
	cset w0, eq
	mov x1, x8
```
## `checked_rem_i16_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-1
	lsl w9, w9, w1
	sbfx w10, w0, #15, #1
	bic w10, w10, w9
	add w10, w10, w0
	and w9, w10, w9
	sub w9, w0, w9
	cmp w8, #15
	csel w1, w8, w9, hi
	cset w0, ls
```
## `checked_rem_i32_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-1
	lsl w9, w9, w1
	asr w10, w0, #31
	bic w10, w10, w9
	add w10, w10, w0
	and w9, w10, w9
	sub w9, w0, w9
	cmp w8, #31
	csel w1, w8, w9, hi
	cset w0, ls
```
## `checked_rem_i64_unb_pow2`
```asm
	and w8, w1, #0xff
	mov x9, #-1
	lsl x9, x9, x1
	asr x10, x0, #63
	bic x10, x10, x9
	add x10, x10, x0
	and x9, x10, x9
	sub x9, x0, x9
	cmp w8, #63
	csel x1, x8, x9, hi
	cset x0, ls
```
## `checked_rem_i128_unb_pow2`
```asm
	tbnz w2, #7, .LBB125_2
	mov x9, #-1
	lsl x9, x9, x2
	and x10, x2, #0xff
	tst x10, #0x40
	csel x10, xzr, x9, ne
	mvn w11, w2
	mov x12, #9223372036854775807
	lsr x11, x12, x11
	orr x11, x9, x11
	csel x9, x9, x11, ne
	asr x11, x1, #63
	bic x12, x11, x9
	bic x11, x11, x10
	adds x11, x11, x0
	and x10, x11, x10
	adc x11, x12, x1
	and x9, x11, x9
	subs x10, x0, x10
	sbc x9, x1, x9
	stp x10, x9, [x8, #16]
	mov w9, #1
	stp x9, xzr, [x8]
	ret
.LBB125_2:
	mov x9, #0
	stp x9, xzr, [x8]
```
## `checked_rem_u128_unb_pow2`
```asm
	tbnz w2, #7, .LBB130_2
	mov x9, #-1
	lsl x9, x9, x2
	and x10, x2, #0xff
	tst x10, #0x40
	csel x10, xzr, x9, ne
	mvn w11, w2
	mov x12, #9223372036854775807
	lsr x11, x12, x11
	orr x11, x9, x11
	csel x9, x9, x11, ne
	bic x9, x1, x9
	bic x10, x0, x10
	stp x10, x9, [x8, #16]
	mov w9, #1
	stp x9, xzr, [x8]
	ret
.LBB130_2:
	mov x9, #0
	stp x9, xzr, [x8]
```
## `checked_round_to_multiple_i8_unb_pow2`
```asm
	and w8, w1, #0xff
	cmp w8, #7
	b.hi .LBB139_3
	mov w8, #1
	lsl w8, w8, w1
	ubfx w9, w8, #1, #7
	and w10, w0, #0x80
	subs w9, w9, w10, lsr #7
	csel w9, wzr, w9, lo
	add w9, w9, w0, sxtb
	sxtb w10, w9
	cmp w10, w9
	b.ne .LBB139_3
	neg w8, w8
	and w1, w9, w8
	mov w0, #1
	ret
.LBB139_3:
	mov w0, #0
```
## `checked_round_to_multiple_i16_unb_pow2`
```asm
	and w8, w1, #0xff
	cmp w8, #15
	b.hi .LBB136_3
	mov w8, #1
	lsl w8, w8, w1
	ubfx w9, w8, #1, #15
	and w10, w0, #0x8000
	subs w9, w9, w10, lsr #15
	csel w9, wzr, w9, lo
	add w9, w9, w0, sxth
	sxth w10, w9
	cmp w10, w9
	b.ne .LBB136_3
	neg w8, w8
	and w1, w9, w8
	mov w0, #1
	ret
.LBB136_3:
	mov w0, #0
```
## `checked_round_to_multiple_i32_unb_pow2`
```asm
	and w8, w1, #0xff
	cmp w8, #31
	b.hi .LBB137_3
	mov w8, #1
	lsl w8, w8, w1
	lsr w9, w8, #1
	subs w9, w9, w0, lsr #31
	csel w9, wzr, w9, lo
	adds w9, w0, w9
	b.vs .LBB137_3
	neg w8, w8
	and w1, w9, w8
	mov w0, #1
	ret
.LBB137_3:
	mov w0, #0
```
## `checked_round_to_multiple_i64_unb_pow2`
```asm
	and w8, w1, #0xff
	cmp w8, #63
	b.hi .LBB138_3
	mov w8, #1
	lsl x8, x8, x1
	lsr x9, x8, #1
	subs x9, x9, x0, lsr #63
	csel x9, xzr, x9, lo
	adds x9, x0, x9
	b.vs .LBB138_3
	neg x8, x8
	and x1, x9, x8
	mov w0, #1
	ret
.LBB138_3:
	mov x0, #0
```
## `checked_round_to_multiple_i128_unb_pow2`
```asm
	tbnz w2, #7, .LBB135_3
	mov w9, #1
	lsl x10, x9, x2
	and x9, x2, #0xff
	tst x9, #0x40
	csel x9, x10, xzr, ne
	csel x10, xzr, x10, ne
	extr x11, x9, x10, #1
	lsr x12, x9, #1
	subs x11, x11, x1, lsr #63
	sbcs x12, x12, xzr
	csel x13, xzr, x12, lo
	csel x11, xzr, x11, lo
	adds x12, x0, x11
	adcs x11, x1, x13
	cset w13, vs
	tbnz w13, #0, .LBB135_3
	negs x10, x10
	and x10, x12, x10
	ngc x9, x9
	and x9, x11, x9
	stp x10, x9, [x8, #16]
	mov w9, #1
	stp x9, xzr, [x8]
	ret
.LBB135_3:
	mov x9, #0
	stp x9, xzr, [x8]
```
## `checked_round_to_multiple_u8_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #1
	lsl w9, w9, w1
	ubfx w10, w9, #1, #7
	add w10, w10, w0, uxtb
	neg w9, w9
	and w9, w10, w9
	tst w10, #0x100
	csel w9, w9, w8, eq
	cset w10, eq
	cmp w8, #7
	csel w1, w8, w9, hi
	csel w0, wzr, w10, hi
```
## `checked_round_to_multiple_u16_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #1
	lsl w9, w9, w1
	ubfx w10, w9, #1, #15
	add w10, w10, w0, uxth
	neg w9, w9
	and w9, w10, w9
	tst w10, #0x10000
	csel w9, w9, w8, eq
	cset w10, eq
	cmp w8, #15
	csel w1, w8, w9, hi
	csel w0, wzr, w10, hi
```
## `checked_round_to_multiple_u32_unb_pow2`
```asm
	and w8, w1, #0xff
	cmp w8, #31
	b.hi .LBB142_3
	mov w8, #1
	lsl w8, w8, w1
	adds w9, w0, w8, lsr #1
	b.hs .LBB142_3
	neg w8, w8
	and w1, w9, w8
	mov w0, #1
	ret
.LBB142_3:
	mov w0, #0
```
## `checked_round_to_multiple_u64_unb_pow2`
```asm
	and w8, w1, #0xff
	cmp w8, #63
	b.hi .LBB143_3
	mov w8, #1
	lsl x8, x8, x1
	adds x9, x0, x8, lsr #1
	b.hs .LBB143_3
	neg x8, x8
	and x1, x9, x8
	mov w0, #1
	ret
.LBB143_3:
	mov x0, #0
```
## `checked_round_to_multiple_u128_unb_pow2`
```asm
	tbnz w2, #7, .LBB140_3
	mov w9, #1
	lsl x10, x9, x2
	and x9, x2, #0xff
	tst x9, #0x40
	csel x9, x10, xzr, ne
	csel x10, xzr, x10, ne
	extr x11, x9, x10, #1
	lsr x13, x9, #1
	adds x12, x11, x0
	adcs x11, x13, x1
	b.hs .LBB140_3
	negs x10, x10
	and x10, x12, x10
	ngc x9, x9
	and x9, x11, x9
	stp x10, x9, [x8, #16]
	mov w9, #1
	stp x9, xzr, [x8]
	ret
.LBB140_3:
	mov x9, #0
	stp x9, xzr, [x8]
```
## `div_ceil_i8_pow2`
```asm
	sxtb w8, w0
	mov w9, #-1
	lsl w9, w9, w1
	asr w8, w8, w1
	bic w9, w0, w9
	tst w9, #0xff
	cinc w0, w8, ne
```
## `div_ceil_i8_unb_pow2`
```asm
	sxtb w8, w0
	and x9, x1, #0x7
	mov w10, #-1
	lsl w10, w10, w9
	asr w8, w8, w9
	bics wzr, w0, w10
	cinc w0, w8, ne
```
## `div_ceil_i16_pow2`
```asm
	sxth w8, w0
	mov w9, #-1
	lsl w9, w9, w1
	asr w8, w8, w1
	bic w9, w0, w9
	tst w9, #0xffff
	cinc w0, w8, ne
```
## `div_ceil_i16_unb_pow2`
```asm
	sxth w8, w0
	and x9, x1, #0xf
	mov w10, #-1
	lsl w10, w10, w9
	asr w8, w8, w9
	bics wzr, w0, w10
	cinc w0, w8, ne
```
## `div_ceil_i32_pow2`, `div_ceil_i32_unb_pow2`
```asm
	mov w8, #-1
	lsl w8, w8, w1
	asr w9, w0, w1
	bics wzr, w0, w8
	cinc w0, w9, ne
```
## `div_ceil_i64_pow2`, `div_ceil_i64_unb_pow2`
```asm
	mov x8, #-1
	lsl x8, x8, x1
	asr x9, x0, x1
	bics xzr, x0, x8
	cinc x0, x9, ne
```
## `div_ceil_i128_pow2`
```asm
	mov x8, #-1
	lsl x8, x8, x2
	mvn w9, w2
	mov x10, #9223372036854775807
	lsr x10, x10, x9
	orr x10, x8, x10
	and x11, x2, #0xff
	tst x11, #0x40
	csel x10, x8, x10, ne
	csel x8, xzr, x8, ne
	asr x11, x1, x2
	asr x12, x1, #63
	csel x12, x12, x11, ne
	lsr x13, x0, x2
	lsl x14, x1, #1
	lsl x9, x14, x9
	orr x9, x9, x13
	csel x9, x11, x9, ne
	bic x8, x0, x8
	bic x10, x1, x10
	orr x8, x8, x10
	cmp x8, #0
	cset w8, ne
	adds x0, x9, x8
	cinc x1, x12, hs
```
## `div_ceil_i128_unb_pow2`
```asm
	mov x8, #-1
	lsl x8, x8, x2
	mvn w9, w2
	mov x10, #9223372036854775807
	lsr x10, x10, x9
	orr x10, x8, x10
	and x11, x2, #0x7f
	tst x11, #0x40
	csel x10, x8, x10, ne
	csel x8, xzr, x8, ne
	asr x11, x1, x2
	asr x12, x1, #63
	csel x12, x12, x11, ne
	lsr x13, x0, x2
	lsl x14, x1, #1
	lsl x9, x14, x9
	orr x9, x9, x13
	csel x9, x11, x9, ne
	bic x8, x0, x8
	bic x10, x1, x10
	orr x8, x8, x10
	cmp x8, #0
	cset w8, ne
	adds x0, x9, x8
	cinc x1, x12, hs
```
## `div_ceil_u8_pow2`
```asm
	and w8, w0, #0xff
	mov w9, #-1
	lsl w9, w9, w1
	lsr w10, w8, w1
	bics wzr, w8, w9
	cinc w0, w10, ne
```
## `div_ceil_u8_unb_pow2`
```asm
	and w8, w0, #0xff
	and x9, x1, #0x7
	mov w10, #-1
	lsl w10, w10, w9
	lsr w8, w8, w9
	bics wzr, w0, w10
	cinc w0, w8, ne
```
## `div_ceil_u16_pow2`
```asm
	and w8, w0, #0xffff
	mov w9, #-1
	lsl w9, w9, w1
	lsr w10, w8, w1
	bics wzr, w8, w9
	cinc w0, w10, ne
```
## `div_ceil_u16_unb_pow2`
```asm
	and w8, w0, #0xffff
	and x9, x1, #0xf
	mov w10, #-1
	lsl w10, w10, w9
	lsr w8, w8, w9
	bics wzr, w0, w10
	cinc w0, w8, ne
```
## `div_ceil_u32_pow2`, `div_ceil_u32_unb_pow2`
```asm
	mov w8, #-1
	lsl w8, w8, w1
	lsr w9, w0, w1
	bics wzr, w0, w8
	cinc w0, w9, ne
```
## `div_ceil_u64_pow2`, `div_ceil_u64_unb_pow2`
```asm
	mov x8, #-1
	lsl x8, x8, x1
	lsr x9, x0, x1
	bics xzr, x0, x8
	cinc x0, x9, ne
```
## `div_ceil_u128_pow2`
```asm
	mov x8, #-1
	lsl x8, x8, x2
	mvn w9, w2
	mov x10, #9223372036854775807
	lsr x10, x10, x9
	orr x10, x8, x10
	and x11, x2, #0xff
	tst x11, #0x40
	csel x10, x8, x10, ne
	csel x8, xzr, x8, ne
	lsr x11, x1, x2
	csel x12, xzr, x11, ne
	lsr x13, x0, x2
	lsl x14, x1, #1
	lsl x9, x14, x9
	orr x9, x9, x13
	csel x9, x11, x9, ne
	bic x8, x0, x8
	bic x10, x1, x10
	orr x8, x8, x10
	cmp x8, #0
	cset w8, ne
	adds x0, x9, x8
	cinc x1, x12, hs
```
## `div_ceil_u128_unb_pow2`
```asm
	mov x8, #-1
	lsl x8, x8, x2
	mvn w9, w2
	mov x10, #9223372036854775807
	lsr x10, x10, x9
	orr x10, x8, x10
	and x11, x2, #0x7f
	tst x11, #0x40
	csel x10, x8, x10, ne
	csel x8, xzr, x8, ne
	lsr x11, x1, x2
	csel x12, xzr, x11, ne
	lsr x13, x0, x2
	lsl x14, x1, #1
	lsl x9, x14, x9
	orr x9, x9, x13
	csel x9, x11, x9, ne
	bic x8, x0, x8
	bic x10, x1, x10
	orr x8, x8, x10
	cmp x8, #0
	cset w8, ne
	adds x0, x9, x8
	cinc x1, x12, hs
```
## `div_floor_i8_pow2`
```asm
	sxtb w8, w0
	asr w0, w8, w1
```
## `div_floor_i8_unb_pow2`
```asm
	sxtb w8, w0
	and x9, x1, #0x7
	asr w0, w8, w9
```
## `div_floor_i16_pow2`
```asm
	sxth w8, w0
	asr w0, w8, w1
```
## `div_floor_i16_unb_pow2`
```asm
	sxth w8, w0
	and x9, x1, #0xf
	asr w0, w8, w9
```
## `div_floor_i32_pow2`, `div_floor_i32_unb_pow2`
```asm
	asr w0, w0, w1
```
## `div_floor_i64_pow2`, `div_floor_i64_unb_pow2`
```asm
	asr x0, x0, x1
```
## `div_floor_i128_pow2`
```asm
	asr x9, x1, x2
	and x8, x2, #0xff
	asr x10, x1, #63
	tst x8, #0x40
	csel x8, x10, x9, ne
	lsr x10, x0, x2
	mvn w11, w2
	lsl x12, x1, #1
	lsl x11, x12, x11
	orr x10, x11, x10
	csel x0, x9, x10, ne
	mov x1, x8
```
## `div_floor_i128_unb_pow2`
```asm
	asr x9, x1, x2
	and x8, x2, #0x7f
	asr x10, x1, #63
	tst x8, #0x40
	csel x8, x10, x9, ne
	lsr x10, x0, x2
	mvn w11, w2
	lsl x12, x1, #1
	lsl x11, x12, x11
	orr x10, x11, x10
	csel x0, x9, x10, ne
	mov x1, x8
```
## `div_floor_u8_pow2`, `div_u8_pow2`
```asm
	and w8, w0, #0xff
	lsr w0, w8, w1
```
## `div_floor_u8_unb_pow2`, `div_u8_unb_pow2`
```asm
	and w8, w0, #0xff
	and x9, x1, #0x7
	lsr w0, w8, w9
```
## `div_floor_u16_pow2`, `div_u16_pow2`
```asm
	and w8, w0, #0xffff
	lsr w0, w8, w1
```
## `div_floor_u16_unb_pow2`, `div_u16_unb_pow2`
```asm
	and w8, w0, #0xffff
	and x9, x1, #0xf
	lsr w0, w8, w9
```
## `div_floor_u32_pow2`, `div_floor_u32_unb_pow2`, `div_u32_pow2`, `div_u32_unb_pow2`
```asm
	lsr w0, w0, w1
```
## `div_floor_u64_pow2`, `div_floor_u64_unb_pow2`, `div_u64_pow2`, `div_u64_unb_pow2`
```asm
	lsr x0, x0, x1
```
## `div_floor_u128_pow2`, `div_u128_pow2`
```asm
	lsr x9, x1, x2
	and x8, x2, #0xff
	tst x8, #0x40
	csel x8, xzr, x9, ne
	lsr x10, x0, x2
	mvn w11, w2
	lsl x12, x1, #1
	lsl x11, x12, x11
	orr x10, x11, x10
	csel x0, x9, x10, ne
	mov x1, x8
```
## `div_floor_u128_unb_pow2`, `div_u128_unb_pow2`
```asm
	lsr x9, x1, x2
	and x8, x2, #0x7f
	tst x8, #0x40
	csel x8, xzr, x9, ne
	lsr x10, x0, x2
	mvn w11, w2
	lsl x12, x1, #1
	lsl x11, x12, x11
	orr x10, x11, x10
	csel x0, x9, x10, ne
	mov x1, x8
```
## `div_i8_pow2`
```asm
	sxtb w8, w0
	mov w9, #-1
	lsl w9, w9, w1
	lsr w8, w8, #7
	bic w8, w8, w9
	add w8, w8, w0
	sxtb w8, w8
	and x9, x1, #0x7
	asr w0, w8, w9
```
## `div_i8_unb_pow2`
```asm
	sxtb w8, w0
	and x9, x1, #0x7
	mov w10, #-1
	lsl w10, w10, w9
	lsr w8, w8, #7
	bic w8, w8, w10
	add w8, w8, w0
	sxtb w8, w8
	asr w0, w8, w9
```
## `div_i16_pow2`
```asm
	sxth w8, w0
	mov w9, #-1
	lsl w9, w9, w1
	lsr w8, w8, #15
	bic w8, w8, w9
	add w8, w8, w0
	sxth w8, w8
	and x9, x1, #0xf
	asr w0, w8, w9
```
## `div_i16_unb_pow2`
```asm
	sxth w8, w0
	and x9, x1, #0xf
	mov w10, #-1
	lsl w10, w10, w9
	lsr w8, w8, #15
	bic w8, w8, w10
	add w8, w8, w0
	sxth w8, w8
	asr w0, w8, w9
```
## `div_i32_pow2`, `div_i32_unb_pow2`
```asm
	mov w8, #-1
	lsl w8, w8, w1
	asr w9, w0, #31
	bic w8, w9, w8
	add w8, w8, w0
	asr w0, w8, w1
```
## `div_i64_pow2`, `div_i64_unb_pow2`
```asm
	mov x8, #-1
	lsl x8, x8, x1
	asr x9, x0, #63
	bic x8, x9, x8
	add x8, x8, x0
	asr x0, x8, x1
```
## `div_i128_pow2`
```asm
	mov x8, #-1
	lsl x8, x8, x2
	and x9, x2, #0xff
	tst x9, #0x40
	csel x9, xzr, x8, ne
	mvn w10, w2
	mov x11, #9223372036854775807
	lsr x11, x11, x10
	orr x11, x8, x11
	csel x8, x8, x11, ne
	asr x11, x1, #63
	bic x8, x11, x8
	bic x9, x11, x9
	adds x9, x9, x0
	adc x8, x8, x1
	asr x11, x8, x2
	asr x12, x8, #63
	and x13, x2, #0x7f
	tst x13, #0x40
	csel x1, x12, x11, ne
	lsl x8, x8, #1
	lsl x8, x8, x10
	lsr x9, x9, x2
	orr x8, x8, x9
	csel x0, x11, x8, ne
```
## `div_i128_unb_pow2`
```asm
	mov x8, #-1
	lsl x8, x8, x2
	and x9, x2, #0x7f
	tst x9, #0x40
	csel x10, xzr, x8, ne
	mvn w11, w2
	mov x12, #9223372036854775807
	lsr x12, x12, x11
	orr x12, x8, x12
	csel x8, x8, x12, ne
	asr x12, x1, #63
	bic x8, x12, x8
	bic x10, x12, x10
	adds x10, x10, x0
	adc x8, x8, x1
	tst x9, #0x40
	asr x9, x8, x2
	asr x12, x8, #63
	csel x1, x12, x9, ne
	lsl x8, x8, #1
	lsl x8, x8, x11
	lsr x10, x10, x2
	orr x8, x8, x10
	csel x0, x9, x8, ne
```
## `div_round_i8_pow2`
```asm
	and w8, w0, #0xff
	sxtb w9, w0
	mov w10, #-1
	lsl w10, w10, w1
	asr w9, w9, w1
	bic w10, w8, w10
	subs w8, w10, w8, lsr #7
	csel w8, wzr, w8, lo
	ubfiz w8, w8, #1, #7
	lsr w8, w8, w1
	and w8, w8, #0x1
	add w0, w8, w9
```
## `div_round_i8_unb_pow2`
```asm
	and w8, w0, #0x80
	sxtb w9, w0
	and x10, x1, #0x7
	mov w11, #-1
	lsl w11, w11, w10
	asr w9, w9, w10
	bic w11, w0, w11
	subs w8, w11, w8, lsr #7
	csel w8, wzr, w8, lo
	lsl w8, w8, #1
	lsr w8, w8, w10
	and w8, w8, #0x1
	add w0, w8, w9
```
## `div_round_i16_pow2`
```asm
	and w8, w0, #0xffff
	sxth w9, w0
	mov w10, #-1
	lsl w10, w10, w1
	asr w9, w9, w1
	bic w10, w8, w10
	subs w8, w10, w8, lsr #15
	csel w8, wzr, w8, lo
	ubfiz w8, w8, #1, #15
	lsr w8, w8, w1
	and w8, w8, #0x1
	add w0, w8, w9
```
## `div_round_i16_unb_pow2`
```asm
	and w8, w0, #0x8000
	sxth w9, w0
	and x10, x1, #0xf
	mov w11, #-1
	lsl w11, w11, w10
	asr w9, w9, w10
	bic w11, w0, w11
	subs w8, w11, w8, lsr #15
	csel w8, wzr, w8, lo
	lsl w8, w8, #1
	lsr w8, w8, w10
	and w8, w8, #0x1
	add w0, w8, w9
```
## `div_round_i32_pow2`, `div_round_i32_unb_pow2`
```asm
	mov w8, #-1
	lsl w8, w8, w1
	asr w9, w0, w1
	bic w8, w0, w8
	subs w8, w8, w0, lsr #31
	csel w8, wzr, w8, lo
	lsl w8, w8, #1
	lsr w8, w8, w1
	and w8, w8, #0x1
	add w0, w8, w9
```
## `div_round_i64_pow2`, `div_round_i64_unb_pow2`
```asm
	mov x8, #-1
	lsl x8, x8, x1
	asr x9, x0, x1
	bic x8, x0, x8
	subs x8, x8, x0, lsr #63
	csel x8, xzr, x8, lo
	lsl x8, x8, #1
	lsr x8, x8, x1
	and x8, x8, #0x1
	add x0, x8, x9
```
## `div_round_i128_pow2`
```asm
	mov x8, #-1
	lsl x8, x8, x2
	and x9, x2, #0xff
	tst x9, #0x40
	csel x10, xzr, x8, ne
	mvn w11, w2
	mov x12, #9223372036854775807
	lsr x12, x12, x11
	orr x12, x8, x12
	csel x8, x8, x12, ne
	bic x8, x1, x8
	bic x10, x0, x10
	subs x10, x10, x1, lsr #63
	sbcs x8, x8, xzr
	csel x8, xzr, x8, lo
	csel x10, xzr, x10, lo
	tst x9, #0x40
	asr x9, x1, x2
	asr x12, x1, #63
	csel x12, x12, x9, ne
	lsr x13, x0, x2
	lsl x14, x1, #1
	lsl x14, x14, x11
	orr x13, x14, x13
	csel x9, x9, x13, ne
	extr x8, x8, x10, #63
	lsl x10, x10, #1
	lsr x13, x8, x2
	lsl x8, x8, #1
	lsl x8, x8, x11
	lsr x10, x10, x2
	orr x8, x8, x10
	csel x8, x13, x8, ne
	and x8, x8, #0x1
	adds x0, x8, x9
	cinc x1, x12, hs
```
## `div_round_i128_unb_pow2`
```asm
	mov x8, #-1
	lsl x8, x8, x2
	and x9, x2, #0x7f
	tst x9, #0x40
	csel x10, xzr, x8, ne
	mvn w11, w2
	mov x12, #9223372036854775807
	lsr x12, x12, x11
	orr x12, x8, x12
	csel x8, x8, x12, ne
	bic x8, x1, x8
	bic x10, x0, x10
	subs x10, x10, x1, lsr #63
	sbcs x8, x8, xzr
	csel x8, xzr, x8, lo
	csel x10, xzr, x10, lo
	tst x9, #0x40
	asr x9, x1, x2
	asr x12, x1, #63
	csel x12, x12, x9, ne
	lsr x13, x0, x2
	lsl x14, x1, #1
	lsl x14, x14, x11
	orr x13, x14, x13
	csel x9, x9, x13, ne
	extr x8, x8, x10, #63
	lsl x10, x10, #1
	lsr x13, x8, x2
	lsl x8, x8, #1
	lsl x8, x8, x11
	lsr x10, x10, x2
	orr x8, x8, x10
	csel x8, x13, x8, ne
	and x8, x8, #0x1
	adds x0, x8, x9
	cinc x1, x12, hs
```
## `div_round_u8_pow2`
```asm
	and w8, w0, #0xff
	mov w9, #1
	lsl w9, w9, w1
	and w9, w9, #0xfe
	lsr w8, w8, w1
	tst w0, w9, lsr #1
	cinc w0, w8, ne
```
## `div_round_u8_unb_pow2`
```asm
	and w8, w0, #0xff
	and x9, x1, #0x7
	mov w10, #1
	lsl w10, w10, w9
	lsr w8, w8, w9
	tst w0, w10, lsr #1
	cinc w0, w8, ne
```
## `div_round_u16_pow2`
```asm
	and w8, w0, #0xffff
	mov w9, #1
	lsl w9, w9, w1
	and w9, w9, #0xfffe
	lsr w8, w8, w1
	tst w0, w9, lsr #1
	cinc w0, w8, ne
```
## `div_round_u16_unb_pow2`
```asm
	and w8, w0, #0xffff
	and x9, x1, #0xf
	mov w10, #1
	lsl w10, w10, w9
	lsr w8, w8, w9
	tst w0, w10, lsr #1
	cinc w0, w8, ne
```
## `div_round_u32_pow2`, `div_round_u32_unb_pow2`
```asm
	mov w8, #1
	lsl w8, w8, w1
	lsr w9, w0, w1
	tst w0, w8, lsr #1
	cinc w0, w9, ne
```
## `div_round_u64_pow2`, `div_round_u64_unb_pow2`
```asm
	mov w8, #1
	lsl x8, x8, x1
	lsr x9, x0, x1
	tst x0, x8, lsr #1
	cinc x0, x9, ne
```
## `div_round_u128_pow2`
```asm
	mov w8, #1
	lsl x8, x8, x2
	and x9, x2, #0xff
	tst x9, #0x40
	csel x9, xzr, x8, ne
	csel x8, x8, xzr, ne
	mvn w10, w2
	extr x9, x8, x9, #1
	lsr x11, x1, x2
	csel x12, xzr, x11, ne
	lsr x13, x0, x2
	lsl x14, x1, #1
	lsl x10, x14, x10
	orr x10, x10, x13
	csel x10, x11, x10, ne
	and x9, x9, x0
	and x8, x1, x8, lsr #1
	orr x8, x9, x8
	cmp x8, #0
	cset w8, ne
	adds x0, x10, x8
	cinc x1, x12, hs
```
## `div_round_u128_unb_pow2`
```asm
	mov w8, #1
	lsl x8, x8, x2
	and x9, x2, #0x7f
	tst x9, #0x40
	csel x9, xzr, x8, ne
	csel x8, x8, xzr, ne
	mvn w10, w2
	extr x9, x8, x9, #1
	lsr x11, x1, x2
	csel x12, xzr, x11, ne
	lsr x13, x0, x2
	lsl x14, x1, #1
	lsl x10, x14, x10
	orr x10, x10, x13
	csel x10, x11, x10, ne
	and x9, x9, x0
	and x8, x1, x8, lsr #1
	orr x8, x9, x8
	cmp x8, #0
	cset w8, ne
	adds x0, x10, x8
	cinc x1, x12, hs
```
## `floor_to_multiple_i8_pow2`, `floor_to_multiple_u8_pow2`
```asm
	and w8, w0, #0xff
	lsr w8, w8, w1
	lsl w0, w8, w1
```
## `floor_to_multiple_i8_unb_pow2`, `floor_to_multiple_u8_unb_pow2`
```asm
	and w8, w0, #0xff
	and x9, x1, #0x7
	lsr w8, w8, w9
	lsl w0, w8, w9
```
## `floor_to_multiple_i16_pow2`, `floor_to_multiple_u16_pow2`
```asm
	and w8, w0, #0xffff
	lsr w8, w8, w1
	lsl w0, w8, w1
```
## `floor_to_multiple_i16_unb_pow2`, `floor_to_multiple_u16_unb_pow2`
```asm
	and w8, w0, #0xffff
	and x9, x1, #0xf
	lsr w8, w8, w9
	lsl w0, w8, w9
```
## `floor_to_multiple_i32_pow2`, `floor_to_multiple_i32_unb_pow2`, `floor_to_multiple_u32_pow2`, `floor_to_multiple_u32_unb_pow2`
```asm
	lsr w8, w0, w1
	lsl w0, w8, w1
```
## `floor_to_multiple_i64_pow2`, `floor_to_multiple_i64_unb_pow2`, `floor_to_multiple_u64_pow2`, `floor_to_multiple_u64_unb_pow2`
```asm
	lsr x8, x0, x1
	lsl x0, x8, x1
```
## `floor_to_multiple_i128_pow2`, `floor_to_multiple_u128_pow2`
```asm
	mov x8, #-1
	lsl x8, x8, x2
	mvn w9, w2
	mov x10, #9223372036854775807
	lsr x9, x10, x9
	orr x9, x8, x9
	and x10, x2, #0xff
	tst x10, #0x40
	csel x9, x8, x9, ne
	csel x8, xzr, x8, ne
	and x0, x8, x0
	and x1, x9, x1
```
## `floor_to_multiple_i128_unb_pow2`, `floor_to_multiple_u128_unb_pow2`
```asm
	mov x8, #-1
	lsl x8, x8, x2
	mvn w9, w2
	mov x10, #9223372036854775807
	lsr x9, x10, x9
	orr x9, x8, x9
	and x10, x2, #0x7f
	tst x10, #0x40
	csel x9, x8, x9, ne
	csel x8, xzr, x8, ne
	and x0, x8, x0
	and x1, x9, x1
```
## `is_multiple_of_i8_pow2`, `is_multiple_of_i8_unb_pow2`, `is_multiple_of_u8_pow2`, `is_multiple_of_u8_unb_pow2`
```asm
	orr w8, w0, #0x100
	rbit w8, w8
	clz w8, w8
	cmp w8, w1, uxtb
	cset w0, hs
```
## `is_multiple_of_i16_pow2`, `is_multiple_of_i16_unb_pow2`, `is_multiple_of_u16_pow2`, `is_multiple_of_u16_unb_pow2`
```asm
	orr w8, w0, #0x10000
	rbit w8, w8
	clz w8, w8
	cmp w8, w1, uxtb
	cset w0, hs
```
## `is_multiple_of_i32_pow2`, `is_multiple_of_i32_unb_pow2`, `is_multiple_of_u32_pow2`, `is_multiple_of_u32_unb_pow2`
```asm
	rbit w8, w0
	clz w8, w8
	cmp w8, w1, uxtb
	cset w0, hs
```
## `is_multiple_of_i64_pow2`, `is_multiple_of_i64_unb_pow2`, `is_multiple_of_u64_pow2`, `is_multiple_of_u64_unb_pow2`
```asm
	rbit x8, x0
	clz x8, x8
	cmp x8, w1, uxtb
	cset w0, hs
```
## `is_multiple_of_i128_pow2`, `is_multiple_of_i128_unb_pow2`, `is_multiple_of_u128_pow2`, `is_multiple_of_u128_unb_pow2`
```asm
	rbit x8, x0
	clz x8, x8
	rbit x9, x1
	clz x9, x9
	add w9, w9, #64
	cmp x0, #0
	csel w8, w8, w9, ne
	cmp w8, w2, uxtb
	cset w0, hs
```
## `mul_i8_pow2`, `mul_i16_pow2`, `mul_i32_pow2`, `mul_i32_unb_pow2`, `mul_u8_pow2`, `mul_u16_pow2`, `mul_u32_pow2`, `mul_u32_unb_pow2`
```asm
	lsl w0, w0, w1
```
## `mul_i8_unb_pow2`, `mul_u8_unb_pow2`
```asm
	and x8, x1, #0x7
	lsl w0, w0, w8
```
## `mul_i16_unb_pow2`, `mul_u16_unb_pow2`
```asm
	and x8, x1, #0xf
	lsl w0, w0, w8
```
## `mul_i64_pow2`, `mul_i64_unb_pow2`, `mul_u64_pow2`, `mul_u64_unb_pow2`
```asm
	lsl x0, x0, x1
```
## `mul_i128_pow2`, `mul_u128_pow2`
```asm
	lsl x9, x0, x2
	and x8, x2, #0xff
	tst x8, #0x40
	csel x8, xzr, x9, ne
	lsl x10, x1, x2
	mvn w11, w2
	lsr x12, x0, #1
	lsr x11, x12, x11
	orr x10, x10, x11
	csel x1, x9, x10, ne
	mov x0, x8
```
## `mul_i128_unb_pow2`, `mul_u128_unb_pow2`
```asm
	lsl x9, x0, x2
	and x8, x2, #0x7f
	tst x8, #0x40
	csel x8, xzr, x9, ne
	lsl x10, x1, x2
	mvn w11, w2
	lsr x12, x0, #1
	lsr x11, x12, x11
	orr x10, x10, x11
	csel x1, x9, x10, ne
	mov x0, x8
```
## `rem_floor_i8_pow2`, `rem_floor_i16_pow2`, `rem_floor_i32_pow2`, `rem_floor_i32_unb_pow2`, `rem_floor_u8_pow2`, `rem_floor_u16_pow2`, `rem_floor_u32_pow2`, `rem_floor_u32_unb_pow2`, `rem_u8_pow2`, `rem_u16_pow2`, `rem_u32_pow2`, `rem_u32_unb_pow2`
```asm
	mov w8, #-1
	lsl w8, w8, w1
	bic w0, w0, w8
```
## `rem_floor_i8_unb_pow2`, `rem_floor_u8_unb_pow2`, `rem_u8_unb_pow2`
```asm
	and x8, x1, #0x7
	mov w9, #-1
	lsl w8, w9, w8
	bic w0, w0, w8
```
## `rem_floor_i16_unb_pow2`, `rem_floor_u16_unb_pow2`, `rem_u16_unb_pow2`
```asm
	and x8, x1, #0xf
	mov w9, #-1
	lsl w8, w9, w8
	bic w0, w0, w8
```
## `rem_floor_i64_pow2`, `rem_floor_i64_unb_pow2`, `rem_floor_u64_pow2`, `rem_floor_u64_unb_pow2`, `rem_u64_pow2`, `rem_u64_unb_pow2`
```asm
	mov x8, #-1
	lsl x8, x8, x1
	bic x0, x0, x8
```
## `rem_floor_i128_pow2`, `rem_floor_u128_pow2`, `rem_u128_pow2`
```asm
	mov x8, #-1
	lsl x8, x8, x2
	mvn w9, w2
	mov x10, #9223372036854775807
	lsr x9, x10, x9
	orr x9, x8, x9
	and x10, x2, #0xff
	tst x10, #0x40
	csel x9, x8, x9, ne
	csel x8, xzr, x8, ne
	bic x0, x0, x8
	bic x1, x1, x9
```
## `rem_floor_i128_unb_pow2`, `rem_floor_u128_unb_pow2`, `rem_u128_unb_pow2`
```asm
	mov x8, #-1
	lsl x8, x8, x2
	mvn w9, w2
	mov x10, #9223372036854775807
	lsr x9, x10, x9
	orr x9, x8, x9
	and x10, x2, #0x7f
	tst x10, #0x40
	csel x9, x8, x9, ne
	csel x8, xzr, x8, ne
	bic x0, x0, x8
	bic x1, x1, x9
```
## `rem_i8_pow2`
```asm
	sxtb w8, w0
	mov w9, #-1
	lsl w9, w9, w1
	lsr w8, w8, #7
	bic w8, w8, w9
	add w8, w8, w0
	sxtb w8, w8
	and x9, x1, #0x7
	asr w8, w8, w9
	lsl w8, w8, w1
	sub w0, w0, w8
```
## `rem_i8_unb_pow2`
```asm
	and x8, x1, #0x7
	mov w9, #-1
	lsl w8, w9, w8
	sbfx w9, w0, #7, #1
	bic w9, w9, w8
	add w9, w9, w0
	and w8, w9, w8
	sub w0, w0, w8
```
## `rem_i16_pow2`
```asm
	sxth w8, w0
	mov w9, #-1
	lsl w9, w9, w1
	lsr w8, w8, #15
	bic w8, w8, w9
	add w8, w8, w0
	sxth w8, w8
	and x9, x1, #0xf
	asr w8, w8, w9
	lsl w8, w8, w1
	sub w0, w0, w8
```
## `rem_i16_unb_pow2`
```asm
	and x8, x1, #0xf
	mov w9, #-1
	lsl w8, w9, w8
	sbfx w9, w0, #15, #1
	bic w9, w9, w8
	add w9, w9, w0
	and w8, w9, w8
	sub w0, w0, w8
```
## `rem_i32_pow2`
```asm
	mov w8, #-1
	lsl w8, w8, w1
	asr w9, w0, #31
	bic w8, w9, w8
	add w8, w8, w0
	asr w8, w8, w1
	lsl w8, w8, w1
	sub w0, w0, w8
```
## `rem_i32_unb_pow2`
```asm
	mov w8, #-1
	lsl w8, w8, w1
	asr w9, w0, #31
	bic w9, w9, w8
	add w9, w9, w0
	and w8, w9, w8
	sub w0, w0, w8
```
## `rem_i64_pow2`
```asm
	mov x8, #-1
	lsl x8, x8, x1
	asr x9, x0, #63
	bic x8, x9, x8
	add x8, x8, x0
	asr x8, x8, x1
	lsl x8, x8, x1
	sub x0, x0, x8
```
## `rem_i64_unb_pow2`
```asm
	mov x8, #-1
	lsl x8, x8, x1
	asr x9, x0, #63
	bic x9, x9, x8
	add x9, x9, x0
	and x8, x9, x8
	sub x0, x0, x8
```
## `rem_i128_pow2`
```asm
	mov x8, #-1
	lsl x8, x8, x2
	and x9, x2, #0xff
	tst x9, #0x40
	csel x10, xzr, x8, ne
	mvn w11, w2
	mov x12, #9223372036854775807
	lsr x12, x12, x11
	orr x12, x8, x12
	csel x8, x8, x12, ne
	asr x12, x1, #63
	bic x8, x12, x8
	bic x10, x12, x10
	adds x10, x10, x0
	adc x8, x8, x1
	asr x12, x8, x2
	lsl x13, x8, #1
	lsl x13, x13, x11
	lsr x10, x10, x2
	orr x10, x13, x10
	and x13, x2, #0x7f
	tst x13, #0x40
	csel x10, x12, x10, ne
	asr x8, x8, #63
	csel x8, x8, x12, ne
	tst x9, #0x40
	lsl x8, x8, x2
	lsr x9, x10, #1
	lsr x9, x9, x11
	orr x8, x8, x9
	lsl x9, x10, x2
	csel x8, x9, x8, ne
	csel x9, xzr, x9, ne
	subs x0, x0, x9
	sbc x1, x1, x8
```
## `rem_i128_unb_pow2`
```asm
	mov x8, #-1
	lsl x8, x8, x2
	and x9, x2, #0x7f
	tst x9, #0x40
	csel x9, xzr, x8, ne
	mvn w10, w2
	mov x11, #9223372036854775807
	lsr x10, x11, x10
	orr x10, x8, x10
	csel x8, x8, x10, ne
	asr x10, x1, #63
	bic x11, x10, x8
	bic x10, x10, x9
	adds x10, x10, x0
	and x9, x10, x9
	adc x10, x11, x1
	and x8, x10, x8
	subs x0, x0, x9
	sbc x1, x1, x8
```
## `round_to_multiple_i8_pow2`
```asm
	and w8, w0, #0x80
	mov w9, #1
	lsl w9, w9, w1
	ubfx w10, w9, #1, #7
	subs w8, w10, w8, lsr #7
	csel w8, wzr, w8, lo
	add w8, w8, w0
	neg w9, w9
	and w0, w8, w9
```
## `round_to_multiple_i8_unb_pow2`
```asm
	and w8, w0, #0x80
	and x9, x1, #0x7
	mov w10, #1
	lsl w9, w10, w9
	lsr w10, w9, #1
	subs w8, w10, w8, lsr #7
	csel w8, wzr, w8, lo
	add w8, w8, w0
	neg w9, w9
	and w0, w8, w9
```
## `round_to_multiple_i16_pow2`
```asm
	and w8, w0, #0x8000
	mov w9, #1
	lsl w9, w9, w1
	ubfx w10, w9, #1, #15
	subs w8, w10, w8, lsr #15
	csel w8, wzr, w8, lo
	add w8, w8, w0
	neg w9, w9
	and w0, w8, w9
```
## `round_to_multiple_i16_unb_pow2`
```asm
	and w8, w0, #0x8000
	and x9, x1, #0xf
	mov w10, #1
	lsl w9, w10, w9
	lsr w10, w9, #1
	subs w8, w10, w8, lsr #15
	csel w8, wzr, w8, lo
	add w8, w8, w0
	neg w9, w9
	and w0, w8, w9
```
## `round_to_multiple_i32_pow2`, `round_to_multiple_i32_unb_pow2`
```asm
	mov w8, #1
	lsl w8, w8, w1
	lsr w9, w8, #1
	subs w9, w9, w0, lsr #31
	csel w9, wzr, w9, lo
	add w9, w9, w0
	neg w8, w8
	and w0, w9, w8
```
## `round_to_multiple_i64_pow2`, `round_to_multiple_i64_unb_pow2`
```asm
	mov w8, #1
	lsl x8, x8, x1
	lsr x9, x8, #1
	subs x9, x9, x0, lsr #63
	csel x9, xzr, x9, lo
	add x9, x9, x0
	neg x8, x8
	and x0, x9, x8
```
## `round_to_multiple_i128_pow2`
```asm
	mov w8, #1
	lsl x8, x8, x2
	and x9, x2, #0xff
	tst x9, #0x40
	csel x9, xzr, x8, ne
	csel x8, x8, xzr, ne
	extr x10, x8, x9, #1
	lsr x11, x8, #1
	subs x10, x10, x1, lsr #63
	sbcs x11, x11, xzr
	csel x11, xzr, x11, lo
	csel x10, xzr, x10, lo
	adds x10, x10, x0
	adc x11, x11, x1
	negs x9, x9
	and x0, x10, x9
	ngc x8, x8
	and x1, x11, x8
```
## `round_to_multiple_i128_unb_pow2`
```asm
	mov w8, #1
	lsl x8, x8, x2
	and x9, x2, #0x7f
	tst x9, #0x40
	csel x9, xzr, x8, ne
	csel x8, x8, xzr, ne
	extr x10, x8, x9, #1
	lsr x11, x8, #1
	subs x10, x10, x1, lsr #63
	sbcs x11, x11, xzr
	csel x11, xzr, x11, lo
	csel x10, xzr, x10, lo
	adds x10, x10, x0
	adc x11, x11, x1
	negs x9, x9
	and x0, x10, x9
	ngc x8, x8
	and x1, x11, x8
```
## `round_to_multiple_u8_pow2`
```asm
	mov w8, #1
	lsl w8, w8, w1
	and w9, w8, #0xfe
	add w9, w0, w9, lsr #1
	neg w8, w8
	and w0, w9, w8
```
## `round_to_multiple_u8_unb_pow2`
```asm
	and x8, x1, #0x7
	mov w9, #1
	lsl w8, w9, w8
	add w9, w0, w8, lsr #1
	neg w8, w8
	and w0, w9, w8
```
## `round_to_multiple_u16_pow2`
```asm
	mov w8, #1
	lsl w8, w8, w1
	and w9, w8, #0xfffe
	add w9, w0, w9, lsr #1
	neg w8, w8
	and w0, w9, w8
```
## `round_to_multiple_u16_unb_pow2`
```asm
	and x8, x1, #0xf
	mov w9, #1
	lsl w8, w9, w8
	add w9, w0, w8, lsr #1
	neg w8, w8
	and w0, w9, w8
```
## `round_to_multiple_u32_pow2`, `round_to_multiple_u32_unb_pow2`
```asm
	mov w8, #1
	lsl w8, w8, w1
	add w9, w0, w8, lsr #1
	neg w8, w8
	and w0, w9, w8
```
## `round_to_multiple_u64_pow2`, `round_to_multiple_u64_unb_pow2`
```asm
	mov w8, #1
	lsl x8, x8, x1
	add x9, x0, x8, lsr #1
	neg x8, x8
	and x0, x9, x8
```
## `round_to_multiple_u128_pow2`
```asm
	mov w8, #1
	lsl x8, x8, x2
	and x9, x2, #0xff
	tst x9, #0x40
	csel x9, xzr, x8, ne
	csel x8, x8, xzr, ne
	extr x10, x8, x9, #1
	lsr x11, x8, #1
	adds x10, x10, x0
	adc x11, x11, x1
	negs x9, x9
	and x0, x10, x9
	ngc x8, x8
	and x1, x11, x8
```
## `round_to_multiple_u128_unb_pow2`
```asm
	mov w8, #1
	lsl x8, x8, x2
	and x9, x2, #0x7f
	tst x9, #0x40
	csel x9, xzr, x8, ne
	csel x8, x8, xzr, ne
	extr x10, x8, x9, #1
	lsr x11, x8, #1
	adds x10, x10, x0
	adc x11, x11, x1
	negs x9, x9
	and x0, x10, x9
	ngc x8, x8
	and x1, x11, x8
```
## `unbounded_ceil_to_multiple_i8_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-1
	lsl w9, w9, w1
	mvn w10, w9
	sxtb w11, w0
	add w10, w11, w10, sxtb
	cmp w10, w10, sxtb
	cset w12, eq
	and w9, w10, w9
	cmp w11, #1
	cset w10, lt
	cmp w8, #8
	csel w1, w9, wzr, lo
	csel w0, w12, w10, lo
```
## `unbounded_ceil_to_multiple_i16_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-1
	lsl w9, w9, w1
	mvn w10, w9
	sxth w11, w0
	add w10, w11, w10, sxth
	cmp w10, w10, sxth
	cset w12, eq
	and w9, w10, w9
	cmp w11, #1
	cset w10, lt
	cmp w8, #16
	csel w1, w9, wzr, lo
	csel w0, w12, w10, lo
```
## `unbounded_ceil_to_multiple_i32_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-1
	lsl w9, w9, w1
	mvn w10, w9
	adds w10, w0, w10
	and w9, w10, w9
	cset w10, vc
	cmp w0, #1
	cset w11, lt
	cmp w8, #32
	csel w1, w9, wzr, lo
	csel w0, w10, w11, lo
```
## `unbounded_ceil_to_multiple_i64_unb_pow2`
```asm
	and w8, w1, #0xff
	mov x9, #-1
	lsl x9, x9, x1
	mvn x10, x9
	adds x10, x0, x10
	and x9, x10, x9
	cset w10, vc
	cmp x0, #1
	cset w11, lt
	cmp w8, #64
	csel w0, w10, w11, lo
	csel x1, x9, xzr, lo
```
## `unbounded_ceil_to_multiple_i128_unb_pow2`
```asm
	tbnz w2, #7, .LBB345_3
	mov x9, #-1
	lsl x10, x9, x2
	and x9, x2, #0xff
	tst x9, #0x40
	csel x9, xzr, x10, ne
	mvn w11, w2
	mov x12, #9223372036854775807
	lsr x11, x12, x11
	orr x11, x10, x11
	csel x10, x10, x11, ne
	mvn x12, x10
	mvn x11, x9
	adds x11, x0, x11
	adcs x12, x1, x12
	cset w13, vs
	tbnz w13, #0, .LBB345_4
	and x9, x11, x9
	and x10, x12, x10
	stp x9, x10, [x8, #16]
	mov w9, #1
	stp x9, xzr, [x8]
	ret
.LBB345_3:
	cmp x0, #1
	sbcs xzr, x1, xzr
	b.lt .LBB345_5
.LBB345_4:
	mov x9, #0
	stp x9, xzr, [x8]
	ret
.LBB345_5:
	mov x9, #0
	mov x10, #0
	stp x9, x10, [x8, #16]
	mov w9, #1
	stp x9, xzr, [x8]
```
## `unbounded_ceil_to_multiple_u8_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-1
	lsl w9, w9, w1
	mov w10, #255
	sub w11, w9, w0
	bic w10, w10, w11
	cmp w10, w0, uxtb
	cset w10, hs
	bic w9, w9, w11
	tst w0, #0xff
	cset w11, eq
	cmp w8, #8
	csel w1, w9, wzr, lo
	csel w0, w10, w11, lo
```
## `unbounded_ceil_to_multiple_u16_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-1
	lsl w9, w9, w1
	mov w10, #65535
	sub w11, w9, w0
	bic w10, w10, w11
	cmp w10, w0, uxth
	cset w10, hs
	bic w9, w9, w11
	tst w0, #0xffff
	cset w11, eq
	cmp w8, #16
	csel w1, w9, wzr, lo
	csel w0, w10, w11, lo
```
## `unbounded_ceil_to_multiple_u32_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-1
	lsl w9, w9, w1
	mvn w10, w9
	add w10, w0, w10
	cmp w10, w0
	cset w10, hs
	sub w11, w9, w0
	bic w9, w9, w11
	cmp w0, #0
	cset w11, eq
	cmp w8, #32
	csel w1, w9, wzr, lo
	csel w0, w10, w11, lo
```
## `unbounded_ceil_to_multiple_u64_unb_pow2`
```asm
	and w8, w1, #0xff
	mov x9, #-1
	lsl x9, x9, x1
	mvn x10, x9
	add x10, x0, x10
	cmp x10, x0
	cset w10, hs
	sub x11, x9, x0
	bic x9, x9, x11
	cmp x0, #0
	cset w11, eq
	cmp w8, #64
	csel w0, w10, w11, lo
	csel x1, x9, xzr, lo
```
## `unbounded_ceil_to_multiple_u128_unb_pow2`
```asm
	tbnz w2, #7, .LBB350_3
	mov x9, #-1
	lsl x10, x9, x2
	and x9, x2, #0xff
	tst x9, #0x40
	csel x9, xzr, x10, ne
	mvn w11, w2
	mov x12, #9223372036854775807
	lsr x11, x12, x11
	orr x11, x10, x11
	csel x10, x10, x11, ne
	mvn x12, x10
	mvn x11, x9
	adds x11, x0, x11
	adcs x12, x1, x12
	cset w13, hs
	tbnz w13, #0, .LBB350_4
	and x9, x11, x9
	and x10, x12, x10
	stp x9, x10, [x8, #16]
	mov w9, #1
	stp x9, xzr, [x8]
	ret
.LBB350_3:
	orr x9, x0, x1
	cbz x9, .LBB350_5
.LBB350_4:
	mov x9, #0
	stp x9, xzr, [x8]
	ret
.LBB350_5:
	mov x10, #0
	stp x9, x10, [x8, #16]
	mov w9, #1
	stp x9, xzr, [x8]
```
## `unbounded_div_ceil_i8_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-1
	lsl w9, w9, w1
	sxtb w10, w0
	asr w11, w10, w1
	bic w9, w0, w9
	tst w9, #0xff
	cinc w9, w11, ne
	cmp w10, #0
	cset w10, gt
	cmp w8, #8
	csel w0, w9, w10, lo
```
## `unbounded_div_ceil_i16_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-1
	lsl w9, w9, w1
	sxth w10, w0
	asr w11, w10, w1
	bic w9, w0, w9
	tst w9, #0xffff
	cinc w9, w11, ne
	cmp w10, #0
	cset w10, gt
	cmp w8, #16
	csel w0, w9, w10, lo
```
## `unbounded_div_ceil_i32_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-1
	lsl w9, w9, w1
	asr w10, w0, w1
	bics wzr, w0, w9
	cinc w9, w10, ne
	cmp w0, #0
	cset w10, gt
	cmp w8, #32
	csel w0, w9, w10, lo
```
## `unbounded_div_ceil_i64_unb_pow2`
```asm
	and w8, w1, #0xff
	mov x9, #-1
	lsl x9, x9, x1
	asr x10, x0, x1
	bics xzr, x0, x9
	cinc x9, x10, ne
	cmp x0, #0
	cset w10, gt
	cmp w8, #64
	csel x0, x9, x10, lo
```
## `unbounded_div_ceil_i128_unb_pow2`
```asm
	tbnz w2, #7, .LBB355_2
	mov x8, #-1
	lsl x8, x8, x2
	mvn w9, w2
	mov x10, #9223372036854775807
	lsr x10, x10, x9
	orr x10, x8, x10
	and x11, x2, #0xff
	tst x11, #0x40
	csel x10, x8, x10, ne
	csel x8, xzr, x8, ne
	asr x11, x1, x2
	asr x12, x1, #63
	csel x12, x12, x11, ne
	lsr x13, x0, x2
	lsl x14, x1, #1
	lsl x9, x14, x9
	orr x9, x9, x13
	csel x9, x11, x9, ne
	bic x8, x0, x8
	bic x10, x1, x10
	orr x8, x8, x10
	cmp x8, #0
	cset w8, ne
	adds x0, x9, x8
	cinc x1, x12, hs
	ret
.LBB355_2:
	cmp xzr, x0
	ngcs xzr, x1
	cset w0, lt
	mov x1, #0
```
## `unbounded_div_ceil_u8_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-1
	lsl w9, w9, w1
	and w10, w0, #0xff
	lsr w11, w10, w1
	bics wzr, w10, w9
	cinc w9, w11, ne
	tst w0, #0xff
	cset w10, ne
	cmp w8, #8
	csel w0, w9, w10, lo
```
## `unbounded_div_ceil_u16_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-1
	lsl w9, w9, w1
	and w10, w0, #0xffff
	lsr w11, w10, w1
	bics wzr, w10, w9
	cinc w9, w11, ne
	tst w0, #0xffff
	cset w10, ne
	cmp w8, #16
	csel w0, w9, w10, lo
```
## `unbounded_div_ceil_u32_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-1
	lsl w9, w9, w1
	lsr w10, w0, w1
	bics wzr, w0, w9
	cinc w9, w10, ne
	cmp w0, #0
	cset w10, ne
	cmp w8, #32
	csel w0, w9, w10, lo
```
## `unbounded_div_ceil_u64_unb_pow2`
```asm
	and w8, w1, #0xff
	mov x9, #-1
	lsl x9, x9, x1
	lsr x10, x0, x1
	bics xzr, x0, x9
	cinc x9, x10, ne
	cmp x0, #0
	cset w10, ne
	cmp w8, #64
	csel x0, x9, x10, lo
```
## `unbounded_div_ceil_u128_unb_pow2`
```asm
	tbnz w2, #7, .LBB360_2
	mov x8, #-1
	lsl x8, x8, x2
	mvn w9, w2
	mov x10, #9223372036854775807
	lsr x10, x10, x9
	orr x10, x8, x10
	and x11, x2, #0xff
	tst x11, #0x40
	csel x10, x8, x10, ne
	csel x8, xzr, x8, ne
	lsr x11, x1, x2
	csel x12, xzr, x11, ne
	lsr x13, x0, x2
	lsl x14, x1, #1
	lsl x9, x14, x9
	orr x9, x9, x13
	csel x9, x11, x9, ne
	bic x8, x0, x8
	bic x10, x1, x10
	orr x8, x8, x10
	cmp x8, #0
	cset w8, ne
	adds x0, x9, x8
	cinc x1, x12, hs
	ret
.LBB360_2:
	orr x9, x0, x1
	cmp x9, #0
	cset w0, ne
	mov x1, #0
```
## `unbounded_div_floor_i8_unb_pow2`
```asm
	sxtb w8, w0
	and w9, w1, #0xff
	mov w10, #7
	cmp w9, #7
	csel w9, w9, w10, lo
	asr w0, w8, w9
```
## `unbounded_div_floor_i16_unb_pow2`
```asm
	sxth w8, w0
	and w9, w1, #0xff
	mov w10, #15
	cmp w9, #15
	csel w9, w9, w10, lo
	asr w0, w8, w9
```
## `unbounded_div_floor_i32_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #31
	cmp w8, #31
	csel w8, w8, w9, lo
	asr w0, w0, w8
```
## `unbounded_div_floor_i64_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #63
	cmp w8, #63
	csel w8, w8, w9, lo
	asr x0, x0, x8
```
## `unbounded_div_floor_i128_unb_pow2`
```asm
	sxtb w8, w2
	cmn w8, #1
	mov w8, #127
	csel w9, w2, w8, gt
	asr x10, x1, x9
	and x8, x9, #0xff
	asr x11, x1, #63
	tst x8, #0x40
	csel x8, x11, x10, ne
	lsr x11, x0, x9
	mvn w9, w9
	lsl x12, x1, #1
	lsl x9, x12, x9
	orr x9, x9, x11
	csel x0, x10, x9, ne
	mov x1, x8
```
## `unbounded_div_floor_u8_unb_pow2`, `unbounded_div_u8_unb_pow2`
```asm
	and w8, w0, #0xff
	lsr w8, w8, w1
	tst w1, #0xf8
	csel w0, w8, wzr, eq
```
## `unbounded_div_floor_u16_unb_pow2`, `unbounded_div_u16_unb_pow2`
```asm
	and w8, w0, #0xffff
	lsr w8, w8, w1
	tst w1, #0xf0
	csel w0, w8, wzr, eq
```
## `unbounded_div_floor_u32_unb_pow2`, `unbounded_div_u32_unb_pow2`
```asm
	lsr w8, w0, w1
	tst w1, #0xe0
	csel w0, w8, wzr, eq
```
## `unbounded_div_floor_u64_unb_pow2`, `unbounded_div_u64_unb_pow2`
```asm
	lsr x8, x0, x1
	tst w1, #0xc0
	csel x0, x8, xzr, eq
```
## `unbounded_div_floor_u128_unb_pow2`, `unbounded_div_u128_unb_pow2`
```asm
	sxtb w8, w2
	lsr x9, x0, x2
	mvn w10, w2
	lsl x11, x1, #1
	lsl x10, x11, x10
	orr x9, x10, x9
	lsr x10, x1, x2
	and x11, x2, #0xff
	tst x11, #0x40
	csel x9, x10, x9, ne
	csel x10, xzr, x10, ne
	cmn w8, #1
	csel x1, x10, xzr, gt
	csel x0, x9, xzr, gt
```
## `unbounded_div_i8_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-1
	lsl w9, w9, w1
	sxtb w10, w0
	lsr w10, w10, #7
	bic w9, w10, w9
	add w9, w9, w0
	sxtb w9, w9
	asr w9, w9, w1
	cmp w8, #7
	csel w0, wzr, w9, hi
```
## `unbounded_div_i16_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-1
	lsl w9, w9, w1
	sxth w10, w0
	lsr w10, w10, #15
	bic w9, w10, w9
	add w9, w9, w0
	sxth w9, w9
	asr w9, w9, w1
	cmp w8, #15
	csel w0, wzr, w9, hi
```
## `unbounded_div_i32_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-1
	lsl w9, w9, w1
	asr w10, w0, #31
	bic w9, w10, w9
	add w9, w9, w0
	asr w9, w9, w1
	cmp w8, #31
	csel w0, wzr, w9, hi
```
## `unbounded_div_i64_unb_pow2`
```asm
	and w8, w1, #0xff
	cmp w8, #63
	mov x8, #-1
	lsl x8, x8, x1
	asr x9, x0, #63
	bic x8, x9, x8
	add x8, x8, x0
	asr x8, x8, x1
	csel x0, xzr, x8, hi
```
## `unbounded_div_i128_unb_pow2`
```asm
	tbnz w2, #7, .LBB375_2
	mov x8, #-1
	lsl x8, x8, x2
	and x9, x2, #0xff
	tst x9, #0x40
	csel x10, xzr, x8, ne
	mvn w11, w2
	mov x12, #9223372036854775807
	lsr x12, x12, x11
	orr x12, x8, x12
	csel x8, x8, x12, ne
	asr x12, x1, #63
	bic x8, x12, x8
	bic x10, x12, x10
	adds x10, x10, x0
	adc x8, x8, x1
	tst x9, #0x40
	asr x9, x8, x2
	asr x12, x8, #63
	csel x1, x12, x9, ne
	lsl x8, x8, #1
	lsl x8, x8, x11
	lsr x10, x10, x2
	orr x8, x8, x10
	csel x0, x9, x8, ne
	ret
.LBB375_2:
	mov x0, #0
	mov x1, #0
```
## `unbounded_div_round_i8_unb_pow2`
```asm
	and w8, w1, #0xff
	and w9, w0, #0xff
	cmp w8, #8
	csetm w10, eq
	cmp w9, #128
	csel w10, wzr, w10, ne
	mov w11, #-1
	lsl w11, w11, w1
	sxtb w12, w0
	asr w12, w12, w1
	bic w11, w9, w11
	subs w9, w11, w9, lsr #7
	csel w9, wzr, w9, lo
	ubfiz w9, w9, #1, #7
	lsr w9, w9, w1
	and w9, w9, #0x1
	add w9, w9, w12
	cmp w8, #7
	csel w0, w10, w9, hi
```
## `unbounded_div_round_i16_unb_pow2`
```asm
	and w8, w1, #0xff
	and w9, w0, #0xffff
	cmp w8, #16
	csetm w10, eq
	cmp w9, #8, lsl #12
	csel w10, wzr, w10, ne
	mov w11, #-1
	lsl w11, w11, w1
	sxth w12, w0
	asr w12, w12, w1
	bic w11, w9, w11
	subs w9, w11, w9, lsr #15
	csel w9, wzr, w9, lo
	ubfiz w9, w9, #1, #15
	lsr w9, w9, w1
	and w9, w9, #0x1
	add w9, w9, w12
	cmp w8, #15
	csel w0, w10, w9, hi
```
## `unbounded_div_round_i32_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-2147483648
	cmp w8, #32
	csetm w10, eq
	cmp w0, w9
	csel w9, wzr, w10, ne
	mov w10, #-1
	lsl w10, w10, w1
	asr w11, w0, w1
	bic w10, w0, w10
	subs w10, w10, w0, lsr #31
	csel w10, wzr, w10, lo
	lsl w10, w10, #1
	lsr w10, w10, w1
	and w10, w10, #0x1
	add w10, w10, w11
	cmp w8, #31
	csel w0, w9, w10, hi
```
## `unbounded_div_round_i64_unb_pow2`
```asm
	and w8, w1, #0xff
	mov x9, #-9223372036854775808
	cmp w8, #64
	csetm x10, eq
	cmp x0, x9
	csel x9, xzr, x10, ne
	mov x10, #-1
	lsl x10, x10, x1
	asr x11, x0, x1
	bic x10, x0, x10
	subs x10, x10, x0, lsr #63
	csel x10, xzr, x10, lo
	lsl x10, x10, #1
	lsr x10, x10, x1
	and x10, x10, #0x1
	add x10, x10, x11
	cmp w8, #63
	csel x0, x9, x10, hi
```
## `unbounded_div_round_i128_unb_pow2`
```asm
	tbnz w2, #7, .LBB380_2
	mov x8, #-1
	lsl x8, x8, x2
	and x9, x2, #0xff
	tst x9, #0x40
	csel x10, xzr, x8, ne
	mvn w11, w2
	mov x12, #9223372036854775807
	lsr x12, x12, x11
	orr x12, x8, x12
	csel x8, x8, x12, ne
	bic x8, x1, x8
	bic x10, x0, x10
	subs x10, x10, x1, lsr #63
	sbcs x8, x8, xzr
	csel x8, xzr, x8, lo
	csel x10, xzr, x10, lo
	tst x9, #0x40
	asr x9, x1, x2
	asr x12, x1, #63
	csel x12, x12, x9, ne
	lsr x13, x0, x2
	lsl x14, x1, #1
	lsl x14, x14, x11
	orr x13, x14, x13
	csel x9, x9, x13, ne
	extr x8, x8, x10, #63
	lsl x10, x10, #1
	lsr x13, x8, x2
	lsl x8, x8, #1
	lsl x8, x8, x11
	lsr x10, x10, x2
	orr x8, x8, x10
	csel x8, x13, x8, ne
	and x8, x8, #0x1
	adds x0, x8, x9
	cinc x1, x12, hs
	ret
.LBB380_2:
	eor x8, x1, #0x8000000000000000
	orr x8, x0, x8
	and w9, w2, #0xff
	cmp w9, #128
	csetm x9, eq
	cmp x8, #0
	csel x0, xzr, x9, ne
	csel x1, xzr, x9, ne
```
## `unbounded_div_round_u8_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #1
	lsl w9, w9, w1
	and w9, w9, #0xfe
	and w10, w0, #0xff
	lsr w10, w10, w1
	tst w0, w9, lsr #1
	cinc w9, w10, ne
	cmp w8, #7
	csel w0, wzr, w9, hi
```
## `unbounded_div_round_u16_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #1
	lsl w9, w9, w1
	and w9, w9, #0xfffe
	and w10, w0, #0xffff
	lsr w10, w10, w1
	tst w0, w9, lsr #1
	cinc w9, w10, ne
	cmp w8, #15
	csel w0, wzr, w9, hi
```
## `unbounded_div_round_u32_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #1
	lsl w9, w9, w1
	lsr w10, w0, w1
	tst w0, w9, lsr #1
	cinc w9, w10, ne
	cmp w8, #31
	csel w0, wzr, w9, hi
```
## `unbounded_div_round_u64_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #1
	lsl x9, x9, x1
	lsr x10, x0, x1
	tst x0, x9, lsr #1
	cinc x9, x10, ne
	cmp w8, #63
	csel x0, xzr, x9, hi
```
## `unbounded_div_round_u128_unb_pow2`
```asm
	tbnz w2, #7, .LBB385_2
	mov w8, #1
	lsl x8, x8, x2
	and x9, x2, #0xff
	tst x9, #0x40
	csel x9, xzr, x8, ne
	csel x8, x8, xzr, ne
	extr x9, x8, x9, #1
	lsr x10, x1, x2
	csel x11, xzr, x10, ne
	lsr x12, x0, x2
	mvn w13, w2
	lsl x14, x1, #1
	lsl x13, x14, x13
	orr x12, x13, x12
	csel x10, x10, x12, ne
	and x9, x9, x0
	and x8, x1, x8, lsr #1
	orr x8, x9, x8
	cmp x8, #0
	cset w8, ne
	adds x0, x10, x8
	cinc x1, x11, hs
	ret
.LBB385_2:
	mov x0, #0
	mov x1, #0
```
## `unbounded_floor_to_multiple_i8_unb_pow2`
```asm
	and w8, w1, #0xff
	sxtb w9, w0
	and w10, w0, #0xff
	lsr w10, w10, w1
	lsl w10, w10, w1
	cmn w9, #1
	cset w9, gt
	cmp w8, #8
	csel w1, w10, wzr, lo
	csinc w0, w9, wzr, hs
```
## `unbounded_floor_to_multiple_i16_unb_pow2`
```asm
	and w8, w1, #0xff
	and w9, w0, #0xffff
	lsr w9, w9, w1
	lsl w9, w9, w1
	ubfx w10, w0, #15, #1
	eor w10, w10, #0x1
	cmp w8, #16
	csel w1, w9, wzr, lo
	csinc w0, w10, wzr, hs
```
## `unbounded_floor_to_multiple_i32_unb_pow2`
```asm
	and w8, w1, #0xff
	lsr w9, w0, w1
	lsl w9, w9, w1
	mvn w10, w0
	lsr w10, w10, #31
	cmp w8, #32
	csel w1, w9, wzr, lo
	csinc w0, w10, wzr, hs
```
## `unbounded_floor_to_multiple_i64_unb_pow2`
```asm
	and w8, w1, #0xff
	lsr x9, x0, x1
	lsl x9, x9, x1
	cmn x0, #1
	cset w10, gt
	cmp w8, #64
	csel x1, x9, xzr, lo
	csinc w0, w10, wzr, hs
```
## `unbounded_floor_to_multiple_i128_unb_pow2`
```asm
	tbnz w2, #7, .LBB395_2
	mov x9, #-1
	lsl x9, x9, x2
	and x10, x2, #0xff
	tst x10, #0x40
	csel x10, xzr, x9, ne
	mvn w11, w2
	mov x12, #9223372036854775807
	lsr x11, x12, x11
	orr x11, x9, x11
	csel x9, x9, x11, ne
	and x9, x9, x1
	and x10, x10, x0
	stp x10, x9, [x8, #16]
	mov w9, #1
	stp x9, xzr, [x8]
	ret
.LBB395_2:
	tbnz x1, #63, .LBB395_4
	mov w9, #1
	stp x9, xzr, [x8]
	stp xzr, xzr, [x8, #16]
	ret
.LBB395_4:
	stp xzr, xzr, [x8]
```
## `unbounded_floor_to_multiple_u8_unb_pow2`
```asm
	and w8, w0, #0xff
	lsr w8, w8, w1
	lsl w8, w8, w1
	tst w1, #0xf8
	csel w0, w8, wzr, eq
```
## `unbounded_floor_to_multiple_u16_unb_pow2`
```asm
	and w8, w0, #0xffff
	lsr w8, w8, w1
	lsl w8, w8, w1
	tst w1, #0xf0
	csel w0, w8, wzr, eq
```
## `unbounded_floor_to_multiple_u32_unb_pow2`
```asm
	lsr w8, w0, w1
	lsl w8, w8, w1
	tst w1, #0xe0
	csel w0, w8, wzr, eq
```
## `unbounded_floor_to_multiple_u64_unb_pow2`
```asm
	lsr x8, x0, x1
	lsl x8, x8, x1
	tst w1, #0xc0
	csel x0, x8, xzr, eq
```
## `unbounded_floor_to_multiple_u128_unb_pow2`
```asm
	sxtb w8, w2
	mov x9, #-1
	lsl x9, x9, x2
	and x10, x2, #0xff
	tst x10, #0x40
	csel x10, xzr, x9, ne
	mvn w11, w2
	mov x12, #9223372036854775807
	lsr x11, x12, x11
	orr x11, x9, x11
	csel x9, x9, x11, ne
	and x9, x9, x1
	and x10, x10, x0
	cmn w8, #1
	csel x0, x10, xzr, gt
	csel x1, x9, xzr, gt
```
## `unbounded_is_multiple_of_i8_unb_pow2`, `unbounded_is_multiple_of_u8_unb_pow2`
```asm
	and w8, w1, #0xff
	tst w0, #0xff
	rbit w9, w0
	clz w9, w9
	ccmp w9, w8, #2, ne
	cset w0, hs
```
## `unbounded_is_multiple_of_i16_unb_pow2`, `unbounded_is_multiple_of_u16_unb_pow2`
```asm
	tst w0, #0xffff
	rbit w8, w0
	clz w8, w8
	and w9, w1, #0xff
	ccmp w8, w9, #2, ne
	cset w0, hs
```
## `unbounded_is_multiple_of_i32_unb_pow2`, `unbounded_is_multiple_of_u32_unb_pow2`
```asm
	cmp w0, #0
	rbit w8, w0
	clz w8, w8
	and w9, w1, #0xff
	ccmp w8, w9, #2, ne
	cset w0, hs
```
## `unbounded_is_multiple_of_i64_unb_pow2`, `unbounded_is_multiple_of_u64_unb_pow2`
```asm
	cmp x0, #0
	rbit x8, x0
	clz x8, x8
	and x9, x1, #0xff
	ccmp x8, x9, #2, ne
	cset w0, hs
```
## `unbounded_is_multiple_of_i128_unb_pow2`, `unbounded_is_multiple_of_u128_unb_pow2`
```asm
	orr x8, x0, x1
	rbit x9, x1
	clz x9, x9
	add w9, w9, #64
	rbit x10, x0
	clz x10, x10
	cmp x0, #0
	csel x9, x10, x9, ne
	cmp x8, #0
	and w8, w2, #0xff
	ccmp w9, w8, #2, ne
	cset w0, hs
```
## `unbounded_rem_i8_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-1
	lsl w9, w9, w1
	sbfx w10, w0, #7, #1
	bic w10, w10, w9
	add w10, w10, w0
	and w9, w10, w9
	sub w9, w0, w9
	cmp w8, #7
	csel w0, w0, w9, hi
```
## `unbounded_rem_i16_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-1
	lsl w9, w9, w1
	sbfx w10, w0, #15, #1
	bic w10, w10, w9
	add w10, w10, w0
	and w9, w10, w9
	sub w9, w0, w9
	cmp w8, #15
	csel w0, w0, w9, hi
```
## `unbounded_rem_i32_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-1
	lsl w9, w9, w1
	asr w10, w0, #31
	bic w10, w10, w9
	add w10, w10, w0
	and w9, w10, w9
	sub w9, w0, w9
	cmp w8, #31
	csel w0, w0, w9, hi
```
## `unbounded_rem_i64_unb_pow2`
```asm
	and w8, w1, #0xff
	mov x9, #-1
	lsl x9, x9, x1
	asr x10, x0, #63
	bic x10, x10, x9
	add x10, x10, x0
	and x9, x10, x9
	sub x9, x0, x9
	cmp w8, #63
	csel x0, x0, x9, hi
```
## `unbounded_rem_i128_unb_pow2`
```asm
	tbnz w2, #7, .LBB415_2
	mov x8, #-1
	lsl x8, x8, x2
	and x9, x2, #0xff
	tst x9, #0x40
	csel x9, xzr, x8, ne
	mvn w10, w2
	mov x11, #9223372036854775807
	lsr x10, x11, x10
	orr x10, x8, x10
	csel x8, x8, x10, ne
	asr x10, x1, #63
	bic x11, x10, x8
	bic x10, x10, x9
	adds x10, x10, x0
	and x9, x10, x9
	adc x10, x11, x1
	and x8, x10, x8
	subs x0, x0, x9
	sbc x1, x1, x8
.LBB415_2:
```
## `unbounded_rem_u8_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-1
	lsl w10, w9, w1
	cmp w8, #8
	csinv w8, w9, w10, hs
	and w0, w8, w0
```
## `unbounded_rem_u16_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-1
	lsl w10, w9, w1
	cmp w8, #16
	csinv w8, w9, w10, hs
	and w0, w8, w0
```
## `unbounded_rem_u32_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-1
	lsl w10, w9, w1
	cmp w8, #32
	csinv w8, w9, w10, hs
	and w0, w8, w0
```
## `unbounded_rem_u64_unb_pow2`
```asm
	and w8, w1, #0xff
	mov x9, #-1
	lsl x10, x9, x1
	cmp w8, #64
	csinv x8, x9, x10, hs
	and x0, x8, x0
```
## `unbounded_rem_u128_unb_pow2`
```asm
	sxtb w8, w2
	mov x9, #-1
	lsl x10, x9, x2
	and x11, x2, #0xff
	tst x11, #0x40
	csel x11, xzr, x10, ne
	mvn w12, w2
	lsr x13, x9, #1
	lsr x12, x13, x12
	orr x12, x10, x12
	csel x10, x10, x12, ne
	cmp w8, #0
	csinv x8, x9, x10, mi
	csinv x9, x9, x11, mi
	and x0, x9, x0
	and x1, x8, x1
```
## `unbounded_round_to_multiple_i8_unb_pow2`
```asm
	and w8, w1, #0xff
	and w9, w0, #0xff
	cmp w9, #128
	ccmp w8, #8, #0, eq
	cset w9, ne
	mov w10, #1
	lsl w10, w10, w1
	ubfx w11, w10, #1, #7
	and w12, w0, #0x80
	subs w11, w11, w12, lsr #7
	csel w11, wzr, w11, lo
	add w11, w11, w0, sxtb
	cmp w11, w11, sxtb
	cset w12, eq
	neg w10, w10
	and w10, w11, w10
	cmp w8, #7
	csel w1, wzr, w10, hi
	csel w0, w9, w12, hi
```
## `unbounded_round_to_multiple_i16_unb_pow2`
```asm
	and w8, w1, #0xff
	and w9, w0, #0xffff
	cmp w9, #8, lsl #12
	ccmp w8, #16, #0, eq
	cset w9, ne
	mov w10, #1
	lsl w10, w10, w1
	ubfx w11, w10, #1, #15
	and w12, w0, #0x8000
	subs w11, w11, w12, lsr #15
	csel w11, wzr, w11, lo
	add w11, w11, w0, sxth
	cmp w11, w11, sxth
	cset w12, eq
	neg w10, w10
	and w10, w11, w10
	cmp w8, #15
	csel w1, wzr, w10, hi
	csel w0, w9, w12, hi
```
## `unbounded_round_to_multiple_i32_unb_pow2`
```asm
	and w8, w1, #0xff
	mov w9, #-2147483648
	cmp w0, w9
	mov w9, #32
	ccmp w8, w9, #0, eq
	cset w9, ne
	mov w10, #1
	lsl w10, w10, w1
	lsr w11, w10, #1
	subs w11, w11, w0, lsr #31
	csel w11, wzr, w11, lo
	adds w11, w0, w11
	cset w12, vc
	neg w10, w10
	and w10, w11, w10
	cmp w8, #31
	csel w1, wzr, w10, hi
	csel w0, w9, w12, hi
```
## `unbounded_round_to_multiple_i64_unb_pow2`
```asm
	and w8, w1, #0xff
	mov x9, #-9223372036854775808
	cmp x0, x9
	mov w9, #64
	ccmp w8, w9, #0, eq
	cset w9, ne
	mov w10, #1
	lsl x10, x10, x1
	lsr x11, x10, #1
	subs x11, x11, x0, lsr #63
	csel x11, xzr, x11, lo
	adds x11, x0, x11
	cset w12, vc
	neg x10, x10
	and x10, x11, x10
	cmp w8, #63
	csel w0, w9, w12, hi
	csel x1, xzr, x10, hi
```
## `unbounded_round_to_multiple_i128_unb_pow2`
```asm
	tbnz w2, #7, .LBB425_5
	mov w9, #1
	lsl x9, x9, x2
	and x10, x2, #0xff
	tst x10, #0x40
	csel x10, x9, xzr, ne
	csel x9, xzr, x9, ne
	extr x11, x10, x9, #1
	lsr x12, x10, #1
	subs x11, x11, x1, lsr #63
	sbcs x12, x12, xzr
	csel x13, xzr, x12, lo
	csel x11, xzr, x11, lo
	adds x12, x0, x11
	adcs x11, x1, x13
	cset w13, vs
	tbnz w13, #0, .LBB425_8
	negs x9, x9
	and x9, x12, x9
	ngc x10, x10
	and x10, x11, x10
.LBB425_3:
	stp x9, x10, [x8, #16]
	mov w9, #1
.LBB425_4:
	stp x9, xzr, [x8]
	ret
.LBB425_5:
	mov x9, #0
	eor x10, x1, #0x8000000000000000
	orr x10, x0, x10
	cbnz x10, .LBB425_7
	and w11, w2, #0xff
	mov x10, x9
	cmp w11, #128
	b.eq .LBB425_4
	b .LBB425_3
.LBB425_7:
	mov x10, x9
	b .LBB425_3
.LBB425_8:
	mov x9, #0
	stp x9, xzr, [x8]
```
## `unbounded_round_to_multiple_u8_unb_pow2`
```asm
	and w8, w1, #0xff
	sxtb w9, w0
	cmn w9, #1
	ccmp w8, #8, #0, le
	cset w9, ne
	mov w10, #1
	lsl w10, w10, w1
	and w11, w10, #0xfe
	add w11, w0, w11, lsr #1
	and w12, w11, #0xff
	cmp w12, w0, uxtb
	cset w12, hs
	neg w10, w10
	and w10, w11, w10
	cmp w8, #7
	csel w1, wzr, w10, hi
	csel w0, w9, w12, hi
```
## `unbounded_round_to_multiple_u16_unb_pow2`
```asm
	and w8, w1, #0xff
	sxth w9, w0
	cmn w9, #1
	ccmp w8, #16, #0, le
	cset w9, ne
	mov w10, #1
	lsl w10, w10, w1
	and w11, w10, #0xfffe
	add w11, w0, w11, lsr #1
	and w12, w11, #0xffff
	cmp w12, w0, uxth
	cset w12, hs
	neg w10, w10
	and w10, w11, w10
	cmp w8, #15
	csel w1, wzr, w10, hi
	csel w0, w9, w12, hi
```
## `unbounded_round_to_multiple_u32_unb_pow2`
```asm
	and w8, w1, #0xff
	cmn w0, #1
	mov w9, #32
	ccmp w8, w9, #0, le
	cset w9, ne
	mov w10, #1
	lsl w10, w10, w1
	add w11, w0, w10, lsr #1
	cmp w11, w0
	cset w12, hs
	neg w10, w10
	and w10, w11, w10
	cmp w8, #31
	csel w1, wzr, w10, hi
	csel w0, w9, w12, hi
```
## `unbounded_round_to_multiple_u64_unb_pow2`
```asm
	and w8, w1, #0xff
	cmn x0, #1
	mov w9, #64
	ccmp w8, w9, #0, le
	cset w9, ne
	mov w10, #1
	lsl x10, x10, x1
	add x11, x0, x10, lsr #1
	cmp x11, x0
	cset w12, hs
	neg x10, x10
	and x10, x11, x10
	cmp w8, #63
	csel w0, w9, w12, hi
	csel x1, xzr, x10, hi
```
## `unbounded_round_to_multiple_u128_unb_pow2`
```asm
	tbnz w2, #7, .LBB430_5
	mov w9, #1
	lsl x9, x9, x2
	and x10, x2, #0xff
	tst x10, #0x40
	csel x10, x9, xzr, ne
	csel x9, xzr, x9, ne
	extr x11, x10, x9, #1
	lsr x13, x10, #1
	adds x12, x11, x0
	adcs x11, x13, x1
	cset w13, hs
	tbnz w13, #0, .LBB430_8
	negs x9, x9
	and x9, x12, x9
	ngc x10, x10
	and x10, x11, x10
.LBB430_3:
	stp x9, x10, [x8, #16]
	mov w9, #1
.LBB430_4:
	stp x9, xzr, [x8]
	ret
.LBB430_5:
	mov x9, #0
	tbz x1, #63, .LBB430_7
	and w11, w2, #0xff
	mov x10, x9
	cmp w11, #128
	b.eq .LBB430_4
	b .LBB430_3
.LBB430_7:
	mov x10, x9
	b .LBB430_3
.LBB430_8:
	mov x9, #0
	stp x9, xzr, [x8]
```
