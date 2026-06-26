use std::hint::black_box;
use ipow2::Pow2;

#[unsafe(no_mangle)]
pub fn div_floor_i8_pow2(lhs: i8, rhs: Pow2<u8>) -> i8 {
    ipow2::div_floor(lhs, rhs)
}

#[unsafe(no_mangle)]
pub fn div_floor_i16_pow2(lhs: i16, rhs: Pow2<u8>) -> i16 {
    ipow2::div_floor(lhs, rhs)
}

#[unsafe(no_mangle)]
pub fn div_floor_i32_pow2(lhs: i32, rhs: Pow2<u8>) -> i32 {
    ipow2::div_floor(lhs, rhs)
}

#[unsafe(no_mangle)]
pub fn div_floor_i64_pow2(lhs: i64, rhs: Pow2<u8>) -> i64 {
    ipow2::div_floor(lhs, rhs)
}

#[unsafe(no_mangle)]
pub fn div_floor_i128_pow2(lhs: i128, rhs: Pow2<u8>) -> i128 {
    ipow2::div_floor(lhs, rhs)
}

#[unsafe(no_mangle)]
pub fn div_floor_u8_pow2(lhs: u8, rhs: Pow2<u8>) -> u8 {
    ipow2::div_floor(lhs, rhs)
}

#[unsafe(no_mangle)]
pub fn div_floor_u16_pow2(lhs: u16, rhs: Pow2<u8>) -> u16 {
    ipow2::div_floor(lhs, rhs)
}

#[unsafe(no_mangle)]
pub fn div_floor_u32_pow2(lhs: u32, rhs: Pow2<u8>) -> u32 {
    ipow2::div_floor(lhs, rhs)
}

#[unsafe(no_mangle)]
pub fn div_floor_u64_pow2(lhs: u64, rhs: Pow2<u8>) -> u64 {
    ipow2::div_floor(lhs, rhs)
}

#[unsafe(no_mangle)]
pub fn div_floor_u128_pow2(lhs: u128, rhs: Pow2<u8>) -> u128 {
    ipow2::div_floor(lhs, rhs)
}