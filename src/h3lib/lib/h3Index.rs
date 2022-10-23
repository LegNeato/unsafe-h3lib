use ::libc;
extern "C" {
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn _rotate60ccw(digit: Direction) -> Direction;
    fn _rotate60cw(digit: Direction) -> Direction;
    fn _unitIjkToDigit(ijk: *const CoordIJK) -> Direction;
    fn _ijkNormalize(c: *mut CoordIJK);
    fn _ijkSub(h1: *const CoordIJK, h2: *const CoordIJK, diff: *mut CoordIJK);
    fn _downAp7r(ijk: *mut CoordIJK);
    fn _upAp7r(ijk: *mut CoordIJK);
    fn _downAp7(ijk: *mut CoordIJK);
    fn _upAp7(ijk: *mut CoordIJK);
    fn _neighbor(ijk: *mut CoordIJK, digit: Direction);
    fn _geoToFaceIjk(g: *const LatLng, res: libc::c_int, h: *mut FaceIJK);
    fn _faceIjkToGeo(h: *const FaceIJK, res: libc::c_int, g: *mut LatLng);
    fn _faceIjkToCellBoundary(
        h: *const FaceIJK,
        res: libc::c_int,
        start: libc::c_int,
        length: libc::c_int,
        g: *mut CellBoundary,
    );
    fn _faceIjkPentToCellBoundary(
        h: *const FaceIJK,
        res: libc::c_int,
        start: libc::c_int,
        length: libc::c_int,
        g: *mut CellBoundary,
    );
    fn _faceIjkToVerts(fijk: *mut FaceIJK, res: *mut libc::c_int, fijkVerts: *mut FaceIJK);
    fn _faceIjkPentToVerts(fijk: *mut FaceIJK, res: *mut libc::c_int, fijkVerts: *mut FaceIJK);
    fn _adjustOverageClassII(
        fijk: *mut FaceIJK,
        res: libc::c_int,
        pentLeading4: libc::c_int,
        substrate: libc::c_int,
    ) -> Overage;
    fn _adjustPentVertOverage(fijk: *mut FaceIJK, res: libc::c_int) -> Overage;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn test_prefix_malloc(size: size_t) -> *mut libc::c_void;
    fn test_prefix_calloc(num: size_t, size: size_t) -> *mut libc::c_void;
    fn test_prefix_free(ptr: *mut libc::c_void);
    static baseCellData: [BaseCellData; 122];
    fn _isBaseCellPentagon(baseCell: libc::c_int) -> libc::c_int;
    fn _faceIjkToBaseCell(h: *const FaceIJK) -> libc::c_int;
    fn _faceIjkToBaseCellCCWrot60(h: *const FaceIJK) -> libc::c_int;
    fn _baseCellIsCwOffset(baseCell: libc::c_int, testFace: libc::c_int) -> bool;
    fn iterInitParent(h: H3Index, childRes: libc::c_int) -> IterCellsChildren;
    fn iterStepChild(iter: *mut IterCellsChildren);
    fn _ipow(base: int64_t, exp: int64_t) -> int64_t;
}
pub type int64_t = libc::c_longlong;
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
pub type __darwin_size_t = libc::c_ulong;
pub type size_t = __darwin_size_t;
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
pub struct LatLng {
    pub lat: libc::c_double,
    pub lng: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CellBoundary {
    pub numVerts: libc::c_int,
    pub verts: [LatLng; 10],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FaceIJK {
    pub face: libc::c_int,
    pub coord: CoordIJK,
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
pub const NO_OVERAGE: Overage = 0;
pub type Overage = libc::c_uint;
pub const NEW_FACE: Overage = 2;
pub const FACE_EDGE: Overage = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BaseCellData {
    pub homeFijk: FaceIJK,
    pub isPentagon: libc::c_int,
    pub cwOffsetPent: [libc::c_int; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IterCellsChildren {
    pub h: H3Index,
    pub _parentRes: libc::c_int,
    pub _skipDigit: libc::c_int,
}
#[inline(always)]
unsafe extern "C" fn __inline_isfinitef(mut __x: libc::c_float) -> libc::c_int {
    (__x.abs() != ::core::f32::INFINITY) as libc::c_int
}
#[inline(always)]
unsafe extern "C" fn __inline_isfinited(mut __x: libc::c_double) -> libc::c_int {
    (__x.abs() != ::core::f64::INFINITY) as libc::c_int
}
#[inline(always)]
unsafe extern "C" fn __inline_isfinitel(mut __x: f64) -> libc::c_int {
    (__x.abs() != ::core::f64::INFINITY) as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn getResolution(mut h: H3Index) -> libc::c_int {
    ((h & (15 as libc::c_ulonglong) << 52 as libc::c_int) >> 52 as libc::c_int) as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn getBaseCellNumber(mut h: H3Index) -> libc::c_int {
    ((h & (127 as libc::c_int as uint64_t) << 45 as libc::c_int) >> 45 as libc::c_int)
        as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn stringToH3(
    mut str: *const libc::c_char,
    mut out: *mut H3Index,
) -> H3Error {
    let mut h: H3Index = 0 as libc::c_int as H3Index;
    let mut read: libc::c_int = sscanf(
        str,
        b"%llx\0" as *const u8 as *const libc::c_char,
        &mut h as *mut H3Index,
    );
    if read != 1 as libc::c_int {
        return E_FAILED as libc::c_int as H3Error;
    }
    *out = h;
    E_SUCCESS as libc::c_int as H3Error
}
#[no_mangle]
pub unsafe extern "C" fn h3ToString(
    mut h: H3Index,
    mut str: *mut libc::c_char,
    mut sz: size_t,
) -> H3Error {
    if sz < 17 as libc::c_int as libc::c_ulong {
        return E_MEMORY_BOUNDS as libc::c_int as H3Error;
    }
    sprintf(str, b"%llx\0" as *const u8 as *const libc::c_char, h);
    E_SUCCESS as libc::c_int as H3Error
}
#[no_mangle]
pub unsafe extern "C" fn isValidCell(mut h: H3Index) -> libc::c_int {
    if ((h & (1 as libc::c_int as uint64_t) << 63 as libc::c_int) >> 63 as libc::c_int)
        as libc::c_int
        != 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if ((h & (15 as libc::c_int as uint64_t) << 59 as libc::c_int) >> 59 as libc::c_int)
        as libc::c_int
        != 1 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if ((h & (7 as libc::c_int as uint64_t) << 56 as libc::c_int) >> 56 as libc::c_int)
        as libc::c_int
        != 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    let mut baseCell: libc::c_int = ((h & (127 as libc::c_int as uint64_t) << 45 as libc::c_int)
        >> 45 as libc::c_int) as libc::c_int;
    if baseCell < 0 as libc::c_int || baseCell >= 122 as libc::c_int {
        return 0 as libc::c_int;
    }
    let mut res: libc::c_int =
        ((h & (15 as libc::c_ulonglong) << 52 as libc::c_int) >> 52 as libc::c_int) as libc::c_int;
    if res < 0 as libc::c_int || res > 15 as libc::c_int {
        return 0 as libc::c_int;
    }
    let mut foundFirstNonZeroDigit: bool = 0 as libc::c_int != 0;
    let mut r: libc::c_int = 1 as libc::c_int;
    while r <= res {
        let mut digit: Direction = (h >> ((15 as libc::c_int - r) * 3 as libc::c_int)
            & 7 as libc::c_int as uint64_t) as Direction;
        if !foundFirstNonZeroDigit
            && digit as libc::c_uint != CENTER_DIGIT as libc::c_int as libc::c_uint
        {
            foundFirstNonZeroDigit = 1 as libc::c_int != 0;
            if _isBaseCellPentagon(baseCell) != 0
                && digit as libc::c_uint == K_AXES_DIGIT as libc::c_int as libc::c_uint
            {
                return 0 as libc::c_int;
            }
        }
        if (digit as libc::c_uint) < CENTER_DIGIT as libc::c_int as libc::c_uint
            || digit as libc::c_uint >= NUM_DIGITS as libc::c_int as libc::c_uint
        {
            return 0 as libc::c_int;
        }
        r += 1;
    }
    let mut r_0: libc::c_int = res + 1 as libc::c_int;
    while r_0 <= 15 as libc::c_int {
        let mut digit_0: Direction = (h >> ((15 as libc::c_int - r_0) * 3 as libc::c_int)
            & 7 as libc::c_int as uint64_t) as Direction;
        if digit_0 as libc::c_uint != INVALID_DIGIT as libc::c_int as libc::c_uint {
            return 0 as libc::c_int;
        }
        r_0 += 1;
    }
    1 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn setH3Index(
    mut hp: *mut H3Index,
    mut res: libc::c_int,
    mut baseCell: libc::c_int,
    mut initDigit: Direction,
) {
    let mut h: H3Index = 35184372088831 as libc::c_ulonglong;
    h = h & !((15 as libc::c_int as uint64_t) << 59 as libc::c_int)
        | (1 as libc::c_int as uint64_t) << 59 as libc::c_int;
    h = h & !((15 as libc::c_ulonglong) << 52 as libc::c_int)
        | (res as uint64_t) << 52 as libc::c_int;
    h = h & !((127 as libc::c_int as uint64_t) << 45 as libc::c_int)
        | (baseCell as uint64_t) << 45 as libc::c_int;
    let mut r: libc::c_int = 1 as libc::c_int;
    while r <= res {
        h = h & !((7 as libc::c_int as uint64_t) << ((15 as libc::c_int - r) * 3 as libc::c_int))
            | (initDigit as uint64_t) << ((15 as libc::c_int - r) * 3 as libc::c_int);
        r += 1;
    }
    *hp = h;
}
#[no_mangle]
pub unsafe extern "C" fn cellToParent(
    mut h: H3Index,
    mut parentRes: libc::c_int,
    mut out: *mut H3Index,
) -> H3Error {
    let mut childRes: libc::c_int =
        ((h & (15 as libc::c_ulonglong) << 52 as libc::c_int) >> 52 as libc::c_int) as libc::c_int;
    if parentRes < 0 as libc::c_int || parentRes > 15 as libc::c_int {
        return E_RES_DOMAIN as libc::c_int as H3Error;
    } else if parentRes > childRes {
        return E_RES_MISMATCH as libc::c_int as H3Error;
    } else if parentRes == childRes {
        *out = h;
        return E_SUCCESS as libc::c_int as H3Error;
    }
    h = h & !((15 as libc::c_ulonglong) << 52 as libc::c_int)
        | (parentRes as uint64_t) << 52 as libc::c_int;
    let mut parentH: H3Index = h;
    let mut i: libc::c_int = parentRes + 1 as libc::c_int;
    while i <= childRes {
        parentH = parentH
            & !((7 as libc::c_int as uint64_t) << ((15 as libc::c_int - i) * 3 as libc::c_int))
            | (7 as libc::c_int as uint64_t) << ((15 as libc::c_int - i) * 3 as libc::c_int);
        i += 1;
    }
    *out = parentH;
    E_SUCCESS as libc::c_int as H3Error
}
unsafe extern "C" fn _hasChildAtRes(mut h: H3Index, mut childRes: libc::c_int) -> bool {
    let mut parentRes: libc::c_int =
        ((h & (15 as libc::c_ulonglong) << 52 as libc::c_int) >> 52 as libc::c_int) as libc::c_int;
    if childRes < parentRes || childRes > 15 as libc::c_int {
        return 0 as libc::c_int != 0;
    }
    1 as libc::c_int != 0
}
#[no_mangle]
pub unsafe extern "C" fn cellToChildrenSize(
    mut h: H3Index,
    mut childRes: libc::c_int,
    mut out: *mut int64_t,
) -> H3Error {
    if !_hasChildAtRes(h, childRes) {
        return E_RES_DOMAIN as libc::c_int as H3Error;
    }
    let mut n: libc::c_int = childRes
        - ((h & (15 as libc::c_ulonglong) << 52 as libc::c_int) >> 52 as libc::c_int)
            as libc::c_int;
    if isPentagon(h) != 0 {
        *out = 1 as libc::c_int as libc::c_longlong
            + 5 as libc::c_int as libc::c_longlong
                * (_ipow(7 as libc::c_int as int64_t, n as int64_t)
                    - 1 as libc::c_int as libc::c_longlong)
                / 6 as libc::c_int as libc::c_longlong;
    } else {
        *out = _ipow(7 as libc::c_int as int64_t, n as int64_t);
    }
    E_SUCCESS as libc::c_int as H3Error
}
#[no_mangle]
pub unsafe extern "C" fn makeDirectChild(mut h: H3Index, mut cellNumber: libc::c_int) -> H3Index {
    let mut childRes: libc::c_int = ((h & (15 as libc::c_ulonglong) << 52 as libc::c_int)
        >> 52 as libc::c_int) as libc::c_int
        + 1 as libc::c_int;
    h = h & !((15 as libc::c_ulonglong) << 52 as libc::c_int)
        | (childRes as uint64_t) << 52 as libc::c_int;
    let mut childH: H3Index = h;
    childH = childH
        & !((7 as libc::c_int as uint64_t) << ((15 as libc::c_int - childRes) * 3 as libc::c_int))
        | (cellNumber as uint64_t) << ((15 as libc::c_int - childRes) * 3 as libc::c_int);
    childH
}
#[no_mangle]
pub unsafe extern "C" fn cellToChildren(
    mut h: H3Index,
    mut childRes: libc::c_int,
    mut children: *mut H3Index,
) -> H3Error {
    let mut i: int64_t = 0 as libc::c_int as int64_t;
    let mut iter: IterCellsChildren = iterInitParent(h, childRes);
    while iter.h != 0 {
        *children.offset(i as isize) = iter.h;
        i += 1;
        iterStepChild(&mut iter);
    }
    E_SUCCESS as libc::c_int as H3Error
}
#[no_mangle]
pub unsafe extern "C" fn _zeroIndexDigits(
    mut h: H3Index,
    mut start: libc::c_int,
    mut end: libc::c_int,
) -> H3Index {
    if start > end {
        return h;
    }
    let mut m: H3Index = 0 as libc::c_int as H3Index;
    m = !m;
    m <<= 3 as libc::c_int * (end - start + 1 as libc::c_int);
    m = !m;
    m <<= 3 as libc::c_int * (15 as libc::c_int - end);
    m = !m;
    h & m
}
#[no_mangle]
pub unsafe extern "C" fn cellToCenterChild(
    mut h: H3Index,
    mut childRes: libc::c_int,
    mut child: *mut H3Index,
) -> H3Error {
    if !_hasChildAtRes(h, childRes) {
        return E_RES_DOMAIN as libc::c_int as H3Error;
    }
    h = _zeroIndexDigits(
        h,
        ((h & (15 as libc::c_ulonglong) << 52 as libc::c_int) >> 52 as libc::c_int) as libc::c_int
            + 1 as libc::c_int,
        childRes,
    );
    h = h & !((15 as libc::c_ulonglong) << 52 as libc::c_int)
        | (childRes as uint64_t) << 52 as libc::c_int;
    *child = h;
    E_SUCCESS as libc::c_int as H3Error
}
#[no_mangle]
pub unsafe extern "C" fn compactCells(
    mut h3Set: *const H3Index,
    mut compactedSet: *mut H3Index,
    numHexes: int64_t,
) -> H3Error {
    if numHexes == 0 as libc::c_int as libc::c_longlong {
        return E_SUCCESS as libc::c_int as H3Error;
    }
    let mut res: libc::c_int = ((*h3Set.offset(0 as libc::c_int as isize)
        & (15 as libc::c_ulonglong) << 52 as libc::c_int)
        >> 52 as libc::c_int) as libc::c_int;
    if res == 0 as libc::c_int {
        let mut i: libc::c_int = 0 as libc::c_int;
        while (i as libc::c_longlong) < numHexes {
            *compactedSet.offset(i as isize) = *h3Set.offset(i as isize);
            i += 1;
        }
        return E_SUCCESS as libc::c_int as H3Error;
    }
    let mut remainingHexes: *mut H3Index = test_prefix_malloc(
        (numHexes as libc::c_ulonglong)
            .wrapping_mul(::core::mem::size_of::<H3Index>() as libc::c_ulong as libc::c_ulonglong)
            as size_t,
    ) as *mut H3Index;
    if remainingHexes.is_null() {
        return E_MEMORY_ALLOC as libc::c_int as H3Error;
    }
    memcpy(
        remainingHexes as *mut libc::c_void,
        h3Set as *const libc::c_void,
        (numHexes as libc::c_ulonglong)
            .wrapping_mul(::core::mem::size_of::<H3Index>() as libc::c_ulong as libc::c_ulonglong)
            as libc::c_ulong,
    );
    let mut hashSetArray: *mut H3Index = test_prefix_calloc(
        numHexes as size_t,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if hashSetArray.is_null() {
        test_prefix_free(remainingHexes as *mut libc::c_void);
        return E_MEMORY_ALLOC as libc::c_int as H3Error;
    }
    let mut compactedSetOffset: *mut H3Index = compactedSet;
    let mut numRemainingHexes: libc::c_int = numHexes as libc::c_int;
    while numRemainingHexes != 0 {
        res = ((*remainingHexes.offset(0 as libc::c_int as isize)
            & (15 as libc::c_ulonglong) << 52 as libc::c_int)
            >> 52 as libc::c_int) as libc::c_int;
        let mut parentRes: libc::c_int = res - 1 as libc::c_int;
        if parentRes >= 0 as libc::c_int {
            let mut i_0: libc::c_int = 0 as libc::c_int;
            while i_0 < numRemainingHexes {
                let mut currIndex: H3Index = *remainingHexes.offset(i_0 as isize);
                if currIndex != 0 as libc::c_int as libc::c_ulonglong {
                    if ((currIndex & (7 as libc::c_int as uint64_t) << 56 as libc::c_int)
                        >> 56 as libc::c_int) as libc::c_int
                        != 0 as libc::c_int
                    {
                        test_prefix_free(remainingHexes as *mut libc::c_void);
                        test_prefix_free(hashSetArray as *mut libc::c_void);
                        return E_CELL_INVALID as libc::c_int as H3Error;
                    }
                    let mut parent: H3Index = 0;
                    let mut parentError: H3Error = cellToParent(currIndex, parentRes, &mut parent);
                    if parentError != 0 {
                        test_prefix_free(remainingHexes as *mut libc::c_void);
                        test_prefix_free(hashSetArray as *mut libc::c_void);
                        return parentError;
                    }
                    let mut loc: libc::c_int =
                        parent.wrapping_rem(numRemainingHexes as libc::c_ulonglong) as libc::c_int;
                    let mut loopCount: libc::c_int = 0 as libc::c_int;
                    while *hashSetArray.offset(loc as isize)
                        != 0 as libc::c_int as libc::c_ulonglong
                    {
                        if loopCount > numRemainingHexes {
                            test_prefix_free(remainingHexes as *mut libc::c_void);
                            test_prefix_free(hashSetArray as *mut libc::c_void);
                            return E_FAILED as libc::c_int as H3Error;
                        }
                        let mut tempIndex: H3Index = *hashSetArray.offset(loc as isize)
                            & !((7 as libc::c_int as uint64_t) << 56 as libc::c_int);
                        if tempIndex == parent {
                            let mut count: libc::c_int = ((*hashSetArray.offset(loc as isize)
                                & (7 as libc::c_int as uint64_t) << 56 as libc::c_int)
                                >> 56 as libc::c_int)
                                as libc::c_int
                                + 1 as libc::c_int;
                            let mut limitCount: libc::c_int = 7 as libc::c_int;
                            if isPentagon(
                                tempIndex & !((7 as libc::c_int as uint64_t) << 56 as libc::c_int),
                            ) != 0
                            {
                                limitCount -= 1;
                            }
                            if count + 1 as libc::c_int > limitCount {
                                test_prefix_free(remainingHexes as *mut libc::c_void);
                                test_prefix_free(hashSetArray as *mut libc::c_void);
                                return E_DUPLICATE_INPUT as libc::c_int as H3Error;
                            }
                            parent = parent
                                & !((7 as libc::c_int as uint64_t) << 56 as libc::c_int)
                                | (count as uint64_t) << 56 as libc::c_int;
                            *hashSetArray.offset(loc as isize) = 0 as libc::c_int as H3Index;
                        } else {
                            loc = (loc + 1 as libc::c_int) % numRemainingHexes;
                        }
                        loopCount += 1;
                    }
                    *hashSetArray.offset(loc as isize) = parent;
                }
                i_0 += 1;
            }
        }
        let mut compactableCount: libc::c_int = 0 as libc::c_int;
        let mut maxCompactableCount: libc::c_int = numRemainingHexes / 6 as libc::c_int;
        if maxCompactableCount == 0 as libc::c_int {
            memcpy(
                compactedSetOffset as *mut libc::c_void,
                remainingHexes as *const libc::c_void,
                (numRemainingHexes as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<H3Index>() as libc::c_ulong),
            );
            break;
        } else {
            let mut compactableHexes: *mut H3Index = test_prefix_calloc(
                maxCompactableCount as size_t,
                ::core::mem::size_of::<H3Index>() as libc::c_ulong,
            ) as *mut H3Index;
            if compactableHexes.is_null() {
                test_prefix_free(remainingHexes as *mut libc::c_void);
                test_prefix_free(hashSetArray as *mut libc::c_void);
                return E_MEMORY_ALLOC as libc::c_int as H3Error;
            }
            let mut i_1: libc::c_int = 0 as libc::c_int;
            while i_1 < numRemainingHexes {
                if *hashSetArray.offset(i_1 as isize) != 0 as libc::c_int as libc::c_ulonglong {
                    let mut count_0: libc::c_int = ((*hashSetArray.offset(i_1 as isize)
                        & (7 as libc::c_int as uint64_t) << 56 as libc::c_int)
                        >> 56 as libc::c_int)
                        as libc::c_int
                        + 1 as libc::c_int;
                    if isPentagon(
                        *hashSetArray.offset(i_1 as isize)
                            & !((7 as libc::c_int as uint64_t) << 56 as libc::c_int),
                    ) != 0
                    {
                        *hashSetArray.offset(i_1 as isize) = *hashSetArray.offset(i_1 as isize)
                            & !((7 as libc::c_int as uint64_t) << 56 as libc::c_int)
                            | (count_0 as uint64_t) << 56 as libc::c_int;
                        count_0 += 1;
                    }
                    if count_0 == 7 as libc::c_int {
                        *compactableHexes.offset(compactableCount as isize) = *hashSetArray
                            .offset(i_1 as isize)
                            & !((7 as libc::c_int as uint64_t) << 56 as libc::c_int);
                        compactableCount += 1;
                    }
                }
                i_1 += 1;
            }
            let mut uncompactableCount: libc::c_int = 0 as libc::c_int;
            let mut i_2: libc::c_int = 0 as libc::c_int;
            while i_2 < numRemainingHexes {
                let mut currIndex_0: H3Index = *remainingHexes.offset(i_2 as isize);
                if currIndex_0 != 0 as libc::c_int as libc::c_ulonglong {
                    let mut parent_0: H3Index = 0;
                    let mut parentError_0: H3Error =
                        cellToParent(currIndex_0, parentRes, &mut parent_0);
                    if parentError_0 != 0 {
                        test_prefix_free(compactableHexes as *mut libc::c_void);
                        test_prefix_free(remainingHexes as *mut libc::c_void);
                        test_prefix_free(hashSetArray as *mut libc::c_void);
                        return parentError_0;
                    }
                    let mut loc_0: libc::c_int = parent_0
                        .wrapping_rem(numRemainingHexes as libc::c_ulonglong)
                        as libc::c_int;
                    let mut loopCount_0: libc::c_int = 0 as libc::c_int;
                    let mut isUncompactable: bool = 1 as libc::c_int != 0;
                    loop {
                        if loopCount_0 > numRemainingHexes {
                            test_prefix_free(compactableHexes as *mut libc::c_void);
                            test_prefix_free(remainingHexes as *mut libc::c_void);
                            test_prefix_free(hashSetArray as *mut libc::c_void);
                            return E_FAILED as libc::c_int as H3Error;
                        }
                        let mut tempIndex_0: H3Index = *hashSetArray.offset(loc_0 as isize)
                            & !((7 as libc::c_int as uint64_t) << 56 as libc::c_int);
                        if tempIndex_0 == parent_0 {
                            let mut count_1: libc::c_int = ((*hashSetArray.offset(loc_0 as isize)
                                & (7 as libc::c_int as uint64_t) << 56 as libc::c_int)
                                >> 56 as libc::c_int)
                                as libc::c_int
                                + 1 as libc::c_int;
                            if count_1 == 7 as libc::c_int {
                                isUncompactable = 0 as libc::c_int != 0;
                            }
                            break;
                        } else {
                            loc_0 = (loc_0 + 1 as libc::c_int) % numRemainingHexes;
                            loopCount_0 += 1;
                            if *hashSetArray.offset(loc_0 as isize) == parent_0 {
                                break;
                            }
                        }
                    }
                    if isUncompactable {
                        *compactedSetOffset.offset(uncompactableCount as isize) =
                            *remainingHexes.offset(i_2 as isize);
                        uncompactableCount += 1;
                    }
                }
                i_2 += 1;
            }
            memset(
                hashSetArray as *mut libc::c_void,
                0 as libc::c_int,
                (numHexes as libc::c_ulonglong).wrapping_mul(::core::mem::size_of::<H3Index>()
                    as libc::c_ulong
                    as libc::c_ulonglong) as libc::c_ulong,
            );
            compactedSetOffset = compactedSetOffset.offset(uncompactableCount as isize);
            memcpy(
                remainingHexes as *mut libc::c_void,
                compactableHexes as *const libc::c_void,
                (compactableCount as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<H3Index>() as libc::c_ulong),
            );
            numRemainingHexes = compactableCount;
            test_prefix_free(compactableHexes as *mut libc::c_void);
        }
    }
    test_prefix_free(remainingHexes as *mut libc::c_void);
    test_prefix_free(hashSetArray as *mut libc::c_void);
    E_SUCCESS as libc::c_int as H3Error
}
#[no_mangle]
pub unsafe extern "C" fn uncompactCells(
    mut compactedSet: *const H3Index,
    numCompacted: int64_t,
    mut outSet: *mut H3Index,
    numOut: int64_t,
    res: libc::c_int,
) -> H3Error {
    let mut i: int64_t = 0 as libc::c_int as int64_t;
    let mut j: int64_t = 0 as libc::c_int as int64_t;
    while j < numCompacted {
        if !_hasChildAtRes(*compactedSet.offset(j as isize), res) {
            return E_RES_MISMATCH as libc::c_int as H3Error;
        }
        let mut iter: IterCellsChildren = iterInitParent(*compactedSet.offset(j as isize), res);
        while iter.h != 0 {
            if i >= numOut {
                return E_MEMORY_BOUNDS as libc::c_int as H3Error;
            }
            *outSet.offset(i as isize) = iter.h;
            i += 1;
            iterStepChild(&mut iter);
        }
        j += 1;
    }
    E_SUCCESS as libc::c_int as H3Error
}
#[no_mangle]
pub unsafe extern "C" fn uncompactCellsSize(
    mut compactedSet: *const H3Index,
    numCompacted: int64_t,
    res: libc::c_int,
    mut out: *mut int64_t,
) -> H3Error {
    let mut numOut: int64_t = 0 as libc::c_int as int64_t;
    let mut i: int64_t = 0 as libc::c_int as int64_t;
    while i < numCompacted {
        if *compactedSet.offset(i as isize) != 0 as libc::c_int as libc::c_ulonglong {
            let mut childrenSize: int64_t = 0;
            let mut childrenError: H3Error =
                cellToChildrenSize(*compactedSet.offset(i as isize), res, &mut childrenSize);
            if childrenError != 0 {
                return E_RES_MISMATCH as libc::c_int as H3Error;
            }
            numOut += childrenSize;
        }
        i += 1;
    }
    *out = numOut;
    E_SUCCESS as libc::c_int as H3Error
}
#[no_mangle]
pub unsafe extern "C" fn isResClassIII(mut h: H3Index) -> libc::c_int {
    ((h & (15 as libc::c_ulonglong) << 52 as libc::c_int) >> 52 as libc::c_int) as libc::c_int
        % 2 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn isPentagon(mut h: H3Index) -> libc::c_int {
    (_isBaseCellPentagon(
        ((h & (127 as libc::c_int as uint64_t) << 45 as libc::c_int) >> 45 as libc::c_int)
            as libc::c_int,
    ) != 0
        && _h3LeadingNonZeroDigit(h) as u64 == 0) as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn _h3LeadingNonZeroDigit(mut h: H3Index) -> Direction {
    let mut r: libc::c_int = 1 as libc::c_int;
    while r
        <= ((h & (15 as libc::c_ulonglong) << 52 as libc::c_int) >> 52 as libc::c_int)
            as libc::c_int
    {
        if (h >> ((15 as libc::c_int - r) * 3 as libc::c_int) & 7 as libc::c_int as uint64_t)
            as Direction as u64
            != 0
        {
            return (h >> ((15 as libc::c_int - r) * 3 as libc::c_int)
                & 7 as libc::c_int as uint64_t) as Direction;
        }
        r += 1;
    }
    CENTER_DIGIT
}
#[no_mangle]
pub unsafe extern "C" fn _h3RotatePent60ccw(mut h: H3Index) -> H3Index {
    let mut foundFirstNonZeroDigit: libc::c_int = 0 as libc::c_int;
    let mut r: libc::c_int = 1 as libc::c_int;
    let mut res: libc::c_int =
        ((h & (15 as libc::c_ulonglong) << 52 as libc::c_int) >> 52 as libc::c_int) as libc::c_int;
    while r <= res {
        h = h & !((7 as libc::c_int as uint64_t) << ((15 as libc::c_int - r) * 3 as libc::c_int))
            | (_rotate60ccw(
                (h >> ((15 as libc::c_int - r) * 3 as libc::c_int) & 7 as libc::c_int as uint64_t)
                    as Direction,
            ) as uint64_t)
                << ((15 as libc::c_int - r) * 3 as libc::c_int);
        if foundFirstNonZeroDigit == 0
            && (h >> ((15 as libc::c_int - r) * 3 as libc::c_int) & 7 as libc::c_int as uint64_t)
                as Direction as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
        {
            foundFirstNonZeroDigit = 1 as libc::c_int;
            if _h3LeadingNonZeroDigit(h) as libc::c_uint
                == K_AXES_DIGIT as libc::c_int as libc::c_uint
            {
                h = _h3Rotate60ccw(h);
            }
        }
        r += 1;
    }
    h
}
#[no_mangle]
pub unsafe extern "C" fn _h3RotatePent60cw(mut h: H3Index) -> H3Index {
    let mut foundFirstNonZeroDigit: libc::c_int = 0 as libc::c_int;
    let mut r: libc::c_int = 1 as libc::c_int;
    let mut res: libc::c_int =
        ((h & (15 as libc::c_ulonglong) << 52 as libc::c_int) >> 52 as libc::c_int) as libc::c_int;
    while r <= res {
        h = h & !((7 as libc::c_int as uint64_t) << ((15 as libc::c_int - r) * 3 as libc::c_int))
            | (_rotate60cw(
                (h >> ((15 as libc::c_int - r) * 3 as libc::c_int) & 7 as libc::c_int as uint64_t)
                    as Direction,
            ) as uint64_t)
                << ((15 as libc::c_int - r) * 3 as libc::c_int);
        if foundFirstNonZeroDigit == 0
            && (h >> ((15 as libc::c_int - r) * 3 as libc::c_int) & 7 as libc::c_int as uint64_t)
                as Direction as libc::c_uint
                != 0 as libc::c_int as libc::c_uint
        {
            foundFirstNonZeroDigit = 1 as libc::c_int;
            if _h3LeadingNonZeroDigit(h) as libc::c_uint
                == K_AXES_DIGIT as libc::c_int as libc::c_uint
            {
                h = _h3Rotate60cw(h);
            }
        }
        r += 1;
    }
    h
}
#[no_mangle]
pub unsafe extern "C" fn _h3Rotate60ccw(mut h: H3Index) -> H3Index {
    let mut r: libc::c_int = 1 as libc::c_int;
    let mut res: libc::c_int =
        ((h & (15 as libc::c_ulonglong) << 52 as libc::c_int) >> 52 as libc::c_int) as libc::c_int;
    while r <= res {
        let mut oldDigit: Direction = (h >> ((15 as libc::c_int - r) * 3 as libc::c_int)
            & 7 as libc::c_int as uint64_t) as Direction;
        h = h & !((7 as libc::c_int as uint64_t) << ((15 as libc::c_int - r) * 3 as libc::c_int))
            | (_rotate60ccw(oldDigit) as uint64_t) << ((15 as libc::c_int - r) * 3 as libc::c_int);
        r += 1;
    }
    h
}
#[no_mangle]
pub unsafe extern "C" fn _h3Rotate60cw(mut h: H3Index) -> H3Index {
    let mut r: libc::c_int = 1 as libc::c_int;
    let mut res: libc::c_int =
        ((h & (15 as libc::c_ulonglong) << 52 as libc::c_int) >> 52 as libc::c_int) as libc::c_int;
    while r <= res {
        h = h & !((7 as libc::c_int as uint64_t) << ((15 as libc::c_int - r) * 3 as libc::c_int))
            | (_rotate60cw(
                (h >> ((15 as libc::c_int - r) * 3 as libc::c_int) & 7 as libc::c_int as uint64_t)
                    as Direction,
            ) as uint64_t)
                << ((15 as libc::c_int - r) * 3 as libc::c_int);
        r += 1;
    }
    h
}
#[no_mangle]
pub unsafe extern "C" fn _faceIjkToH3(mut fijk: *const FaceIJK, mut res: libc::c_int) -> H3Index {
    let mut h: H3Index = 35184372088831 as libc::c_ulonglong;
    h = h & !((15 as libc::c_int as uint64_t) << 59 as libc::c_int)
        | (1 as libc::c_int as uint64_t) << 59 as libc::c_int;
    h = h & !((15 as libc::c_ulonglong) << 52 as libc::c_int)
        | (res as uint64_t) << 52 as libc::c_int;
    if res == 0 as libc::c_int {
        if (*fijk).coord.i > 2 as libc::c_int
            || (*fijk).coord.j > 2 as libc::c_int
            || (*fijk).coord.k > 2 as libc::c_int
        {
            return 0 as libc::c_int as H3Index;
        }
        h = h & !((127 as libc::c_int as uint64_t) << 45 as libc::c_int)
            | (_faceIjkToBaseCell(fijk) as uint64_t) << 45 as libc::c_int;
        return h;
    }
    let mut fijkBC: FaceIJK = *fijk;
    let mut ijk: *mut CoordIJK = &mut fijkBC.coord;
    let mut r: libc::c_int = res - 1 as libc::c_int;
    while r >= 0 as libc::c_int {
        let mut lastIJK: CoordIJK = *ijk;
        let mut lastCenter: CoordIJK = CoordIJK { i: 0, j: 0, k: 0 };
        if isResolutionClassIII(r + 1 as libc::c_int) != 0 {
            _upAp7(ijk);
            lastCenter = *ijk;
            _downAp7(&mut lastCenter);
        } else {
            _upAp7r(ijk);
            lastCenter = *ijk;
            _downAp7r(&mut lastCenter);
        }
        let mut diff: CoordIJK = CoordIJK { i: 0, j: 0, k: 0 };
        _ijkSub(&lastIJK, &lastCenter, &mut diff);
        _ijkNormalize(&mut diff);
        h = h & !((7 as libc::c_int as uint64_t)
            << ((15 as libc::c_int - (r + 1 as libc::c_int)) * 3 as libc::c_int))
            | (_unitIjkToDigit(&diff) as uint64_t)
                << ((15 as libc::c_int - (r + 1 as libc::c_int)) * 3 as libc::c_int);
        r -= 1;
    }
    if fijkBC.coord.i > 2 as libc::c_int
        || fijkBC.coord.j > 2 as libc::c_int
        || fijkBC.coord.k > 2 as libc::c_int
    {
        return 0 as libc::c_int as H3Index;
    }
    let mut baseCell: libc::c_int = _faceIjkToBaseCell(&fijkBC);
    h = h & !((127 as libc::c_int as uint64_t) << 45 as libc::c_int)
        | (baseCell as uint64_t) << 45 as libc::c_int;
    let mut numRots: libc::c_int = _faceIjkToBaseCellCCWrot60(&fijkBC);
    if _isBaseCellPentagon(baseCell) != 0 {
        if _h3LeadingNonZeroDigit(h) as libc::c_uint == K_AXES_DIGIT as libc::c_int as libc::c_uint
        {
            if _baseCellIsCwOffset(baseCell, fijkBC.face) {
                h = _h3Rotate60cw(h);
            } else {
                h = _h3Rotate60ccw(h);
            }
        }
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < numRots {
            h = _h3RotatePent60ccw(h);
            i += 1;
        }
    } else {
        let mut i_0: libc::c_int = 0 as libc::c_int;
        while i_0 < numRots {
            h = _h3Rotate60ccw(h);
            i_0 += 1;
        }
    }
    h
}
#[no_mangle]
pub unsafe extern "C" fn latLngToCell(
    mut g: *const LatLng,
    mut res: libc::c_int,
    mut out: *mut H3Index,
) -> H3Error {
    if res < 0 as libc::c_int || res > 15 as libc::c_int {
        return E_RES_DOMAIN as libc::c_int as H3Error;
    }
    if (if ::core::mem::size_of::<libc::c_double>() as libc::c_ulong
        == ::core::mem::size_of::<libc::c_float>() as libc::c_ulong
    {
        __inline_isfinitef((*g).lat as libc::c_float)
    } else if ::core::mem::size_of::<libc::c_double>() as libc::c_ulong
        == ::core::mem::size_of::<libc::c_double>() as libc::c_ulong
    {
        __inline_isfinited((*g).lat)
    } else {
        __inline_isfinitel((*g).lat)
    }) == 0
        || (if ::core::mem::size_of::<libc::c_double>() as libc::c_ulong
            == ::core::mem::size_of::<libc::c_float>() as libc::c_ulong
        {
            __inline_isfinitef((*g).lng as libc::c_float)
        } else if ::core::mem::size_of::<libc::c_double>() as libc::c_ulong
            == ::core::mem::size_of::<libc::c_double>() as libc::c_ulong
        {
            __inline_isfinited((*g).lng)
        } else {
            __inline_isfinitel((*g).lng)
        }) == 0
    {
        return E_LATLNG_DOMAIN as libc::c_int as H3Error;
    }
    let mut fijk: FaceIJK = FaceIJK {
        face: 0,
        coord: CoordIJK { i: 0, j: 0, k: 0 },
    };
    _geoToFaceIjk(g, res, &mut fijk);
    *out = _faceIjkToH3(&fijk, res);
    if *out != 0 {
        E_SUCCESS as libc::c_int as H3Error
    } else {
        E_FAILED as libc::c_int as H3Error
    }
}
#[no_mangle]
pub unsafe extern "C" fn _h3ToFaceIjkWithInitializedFijk(
    mut h: H3Index,
    mut fijk: *mut FaceIJK,
) -> libc::c_int {
    let mut ijk: *mut CoordIJK = &mut (*fijk).coord;
    let mut res: libc::c_int =
        ((h & (15 as libc::c_ulonglong) << 52 as libc::c_int) >> 52 as libc::c_int) as libc::c_int;
    let mut possibleOverage: libc::c_int = 1 as libc::c_int;
    if _isBaseCellPentagon(
        ((h & (127 as libc::c_int as uint64_t) << 45 as libc::c_int) >> 45 as libc::c_int)
            as libc::c_int,
    ) == 0
        && (res == 0 as libc::c_int
            || (*fijk).coord.i == 0 as libc::c_int
                && (*fijk).coord.j == 0 as libc::c_int
                && (*fijk).coord.k == 0 as libc::c_int)
    {
        possibleOverage = 0 as libc::c_int;
    }
    let mut r: libc::c_int = 1 as libc::c_int;
    while r <= res {
        if isResolutionClassIII(r) != 0 {
            _downAp7(ijk);
        } else {
            _downAp7r(ijk);
        }
        _neighbor(
            ijk,
            (h >> ((15 as libc::c_int - r) * 3 as libc::c_int) & 7 as libc::c_int as uint64_t)
                as Direction,
        );
        r += 1;
    }
    possibleOverage
}
#[no_mangle]
pub unsafe extern "C" fn _h3ToFaceIjk(mut h: H3Index, mut fijk: *mut FaceIJK) -> H3Error {
    let mut baseCell: libc::c_int = ((h & (127 as libc::c_int as uint64_t) << 45 as libc::c_int)
        >> 45 as libc::c_int) as libc::c_int;
    if baseCell < 0 as libc::c_int || baseCell >= 122 as libc::c_int {
        (*fijk).face = 0 as libc::c_int;
        (*fijk).coord.k = 0 as libc::c_int;
        (*fijk).coord.j = (*fijk).coord.k;
        (*fijk).coord.i = (*fijk).coord.j;
        return E_CELL_INVALID as libc::c_int as H3Error;
    }
    if _isBaseCellPentagon(baseCell) != 0
        && _h3LeadingNonZeroDigit(h) as libc::c_uint == 5 as libc::c_int as libc::c_uint
    {
        h = _h3Rotate60cw(h);
    }
    *fijk = baseCellData[baseCell as usize].homeFijk;
    if _h3ToFaceIjkWithInitializedFijk(h, fijk) == 0 {
        return E_SUCCESS as libc::c_int as H3Error;
    }
    let mut origIJK: CoordIJK = (*fijk).coord;
    let mut res: libc::c_int =
        ((h & (15 as libc::c_ulonglong) << 52 as libc::c_int) >> 52 as libc::c_int) as libc::c_int;
    if isResolutionClassIII(res) != 0 {
        _downAp7r(&mut (*fijk).coord);
        res += 1;
    }
    let mut pentLeading4: libc::c_int = (_isBaseCellPentagon(baseCell) != 0
        && _h3LeadingNonZeroDigit(h) as libc::c_uint == 4 as libc::c_int as libc::c_uint)
        as libc::c_int;
    if _adjustOverageClassII(fijk, res, pentLeading4, 0 as libc::c_int) as libc::c_uint
        != NO_OVERAGE as libc::c_int as libc::c_uint
    {
        if _isBaseCellPentagon(baseCell) != 0 {
            while _adjustOverageClassII(fijk, res, 0 as libc::c_int, 0 as libc::c_int)
                as libc::c_uint
                != NO_OVERAGE as libc::c_int as libc::c_uint
            {}
        }
        if res
            != ((h & (15 as libc::c_ulonglong) << 52 as libc::c_int) >> 52 as libc::c_int)
                as libc::c_int
        {
            _upAp7r(&mut (*fijk).coord);
        }
    } else if res
        != ((h & (15 as libc::c_ulonglong) << 52 as libc::c_int) >> 52 as libc::c_int)
            as libc::c_int
    {
        (*fijk).coord = origIJK;
    }
    E_SUCCESS as libc::c_int as H3Error
}
#[no_mangle]
pub unsafe extern "C" fn cellToLatLng(mut h3: H3Index, mut g: *mut LatLng) -> H3Error {
    let mut fijk: FaceIJK = FaceIJK {
        face: 0,
        coord: CoordIJK { i: 0, j: 0, k: 0 },
    };
    let mut e: H3Error = _h3ToFaceIjk(h3, &mut fijk);
    if e != 0 {
        return e;
    }
    _faceIjkToGeo(
        &fijk,
        ((h3 & (15 as libc::c_ulonglong) << 52 as libc::c_int) >> 52 as libc::c_int) as libc::c_int,
        g,
    );
    E_SUCCESS as libc::c_int as H3Error
}
#[no_mangle]
pub unsafe extern "C" fn cellToBoundary(mut h3: H3Index, mut cb: *mut CellBoundary) -> H3Error {
    let mut fijk: FaceIJK = FaceIJK {
        face: 0,
        coord: CoordIJK { i: 0, j: 0, k: 0 },
    };
    let mut e: H3Error = _h3ToFaceIjk(h3, &mut fijk);
    if e != 0 {
        return e;
    }
    if isPentagon(h3) != 0 {
        _faceIjkPentToCellBoundary(
            &fijk,
            ((h3 & (15 as libc::c_ulonglong) << 52 as libc::c_int) >> 52 as libc::c_int)
                as libc::c_int,
            0 as libc::c_int,
            5 as libc::c_int,
            cb,
        );
    } else {
        _faceIjkToCellBoundary(
            &fijk,
            ((h3 & (15 as libc::c_ulonglong) << 52 as libc::c_int) >> 52 as libc::c_int)
                as libc::c_int,
            0 as libc::c_int,
            6 as libc::c_int,
            cb,
        );
    }
    E_SUCCESS as libc::c_int as H3Error
}
#[no_mangle]
pub unsafe extern "C" fn maxFaceCount(mut h3: H3Index, mut out: *mut libc::c_int) -> H3Error {
    *out = if isPentagon(h3) != 0 {
        5 as libc::c_int
    } else {
        2 as libc::c_int
    };
    E_SUCCESS as libc::c_int as H3Error
}
#[no_mangle]
pub unsafe extern "C" fn getIcosahedronFaces(
    mut h3: H3Index,
    mut out: *mut libc::c_int,
) -> H3Error {
    let mut res: libc::c_int =
        ((h3 & (15 as libc::c_ulonglong) << 52 as libc::c_int) >> 52 as libc::c_int) as libc::c_int;
    let mut isPent: libc::c_int = isPentagon(h3);
    if isPent != 0 && isResolutionClassIII(res) == 0 {
        let mut childPentagon: H3Index = makeDirectChild(h3, 0 as libc::c_int);
        return getIcosahedronFaces(childPentagon, out);
    }
    let mut fijk: FaceIJK = FaceIJK {
        face: 0,
        coord: CoordIJK { i: 0, j: 0, k: 0 },
    };
    let mut err: H3Error = _h3ToFaceIjk(h3, &mut fijk);
    if err != 0 {
        return err;
    }
    let mut fijkVerts: [FaceIJK; 6] = [FaceIJK {
        face: 0,
        coord: CoordIJK { i: 0, j: 0, k: 0 },
    }; 6];
    let mut vertexCount: libc::c_int = 0;
    if isPent != 0 {
        vertexCount = 5 as libc::c_int;
        _faceIjkPentToVerts(&mut fijk, &mut res, fijkVerts.as_mut_ptr());
    } else {
        vertexCount = 6 as libc::c_int;
        _faceIjkToVerts(&mut fijk, &mut res, fijkVerts.as_mut_ptr());
    }
    let mut faceCount: libc::c_int = 0;
    let mut maxFaceCountError: H3Error = maxFaceCount(h3, &mut faceCount);
    if maxFaceCountError != E_SUCCESS as libc::c_int as libc::c_uint {
        return maxFaceCountError;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < faceCount {
        *out.offset(i as isize) = -(1 as libc::c_int);
        i += 1;
    }
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < vertexCount {
        let mut vert: *mut FaceIJK =
            &mut *fijkVerts.as_mut_ptr().offset(i_0 as isize) as *mut FaceIJK;
        if isPent != 0 {
            _adjustPentVertOverage(vert, res);
        } else {
            _adjustOverageClassII(vert, res, 0 as libc::c_int, 1 as libc::c_int);
        }
        let mut face: libc::c_int = (*vert).face;
        let mut pos: libc::c_int = 0 as libc::c_int;
        while *out.offset(pos as isize) != -(1 as libc::c_int) && *out.offset(pos as isize) != face
        {
            pos += 1;
            if pos >= faceCount {
                return E_FAILED as libc::c_int as H3Error;
            }
        }
        *out.offset(pos as isize) = face;
        i_0 += 1;
    }
    E_SUCCESS as libc::c_int as H3Error
}
#[no_mangle]
pub unsafe extern "C" fn pentagonCount() -> libc::c_int {
    12 as libc::c_int
}
#[no_mangle]
pub unsafe extern "C" fn getPentagons(mut res: libc::c_int, mut out: *mut H3Index) -> H3Error {
    if res < 0 as libc::c_int || res > 15 as libc::c_int {
        return E_RES_DOMAIN as libc::c_int as H3Error;
    }
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut bc: libc::c_int = 0 as libc::c_int;
    while bc < 122 as libc::c_int {
        if _isBaseCellPentagon(bc) != 0 {
            let mut pentagon: H3Index = 0;
            setH3Index(&mut pentagon, res, bc, CENTER_DIGIT);
            let fresh0 = i;
            i += 1;
            *out.offset(fresh0 as isize) = pentagon;
        }
        bc += 1;
    }
    E_SUCCESS as libc::c_int as H3Error
}
#[no_mangle]
pub unsafe extern "C" fn isResolutionClassIII(mut res: libc::c_int) -> libc::c_int {
    res % 2 as libc::c_int
}
