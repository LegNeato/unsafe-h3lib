use ::libc;
use ::num_traits;
use num_traits::ToPrimitive;
extern "C" {
    fn abs(_: libc::c_int) -> libc::c_int;
    fn fabsl(_: f64) -> f64;
    fn lroundl(_: f64) -> libc::c_long;
}
pub type uint32_t = libc::c_uint;
pub type H3Error = uint32_t;
pub type C2RustUnnamed = libc::c_uint;
pub const E_OPTION_INVALID: C2RustUnnamed = 15;
pub const E_MEMORY_BOUNDS: C2RustUnnamed = 14;
pub const E_MEMORY_ALLOC: C2RustUnnamed = 13;
pub const E_RES_MISMATCH: C2RustUnnamed = 12;
pub const E_NOT_NEIGHBORS: C2RustUnnamed = 11;
pub const E_DUPLICATE_INPUT: C2RustUnnamed = 10;
pub const E_PENTAGON: C2RustUnnamed = 9;
pub const E_VERTEX_INVALID: C2RustUnnamed = 8;
pub const E_UNDIR_EDGE_INVALID: C2RustUnnamed = 7;
pub const E_DIR_EDGE_INVALID: C2RustUnnamed = 6;
pub const E_CELL_INVALID: C2RustUnnamed = 5;
pub const E_RES_DOMAIN: C2RustUnnamed = 4;
pub const E_LATLNG_DOMAIN: C2RustUnnamed = 3;
pub const E_DOMAIN: C2RustUnnamed = 2;
pub const E_FAILED: C2RustUnnamed = 1;
pub const E_SUCCESS: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CoordIJ {
    pub i: libc::c_int,
    pub j: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vec2d {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CoordIJK {
    pub i: libc::c_int,
    pub j: libc::c_int,
    pub k: libc::c_int,
}
pub type Direction = libc::c_uint;
pub const PENTAGON_SKIPPED_DIGIT: Direction = 1;
pub const NUM_DIGITS: Direction = 7;
pub const INVALID_DIGIT: Direction = 7;
pub const IJ_AXES_DIGIT: Direction = 6;
pub const IK_AXES_DIGIT: Direction = 5;
pub const I_AXES_DIGIT: Direction = 4;
pub const JK_AXES_DIGIT: Direction = 3;
pub const J_AXES_DIGIT: Direction = 2;
pub const K_AXES_DIGIT: Direction = 1;
pub const CENTER_DIGIT: Direction = 0;
static mut UNIT_VECS: [CoordIJK; 7] = [
    {
        CoordIJK {
            i: 0 as libc::c_int,
            j: 0 as libc::c_int,
            k: 0 as libc::c_int,
        }
    },
    {
        CoordIJK {
            i: 0 as libc::c_int,
            j: 0 as libc::c_int,
            k: 1 as libc::c_int,
        }
    },
    {
        CoordIJK {
            i: 0 as libc::c_int,
            j: 1 as libc::c_int,
            k: 0 as libc::c_int,
        }
    },
    {
        CoordIJK {
            i: 0 as libc::c_int,
            j: 1 as libc::c_int,
            k: 1 as libc::c_int,
        }
    },
    {
        CoordIJK {
            i: 1 as libc::c_int,
            j: 0 as libc::c_int,
            k: 0 as libc::c_int,
        }
    },
    {
        CoordIJK {
            i: 1 as libc::c_int,
            j: 0 as libc::c_int,
            k: 1 as libc::c_int,
        }
    },
    {
        CoordIJK {
            i: 1 as libc::c_int,
            j: 1 as libc::c_int,
            k: 0 as libc::c_int,
        }
    },
];
#[no_mangle]
pub unsafe extern "C" fn _setIJK(
    mut ijk: *mut CoordIJK,
    mut i: libc::c_int,
    mut j: libc::c_int,
    mut k: libc::c_int,
) {
    (*ijk).i = i;
    (*ijk).j = j;
    (*ijk).k = k;
}
#[no_mangle]
pub unsafe extern "C" fn _hex2dToCoordIJK(mut v: *const Vec2d, mut h: *mut CoordIJK) {
    let mut a1: libc::c_double = 0.;
    let mut a2: libc::c_double = 0.;
    let mut x1: libc::c_double = 0.;
    let mut x2: libc::c_double = 0.;
    let mut m1: libc::c_int = 0;
    let mut m2: libc::c_int = 0;
    let mut r1: libc::c_double = 0.;
    let mut r2: libc::c_double = 0.;
    (*h).k = 0 as libc::c_int;
    a1 = fabsl((*v).x).to_f64().unwrap();
    a2 = fabsl((*v).y).to_f64().unwrap();
    x2 = (a2 / 0.866_025_403_784_438_6).to_f64().unwrap();
    x1 = (a1 + x2 / 2.0).to_f64().unwrap();
    m1 = x1 as libc::c_int;
    m2 = x2 as libc::c_int;
    r1 = x1 - m1 as libc::c_double;
    r2 = x2 - m2 as libc::c_double;
    if r1 < 0.5 {
        if r1 < 1.0 / 3.0 {
            if r2 < (1.0 + r1) / 2.0 {
                (*h).i = m1;
                (*h).j = m2;
            } else {
                (*h).i = m1;
                (*h).j = m2 + 1 as libc::c_int;
            }
        } else {
            if r2 < 1.0 - r1 {
                (*h).j = m2;
            } else {
                (*h).j = m2 + 1 as libc::c_int;
            }
            if 1.0 - r1 <= r2 && r2 < 2.0f64 * r1 {
                (*h).i = m1 + 1 as libc::c_int;
            } else {
                (*h).i = m1;
            }
        }
    } else if r1 < 2.0 / 3.0 {
        if r2 < 1.0 - r1 {
            (*h).j = m2;
        } else {
            (*h).j = m2 + 1 as libc::c_int;
        }
        if 2.0 * r1 - 1.0 < r2 && r2 < 1.0 - r1 {
            (*h).i = m1;
        } else {
            (*h).i = m1 + 1 as libc::c_int;
        }
    } else if r2 < r1 / 2.0 {
        (*h).i = m1 + 1 as libc::c_int;
        (*h).j = m2;
    } else {
        (*h).i = m1 + 1 as libc::c_int;
        (*h).j = m2 + 1 as libc::c_int;
    }
    if (*v).x < 0.0 {
        if (*h).j % 2 as libc::c_int == 0 as libc::c_int {
            let mut axisi: libc::c_longlong = ((*h).j / 2 as libc::c_int) as libc::c_longlong;
            let mut diff: libc::c_longlong = (*h).i as libc::c_longlong - axisi;
            (*h).i = ((*h).i as libc::c_double - 2.0f64 * diff as libc::c_double) as libc::c_int;
        } else {
            let mut axisi_0: libc::c_longlong =
                (((*h).j + 1 as libc::c_int) / 2 as libc::c_int) as libc::c_longlong;
            let mut diff_0: libc::c_longlong = (*h).i as libc::c_longlong - axisi_0;
            (*h).i = ((*h).i as libc::c_double
                - (2.0f64 * diff_0 as libc::c_double + 1 as libc::c_int as libc::c_double))
                as libc::c_int;
        }
    }
    if (*v).y < 0.0 {
        (*h).i -= (2 as libc::c_int * (*h).j + 1 as libc::c_int) / 2 as libc::c_int;
        (*h).j *= -(1 as libc::c_int);
    }
    _ijkNormalize(h);
}
#[no_mangle]
pub unsafe extern "C" fn _ijkToHex2d(mut h: *const CoordIJK, mut v: *mut Vec2d) {
    let mut i: libc::c_int = (*h).i - (*h).k;
    let mut j: libc::c_int = (*h).j - (*h).k;
    (*v).x = (i as f64 - 0.5 * j as f64).to_f64().unwrap();
    (*v).y = (j as f64 * 0.866_025_403_784_438_6).to_f64().unwrap();
}
#[no_mangle]
pub unsafe extern "C" fn _ijkMatches(
    mut c1: *const CoordIJK,
    mut c2: *const CoordIJK,
) -> libc::c_int {
    ((*c1).i == (*c2).i && (*c1).j == (*c2).j && (*c1).k == (*c2).k) as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn _ijkAdd(
    mut h1: *const CoordIJK,
    mut h2: *const CoordIJK,
    mut sum: *mut CoordIJK,
) {
    (*sum).i = (*h1).i + (*h2).i;
    (*sum).j = (*h1).j + (*h2).j;
    (*sum).k = (*h1).k + (*h2).k;
}
#[no_mangle]
pub unsafe extern "C" fn _ijkSub(
    mut h1: *const CoordIJK,
    mut h2: *const CoordIJK,
    mut diff: *mut CoordIJK,
) {
    (*diff).i = (*h1).i - (*h2).i;
    (*diff).j = (*h1).j - (*h2).j;
    (*diff).k = (*h1).k - (*h2).k;
}
#[no_mangle]
pub unsafe extern "C" fn _ijkScale(mut c: *mut CoordIJK, mut factor: libc::c_int) {
    (*c).i *= factor;
    (*c).j *= factor;
    (*c).k *= factor;
}
#[no_mangle]
pub unsafe extern "C" fn _ijkNormalize(mut c: *mut CoordIJK) {
    if (*c).i < 0 as libc::c_int {
        (*c).j -= (*c).i;
        (*c).k -= (*c).i;
        (*c).i = 0 as libc::c_int;
    }
    if (*c).j < 0 as libc::c_int {
        (*c).i -= (*c).j;
        (*c).k -= (*c).j;
        (*c).j = 0 as libc::c_int;
    }
    if (*c).k < 0 as libc::c_int {
        (*c).i -= (*c).k;
        (*c).j -= (*c).k;
        (*c).k = 0 as libc::c_int;
    }
    let mut min: libc::c_int = (*c).i;
    if (*c).j < min {
        min = (*c).j;
    }
    if (*c).k < min {
        min = (*c).k;
    }
    if min > 0 as libc::c_int {
        (*c).i -= min;
        (*c).j -= min;
        (*c).k -= min;
    }
}
#[no_mangle]
pub unsafe extern "C" fn _unitIjkToDigit(mut ijk: *const CoordIJK) -> Direction {
    let mut c: CoordIJK = *ijk;
    _ijkNormalize(&mut c);
    let mut digit: Direction = INVALID_DIGIT;
    let mut i: Direction = CENTER_DIGIT;
    while (i as libc::c_uint) < NUM_DIGITS as libc::c_int as libc::c_uint {
        if _ijkMatches(&c, &*UNIT_VECS.as_ptr().offset(i as isize)) != 0 {
            digit = i;
            break;
        } else {
            i += 1;
        }
    }
    digit
}
#[no_mangle]
pub unsafe extern "C" fn _upAp7(mut ijk: *mut CoordIJK) {
    let mut i: libc::c_int = (*ijk).i - (*ijk).k;
    let mut j: libc::c_int = (*ijk).j - (*ijk).k;
    (*ijk).i = lroundl((3 as libc::c_int * i - j) as f64 / 7.0) as libc::c_int;
    (*ijk).j = lroundl((i + 2 as libc::c_int * j) as f64 / 7.0) as libc::c_int;
    (*ijk).k = 0 as libc::c_int;
    _ijkNormalize(ijk);
}
#[no_mangle]
pub unsafe extern "C" fn _upAp7r(mut ijk: *mut CoordIJK) {
    let mut i: libc::c_int = (*ijk).i - (*ijk).k;
    let mut j: libc::c_int = (*ijk).j - (*ijk).k;
    (*ijk).i = lroundl((2 as libc::c_int * i + j) as f64 / 7.0) as libc::c_int;
    (*ijk).j = lroundl((3 as libc::c_int * j - i) as f64 / 7.0) as libc::c_int;
    (*ijk).k = 0 as libc::c_int;
    _ijkNormalize(ijk);
}
#[no_mangle]
pub unsafe extern "C" fn _downAp7(mut ijk: *mut CoordIJK) {
    let mut iVec: CoordIJK = {
        CoordIJK {
            i: 3 as libc::c_int,
            j: 0 as libc::c_int,
            k: 1 as libc::c_int,
        }
    };
    let mut jVec: CoordIJK = {
        CoordIJK {
            i: 1 as libc::c_int,
            j: 3 as libc::c_int,
            k: 0 as libc::c_int,
        }
    };
    let mut kVec: CoordIJK = {
        CoordIJK {
            i: 0 as libc::c_int,
            j: 1 as libc::c_int,
            k: 3 as libc::c_int,
        }
    };
    _ijkScale(&mut iVec, (*ijk).i);
    _ijkScale(&mut jVec, (*ijk).j);
    _ijkScale(&mut kVec, (*ijk).k);
    _ijkAdd(&iVec, &jVec, ijk);
    _ijkAdd(ijk, &kVec, ijk);
    _ijkNormalize(ijk);
}
#[no_mangle]
pub unsafe extern "C" fn _downAp7r(mut ijk: *mut CoordIJK) {
    let mut iVec: CoordIJK = {
        CoordIJK {
            i: 3 as libc::c_int,
            j: 1 as libc::c_int,
            k: 0 as libc::c_int,
        }
    };
    let mut jVec: CoordIJK = {
        CoordIJK {
            i: 0 as libc::c_int,
            j: 3 as libc::c_int,
            k: 1 as libc::c_int,
        }
    };
    let mut kVec: CoordIJK = {
        CoordIJK {
            i: 1 as libc::c_int,
            j: 0 as libc::c_int,
            k: 3 as libc::c_int,
        }
    };
    _ijkScale(&mut iVec, (*ijk).i);
    _ijkScale(&mut jVec, (*ijk).j);
    _ijkScale(&mut kVec, (*ijk).k);
    _ijkAdd(&iVec, &jVec, ijk);
    _ijkAdd(ijk, &kVec, ijk);
    _ijkNormalize(ijk);
}
#[no_mangle]
pub unsafe extern "C" fn _neighbor(mut ijk: *mut CoordIJK, mut digit: Direction) {
    if digit as libc::c_uint > CENTER_DIGIT as libc::c_int as libc::c_uint
        && (digit as libc::c_uint) < NUM_DIGITS as libc::c_int as libc::c_uint
    {
        _ijkAdd(ijk, &*UNIT_VECS.as_ptr().offset(digit as isize), ijk);
        _ijkNormalize(ijk);
    }
}
#[no_mangle]
pub unsafe extern "C" fn _ijkRotate60ccw(mut ijk: *mut CoordIJK) {
    let mut iVec: CoordIJK = {
        CoordIJK {
            i: 1 as libc::c_int,
            j: 1 as libc::c_int,
            k: 0 as libc::c_int,
        }
    };
    let mut jVec: CoordIJK = {
        CoordIJK {
            i: 0 as libc::c_int,
            j: 1 as libc::c_int,
            k: 1 as libc::c_int,
        }
    };
    let mut kVec: CoordIJK = {
        CoordIJK {
            i: 1 as libc::c_int,
            j: 0 as libc::c_int,
            k: 1 as libc::c_int,
        }
    };
    _ijkScale(&mut iVec, (*ijk).i);
    _ijkScale(&mut jVec, (*ijk).j);
    _ijkScale(&mut kVec, (*ijk).k);
    _ijkAdd(&iVec, &jVec, ijk);
    _ijkAdd(ijk, &kVec, ijk);
    _ijkNormalize(ijk);
}
#[no_mangle]
pub unsafe extern "C" fn _ijkRotate60cw(mut ijk: *mut CoordIJK) {
    let mut iVec: CoordIJK = {
        CoordIJK {
            i: 1 as libc::c_int,
            j: 0 as libc::c_int,
            k: 1 as libc::c_int,
        }
    };
    let mut jVec: CoordIJK = {
        CoordIJK {
            i: 1 as libc::c_int,
            j: 1 as libc::c_int,
            k: 0 as libc::c_int,
        }
    };
    let mut kVec: CoordIJK = {
        CoordIJK {
            i: 0 as libc::c_int,
            j: 1 as libc::c_int,
            k: 1 as libc::c_int,
        }
    };
    _ijkScale(&mut iVec, (*ijk).i);
    _ijkScale(&mut jVec, (*ijk).j);
    _ijkScale(&mut kVec, (*ijk).k);
    _ijkAdd(&iVec, &jVec, ijk);
    _ijkAdd(ijk, &kVec, ijk);
    _ijkNormalize(ijk);
}
#[no_mangle]
pub unsafe extern "C" fn _rotate60ccw(mut digit: Direction) -> Direction {
    match digit as libc::c_uint {
        1 => IK_AXES_DIGIT,
        5 => I_AXES_DIGIT,
        4 => IJ_AXES_DIGIT,
        6 => J_AXES_DIGIT,
        2 => JK_AXES_DIGIT,
        3 => K_AXES_DIGIT,
        _ => digit,
    }
}
#[no_mangle]
pub unsafe extern "C" fn _rotate60cw(mut digit: Direction) -> Direction {
    match digit as libc::c_uint {
        1 => JK_AXES_DIGIT,
        3 => J_AXES_DIGIT,
        2 => IJ_AXES_DIGIT,
        6 => I_AXES_DIGIT,
        4 => IK_AXES_DIGIT,
        5 => K_AXES_DIGIT,
        _ => digit,
    }
}
#[no_mangle]
pub unsafe extern "C" fn _downAp3(mut ijk: *mut CoordIJK) {
    let mut iVec: CoordIJK = {
        CoordIJK {
            i: 2 as libc::c_int,
            j: 0 as libc::c_int,
            k: 1 as libc::c_int,
        }
    };
    let mut jVec: CoordIJK = {
        CoordIJK {
            i: 1 as libc::c_int,
            j: 2 as libc::c_int,
            k: 0 as libc::c_int,
        }
    };
    let mut kVec: CoordIJK = {
        CoordIJK {
            i: 0 as libc::c_int,
            j: 1 as libc::c_int,
            k: 2 as libc::c_int,
        }
    };
    _ijkScale(&mut iVec, (*ijk).i);
    _ijkScale(&mut jVec, (*ijk).j);
    _ijkScale(&mut kVec, (*ijk).k);
    _ijkAdd(&iVec, &jVec, ijk);
    _ijkAdd(ijk, &kVec, ijk);
    _ijkNormalize(ijk);
}
#[no_mangle]
pub unsafe extern "C" fn _downAp3r(mut ijk: *mut CoordIJK) {
    let mut iVec: CoordIJK = {
        CoordIJK {
            i: 2 as libc::c_int,
            j: 1 as libc::c_int,
            k: 0 as libc::c_int,
        }
    };
    let mut jVec: CoordIJK = {
        CoordIJK {
            i: 0 as libc::c_int,
            j: 2 as libc::c_int,
            k: 1 as libc::c_int,
        }
    };
    let mut kVec: CoordIJK = {
        CoordIJK {
            i: 1 as libc::c_int,
            j: 0 as libc::c_int,
            k: 2 as libc::c_int,
        }
    };
    _ijkScale(&mut iVec, (*ijk).i);
    _ijkScale(&mut jVec, (*ijk).j);
    _ijkScale(&mut kVec, (*ijk).k);
    _ijkAdd(&iVec, &jVec, ijk);
    _ijkAdd(ijk, &kVec, ijk);
    _ijkNormalize(ijk);
}
#[no_mangle]
pub unsafe extern "C" fn ijkDistance(
    mut c1: *const CoordIJK,
    mut c2: *const CoordIJK,
) -> libc::c_int {
    let mut diff: CoordIJK = CoordIJK { i: 0, j: 0, k: 0 };
    _ijkSub(c1, c2, &mut diff);
    _ijkNormalize(&mut diff);
    let mut absDiff: CoordIJK = {
        CoordIJK {
            i: abs(diff.i),
            j: abs(diff.j),
            k: abs(diff.k),
        }
    };
    if absDiff.i
        > (if absDiff.j > absDiff.k {
            absDiff.j
        } else {
            absDiff.k
        })
    {
        absDiff.i
    } else if absDiff.j > absDiff.k {
        absDiff.j
    } else {
        absDiff.k
    }
}
#[no_mangle]
pub unsafe extern "C" fn ijkToIj(mut ijk: *const CoordIJK, mut ij: *mut CoordIJ) {
    (*ij).i = (*ijk).i - (*ijk).k;
    (*ij).j = (*ijk).j - (*ijk).k;
}
#[no_mangle]
pub unsafe extern "C" fn ijToIjk(mut ij: *const CoordIJ, mut ijk: *mut CoordIJK) -> H3Error {
    (*ijk).i = (*ij).i;
    (*ijk).j = (*ij).j;
    (*ijk).k = 0 as libc::c_int;
    let mut max: libc::c_int = 0;
    let mut min: libc::c_int = 0;
    if (*ijk).i > (*ijk).j {
        max = (*ijk).i;
        min = (*ijk).j;
    } else {
        max = (*ijk).j;
        min = (*ijk).i;
    }
    if min < 0 as libc::c_int {
        if max < -(2147483647 as libc::c_int) - 1 as libc::c_int - min {
            return E_FAILED as libc::c_int as H3Error;
        }
        if min == -(2147483647 as libc::c_int) - 1 as libc::c_int {
            return E_FAILED as libc::c_int as H3Error;
        }
    }
    _ijkNormalize(ijk);
    E_SUCCESS as libc::c_int as H3Error
}
#[no_mangle]
pub unsafe extern "C" fn ijkToCube(mut ijk: *mut CoordIJK) {
    (*ijk).i = -(*ijk).i + (*ijk).k;
    (*ijk).j -= (*ijk).k;
    (*ijk).k = -(*ijk).i - (*ijk).j;
}
#[no_mangle]
pub unsafe extern "C" fn cubeToIjk(mut ijk: *mut CoordIJK) {
    (*ijk).i = -(*ijk).i;
    (*ijk).k = 0 as libc::c_int;
    _ijkNormalize(ijk);
}
