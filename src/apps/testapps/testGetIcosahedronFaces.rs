#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(clippy::missing_safety_doc)]

extern crate unsafe_h3lib_testapps;
use ::libc;
extern "C" {

    static mut __stderrp: *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn isPentagon(h: H3Index) -> libc::c_int;
    fn maxFaceCount(h3: H3Index, out: *mut libc::c_int) -> H3Error;
    fn getIcosahedronFaces(h3: H3Index, out: *mut libc::c_int) -> H3Error;
    fn _isBaseCellPentagon(baseCell: libc::c_int) -> libc::c_int;
    fn setH3Index(h: *mut H3Index, res: libc::c_int, baseCell: libc::c_int, initDigit: Direction);
    static mut globalTestCount: libc::c_int;
    static mut currentSuiteName: *const libc::c_char;
    static mut currentTestName: *const libc::c_char;
    fn iterateBaseCellIndexesAtRes(
        res: libc::c_int,
        callback: Option<unsafe extern "C" fn(H3Index) -> ()>,
        baseCell: libc::c_int,
    );
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_off_t = __int64_t;
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut libc::c_uchar,
    pub _size: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut libc::c_uchar,
    pub _r: libc::c_int,
    pub _w: libc::c_int,
    pub _flags: libc::c_short,
    pub _file: libc::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: libc::c_int,
    pub _cookie: *mut libc::c_void,
    pub _close: Option<unsafe extern "C" fn(*mut libc::c_void) -> libc::c_int>,
    pub _read: Option<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub _seek: Option<unsafe extern "C" fn(*mut libc::c_void, fpos_t, libc::c_int) -> fpos_t>,
    pub _write: Option<
        unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, libc::c_int) -> libc::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut libc::c_void,
    pub _ur: libc::c_int,
    pub _ubuf: [libc::c_uchar; 3],
    pub _nbuf: [libc::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: libc::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
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
unsafe extern "C" fn countFaces(mut h3: H3Index, mut expectedMax: libc::c_int) -> libc::c_int {
    let mut sz: libc::c_int = 0;
    if maxFaceCount(h3, &mut sz) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGetIcosahedronFaces.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            b"!(maxFaceCount(h3, &sz))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(sz == expectedMax) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGetIcosahedronFaces.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            b"sz == expectedMax\0" as *const u8 as *const libc::c_char,
            b"got expected max face count\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut faces: *mut libc::c_int = calloc(
        sz as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    if getIcosahedronFaces(h3, faces) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGetIcosahedronFaces.c\0" as *const u8 as *const libc::c_char,
            35 as libc::c_int,
            b"!(getIcosahedronFaces(h3, faces))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut validCount: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < sz {
        if *faces.offset(i as isize) >= 0 as libc::c_int
            && *faces.offset(i as isize) <= 19 as libc::c_int
        {
            validCount += 1;
        }
        i += 1;
    }
    free(faces as *mut libc::c_void);
    return validCount;
}
unsafe extern "C" fn assertSingleHexFace(mut h3: H3Index) {
    let mut validCount: libc::c_int = countFaces(h3, 2 as libc::c_int);
    if !(validCount == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGetIcosahedronFaces.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            b"validCount == 1\0" as *const u8 as *const libc::c_char,
            b"got a single valid face\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn assertMultipleHexFaces(mut h3: H3Index) {
    let mut validCount: libc::c_int = countFaces(h3, 2 as libc::c_int);
    if !(validCount == 2 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGetIcosahedronFaces.c\0" as *const u8 as *const libc::c_char,
            53 as libc::c_int,
            b"validCount == 2\0" as *const u8 as *const libc::c_char,
            b"got multiple valid faces for a hexagon\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn assertPentagonFaces(mut h3: H3Index) {
    if isPentagon(h3) == 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGetIcosahedronFaces.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            b"H3_EXPORT(isPentagon)(h3)\0" as *const u8 as *const libc::c_char,
            b"got a pentagon\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut validCount: libc::c_int = countFaces(h3, 5 as libc::c_int);
    if !(validCount == 5 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGetIcosahedronFaces.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
            b"validCount == 5\0" as *const u8 as *const libc::c_char,
            b"got 5 valid faces for a pentagon\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn runTests() {
    currentTestName = b"singleFaceHexes\0" as *const u8 as *const libc::c_char;
    iterateBaseCellIndexesAtRes(
        2 as libc::c_int,
        Some(assertSingleHexFace as unsafe extern "C" fn(H3Index) -> ()),
        16 as libc::c_int,
    );
    iterateBaseCellIndexesAtRes(
        3 as libc::c_int,
        Some(assertSingleHexFace as unsafe extern "C" fn(H3Index) -> ()),
        16 as libc::c_int,
    );
    currentTestName = b"hexagonWithEdgeVertices\0" as *const u8 as *const libc::c_char;
    let mut h3: H3Index = 0x821c37fffffffff as libc::c_long as H3Index;
    assertSingleHexFace(h3);
    currentTestName = b"hexagonWithDistortion\0" as *const u8 as *const libc::c_char;
    let mut h3_0: H3Index = 0x831c06fffffffff as libc::c_long as H3Index;
    assertMultipleHexFaces(h3_0);
    currentTestName = b"hexagonCrossingFaces\0" as *const u8 as *const libc::c_char;
    let mut h3_1: H3Index = 0x821ce7fffffffff as libc::c_long as H3Index;
    assertMultipleHexFaces(h3_1);
    currentTestName = b"classIIIPentagon\0" as *const u8 as *const libc::c_char;
    let mut pentagon: H3Index = 0;
    setH3Index(
        &mut pentagon,
        1 as libc::c_int,
        4 as libc::c_int,
        CENTER_DIGIT,
    );
    assertPentagonFaces(pentagon);
    currentTestName = b"classIIPentagon\0" as *const u8 as *const libc::c_char;
    let mut pentagon_0: H3Index = 0;
    setH3Index(
        &mut pentagon_0,
        2 as libc::c_int,
        4 as libc::c_int,
        CENTER_DIGIT,
    );
    assertPentagonFaces(pentagon_0);
    currentTestName = b"res15Pentagon\0" as *const u8 as *const libc::c_char;
    let mut pentagon_1: H3Index = 0;
    setH3Index(
        &mut pentagon_1,
        15 as libc::c_int,
        4 as libc::c_int,
        CENTER_DIGIT,
    );
    assertPentagonFaces(pentagon_1);
    currentTestName = b"baseCellHexagons\0" as *const u8 as *const libc::c_char;
    let mut singleCount: libc::c_int = 0 as libc::c_int;
    let mut multipleCount: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 122 as libc::c_int {
        if _isBaseCellPentagon(i) == 0 {
            let mut baseCell: H3Index = 0;
            setH3Index(&mut baseCell, 0 as libc::c_int, i, CENTER_DIGIT);
            let mut validCount: libc::c_int = countFaces(baseCell, 2 as libc::c_int);
            if !(validCount > 0 as libc::c_int) {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"src/apps/testapps/testGetIcosahedronFaces.c\0" as *const u8
                        as *const libc::c_char,
                    115 as libc::c_int,
                    b"validCount > 0\0" as *const u8 as *const libc::c_char,
                    b"got at least one face\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
            if validCount == 1 as libc::c_int {
                singleCount += 1;
            } else {
                multipleCount += 1;
            }
        }
        i += 1;
    }
    if !(singleCount == 4 as libc::c_int * 20 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGetIcosahedronFaces.c\0" as *const u8 as *const libc::c_char,
            123 as libc::c_int,
            b"singleCount == 4 * 20\0" as *const u8 as *const libc::c_char,
            b"got single face for 4 aligned hex base cells per face\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(multipleCount as libc::c_double == 1.5f64 * 20 as libc::c_int as libc::c_double) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGetIcosahedronFaces.c\0" as *const u8 as *const libc::c_char,
            125 as libc::c_int,
            b"multipleCount == 1.5 * 20\0" as *const u8 as *const libc::c_char,
            b"got multiple faces for non-aligned hex base cells\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"baseCellPentagons\0" as *const u8 as *const libc::c_char;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 122 as libc::c_int {
        if _isBaseCellPentagon(i_0) != 0 {
            let mut baseCell_0: H3Index = 0;
            setH3Index(&mut baseCell_0, 0 as libc::c_int, i_0, CENTER_DIGIT);
            assertPentagonFaces(baseCell_0);
        }
        i_0 += 1;
    }
    currentTestName = b"invalid\0" as *const u8 as *const libc::c_char;
    let mut invalid: H3Index = 0xffffffffffffffff as libc::c_ulong as H3Index;
    let mut out: libc::c_int = 0;
    if !(getIcosahedronFaces(invalid, &mut out) == E_CELL_INVALID as libc::c_int as libc::c_uint) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGetIcosahedronFaces.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            b"H3_EXPORT(getIcosahedronFaces)(invalid, &out) == E_CELL_INVALID\0" as *const u8
                as *const libc::c_char,
            b"Invalid cell\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"invalid2\0" as *const u8 as *const libc::c_char;
    let mut invalid_0: H3Index = 0x71330073003f004e as libc::c_long as H3Index;
    let mut sz: libc::c_int = 0;
    if maxFaceCount(invalid_0, &mut sz) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGetIcosahedronFaces.c\0" as *const u8 as *const libc::c_char,
            150 as libc::c_int,
            b"!(maxFaceCount(invalid, &sz))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut faces: *mut libc::c_int = calloc(
        sz as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    if !(getIcosahedronFaces(invalid_0, faces) == E_FAILED as libc::c_int as libc::c_uint) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGetIcosahedronFaces.c\0" as *const u8 as *const libc::c_char,
            153 as libc::c_int,
            b"H3_EXPORT(getIcosahedronFaces)(invalid, faces) == E_FAILED\0" as *const u8
                as *const libc::c_char,
            b"Invalid cell\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    free(faces as *mut libc::c_void);
}
unsafe fn main_0() -> libc::c_int {
    currentSuiteName = b"getIcosahedronFaces\0" as *const u8 as *const libc::c_char;
    printf(
        b"TEST %s\n\0" as *const u8 as *const libc::c_char,
        b"getIcosahedronFaces\0" as *const u8 as *const libc::c_char,
    );
    runTests();
    printf(
        b"\nDONE: %d assertions\n\0" as *const u8 as *const libc::c_char,
        globalTestCount,
    );
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
