use ::libc;
extern "C" {
    fn __assert_rtn(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
        _: *const libc::c_char,
    ) -> !;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn round(_: libc::c_double) -> libc::c_double;
    fn ijkDistance(a: *const CoordIJK, b: *const CoordIJK) -> libc::c_int;
    fn _ijkRotate60cw(ijk: *mut CoordIJK);
    fn _isBaseCellPentagon(baseCell: libc::c_int) -> libc::c_int;
    fn _ijkNormalize(c: *mut CoordIJK);
    fn _ijkAdd(h1: *const CoordIJK, h2: *const CoordIJK, sum: *mut CoordIJK);
    fn _downAp7r(ijk: *mut CoordIJK);
    fn _downAp7(ijk: *mut CoordIJK);
    fn _neighbor(ijk: *mut CoordIJK, digit: Direction);
    fn _rotate60cw(digit: Direction) -> Direction;
    static baseCellNeighbor60CCWRots: [[libc::c_int; 7]; 122];
    fn _getBaseCellDirection(
        originBaseCell: libc::c_int,
        destinationBaseCell: libc::c_int,
    ) -> Direction;
    fn _unitIjkToDigit(ijk: *const CoordIJK) -> Direction;
    fn _getBaseCellNeighbor(baseCell: libc::c_int, dir: Direction) -> libc::c_int;
    fn _isBaseCellPolarPentagon(baseCell: libc::c_int) -> bool;
    fn _rotate60ccw(digit: Direction) -> Direction;
    fn _ijkSub(h1: *const CoordIJK, h2: *const CoordIJK, diff: *mut CoordIJK);
    fn _upAp7r(ijk: *mut CoordIJK);
    fn _upAp7(ijk: *mut CoordIJK);
    fn cubeToIjk(ijk: *mut CoordIJK);
    fn ijkToCube(ijk: *mut CoordIJK);
    fn ijkToIj(ijk: *const CoordIJK, ij: *mut CoordIJ);
    fn ijToIjk(ij: *const CoordIJ, ijk: *mut CoordIJK) -> H3Error;
    fn isResolutionClassIII(r: libc::c_int) -> libc::c_int;
    fn _h3ToFaceIjkWithInitializedFijk(h: H3Index, fijk: *mut FaceIJK) -> libc::c_int;
    fn _h3LeadingNonZeroDigit(h: H3Index) -> Direction;
    fn _h3RotatePent60ccw(h: H3Index) -> H3Index;
    fn _h3RotatePent60cw(h: H3Index) -> H3Index;
    fn _h3Rotate60ccw(h: H3Index) -> H3Index;
    fn _h3Rotate60cw(h: H3Index) -> H3Index;
}
pub type int64_t = libc::c_longlong;
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
pub type H3Index = uint64_t;
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
pub struct CoordIJK {
    pub i: libc::c_int,
    pub j: libc::c_int,
    pub k: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FaceIJK {
    pub face: libc::c_int,
    pub coord: CoordIJK,
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
#[no_mangle]
pub static mut PENTAGON_ROTATIONS: [[libc::c_int; 7]; 7] = [
    [
        0 as libc::c_int,
        -(1 as libc::c_int),
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
    ],
    [
        0 as libc::c_int,
        -(1 as libc::c_int),
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        -(1 as libc::c_int),
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        -(1 as libc::c_int),
        0 as libc::c_int,
        5 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        -(1 as libc::c_int),
        5 as libc::c_int,
        5 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        -(1 as libc::c_int),
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ],
];
#[no_mangle]
pub static mut PENTAGON_ROTATIONS_REVERSE: [[libc::c_int; 7]; 7] = [
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
    ],
    [
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        5 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        5 as libc::c_int,
        0 as libc::c_int,
        5 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ],
];
#[no_mangle]
pub static mut PENTAGON_ROTATIONS_REVERSE_NONPOLAR: [[libc::c_int; 7]; 7] = [
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
    ],
    [
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        5 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        5 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ],
];
#[no_mangle]
pub static mut PENTAGON_ROTATIONS_REVERSE_POLAR: [[libc::c_int; 7]; 7] = [
    [
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
        -(1 as libc::c_int),
    ],
    [
        0 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        5 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
    ],
    [
        0 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
        1 as libc::c_int,
    ],
];
#[no_mangle]
pub static mut FAILED_DIRECTIONS: [[bool; 7]; 7] = [
    [
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
    ],
    [
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
    ],
    [
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
        1 as libc::c_int != 0,
        1 as libc::c_int != 0,
        0 as libc::c_int != 0,
    ],
    [
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
        1 as libc::c_int != 0,
        0 as libc::c_int != 0,
        1 as libc::c_int != 0,
    ],
    [
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
        1 as libc::c_int != 0,
        1 as libc::c_int != 0,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
    ],
    [
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
        1 as libc::c_int != 0,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
        1 as libc::c_int != 0,
    ],
    [
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
        0 as libc::c_int != 0,
        1 as libc::c_int != 0,
        0 as libc::c_int != 0,
        1 as libc::c_int != 0,
        0 as libc::c_int != 0,
    ],
];
#[no_mangle]
pub unsafe extern "C" fn cellToLocalIjk(
    mut origin: H3Index,
    mut h3: H3Index,
    mut out: *mut CoordIJK,
) -> H3Error {
    let mut res: libc::c_int = ((origin & (15 as libc::c_ulonglong) << 52 as libc::c_int)
        >> 52 as libc::c_int) as libc::c_int;
    if res
        != ((h3 & (15 as libc::c_ulonglong) << 52 as libc::c_int) >> 52 as libc::c_int)
            as libc::c_int
    {
        return E_RES_MISMATCH as libc::c_int as H3Error;
    }
    let mut originBaseCell: libc::c_int = ((origin
        & (127 as libc::c_int as uint64_t) << 45 as libc::c_int)
        >> 45 as libc::c_int) as libc::c_int;
    let mut baseCell: libc::c_int = ((h3 & (127 as libc::c_int as uint64_t) << 45 as libc::c_int)
        >> 45 as libc::c_int) as libc::c_int;
    if originBaseCell < 0 as libc::c_int || originBaseCell >= 122 as libc::c_int {
        return E_CELL_INVALID as libc::c_int as H3Error;
    }
    if baseCell < 0 as libc::c_int || baseCell >= 122 as libc::c_int {
        return E_CELL_INVALID as libc::c_int as H3Error;
    }
    let mut dir: Direction = CENTER_DIGIT;
    let mut revDir: Direction = CENTER_DIGIT;
    if originBaseCell != baseCell {
        dir = _getBaseCellDirection(originBaseCell, baseCell);
        if dir as libc::c_uint == INVALID_DIGIT as libc::c_int as libc::c_uint {
            return E_FAILED as libc::c_int as H3Error;
        }
        revDir = _getBaseCellDirection(baseCell, originBaseCell);
        if i64::from(revDir as libc::c_uint) == INVALID_DIGIT as i64 {
            __assert_rtn(
                (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"cellToLocalIjk\0"))
                    .as_ptr(),
                b"localij.c\0" as *const u8 as *const libc::c_char,
                161 as libc::c_int,
                b"revDir != INVALID_DIGIT\0" as *const u8 as *const libc::c_char,
            );
        } else {
        };
    }
    let mut originOnPent: libc::c_int = _isBaseCellPentagon(originBaseCell);
    let mut indexOnPent: libc::c_int = _isBaseCellPentagon(baseCell);
    let mut indexFijk: FaceIJK = {
        FaceIJK {
            face: 0 as libc::c_int,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        }
    };
    if dir as libc::c_uint != CENTER_DIGIT as libc::c_int as libc::c_uint {
        let mut baseCellRotations: libc::c_int =
            baseCellNeighbor60CCWRots[originBaseCell as usize][dir as usize];
        if indexOnPent != 0 {
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < baseCellRotations {
                h3 = _h3RotatePent60cw(h3);
                revDir = _rotate60cw(revDir);
                if revDir as libc::c_uint == K_AXES_DIGIT as libc::c_int as libc::c_uint {
                    revDir = _rotate60cw(revDir);
                }
                i += 1;
            }
        } else {
            let mut i_0: libc::c_int = 0 as libc::c_int;
            while i_0 < baseCellRotations {
                h3 = _h3Rotate60cw(h3);
                revDir = _rotate60cw(revDir);
                i_0 += 1;
            }
        }
    }
    _h3ToFaceIjkWithInitializedFijk(h3, &mut indexFijk);
    if dir as libc::c_uint != CENTER_DIGIT as libc::c_int as libc::c_uint {
        if i64::from(baseCell) == originBaseCell as i64 {
            __assert_rtn(
                (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"cellToLocalIjk\0"))
                    .as_ptr(),
                b"localij.c\0" as *const u8 as *const libc::c_char,
                191 as libc::c_int,
                b"baseCell != originBaseCell\0" as *const u8 as *const libc::c_char,
            );
        } else {
        };
        if (originOnPent != 0 && indexOnPent != 0) as libc::c_int as libc::c_long != 0 {
            __assert_rtn(
                (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"cellToLocalIjk\0"))
                    .as_ptr(),
                b"localij.c\0" as *const u8 as *const libc::c_char,
                192 as libc::c_int,
                b"!(originOnPent && indexOnPent)\0" as *const u8 as *const libc::c_char,
            );
        } else {
        };
        let mut pentagonRotations: libc::c_int = 0 as libc::c_int;
        let mut directionRotations: libc::c_int = 0 as libc::c_int;
        if originOnPent != 0 {
            let mut originLeadingDigit: libc::c_int = _h3LeadingNonZeroDigit(origin) as libc::c_int;
            if originLeadingDigit == INVALID_DIGIT as libc::c_int {
                return E_CELL_INVALID as libc::c_int as H3Error;
            }
            if FAILED_DIRECTIONS[originLeadingDigit as usize][dir as usize] {
                return E_FAILED as libc::c_int as H3Error;
            }
            directionRotations = PENTAGON_ROTATIONS[originLeadingDigit as usize][dir as usize];
            pentagonRotations = directionRotations;
        } else if indexOnPent != 0 {
            let mut indexLeadingDigit: libc::c_int = _h3LeadingNonZeroDigit(h3) as libc::c_int;
            if indexLeadingDigit == INVALID_DIGIT as libc::c_int {
                return E_CELL_INVALID as libc::c_int as H3Error;
            }
            if FAILED_DIRECTIONS[indexLeadingDigit as usize][revDir as usize] {
                return E_FAILED as libc::c_int as H3Error;
            }
            pentagonRotations = PENTAGON_ROTATIONS[revDir as usize][indexLeadingDigit as usize];
        }
        if pentagonRotations < 0 as libc::c_int || directionRotations < 0 as libc::c_int {
            return E_CELL_INVALID as libc::c_int as H3Error;
        }
        let mut i_1: libc::c_int = 0 as libc::c_int;
        while i_1 < pentagonRotations {
            _ijkRotate60cw(&mut indexFijk.coord);
            i_1 += 1;
        }
        let mut offset: CoordIJK = {
            CoordIJK {
                i: 0 as libc::c_int,
                j: 0,
                k: 0,
            }
        };
        _neighbor(&mut offset, dir);
        let mut r: libc::c_int = res - 1 as libc::c_int;
        while r >= 0 as libc::c_int {
            if isResolutionClassIII(r + 1 as libc::c_int) != 0 {
                _downAp7(&mut offset);
            } else {
                _downAp7r(&mut offset);
            }
            r -= 1;
        }
        let mut i_2: libc::c_int = 0 as libc::c_int;
        while i_2 < directionRotations {
            _ijkRotate60cw(&mut offset);
            i_2 += 1;
        }
        _ijkAdd(&indexFijk.coord, &offset, &mut indexFijk.coord);
        _ijkNormalize(&mut indexFijk.coord);
    } else if originOnPent != 0 && indexOnPent != 0 {
        if i64::from(baseCell) != originBaseCell as i64 {
            __assert_rtn(
                (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"cellToLocalIjk\0"))
                    .as_ptr(),
                b"localij.c\0" as *const u8 as *const libc::c_char,
                261 as libc::c_int,
                b"baseCell == originBaseCell\0" as *const u8 as *const libc::c_char,
            );
        } else {
        };
        let mut originLeadingDigit_0: libc::c_int = _h3LeadingNonZeroDigit(origin) as libc::c_int;
        let mut indexLeadingDigit_0: libc::c_int = _h3LeadingNonZeroDigit(h3) as libc::c_int;
        if originLeadingDigit_0 == INVALID_DIGIT as libc::c_int
            || indexLeadingDigit_0 == INVALID_DIGIT as libc::c_int
        {
            return E_CELL_INVALID as libc::c_int as H3Error;
        }
        if FAILED_DIRECTIONS[originLeadingDigit_0 as usize][indexLeadingDigit_0 as usize] {
            return E_FAILED as libc::c_int as H3Error;
        }
        let mut withinPentagonRotations: libc::c_int =
            PENTAGON_ROTATIONS[originLeadingDigit_0 as usize][indexLeadingDigit_0 as usize];
        let mut i_3: libc::c_int = 0 as libc::c_int;
        while i_3 < withinPentagonRotations {
            _ijkRotate60cw(&mut indexFijk.coord);
            i_3 += 1;
        }
    }
    *out = indexFijk.coord;
    E_SUCCESS as libc::c_int as H3Error
}
#[no_mangle]
pub unsafe extern "C" fn localIjkToCell(
    mut origin: H3Index,
    mut ijk: *const CoordIJK,
    mut out: *mut H3Index,
) -> H3Error {
    let mut res: libc::c_int = ((origin & (15 as libc::c_ulonglong) << 52 as libc::c_int)
        >> 52 as libc::c_int) as libc::c_int;
    let mut originBaseCell: libc::c_int = ((origin
        & (127 as libc::c_int as uint64_t) << 45 as libc::c_int)
        >> 45 as libc::c_int) as libc::c_int;
    if originBaseCell < 0 as libc::c_int || originBaseCell >= 122 as libc::c_int {
        return E_CELL_INVALID as libc::c_int as H3Error;
    }
    let mut originOnPent: libc::c_int = _isBaseCellPentagon(originBaseCell);
    *out = 35184372088831 as libc::c_ulonglong;
    *out = *out & !((15 as libc::c_int as uint64_t) << 59 as libc::c_int)
        | (1 as libc::c_int as uint64_t) << 59 as libc::c_int;
    *out = *out & !((15 as libc::c_ulonglong) << 52 as libc::c_int)
        | (res as uint64_t) << 52 as libc::c_int;
    if res == 0 as libc::c_int {
        let dir: Direction = _unitIjkToDigit(ijk);
        if dir as libc::c_uint == INVALID_DIGIT as libc::c_int as libc::c_uint {
            return E_FAILED as libc::c_int as H3Error;
        }
        let newBaseCell: libc::c_int = _getBaseCellNeighbor(originBaseCell, dir);
        if newBaseCell == 127 as libc::c_int {
            return E_FAILED as libc::c_int as H3Error;
        }
        *out = *out & !((127 as libc::c_int as uint64_t) << 45 as libc::c_int)
            | (newBaseCell as uint64_t) << 45 as libc::c_int;
        return E_SUCCESS as libc::c_int as H3Error;
    }
    let mut ijkCopy: CoordIJK = *ijk;
    let mut r: libc::c_int = res - 1 as libc::c_int;
    while r >= 0 as libc::c_int {
        let mut lastIJK: CoordIJK = ijkCopy;
        let mut lastCenter: CoordIJK = CoordIJK { i: 0, j: 0, k: 0 };
        if isResolutionClassIII(r + 1 as libc::c_int) != 0 {
            _upAp7(&mut ijkCopy);
            lastCenter = ijkCopy;
            _downAp7(&mut lastCenter);
        } else {
            _upAp7r(&mut ijkCopy);
            lastCenter = ijkCopy;
            _downAp7r(&mut lastCenter);
        }
        let mut diff: CoordIJK = CoordIJK { i: 0, j: 0, k: 0 };
        _ijkSub(&lastIJK, &lastCenter, &mut diff);
        _ijkNormalize(&mut diff);
        *out = *out
            & !((7 as libc::c_int as uint64_t)
                << ((15 as libc::c_int - (r + 1 as libc::c_int)) * 3 as libc::c_int))
            | (_unitIjkToDigit(&diff) as uint64_t)
                << ((15 as libc::c_int - (r + 1 as libc::c_int)) * 3 as libc::c_int);
        r -= 1;
    }
    if ijkCopy.i > 1 as libc::c_int || ijkCopy.j > 1 as libc::c_int || ijkCopy.k > 1 as libc::c_int
    {
        return E_FAILED as libc::c_int as H3Error;
    }
    let mut dir_0: Direction = _unitIjkToDigit(&ijkCopy);
    let mut baseCell: libc::c_int = _getBaseCellNeighbor(originBaseCell, dir_0);
    let mut indexOnPent: libc::c_int = if baseCell == 127 as libc::c_int {
        0 as libc::c_int
    } else {
        _isBaseCellPentagon(baseCell)
    };
    if dir_0 as libc::c_uint != CENTER_DIGIT as libc::c_int as libc::c_uint {
        let mut pentagonRotations: libc::c_int = 0 as libc::c_int;
        if originOnPent != 0 {
            let originLeadingDigit: Direction = _h3LeadingNonZeroDigit(origin);
            if originLeadingDigit as libc::c_uint == INVALID_DIGIT as libc::c_int as libc::c_uint {
                return E_CELL_INVALID as libc::c_int as H3Error;
            }
            pentagonRotations =
                PENTAGON_ROTATIONS_REVERSE[originLeadingDigit as usize][dir_0 as usize];
            let mut i: libc::c_int = 0 as libc::c_int;
            while i < pentagonRotations {
                dir_0 = _rotate60ccw(dir_0);
                i += 1;
            }
            if dir_0 as libc::c_uint == K_AXES_DIGIT as libc::c_int as libc::c_uint {
                return E_PENTAGON as libc::c_int as H3Error;
            }
            baseCell = _getBaseCellNeighbor(originBaseCell, dir_0);
            if i64::from(baseCell) == 127_i64 {
                __assert_rtn(
                    (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"localIjkToCell\0",
                    ))
                    .as_ptr(),
                    b"localij.c\0" as *const u8 as *const libc::c_char,
                    406 as libc::c_int,
                    b"baseCell != INVALID_BASE_CELL\0" as *const u8 as *const libc::c_char,
                );
            } else {
            };
            if (_isBaseCellPentagon(baseCell) != 0) as libc::c_int as libc::c_long != 0 {
                __assert_rtn(
                    (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"localIjkToCell\0",
                    ))
                    .as_ptr(),
                    b"localij.c\0" as *const u8 as *const libc::c_char,
                    407 as libc::c_int,
                    b"!_isBaseCellPentagon(baseCell)\0" as *const u8 as *const libc::c_char,
                );
            } else {
            };
        }
        let baseCellRotations: libc::c_int =
            baseCellNeighbor60CCWRots[originBaseCell as usize][dir_0 as usize];
        if i64::from(baseCellRotations) < 0 {
            __assert_rtn(
                (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"localIjkToCell\0"))
                    .as_ptr(),
                b"localij.c\0" as *const u8 as *const libc::c_char,
                414 as libc::c_int,
                b"baseCellRotations >= 0\0" as *const u8 as *const libc::c_char,
            );
        } else {
        };
        if indexOnPent != 0 {
            let revDir: Direction = _getBaseCellDirection(baseCell, originBaseCell);
            if i64::from(revDir as libc::c_uint) == INVALID_DIGIT as i64 {
                __assert_rtn(
                    (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(
                        b"localIjkToCell\0",
                    ))
                    .as_ptr(),
                    b"localij.c\0" as *const u8 as *const libc::c_char,
                    423 as libc::c_int,
                    b"revDir != INVALID_DIGIT\0" as *const u8 as *const libc::c_char,
                );
            } else {
            };
            let mut i_0: libc::c_int = 0 as libc::c_int;
            while i_0 < baseCellRotations {
                *out = _h3Rotate60ccw(*out);
                i_0 += 1;
            }
            let indexLeadingDigit: Direction = _h3LeadingNonZeroDigit(*out);
            if indexLeadingDigit as libc::c_uint == INVALID_DIGIT as libc::c_int as libc::c_uint {
                return E_CELL_INVALID as libc::c_int as H3Error;
            }
            if _isBaseCellPolarPentagon(baseCell) {
                pentagonRotations =
                    PENTAGON_ROTATIONS_REVERSE_POLAR[revDir as usize][indexLeadingDigit as usize];
            } else {
                pentagonRotations = PENTAGON_ROTATIONS_REVERSE_NONPOLAR[revDir as usize]
                    [indexLeadingDigit as usize];
            }
            if pentagonRotations < 0 as libc::c_int {
                return E_CELL_INVALID as libc::c_int as H3Error;
            }
            let mut i_1: libc::c_int = 0 as libc::c_int;
            while i_1 < pentagonRotations {
                *out = _h3RotatePent60ccw(*out);
                i_1 += 1;
            }
        } else {
            if pentagonRotations < 0 as libc::c_int {
                return E_CELL_INVALID as libc::c_int as H3Error;
            }
            let mut i_2: libc::c_int = 0 as libc::c_int;
            while i_2 < pentagonRotations {
                *out = _h3Rotate60ccw(*out);
                i_2 += 1;
            }
            let mut i_3: libc::c_int = 0 as libc::c_int;
            while i_3 < baseCellRotations {
                *out = _h3Rotate60ccw(*out);
                i_3 += 1;
            }
        }
    } else if originOnPent != 0 && indexOnPent != 0 {
        let originLeadingDigit_0: libc::c_int = _h3LeadingNonZeroDigit(origin) as libc::c_int;
        let indexLeadingDigit_0: libc::c_int = _h3LeadingNonZeroDigit(*out) as libc::c_int;
        if originLeadingDigit_0 == INVALID_DIGIT as libc::c_int
            || indexLeadingDigit_0 == INVALID_DIGIT as libc::c_int
        {
            return E_CELL_INVALID as libc::c_int as H3Error;
        }
        let withinPentagonRotations: libc::c_int =
            PENTAGON_ROTATIONS_REVERSE[originLeadingDigit_0 as usize][indexLeadingDigit_0 as usize];
        if withinPentagonRotations < 0 as libc::c_int {
            return E_CELL_INVALID as libc::c_int as H3Error;
        }
        let mut i_4: libc::c_int = 0 as libc::c_int;
        while i_4 < withinPentagonRotations {
            *out = _h3Rotate60ccw(*out);
            i_4 += 1;
        }
    }
    if indexOnPent != 0
        && _h3LeadingNonZeroDigit(*out) as libc::c_uint
            == K_AXES_DIGIT as libc::c_int as libc::c_uint
    {
        return E_PENTAGON as libc::c_int as H3Error;
    }
    *out = *out & !((127 as libc::c_int as uint64_t) << 45 as libc::c_int)
        | (baseCell as uint64_t) << 45 as libc::c_int;
    E_SUCCESS as libc::c_int as H3Error
}
#[no_mangle]
pub unsafe extern "C" fn cellToLocalIj(
    mut origin: H3Index,
    mut h3: H3Index,
    mut mode: uint32_t,
    mut out: *mut CoordIJ,
) -> H3Error {
    if mode != 0 as libc::c_int as libc::c_uint {
        return E_OPTION_INVALID as libc::c_int as H3Error;
    }
    let mut ijk: CoordIJK = CoordIJK { i: 0, j: 0, k: 0 };
    let mut failed: H3Error = cellToLocalIjk(origin, h3, &mut ijk);
    if failed != 0 {
        return failed;
    }
    ijkToIj(&ijk, out);
    E_SUCCESS as libc::c_int as H3Error
}
#[no_mangle]
pub unsafe extern "C" fn localIjToCell(
    mut origin: H3Index,
    mut ij: *const CoordIJ,
    mut mode: uint32_t,
    mut out: *mut H3Index,
) -> H3Error {
    if mode != 0 as libc::c_int as libc::c_uint {
        return E_OPTION_INVALID as libc::c_int as H3Error;
    }
    let mut ijk: CoordIJK = CoordIJK { i: 0, j: 0, k: 0 };
    let mut ijToIjkError: H3Error = ijToIjk(ij, &mut ijk);
    if ijToIjkError != 0 {
        return ijToIjkError;
    }
    localIjkToCell(origin, &ijk, out)
}
#[no_mangle]
pub unsafe extern "C" fn gridDistance(
    mut origin: H3Index,
    mut h3: H3Index,
    mut out: *mut int64_t,
) -> H3Error {
    let mut originIjk: CoordIJK = CoordIJK { i: 0, j: 0, k: 0 };
    let mut h3Ijk: CoordIJK = CoordIJK { i: 0, j: 0, k: 0 };
    let mut originError: H3Error = cellToLocalIjk(origin, origin, &mut originIjk);
    if originError != 0 {
        return originError;
    }
    let mut destError: H3Error = cellToLocalIjk(origin, h3, &mut h3Ijk);
    if destError != 0 {
        return destError;
    }
    *out = ijkDistance(&originIjk, &h3Ijk) as int64_t;
    E_SUCCESS as libc::c_int as H3Error
}
#[no_mangle]
pub unsafe extern "C" fn gridPathCellsSize(
    mut start: H3Index,
    mut end: H3Index,
    mut size: *mut int64_t,
) -> H3Error {
    let mut distance: int64_t = 0;
    let mut distanceError: H3Error = gridDistance(start, end, &mut distance);
    if distanceError != 0 {
        return distanceError;
    }
    *size = distance + 1 as libc::c_int as libc::c_longlong;
    E_SUCCESS as libc::c_int as H3Error
}
unsafe extern "C" fn cubeRound(
    mut i: libc::c_double,
    mut j: libc::c_double,
    mut k: libc::c_double,
    mut ijk: *mut CoordIJK,
) {
    let mut ri: libc::c_int = round(i) as libc::c_int;
    let mut rj: libc::c_int = round(j) as libc::c_int;
    let mut rk: libc::c_int = round(k) as libc::c_int;
    let mut iDiff: libc::c_double = fabs(ri as libc::c_double - i);
    let mut jDiff: libc::c_double = fabs(rj as libc::c_double - j);
    let mut kDiff: libc::c_double = fabs(rk as libc::c_double - k);
    if iDiff > jDiff && iDiff > kDiff {
        ri = -rj - rk;
    } else if jDiff > kDiff {
        rj = -ri - rk;
    } else {
        rk = -ri - rj;
    }
    (*ijk).i = ri;
    (*ijk).j = rj;
    (*ijk).k = rk;
}
#[no_mangle]
pub unsafe extern "C" fn gridPathCells(
    mut start: H3Index,
    mut end: H3Index,
    mut out: *mut H3Index,
) -> H3Error {
    let mut distance: int64_t = 0;
    let mut distanceError: H3Error = gridDistance(start, end, &mut distance);
    if distanceError != 0 {
        return distanceError;
    }
    let mut startIjk: CoordIJK = {
        CoordIJK {
            i: 0 as libc::c_int,
            j: 0,
            k: 0,
        }
    };
    let mut endIjk: CoordIJK = {
        CoordIJK {
            i: 0 as libc::c_int,
            j: 0,
            k: 0,
        }
    };
    let mut startError: H3Error = cellToLocalIjk(start, start, &mut startIjk);
    if startError != 0 {
        return startError;
    }
    let mut endError: H3Error = cellToLocalIjk(start, end, &mut endIjk);
    if endError != 0 {
        return endError;
    }
    ijkToCube(&mut startIjk);
    ijkToCube(&mut endIjk);
    let mut iStep: libc::c_double = if distance != 0 {
        (endIjk.i - startIjk.i) as libc::c_double / distance as libc::c_double
    } else {
        0 as libc::c_int as libc::c_double
    };
    let mut jStep: libc::c_double = if distance != 0 {
        (endIjk.j - startIjk.j) as libc::c_double / distance as libc::c_double
    } else {
        0 as libc::c_int as libc::c_double
    };
    let mut kStep: libc::c_double = if distance != 0 {
        (endIjk.k - startIjk.k) as libc::c_double / distance as libc::c_double
    } else {
        0 as libc::c_int as libc::c_double
    };
    let mut currentIjk: CoordIJK = {
        CoordIJK {
            i: startIjk.i,
            j: startIjk.j,
            k: startIjk.k,
        }
    };
    let mut n: int64_t = 0 as libc::c_int as int64_t;
    while n <= distance {
        cubeRound(
            startIjk.i as libc::c_double + iStep * n as libc::c_double,
            startIjk.j as libc::c_double + jStep * n as libc::c_double,
            startIjk.k as libc::c_double + kStep * n as libc::c_double,
            &mut currentIjk,
        );
        cubeToIjk(&mut currentIjk);
        let mut currentError: H3Error =
            localIjkToCell(start, &currentIjk, &mut *out.offset(n as isize));
        if currentError != 0 {
            return currentError;
        }
        n += 1;
    }
    E_SUCCESS as libc::c_int as H3Error
}
