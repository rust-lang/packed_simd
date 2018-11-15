#[allow(improper_ctypes)]
extern "C" {
    #[link_name = "llvm.ctlz.v2xi8"]
    fn ctlz_u8x2(x: u8x2, is_zero_undef: bool) -> u8x2;
    #[link_name = "llvm.ctlz.v4xi8"]
    fn ctlz_u8x4(x: u8x4, is_zero_undef: bool) -> u8x4;
    #[link_name = "llvm.ctlz.v8xi8"]
    fn ctlz_u8x8(x: u8x8, is_zero_undef: bool) -> u8x8;
    #[link_name = "llvm.ctlz.v16xi8"]
    fn ctlz_u8x16(x: u8x16, is_zero_undef: bool) -> u8x16;
    #[link_name = "llvm.ctlz.v32xi8"]
    fn ctlz_u8x32(x: u8x32, is_zero_undef: bool) -> u8x32;
    #[link_name = "llvm.ctlz.v64xi8"]
    fn ctlz_u8x64(x: u8x64, is_zero_undef: bool) -> u8x64;

    #[link_name = "llvm.ctlz.v2xi16"]
    fn ctlz_u16x2(x: u16x2, is_zero_undef: bool) -> u16x2;
    #[link_name = "llvm.ctlz.v4xi16"]
    fn ctlz_u16x4(x: u16x4, is_zero_undef: bool) -> u16x4;
    #[link_name = "llvm.ctlz.v8xi16"]
    fn ctlz_u16x8(x: u16x8, is_zero_undef: bool) -> u16x8;
    #[link_name = "llvm.ctlz.v16xi16"]
    fn ctlz_u16x16(x: u16x16, is_zero_undef: bool) -> u16x16;
    #[link_name = "llvm.ctlz.v32xi16"]
    fn ctlz_u16x32(x: u16x32, is_zero_undef: bool) -> u16x32;

    #[link_name = "llvm.ctlz.v2xi32"]
    fn ctlz_u32x2(x: u32x2, is_zero_undef: bool) -> u32x2;
    #[link_name = "llvm.ctlz.v4xi32"]
    fn ctlz_u32x4(x: u32x4, is_zero_undef: bool) -> u32x4;
    #[link_name = "llvm.ctlz.v8xi32"]
    fn ctlz_u32x8(x: u32x8, is_zero_undef: bool) -> u32x8;
    #[link_name = "llvm.ctlz.v16xi32"]
    fn ctlz_u32x16(x: u32x16, is_zero_undef: bool) -> u32x16;

    #[link_name = "llvm.ctlz.v2xi64"]
    fn ctlz_u64x2(x: u64x2, is_zero_undef: bool) -> u64x2;
    #[link_name = "llvm.ctlz.v4xi64"]
    fn ctlz_u64x4(x: u64x4, is_zero_undef: bool) -> u64x4;
    #[link_name = "llvm.ctlz.v8xi64"]
    fn ctlz_u64x8(x: u64x8, is_zero_undef: bool) -> u64x8;
}
