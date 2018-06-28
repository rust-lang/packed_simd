//! 256-bit wide vector types

use crate::*;

impl_i!([i8; 32]: i8x32, m8x32 | x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15, x16, x17, x18, x19, x20, x21, x22, x23, x24, x25, x26, x27, x28, x29, x30, x31 |
        /// A 256-bit vector with 32 `i8` lanes.
);
impl_u!([u8; 32]: u8x32, m8x32 | x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15, x16, x17, x18, x19, x20, x21, x22, x23, x24, x25, x26, x27, x28, x29, x30, x31 |
        /// A 256-bit vector with 32 `u8` lanes.
);
impl_m!([m8; 32]: m8x32 | i8 | x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15, x16, x17, x18, x19, x20, x21, x22, x23, x24, x25, x26, x27, x28, x29, x30, x31 |
        /// A 256-bit vector mask with 32 `m8` lanes.
);

impl_i!([i16; 16]: i16x16, m16x16 | x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
        /// A 256-bit vector with 16 `i16` lanes.
);
impl_u!([u16; 16]: u16x16, m16x16 | x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
        /// A 256-bit vector with 16 `u16` lanes.
);
impl_m!([m16; 16]: m16x16 | i16 | x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
        /// A 256-bit vector mask with 16 `m16` lanes.
);

impl_i!([i32; 8]: i32x8, m32x8 | x0, x1, x2, x3, x4, x5, x6, x7  |
        /// A 256-bit vector with 8 `i32` lanes.
);
impl_u!([u32; 8]: u32x8, m32x8 | x0, x1, x2, x3, x4, x5, x6, x7 |
        /// A 256-bit vector with 8 `u32` lanes.
);
impl_f!([f32; 8]: f32x8, m32x8 | x0, x1, x2, x3, x4, x5, x6, x7 |
        /// A 256-bit vector with 8 `f32` lanes.
);
impl_m!([m32; 8]: m32x8 | i32 | x0, x1, x2, x3, x4, x5, x6, x7 |
        /// A 256-bit vector mask with 8 `m32` lanes.
);

impl_i!([i64; 4]: i64x4, m64x4 | x0, x1, x2, x3 |
        /// A 256-bit vector with 4 `i64` lanes.
);
impl_u!([u64; 4]: u64x4, m64x4 | x0, x1, x2, x3 |
        /// A 256-bit vector with 4 `u64` lanes.
);
impl_f!([f64; 4]: f64x4, m64x4 | x0, x1, x2, x3 |
        /// A 256-bit vector with 4 `f64` lanes.
);
impl_m!([m64; 4]: m64x4 | i64 | x0, x1, x2, x3 |
        /// A 256-bit vector mask with 4 `m64` lanes.
);

impl_i!([i128; 2]: i128x2, m128x2 | x0, x1 |
        /// A 256-bit vector with 2 `i128` lanes.
);
impl_u!([u128; 2]: u128x2, m128x2 | x0, x1 |
        /// A 256-bit vector with 2 `u128` lanes.
);
impl_m!([m128; 2]: m128x2 | i128 | x0, x1 |
        /// A 256-bit vector mask with 2 `m128` lanes.
);
