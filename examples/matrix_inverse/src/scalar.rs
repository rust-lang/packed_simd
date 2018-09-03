//! Scalar implementation
#![rustfmt::skip]
use ::*;

pub fn inv4x4(m: Matrix4x4) -> Option<Matrix4x4> {
    let m = m.0;

    let mut inv = [
        [ // row 0:
            // 0,0:
            m[1][1]  * m[2][2] * m[3][3] -
            m[1][1]  * m[2][3] * m[3][2] -
            m[2][1]  * m[1][2]  * m[3][3] +
            m[2][1]  * m[1][3]  * m[3][2] +
            m[3][1] * m[1][2]  * m[2][3] -
            m[3][1] * m[1][3]  * m[2][2],
            // 0,1:
           -m[0][1]  * m[2][2] * m[3][3] +
            m[0][1]  * m[2][3] * m[3][2] +
            m[2][1]  * m[0][2] * m[3][3] -
            m[2][1]  * m[0][3] * m[3][2] -
            m[3][1] * m[0][2] * m[2][3] +
            m[3][1] * m[0][3] * m[2][2],
            // 0,2:
            m[0][1]  * m[1][2] * m[3][3] -
            m[0][1]  * m[1][3] * m[3][2] -
            m[1][1]  * m[0][2] * m[3][3] +
            m[1][1]  * m[0][3] * m[3][2] +
            m[3][1] * m[0][2] * m[1][3] -
            m[3][1] * m[0][3] * m[1][2],
            // 0,3:
           -m[0][1] * m[1][2] * m[2][3] +
            m[0][1] * m[1][3] * m[2][2] +
            m[1][1] * m[0][2] * m[2][3] -
            m[1][1] * m[0][3] * m[2][2] -
            m[2][1] * m[0][2] * m[1][3] +
            m[2][1] * m[0][3] * m[1][2],
        ],
        [ // row 1
            // 1,0:
           -m[1][0]  * m[2][2] * m[3][3] +
            m[1][0]  * m[2][3] * m[3][2] +
            m[2][0]  * m[1][2]  * m[3][3] -
            m[2][0]  * m[1][3]  * m[3][2] -
            m[3][0] * m[1][2]  * m[2][3] +
            m[3][0] * m[1][3]  * m[2][2],
            // 1,1:
            m[0][0]  * m[2][2] * m[3][3] -
            m[0][0]  * m[2][3] * m[3][2] -
            m[2][0]  * m[0][2] * m[3][3] +
            m[2][0]  * m[0][3] * m[3][2] +
            m[3][0] * m[0][2] * m[2][3] -
            m[3][0] * m[0][3] * m[2][2],
            // 1,2:
           -m[0][0]  * m[1][2] * m[3][3] +
            m[0][0]  * m[1][3] * m[3][2] +
            m[1][0]  * m[0][2] * m[3][3] -
            m[1][0]  * m[0][3] * m[3][2] -
            m[3][0] * m[0][2] * m[1][3] +
            m[3][0] * m[0][3] * m[1][2],
            // 1,3:
            m[0][0] * m[1][2] * m[2][3] -
            m[0][0] * m[1][3] * m[2][2] -
            m[1][0] * m[0][2] * m[2][3] +
            m[1][0] * m[0][3] * m[2][2] +
            m[2][0] * m[0][2] * m[1][3] -
            m[2][0] * m[0][3] * m[1][2],
        ],
        [ // row 2
            // 2,0:
            m[1][0]  * m[2][1] * m[3][3] -
            m[1][0]  * m[2][3] * m[3][1] -
            m[2][0]  * m[1][1] * m[3][3] +
            m[2][0]  * m[1][3] * m[3][1] +
            m[3][0] * m[1][1] * m[2][3] -
            m[3][0] * m[1][3] * m[2][1],
            // 2,1:
           -m[0][0]  * m[2][1] * m[3][3] +
            m[0][0]  * m[2][3] * m[3][1] +
            m[2][0]  * m[0][1] * m[3][3] -
            m[2][0]  * m[0][3] * m[3][1] -
            m[3][0] * m[0][1] * m[2][3] +
            m[3][0] * m[0][3] * m[2][1],
            // 2,2:
            m[0][0]  * m[1][1] * m[3][3] -
            m[0][0]  * m[1][3] * m[3][1] -
            m[1][0]  * m[0][1] * m[3][3] +
            m[1][0]  * m[0][3] * m[3][1] +
            m[3][0] * m[0][1] * m[1][3] -
            m[3][0] * m[0][3] * m[1][1],
            // 2,3:
           -m[0][0] * m[1][1] * m[2][3] +
            m[0][0] * m[1][3] * m[2][1] +
            m[1][0] * m[0][1] * m[2][3] -
            m[1][0] * m[0][3] * m[2][1] -
            m[2][0] * m[0][1] * m[1][3] +
            m[2][0] * m[0][3] * m[1][1],
        ],
        [ // row 3
            // 3,0:
           -m[1][0]  * m[2][1] * m[3][2] +
            m[1][0]  * m[2][2] * m[3][1] +
            m[2][0]  * m[1][1] * m[3][2] -
            m[2][0]  * m[1][2] * m[3][1] -
            m[3][0] * m[1][1] * m[2][2] +
            m[3][0] * m[1][2] * m[2][1],
            // 3,1:
            m[0][0]  * m[2][1] * m[3][2] -
            m[0][0]  * m[2][2] * m[3][1] -
            m[2][0]  * m[0][1] * m[3][2] +
            m[2][0]  * m[0][2] * m[3][1] +
            m[3][0] * m[0][1] * m[2][2] -
            m[3][0] * m[0][2] * m[2][1],
            // 3,2:
           -m[0][0]  * m[1][1] * m[3][2] +
            m[0][0]  * m[1][2] * m[3][1] +
            m[1][0]  * m[0][1] * m[3][2] -
            m[1][0]  * m[0][2] * m[3][1] -
            m[3][0] * m[0][1] * m[1][2] +
            m[3][0] * m[0][2] * m[1][1],
            // 3,3:
            m[0][0] * m[1][1] * m[2][2] -
            m[0][0] * m[1][2] * m[2][1] -
            m[1][0] * m[0][1] * m[2][2] +
            m[1][0] * m[0][2] * m[2][1] +
            m[2][0] * m[0][1] * m[1][2] -
            m[2][0] * m[0][2] * m[1][1],
        ],
    ];

    let det = m[0][0] * inv[0][0] + m[0][1] * inv[1][0] +
              m[0][2] * inv[2][0] + m[0][3] * inv[3][0];
    if det == 0. { return None; }

    let det_inv = 1. / det;

    for row in &mut inv {
        for elem in row.iter_mut() {
            *elem *= det_inv;
        }
    }

    Some(Matrix4x4(inv))
}

#[cfg(test)]
#[test]
fn test() {
    ::test(inv4x4)
}
