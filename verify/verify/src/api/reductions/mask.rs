//! Verify the mask reduction API.

#[allow(unused)]
macro_rules! verify_mask {
    ($mask_id:ident[$target_feature:tt] => $all_instr:tt, $any_instr:tt, $none_instr:tt) => {
        interpolate_idents! {
            #[inline]
            #[target_feature(enable = $target_feature)]
            #[assert_instr($all_instr)]
            pub unsafe fn [$mask_id _all](x: $mask_id) -> bool {
                x.all()
            }
            #[inline]
            #[target_feature(enable = $target_feature)]
            #[assert_instr($any_instr)]
            pub unsafe fn [$mask_id _any](x: $mask_id) -> bool {
                x.any()
            }
            #[inline]
            #[target_feature(enable = $target_feature)]
            #[assert_instr($none_instr)]
            pub unsafe fn [$mask_id _none](x: $mask_id) -> bool {
                x.none()
            }
        }
    };
    ($mask_id:ident[$target_feature:tt] => $instr:tt) => {
        verify_mask!($mask_id[$target_feature] => $instr, $instr, $instr);
    };
}

cfg_if! {
    if #[cfg(any(target_arch = "x86", target_arch = "x86_64"))] {
        // FIXME: avx512
        #[cfg(not(target_feature = "avx512f"))]
        mod avx2;
        #[cfg(not(target_feature = "avx2"))]
        mod avx;
        #[cfg(not(target_feature = "avx"))]
        mod sse2;
        #[cfg(not(target_feature = "sse2"))]
        mod sse;
    }
}
