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
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn cellsToDirectedEdge(origin: H3Index, destination: H3Index, out: *mut H3Index) -> H3Error;
    fn gridDistance(origin: H3Index, h3: H3Index, distance: *mut int64_t) -> H3Error;
    fn ijkDistance(a: *const CoordIJK, b: *const CoordIJK) -> libc::c_int;
    fn setH3Index(h: *mut H3Index, res: libc::c_int, baseCell: libc::c_int, initDigit: Direction);
    static mut globalTestCount: libc::c_int;
    static mut currentSuiteName: *const libc::c_char;
    static mut currentTestName: *const libc::c_char;
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_off_t = __int64_t;
pub type int64_t = libc::c_longlong;
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
unsafe extern "C" fn runTests() {
    let mut bc1: H3Index = 35184372088831 as libc::c_ulonglong;
    setH3Index(&mut bc1, 0 as libc::c_int, 15 as libc::c_int, CENTER_DIGIT);
    let mut bc2: H3Index = 35184372088831 as libc::c_ulonglong;
    setH3Index(&mut bc2, 0 as libc::c_int, 8 as libc::c_int, CENTER_DIGIT);
    let mut bc3: H3Index = 35184372088831 as libc::c_ulonglong;
    setH3Index(&mut bc3, 0 as libc::c_int, 31 as libc::c_int, CENTER_DIGIT);
    let mut pent1: H3Index = 35184372088831 as libc::c_ulonglong;
    setH3Index(&mut pent1, 0 as libc::c_int, 4 as libc::c_int, CENTER_DIGIT);
    currentTestName = b"testIndexDistance\0" as *const u8 as *const libc::c_char;
    let mut bc: H3Index = 0 as libc::c_int as H3Index;
    setH3Index(&mut bc, 1 as libc::c_int, 17 as libc::c_int, CENTER_DIGIT);
    let mut p: H3Index = 0 as libc::c_int as H3Index;
    setH3Index(&mut p, 1 as libc::c_int, 14 as libc::c_int, CENTER_DIGIT);
    let mut p2: H3Index = 0;
    setH3Index(&mut p2, 1 as libc::c_int, 14 as libc::c_int, J_AXES_DIGIT);
    let mut p3: H3Index = 0;
    setH3Index(&mut p3, 1 as libc::c_int, 14 as libc::c_int, JK_AXES_DIGIT);
    let mut p4: H3Index = 0;
    setH3Index(&mut p4, 1 as libc::c_int, 14 as libc::c_int, I_AXES_DIGIT);
    let mut p5: H3Index = 0;
    setH3Index(&mut p5, 1 as libc::c_int, 14 as libc::c_int, IK_AXES_DIGIT);
    let mut p6: H3Index = 0;
    setH3Index(&mut p6, 1 as libc::c_int, 14 as libc::c_int, IJ_AXES_DIGIT);
    let mut distance: int64_t = 0;
    if gridDistance(bc, p, &mut distance) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            67 as libc::c_int,
            b"!(gridDistance(bc, p, &distance))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if distance != 3 as libc::c_int as libc::c_longlong {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            68 as libc::c_int,
            b"distance == 3\0" as *const u8 as *const libc::c_char,
            b"distance onto pentagon\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if gridDistance(bc, p2, &mut distance) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            69 as libc::c_int,
            b"!(gridDistance(bc, p2, &distance))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if distance != 2 as libc::c_int as libc::c_longlong {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            70 as libc::c_int,
            b"distance == 2\0" as *const u8 as *const libc::c_char,
            b"distance onto p2\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if gridDistance(bc, p3, &mut distance) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            71 as libc::c_int,
            b"!(gridDistance(bc, p3, &distance))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if distance != 3 as libc::c_int as libc::c_longlong {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            72 as libc::c_int,
            b"distance == 3\0" as *const u8 as *const libc::c_char,
            b"distance onto p3\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if gridDistance(bc, p6, &mut distance) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            79 as libc::c_int,
            b"!(gridDistance(bc, p6, &distance))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if distance != 2 as libc::c_int as libc::c_longlong {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            80 as libc::c_int,
            b"distance == 2\0" as *const u8 as *const libc::c_char,
            b"distance onto p6\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"testIndexDistance2\0" as *const u8 as *const libc::c_char;
    let mut origin: H3Index = 0x820c4ffffffffff as libc::c_long as H3Index;
    let mut destination: H3Index = 0x821ce7fffffffff as libc::c_long as H3Index;
    let mut distance_0: int64_t = 0;
    if gridDistance(destination, origin, &mut distance_0)
        == E_SUCCESS as libc::c_int as libc::c_uint
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
            b"H3_EXPORT(gridDistance)(destination, origin, &distance) != E_SUCCESS\0" as *const u8
                as *const libc::c_char,
            b"distance in res 2 across pentagon\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if gridDistance(origin, destination, &mut distance_0)
        == E_SUCCESS as libc::c_int as libc::c_uint
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            95 as libc::c_int,
            b"H3_EXPORT(gridDistance)(origin, destination, &distance) != E_SUCCESS\0" as *const u8
                as *const libc::c_char,
            b"distance in res 2 across pentagon (reversed)\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"gridDistanceBaseCells\0" as *const u8 as *const libc::c_char;
    let mut distance_1: int64_t = 0;
    if gridDistance(bc1, pent1, &mut distance_1) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            b"!(gridDistance(bc1, pent1, &distance))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if distance_1 != 1 as libc::c_int as libc::c_longlong {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            101 as libc::c_int,
            b"distance == 1\0" as *const u8 as *const libc::c_char,
            b"distance to neighbor is 1 (15, 4)\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if gridDistance(bc1, bc2, &mut distance_1) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            102 as libc::c_int,
            b"!(gridDistance(bc1, bc2, &distance))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if distance_1 != 1 as libc::c_int as libc::c_longlong {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            103 as libc::c_int,
            b"distance == 1\0" as *const u8 as *const libc::c_char,
            b"distance to neighbor is 1 (15, 8)\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if gridDistance(bc1, bc3, &mut distance_1) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            104 as libc::c_int,
            b"!(gridDistance(bc1, bc3, &distance))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if distance_1 != 1 as libc::c_int as libc::c_longlong {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            105 as libc::c_int,
            b"distance == 1\0" as *const u8 as *const libc::c_char,
            b"distance to neighbor is 1 (15, 31)\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if gridDistance(pent1, bc3, &mut distance_1) == E_SUCCESS as libc::c_int as libc::c_uint {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            107 as libc::c_int,
            b"H3_EXPORT(gridDistance)(pent1, bc3, &distance) != E_SUCCESS\0" as *const u8
                as *const libc::c_char,
            b"distance to neighbor is invalid\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"ijkDistance\0" as *const u8 as *const libc::c_char;
    let mut z: CoordIJK = {
        CoordIJK {
            i: 0 as libc::c_int,
            j: 0 as libc::c_int,
            k: 0 as libc::c_int,
        }
    };
    let mut i: CoordIJK = {
        CoordIJK {
            i: 1 as libc::c_int,
            j: 0 as libc::c_int,
            k: 0 as libc::c_int,
        }
    };
    let mut ik: CoordIJK = {
        CoordIJK {
            i: 1 as libc::c_int,
            j: 0 as libc::c_int,
            k: 1 as libc::c_int,
        }
    };
    let mut ij: CoordIJK = {
        CoordIJK {
            i: 1 as libc::c_int,
            j: 1 as libc::c_int,
            k: 0 as libc::c_int,
        }
    };
    let mut j2: CoordIJK = {
        CoordIJK {
            i: 0 as libc::c_int,
            j: 2 as libc::c_int,
            k: 0 as libc::c_int,
        }
    };
    if ijkDistance(&mut z, &mut z) != 0 as libc::c_int {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            117 as libc::c_int,
            b"ijkDistance(&z, &z) == 0\0" as *const u8 as *const libc::c_char,
            b"identity distance 0,0,0\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if ijkDistance(&mut i, &mut i) != 0 as libc::c_int {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            b"ijkDistance(&i, &i) == 0\0" as *const u8 as *const libc::c_char,
            b"identity distance 1,0,0\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if ijkDistance(&mut ik, &mut ik) != 0 as libc::c_int {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            119 as libc::c_int,
            b"ijkDistance(&ik, &ik) == 0\0" as *const u8 as *const libc::c_char,
            b"identity distance 1,0,1\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if ijkDistance(&mut ij, &mut ij) != 0 as libc::c_int {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            120 as libc::c_int,
            b"ijkDistance(&ij, &ij) == 0\0" as *const u8 as *const libc::c_char,
            b"identity distance 1,1,0\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if ijkDistance(&mut j2, &mut j2) != 0 as libc::c_int {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            121 as libc::c_int,
            b"ijkDistance(&j2, &j2) == 0\0" as *const u8 as *const libc::c_char,
            b"identity distance 0,2,0\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if ijkDistance(&mut z, &mut i) != 1 as libc::c_int {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            123 as libc::c_int,
            b"ijkDistance(&z, &i) == 1\0" as *const u8 as *const libc::c_char,
            b"0,0,0 to 1,0,0\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if ijkDistance(&mut z, &mut j2) != 2 as libc::c_int {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            124 as libc::c_int,
            b"ijkDistance(&z, &j2) == 2\0" as *const u8 as *const libc::c_char,
            b"0,0,0 to 0,2,0\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if ijkDistance(&mut z, &mut ik) != 1 as libc::c_int {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            125 as libc::c_int,
            b"ijkDistance(&z, &ik) == 1\0" as *const u8 as *const libc::c_char,
            b"0,0,0 to 1,0,1\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if ijkDistance(&mut i, &mut ik) != 1 as libc::c_int {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_int,
            b"ijkDistance(&i, &ik) == 1\0" as *const u8 as *const libc::c_char,
            b"1,0,0 to 1,0,1\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if ijkDistance(&mut ik, &mut j2) != 3 as libc::c_int {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            127 as libc::c_int,
            b"ijkDistance(&ik, &j2) == 3\0" as *const u8 as *const libc::c_char,
            b"1,0,1 to 0,2,0\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if ijkDistance(&mut ij, &mut ik) != 2 as libc::c_int {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            128 as libc::c_int,
            b"ijkDistance(&ij, &ik) == 2\0" as *const u8 as *const libc::c_char,
            b"1,0,1 to 1,1,0\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"gridDistanceResolutionMismatch\0" as *const u8 as *const libc::c_char;
    let mut distance_2: int64_t = 0;
    if gridDistance(
        0x832830fffffffff as libc::c_long as H3Index,
        0x822837fffffffff as libc::c_long as H3Index,
        &mut distance_2,
    ) != E_RES_MISMATCH as libc::c_int as libc::c_uint
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8
                as *const libc::c_char,
            135 as libc::c_int,
            b"H3_EXPORT(gridDistance)(0x832830fffffffffL, 0x822837fffffffffL, &distance) == E_RES_MISMATCH\0"
                as *const u8 as *const libc::c_char,
            b"cannot compare at different resolutions\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"gridDistanceEdge\0" as *const u8 as *const libc::c_char;
    let mut origin_0: H3Index = 0x832830fffffffff as libc::c_long as H3Index;
    let mut dest: H3Index = 0x832834fffffffff as libc::c_long as H3Index;
    let mut edge: H3Index = 0;
    if cellsToDirectedEdge(origin_0, dest, &mut edge) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            142 as libc::c_int,
            b"!(cellsToDirectedEdge(origin, dest, &edge))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if 0 as libc::c_int as libc::c_ulonglong == edge {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            b"0 != edge\0" as *const u8 as *const libc::c_char,
            b"test edge is valid\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut distance_3: int64_t = 0;
    if gridDistance(edge, origin_0, &mut distance_3) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            146 as libc::c_int,
            b"!(gridDistance(edge, origin, &distance))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if distance_3 != 0 as libc::c_int as libc::c_longlong {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            b"distance == 0\0" as *const u8 as *const libc::c_char,
            b"edge has zero distance to origin\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if gridDistance(origin_0, edge, &mut distance_3) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            148 as libc::c_int,
            b"!(gridDistance(origin, edge, &distance))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if distance_3 != 0 as libc::c_int as libc::c_longlong {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            149 as libc::c_int,
            b"distance == 0\0" as *const u8 as *const libc::c_char,
            b"origin has zero distance to edge\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if gridDistance(edge, dest, &mut distance_3) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int,
            b"!(gridDistance(edge, dest, &distance))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if distance_3 != 1 as libc::c_int as libc::c_longlong {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            152 as libc::c_int,
            b"distance == 1\0" as *const u8 as *const libc::c_char,
            b"edge has distance to destination\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if gridDistance(dest, edge, &mut distance_3) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            153 as libc::c_int,
            b"!(gridDistance(dest, edge, &distance))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if distance_3 != 1 as libc::c_int as libc::c_longlong {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            154 as libc::c_int,
            b"distance == 1\0" as *const u8 as *const libc::c_char,
            b"destination has distance to edge\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"gridDistanceInvalid\0" as *const u8 as *const libc::c_char;
    let mut invalid: H3Index = 0xffffffffffffffff as libc::c_ulong as H3Index;
    let mut distance_4: int64_t = 0;
    if gridDistance(invalid, invalid, &mut distance_4)
        != E_CELL_INVALID as libc::c_int as libc::c_uint
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            162 as libc::c_int,
            b"H3_EXPORT(gridDistance)(invalid, invalid, &distance) == E_CELL_INVALID\0" as *const u8
                as *const libc::c_char,
            b"distance from invalid cell\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if gridDistance(bc1, invalid, &mut distance_4) != E_RES_MISMATCH as libc::c_int as libc::c_uint
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testGridDistance.c\0" as *const u8 as *const libc::c_char,
            166 as libc::c_int,
            b"H3_EXPORT(gridDistance)(bc1, invalid, &distance) == E_RES_MISMATCH\0" as *const u8
                as *const libc::c_char,
            b"distance to invalid cell\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0() -> libc::c_int {
    currentSuiteName = b"gridDistance\0" as *const u8 as *const libc::c_char;
    printf(
        b"TEST %s\n\0" as *const u8 as *const libc::c_char,
        b"gridDistance\0" as *const u8 as *const libc::c_char,
    );
    runTests();
    printf(
        b"\nDONE: %d assertions\n\0" as *const u8 as *const libc::c_char,
        globalTestCount,
    );
    0 as libc::c_int
}
pub fn main() {
    unsafe { ::std::process::exit(main_0()) }
}
