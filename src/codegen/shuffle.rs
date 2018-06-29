//! Implementations of the `ShuffleResult` trait for the different numbers of
//! lanes and vector element types.

use sealed::{Shuffle};
/*
        impl Shuffle<[u32; 2]> for i8 {
            type Output = crate::codegen::i8x2;
        }
        impl Shuffle<[u32; 4]> for i8 {
            type Output = crate::codegen::i8x4;
        }
        impl Shuffle<[u32; 8]> for i8 {
            type Output = crate::codegen::i8x8;
        }
*/
impl Shuffle<[u32; 16]> for i8 {
    type Output = crate::codegen::i8x16;
}
impl Shuffle<[u32; 32]> for i8 {
    type Output = crate::codegen::i8x32;
}
/*
        impl Shuffle<[u32; 64]> for i8 {
            type Output = crate::codegen::i8x64;
        }
        impl Shuffle<[u32; 2]> for u8 {
            type Output = crate::codegen::u8x2;
        }
        impl Shuffle<[u32; 4]> for u8 {
            type Output = crate::codegen::u8x4;
        }
        impl Shuffle<[u32; 8]> for u8 {
            type Output = crate::codegen::u8x8;
        }
 */
impl Shuffle<[u32; 16]> for u8 {
    type Output = crate::codegen::u8x16;
}
impl Shuffle<[u32; 32]> for u8 {
    type Output = crate::codegen::u8x32;
}
/*
        impl Shuffle<[u32; 64]> for u8 {
            type Output = crate::codegen::u8x64;
        }

        impl Shuffle<[u32; 2]> for i16 {
            type Output = crate::codegen::i16x2;
        }
        impl Shuffle<[u32; 4]> for i16 {
            type Output = crate::codegen::i16x4;
        }
*/
impl Shuffle<[u32; 8]> for i16 {
    type Output = crate::codegen::i16x8;
}
impl Shuffle<[u32; 16]> for i16 {
    type Output = crate::codegen::i16x16;
}
/*
        impl Shuffle<[u32; 32]> for i16 {
            type Output = crate::codegen::i16x32;
        }
        impl Shuffle<[u32; 2]> for u16 {
            type Output = crate::codegen::u16x2;
        }
        impl Shuffle<[u32; 4]> for u16 {
            type Output = crate::codegen::u16x4;
        }
*/
impl Shuffle<[u32; 8]> for u16 {
    type Output = crate::codegen::u16x8;
}
impl Shuffle<[u32; 16]> for u16 {
    type Output = crate::codegen::u16x16;
}
/*
        impl Shuffle<[u32; 32]> for u16 {
            type Output = crate::codegen::u16x32;
        }
        impl Shuffle<[u32; 2]> for i32 {
            type Output = crate::codegen::i32x2;
        }
*/
impl Shuffle<[u32; 4]> for i32 {
    type Output = crate::codegen::i32x4;
}
impl Shuffle<[u32; 8]> for i32 {
    type Output = crate::codegen::i32x8;
}
/*
        impl Shuffle<[u32; 16]> for i32 {
            type Output = crate::codegen::i32x16;
        }
        impl Shuffle<[u32; 2]> for u32 {
            type Output = crate::codegen::u32x2;
        }
*/
impl Shuffle<[u32; 4]> for u32 {
    type Output = crate::codegen::u32x4;
}
impl Shuffle<[u32; 8]> for u32 {
    type Output = crate::codegen::u32x8;
}
/*
        impl Shuffle<[u32; 16]> for u32 {
            type Output = crate::codegen::u32x16;
        }
        impl Shuffle<[u32; 2]> for f32 {
            type Output = crate::codegen::f32x2;
        }
*/
impl Shuffle<[u32; 4]> for f32 {
    type Output = crate::codegen::f32x4;
}
impl Shuffle<[u32; 8]> for f32 {
    type Output = crate::codegen::f32x8;
}
/*
        impl Shuffle<[u32; 16]> for f32 {
            type Output = crate::codegen::f32x16;
        }
*/
impl Shuffle<[u32; 2]> for i64 {
    type Output = crate::codegen::i64x2;
}
impl Shuffle<[u32; 4]> for i64 {
    type Output = crate::codegen::i64x4;
}
/*
        impl Shuffle<[u32; 8]> for i64 {
            type Output = crate::codegen::i64x8;
        }
*/
impl Shuffle<[u32; 2]> for u64 {
    type Output = crate::codegen::u64x2;
}
impl Shuffle<[u32; 4]> for u64 {
    type Output = crate::codegen::u64x4;
}
/*
        impl Shuffle<[u32; 8]> for u64 {
            type Output = crate::codegen::u64x8;
        }
*/
impl Shuffle<[u32; 2]> for f64 {
    type Output = crate::codegen::f64x2;
}
impl Shuffle<[u32; 4]> for f64 {
    type Output = crate::codegen::f64x4;
}
/*
        impl Shuffle<[u32; 8]> for f64 {
            type Output = crate::codegen::f64x8;
        }
 */

// FIXME: 128-bit types, missing types, tests
