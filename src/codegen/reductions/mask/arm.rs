//! Mask reductions implementation for `arm` targets

/// Implementation for ARM + v7 + NEON for 64-bitwide vectors
macro_rules! arm_64_v7_neon_impl {
    ($id:ident, $half:ident, $vpmin:ident, $vpmax:ident,
     $all:expr, $any:expr) => {
        impl All for $id {
            #[inline]
            #[target_feature(enable = "v7,neon")]
            unsafe fn all(self) -> bool {
                use crate::arch::arm::$vpmin;
                use crate::mem::{transmute, uninitialized};
                union U {
                    halves: ($half, $half),
                    vec: $id,
                }
                let vec: Self = transmute(
                    $vpmin(transmute(self), uninitialized())
                );
                let half: $half = U { vec }.halves.0;
                $all(half)
            }
        }
        impl Any for $id {
            #[inline]
            #[target_feature(enable = "v7,neon")]
            unsafe fn any(self) -> bool {
                use crate::arch::arm::$vpmax;
                use crate::mem::{transmute, uninitialized};
                union U {
                    halves: ($half, $half),
                    vec: $id,
                }
                let vec: Self = transmute(
                    $vpmax(transmute(self), uninitialized())
                );
                let half: $half = U { vec }.halves.0;
                $any(half)
            }
        }
    };

}


/// Implementation for ARM + v7 + NEON for 128-bit wide vectors with
/// more than two elements.
macro_rules! arm_128_v7_neon_impl {
    ($id:ident, $half:ident, $vpmin:ident, $vpmax:ident) => {
        impl All for $id {
            #[inline]
            #[target_feature(enable = "v7,neon")]
            unsafe fn all(self) -> bool {
                use crate::arch::arm::$vpmin;
                use crate::mem::transmute;
                union U {
                    halves: ($half, $half),
                    vec: $id,
                }
                let halves = U { vec: self }.halves;
                let h: $half = transmute($vpmin(
                    transmute(halves.0),
                    transmute(halves.1),
                ));
                h.all()
            }
        }
        impl Any for $id {
            #[inline]
            #[target_feature(enable = "v7,neon")]
            unsafe fn any(self) -> bool {
                use crate::arch::arm::$vpmax;
                use crate::mem::transmute;
                union U {
                    halves: ($half, $half),
                    vec: $id,
                }
                let halves = U { vec: self }.halves;
                let h: $half = transmute($vpmax(
                    transmute(halves.0),
                    transmute(halves.1),
                ));
                h.any()
            }
        }
    };
}

/// Mask reduction implementation for `arm` targets
macro_rules! impl_mask_reductions {
    // 64-bit wide masks
    (m8x8) => { arm_64_v7_neon_impl!(m8x8, m8x4, vpmin_u8, vpmax_u8,
                                     |x: m8x4| { x.all() }, |x: m8x4| { x.any() }); };
    (m16x4) => { arm_64_v7_neon_impl!(m16x4, m16x2, vpmin_u16, vpmax_u16,
                                      |x: m16x2| { x.all() }, |x: m16x2| { x.any() }); };
    (m32x2) => { arm_64_v7_neon_impl!(m32x2, u32, vpmin_u32, vpmax_u32,
                                      |x: u32| { x == u32::max_value() },
                                      |x: u32| { x != 0 }); };
    // 128-bit wide masks
    (m8x16) => { arm_128_v7_neon_impl!(m8x16, m8x8, vpmin_u8, vpmax_u8); };
    (m16x8) => { arm_128_v7_neon_impl!(m16x8, m16x4, vpmin_u16, vpmax_u16); };
    (m32x4) => { arm_128_v7_neon_impl!(m32x4, m32x2, vpmin_u32, vpmax_u32); };
    // Fallback to LLVM's default code-generation:
    ($id:ident) => { fallback_impl!($id); };
}
