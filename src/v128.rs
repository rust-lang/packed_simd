//! 128-bit wide vector types

use crate::*;

impl_i!([i8; 16]: i8x16, m8x16 | x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
        /// A 128-bit vector with 16 `i8` lanes.
);
impl_u!([u8; 16]: u8x16, m8x16 | x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
        /// A 128-bit vector with 16 `u8` lanes.
);
impl_m!([m8; 16]: m8x16 | i8 | x0, x1, x2, x3, x4, x5, x6, x7, x8, x9, x10, x11, x12, x13, x14, x15 |
        /// A 128-bit vector mask with 16 `m8` lanes.
);

impl_i!([i16; 8]: i16x8, m16x8 | x0, x1, x2, x3, x4, x5, x6, x7 |
        /// A 128-bit vector with 8 `i16` lanes.
);
impl_u!([u16; 8]: u16x8, m16x8 | x0, x1, x2, x3, x4, x5, x6, x7 |
        /// A 128-bit vector with 8 `u16` lanes.
);
impl_m!([m16; 8]: m16x8 | i16 | x0, x1, x2, x3, x4, x5, x6, x7 |
        /// A 128-bit vector mask with 8 `m16` lanes.
);

impl_i!([i32; 4]: i32x4, m32x4 | x0, x1, x2, x3 |
        /// A 128-bit vector with 4 `i32` lanes.
);
impl_u!([u32; 4]: u32x4, m32x4 | x0, x1, x2, x3 |
        /// A 128-bit vector with 4 `u32` lanes.
);
impl_f!([f32; 4]: f32x4, m32x4 | x0, x1, x2, x3 |
        /// A 128-bit vector with 4 `f32` lanes.
);
impl_m!([m32; 4]: m32x4 | i32 | x0, x1, x2, x3 |
        /// A 128-bit vector mask with 4 `m32` lanes.
);

impl_i!([i64; 2]: i64x2, m64x2 | x0, x1 |
        /// A 128-bit vector with 2 `i64` lanes.
);
impl_u!([u64; 2]: u64x2, m64x2 | x0, x1 |
        /// A 128-bit vector with 2 `u64` lanes.
);
impl_f!([f64; 2]: f64x2, m64x2 | x0, x1 |
        /// A 128-bit vector with 2 `f64` lanes.
);
impl_m!([m64; 2]: m64x2 | i64 | x0, x1 |
        /// A 128-bit vector mask with 2 `m64` lanes.
);

impl_i!([i128; 1]: i128x1, m128x1 | x0 |
        /// A 128-bit vector with 1 `i128` lane.
);
impl_u!([u128; 1]: u128x1, m128x1 | x0 |
        /// A 128-bit vector with 1 `u128` lane.
);
impl_m!([m128; 1]: m128x1 | i128 | x0 |
        /// A 128-bit vector mask with 1 `m128` lane.
);
