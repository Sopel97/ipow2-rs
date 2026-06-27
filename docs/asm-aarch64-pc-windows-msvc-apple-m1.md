## `ceil_to_multiple_i8_pow2`
```asm
ceil_to_multiple_i8_pow2:
	mov w8, #-1
	lsl w8, w8, w1
	sub w9, w8, w0
	bic w0, w8, w9
```
## `ceil_to_multiple_i8_unb_pow2`
```asm
ceil_to_multiple_i8_unb_pow2:
	mov w8, #-1
	and x9, x1, #0x7
	lsl w8, w8, w9
	sub w9, w8, w0
	bic w0, w8, w9
```
## `ceil_to_multiple_i16_pow2`
```asm
ceil_to_multiple_i16_pow2:
	mov w8, #-1
	lsl w8, w8, w1
	sub w9, w8, w0
	bic w0, w8, w9
```
## `ceil_to_multiple_i16_unb_pow2`
```asm
ceil_to_multiple_i16_unb_pow2:
	mov w8, #-1
	and x9, x1, #0xf
	lsl w8, w8, w9
	sub w9, w8, w0
	bic w0, w8, w9
```
## `ceil_to_multiple_i32_pow2`
```asm
ceil_to_multiple_i32_pow2:
	mov w8, #-1
	lsl w8, w8, w1
	sub w9, w8, w0
	bic w0, w8, w9
```
## `ceil_to_multiple_i32_unb_pow2`
```asm
ceil_to_multiple_i32_unb_pow2:
	mov w8, #-1
	lsl w8, w8, w1
	sub w9, w8, w0
	bic w0, w8, w9
```
## `ceil_to_multiple_i64_pow2`
```asm
ceil_to_multiple_i64_pow2:
	mov x8, #-1
	lsl x8, x8, x1
	sub x9, x8, x0
	bic x0, x8, x9
```
## `ceil_to_multiple_i64_unb_pow2`
```asm
ceil_to_multiple_i64_unb_pow2:
	mov x8, #-1
	lsl x8, x8, x1
	sub x9, x8, x0
	bic x0, x8, x9
```
## `ceil_to_multiple_i128_pow2`
```asm
ceil_to_multiple_i128_pow2:
	mov x8, #-1
	mvn w9, w2
	mov x10, #9223372036854775807
	lsl x8, x8, x2
	lsr x9, x10, x9
	and x10, x2, #0xff
	tst x10, #0x40
	orr x9, x8, x9
	csel x10, xzr, x8, ne
	csel x8, x8, x9, ne
	subs x9, x10, x0
	sbc x11, x8, x1
	bic x0, x10, x9
	bic x1, x8, x11
```
## `ceil_to_multiple_i128_unb_pow2`
```asm
ceil_to_multiple_i128_unb_pow2:
	mov x8, #-1
	mvn w9, w2
	mov x10, #9223372036854775807
	lsl x8, x8, x2
	lsr x9, x10, x9
	and x10, x2, #0x7f
	tst x10, #0x40
	orr x9, x8, x9
	csel x10, xzr, x8, ne
	csel x8, x8, x9, ne
	subs x9, x10, x0
	sbc x11, x8, x1
	bic x0, x10, x9
	bic x1, x8, x11
```
## `checked_ceil_to_multiple_i8_pow2`
```asm
checked_ceil_to_multiple_i8_pow2:
	mov w8, #-1
	sxtb w9, w0
	lsl w8, w8, w1
	mvn w10, w8
	add w9, w9, w10, sxtb
	cmp w9, w9, sxtb
	and w1, w9, w8
	cset w0, eq
```
## `checked_ceil_to_multiple_i8_unb_pow2`
```asm
checked_ceil_to_multiple_i8_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #7
	b.hi .LBB24_2
	mov w8, #-1
	sxtb w9, w0
	lsl w8, w8, w1
	mvn w10, w8
	add w9, w9, w10, sxtb
	sxtb w10, w9
	cmp w10, w9
	b.eq .LBB24_3
.LBB24_2:
	mov w0, wzr
	ret
.LBB24_3:
	and w1, w9, w8
	mov w0, #1
```
## `checked_ceil_to_multiple_i16_pow2`
```asm
checked_ceil_to_multiple_i16_pow2:
	mov w8, #-1
	sxth w9, w0
	lsl w8, w8, w1
	mvn w10, w8
	add w9, w9, w10, sxth
	cmp w9, w9, sxth
	and w1, w9, w8
	cset w0, eq
```
## `checked_ceil_to_multiple_i16_unb_pow2`
```asm
checked_ceil_to_multiple_i16_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #15
	b.hi .LBB18_2
	mov w8, #-1
	sxth w9, w0
	lsl w8, w8, w1
	mvn w10, w8
	add w9, w9, w10, sxth
	sxth w10, w9
	cmp w10, w9
	b.eq .LBB18_3
.LBB18_2:
	mov w0, wzr
	ret
.LBB18_3:
	and w1, w9, w8
	mov w0, #1
```
## `checked_ceil_to_multiple_i32_pow2`
```asm
checked_ceil_to_multiple_i32_pow2:
	mov w8, #-1
	lsl w8, w8, w1
	mvn w9, w8
	adds w9, w0, w9
	cset w0, vc
	and w1, w9, w8
```
## `checked_ceil_to_multiple_i32_unb_pow2`
```asm
checked_ceil_to_multiple_i32_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #31
	b.hi .LBB20_2
	mov w8, #-1
	lsl w8, w8, w1
	mvn w9, w8
	adds w9, w0, w9
	b.vc .LBB20_3
.LBB20_2:
	mov w0, wzr
	ret
.LBB20_3:
	and w1, w9, w8
	mov w0, #1
```
## `checked_ceil_to_multiple_i64_pow2`
```asm
checked_ceil_to_multiple_i64_pow2:
	mov x8, #-1
	lsl x8, x8, x1
	mvn x9, x8
	adds x9, x0, x9
	cset w0, vc
	and x1, x9, x8
```
## `checked_ceil_to_multiple_i64_unb_pow2`
```asm
checked_ceil_to_multiple_i64_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #63
	b.hi .LBB22_2
	mov x8, #-1
	lsl x8, x8, x1
	mvn x9, x8
	adds x9, x0, x9
	b.vc .LBB22_3
.LBB22_2:
	mov x0, xzr
	ret
.LBB22_3:
	and x1, x9, x8
	mov w0, #1
```
## `checked_ceil_to_multiple_i128_pow2`
```asm
checked_ceil_to_multiple_i128_pow2:
	mov x9, #-1
	mvn w10, w2
	mov x11, #9223372036854775807
	lsl x12, x9, x2
	and x9, x2, #0xff
	lsr x10, x11, x10
	tst x9, #0x40
	csel x9, xzr, x12, ne
	orr x10, x12, x10
	csel x10, x12, x10, ne
	mvn x11, x9
	mvn x12, x10
	adds x11, x0, x11
	adcs x12, x1, x12
	cset w13, vs
	tbnz w13, #0, .LBB15_2
	and x9, x11, x9
	and x10, x12, x10
	stp x9, x10, [x8, #16]
	mov w9, #1
	stp x9, xzr, [x8]
	ret
.LBB15_2:
	stp xzr, xzr, [x8]
```
## `checked_ceil_to_multiple_i128_unb_pow2`
```asm
checked_ceil_to_multiple_i128_unb_pow2:
	tbnz w2, #7, .LBB16_3
	mov x9, #-1
	mvn w10, w2
	mov x11, #9223372036854775807
	lsl x12, x9, x2
	and x9, x2, #0xff
	lsr x10, x11, x10
	tst x9, #0x40
	csel x9, xzr, x12, ne
	orr x10, x12, x10
	csel x10, x12, x10, ne
	mvn x11, x9
	mvn x12, x10
	adds x11, x0, x11
	adcs x12, x1, x12
	cset w13, vs
	tbnz w13, #0, .LBB16_3
	and x9, x11, x9
	and x10, x12, x10
	stp x9, x10, [x8, #16]
	mov w9, #1
	stp x9, xzr, [x8]
	ret
.LBB16_3:
	stp xzr, xzr, [x8]
```
## `checked_ceil_to_multiple_u8_pow2`
```asm
checked_ceil_to_multiple_u8_pow2:
	mov w8, #-1
	mov w10, #255
	lsl w8, w8, w1
	sub w9, w8, w0
	bic w10, w10, w9
	bic w1, w8, w9
	cmp w10, w0, uxtb
	cset w0, hs
```
## `checked_ceil_to_multiple_u8_unb_pow2`
```asm
checked_ceil_to_multiple_u8_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #7
	b.hi .LBB34_2
	mov w8, #-1
	mov w10, #255
	lsl w8, w8, w1
	sub w9, w8, w0
	bic w10, w10, w9
	bic w1, w8, w9
	cmp w10, w0, uxtb
	cset w0, hs
	ret
.LBB34_2:
	mov w0, wzr
```
## `checked_ceil_to_multiple_u16_pow2`
```asm
checked_ceil_to_multiple_u16_pow2:
	mov w8, #-1
	mov w10, #65535
	lsl w8, w8, w1
	sub w9, w8, w0
	bic w10, w10, w9
	bic w1, w8, w9
	cmp w10, w0, uxth
	cset w0, hs
```
## `checked_ceil_to_multiple_u16_unb_pow2`
```asm
checked_ceil_to_multiple_u16_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #15
	b.hi .LBB28_2
	mov w8, #-1
	mov w10, #65535
	lsl w8, w8, w1
	sub w9, w8, w0
	bic w10, w10, w9
	bic w1, w8, w9
	cmp w10, w0, uxth
	cset w0, hs
	ret
.LBB28_2:
	mov w0, wzr
```
## `checked_ceil_to_multiple_u32_pow2`
```asm
checked_ceil_to_multiple_u32_pow2:
	mov w8, #-1
	lsl w9, w8, w1
	mvn w8, w9
	sub w10, w9, w0
	add w8, w0, w8
	bic w1, w9, w10
	cmp w8, w0
	cset w8, hs
	mov w0, w8
```
## `checked_ceil_to_multiple_u32_unb_pow2`
```asm
checked_ceil_to_multiple_u32_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #31
	b.hi .LBB30_2
	mov w8, #-1
	lsl w8, w8, w1
	mvn w9, w8
	add w9, w0, w9
	cmp w9, w0
	sub w9, w8, w0
	cset w0, hs
	bic w1, w8, w9
	ret
.LBB30_2:
	mov w0, wzr
```
## `checked_ceil_to_multiple_u64_pow2`
```asm
checked_ceil_to_multiple_u64_pow2:
	mov x8, #-1
	lsl x9, x8, x1
	mvn x8, x9
	sub x10, x9, x0
	add x8, x0, x8
	bic x1, x9, x10
	cmp x8, x0
	cset w8, hs
	mov x0, x8
```
## `checked_ceil_to_multiple_u64_unb_pow2`
```asm
checked_ceil_to_multiple_u64_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #63
	b.hi .LBB32_2
	mov x8, #-1
	lsl x8, x8, x1
	mvn x9, x8
	add x9, x0, x9
	cmp x9, x0
	sub x9, x8, x0
	cset w0, hs
	bic x1, x8, x9
	ret
.LBB32_2:
	mov x0, xzr
```
## `checked_ceil_to_multiple_u128_pow2`
```asm
checked_ceil_to_multiple_u128_pow2:
	mov x9, #-1
	mvn w10, w2
	mov x11, #9223372036854775807
	lsl x12, x9, x2
	and x9, x2, #0xff
	lsr x10, x11, x10
	tst x9, #0x40
	csel x9, xzr, x12, ne
	orr x10, x12, x10
	csel x10, x12, x10, ne
	mvn x11, x9
	mvn x12, x10
	adds x11, x0, x11
	adcs x12, x1, x12
	cset w13, hs
	tbnz w13, #0, .LBB25_2
	and x9, x11, x9
	and x10, x12, x10
	stp x9, x10, [x8, #16]
	mov w9, #1
	stp x9, xzr, [x8]
	ret
.LBB25_2:
	stp xzr, xzr, [x8]
```
## `checked_ceil_to_multiple_u128_unb_pow2`
```asm
checked_ceil_to_multiple_u128_unb_pow2:
	tbnz w2, #7, .LBB26_3
	mov x9, #-1
	mvn w10, w2
	mov x11, #9223372036854775807
	lsl x12, x9, x2
	and x9, x2, #0xff
	lsr x10, x11, x10
	tst x9, #0x40
	csel x9, xzr, x12, ne
	orr x10, x12, x10
	csel x10, x12, x10, ne
	mvn x11, x9
	mvn x12, x10
	adds x11, x0, x11
	adcs x12, x1, x12
	cset w13, hs
	tbnz w13, #0, .LBB26_3
	and x9, x11, x9
	and x10, x12, x10
	stp x9, x10, [x8, #16]
	mov w9, #1
	stp x9, xzr, [x8]
	ret
.LBB26_3:
	stp xzr, xzr, [x8]
```
## `checked_div_ceil_i8_unb_pow2`
```asm
checked_div_ceil_i8_unb_pow2:
	mov w8, #-1
	sxtb w9, w0
	and w10, w1, #0xff
	lsl w8, w8, w1
	asr w9, w9, w1
	bic w8, w0, w8
	tst w8, #0xff
	cinc w8, w9, ne
	cmp w10, #7
	csel w8, w8, w8, hi
	tst w1, #0xf8
	cset w0, eq
	mov w1, w8
```
## `checked_div_ceil_i16_unb_pow2`
```asm
checked_div_ceil_i16_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #15
	b.hi .LBB36_2
	mov w8, #-1
	sxth w9, w0
	lsl w8, w8, w1
	asr w9, w9, w1
	bic w8, w0, w8
	mov w0, #1
	tst w8, #0xffff
	cinc w1, w9, ne
	ret
.LBB36_2:
	mov w0, wzr
```
## `checked_div_ceil_i32_unb_pow2`
```asm
checked_div_ceil_i32_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #31
	b.hi .LBB37_2
	mov w8, #-1
	asr w9, w0, w1
	lsl w8, w8, w1
	bics wzr, w0, w8
	mov w0, #1
	cinc w1, w9, ne
	ret
.LBB37_2:
	mov w0, wzr
```
## `checked_div_ceil_i64_unb_pow2`
```asm
checked_div_ceil_i64_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #63
	b.hi .LBB38_2
	mov x8, #-1
	asr x9, x0, x1
	lsl x8, x8, x1
	bics xzr, x0, x8
	mov w0, #1
	cinc x1, x9, ne
	ret
.LBB38_2:
	mov x0, xzr
```
## `checked_div_ceil_i128_unb_pow2`
```asm
checked_div_ceil_i128_unb_pow2:
	tbnz w2, #7, .LBB35_2
	mov x9, #-1
	mvn w10, w2
	mov x11, #9223372036854775807
	lsl x9, x9, x2
	lsr x11, x11, x10
	and x12, x2, #0xff
	lsl x13, x1, #1
	tst x12, #0x40
	lsr x14, x0, x2
	orr x11, x9, x11
	csel x12, xzr, x9, ne
	csel x9, x9, x11, ne
	lsl x10, x13, x10
	asr x11, x1, x2
	asr x13, x1, #63
	bic x12, x0, x12
	bic x9, x1, x9
	orr x10, x10, x14
	orr x9, x12, x9
	csel x12, x13, x11, ne
	csel x10, x11, x10, ne
	cmp x9, #0
	cset w9, ne
	adds x9, x10, x9
	cinc x10, x12, hs
	stp x9, x10, [x8, #16]
	mov w9, #1
	stp x9, xzr, [x8]
	ret
.LBB35_2:
	stp xzr, xzr, [x8]
```
## `checked_div_ceil_u8_unb_pow2`
```asm
checked_div_ceil_u8_unb_pow2:
	mov w8, #-1
	and w9, w0, #0xff
	and w11, w1, #0xff
	lsl w8, w8, w1
	lsr w10, w9, w1
	bics wzr, w9, w8
	cinc w8, w10, ne
	cmp w11, #7
	csel w8, w8, w8, hi
	tst w1, #0xf8
	cset w0, eq
	mov w1, w8
```
## `checked_div_ceil_u16_unb_pow2`
```asm
checked_div_ceil_u16_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #15
	b.hi .LBB41_2
	mov w8, #-1
	and w9, w0, #0xffff
	mov w0, #1
	lsl w8, w8, w1
	lsr w10, w9, w1
	bics wzr, w9, w8
	cinc w1, w10, ne
	ret
.LBB41_2:
	mov w0, wzr
```
## `checked_div_ceil_u32_unb_pow2`
```asm
checked_div_ceil_u32_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #31
	b.hi .LBB42_2
	mov w8, #-1
	lsr w9, w0, w1
	lsl w8, w8, w1
	bics wzr, w0, w8
	mov w0, #1
	cinc w1, w9, ne
	ret
.LBB42_2:
	mov w0, wzr
```
## `checked_div_ceil_u64_unb_pow2`
```asm
checked_div_ceil_u64_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #63
	b.hi .LBB43_2
	mov x8, #-1
	lsr x9, x0, x1
	lsl x8, x8, x1
	bics xzr, x0, x8
	mov w0, #1
	cinc x1, x9, ne
	ret
.LBB43_2:
	mov x0, xzr
```
## `checked_div_ceil_u128_unb_pow2`
```asm
checked_div_ceil_u128_unb_pow2:
	tbnz w2, #7, .LBB40_2
	mov x9, #-1
	mvn w10, w2
	mov x11, #9223372036854775807
	lsl x9, x9, x2
	lsr x11, x11, x10
	and x12, x2, #0xff
	lsl x13, x1, #1
	tst x12, #0x40
	lsr x12, x0, x2
	orr x11, x9, x11
	csel x11, x9, x11, ne
	csel x9, xzr, x9, ne
	lsl x10, x13, x10
	lsr x13, x1, x2
	bic x9, x0, x9
	bic x11, x1, x11
	orr x10, x10, x12
	orr x9, x9, x11
	csel x11, xzr, x13, ne
	csel x10, x13, x10, ne
	cmp x9, #0
	cset w9, ne
	adds x9, x10, x9
	cinc x10, x11, hs
	stp x9, x10, [x8, #16]
	mov w9, #1
	stp x9, xzr, [x8]
	ret
.LBB40_2:
	stp xzr, xzr, [x8]
```
## `checked_div_floor_i8_unb_pow2`
```asm
checked_div_floor_i8_unb_pow2:
	sxtb w8, w0
	tst w1, #0xf8
	cset w0, eq
	asr w1, w8, w1
```
## `checked_div_floor_i16_unb_pow2`
```asm
checked_div_floor_i16_unb_pow2:
	sxth w8, w0
	tst w1, #0xf0
	cset w0, eq
	asr w1, w8, w1
```
## `checked_div_floor_i32_unb_pow2`
```asm
checked_div_floor_i32_unb_pow2:
	tst w1, #0xe0
	asr w1, w0, w1
	cset w0, eq
```
## `checked_div_floor_i64_unb_pow2`
```asm
checked_div_floor_i64_unb_pow2:
	tst w1, #0xc0
	asr x1, x0, x1
	cset w0, eq
```
## `checked_div_floor_i128_unb_pow2`
```asm
checked_div_floor_i128_unb_pow2:
	tbnz w2, #7, .LBB45_2
	lsl x9, x1, #1
	mvn w10, w2
	lsr x11, x0, x2
	and x12, x2, #0xff
	asr x13, x1, #63
	lsl x9, x9, x10
	asr x10, x1, x2
	tst x12, #0x40
	orr x9, x9, x11
	csel x11, x13, x10, ne
	csel x9, x10, x9, ne
	stp x9, x11, [x8, #16]
	mov w9, #1
	stp x9, xzr, [x8]
	ret
.LBB45_2:
	stp xzr, xzr, [x8]
```
## `checked_div_floor_u8_unb_pow2`
```asm
checked_div_floor_u8_unb_pow2:
	and w8, w0, #0xff
	tst w1, #0xf8
	lsr w1, w8, w1
	cset w0, eq
```
## `checked_div_floor_u16_unb_pow2`
```asm
checked_div_floor_u16_unb_pow2:
	and w8, w0, #0xffff
	tst w1, #0xf0
	lsr w1, w8, w1
	cset w0, eq
```
## `checked_div_floor_u32_unb_pow2`
```asm
checked_div_floor_u32_unb_pow2:
	tst w1, #0xe0
	lsr w1, w0, w1
	cset w0, eq
```
## `checked_div_floor_u64_unb_pow2`
```asm
checked_div_floor_u64_unb_pow2:
	tst w1, #0xc0
	lsr x1, x0, x1
	cset w0, eq
```
## `checked_div_floor_u128_unb_pow2`
```asm
checked_div_floor_u128_unb_pow2:
	tbnz w2, #7, .LBB50_2
	lsl x9, x1, #1
	mvn w11, w2
	lsr x10, x0, x2
	and x12, x2, #0xff
	lsl x9, x9, x11
	lsr x11, x1, x2
	tst x12, #0x40
	orr x9, x9, x10
	csel x10, xzr, x11, ne
	csel x9, x11, x9, ne
	stp x9, x10, [x8, #16]
	mov w9, #1
	stp x9, xzr, [x8]
	ret
.LBB50_2:
	stp xzr, xzr, [x8]
```
## `checked_div_i8_unb_pow2`
```asm
checked_div_i8_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #7
	b.hi .LBB59_3
	tbnz w0, #7, .LBB59_4
	and w8, w0, #0xff
	lsr w8, w8, w1
.LBB59_3:
	tst w1, #0xf8
	mov w1, w8
	cset w0, eq
	ret
.LBB59_4:
	mov w8, #-1
	lsl w8, w8, w1
	mvn w8, w8
	add w8, w0, w8
	sxtb w8, w8
	asr w8, w8, w1
	tst w1, #0xf8
	mov w1, w8
	cset w0, eq
```
## `checked_div_i16_unb_pow2`
```asm
checked_div_i16_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #15
	b.hi .LBB56_3
	tbnz w0, #15, .LBB56_4
	and w9, w0, #0xffff
	mov w0, #1
	lsr w1, w9, w8
	ret
.LBB56_3:
	mov w0, wzr
	ret
.LBB56_4:
	mov w9, #-1
	lsl w9, w9, w8
	mvn w9, w9
	add w9, w0, w9
	mov w0, #1
	sxth w9, w9
	asr w1, w9, w8
```
## `checked_div_i32_unb_pow2`
```asm
checked_div_i32_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #31
	b.hi .LBB57_3
	tbnz w0, #31, .LBB57_4
	lsr w1, w0, w8
	mov w0, #1
	ret
.LBB57_3:
	mov w0, wzr
	ret
.LBB57_4:
	mov w9, #-1
	lsl w9, w9, w8
	mvn w9, w9
	add w9, w0, w9
	mov w0, #1
	asr w1, w9, w8
```
## `checked_div_i64_unb_pow2`
```asm
checked_div_i64_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #63
	b.hi .LBB58_3
	and x8, x1, #0xff
	tbnz x0, #63, .LBB58_4
	lsr x1, x0, x8
	mov w0, #1
	ret
.LBB58_3:
	mov x0, xzr
	ret
.LBB58_4:
	mov x9, #-1
	lsl x9, x9, x8
	mvn x9, x9
	add x9, x0, x9
	mov w0, #1
	asr x1, x9, x8
```
## `checked_div_i128_unb_pow2`
```asm
checked_div_i128_unb_pow2:
	tbnz w2, #7, .LBB55_3
	tbnz x1, #63, .LBB55_4
	lsl x9, x1, #1
	mov w10, w2
	mvn w11, w2
	lsr x12, x0, x10
	lsl x9, x9, x11
	lsr x11, x1, x10
	and x10, x10, #0xff
	tst x10, #0x40
	orr x10, x9, x12
	csel x9, xzr, x11, ne
	csel x10, x11, x10, ne
	stp x10, x9, [x8, #16]
	mov w10, #1
	stp x10, xzr, [x8]
	ret
.LBB55_3:
	stp xzr, xzr, [x8]
	ret
.LBB55_4:
	mov x9, #-1
	mvn w10, w2
	mov x11, #9223372036854775807
	lsl x9, x9, x2
	and x12, x2, #0xff
	lsr x11, x11, x10
	tst x12, #0x40
	csel x13, xzr, x9, ne
	orr x11, x9, x11
	csel x9, x9, x11, ne
	mvn x11, x13
	mvn x9, x9
	adds x11, x0, x11
	adc x9, x1, x9
	lsr x11, x11, x2
	tst x12, #0x40
	lsl x13, x9, #1
	lsl x10, x13, x10
	asr x13, x9, x2
	asr x9, x9, #63
	orr x10, x10, x11
	csel x9, x9, x13, ne
	csel x10, x13, x10, ne
	stp x10, x9, [x8, #16]
	mov w10, #1
	stp x10, xzr, [x8]
```
## `checked_div_round_i8_unb_pow2`
```asm
checked_div_round_i8_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #7
	b.hi .LBB64_2
	mov w8, #-1
	and w9, w0, #0xff
	lsl w8, w8, w1
	bic w8, w9, w8
	subs w8, w8, w9, lsr #7
	sxtb w9, w0
	csel w8, wzr, w8, lo
	asr w9, w9, w1
	ubfiz w8, w8, #1, #7
	lsr w8, w8, w1
	and w8, w8, #0x1
	add w8, w8, w9
.LBB64_2:
	tst w1, #0xf8
	mov w1, w8
	cset w0, eq
```
## `checked_div_round_i16_unb_pow2`
```asm
checked_div_round_i16_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #15
	b.hi .LBB61_2
	mov w8, #-1
	and w9, w0, #0xffff
	lsl w8, w8, w1
	bic w8, w9, w8
	subs w8, w8, w9, lsr #15
	sxth w9, w0
	mov w0, #1
	csel w8, wzr, w8, lo
	asr w9, w9, w1
	ubfiz w8, w8, #1, #15
	lsr w8, w8, w1
	and w8, w8, #0x1
	add w1, w8, w9
	ret
.LBB61_2:
	mov w0, wzr
```
## `checked_div_round_i32_unb_pow2`
```asm
checked_div_round_i32_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #31
	b.hi .LBB62_2
	mov w8, #-1
	asr w9, w0, w1
	lsl w8, w8, w1
	bic w8, w0, w8
	subs w8, w8, w0, lsr #31
	mov w0, #1
	csel w8, wzr, w8, lo
	lsl w8, w8, #1
	lsr w8, w8, w1
	and w8, w8, #0x1
	add w1, w8, w9
	ret
.LBB62_2:
	mov w0, wzr
```
## `checked_div_round_i64_unb_pow2`
```asm
checked_div_round_i64_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #63
	b.hi .LBB63_2
	mov x8, #-1
	asr x9, x0, x1
	lsl x8, x8, x1
	bic x8, x0, x8
	subs x8, x8, x0, lsr #63
	mov w0, #1
	csel x8, xzr, x8, lo
	lsl x8, x8, #1
	lsr x8, x8, x1
	and x8, x8, #0x1
	add x1, x8, x9
	ret
.LBB63_2:
	mov x0, xzr
```
## `checked_div_round_i128_unb_pow2`
```asm
checked_div_round_i128_unb_pow2:
	tbnz w2, #7, .LBB60_2
	mov x9, #-1
	mvn w10, w2
	mov x11, #9223372036854775807
	lsl x9, x9, x2
	and x12, x2, #0xff
	lsr x11, x11, x10
	tst x12, #0x40
	lsl x14, x1, #1
	csel x13, xzr, x9, ne
	orr x11, x9, x11
	csel x9, x9, x11, ne
	bic x11, x0, x13
	bic x9, x1, x9
	subs x11, x11, x1, lsr #63
	sbcs x9, x9, xzr
	csel x9, xzr, x9, lo
	csel x11, xzr, x11, lo
	tst x12, #0x40
	extr x9, x9, x11, #63
	lsl x11, x11, #1
	asr x12, x1, x2
	lsl x13, x9, #1
	lsr x11, x11, x2
	lsr x9, x9, x2
	lsl x13, x13, x10
	lsl x10, x14, x10
	lsr x14, x0, x2
	orr x11, x13, x11
	asr x13, x1, #63
	orr x10, x10, x14
	csel x9, x9, x11, ne
	csel x10, x12, x10, ne
	and x9, x9, #0x1
	csel x11, x13, x12, ne
	adds x9, x9, x10
	cinc x10, x11, hs
	stp x9, x10, [x8, #16]
	mov w9, #1
	stp x9, xzr, [x8]
	ret
.LBB60_2:
	stp xzr, xzr, [x8]
```
## `checked_div_round_u8_unb_pow2`
```asm
checked_div_round_u8_unb_pow2:
	mov w8, #1
	and w9, w0, #0xff
	and w10, w1, #0xff
	lsl w8, w8, w1
	lsr w9, w9, w1
	and w8, w8, #0xfe
	tst w0, w8, lsr #1
	cinc w8, w9, ne
	cmp w10, #7
	csel w8, w8, w8, hi
	tst w1, #0xf8
	cset w0, eq
	mov w1, w8
```
## `checked_div_round_u16_unb_pow2`
```asm
checked_div_round_u16_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #15
	b.hi .LBB66_2
	mov w8, w0
	mov w0, #1
	lsl w9, w0, w1
	and w10, w8, #0xffff
	lsr w10, w10, w1
	and w9, w9, #0xfffe
	tst w8, w9, lsr #1
	cinc w1, w10, ne
	ret
.LBB66_2:
	mov w0, wzr
```
## `checked_div_round_u32_unb_pow2`
```asm
checked_div_round_u32_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #31
	b.hi .LBB67_2
	mov w8, w0
	mov w0, #1
	lsl w9, w0, w1
	lsr w10, w8, w1
	tst w8, w9, lsr #1
	cinc w1, w10, ne
	ret
.LBB67_2:
	mov w0, wzr
```
## `checked_div_round_u64_unb_pow2`
```asm
checked_div_round_u64_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #63
	b.hi .LBB68_2
	mov x8, x0
	mov w0, #1
	lsl x9, x0, x1
	lsr x10, x8, x1
	tst x8, x9, lsr #1
	cinc x1, x10, ne
	ret
.LBB68_2:
	mov x0, xzr
```
## `checked_div_round_u128_unb_pow2`
```asm
checked_div_round_u128_unb_pow2:
	tbnz w2, #7, .LBB65_2
	mov w9, #1
	and x11, x2, #0xff
	lsl x12, x1, #1
	lsl x10, x9, x2
	tst x11, #0x40
	mvn w13, w2
	lsr x14, x0, x2
	lsl x12, x12, x13
	lsr x13, x1, x2
	csel x11, xzr, x10, ne
	csel x10, x10, xzr, ne
	extr x11, x10, x11, #1
	and x10, x1, x10, lsr #1
	orr x12, x12, x14
	csel x12, x13, x12, ne
	and x11, x11, x0
	orr x10, x11, x10
	csel x11, xzr, x13, ne
	cmp x10, #0
	cset w10, ne
	adds x10, x12, x10
	cinc x11, x11, hs
	stp x10, x11, [x8, #16]
	stp x9, xzr, [x8]
	ret
.LBB65_2:
	stp xzr, xzr, [x8]
```
## `checked_floor_to_multiple_i8_unb_pow2`
```asm
checked_floor_to_multiple_i8_unb_pow2:
	mov w8, #-1
	tst w1, #0xf8
	lsl w9, w8, w1
	cset w8, eq
	and w1, w9, w0
	mov w0, w8
```
## `checked_floor_to_multiple_i16_unb_pow2`
```asm
checked_floor_to_multiple_i16_unb_pow2:
	mov w8, #-1
	tst w1, #0xf0
	lsl w9, w8, w1
	cset w8, eq
	and w1, w9, w0
	mov w0, w8
```
## `checked_floor_to_multiple_i32_unb_pow2`
```asm
checked_floor_to_multiple_i32_unb_pow2:
	mov w8, #-1
	tst w1, #0xe0
	lsl w9, w8, w1
	cset w8, eq
	and w1, w9, w0
	mov w0, w8
```
## `checked_floor_to_multiple_i64_unb_pow2`
```asm
checked_floor_to_multiple_i64_unb_pow2:
	mov x8, #-1
	tst w1, #0xc0
	lsl x9, x8, x1
	cset w8, eq
	and x1, x9, x0
	mov x0, x8
```
## `checked_floor_to_multiple_i128_unb_pow2`
```asm
checked_floor_to_multiple_i128_unb_pow2:
	tbnz w2, #7, .LBB70_2
	mov x9, #-1
	mvn w10, w2
	mov x11, #9223372036854775807
	lsl x9, x9, x2
	lsr x10, x11, x10
	and x12, x2, #0xff
	tst x12, #0x40
	orr x10, x9, x10
	csel x11, xzr, x9, ne
	csel x9, x9, x10, ne
	and x10, x11, x0
	and x9, x9, x1
	stp x10, x9, [x8, #16]
	mov w9, #1
	stp x9, xzr, [x8]
	ret
.LBB70_2:
	stp xzr, xzr, [x8]
```
## `checked_mod_floor_i8_unb_pow2`
```asm
checked_mod_floor_i8_unb_pow2:
	mov w8, #-1
	tst w1, #0xf8
	lsl w9, w8, w1
	cset w8, eq
	bic w1, w0, w9
	mov w0, w8
```
## `checked_mod_floor_i16_unb_pow2`
```asm
checked_mod_floor_i16_unb_pow2:
	mov w8, #-1
	tst w1, #0xf0
	lsl w9, w8, w1
	cset w8, eq
	bic w1, w0, w9
	mov w0, w8
```
## `checked_mod_floor_i32_unb_pow2`
```asm
checked_mod_floor_i32_unb_pow2:
	mov w8, #-1
	tst w1, #0xe0
	lsl w9, w8, w1
	cset w8, eq
	bic w1, w0, w9
	mov w0, w8
```
## `checked_mod_floor_i64_unb_pow2`
```asm
checked_mod_floor_i64_unb_pow2:
	mov x8, #-1
	tst w1, #0xc0
	lsl x9, x8, x1
	cset w8, eq
	bic x1, x0, x9
	mov x0, x8
```
## `checked_mod_floor_i128_unb_pow2`
```asm
checked_mod_floor_i128_unb_pow2:
	tbnz w2, #7, .LBB75_2
	mov x9, #-1
	mvn w10, w2
	mov x11, #9223372036854775807
	lsl x9, x9, x2
	lsr x10, x11, x10
	and x12, x2, #0xff
	tst x12, #0x40
	orr x10, x9, x10
	csel x11, xzr, x9, ne
	csel x9, x9, x10, ne
	bic x10, x0, x11
	bic x9, x1, x9
	stp x10, x9, [x8, #16]
	mov w9, #1
	stp x9, xzr, [x8]
	ret
.LBB75_2:
	stp xzr, xzr, [x8]
```
## `checked_mul_i8_pow2`
```asm
checked_mul_i8_pow2:
	lsl w8, w0, w1
	and w10, w0, #0xff
	sxtb w9, w8
	asr w9, w9, w1
	mov w1, w8
	cmp w10, w9, uxtb
	cset w0, eq
```
## `checked_mul_i8_unb_pow2`
```asm
checked_mul_i8_unb_pow2:
	lsl w8, w0, w1
	and x10, x1, #0x7
	tst w1, #0xf8
	sxtb w9, w8
	mov w1, w8
	asr w9, w9, w10
	and w10, w0, #0xff
	and w9, w9, #0xff
	ccmp w10, w9, #0, eq
	cset w0, eq
```
## `checked_mul_i16_pow2`
```asm
checked_mul_i16_pow2:
	lsl w8, w0, w1
	and w10, w0, #0xffff
	sxth w9, w8
	asr w9, w9, w1
	mov w1, w8
	cmp w10, w9, uxth
	cset w0, eq
```
## `checked_mul_i16_unb_pow2`
```asm
checked_mul_i16_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #15
	b.hi .LBB83_2
	mov w8, w1
	lsl w1, w0, w1
	sxth w9, w1
	asr w8, w9, w8
	and w9, w0, #0xffff
	cmp w9, w8, uxth
	cset w0, eq
	ret
.LBB83_2:
	mov w0, wzr
```
## `checked_mul_i32_pow2`
```asm
checked_mul_i32_pow2:
	lsl w8, w0, w1
	asr w9, w8, w1
	mov w1, w8
	cmp w0, w9
	cset w0, eq
```
## `checked_mul_i32_unb_pow2`
```asm
checked_mul_i32_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #31
	b.hi .LBB85_2
	mov w8, w1
	lsl w1, w0, w1
	asr w8, w1, w8
	cmp w0, w8
	cset w0, eq
	ret
.LBB85_2:
	mov w0, wzr
```
## `checked_mul_i64_pow2`
```asm
checked_mul_i64_pow2:
	lsl x8, x0, x1
	asr x9, x8, x1
	mov x1, x8
	cmp x0, x9
	cset w0, eq
```
## `checked_mul_i64_unb_pow2`
```asm
checked_mul_i64_unb_pow2:
	lsl x8, x0, x1
	tst w1, #0xc0
	asr x9, x8, x1
	mov x1, x8
	ccmp x0, x9, #0, eq
	cset w0, eq
```
## `checked_mul_i128_pow2`
```asm
checked_mul_i128_pow2:
	lsr x9, x0, #1
	mvn w11, w2
	lsl x10, x1, x2
	lsl x12, x0, x2
	and x13, x2, #0xff
	lsr x9, x9, x11
	tst x13, #0x40
	orr x9, x10, x9
	csel x10, xzr, x12, ne
	csel x9, x12, x9, ne
	lsr x13, x10, x2
	lsl x12, x9, #1
	asr x14, x9, #63
	lsl x11, x12, x11
	asr x12, x9, x2
	orr x11, x11, x13
	csel x13, x14, x12, ne
	csel x11, x12, x11, ne
	cmp x1, x13
	ccmp x0, x11, #0, eq
	mov x11, xzr
	b.ne .LBB80_2
	mov w11, #1
	stp x10, x9, [x8, #16]
.LBB80_2:
	stp x11, xzr, [x8]
```
## `checked_mul_i128_unb_pow2`
```asm
checked_mul_i128_unb_pow2:
	tbnz w2, #7, .LBB81_3
	lsr x9, x0, #1
	mvn w11, w2
	lsl x10, x1, x2
	lsl x12, x0, x2
	and x13, x2, #0xff
	lsr x9, x9, x11
	tst x13, #0x40
	orr x9, x10, x9
	csel x10, xzr, x12, ne
	csel x9, x12, x9, ne
	lsr x13, x10, x2
	lsl x12, x9, #1
	asr x14, x9, #63
	lsl x11, x12, x11
	asr x12, x9, x2
	orr x11, x11, x13
	csel x13, x14, x12, ne
	csel x11, x12, x11, ne
	cmp x1, x13
	ccmp x0, x11, #0, eq
	b.ne .LBB81_3
	stp x10, x9, [x8, #16]
	mov w10, #1
	stp x10, xzr, [x8]
	ret
.LBB81_3:
	stp xzr, xzr, [x8]
```
## `checked_mul_u8_pow2`
```asm
checked_mul_u8_pow2:
	lsl w8, w0, w1
	and w9, w8, #0xff
	lsr w9, w9, w1
	mov w1, w8
	cmp w9, w0, uxtb
	cset w0, eq
```
## `checked_mul_u8_unb_pow2`
```asm
checked_mul_u8_unb_pow2:
	lsl w8, w0, w1
	and x10, x1, #0x7
	tst w1, #0xf8
	and w9, w8, #0xff
	mov w1, w8
	lsr w9, w9, w10
	and w10, w0, #0xff
	ccmp w9, w10, #0, eq
	cset w0, eq
```
## `checked_mul_u16_pow2`
```asm
checked_mul_u16_pow2:
	lsl w8, w0, w1
	and w9, w8, #0xffff
	lsr w9, w9, w1
	mov w1, w8
	cmp w9, w0, uxth
	cset w0, eq
```
## `checked_mul_u16_unb_pow2`
```asm
checked_mul_u16_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #15
	b.hi .LBB93_2
	mov w8, w1
	lsl w1, w0, w1
	and w9, w1, #0xffff
	lsr w8, w9, w8
	cmp w8, w0, uxth
	cset w0, eq
	ret
.LBB93_2:
	mov w0, wzr
```
## `checked_mul_u32_pow2`
```asm
checked_mul_u32_pow2:
	lsl w8, w0, w1
	lsr w9, w8, w1
	mov w1, w8
	cmp w0, w9
	cset w0, eq
```
## `checked_mul_u32_unb_pow2`
```asm
checked_mul_u32_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #31
	b.hi .LBB95_2
	mov w8, w1
	lsl w1, w0, w1
	lsr w8, w1, w8
	cmp w0, w8
	cset w0, eq
	ret
.LBB95_2:
	mov w0, wzr
```
## `checked_mul_u64_pow2`
```asm
checked_mul_u64_pow2:
	lsl x8, x0, x1
	lsr x9, x8, x1
	mov x1, x8
	cmp x0, x9
	cset w0, eq
```
## `checked_mul_u64_unb_pow2`
```asm
checked_mul_u64_unb_pow2:
	lsl x8, x0, x1
	tst w1, #0xc0
	lsr x9, x8, x1
	mov x1, x8
	ccmp x0, x9, #0, eq
	cset w0, eq
```
## `checked_mul_u128_pow2`
```asm
checked_mul_u128_pow2:
	lsr x9, x0, #1
	mvn w11, w2
	lsl x10, x1, x2
	lsl x12, x0, x2
	and x13, x2, #0xff
	lsr x9, x9, x11
	tst x13, #0x40
	orr x9, x10, x9
	csel x10, xzr, x12, ne
	csel x9, x12, x9, ne
	lsr x12, x10, x2
	lsl x13, x9, #1
	lsl x11, x13, x11
	lsr x13, x9, x2
	orr x11, x11, x12
	csel x12, xzr, x13, ne
	csel x11, x13, x11, ne
	cmp x1, x12
	ccmp x0, x11, #0, eq
	mov x11, xzr
	b.ne .LBB90_2
	mov w11, #1
	stp x10, x9, [x8, #16]
.LBB90_2:
	stp x11, xzr, [x8]
```
## `checked_mul_u128_unb_pow2`
```asm
checked_mul_u128_unb_pow2:
	tbnz w2, #7, .LBB91_3
	lsr x9, x0, #1
	mvn w11, w2
	lsl x10, x1, x2
	lsl x12, x0, x2
	and x13, x2, #0xff
	lsr x9, x9, x11
	tst x13, #0x40
	orr x9, x10, x9
	csel x10, xzr, x12, ne
	csel x9, x12, x9, ne
	lsr x12, x10, x2
	lsl x13, x9, #1
	lsl x11, x13, x11
	lsr x13, x9, x2
	orr x11, x11, x12
	csel x12, xzr, x13, ne
	csel x11, x13, x11, ne
	cmp x1, x12
	ccmp x0, x11, #0, eq
	b.ne .LBB91_3
	stp x10, x9, [x8, #16]
	mov w10, #1
	stp x10, xzr, [x8]
	ret
.LBB91_3:
	stp xzr, xzr, [x8]
```
## `div_ceil_i8_pow2`
```asm
div_ceil_i8_pow2:
	mov w8, #-1
	sxtb w9, w0
	lsl w8, w8, w1
	asr w9, w9, w1
	bic w8, w0, w8
	tst w8, #0xff
	cinc w0, w9, ne
```
## `div_ceil_i8_unb_pow2`
```asm
div_ceil_i8_unb_pow2:
	mov w8, #-1
	sxtb w9, w0
	and x10, x1, #0x7
	lsl w8, w8, w10
	asr w9, w9, w10
	bics wzr, w0, w8
	cinc w0, w9, ne
```
## `div_ceil_i16_pow2`
```asm
div_ceil_i16_pow2:
	mov w8, #-1
	sxth w9, w0
	lsl w8, w8, w1
	asr w9, w9, w1
	bic w8, w0, w8
	tst w8, #0xffff
	cinc w0, w9, ne
```
## `div_ceil_i16_unb_pow2`
```asm
div_ceil_i16_unb_pow2:
	mov w8, #-1
	sxth w9, w0
	and x10, x1, #0xf
	lsl w8, w8, w10
	asr w9, w9, w10
	bics wzr, w0, w8
	cinc w0, w9, ne
```
## `div_ceil_i32_pow2`
```asm
div_ceil_i32_pow2:
	mov w8, #-1
	asr w9, w0, w1
	lsl w8, w8, w1
	bics wzr, w0, w8
	cinc w0, w9, ne
```
## `div_ceil_i32_unb_pow2`
```asm
div_ceil_i32_unb_pow2:
	mov w8, #-1
	asr w9, w0, w1
	lsl w8, w8, w1
	bics wzr, w0, w8
	cinc w0, w9, ne
```
## `div_ceil_i64_pow2`
```asm
div_ceil_i64_pow2:
	mov x8, #-1
	asr x9, x0, x1
	lsl x8, x8, x1
	bics xzr, x0, x8
	cinc x0, x9, ne
```
## `div_ceil_i64_unb_pow2`
```asm
div_ceil_i64_unb_pow2:
	mov x8, #-1
	asr x9, x0, x1
	lsl x8, x8, x1
	bics xzr, x0, x8
	cinc x0, x9, ne
```
## `div_ceil_i128_pow2`
```asm
div_ceil_i128_pow2:
	mov x8, #-1
	mov x9, #9223372036854775807
	mvn w10, w2
	lsl x8, x8, x2
	lsr x9, x9, x10
	and x11, x2, #0xff
	lsl x12, x1, #1
	tst x11, #0x40
	lsr x13, x0, x2
	orr x9, x8, x9
	csel x11, xzr, x8, ne
	csel x8, x8, x9, ne
	lsl x10, x12, x10
	asr x9, x1, x2
	asr x12, x1, #63
	bic x11, x0, x11
	bic x8, x1, x8
	orr x10, x10, x13
	orr x8, x11, x8
	csel x11, x12, x9, ne
	csel x9, x9, x10, ne
	cmp x8, #0
	cset w8, ne
	adds x0, x9, x8
	cinc x1, x11, hs
```
## `div_ceil_i128_unb_pow2`
```asm
div_ceil_i128_unb_pow2:
	mov x8, #-1
	mov x9, #9223372036854775807
	mvn w10, w2
	lsl x8, x8, x2
	lsr x9, x9, x10
	and x11, x2, #0x7f
	lsl x12, x1, #1
	tst x11, #0x40
	lsr x13, x0, x2
	orr x9, x8, x9
	csel x11, xzr, x8, ne
	csel x8, x8, x9, ne
	lsl x10, x12, x10
	asr x9, x1, x2
	asr x12, x1, #63
	bic x11, x0, x11
	bic x8, x1, x8
	orr x10, x10, x13
	orr x8, x11, x8
	csel x11, x12, x9, ne
	csel x9, x9, x10, ne
	cmp x8, #0
	cset w8, ne
	adds x0, x9, x8
	cinc x1, x11, hs
```
## `div_ceil_u8_pow2`
```asm
div_ceil_u8_pow2:
	mov w8, #-1
	and w9, w0, #0xff
	lsl w8, w8, w1
	lsr w10, w9, w1
	bics wzr, w9, w8
	cinc w0, w10, ne
```
## `div_ceil_u8_unb_pow2`
```asm
div_ceil_u8_unb_pow2:
	mov w8, #-1
	and x9, x1, #0x7
	and w10, w0, #0xff
	lsl w8, w8, w9
	lsr w9, w10, w9
	bics wzr, w0, w8
	cinc w0, w9, ne
```
## `div_ceil_u16_pow2`
```asm
div_ceil_u16_pow2:
	mov w8, #-1
	and w9, w0, #0xffff
	lsl w8, w8, w1
	lsr w10, w9, w1
	bics wzr, w9, w8
	cinc w0, w10, ne
```
## `div_ceil_u16_unb_pow2`
```asm
div_ceil_u16_unb_pow2:
	mov w8, #-1
	and x9, x1, #0xf
	and w10, w0, #0xffff
	lsl w8, w8, w9
	lsr w9, w10, w9
	bics wzr, w0, w8
	cinc w0, w9, ne
```
## `div_ceil_u32_pow2`
```asm
div_ceil_u32_pow2:
	mov w8, #-1
	lsr w9, w0, w1
	lsl w8, w8, w1
	bics wzr, w0, w8
	cinc w0, w9, ne
```
## `div_ceil_u32_unb_pow2`
```asm
div_ceil_u32_unb_pow2:
	mov w8, #-1
	lsr w9, w0, w1
	lsl w8, w8, w1
	bics wzr, w0, w8
	cinc w0, w9, ne
```
## `div_ceil_u64_pow2`
```asm
div_ceil_u64_pow2:
	mov x8, #-1
	lsr x9, x0, x1
	lsl x8, x8, x1
	bics xzr, x0, x8
	cinc x0, x9, ne
```
## `div_ceil_u64_unb_pow2`
```asm
div_ceil_u64_unb_pow2:
	mov x8, #-1
	lsr x9, x0, x1
	lsl x8, x8, x1
	bics xzr, x0, x8
	cinc x0, x9, ne
```
## `div_ceil_u128_pow2`
```asm
div_ceil_u128_pow2:
	mov x8, #-1
	mov x9, #9223372036854775807
	mvn w10, w2
	lsl x8, x8, x2
	lsr x9, x9, x10
	and x11, x2, #0xff
	lsl x12, x1, #1
	tst x11, #0x40
	lsr x11, x0, x2
	orr x9, x8, x9
	csel x9, x8, x9, ne
	csel x8, xzr, x8, ne
	lsl x10, x12, x10
	lsr x12, x1, x2
	bic x8, x0, x8
	bic x9, x1, x9
	orr x10, x10, x11
	orr x8, x8, x9
	csel x9, xzr, x12, ne
	csel x10, x12, x10, ne
	cmp x8, #0
	cset w8, ne
	adds x0, x10, x8
	cinc x1, x9, hs
```
## `div_ceil_u128_unb_pow2`
```asm
div_ceil_u128_unb_pow2:
	mov x8, #-1
	mov x9, #9223372036854775807
	mvn w10, w2
	lsl x8, x8, x2
	lsr x9, x9, x10
	and x11, x2, #0x7f
	lsl x12, x1, #1
	tst x11, #0x40
	lsr x11, x0, x2
	orr x9, x8, x9
	csel x9, x8, x9, ne
	csel x8, xzr, x8, ne
	lsl x10, x12, x10
	lsr x12, x1, x2
	bic x8, x0, x8
	bic x9, x1, x9
	orr x10, x10, x11
	orr x8, x8, x9
	csel x9, xzr, x12, ne
	csel x10, x12, x10, ne
	cmp x8, #0
	cset w8, ne
	adds x0, x10, x8
	cinc x1, x9, hs
```
## `div_floor_i8_pow2`
```asm
div_floor_i8_pow2:
	sxtb w8, w0
	asr w0, w8, w1
```
## `div_floor_i8_unb_pow2`
```asm
div_floor_i8_unb_pow2:
	sxtb w8, w0
	and x9, x1, #0x7
	asr w0, w8, w9
```
## `div_floor_i16_pow2`
```asm
div_floor_i16_pow2:
	sxth w8, w0
	asr w0, w8, w1
```
## `div_floor_i16_unb_pow2`
```asm
div_floor_i16_unb_pow2:
	sxth w8, w0
	and x9, x1, #0xf
	asr w0, w8, w9
```
## `div_floor_i32_pow2`
```asm
div_floor_i32_pow2:
	asr w0, w0, w1
```
## `div_floor_i32_unb_pow2`
```asm
div_floor_i32_unb_pow2:
	asr w0, w0, w1
```
## `div_floor_i64_pow2`
```asm
div_floor_i64_pow2:
	asr x0, x0, x1
```
## `div_floor_i64_unb_pow2`
```asm
div_floor_i64_unb_pow2:
	asr x0, x0, x1
```
## `div_floor_i128_pow2`
```asm
div_floor_i128_pow2:
	lsl x8, x1, #1
	mvn w9, w2
	lsr x10, x0, x2
	and x11, x2, #0xff
	asr x12, x1, #63
	lsl x8, x8, x9
	asr x9, x1, x2
	tst x11, #0x40
	orr x8, x8, x10
	csel x1, x12, x9, ne
	csel x0, x9, x8, ne
```
## `div_floor_i128_unb_pow2`
```asm
div_floor_i128_unb_pow2:
	lsl x8, x1, #1
	mvn w9, w2
	lsr x10, x0, x2
	and x11, x2, #0x7f
	asr x12, x1, #63
	lsl x8, x8, x9
	asr x9, x1, x2
	tst x11, #0x40
	orr x8, x8, x10
	csel x1, x12, x9, ne
	csel x0, x9, x8, ne
```
## `div_floor_u8_pow2`
```asm
div_floor_u8_pow2:
	and w8, w0, #0xff
	lsr w0, w8, w1
```
## `div_floor_u8_unb_pow2`
```asm
div_floor_u8_unb_pow2:
	and w8, w0, #0xff
	and x9, x1, #0x7
	lsr w0, w8, w9
```
## `div_floor_u16_pow2`
```asm
div_floor_u16_pow2:
	and w8, w0, #0xffff
	lsr w0, w8, w1
```
## `div_floor_u16_unb_pow2`
```asm
div_floor_u16_unb_pow2:
	and w8, w0, #0xffff
	and x9, x1, #0xf
	lsr w0, w8, w9
```
## `div_floor_u32_pow2`
```asm
div_floor_u32_pow2:
	lsr w0, w0, w1
```
## `div_floor_u32_unb_pow2`
```asm
div_floor_u32_unb_pow2:
	lsr w0, w0, w1
```
## `div_floor_u64_pow2`
```asm
div_floor_u64_pow2:
	lsr x0, x0, x1
```
## `div_floor_u64_unb_pow2`
```asm
div_floor_u64_unb_pow2:
	lsr x0, x0, x1
```
## `div_floor_u128_pow2`
```asm
div_floor_u128_pow2:
	lsl x8, x1, #1
	mvn w9, w2
	lsr x10, x0, x2
	and x11, x2, #0xff
	lsl x8, x8, x9
	lsr x9, x1, x2
	tst x11, #0x40
	orr x8, x8, x10
	csel x1, xzr, x9, ne
	csel x0, x9, x8, ne
```
## `div_floor_u128_unb_pow2`
```asm
div_floor_u128_unb_pow2:
	lsl x8, x1, #1
	mvn w9, w2
	lsr x10, x0, x2
	and x11, x2, #0x7f
	lsl x8, x8, x9
	lsr x9, x1, x2
	tst x11, #0x40
	orr x8, x8, x10
	csel x1, xzr, x9, ne
	csel x0, x9, x8, ne
```
## `div_i8_pow2`
```asm
div_i8_pow2:
	mov w8, #-1
	and w9, w0, #0xff
	tst w0, #0x80
	lsl w8, w8, w1
	lsr w9, w9, w1
	mvn w8, w8
	add w8, w0, w8
	sxtb w8, w8
	asr w8, w8, w1
	csel w0, w9, w8, eq
```
## `div_i8_unb_pow2`
```asm
div_i8_unb_pow2:
	mov w8, #-1
	and w9, w1, #0x7
	and w10, w0, #0xff
	lsl w8, w8, w9
	lsr w10, w10, w9
	tst w0, #0x80
	mvn w8, w8
	add w8, w0, w8
	sxtb w8, w8
	asr w8, w8, w9
	csel w0, w10, w8, eq
```
## `div_i16_pow2`
```asm
div_i16_pow2:
	mov w8, #-1
	and w9, w1, #0xff
	and w10, w0, #0xffff
	lsl w8, w8, w9
	lsr w10, w10, w9
	tst w0, #0x8000
	mvn w8, w8
	add w8, w0, w8
	sxth w8, w8
	asr w8, w8, w9
	csel w0, w10, w8, eq
```
## `div_i16_unb_pow2`
```asm
div_i16_unb_pow2:
	mov w8, #-1
	and w9, w1, #0xf
	and w10, w0, #0xffff
	lsl w8, w8, w9
	lsr w10, w10, w9
	tst w0, #0x8000
	mvn w8, w8
	add w8, w0, w8
	sxth w8, w8
	asr w8, w8, w9
	csel w0, w10, w8, eq
```
## `div_i32_pow2`
```asm
div_i32_pow2:
	mov w8, #-1
	and w9, w1, #0xff
	tst w0, #0x80000000
	lsl w8, w8, w9
	lsr w10, w0, w9
	mvn w8, w8
	add w8, w0, w8
	asr w8, w8, w9
	csel w0, w10, w8, eq
```
## `div_i32_unb_pow2`
```asm
div_i32_unb_pow2:
	mov w8, #-1
	and w9, w1, #0x1f
	tst w0, #0x80000000
	lsl w8, w8, w9
	lsr w10, w0, w9
	mvn w8, w8
	add w8, w0, w8
	asr w8, w8, w9
	csel w0, w10, w8, eq
```
## `div_i64_pow2`
```asm
div_i64_pow2:
	mov x8, #-1
	and x9, x1, #0xff
	tst x0, #0x8000000000000000
	lsl x8, x8, x9
	lsr x10, x0, x9
	mvn x8, x8
	add x8, x0, x8
	asr x8, x8, x9
	csel x0, x10, x8, eq
```
## `div_i64_unb_pow2`
```asm
div_i64_unb_pow2:
	mov x8, #-1
	and x9, x1, #0x3f
	tst x0, #0x8000000000000000
	lsl x8, x8, x9
	lsr x10, x0, x9
	mvn x8, x8
	add x8, x0, x8
	asr x8, x8, x9
	csel x0, x10, x8, eq
```
## `div_i128_pow2`
```asm
div_i128_pow2:
	tbnz x1, #63, .LBB140_2
	lsl x8, x1, #1
	mov w9, w2
	mvn w10, w2
	lsr x11, x0, x9
	lsl x8, x8, x10
	lsr x10, x1, x9
	and x9, x9, #0xff
	tst x9, #0x40
	orr x8, x8, x11
	csel x1, xzr, x10, ne
	csel x0, x10, x8, ne
	ret
.LBB140_2:
	mov x8, #-1
	mvn w9, w2
	mov x10, #9223372036854775807
	lsl x8, x8, x2
	and x11, x2, #0xff
	lsr x10, x10, x9
	tst x11, #0x40
	csel x12, xzr, x8, ne
	orr x10, x8, x10
	csel x8, x8, x10, ne
	mvn x10, x12
	mvn x8, x8
	adds x10, x0, x10
	adc x8, x1, x8
	lsr x10, x10, x2
	tst x11, #0x40
	lsl x12, x8, #1
	lsl x9, x12, x9
	asr x12, x8, x2
	asr x8, x8, #63
	orr x9, x9, x10
	csel x1, x8, x12, ne
	csel x0, x12, x9, ne
```
## `div_i128_unb_pow2`
```asm
div_i128_unb_pow2:
	and w8, w2, #0x7f
	tbnz x1, #63, .LBB141_2
	lsl x9, x1, #1
	mvn w10, w8
	lsr x11, x0, x8
	tst x8, #0x40
	lsl x9, x9, x10
	lsr x10, x1, x8
	orr x9, x9, x11
	csel x1, xzr, x10, ne
	csel x0, x10, x9, ne
	ret
.LBB141_2:
	mvn w9, w8
	mov x10, #9223372036854775807
	mov x11, #-1
	lsr x10, x10, x9
	lsl x11, x11, x8
	tst x8, #0x40
	orr x10, x11, x10
	csel x12, xzr, x11, ne
	csel x10, x11, x10, ne
	mvn x11, x12
	mvn x10, x10
	adds x11, x0, x11
	adc x10, x1, x10
	lsr x11, x11, x8
	tst x8, #0x40
	lsl x12, x10, #1
	lsl x9, x12, x9
	asr x12, x10, x8
	asr x10, x10, #63
	orr x8, x9, x11
	csel x1, x10, x12, ne
	csel x0, x12, x8, ne
```
## `div_round_i8_pow2`
```asm
div_round_i8_pow2:
	mov w8, #-1
	and w9, w0, #0xff
	lsl w8, w8, w1
	bic w8, w9, w8
	subs w8, w8, w9, lsr #7
	sxtb w9, w0
	csel w8, wzr, w8, lo
	asr w9, w9, w1
	ubfiz w8, w8, #1, #7
	lsr w8, w8, w1
	and w8, w8, #0x1
	add w0, w8, w9
```
## `div_round_i8_unb_pow2`
```asm
div_round_i8_unb_pow2:
	mov w8, #-1
	and x9, x1, #0x7
	and w10, w0, #0x80
	lsl w8, w8, w9
	bic w8, w0, w8
	subs w8, w8, w10, lsr #7
	sxtb w10, w0
	csel w8, wzr, w8, lo
	lsl w8, w8, #1
	lsr w8, w8, w9
	asr w9, w10, w9
	and w8, w8, #0x1
	add w0, w8, w9
```
## `div_round_i16_pow2`
```asm
div_round_i16_pow2:
	mov w8, #-1
	and w9, w0, #0xffff
	lsl w8, w8, w1
	bic w8, w9, w8
	subs w8, w8, w9, lsr #15
	sxth w9, w0
	csel w8, wzr, w8, lo
	asr w9, w9, w1
	ubfiz w8, w8, #1, #15
	lsr w8, w8, w1
	and w8, w8, #0x1
	add w0, w8, w9
```
## `div_round_i16_unb_pow2`
```asm
div_round_i16_unb_pow2:
	mov w8, #-1
	and x9, x1, #0xf
	and w10, w0, #0x8000
	lsl w8, w8, w9
	bic w8, w0, w8
	subs w8, w8, w10, lsr #15
	sxth w10, w0
	csel w8, wzr, w8, lo
	lsl w8, w8, #1
	lsr w8, w8, w9
	asr w9, w10, w9
	and w8, w8, #0x1
	add w0, w8, w9
```
## `div_round_i32_pow2`
```asm
div_round_i32_pow2:
	mov w8, #-1
	asr w9, w0, w1
	lsl w8, w8, w1
	bic w8, w0, w8
	subs w8, w8, w0, lsr #31
	csel w8, wzr, w8, lo
	lsl w8, w8, #1
	lsr w8, w8, w1
	and w8, w8, #0x1
	add w0, w8, w9
```
## `div_round_i32_unb_pow2`
```asm
div_round_i32_unb_pow2:
	mov w8, #-1
	asr w9, w0, w1
	lsl w8, w8, w1
	bic w8, w0, w8
	subs w8, w8, w0, lsr #31
	csel w8, wzr, w8, lo
	lsl w8, w8, #1
	lsr w8, w8, w1
	and w8, w8, #0x1
	add w0, w8, w9
```
## `div_round_i64_pow2`
```asm
div_round_i64_pow2:
	mov x8, #-1
	asr x9, x0, x1
	lsl x8, x8, x1
	bic x8, x0, x8
	subs x8, x8, x0, lsr #63
	csel x8, xzr, x8, lo
	lsl x8, x8, #1
	lsr x8, x8, x1
	and x8, x8, #0x1
	add x0, x8, x9
```
## `div_round_i64_unb_pow2`
```asm
div_round_i64_unb_pow2:
	mov x8, #-1
	asr x9, x0, x1
	lsl x8, x8, x1
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
div_round_i128_pow2:
	mov x8, #-1
	mvn w9, w2
	mov x10, #9223372036854775807
	lsl x8, x8, x2
	and x11, x2, #0xff
	lsr x10, x10, x9
	tst x11, #0x40
	lsl x13, x1, #1
	csel x12, xzr, x8, ne
	orr x10, x8, x10
	csel x8, x8, x10, ne
	bic x10, x0, x12
	bic x8, x1, x8
	subs x10, x10, x1, lsr #63
	sbcs x8, x8, xzr
	csel x8, xzr, x8, lo
	csel x10, xzr, x10, lo
	tst x11, #0x40
	extr x8, x8, x10, #63
	lsl x10, x10, #1
	asr x11, x1, x2
	lsl x12, x8, #1
	lsr x10, x10, x2
	lsr x8, x8, x2
	lsl x12, x12, x9
	lsl x9, x13, x9
	lsr x13, x0, x2
	orr x10, x12, x10
	asr x12, x1, #63
	orr x9, x9, x13
	csel x8, x8, x10, ne
	csel x9, x11, x9, ne
	and x8, x8, #0x1
	csel x10, x12, x11, ne
	adds x0, x8, x9
	cinc x1, x10, hs
```
## `div_round_i128_unb_pow2`
```asm
div_round_i128_unb_pow2:
	mov x8, #-1
	mvn w9, w2
	mov x10, #9223372036854775807
	lsl x8, x8, x2
	and x11, x2, #0x7f
	lsr x10, x10, x9
	tst x11, #0x40
	lsl x13, x1, #1
	csel x12, xzr, x8, ne
	orr x10, x8, x10
	csel x8, x8, x10, ne
	bic x10, x0, x12
	bic x8, x1, x8
	subs x10, x10, x1, lsr #63
	sbcs x8, x8, xzr
	csel x8, xzr, x8, lo
	csel x10, xzr, x10, lo
	tst x11, #0x40
	extr x8, x8, x10, #63
	lsl x10, x10, #1
	asr x11, x1, x2
	lsl x12, x8, #1
	lsr x10, x10, x2
	lsr x8, x8, x2
	lsl x12, x12, x9
	lsl x9, x13, x9
	lsr x13, x0, x2
	orr x10, x12, x10
	asr x12, x1, #63
	orr x9, x9, x13
	csel x8, x8, x10, ne
	csel x9, x11, x9, ne
	and x8, x8, #0x1
	csel x10, x12, x11, ne
	adds x0, x8, x9
	cinc x1, x10, hs
```
## `div_round_u8_pow2`
```asm
div_round_u8_pow2:
	mov w8, #1
	and w9, w0, #0xff
	lsl w8, w8, w1
	lsr w9, w9, w1
	and w8, w8, #0xfe
	tst w0, w8, lsr #1
	cinc w0, w9, ne
```
## `div_round_u8_unb_pow2`
```asm
div_round_u8_unb_pow2:
	mov w8, #1
	and x9, x1, #0x7
	and w10, w0, #0xff
	lsl w8, w8, w9
	lsr w9, w10, w9
	tst w0, w8, lsr #1
	cinc w0, w9, ne
```
## `div_round_u16_pow2`
```asm
div_round_u16_pow2:
	mov w8, #1
	and w9, w0, #0xffff
	lsl w8, w8, w1
	lsr w9, w9, w1
	and w8, w8, #0xfffe
	tst w0, w8, lsr #1
	cinc w0, w9, ne
```
## `div_round_u16_unb_pow2`
```asm
div_round_u16_unb_pow2:
	mov w8, #1
	and x9, x1, #0xf
	and w10, w0, #0xffff
	lsl w8, w8, w9
	lsr w9, w10, w9
	tst w0, w8, lsr #1
	cinc w0, w9, ne
```
## `div_round_u32_pow2`
```asm
div_round_u32_pow2:
	mov w8, #1
	lsr w9, w0, w1
	lsl w8, w8, w1
	tst w0, w8, lsr #1
	cinc w0, w9, ne
```
## `div_round_u32_unb_pow2`
```asm
div_round_u32_unb_pow2:
	mov w8, #1
	lsr w9, w0, w1
	lsl w8, w8, w1
	tst w0, w8, lsr #1
	cinc w0, w9, ne
```
## `div_round_u64_pow2`
```asm
div_round_u64_pow2:
	mov w8, #1
	lsr x9, x0, x1
	lsl x8, x8, x1
	tst x0, x8, lsr #1
	cinc x0, x9, ne
```
## `div_round_u64_unb_pow2`
```asm
div_round_u64_unb_pow2:
	mov w8, #1
	lsr x9, x0, x1
	lsl x8, x8, x1
	tst x0, x8, lsr #1
	cinc x0, x9, ne
```
## `div_round_u128_pow2`
```asm
div_round_u128_pow2:
	mov w8, #1
	and x9, x2, #0xff
	lsl x10, x1, #1
	lsl x8, x8, x2
	tst x9, #0x40
	mvn w11, w2
	lsr x12, x0, x2
	lsl x10, x10, x11
	lsr x11, x1, x2
	csel x9, xzr, x8, ne
	csel x8, x8, xzr, ne
	extr x9, x8, x9, #1
	and x8, x1, x8, lsr #1
	orr x10, x10, x12
	csel x10, x11, x10, ne
	and x9, x9, x0
	orr x8, x9, x8
	csel x9, xzr, x11, ne
	cmp x8, #0
	cset w8, ne
	adds x0, x10, x8
	cinc x1, x9, hs
```
## `div_round_u128_unb_pow2`
```asm
div_round_u128_unb_pow2:
	mov w8, #1
	and x9, x2, #0x7f
	lsl x10, x1, #1
	lsl x8, x8, x2
	tst x9, #0x40
	mvn w11, w2
	lsr x12, x0, x2
	lsl x10, x10, x11
	lsr x11, x1, x2
	csel x9, xzr, x8, ne
	csel x8, x8, xzr, ne
	extr x9, x8, x9, #1
	and x8, x1, x8, lsr #1
	orr x10, x10, x12
	csel x10, x11, x10, ne
	and x9, x9, x0
	orr x8, x9, x8
	csel x9, xzr, x11, ne
	cmp x8, #0
	cset w8, ne
	adds x0, x10, x8
	cinc x1, x9, hs
```
## `floor_to_multiple_i8_pow2`
```asm
floor_to_multiple_i8_pow2:
	and w8, w0, #0xff
	lsr w8, w8, w1
	lsl w0, w8, w1
```
## `floor_to_multiple_i8_unb_pow2`
```asm
floor_to_multiple_i8_unb_pow2:
	and w8, w0, #0xff
	and x9, x1, #0x7
	lsr w8, w8, w9
	lsl w0, w8, w9
```
## `floor_to_multiple_i16_pow2`
```asm
floor_to_multiple_i16_pow2:
	and w8, w0, #0xffff
	lsr w8, w8, w1
	lsl w0, w8, w1
```
## `floor_to_multiple_i16_unb_pow2`
```asm
floor_to_multiple_i16_unb_pow2:
	and w8, w0, #0xffff
	and x9, x1, #0xf
	lsr w8, w8, w9
	lsl w0, w8, w9
```
## `floor_to_multiple_i32_pow2`
```asm
floor_to_multiple_i32_pow2:
	lsr w8, w0, w1
	lsl w0, w8, w1
```
## `floor_to_multiple_i32_unb_pow2`
```asm
floor_to_multiple_i32_unb_pow2:
	lsr w8, w0, w1
	lsl w0, w8, w1
```
## `floor_to_multiple_i64_pow2`
```asm
floor_to_multiple_i64_pow2:
	lsr x8, x0, x1
	lsl x0, x8, x1
```
## `floor_to_multiple_i64_unb_pow2`
```asm
floor_to_multiple_i64_unb_pow2:
	lsr x8, x0, x1
	lsl x0, x8, x1
```
## `floor_to_multiple_i128_pow2`
```asm
floor_to_multiple_i128_pow2:
	mov x8, #-1
	mvn w9, w2
	mov x10, #9223372036854775807
	lsl x8, x8, x2
	lsr x9, x10, x9
	and x10, x2, #0xff
	tst x10, #0x40
	orr x9, x8, x9
	csel x9, x8, x9, ne
	csel x8, xzr, x8, ne
	and x0, x8, x0
	and x1, x9, x1
```
## `floor_to_multiple_i128_unb_pow2`
```asm
floor_to_multiple_i128_unb_pow2:
	mov x8, #-1
	mvn w9, w2
	mov x10, #9223372036854775807
	lsl x8, x8, x2
	lsr x9, x10, x9
	and x10, x2, #0x7f
	tst x10, #0x40
	orr x9, x8, x9
	csel x9, x8, x9, ne
	csel x8, xzr, x8, ne
	and x0, x8, x0
	and x1, x9, x1
```
## `is_multiple_of_i8_pow2`
```asm
is_multiple_of_i8_pow2:
	orr w8, w0, #0x100
	rbit w8, w8
	clz w8, w8
	cmp w8, w1, uxtb
	cset w0, hs
```
## `is_multiple_of_i16_pow2`
```asm
is_multiple_of_i16_pow2:
	orr w8, w0, #0x10000
	rbit w8, w8
	clz w8, w8
	cmp w8, w1, uxtb
	cset w0, hs
```
## `is_multiple_of_i32_pow2`
```asm
is_multiple_of_i32_pow2:
	rbit w8, w0
	clz w8, w8
	cmp w8, w1, uxtb
	cset w0, hs
```
## `is_multiple_of_i64_pow2`
```asm
is_multiple_of_i64_pow2:
	rbit x8, x0
	clz x8, x8
	cmp x8, w1, uxtb
	cset w0, hs
```
## `is_multiple_of_i128_pow2`
```asm
is_multiple_of_i128_pow2:
	rbit x8, x1
	rbit x9, x0
	cmp x0, #0
	clz x8, x8
	clz x9, x9
	add w8, w8, #64
	csel w8, w9, w8, ne
	cmp w8, w2, uxtb
	cset w0, hs
```
## `mod_floor_i8_pow2`
```asm
mod_floor_i8_pow2:
	mov w8, #-1
	lsl w8, w8, w1
	bic w0, w0, w8
```
## `mod_floor_i8_unb_pow2`
```asm
mod_floor_i8_unb_pow2:
	mov w8, #-1
	and x9, x1, #0x7
	lsl w8, w8, w9
	bic w0, w0, w8
```
## `mod_floor_i16_pow2`
```asm
mod_floor_i16_pow2:
	mov w8, #-1
	lsl w8, w8, w1
	bic w0, w0, w8
```
## `mod_floor_i16_unb_pow2`
```asm
mod_floor_i16_unb_pow2:
	mov w8, #-1
	and x9, x1, #0xf
	lsl w8, w8, w9
	bic w0, w0, w8
```
## `mod_floor_i32_pow2`
```asm
mod_floor_i32_pow2:
	mov w8, #-1
	lsl w8, w8, w1
	bic w0, w0, w8
```
## `mod_floor_i32_unb_pow2`
```asm
mod_floor_i32_unb_pow2:
	mov w8, #-1
	lsl w8, w8, w1
	bic w0, w0, w8
```
## `mod_floor_i64_pow2`
```asm
mod_floor_i64_pow2:
	mov x8, #-1
	lsl x8, x8, x1
	bic x0, x0, x8
```
## `mod_floor_i64_unb_pow2`
```asm
mod_floor_i64_unb_pow2:
	mov x8, #-1
	lsl x8, x8, x1
	bic x0, x0, x8
```
## `mod_floor_i128_pow2`
```asm
mod_floor_i128_pow2:
	mov x8, #-1
	mvn w9, w2
	mov x10, #9223372036854775807
	lsl x8, x8, x2
	lsr x9, x10, x9
	and x10, x2, #0xff
	tst x10, #0x40
	orr x9, x8, x9
	csel x9, x8, x9, ne
	csel x8, xzr, x8, ne
	bic x0, x0, x8
	bic x1, x1, x9
```
## `mod_floor_i128_unb_pow2`
```asm
mod_floor_i128_unb_pow2:
	mov x8, #-1
	mvn w9, w2
	mov x10, #9223372036854775807
	lsl x8, x8, x2
	lsr x9, x10, x9
	and x10, x2, #0x7f
	tst x10, #0x40
	orr x9, x8, x9
	csel x9, x8, x9, ne
	csel x8, xzr, x8, ne
	bic x0, x0, x8
	bic x1, x1, x9
```
## `mul_i8_pow2`
```asm
mul_i8_pow2:
	lsl w0, w0, w1
```
## `mul_i8_unb_pow2`
```asm
mul_i8_unb_pow2:
	and x8, x1, #0x7
	lsl w0, w0, w8
```
## `mul_i16_pow2`
```asm
mul_i16_pow2:
	lsl w0, w0, w1
```
## `mul_i16_unb_pow2`
```asm
mul_i16_unb_pow2:
	and x8, x1, #0xf
	lsl w0, w0, w8
```
## `mul_i32_pow2`
```asm
mul_i32_pow2:
	lsl w0, w0, w1
```
## `mul_i32_unb_pow2`
```asm
mul_i32_unb_pow2:
	lsl w0, w0, w1
```
## `mul_i64_pow2`
```asm
mul_i64_pow2:
	lsl x0, x0, x1
```
## `mul_i64_unb_pow2`
```asm
mul_i64_unb_pow2:
	lsl x0, x0, x1
```
## `mul_i128_pow2`
```asm
mul_i128_pow2:
	lsr x8, x0, #1
	mvn w9, w2
	lsl x10, x1, x2
	and x11, x2, #0xff
	lsr x8, x8, x9
	lsl x9, x0, x2
	tst x11, #0x40
	orr x8, x10, x8
	csel x0, xzr, x9, ne
	csel x1, x9, x8, ne
```
## `mul_i128_unb_pow2`
```asm
mul_i128_unb_pow2:
	lsr x8, x0, #1
	mvn w9, w2
	lsl x10, x1, x2
	and x11, x2, #0x7f
	lsr x8, x8, x9
	lsl x9, x0, x2
	tst x11, #0x40
	orr x8, x10, x8
	csel x0, xzr, x9, ne
	csel x1, x9, x8, ne
```
## `unbounded_ceil_to_multiple_i8_unb_pow2`
```asm
unbounded_ceil_to_multiple_i8_unb_pow2:
	mov w8, #-1
	sxtb w9, w0
	and w11, w1, #0xff
	lsl w8, w8, w1
	mvn w10, w8
	add w10, w9, w10, sxtb
	cmp w10, w10, sxtb
	and w8, w10, w8
	cset w12, eq
	cmp w9, #1
	cset w9, lt
	cmp w11, #8
	csel w1, w8, wzr, lo
	csel w0, w12, w9, lo
```
## `unbounded_ceil_to_multiple_i16_unb_pow2`
```asm
unbounded_ceil_to_multiple_i16_unb_pow2:
	mov w8, #-1
	sxth w9, w0
	and w11, w1, #0xff
	lsl w8, w8, w1
	mvn w10, w8
	add w10, w9, w10, sxth
	cmp w10, w10, sxth
	and w8, w10, w8
	cset w12, eq
	cmp w9, #1
	cset w9, lt
	cmp w11, #16
	csel w1, w8, wzr, lo
	csel w0, w12, w9, lo
```
## `unbounded_ceil_to_multiple_i32_unb_pow2`
```asm
unbounded_ceil_to_multiple_i32_unb_pow2:
	mov w8, #-1
	and w10, w1, #0xff
	lsl w8, w8, w1
	mvn w9, w8
	adds w9, w0, w9
	cset w11, vc
	cmp w0, #1
	and w8, w9, w8
	cset w9, lt
	cmp w10, #32
	csel w1, w8, wzr, lo
	csel w0, w11, w9, lo
```
## `unbounded_ceil_to_multiple_i64_unb_pow2`
```asm
unbounded_ceil_to_multiple_i64_unb_pow2:
	mov x8, #-1
	and w10, w1, #0xff
	lsl x8, x8, x1
	mvn x9, x8
	adds x9, x0, x9
	cset w11, vc
	cmp x0, #1
	and x8, x9, x8
	cset w9, lt
	cmp w10, #64
	csel x1, x8, xzr, lo
	csel w0, w11, w9, lo
```
## `unbounded_ceil_to_multiple_i128_unb_pow2`
```asm
unbounded_ceil_to_multiple_i128_unb_pow2:
	tbnz w2, #7, .LBB205_3
	mov x9, #-1
	mvn w10, w2
	mov x11, #9223372036854775807
	lsl x12, x9, x2
	and x9, x2, #0xff
	lsr x10, x11, x10
	tst x9, #0x40
	csel x9, xzr, x12, ne
	orr x10, x12, x10
	csel x11, x12, x10, ne
	mvn x10, x9
	mvn x12, x11
	adds x10, x0, x10
	adcs x12, x1, x12
	cset w13, vs
	tbnz w13, #0, .LBB205_4
	and x10, x10, x9
	and x9, x12, x11
	stp x10, x9, [x8, #16]
	mov w10, #1
	stp x10, xzr, [x8]
	ret
.LBB205_3:
	cmp x0, #1
	sbcs xzr, x1, xzr
	b.lt .LBB205_5
.LBB205_4:
	stp xzr, xzr, [x8]
	ret
.LBB205_5:
	stp xzr, xzr, [x8, #16]
	mov w10, #1
	stp x10, xzr, [x8]
```
## `unbounded_ceil_to_multiple_u8_unb_pow2`
```asm
unbounded_ceil_to_multiple_u8_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #8
	b.hs .LBB214_2
	mov w8, #-1
	mov w10, #255
	lsl w8, w8, w1
	sub w9, w8, w0
	bic w10, w10, w9
	bic w1, w8, w9
	cmp w10, w0, uxtb
	cset w0, hs
	ret
.LBB214_2:
	tst w0, #0xff
	mov w1, wzr
	cset w0, eq
```
## `unbounded_ceil_to_multiple_u16_unb_pow2`
```asm
unbounded_ceil_to_multiple_u16_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #16
	b.hs .LBB211_2
	mov w8, #-1
	mov w10, #65535
	lsl w8, w8, w1
	sub w9, w8, w0
	bic w10, w10, w9
	bic w1, w8, w9
	cmp w10, w0, uxth
	cset w0, hs
	ret
.LBB211_2:
	tst w0, #0xffff
	mov w1, wzr
	cset w0, eq
```
## `unbounded_ceil_to_multiple_u32_unb_pow2`
```asm
unbounded_ceil_to_multiple_u32_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #32
	b.hs .LBB212_2
	mov w8, #-1
	lsl w8, w8, w1
	mvn w9, w8
	add w9, w0, w9
	cmp w9, w0
	sub w9, w8, w0
	cset w0, hs
	bic w1, w8, w9
	ret
.LBB212_2:
	cmp w0, #0
	mov w1, wzr
	cset w0, eq
```
## `unbounded_ceil_to_multiple_u64_unb_pow2`
```asm
unbounded_ceil_to_multiple_u64_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #64
	b.hs .LBB213_2
	mov x8, #-1
	lsl x9, x8, x1
	mvn x8, x9
	sub x10, x9, x0
	add x8, x0, x8
	bic x1, x9, x10
	cmp x8, x0
	cset w8, hs
	mov w0, w8
	ret
.LBB213_2:
	cmp x0, #0
	mov x1, xzr
	cset w8, eq
	mov w0, w8
```
## `unbounded_ceil_to_multiple_u128_unb_pow2`
```asm
unbounded_ceil_to_multiple_u128_unb_pow2:
	tbnz w2, #7, .LBB210_5
	mov x9, #-1
	mvn w10, w2
	mov x11, #9223372036854775807
	lsl x12, x9, x2
	and x9, x2, #0xff
	lsr x10, x11, x10
	tst x9, #0x40
	csel x9, xzr, x12, ne
	orr x10, x12, x10
	csel x11, x12, x10, ne
	mvn x10, x9
	mvn x12, x11
	adds x10, x0, x10
	adcs x12, x1, x12
	cset w13, hs
	tbnz w13, #0, .LBB210_6
	and x10, x10, x9
	and x9, x12, x11
.LBB210_3:
	stp x10, x9, [x8, #16]
	mov w10, #1
.LBB210_4:
	stp x10, xzr, [x8]
	ret
.LBB210_5:
	mov x10, xzr
	orr x9, x0, x1
	cbnz x9, .LBB210_4
	b .LBB210_3
.LBB210_6:
	stp xzr, xzr, [x8]
```
## `unbounded_div_ceil_i8_unb_pow2`
```asm
unbounded_div_ceil_i8_unb_pow2:
	mov w8, #-1
	sxtb w9, w0
	lsl w8, w8, w1
	asr w10, w9, w1
	bic w8, w0, w8
	tst w8, #0xff
	and w8, w1, #0xff
	cinc w10, w10, ne
	cmp w9, #0
	cset w9, gt
	cmp w8, #8
	csel w0, w10, w9, lo
```
## `unbounded_div_ceil_i16_unb_pow2`
```asm
unbounded_div_ceil_i16_unb_pow2:
	mov w8, #-1
	sxth w9, w0
	lsl w8, w8, w1
	asr w10, w9, w1
	bic w8, w0, w8
	tst w8, #0xffff
	and w8, w1, #0xff
	cinc w10, w10, ne
	cmp w9, #0
	cset w9, gt
	cmp w8, #16
	csel w0, w10, w9, lo
```
## `unbounded_div_ceil_i32_unb_pow2`
```asm
unbounded_div_ceil_i32_unb_pow2:
	mov w8, #-1
	asr w9, w0, w1
	lsl w8, w8, w1
	bics wzr, w0, w8
	and w8, w1, #0xff
	cinc w9, w9, ne
	cmp w0, #0
	cset w10, gt
	cmp w8, #32
	csel w0, w9, w10, lo
```
## `unbounded_div_ceil_i64_unb_pow2`
```asm
unbounded_div_ceil_i64_unb_pow2:
	mov x8, #-1
	asr x9, x0, x1
	lsl x8, x8, x1
	bics xzr, x0, x8
	and w8, w1, #0xff
	cinc x9, x9, ne
	cmp x0, #0
	cset w10, gt
	cmp w8, #64
	csel x0, x9, x10, lo
```
## `unbounded_div_ceil_i128_unb_pow2`
```asm
unbounded_div_ceil_i128_unb_pow2:
	tbnz w2, #7, .LBB215_2
	mov x8, #-1
	mvn w9, w2
	mov x10, #9223372036854775807
	lsl x8, x8, x2
	lsr x10, x10, x9
	and x11, x2, #0xff
	lsl x12, x1, #1
	tst x11, #0x40
	lsr x13, x0, x2
	orr x10, x8, x10
	csel x11, xzr, x8, ne
	csel x8, x8, x10, ne
	lsl x9, x12, x9
	asr x10, x1, x2
	asr x12, x1, #63
	bic x11, x0, x11
	bic x8, x1, x8
	orr x9, x9, x13
	orr x8, x11, x8
	csel x11, x12, x10, ne
	csel x9, x10, x9, ne
	cmp x8, #0
	cset w8, ne
	adds x0, x9, x8
	cinc x1, x11, hs
	ret
.LBB215_2:
	cmp xzr, x0
	ngcs xzr, x1
	mov x1, xzr
	cset w0, lt
```
## `unbounded_div_ceil_u8_unb_pow2`
```asm
unbounded_div_ceil_u8_unb_pow2:
	mov w8, #-1
	and w9, w0, #0xff
	lsl w8, w8, w1
	lsr w10, w9, w1
	bics wzr, w9, w8
	and w8, w1, #0xff
	cinc w9, w10, ne
	tst w0, #0xff
	cset w10, ne
	cmp w8, #8
	csel w0, w9, w10, lo
```
## `unbounded_div_ceil_u16_unb_pow2`
```asm
unbounded_div_ceil_u16_unb_pow2:
	mov w8, #-1
	and w9, w0, #0xffff
	lsl w8, w8, w1
	lsr w10, w9, w1
	bics wzr, w9, w8
	and w8, w1, #0xff
	cinc w9, w10, ne
	tst w0, #0xffff
	cset w10, ne
	cmp w8, #16
	csel w0, w9, w10, lo
```
## `unbounded_div_ceil_u32_unb_pow2`
```asm
unbounded_div_ceil_u32_unb_pow2:
	mov w8, #-1
	lsr w9, w0, w1
	lsl w8, w8, w1
	bics wzr, w0, w8
	and w8, w1, #0xff
	cinc w9, w9, ne
	cmp w0, #0
	cset w10, ne
	cmp w8, #32
	csel w0, w9, w10, lo
```
## `unbounded_div_ceil_u64_unb_pow2`
```asm
unbounded_div_ceil_u64_unb_pow2:
	mov x8, #-1
	lsr x9, x0, x1
	lsl x8, x8, x1
	bics xzr, x0, x8
	and w8, w1, #0xff
	cinc x9, x9, ne
	cmp x0, #0
	cset w10, ne
	cmp w8, #64
	csel x0, x9, x10, lo
```
## `unbounded_div_ceil_u128_unb_pow2`
```asm
unbounded_div_ceil_u128_unb_pow2:
	tbnz w2, #7, .LBB220_2
	mov x8, #-1
	mvn w9, w2
	mov x10, #9223372036854775807
	lsl x8, x8, x2
	lsr x10, x10, x9
	and x11, x2, #0xff
	lsl x12, x1, #1
	tst x11, #0x40
	lsr x11, x0, x2
	orr x10, x8, x10
	csel x10, x8, x10, ne
	csel x8, xzr, x8, ne
	lsl x9, x12, x9
	lsr x12, x1, x2
	bic x8, x0, x8
	bic x10, x1, x10
	orr x9, x9, x11
	orr x8, x8, x10
	csel x10, xzr, x12, ne
	csel x9, x12, x9, ne
	cmp x8, #0
	cset w8, ne
	adds x0, x9, x8
	cinc x1, x10, hs
	ret
.LBB220_2:
	orr x9, x0, x1
	mov x1, xzr
	cmp x9, #0
	cset w0, ne
```
## `unbounded_div_floor_i8_unb_pow2`
```asm
unbounded_div_floor_i8_unb_pow2:
	and w9, w1, #0xff
	mov w8, #7
	sxtb w10, w0
	cmp w9, #7
	csel w8, w9, w8, lo
	asr w0, w10, w8
```
## `unbounded_div_floor_i16_unb_pow2`
```asm
unbounded_div_floor_i16_unb_pow2:
	and w9, w1, #0xff
	mov w8, #15
	sxth w10, w0
	cmp w9, #15
	csel w8, w9, w8, lo
	asr w0, w10, w8
```
## `unbounded_div_floor_i32_unb_pow2`
```asm
unbounded_div_floor_i32_unb_pow2:
	and w9, w1, #0xff
	mov w8, #31
	cmp w9, #31
	csel w8, w9, w8, lo
	asr w0, w0, w8
```
## `unbounded_div_floor_i64_unb_pow2`
```asm
unbounded_div_floor_i64_unb_pow2:
	and w9, w1, #0xff
	mov w8, #63
	cmp w9, #63
	csel w8, w9, w8, lo
	asr x0, x0, x8
```
## `unbounded_div_floor_i128_unb_pow2`
```asm
unbounded_div_floor_i128_unb_pow2:
	sxtb w8, w2
	lsl x9, x1, #1
	asr x12, x1, #63
	cmn w8, #1
	mov w8, #127
	csel w8, w2, w8, gt
	mvn w10, w8
	lsr x11, x0, x8
	lsl x9, x9, x10
	asr x10, x1, x8
	and x8, x8, #0xff
	tst x8, #0x40
	orr x8, x9, x11
	csel x1, x12, x10, ne
	csel x0, x10, x8, ne
```
## `unbounded_div_floor_u8_unb_pow2`
```asm
unbounded_div_floor_u8_unb_pow2:
	and w8, w0, #0xff
	tst w1, #0xf8
	lsr w8, w8, w1
	csel w0, w8, wzr, eq
```
## `unbounded_div_floor_u16_unb_pow2`
```asm
unbounded_div_floor_u16_unb_pow2:
	and w8, w0, #0xffff
	tst w1, #0xf0
	lsr w8, w8, w1
	csel w0, w8, wzr, eq
```
## `unbounded_div_floor_u32_unb_pow2`
```asm
unbounded_div_floor_u32_unb_pow2:
	lsr w8, w0, w1
	tst w1, #0xe0
	csel w0, w8, wzr, eq
```
## `unbounded_div_floor_u64_unb_pow2`
```asm
unbounded_div_floor_u64_unb_pow2:
	lsr x8, x0, x1
	tst w1, #0xc0
	csel x0, x8, xzr, eq
```
## `unbounded_div_floor_u128_unb_pow2`
```asm
unbounded_div_floor_u128_unb_pow2:
	lsl x8, x1, #1
	mvn w9, w2
	lsr x10, x0, x2
	lsr x11, x1, x2
	and x12, x2, #0xff
	lsl x8, x8, x9
	sxtb w9, w2
	tst x12, #0x40
	orr x8, x8, x10
	csel x10, xzr, x11, ne
	csel x8, x11, x8, ne
	cmn w9, #1
	csel x1, x10, xzr, gt
	csel x0, x8, xzr, gt
```
## `unbounded_div_i8_unb_pow2`
```asm
unbounded_div_i8_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #7
	b.hi .LBB239_3
	tbnz w0, #7, .LBB239_4
	and w8, w0, #0xff
	lsr w0, w8, w1
	ret
.LBB239_3:
	mov w0, wzr
	ret
.LBB239_4:
	mov w8, #-1
	lsl w8, w8, w1
	mvn w8, w8
	add w8, w0, w8
	sxtb w8, w8
	asr w0, w8, w1
```
## `unbounded_div_i16_unb_pow2`
```asm
unbounded_div_i16_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #15
	b.hi .LBB236_3
	tbnz w0, #15, .LBB236_4
	and w9, w0, #0xffff
	lsr w0, w9, w8
	ret
.LBB236_3:
	mov w0, wzr
	ret
.LBB236_4:
	mov w9, #-1
	lsl w9, w9, w8
	mvn w9, w9
	add w9, w0, w9
	sxth w9, w9
	asr w0, w9, w8
```
## `unbounded_div_i32_unb_pow2`
```asm
unbounded_div_i32_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #31
	b.hi .LBB237_3
	tbnz w0, #31, .LBB237_4
	lsr w0, w0, w8
	ret
.LBB237_3:
	mov w0, wzr
	ret
.LBB237_4:
	mov w9, #-1
	lsl w9, w9, w8
	mvn w9, w9
	add w9, w0, w9
	asr w0, w9, w8
```
## `unbounded_div_i64_unb_pow2`
```asm
unbounded_div_i64_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #63
	b.hi .LBB238_3
	and x8, x1, #0xff
	tbnz x0, #63, .LBB238_4
	lsr x0, x0, x8
	ret
.LBB238_3:
	mov x0, xzr
	ret
.LBB238_4:
	mov x9, #-1
	lsl x9, x9, x8
	mvn x9, x9
	add x9, x0, x9
	asr x0, x9, x8
```
## `unbounded_div_i128_unb_pow2`
```asm
unbounded_div_i128_unb_pow2:
	tbnz w2, #7, .LBB235_3
	tbnz x1, #63, .LBB235_4
	lsl x8, x1, #1
	mov w9, w2
	mvn w10, w2
	lsr x11, x0, x9
	lsl x8, x8, x10
	lsr x10, x1, x9
	and x9, x9, #0xff
	tst x9, #0x40
	orr x8, x8, x11
	csel x1, xzr, x10, ne
	csel x0, x10, x8, ne
	ret
.LBB235_3:
	mov x0, xzr
	mov x1, xzr
	ret
.LBB235_4:
	mov x8, #-1
	mvn w9, w2
	mov x10, #9223372036854775807
	lsl x8, x8, x2
	and x11, x2, #0xff
	lsr x10, x10, x9
	tst x11, #0x40
	csel x12, xzr, x8, ne
	orr x10, x8, x10
	csel x8, x8, x10, ne
	mvn x10, x12
	mvn x8, x8
	adds x10, x0, x10
	adc x8, x1, x8
	lsr x10, x10, x2
	tst x11, #0x40
	lsl x12, x8, #1
	lsl x9, x12, x9
	asr x12, x8, x2
	asr x8, x8, #63
	orr x9, x9, x10
	csel x1, x8, x12, ne
	csel x0, x12, x9, ne
```
## `unbounded_div_round_i8_unb_pow2`
```asm
unbounded_div_round_i8_unb_pow2:
	mov w8, #-1
	and w9, w1, #0xff
	and w10, w0, #0xff
	lsl w8, w8, w1
	cmp w9, #8
	csetm w11, eq
	cmp w10, #128
	bic w8, w10, w8
	csel w11, wzr, w11, ne
	subs w8, w8, w10, lsr #7
	sxtb w10, w0
	csel w8, wzr, w8, lo
	asr w10, w10, w1
	cmp w9, #7
	ubfiz w8, w8, #1, #7
	lsr w8, w8, w1
	and w8, w8, #0x1
	add w8, w8, w10
	csel w0, w11, w8, hi
```
## `unbounded_div_round_i16_unb_pow2`
```asm
unbounded_div_round_i16_unb_pow2:
	mov w8, #-1
	and w9, w1, #0xff
	and w10, w0, #0xffff
	lsl w8, w8, w1
	cmp w9, #16
	csetm w11, eq
	cmp w10, #8, lsl #12
	bic w8, w10, w8
	csel w11, wzr, w11, ne
	subs w8, w8, w10, lsr #15
	sxth w10, w0
	csel w8, wzr, w8, lo
	asr w10, w10, w1
	cmp w9, #15
	ubfiz w8, w8, #1, #15
	lsr w8, w8, w1
	and w8, w8, #0x1
	add w8, w8, w10
	csel w0, w11, w8, hi
```
## `unbounded_div_round_i32_unb_pow2`
```asm
unbounded_div_round_i32_unb_pow2:
	mov w8, #-1
	and w9, w1, #0xff
	mov w10, #-2147483648
	lsl w8, w8, w1
	cmp w9, #32
	csetm w11, eq
	cmp w0, w10
	bic w8, w0, w8
	csel w10, wzr, w11, ne
	asr w11, w0, w1
	subs w8, w8, w0, lsr #31
	csel w8, wzr, w8, lo
	cmp w9, #31
	lsl w8, w8, #1
	lsr w8, w8, w1
	and w8, w8, #0x1
	add w8, w8, w11
	csel w0, w10, w8, hi
```
## `unbounded_div_round_i64_unb_pow2`
```asm
unbounded_div_round_i64_unb_pow2:
	mov x8, #-1
	and w9, w1, #0xff
	mov x10, #-9223372036854775808
	lsl x8, x8, x1
	cmp w9, #64
	csetm x11, eq
	cmp x0, x10
	bic x8, x0, x8
	csel x10, xzr, x11, ne
	asr x11, x0, x1
	subs x8, x8, x0, lsr #63
	csel x8, xzr, x8, lo
	cmp w9, #63
	lsl x8, x8, #1
	lsr x8, x8, x1
	and x8, x8, #0x1
	add x8, x8, x11
	csel x0, x10, x8, hi
```
## `unbounded_div_round_i128_unb_pow2`
```asm
unbounded_div_round_i128_unb_pow2:
	tbnz w2, #7, .LBB240_2
	mov x8, #-1
	mvn w9, w2
	mov x10, #9223372036854775807
	lsl x8, x8, x2
	and x11, x2, #0xff
	lsr x10, x10, x9
	tst x11, #0x40
	lsl x13, x1, #1
	csel x12, xzr, x8, ne
	orr x10, x8, x10
	csel x8, x8, x10, ne
	bic x10, x0, x12
	bic x8, x1, x8
	subs x10, x10, x1, lsr #63
	sbcs x8, x8, xzr
	csel x8, xzr, x8, lo
	csel x10, xzr, x10, lo
	tst x11, #0x40
	extr x8, x8, x10, #63
	lsl x10, x10, #1
	asr x11, x1, x2
	lsl x12, x8, #1
	lsr x10, x10, x2
	lsr x8, x8, x2
	lsl x12, x12, x9
	lsl x9, x13, x9
	lsr x13, x0, x2
	orr x10, x12, x10
	asr x12, x1, #63
	orr x9, x9, x13
	csel x8, x8, x10, ne
	csel x9, x11, x9, ne
	and x8, x8, #0x1
	csel x10, x12, x11, ne
	adds x0, x8, x9
	cinc x1, x10, hs
	ret
.LBB240_2:
	eor x8, x1, #0x8000000000000000
	and w9, w2, #0xff
	orr x8, x0, x8
	cmp w9, #128
	csetm x9, eq
	cmp x8, #0
	csel x0, xzr, x9, ne
	csel x1, xzr, x9, ne
```
## `unbounded_div_round_u8_unb_pow2`
```asm
unbounded_div_round_u8_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #7
	b.hi .LBB249_2
	mov w8, #1
	and w9, w0, #0xff
	lsl w8, w8, w1
	lsr w9, w9, w1
	and w8, w8, #0xfe
	tst w0, w8, lsr #1
	cinc w0, w9, ne
	ret
.LBB249_2:
	mov w0, wzr
```
## `unbounded_div_round_u16_unb_pow2`
```asm
unbounded_div_round_u16_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #15
	b.hi .LBB246_2
	mov w8, #1
	and w9, w0, #0xffff
	lsl w8, w8, w1
	lsr w9, w9, w1
	and w8, w8, #0xfffe
	tst w0, w8, lsr #1
	cinc w0, w9, ne
	ret
.LBB246_2:
	mov w0, wzr
```
## `unbounded_div_round_u32_unb_pow2`
```asm
unbounded_div_round_u32_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #31
	b.hi .LBB247_2
	mov w8, #1
	lsr w9, w0, w1
	lsl w8, w8, w1
	tst w0, w8, lsr #1
	cinc w0, w9, ne
	ret
.LBB247_2:
	mov w0, wzr
```
## `unbounded_div_round_u64_unb_pow2`
```asm
unbounded_div_round_u64_unb_pow2:
	and w8, w1, #0xff
	cmp w8, #63
	b.hi .LBB248_2
	mov w8, #1
	lsr x9, x0, x1
	lsl x8, x8, x1
	tst x0, x8, lsr #1
	cinc x0, x9, ne
	ret
.LBB248_2:
	mov x0, xzr
```
## `unbounded_div_round_u128_unb_pow2`
```asm
unbounded_div_round_u128_unb_pow2:
	tbnz w2, #7, .LBB245_2
	mov w8, #1
	and x9, x2, #0xff
	lsl x10, x1, #1
	lsl x8, x8, x2
	tst x9, #0x40
	mvn w11, w2
	lsr x12, x0, x2
	lsl x10, x10, x11
	lsr x11, x1, x2
	csel x9, xzr, x8, ne
	csel x8, x8, xzr, ne
	extr x9, x8, x9, #1
	and x8, x1, x8, lsr #1
	orr x10, x10, x12
	csel x10, x11, x10, ne
	and x9, x9, x0
	orr x8, x9, x8
	csel x9, xzr, x11, ne
	cmp x8, #0
	cset w8, ne
	adds x0, x10, x8
	cinc x1, x9, hs
	ret
.LBB245_2:
	mov x0, xzr
	mov x1, xzr
```
## `unbounded_floor_to_multiple_i8_unb_pow2`
```asm
unbounded_floor_to_multiple_i8_unb_pow2:
	and w8, w0, #0xff
	sxtb w9, w0
	and w10, w1, #0xff
	lsr w8, w8, w1
	cmn w9, #1
	lsl w8, w8, w1
	cset w9, gt
	cmp w10, #8
	csinc w0, w9, wzr, hs
	csel w1, w8, wzr, lo
```
## `unbounded_floor_to_multiple_i16_unb_pow2`
```asm
unbounded_floor_to_multiple_i16_unb_pow2:
	and w8, w0, #0xffff
	ubfx w9, w0, #15, #1
	and w10, w1, #0xff
	lsr w8, w8, w1
	cmp w10, #16
	eor w9, w9, #0x1
	lsl w8, w8, w1
	csinc w0, w9, wzr, hs
	csel w1, w8, wzr, lo
```
## `unbounded_floor_to_multiple_i32_unb_pow2`
```asm
unbounded_floor_to_multiple_i32_unb_pow2:
	lsr w8, w0, w1
	mvn w9, w0
	and w10, w1, #0xff
	lsr w9, w9, #31
	cmp w10, #32
	lsl w8, w8, w1
	csinc w0, w9, wzr, hs
	csel w1, w8, wzr, lo
```
## `unbounded_floor_to_multiple_i64_unb_pow2`
```asm
unbounded_floor_to_multiple_i64_unb_pow2:
	lsr x8, x0, x1
	and w9, w1, #0xff
	cmn x0, #1
	cset w10, gt
	cmp w9, #64
	lsl x8, x8, x1
	csinc w0, w10, wzr, hs
	csel x1, x8, xzr, lo
```
## `unbounded_floor_to_multiple_i128_unb_pow2`
```asm
unbounded_floor_to_multiple_i128_unb_pow2:
	tbnz w2, #7, .LBB250_2
	mov x9, #-1
	mvn w10, w2
	mov x11, #9223372036854775807
	lsl x9, x9, x2
	lsr x10, x11, x10
	and x12, x2, #0xff
	tst x12, #0x40
	orr x10, x9, x10
	csel x11, xzr, x9, ne
	csel x9, x9, x10, ne
	and x10, x11, x0
	and x9, x9, x1
	stp x10, x9, [x8, #16]
	mov w9, #1
	stp x9, xzr, [x8]
	ret
.LBB250_2:
	tbnz x1, #63, .LBB250_4
	mov w9, #1
	stp xzr, xzr, [x8, #16]
	stp x9, xzr, [x8]
	ret
.LBB250_4:
	stp xzr, xzr, [x8]
```
## `unbounded_floor_to_multiple_u8_unb_pow2`
```asm
unbounded_floor_to_multiple_u8_unb_pow2:
	and w8, w0, #0xff
	tst w1, #0xf8
	lsr w8, w8, w1
	lsl w8, w8, w1
	csel w0, w8, wzr, eq
```
## `unbounded_floor_to_multiple_u16_unb_pow2`
```asm
unbounded_floor_to_multiple_u16_unb_pow2:
	and w8, w0, #0xffff
	tst w1, #0xf0
	lsr w8, w8, w1
	lsl w8, w8, w1
	csel w0, w8, wzr, eq
```
## `unbounded_floor_to_multiple_u32_unb_pow2`
```asm
unbounded_floor_to_multiple_u32_unb_pow2:
	lsr w8, w0, w1
	tst w1, #0xe0
	lsl w8, w8, w1
	csel w0, w8, wzr, eq
```
## `unbounded_floor_to_multiple_u64_unb_pow2`
```asm
unbounded_floor_to_multiple_u64_unb_pow2:
	lsr x8, x0, x1
	tst w1, #0xc0
	lsl x8, x8, x1
	csel x0, x8, xzr, eq
```
## `unbounded_floor_to_multiple_u128_unb_pow2`
```asm
unbounded_floor_to_multiple_u128_unb_pow2:
	mov x8, #-1
	mvn w9, w2
	mov x10, #9223372036854775807
	lsl x8, x8, x2
	lsr x9, x10, x9
	and x10, x2, #0xff
	sxtb w11, w2
	tst x10, #0x40
	orr x9, x8, x9
	csel x10, xzr, x8, ne
	csel x8, x8, x9, ne
	and x9, x10, x0
	cmn w11, #1
	and x8, x8, x1
	csel x0, x9, xzr, gt
	csel x1, x8, xzr, gt
```
## `unbounded_is_multiple_of_i8_unb_pow2`
```asm
unbounded_is_multiple_of_i8_unb_pow2:
	orr w8, w0, #0x100
	and w9, w1, #0xff
	tst w0, #0xff
	rbit w8, w8
	clz w8, w8
	ccmp w8, w9, #2, ne
	cset w0, hs
```
## `unbounded_is_multiple_of_i16_unb_pow2`
```asm
unbounded_is_multiple_of_i16_unb_pow2:
	orr w8, w0, #0x10000
	tst w0, #0xffff
	and w9, w1, #0xff
	rbit w8, w8
	clz w8, w8
	ccmp w8, w9, #2, ne
	cset w0, hs
```
## `unbounded_is_multiple_of_i32_unb_pow2`
```asm
unbounded_is_multiple_of_i32_unb_pow2:
	rbit w8, w0
	cmp w0, #0
	and w9, w1, #0xff
	clz w8, w8
	ccmp w8, w9, #2, ne
	cset w0, hs
```
## `unbounded_is_multiple_of_i64_unb_pow2`
```asm
unbounded_is_multiple_of_i64_unb_pow2:
	rbit x8, x0
	cmp x0, #0
	and x9, x1, #0xff
	clz x8, x8
	ccmp x8, x9, #2, ne
	cset w0, hs
```
## `unbounded_is_multiple_of_i128_unb_pow2`
```asm
unbounded_is_multiple_of_i128_unb_pow2:
	rbit x8, x1
	rbit x9, x0
	orr x10, x0, x1
	cmp x0, #0
	clz x8, x8
	clz x9, x9
	add w8, w8, #64
	csel w8, w9, w8, ne
	cmp x10, #0
	and w9, w2, #0xff
	ccmp w8, w9, #2, ne
	cset w0, hs
```
