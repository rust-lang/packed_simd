//! Scalar intersection result

use geometry::V3D;

/// Intersection result
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Isect {
    pub t: f32,
    pub p: V3D,
    pub n: V3D,
    pub hit: bool,
}

impl Default for Isect {
    #[inline]
    fn default() -> Self {
        Self {
            t: 1e17,
            hit: false,
            p: V3D::default(),
            n: V3D::default(),
        }
    }
}
