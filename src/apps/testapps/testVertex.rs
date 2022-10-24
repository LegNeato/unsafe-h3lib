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
    fn cellToVertex(origin: H3Index, vertexNum: libc::c_int, out: *mut H3Index) -> H3Error;
    fn cellToVertexes(origin: H3Index, vertexes: *mut H3Index) -> H3Error;
    fn vertexToLatLng(vertex: H3Index, point: *mut LatLng) -> H3Error;
    fn isValidVertex(vertex: H3Index) -> libc::c_int;
    static mut globalTestCount: libc::c_int;
    static mut currentSuiteName: *const libc::c_char;
    static mut currentTestName: *const libc::c_char;
    fn vertexNumForDirection(origin: H3Index, direction: Direction) -> libc::c_int;
    fn directionForVertexNum(origin: H3Index, vertexNum: libc::c_int) -> Direction;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LatLng {
    pub lat: libc::c_double,
    pub lng: libc::c_double,
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
    currentTestName = b"vertexNumForDirection_hex\0" as *const u8 as *const libc::c_char;
    let mut origin: H3Index = 0x823d6ffffffffff as libc::c_long as H3Index;
    let mut vertexNums: [libc::c_int; 6] = [0 as libc::c_int, 0, 0, 0, 0, 0];
    let mut dir: Direction = K_AXES_DIGIT;
    while (dir as libc::c_uint) < NUM_DIGITS as libc::c_int as libc::c_uint {
        let mut vertexNum: libc::c_int = vertexNumForDirection(origin, dir);
        if !(vertexNum >= 0 as libc::c_int && vertexNum < 6 as libc::c_int) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testVertex.c\0" as *const u8 as *const libc::c_char,
                31 as libc::c_int,
                b"vertexNum >= 0 && vertexNum < NUM_HEX_VERTS\0" as *const u8
                    as *const libc::c_char,
                b"vertex number appears valid\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        if vertexNums[vertexNum as usize] != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testVertex.c\0" as *const u8 as *const libc::c_char,
                32 as libc::c_int,
                b"!vertexNums[vertexNum]\0" as *const u8 as *const libc::c_char,
                b"vertex number appears only once\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        vertexNums[vertexNum as usize] = 1 as libc::c_int;
        dir += 1;
    }
    currentTestName = b"vertexNumForDirection_pent\0" as *const u8 as *const libc::c_char;
    let mut pentagon: H3Index = 0x823007fffffffff as libc::c_long as H3Index;
    let mut vertexNums_0: [libc::c_int; 5] = [0 as libc::c_int, 0, 0, 0, 0];
    let mut dir_0: Direction = J_AXES_DIGIT;
    while (dir_0 as libc::c_uint) < NUM_DIGITS as libc::c_int as libc::c_uint {
        let mut vertexNum_0: libc::c_int = vertexNumForDirection(pentagon, dir_0);
        if !(vertexNum_0 >= 0 as libc::c_int && vertexNum_0 < 5 as libc::c_int) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testVertex.c\0" as *const u8 as *const libc::c_char,
                43 as libc::c_int,
                b"vertexNum >= 0 && vertexNum < NUM_PENT_VERTS\0" as *const u8
                    as *const libc::c_char,
                b"vertex number appears valid\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        if vertexNums_0[vertexNum_0 as usize] != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testVertex.c\0" as *const u8 as *const libc::c_char,
                44 as libc::c_int,
                b"!vertexNums[vertexNum]\0" as *const u8 as *const libc::c_char,
                b"vertex number appears only once\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        vertexNums_0[vertexNum_0 as usize] = 1 as libc::c_int;
        dir_0 += 1;
    }
    currentTestName = b"vertexNumForDirection_badDirections\0" as *const u8 as *const libc::c_char;
    let mut origin_0: H3Index = 0x823007fffffffff as libc::c_long as H3Index;
    if vertexNumForDirection(origin_0, CENTER_DIGIT) != -(1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVertex.c\0" as *const u8 as *const libc::c_char,
            54 as libc::c_int,
            b"vertexNumForDirection(origin, CENTER_DIGIT) == INVALID_VERTEX_NUM\0" as *const u8
                as *const libc::c_char,
            b"center digit should return invalid vertex\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if vertexNumForDirection(origin_0, INVALID_DIGIT) != -(1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVertex.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            b"vertexNumForDirection(origin, INVALID_DIGIT) == INVALID_VERTEX_NUM\0" as *const u8
                as *const libc::c_char,
            b"invalid digit should return invalid vertex\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut pentagon_0: H3Index = 0x823007fffffffff as libc::c_long as H3Index;
    if vertexNumForDirection(pentagon_0, K_AXES_DIGIT) != -(1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVertex.c\0" as *const u8 as *const libc::c_char,
            62 as libc::c_int,
            b"vertexNumForDirection(pentagon, K_AXES_DIGIT) == INVALID_VERTEX_NUM\0" as *const u8
                as *const libc::c_char,
            b"K direction on pentagon should return invalid vertex\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"directionForVertexNum_hex\0" as *const u8 as *const libc::c_char;
    let mut origin_1: H3Index = 0x823d6ffffffffff as libc::c_long as H3Index;
    let mut seenDirs: [bool; 7] = [
        0 as libc::c_int != 0,
        false,
        false,
        false,
        false,
        false,
        false,
    ];
    let mut vertexNum_1: libc::c_int = 0 as libc::c_int;
    while vertexNum_1 < 6 as libc::c_int {
        let mut dir_1: Direction = directionForVertexNum(origin_1, vertexNum_1);
        if !(dir_1 as libc::c_uint > 0 as libc::c_int as libc::c_uint
            && (dir_1 as libc::c_uint) < INVALID_DIGIT as libc::c_int as libc::c_uint)
        {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testVertex.c\0" as *const u8 as *const libc::c_char,
                70 as libc::c_int,
                b"dir > 0 && dir < INVALID_DIGIT\0" as *const u8 as *const libc::c_char,
                b"direction appears valid\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        if seenDirs[dir_1 as usize] {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testVertex.c\0" as *const u8 as *const libc::c_char,
                71 as libc::c_int,
                b"!seenDirs[dir]\0" as *const u8 as *const libc::c_char,
                b"direction appears only once\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        seenDirs[dir_1 as usize] = 1 as libc::c_int != 0;
        vertexNum_1 += 1;
    }
    currentTestName = b"directionForVertexNum_badVerts\0" as *const u8 as *const libc::c_char;
    let mut origin_2: H3Index = 0x823d6ffffffffff as libc::c_long as H3Index;
    if directionForVertexNum(origin_2, -(1 as libc::c_int)) as libc::c_uint
        != INVALID_DIGIT as libc::c_int as libc::c_uint
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVertex.c\0" as *const u8 as *const libc::c_char,
            80 as libc::c_int,
            b"directionForVertexNum(origin, -1) == INVALID_DIGIT\0" as *const u8
                as *const libc::c_char,
            b"negative vertex should return invalid direction\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if directionForVertexNum(origin_2, 6 as libc::c_int) as libc::c_uint
        != INVALID_DIGIT as libc::c_int as libc::c_uint
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVertex.c\0" as *const u8 as *const libc::c_char,
            82 as libc::c_int,
            b"directionForVertexNum(origin, 6) == INVALID_DIGIT\0" as *const u8
                as *const libc::c_char,
            b"invalid vertex should return invalid direction\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut pentagon_1: H3Index = 0x823007fffffffff as libc::c_long as H3Index;
    if directionForVertexNum(pentagon_1, 5 as libc::c_int) as libc::c_uint
        != INVALID_DIGIT as libc::c_int as libc::c_uint
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVertex.c\0" as *const u8 as *const libc::c_char,
            86 as libc::c_int,
            b"directionForVertexNum(pentagon, 5) == INVALID_DIGIT\0" as *const u8
                as *const libc::c_char,
            b"invalid pent vertex should return invalid direction\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"cellToVertex_badVerts\0" as *const u8 as *const libc::c_char;
    let mut origin_3: H3Index = 0x823d6ffffffffff as libc::c_long as H3Index;
    let mut vert: H3Index = 0;
    if cellToVertex(origin_3, -(1 as libc::c_int), &mut vert)
        != E_DOMAIN as libc::c_int as libc::c_uint
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVertex.c\0" as *const u8 as *const libc::c_char,
            94 as libc::c_int,
            b"H3_EXPORT(cellToVertex)(origin, -1, &vert) == E_DOMAIN\0" as *const u8
                as *const libc::c_char,
            b"negative vertex should return null index\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if cellToVertex(origin_3, 6 as libc::c_int, &mut vert)
        != E_DOMAIN as libc::c_int as libc::c_uint
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVertex.c\0" as *const u8 as *const libc::c_char,
            96 as libc::c_int,
            b"H3_EXPORT(cellToVertex)(origin, 6, &vert) == E_DOMAIN\0" as *const u8
                as *const libc::c_char,
            b"invalid vertex should return null index\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut pentagon_2: H3Index = 0x823007fffffffff as libc::c_long as H3Index;
    if cellToVertex(pentagon_2, 5 as libc::c_int, &mut vert)
        != E_DOMAIN as libc::c_int as libc::c_uint
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVertex.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            b"H3_EXPORT(cellToVertex)(pentagon, 5, &vert) == E_DOMAIN\0" as *const u8
                as *const libc::c_char,
            b"invalid pent vertex should return null index\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"cellToVertex_invalid\0" as *const u8 as *const libc::c_char;
    let mut invalid: H3Index = 0xffffffffffffffff as libc::c_ulong as H3Index;
    let mut vert_0: H3Index = 0;
    if cellToVertex(invalid, 3 as libc::c_int, &mut vert_0)
        != E_FAILED as libc::c_int as libc::c_uint
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVertex.c\0" as *const u8 as *const libc::c_char,
            107 as libc::c_int,
            b"H3_EXPORT(cellToVertex)(invalid, 3, &vert) == E_FAILED\0" as *const u8
                as *const libc::c_char,
            b"Invalid cell returns error\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"cellToVertex_invalid2\0" as *const u8 as *const libc::c_char;
    let mut index: H3Index = 0x685b2396e900fff9 as libc::c_long as H3Index;
    let mut vert_1: H3Index = 0;
    if cellToVertex(index, 2 as libc::c_int, &mut vert_1)
        != E_CELL_INVALID as libc::c_int as libc::c_uint
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVertex.c\0" as *const u8 as *const libc::c_char,
            114 as libc::c_int,
            b"H3_EXPORT(cellToVertex)(index, 2, &vert) == E_CELL_INVALID\0" as *const u8
                as *const libc::c_char,
            b"Invalid cell returns error\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"cellToVertex_invalid3\0" as *const u8 as *const libc::c_char;
    let mut index_0: H3Index = 0x20ff20202020ff35 as libc::c_long as H3Index;
    let mut vert_2: H3Index = 0;
    if cellToVertex(index_0, 0 as libc::c_int, &mut vert_2)
        != E_CELL_INVALID as libc::c_int as libc::c_uint
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVertex.c\0" as *const u8 as *const libc::c_char,
            121 as libc::c_int,
            b"H3_EXPORT(cellToVertex)(index, 0, &vert) == E_CELL_INVALID\0" as *const u8
                as *const libc::c_char,
            b"Invalid cell returns error\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"isValidVertex_hex\0" as *const u8 as *const libc::c_char;
    let mut origin_4: H3Index = 0x823d6ffffffffff as libc::c_long as H3Index;
    let mut vert_3: H3Index = 0x2222597fffffffff as libc::c_long as H3Index;
    if isValidVertex(vert_3) == 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVertex.c\0" as *const u8 as *const libc::c_char,
            128 as libc::c_int,
            b"H3_EXPORT(isValidVertex)(vert)\0" as *const u8 as *const libc::c_char,
            b"known vertex is valid\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 6 as libc::c_int {
        if cellToVertex(origin_4, i, &mut vert_3) != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testVertex.c\0" as *const u8 as *const libc::c_char,
                131 as libc::c_int,
                b"!(cellToVertex(origin, i, &vert))\0" as *const u8 as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        if isValidVertex(vert_3) == 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testVertex.c\0" as *const u8 as *const libc::c_char,
                132 as libc::c_int,
                b"H3_EXPORT(isValidVertex)(vert)\0" as *const u8 as *const libc::c_char,
                b"vertex is valid\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        i += 1;
    }
    currentTestName = b"isValidVertex_invalidOwner\0" as *const u8 as *const libc::c_char;
    let mut origin_5: H3Index = 0x823d6ffffffffff as libc::c_long as H3Index;
    let mut vertexNum_2: libc::c_int = 0 as libc::c_int;
    let mut vert_4: H3Index = 0;
    if cellToVertex(origin_5, vertexNum_2, &mut vert_4) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVertex.c\0" as *const u8 as *const libc::c_char,
            140 as libc::c_int,
            b"!(cellToVertex(origin, vertexNum, &vert))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    vert_4 ^= 1 as libc::c_int as libc::c_ulonglong;
    if isValidVertex(vert_4) != 0 as libc::c_int {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVertex.c\0" as *const u8 as *const libc::c_char,
            146 as libc::c_int,
            b"H3_EXPORT(isValidVertex)(vert) == 0\0" as *const u8 as *const libc::c_char,
            b"vertex with invalid owner is not valid\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"isValidVertex_wrongOwner\0" as *const u8 as *const libc::c_char;
    let mut origin_6: H3Index = 0x823d6ffffffffff as libc::c_long as H3Index;
    let mut vertexNum_3: libc::c_int = 0 as libc::c_int;
    let mut vert_5: H3Index = 0;
    if cellToVertex(origin_6, vertexNum_3, &mut vert_5) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVertex.c\0" as *const u8 as *const libc::c_char,
            153 as libc::c_int,
            b"!(cellToVertex(origin, vertexNum, &vert))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut owner: H3Index = vert_5;
    owner = owner & !((15 as libc::c_int as uint64_t) << 59 as libc::c_int)
        | (1 as libc::c_int as uint64_t) << 59 as libc::c_int;
    owner = owner & !((7 as libc::c_int as uint64_t) << 56 as libc::c_int)
        | (0 as libc::c_int as uint64_t) << 56 as libc::c_int;
    if origin_6 == owner {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVertex.c\0" as *const u8 as *const libc::c_char,
            160 as libc::c_int,
            b"origin != owner\0" as *const u8 as *const libc::c_char,
            b"origin does not own the canonical vertex\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut nonCanonicalVertex: H3Index = origin_6;
    nonCanonicalVertex = nonCanonicalVertex
        & !((15 as libc::c_int as uint64_t) << 59 as libc::c_int)
        | (4 as libc::c_int as uint64_t) << 59 as libc::c_int;
    nonCanonicalVertex = nonCanonicalVertex
        & !((7 as libc::c_int as uint64_t) << 56 as libc::c_int)
        | (vertexNum_3 as uint64_t) << 56 as libc::c_int;
    if isValidVertex(nonCanonicalVertex) != 0 as libc::c_int {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVertex.c\0" as *const u8 as *const libc::c_char,
            167 as libc::c_int,
            b"H3_EXPORT(isValidVertex)(nonCanonicalVertex) == 0\0" as *const u8
                as *const libc::c_char,
            b"vertex with incorrect owner is not valid\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"isValidVertex_badVerts\0" as *const u8 as *const libc::c_char;
    let mut origin_7: H3Index = 0x823d6ffffffffff as libc::c_long as H3Index;
    if isValidVertex(origin_7) != 0 as libc::c_int {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVertex.c\0" as *const u8 as *const libc::c_char,
            172 as libc::c_int,
            b"H3_EXPORT(isValidVertex)(origin) == 0\0" as *const u8 as *const libc::c_char,
            b"cell is not valid\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut fakeEdge: H3Index = origin_7;
    fakeEdge = fakeEdge & !((15 as libc::c_int as uint64_t) << 59 as libc::c_int)
        | (2 as libc::c_int as uint64_t) << 59 as libc::c_int;
    if isValidVertex(fakeEdge) != 0 as libc::c_int {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVertex.c\0" as *const u8 as *const libc::c_char,
            177 as libc::c_int,
            b"H3_EXPORT(isValidVertex)(fakeEdge) == 0\0" as *const u8 as *const libc::c_char,
            b"edge mode is not valid\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut vert_6: H3Index = 0;
    if cellToVertex(origin_7, 0 as libc::c_int, &mut vert_6) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVertex.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
            b"!(cellToVertex(origin, 0, &vert))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    vert_6 = vert_6 & !((7 as libc::c_int as uint64_t) << 56 as libc::c_int)
        | (6 as libc::c_int as uint64_t) << 56 as libc::c_int;
    if isValidVertex(vert_6) != 0 as libc::c_int {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVertex.c\0" as *const u8 as *const libc::c_char,
            183 as libc::c_int,
            b"H3_EXPORT(isValidVertex)(vert) == 0\0" as *const u8 as *const libc::c_char,
            b"invalid vertexNum is not valid\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut pentagon_3: H3Index = 0x823007fffffffff as libc::c_long as H3Index;
    let mut vert2: H3Index = 0;
    if cellToVertex(pentagon_3, 0 as libc::c_int, &mut vert2) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVertex.c\0" as *const u8 as *const libc::c_char,
            187 as libc::c_int,
            b"!(cellToVertex(pentagon, 0, &vert2))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    vert2 = vert2 & !((7 as libc::c_int as uint64_t) << 56 as libc::c_int)
        | (5 as libc::c_int as uint64_t) << 56 as libc::c_int;
    if isValidVertex(vert2) != 0 as libc::c_int {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVertex.c\0" as *const u8 as *const libc::c_char,
            190 as libc::c_int,
            b"H3_EXPORT(isValidVertex)(vert2) == 0\0" as *const u8 as *const libc::c_char,
            b"invalid pentagon vertexNum is not valid\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"vertexToLatLng_invalid\0" as *const u8 as *const libc::c_char;
    let mut invalid_0: H3Index = 0xffffffffffffffff as libc::c_ulong as H3Index;
    let mut latLng: LatLng = LatLng { lat: 0., lng: 0. };
    if vertexToLatLng(invalid_0, &mut latLng) == E_SUCCESS as libc::c_int as libc::c_uint {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVertex.c\0" as *const u8 as *const libc::c_char,
            197 as libc::c_int,
            b"H3_EXPORT(vertexToLatLng)(invalid, &latLng) != E_SUCCESS\0" as *const u8
                as *const libc::c_char,
            b"Invalid vertex returns error\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"cellToVertexes_invalid\0" as *const u8 as *const libc::c_char;
    let mut invalid_1: H3Index = 0xffffffffffffffff as libc::c_ulong as H3Index;
    let mut verts: [H3Index; 6] = [0 as libc::c_int as H3Index, 0, 0, 0, 0, 0];
    if cellToVertexes(invalid_1, verts.as_mut_ptr()) != E_FAILED as libc::c_int as libc::c_uint {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVertex.c\0" as *const u8 as *const libc::c_char,
            204 as libc::c_int,
            b"H3_EXPORT(cellToVertexes)(invalid, verts) == E_FAILED\0" as *const u8
                as *const libc::c_char,
            b"cellToVertexes fails for invalid cell\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0() -> libc::c_int {
    currentSuiteName = b"Vertex\0" as *const u8 as *const libc::c_char;
    printf(
        b"TEST %s\n\0" as *const u8 as *const libc::c_char,
        b"Vertex\0" as *const u8 as *const libc::c_char,
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
