//! Mask reductions implementation for `x86` and `x86_64` targets

#[cfg(target_feature = "mmx")]
#[macro_use]
mod mmx;

#[cfg(target_feature = "sse")]
#[macro_use]
mod sse;

#[cfg(target_feature = "sse2")]
#[macro_use]
mod sse2;

#[cfg(target_feature = "avx")]
#[macro_use]
mod avx;

/// x86 64-bit m8x8 implementation
macro_rules! x86_m8x8_impl {
    ($id:ident) => {
        cfg_if! {
            if #[cfg(all(target_arch = "x86_64", target_feature = "mmx"))] {
                x86_m8x8_mmx_impl!($id);
            } else {
                fallback_impl!($id);
            }
        }
    };
}

/// x86 128-bit m8x16 implementation
macro_rules! x86_m8x16_impl {
    ($id:ident) => {
        cfg_if! {
            if #[cfg(target_feature = "sse2")] {
                x86_m8x16_sse2_impl!($id);
            } else {
                fallback_impl!($id);
            }
        }
    };
}

/// x86 128-bit m32x4 implementation
macro_rules! x86_m32x4_impl {
    ($id:ident) => {
        cfg_if! {
            if #[cfg(target_feature = "sse4.1")] {
                x86_128_sse41_impl!($id);
            } else if #[cfg(target_feature = "sse")] {
                x86_m32x4_sse_impl!($id);
            } else {
                fallback_impl!($id);
            }
        }
    };
}

/// x86 128-bit m64x2 implementation
macro_rules! x86_m64x2_impl {
    ($id:ident) => {
        cfg_if! {
            if #[cfg(target_feature = "sse4.1")] {
                x86_128_sse41_impl!($id);
            } else if #[cfg(target_feature = "sse2")] {
                x86_m64x2_sse2_impl!($id);
            } else {
                fallback_impl!($id);
            }
        }
    };
}

/// x86 256-bit implementation
macro_rules! x86_m8x32_impl {
    ($id:ident, $half_id:ident) => {
        cfg_if! {
            if #[cfg(target_feature = "avx")] {
                x86_m8x32_avx_impl!($id);
            } else if #[cfg(target_feature = "sse2")] {
                recurse_half!($id, $half_id);
            } else {
                fallback_impl!($id);
            }
        }
    };
}

/// Mask reduction implementation for `x86` and `x86_64` targets
macro_rules! impl_mask_reductions {
    // 64-bit wide masks
    (m8x8) => { x86_m8x8_impl!(m8x8); };
    (m16x4) => { x86_m8x8_impl!(m16x4); };
    (m32x2) => { x86_m8x8_impl!(m32x2); };
    // 128-bit wide masks
    (m8x16) => { x86_m8x16_impl!(m8x16); };
    (m16x8) => { x86_m8x16_impl!(m16x8); };
    (m32x4) => { x86_m32x4_impl!(m32x4); };
    (m64x2) => { x86_m64x2_impl!(m64x2); };
    // 256-bit wide masks:
    (m8x32) => { x86_m8x32_impl!(m8x32, m8x16); };
    (m16x16) => { x86_m8x32_impl!(m16x16, m16x8); };
    (m32x8) => { x86_m8x32_impl!(m32x8, m32x4); };
    (m64x4) => { x86_m8x32_impl!(m64x4, m64x2); };
    // Fallback to LLVM's default code-generation:
    ($id:ident) => { fallback_impl!($id); };
}
