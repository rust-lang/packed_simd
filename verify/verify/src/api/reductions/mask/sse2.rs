//! Verification of the mask reduction API for `x86`/`x86_64`+`SSE2`

#![allow(unused)]
use packed_simd::*;
use stdsimd_test::assert_instr;

#[inline]
#[target_feature(enable = "sse2")]
#[assert_instr(pmovmskb)]
unsafe fn all_m8x16(x: m8x16) -> bool {
    x.all()
}

#[inline]
#[target_feature(enable = "sse2")]
#[assert_instr(pmovmskb)]
unsafe fn all_m16x8(x: m8x16) -> bool {
    x.all()
}

#[inline]
#[target_feature(enable = "sse2")]
#[assert_instr(pmovmsks)]
unsafe fn all_m32x4(x: m8x16) -> bool {
    x.all()
}

#[inline]
#[target_feature(enable = "sse2")]
#[assert_instr(pmovmskd)]
unsafe fn all_m64x2(x: m8x16) -> bool {
    x.all()
}

#[inline]
#[target_feature(enable = "sse2")]
#[assert_instr(pmovmskb)]
unsafe fn any_m8x16(x: m8x16) -> bool {
    x.any()
}

#[inline]
#[target_feature(enable = "sse2")]
#[assert_instr(pmovmskb)]
unsafe fn any_m16x8(x: m8x16) -> bool {
    x.any()
}

#[inline]
#[target_feature(enable = "sse2")]
#[assert_instr(pmovmsks)]
unsafe fn any_m32x4(x: m8x16) -> bool {
    x.any()
}

#[inline]
#[target_feature(enable = "sse2")]
#[assert_instr(pmovmskd)]
unsafe fn any_m64x2(x: m8x16) -> bool {
    x.any()
}

#[inline]
#[target_feature(enable = "sse2")]
#[assert_instr(pmovmskb)]
unsafe fn none_m8x16(x: m8x16) -> bool {
    x.none()
}

#[inline]
#[target_feature(enable = "sse2")]
#[assert_instr(pmovmskb)]
unsafe fn none_m16x8(x: m8x16) -> bool {
    x.none()
}

#[inline]
#[target_feature(enable = "sse2")]
#[assert_instr(pmovmsks)]
unsafe fn none_m32x4(x: m8x16) -> bool {
    x.none()
}

#[inline]
#[target_feature(enable = "sse2")]
#[assert_instr(pmovmskd)]
unsafe fn none_m64x2(x: m8x16) -> bool {
    x.none()
}
