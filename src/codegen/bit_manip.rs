//! LLVM bit manipulation intrinsics.

use crate::*;

#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "llvm.ctlz.v2i8"]
    fn ctlz_u8x2(x: u8x2, is_zero_undef: bool) -> u8x2;
    #[link_name = "llvm.ctlz.v4i8"]
    fn ctlz_u8x4(x: u8x4, is_zero_undef: bool) -> u8x4;
    #[link_name = "llvm.ctlz.v8i8"]
    fn ctlz_u8x8(x: u8x8, is_zero_undef: bool) -> u8x8;
    #[link_name = "llvm.ctlz.v16i8"]
    fn ctlz_u8x16(x: u8x16, is_zero_undef: bool) -> u8x16;
    #[link_name = "llvm.ctlz.v32i8"]
    fn ctlz_u8x32(x: u8x32, is_zero_undef: bool) -> u8x32;
    #[link_name = "llvm.ctlz.v64i8"]
    fn ctlz_u8x64(x: u8x64, is_zero_undef: bool) -> u8x64;

    #[link_name = "llvm.ctlz.v2i16"]
    fn ctlz_u16x2(x: u16x2, is_zero_undef: bool) -> u16x2;
    #[link_name = "llvm.ctlz.v4i16"]
    fn ctlz_u16x4(x: u16x4, is_zero_undef: bool) -> u16x4;
    #[link_name = "llvm.ctlz.v8i16"]
    fn ctlz_u16x8(x: u16x8, is_zero_undef: bool) -> u16x8;
    #[link_name = "llvm.ctlz.v16i16"]
    fn ctlz_u16x16(x: u16x16, is_zero_undef: bool) -> u16x16;
    #[link_name = "llvm.ctlz.v32i16"]
    fn ctlz_u16x32(x: u16x32, is_zero_undef: bool) -> u16x32;

    #[link_name = "llvm.ctlz.v2i32"]
    fn ctlz_u32x2(x: u32x2, is_zero_undef: bool) -> u32x2;
    #[link_name = "llvm.ctlz.v4i32"]
    fn ctlz_u32x4(x: u32x4, is_zero_undef: bool) -> u32x4;
    #[link_name = "llvm.ctlz.v8i32"]
    fn ctlz_u32x8(x: u32x8, is_zero_undef: bool) -> u32x8;
    #[link_name = "llvm.ctlz.v16i32"]
    fn ctlz_u32x16(x: u32x16, is_zero_undef: bool) -> u32x16;

    #[link_name = "llvm.ctlz.v2i64"]
    fn ctlz_u64x2(x: u64x2, is_zero_undef: bool) -> u64x2;
    #[link_name = "llvm.ctlz.v4i64"]
    fn ctlz_u64x4(x: u64x4, is_zero_undef: bool) -> u64x4;
    #[link_name = "llvm.ctlz.v8i64"]
    fn ctlz_u64x8(x: u64x8, is_zero_undef: bool) -> u64x8;

    #[link_name = "llvm.ctlz.v1i128"]
    fn ctlz_u128x1(x: u128x1, is_zero_undef: bool) -> u128x1;
    #[link_name = "llvm.ctlz.v2i128"]
    fn ctlz_u128x2(x: u128x2, is_zero_undef: bool) -> u128x2;
    #[link_name = "llvm.ctlz.v4i128"]
    fn ctlz_u128x4(x: u128x4, is_zero_undef: bool) -> u128x4;


    #[link_name = "llvm.cttz.v2i8"]
    fn cttz_u8x2(x: u8x2, is_zero_undef: bool) -> u8x2;
    #[link_name = "llvm.cttz.v4i8"]
    fn cttz_u8x4(x: u8x4, is_zero_undef: bool) -> u8x4;
    #[link_name = "llvm.cttz.v8i8"]
    fn cttz_u8x8(x: u8x8, is_zero_undef: bool) -> u8x8;
    #[link_name = "llvm.cttz.v16i8"]
    fn cttz_u8x16(x: u8x16, is_zero_undef: bool) -> u8x16;
    #[link_name = "llvm.cttz.v32i8"]
    fn cttz_u8x32(x: u8x32, is_zero_undef: bool) -> u8x32;
    #[link_name = "llvm.cttz.v64i8"]
    fn cttz_u8x64(x: u8x64, is_zero_undef: bool) -> u8x64;

    #[link_name = "llvm.cttz.v2i16"]
    fn cttz_u16x2(x: u16x2, is_zero_undef: bool) -> u16x2;
    #[link_name = "llvm.cttz.v4i16"]
    fn cttz_u16x4(x: u16x4, is_zero_undef: bool) -> u16x4;
    #[link_name = "llvm.cttz.v8i16"]
    fn cttz_u16x8(x: u16x8, is_zero_undef: bool) -> u16x8;
    #[link_name = "llvm.cttz.v16i16"]
    fn cttz_u16x16(x: u16x16, is_zero_undef: bool) -> u16x16;
    #[link_name = "llvm.cttz.v32i16"]
    fn cttz_u16x32(x: u16x32, is_zero_undef: bool) -> u16x32;

    #[link_name = "llvm.cttz.v2i32"]
    fn cttz_u32x2(x: u32x2, is_zero_undef: bool) -> u32x2;
    #[link_name = "llvm.cttz.v4i32"]
    fn cttz_u32x4(x: u32x4, is_zero_undef: bool) -> u32x4;
    #[link_name = "llvm.cttz.v8i32"]
    fn cttz_u32x8(x: u32x8, is_zero_undef: bool) -> u32x8;
    #[link_name = "llvm.cttz.v16i32"]
    fn cttz_u32x16(x: u32x16, is_zero_undef: bool) -> u32x16;

    #[link_name = "llvm.cttz.v2i64"]
    fn cttz_u64x2(x: u64x2, is_zero_undef: bool) -> u64x2;
    #[link_name = "llvm.cttz.v4i64"]
    fn cttz_u64x4(x: u64x4, is_zero_undef: bool) -> u64x4;
    #[link_name = "llvm.cttz.v8i64"]
    fn cttz_u64x8(x: u64x8, is_zero_undef: bool) -> u64x8;

    #[link_name = "llvm.cttz.v1i128"]
    fn cttz_u128x1(x: u128x1, is_zero_undef: bool) -> u128x1;
    #[link_name = "llvm.cttz.v2i128"]
    fn cttz_u128x2(x: u128x2, is_zero_undef: bool) -> u128x2;
    #[link_name = "llvm.cttz.v4i128"]
    fn cttz_u128x4(x: u128x4, is_zero_undef: bool) -> u128x4;


    #[link_name = "llvm.ctpop.v2i8"]
    fn ctpop_u8x2(x: u8x2) -> u8x2;
    #[link_name = "llvm.ctpop.v4i8"]
    fn ctpop_u8x4(x: u8x4) -> u8x4;
    #[link_name = "llvm.ctpop.v8i8"]
    fn ctpop_u8x8(x: u8x8) -> u8x8;
    #[link_name = "llvm.ctpop.v16i8"]
    fn ctpop_u8x16(x: u8x16) -> u8x16;
    #[link_name = "llvm.ctpop.v32i8"]
    fn ctpop_u8x32(x: u8x32) -> u8x32;
    #[link_name = "llvm.ctpop.v64i8"]
    fn ctpop_u8x64(x: u8x64) -> u8x64;

    #[link_name = "llvm.ctpop.v2i16"]
    fn ctpop_u16x2(x: u16x2) -> u16x2;
    #[link_name = "llvm.ctpop.v4i16"]
    fn ctpop_u16x4(x: u16x4) -> u16x4;
    #[link_name = "llvm.ctpop.v8i16"]
    fn ctpop_u16x8(x: u16x8) -> u16x8;
    #[link_name = "llvm.ctpop.v16i16"]
    fn ctpop_u16x16(x: u16x16) -> u16x16;
    #[link_name = "llvm.ctpop.v32i16"]
    fn ctpop_u16x32(x: u16x32) -> u16x32;

    #[link_name = "llvm.ctpop.v2i32"]
    fn ctpop_u32x2(x: u32x2) -> u32x2;
    #[link_name = "llvm.ctpop.v4i32"]
    fn ctpop_u32x4(x: u32x4) -> u32x4;
    #[link_name = "llvm.ctpop.v8i32"]
    fn ctpop_u32x8(x: u32x8) -> u32x8;
    #[link_name = "llvm.ctpop.v16i32"]
    fn ctpop_u32x16(x: u32x16) -> u32x16;

    #[link_name = "llvm.ctpop.v2i64"]
    fn ctpop_u64x2(x: u64x2) -> u64x2;
    #[link_name = "llvm.ctpop.v4i64"]
    fn ctpop_u64x4(x: u64x4) -> u64x4;
    #[link_name = "llvm.ctpop.v8i64"]
    fn ctpop_u64x8(x: u64x8) -> u64x8;

    #[link_name = "llvm.ctpop.v1i128"]
    fn ctpop_u128x1(x: u128x1) -> u128x1;
    #[link_name = "llvm.ctpop.v2i128"]
    fn ctpop_u128x2(x: u128x2) -> u128x2;
    #[link_name = "llvm.ctpop.v4i128"]
    fn ctpop_u128x4(x: u128x4) -> u128x4;
}

crate trait BitManip {
    fn ctpop(self) -> Self;
    fn ctlz(self) -> Self;
    fn cttz(self) -> Self;
}

macro_rules! impl_bit_manip {
    (inner: $ty:ident, $uty:ident, $ctpop:ident, $ctlz:ident, $cttz:ident) => {
        impl BitManip for $ty {
            #[inline]
            fn ctpop(self) -> Self {
                let y: $uty = self.cast();
                unsafe { $ctpop(y).cast() }
            }

            #[inline]
            fn ctlz(self) -> Self {
                let y: $uty = self.cast();
                // the ctxx intrinsics need compile-time constant `is_zero_undef`
                unsafe { $ctlz(y, false).cast() }
            }

            #[inline]
            fn cttz(self) -> Self {
                let y: $uty = self.cast();
                unsafe { $cttz(y, false).cast() }
            }
        }
    };
    ($uty:ident, $ity:ident, $ctpop:ident, $ctlz:ident, $cttz:ident) => {
        impl_bit_manip! { inner: $uty, $uty, $ctpop, $ctlz, $cttz }
        impl_bit_manip! { inner: $ity, $uty, $ctpop, $ctlz, $cttz }
    };
}

impl_bit_manip! { u8x2,   i8x2,   ctpop_u8x2,   ctlz_u8x2,   cttz_u8x2   }
impl_bit_manip! { u8x4,   i8x4,   ctpop_u8x4,   ctlz_u8x4,   cttz_u8x4   }
impl_bit_manip! { u8x8,   i8x8,   ctpop_u8x8,   ctlz_u8x8,   cttz_u8x8   }
impl_bit_manip! { u8x16,  i8x16,  ctpop_u8x16,  ctlz_u8x16,  cttz_u8x16  }
impl_bit_manip! { u8x32,  i8x32,  ctpop_u8x32,  ctlz_u8x32,  cttz_u8x32  }
impl_bit_manip! { u8x64,  i8x64,  ctpop_u8x64,  ctlz_u8x64,  cttz_u8x64  }
impl_bit_manip! { u16x2,  i16x2,  ctpop_u16x2,  ctlz_u16x2,  cttz_u16x2  }
impl_bit_manip! { u16x4,  i16x4,  ctpop_u16x4,  ctlz_u16x4,  cttz_u16x4  }
impl_bit_manip! { u16x8,  i16x8,  ctpop_u16x8,  ctlz_u16x8,  cttz_u16x8  }
impl_bit_manip! { u16x16, i16x16, ctpop_u16x16, ctlz_u16x16, cttz_u16x16 }
impl_bit_manip! { u16x32, i16x32, ctpop_u16x32, ctlz_u16x32, cttz_u16x32 }
impl_bit_manip! { u32x2,  i32x2,  ctpop_u32x2,  ctlz_u32x2,  cttz_u32x2  }
impl_bit_manip! { u32x4,  i32x4,  ctpop_u32x4,  ctlz_u32x4,  cttz_u32x4  }
impl_bit_manip! { u32x8,  i32x8,  ctpop_u32x8,  ctlz_u32x8,  cttz_u32x8  }
impl_bit_manip! { u32x16, i32x16, ctpop_u32x16, ctlz_u32x16, cttz_u32x16 }
impl_bit_manip! { u64x2,  i64x2,  ctpop_u64x2,  ctlz_u64x2,  cttz_u64x2  }
impl_bit_manip! { u64x4,  i64x4,  ctpop_u64x4,  ctlz_u64x4,  cttz_u64x4  }
impl_bit_manip! { u64x8,  i64x8,  ctpop_u64x8,  ctlz_u64x8,  cttz_u64x8  }
impl_bit_manip! { u128x1, i128x1, ctpop_u128x1, ctlz_u128x1, cttz_u128x1 }
impl_bit_manip! { u128x2, i128x2, ctpop_u128x2, ctlz_u128x2, cttz_u128x2 }
impl_bit_manip! { u128x4, i128x4, ctpop_u128x4, ctlz_u128x4, cttz_u128x4 }

cfg_if! {
    if #[cfg(target_pointer_width = "8")] {
        impl_bit_manip! { inner: usizex2, u8x2,  ctpop_u8x2,  ctlz_u8x2,  cttz_u8x2  }
        impl_bit_manip! { inner: isizex2, u8x2,  ctpop_u8x2,  ctlz_u8x2,  cttz_u8x2  }
        impl_bit_manip! { inner: usizex4, u8x4,  ctpop_u8x4,  ctlz_u8x4,  cttz_u8x4  }
        impl_bit_manip! { inner: isizex4, u8x4,  ctpop_u8x4,  ctlz_u8x4,  cttz_u8x4  }
        impl_bit_manip! { inner: usizex8, u8x8,  ctpop_u8x8,  ctlz_u8x8,  cttz_u8x8  }
        impl_bit_manip! { inner: isizex8, u8x8,  ctpop_u8x8,  ctlz_u8x8,  cttz_u8x8  }
    } else if #[cfg(target_pointer_width = "16")] {
        impl_bit_manip! { inner: usizex2, u16x2, ctpop_u16x2, ctlz_u16x2, cttz_u16x2 }
        impl_bit_manip! { inner: isizex2, u16x2, ctpop_u16x2, ctlz_u16x2, cttz_u16x2 }
        impl_bit_manip! { inner: usizex4, u16x4, ctpop_u16x4, ctlz_u16x4, cttz_u16x4 }
        impl_bit_manip! { inner: isizex4, u16x4, ctpop_u16x4, ctlz_u16x4, cttz_u16x4 }
        impl_bit_manip! { inner: usizex8, u16x8, ctpop_u16x8, ctlz_u16x8, cttz_u16x8 }
        impl_bit_manip! { inner: isizex8, u16x8, ctpop_u16x8, ctlz_u16x8, cttz_u16x8 }
    } else if #[cfg(target_pointer_width = "32")] {
        impl_bit_manip! { inner: usizex2, u32x2, ctpop_u32x2, ctlz_u32x2, cttz_u32x2 }
        impl_bit_manip! { inner: isizex2, u32x2, ctpop_u32x2, ctlz_u32x2, cttz_u32x2 }
        impl_bit_manip! { inner: usizex4, u32x4, ctpop_u32x4, ctlz_u32x4, cttz_u32x4 }
        impl_bit_manip! { inner: isizex4, u32x4, ctpop_u32x4, ctlz_u32x4, cttz_u32x4 }
        impl_bit_manip! { inner: usizex8, u32x8, ctpop_u32x8, ctlz_u32x8, cttz_u32x8 }
        impl_bit_manip! { inner: isizex8, u32x8, ctpop_u32x8, ctlz_u32x8, cttz_u32x8 }
    } else if #[cfg(target_pointer_width = "64")] {
        impl_bit_manip! { inner: usizex2, u64x2, ctpop_u64x2, ctlz_u64x2, cttz_u64x2 }
        impl_bit_manip! { inner: isizex2, u64x2, ctpop_u64x2, ctlz_u64x2, cttz_u64x2 }
        impl_bit_manip! { inner: usizex4, u64x4, ctpop_u64x4, ctlz_u64x4, cttz_u64x4 }
        impl_bit_manip! { inner: isizex4, u64x4, ctpop_u64x4, ctlz_u64x4, cttz_u64x4 }
        impl_bit_manip! { inner: usizex8, u64x8, ctpop_u64x8, ctlz_u64x8, cttz_u64x8 }
        impl_bit_manip! { inner: isizex8, u64x8, ctpop_u64x8, ctlz_u64x8, cttz_u64x8 }
    } else {
        compile_error!("unsupported target_pointer_width");
    }
}
