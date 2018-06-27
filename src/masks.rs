//! Mask types

/// 8-bit wide mask.
#[derive(Copy, Clone)]
pub struct m8(i8);

/// 16-bit wide mask.
#[derive(Copy, Clone)]
pub struct m16(i16);

/// 32-bit wide mask.
#[derive(Copy, Clone)]
pub struct m32(i32);

/// 64-bit wide mask.
#[derive(Copy, Clone)]
pub struct m64(i64);

/// 128-bit wide mask.
#[derive(Copy, Clone)]
pub struct m128(i128);
