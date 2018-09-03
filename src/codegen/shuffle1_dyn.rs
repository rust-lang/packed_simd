//! Shuffle vector lanes with run-time indices.

use *;

crate trait Shuffle1Dyn {
    fn shuffle1_dyn(self, Self) -> Self;
}

// Fallback implementation
macro_rules! impl_fallback {
    ($id:ident) => {
        impl Shuffle1Dyn for $id {
            #[inline]
            fn shuffle1_dyn(self, indices: Self) -> Self {
                let mut result = Self::splat(0);
                for i in 0..$id::lanes() {
                    result = result
                        .replace(i, self.extract(indices.extract(i) as usize));
                }
                result
            }
        }
    };
}

macro_rules! impl_shuffle1_dyn {
    (u8x8) => {
        cfg_if! {
            if #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"),
                         target_feature = "ssse3"))] {
                impl Shuffle1Dyn for u8x8 {
                    #[inline]
                    fn shuffle1_dyn(self, indices: Self) -> Self {
                        #[cfg(target_arch = "x86")]
                        use arch::x86::_mm_shuffle_pi8;
                        #[cfg(target_arch = "x86_64")]
                        use arch::x86_64::_mm_shuffle_pi8;

                        unsafe {
                            mem::transmute(_mm_shuffle_pi8(
                                mem::transmute(self.0), mem::transmute(indices.0))
                            )
                        }
                    }
                }
            } else if #[cfg(all(any(
                    all(target_aarch = "aarch64", target_feature = "neon"),
                    all(target_aarch = "arm", target_feature = "v7",
                        target_feature = "neon")),
                feature = "coresimd")
            )] {
                impl Shuffle1Dyn for u8x8 {
                    #[inline]
                    fn shuffle1_dyn(self, indices: Self) -> Self {
                        #[cfg(targt_arch = "aarch64")]
                        use arch::aarch64::vtbl1_u8;
                        #[cfg(targt_arch = "arm")]
                        use arch::arm::vtbl1_u8;

                        // This is safe because the binary is compiled with
                        // neon enabled at compile-time and can therefore only
                        // run on CPUs that have it enabled.
                        unsafe {
                            Simd(mem::transmute(
                                vtbl1_u8(mem::transmute(self.0),
                                         mem::transmute(indices.0))
                            ))
                        }
                    }
                }
            } else {
                impl_fallback!(u8x8);
            }
        }
    };
    (u8x16) => {
        cfg_if! {
            if #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"),
                         target_feature = "ssse3"))] {
                impl Shuffle1Dyn for u8x16 {
                    #[inline]
                    fn shuffle1_dyn(self, indices: Self) -> Self {
                        #[cfg(target_arch = "x86")]
                        use arch::x86::_mm_shuffle_epi8;
                        #[cfg(target_arch = "x86_64")]
                        use arch::x86_64::_mm_shuffle_epi8;
                        // This is safe because the binary is compiled with
                        // ssse3 enabled at compile-time and can therefore only
                        // run on CPUs that have it enabled.
                        unsafe {
                            Simd(mem::transmute(
                                _mm_shuffle_epi8(mem::transmute(self.0),
                                                 mem::transmute(indices))
                            ))
                        }
                    }
                }
            } else if #[cfg(all(target_aarch = "aarch64", target_feature = "neon",
                                feature = "coresimd"))] {
                impl Shuffle1Dyn for u8x16 {
                    #[inline]
                    fn shuffle1_dyn(self, indices: Self) -> Self {
                        use arch::aarch64::vqtbl1q_u8;

                        // This is safe because the binary is compiled with
                        // neon enabled at compile-time and can therefore only
                        // run on CPUs that have it enabled.
                        unsafe {
                            Simd(mem::transmute(
                                vqtbl1q_u8(mem::transmute(self.0),
                                           mem::transmute(indices.0))
                            ))
                        }
                    }
                }
            } else if #[cfg(all(target_aarch = "arm", target_feature = "v7",
                                target_feature = "neon", feature = "coresimd"))] {
                impl Shuffle1Dyn for u8x16 {
                    #[inline]
                    fn shuffle1_dyn(self, indices: Self) -> Self {
                        use arch::arm::vtbl2_u8;

                        // This is safe because the binary is compiled with
                        // neon enabled at compile-time and can therefore only
                        // run on CPUs that have it enabled.
                        unsafe {
                            union U {
                                j: u8x16,
                                s: (u8x8, u8x8),
                            }

                            let (i0, i1) = U { j: y }.s;

                            let r0 = vtbl2_u8(mem::transmute(x), mem::transmute(i0));
                            let r1 = vtbl2_u8(mem::transmute(x), mem::transmute(i1));

                            let r = U { s: (r0, r1) }.j;

                            Simd(mem::transmute(r))
                        }
                    }
                }
            } else {
                impl_fallback!(u8x16);
            }
        }
    };
    (u16x8) => {
        impl Shuffle1Dyn for u16x8 {
            #[inline]
            fn shuffle1_dyn(self, indices: Self) -> Self {
                let indices: u8x8 = (indices * 2).cast();
                let indices: u8x16 = shuffle!(indices, [0, 0, 1, 1, 2, 2, 3, 3,
                                                        4, 4, 5, 5, 6, 6, 7, 7]);
                let v = u8x16::new(0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1);
                let indices = indices + v;
                unsafe {
                    let s: u8x16 = mem::transmute(self);
                    mem::transmute(s.shuffle1_dyn(indices))
                }
            }
        }
    };
    (u32x4) => {
        cfg_if! {
            if #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"),
                         target_feature = "avx"))] {
                impl Shuffle1Dyn for u32x4 {
                    #[inline]
                    fn shuffle1_dyn(self, indices: Self) -> Self {
                        #[cfg(target_arch = "x86")]
                        use arch::x86::{_mm_permutevar_ps};
                        #[cfg(target_arch = "x86_64")]
                        use arch::x86_64::{_mm_permutevar_ps};

                        unsafe {
                            mem::transmute(_mm_permutevar_ps(
                                mem::transmute(self.0), mem::transmute(indices.0))
                            )
                        }
                    }
                }
            } else {
                impl Shuffle1Dyn for u32x4 {
                    #[inline]
                    fn shuffle1_dyn(self, indices: Self) -> Self {
                        let indices: u8x4 = (indices * 4).cast();
                        let indices: u8x16 = shuffle!(indices, [0, 0, 0, 0, 1, 1, 1, 1,
                                                                2, 2, 2, 2, 3, 3, 3, 3]);
                        let v = u8x16::new(0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3, 0, 1, 2, 3);
                        let indices = indices + v;
                        unsafe {
                            let s: u8x16 = mem::transmute(self);
                            mem::transmute(s.shuffle1_dyn(indices))
                        }
                    }
                }
            }
        }
    };
    (u64x2) => {
        cfg_if! {
            if #[cfg(all(any(target_arch = "x86", target_arch = "x86_64"),
                         target_feature = "avx"))] {
                impl Shuffle1Dyn for u64x2 {
                    #[inline]
                    fn shuffle1_dyn(self, indices: Self) -> Self {
                        #[cfg(target_arch = "x86")]
                        use arch::x86::{_mm_permutevar_pd};
                        #[cfg(target_arch = "x86_64")]
                        use arch::x86_64::{_mm_permutevar_pd};
                        // _mm_permutevar_pd uses the _second_ bit of each
                        // element to perform the selection, that is: 0b00 => 0,
                        // 0b10 => 1:
                        let indices = indices << 1;
                        unsafe {
                            mem::transmute(_mm_permutevar_pd(
                                mem::transmute(self), mem::transmute(indices))
                            )
                        }
                    }
                }
            } else {
                impl Shuffle1Dyn for u64x2 {
                    #[inline]
                    fn shuffle1_dyn(self, indices: Self) -> Self {
                        let indices: u8x2 = (indices * 8).cast();
                        let indices: u8x16 = shuffle!(indices, [0, 0, 0, 0, 0, 0, 0, 0,
                                                                1, 1, 1, 1, 1, 1, 1, 1]);
                        let v = u8x16::new(0, 1, 2, 3, 4, 5, 6, 7, 0, 1, 2, 3, 4, 5, 6, 7);
                        let indices = indices + v;
                        unsafe {
                            let s: u8x16 = mem::transmute(self);
                            mem::transmute(s.shuffle1_dyn(indices))
                        }
                    }
                }
            }
        }
    };
    (u128x1) => {
        impl Shuffle1Dyn for u128x1 {
            #[inline]
            fn shuffle1_dyn(self, _indices: Self) -> Self {
                self
            }
        }
    };
    ($id:ident) => { impl_fallback!($id); }
}

impl_shuffle1_dyn!(u8x2);
impl_shuffle1_dyn!(u8x4);
impl_shuffle1_dyn!(u8x8);
impl_shuffle1_dyn!(u8x16);
impl_shuffle1_dyn!(u8x32);
impl_shuffle1_dyn!(u8x64);

impl_shuffle1_dyn!(u16x2);
impl_shuffle1_dyn!(u16x4);
impl_shuffle1_dyn!(u16x8);
impl_shuffle1_dyn!(u16x16);
impl_shuffle1_dyn!(u16x32);

impl_shuffle1_dyn!(u32x2);
impl_shuffle1_dyn!(u32x4);
impl_shuffle1_dyn!(u32x8);
impl_shuffle1_dyn!(u32x16);

impl_shuffle1_dyn!(u64x2);
impl_shuffle1_dyn!(u64x4);
impl_shuffle1_dyn!(u64x8);

impl_shuffle1_dyn!(usizex2);
impl_shuffle1_dyn!(usizex4);
impl_shuffle1_dyn!(usizex8);

impl_shuffle1_dyn!(u128x1);
impl_shuffle1_dyn!(u128x2);
impl_shuffle1_dyn!(u128x4);
