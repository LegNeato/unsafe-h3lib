extern crate unsafe_h3lib;
extern crate unsafe_h3lib_applib;
extern crate unsafe_h3lib_testapps_lib;
use ::libc;
extern "C" {

    fn exit(_: libc::c_int) -> !;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn gridDisk(origin: H3Index, k: libc::c_int, out: *mut H3Index) -> H3Error;
    fn cellToChildrenSize(h: H3Index, childRes: libc::c_int, out: *mut int64_t) -> H3Error;
    fn cellToChildren(h: H3Index, childRes: libc::c_int, children: *mut H3Index) -> H3Error;
    fn cellToCenterChild(h: H3Index, childRes: libc::c_int, child: *mut H3Index) -> H3Error;
    fn compactCells(
        h3Set: *const H3Index,
        compactedSet: *mut H3Index,
        numHexes: int64_t,
    ) -> H3Error;
    fn uncompactCellsSize(
        compactedSet: *const H3Index,
        numCompacted: int64_t,
        res: libc::c_int,
        out: *mut int64_t,
    ) -> H3Error;
    fn uncompactCells(
        compactedSet: *const H3Index,
        numCompacted: int64_t,
        outSet: *mut H3Index,
        numOut: int64_t,
        res: libc::c_int,
    ) -> H3Error;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn maxGridDiskSize(k: libc::c_int, out: *mut int64_t) -> H3Error;
    fn setH3Index(h: *mut H3Index, res: libc::c_int, baseCell: libc::c_int, initDigit: Direction);
    static mut globalTestCount: libc::c_int;
    static mut currentSuiteName: *const libc::c_char;
    static mut currentTestName: *const libc::c_char;
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_off_t = __int64_t;
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
pub static mut sunnyvale: H3Index = 0x89283470c27ffff as libc::c_long as H3Index;
#[no_mangle]
pub static mut uncompactable: [H3Index; 3] = [
    0x89283470803ffff as libc::c_long as H3Index,
    0x8928347081bffff as libc::c_long as H3Index,
    0x8928347080bffff as libc::c_long as H3Index,
];
#[no_mangle]
pub static mut uncompactableWithZero: [H3Index; 4] = [
    0x89283470803ffff as libc::c_long as H3Index,
    0x8928347081bffff as libc::c_long as H3Index,
    0 as libc::c_int as H3Index,
    0x8928347080bffff as libc::c_long as H3Index,
];
unsafe extern "C" fn runTests() {
    currentTestName = b"roundtrip\0" as *const u8 as *const libc::c_char;
    let mut k: libc::c_int = 9 as libc::c_int;
    let mut hexCount: int64_t = 0;
    if maxGridDiskSize(k, &mut hexCount) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int,
            b"!(maxGridDiskSize(k, &hexCount))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut expectedCompactCount: libc::c_int = 73 as libc::c_int;
    let mut sunnyvaleExpanded: *mut H3Index = calloc(
        hexCount as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if gridDisk(sunnyvale, k, sunnyvaleExpanded) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            39 as libc::c_int,
            b"!(gridDisk(sunnyvale, k, sunnyvaleExpanded))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut compressed: *mut H3Index = calloc(
        hexCount as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if compactCells(sunnyvaleExpanded, compressed, hexCount) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            43 as libc::c_int,
            b"!(compactCells(sunnyvaleExpanded, compressed, hexCount))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut i: int64_t = 0 as libc::c_int as int64_t;
    while i < hexCount {
        if *compressed.offset(i as isize) != 0 as libc::c_int as libc::c_ulonglong {
            count += 1;
        }
        i += 1;
    }
    if !(count == expectedCompactCount) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            b"count == expectedCompactCount\0" as *const u8 as *const libc::c_char,
            b"got expected compacted count\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut countUncompact: int64_t = 0;
    if uncompactCellsSize(
        compressed,
        count as int64_t,
        9 as libc::c_int,
        &mut countUncompact,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            55 as libc::c_int,
            b"!(uncompactCellsSize(compressed, count, 9, &countUncompact))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut decompressed: *mut H3Index = calloc(
        countUncompact as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if uncompactCells(
        compressed,
        count as int64_t,
        decompressed,
        hexCount,
        9 as libc::c_int,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            b"!(uncompactCells(compressed, count, decompressed, hexCount, 9))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut count2: libc::c_int = 0 as libc::c_int;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while (i_0 as libc::c_longlong) < hexCount {
        if *decompressed.offset(i_0 as isize) != 0 as libc::c_int as libc::c_ulonglong {
            count2 += 1;
        }
        i_0 += 1;
    }
    if !(count2 as libc::c_longlong == hexCount) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            66 as libc::c_int,
            b"count2 == hexCount\0" as *const u8 as *const libc::c_char,
            b"got expected uncompacted count\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    free(compressed as *mut libc::c_void);
    free(decompressed as *mut libc::c_void);
    free(sunnyvaleExpanded as *mut libc::c_void);
    currentTestName = b"res0children\0" as *const u8 as *const libc::c_char;
    let mut parent: H3Index = 0;
    setH3Index(
        &mut parent,
        0 as libc::c_int,
        0 as libc::c_int,
        CENTER_DIGIT,
    );
    let mut arrSize: int64_t = 0;
    if cellToChildrenSize(parent, 1 as libc::c_int, &mut arrSize) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            78 as libc::c_int,
            b"!(cellToChildrenSize(parent, 1, &arrSize))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut children: *mut H3Index = calloc(
        arrSize as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if cellToChildren(parent, 1 as libc::c_int, children) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            81 as libc::c_int,
            b"!(cellToChildren(parent, 1, children))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut compressed_0: *mut H3Index = calloc(
        arrSize as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if compactCells(children, compressed_0, arrSize) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            84 as libc::c_int,
            b"!(compactCells(children, compressed, arrSize))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(*compressed_0.offset(0 as libc::c_int as isize) == parent) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            85 as libc::c_int,
            b"compressed[0] == parent\0" as *const u8 as *const libc::c_char,
            b"got expected parent\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut idx: libc::c_int = 1 as libc::c_int;
    while (idx as libc::c_longlong) < arrSize {
        if !(*compressed_0.offset(idx as isize) == 0 as libc::c_int as libc::c_ulonglong) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
                87 as libc::c_int,
                b"compressed[idx] == 0\0" as *const u8 as *const libc::c_char,
                b"expected only 1 cell\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        idx += 1;
    }
    free(compressed_0 as *mut libc::c_void);
    free(children as *mut libc::c_void);
    currentTestName = b"res0\0" as *const u8 as *const libc::c_char;
    let mut hexCount_0: libc::c_int = 122 as libc::c_int;
    let mut res0Hexes: *mut H3Index = calloc(
        hexCount_0 as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 < hexCount_0 {
        setH3Index(
            &mut *res0Hexes.offset(i_1 as isize),
            0 as libc::c_int,
            i_1,
            CENTER_DIGIT,
        );
        i_1 += 1;
    }
    let mut compressed_1: *mut H3Index = calloc(
        hexCount_0 as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if compactCells(res0Hexes, compressed_1, hexCount_0 as int64_t) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            103 as libc::c_int,
            b"!(compactCells(res0Hexes, compressed, hexCount))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut i_2: libc::c_int = 0 as libc::c_int;
    while i_2 < hexCount_0 {
        if !(*compressed_1.offset(i_2 as isize) == *res0Hexes.offset(i_2 as isize)) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
                110 as libc::c_int,
                b"compressed[i] == res0Hexes[i]\0" as *const u8 as *const libc::c_char,
                b"got expected compressed result\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        i_2 += 1;
    }
    let mut countUncompact_0: int64_t = 0;
    if uncompactCellsSize(
        compressed_1,
        hexCount_0 as int64_t,
        0 as libc::c_int,
        &mut countUncompact_0,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            115 as libc::c_int,
            b"!(uncompactCellsSize(compressed, hexCount, 0, &countUncompact))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut decompressed_0: *mut H3Index = calloc(
        countUncompact_0 as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if uncompactCells(
        compressed_1,
        hexCount_0 as int64_t,
        decompressed_0,
        hexCount_0 as int64_t,
        0 as libc::c_int,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            b"!(uncompactCells(compressed, hexCount, decompressed, hexCount, 0))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut count2_0: libc::c_int = 0 as libc::c_int;
    let mut i_3: libc::c_int = 0 as libc::c_int;
    while i_3 < hexCount_0 {
        if *decompressed_0.offset(i_3 as isize) != 0 as libc::c_int as libc::c_ulonglong {
            count2_0 += 1;
        }
        i_3 += 1;
    }
    if !(count2_0 == hexCount_0) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            126 as libc::c_int,
            b"count2 == hexCount\0" as *const u8 as *const libc::c_char,
            b"got expected uncompacted count\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    free(res0Hexes as *mut libc::c_void);
    free(compressed_1 as *mut libc::c_void);
    free(decompressed_0 as *mut libc::c_void);
    currentTestName = b"uncompactable\0" as *const u8 as *const libc::c_char;
    let mut hexCount_1: libc::c_int = 3 as libc::c_int;
    let mut expectedCompactCount_0: libc::c_int = 3 as libc::c_int;
    let mut compressed_2: *mut H3Index = calloc(
        hexCount_1 as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if compactCells(
        uncompactable.as_mut_ptr(),
        compressed_2,
        hexCount_1 as int64_t,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            139 as libc::c_int,
            b"!(compactCells(uncompactable, compressed, hexCount))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut count_0: libc::c_int = 0 as libc::c_int;
    let mut i_4: libc::c_int = 0 as libc::c_int;
    while i_4 < hexCount_1 {
        if *compressed_2.offset(i_4 as isize) != 0 as libc::c_int as libc::c_ulonglong {
            count_0 += 1;
        }
        i_4 += 1;
    }
    if !(count_0 == expectedCompactCount_0) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            147 as libc::c_int,
            b"count == expectedCompactCount\0" as *const u8 as *const libc::c_char,
            b"got expected compacted count\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut countUncompact_1: int64_t = 0;
    if uncompactCellsSize(
        compressed_2,
        count_0 as int64_t,
        9 as libc::c_int,
        &mut countUncompact_1,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int,
            b"!(uncompactCellsSize(compressed, count, 9, &countUncompact))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut decompressed_1: *mut H3Index = calloc(
        countUncompact_1 as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if uncompactCells(
        compressed_2,
        count_0 as int64_t,
        decompressed_1,
        hexCount_1 as int64_t,
        9 as libc::c_int,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            154 as libc::c_int,
            b"!(uncompactCells(compressed, count, decompressed, hexCount, 9))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut count2_1: libc::c_int = 0 as libc::c_int;
    let mut i_5: libc::c_int = 0 as libc::c_int;
    while i_5 < hexCount_1 {
        if *decompressed_1.offset(i_5 as isize) != 0 as libc::c_int as libc::c_ulonglong {
            count2_1 += 1;
        }
        i_5 += 1;
    }
    if !(count2_1 == hexCount_1) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            162 as libc::c_int,
            b"count2 == hexCount\0" as *const u8 as *const libc::c_char,
            b"got expected uncompacted count\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    free(compressed_2 as *mut libc::c_void);
    free(decompressed_1 as *mut libc::c_void);
    currentTestName = b"compactCells_duplicate\0" as *const u8 as *const libc::c_char;
    let mut numHex: libc::c_int = 10 as libc::c_int;
    let mut someHexagons: [H3Index; 10] = [0 as libc::c_int as H3Index, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut i_6: libc::c_int = 0 as libc::c_int;
    while i_6 < numHex {
        setH3Index(
            &mut *someHexagons.as_mut_ptr().offset(i_6 as isize),
            5 as libc::c_int,
            0 as libc::c_int,
            J_AXES_DIGIT,
        );
        i_6 += 1;
    }
    let mut compressed_3: [H3Index; 10] = [0; 10];
    if !(compactCells(
        someHexagons.as_mut_ptr(),
        compressed_3.as_mut_ptr(),
        numHex as int64_t,
    ) == E_DUPLICATE_INPUT as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            178 as libc::c_int,
            b"H3_EXPORT(compactCells)(someHexagons, compressed, numHex) == E_DUPLICATE_INPUT\0"
                as *const u8 as *const libc::c_char,
            b"compactCells fails on duplicate input\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"compactCells_duplicateMinimum\0" as *const u8 as *const libc::c_char;
    let mut h3: H3Index = 0;
    let mut res: libc::c_int = 10 as libc::c_int;
    setH3Index(&mut h3, res, 0 as libc::c_int, J_AXES_DIGIT);
    let mut arrSize_0: int64_t = 0;
    if cellToChildrenSize(h3, res + 1 as libc::c_int, &mut arrSize_0) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            189 as libc::c_int,
            b"!(cellToChildrenSize(h3, res + 1, &arrSize))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    arrSize_0 += 1;
    let mut children_0: *mut H3Index = calloc(
        arrSize_0 as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if cellToChildren(h3, res + 1 as libc::c_int, children_0) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            193 as libc::c_int,
            b"!(cellToChildren(h3, res + 1, children))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    *children_0.offset((arrSize_0 - 1 as libc::c_int as libc::c_longlong) as isize) =
        *children_0.offset(0 as libc::c_int as isize);
    let mut output: *mut H3Index = calloc(
        arrSize_0 as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    let mut compactCellsResult: H3Error = compactCells(children_0, output, arrSize_0);
    if !(compactCellsResult == E_DUPLICATE_INPUT as libc::c_int as libc::c_uint) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            202 as libc::c_int,
            b"compactCellsResult == E_DUPLICATE_INPUT\0" as *const u8 as *const libc::c_char,
            b"compactCells fails on duplicate input (single duplicate)\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    free(output as *mut libc::c_void);
    free(children_0 as *mut libc::c_void);
    currentTestName = b"compactCells_duplicatePentagonLimit\0" as *const u8 as *const libc::c_char;
    let mut h3_0: H3Index = 0;
    let mut res_0: libc::c_int = 10 as libc::c_int;
    setH3Index(&mut h3_0, res_0, 4 as libc::c_int, CENTER_DIGIT);
    let mut arrSize_1: int64_t = 0;
    if cellToChildrenSize(h3_0, res_0 + 1 as libc::c_int, &mut arrSize_1) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            216 as libc::c_int,
            b"!(cellToChildrenSize(h3, res + 1, &arrSize))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    arrSize_1 += 1;
    let mut children_1: *mut H3Index = calloc(
        arrSize_1 as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if cellToChildren(h3_0, res_0 + 1 as libc::c_int, children_1) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            220 as libc::c_int,
            b"!(cellToChildren(h3, res + 1, children))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if cellToCenterChild(
        h3_0,
        res_0 + 1 as libc::c_int,
        &mut *children_1.offset((arrSize_1 - 1 as libc::c_int as libc::c_longlong) as isize),
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            223 as libc::c_int,
            b"!(cellToCenterChild(h3, res + 1, &children[arrSize - 1]))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut output_0: *mut H3Index = calloc(
        arrSize_1 as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    let mut compactCellsResult_0: H3Error = compactCells(children_1, output_0, arrSize_1);
    if !(compactCellsResult_0 == E_DUPLICATE_INPUT as libc::c_int as libc::c_uint) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            230 as libc::c_int,
            b"compactCellsResult == E_DUPLICATE_INPUT\0" as *const u8 as *const libc::c_char,
            b"compactCells fails on duplicate input (pentagon parent)\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    free(output_0 as *mut libc::c_void);
    free(children_1 as *mut libc::c_void);
    currentTestName = b"compactCells_duplicateIgnored\0" as *const u8 as *const libc::c_char;
    let mut h3_1: H3Index = 0;
    let mut res_1: libc::c_int = 10 as libc::c_int;
    setH3Index(&mut h3_1, res_1, 0 as libc::c_int, J_AXES_DIGIT);
    let mut arrSize_2: int64_t = 0;
    if cellToChildrenSize(h3_1, res_1 + 1 as libc::c_int, &mut arrSize_2) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            246 as libc::c_int,
            b"!(cellToChildrenSize(h3, res + 1, &arrSize))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut children_2: *mut H3Index = calloc(
        arrSize_2 as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if cellToChildren(h3_1, res_1 + 1 as libc::c_int, children_2) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            249 as libc::c_int,
            b"!(cellToChildren(h3, res + 1, children))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    *children_2.offset((arrSize_2 - 1 as libc::c_int as libc::c_longlong) as isize) =
        *children_2.offset(0 as libc::c_int as isize);
    let mut output_1: *mut H3Index = calloc(
        arrSize_2 as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if compactCells(children_2, output_1, arrSize_2) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            255 as libc::c_int,
            b"!(compactCells(children, output, arrSize))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    free(output_1 as *mut libc::c_void);
    free(children_2 as *mut libc::c_void);
    currentTestName = b"compactCells_empty\0" as *const u8 as *const libc::c_char;
    if !(compactCells(
        std::ptr::null::<H3Index>(),
        std::ptr::null_mut::<H3Index>(),
        0 as libc::c_int as int64_t,
    ) == E_SUCCESS as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            263 as libc::c_int,
            b"H3_EXPORT(compactCells)(NULL, NULL, 0) == E_SUCCESS\0" as *const u8
                as *const libc::c_char,
            b"compactCells succeeds on empty input\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"compactCells_disparate\0" as *const u8 as *const libc::c_char;
    let numHex_0: libc::c_int = 7 as libc::c_int;
    let mut disparate: [H3Index; 7] = [
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
    ];
    let mut i_7: libc::c_int = 0 as libc::c_int;
    while i_7 < numHex_0 {
        setH3Index(
            &mut *disparate.as_mut_ptr().offset(i_7 as isize),
            1 as libc::c_int,
            i_7,
            CENTER_DIGIT,
        );
        i_7 += 1;
    }
    let mut output_2: [H3Index; 7] = [
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
    ];
    if !(compactCells(
        disparate.as_mut_ptr(),
        output_2.as_mut_ptr(),
        numHex_0 as int64_t,
    ) == E_SUCCESS as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            278 as libc::c_int,
            b"H3_EXPORT(compactCells)(disparate, output, numHex) == E_SUCCESS\0" as *const u8
                as *const libc::c_char,
            b"compactCells succeeds on disparate input\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut i_8: libc::c_int = 0 as libc::c_int;
    while i_8 < numHex_0 {
        if !(disparate[i_8 as usize] == output_2[i_8 as usize]) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
                283 as libc::c_int,
                b"disparate[i] == output[i]\0" as *const u8 as *const libc::c_char,
                b"output set equals input set\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        i_8 += 1;
    }
    currentTestName = b"compactCells_reservedBitsSet\0" as *const u8 as *const libc::c_char;
    let numHex_1: libc::c_int = 7 as libc::c_int;
    let mut bad: [H3Index; 7] = [
        0x10000000010000 as libc::c_long as H3Index,
        0x180c6c6c6c61616 as libc::c_long as H3Index,
        0x1616ffffffffffff as libc::c_long as H3Index,
        0xffff8affffffffff as libc::c_ulong as H3Index,
        0xffffffffffffc6c6 as libc::c_ulong as H3Index,
        0xffffffffffffffc6 as libc::c_ulong as H3Index,
        0xc6c6c6c6c66fffe0 as libc::c_ulong as H3Index,
    ];
    let mut output_3: [H3Index; 7] = [
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
    ];
    if !(compactCells(bad.as_mut_ptr(), output_3.as_mut_ptr(), numHex_1 as int64_t)
        == E_CELL_INVALID as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            297 as libc::c_int,
            b"H3_EXPORT(compactCells)(bad, output, numHex) == E_CELL_INVALID\0" as *const u8
                as *const libc::c_char,
            b"compactCells returns E_CELL_INVALID on bad input\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"compactCells_parentError\0" as *const u8 as *const libc::c_char;
    let numHex_2: libc::c_int = 3 as libc::c_int;
    let mut bad_0: [H3Index; 3] = [
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
    ];
    let mut output_4: [H3Index; 3] = [
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
    ];
    bad_0[0 as libc::c_int as usize] = bad_0[0 as libc::c_int as usize]
        & !((15 as libc::c_ulonglong) << 52 as libc::c_int)
        | (10 as libc::c_int as uint64_t) << 52 as libc::c_int;
    bad_0[1 as libc::c_int as usize] = bad_0[1 as libc::c_int as usize]
        & !((15 as libc::c_ulonglong) << 52 as libc::c_int)
        | (5 as libc::c_int as uint64_t) << 52 as libc::c_int;
    if !(compactCells(
        bad_0.as_mut_ptr(),
        output_4.as_mut_ptr(),
        numHex_2 as int64_t,
    ) == E_RES_MISMATCH as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            309 as libc::c_int,
            b"H3_EXPORT(compactCells)(bad, output, numHex) == E_RES_MISMATCH\0" as *const u8
                as *const libc::c_char,
            b"compactCells returns E_RES_MISMATCH on bad input (parent error)\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"compactCells_parentError2\0" as *const u8 as *const libc::c_char;
    let numHex_3: libc::c_int = 43 as libc::c_int;
    let mut bad_1: [H3Index; 43] = [
        0x2010202020202020 as libc::c_long as H3Index,
        0x2100000000 as libc::c_long as H3Index,
        0x7 as libc::c_int as H3Index,
        0x400000000 as libc::c_long as H3Index,
        0x20000000 as libc::c_int as H3Index,
        0x5000000000 as libc::c_long as H3Index,
        0x100321 as libc::c_int as H3Index,
        0x2100000000 as libc::c_long as H3Index,
        0x7 as libc::c_int as H3Index,
        0x400000000 as libc::c_long as H3Index,
        0x20000000 as libc::c_int as H3Index,
        0x2100000000 as libc::c_long as H3Index,
        0x7 as libc::c_int as H3Index,
        0x400000000 as libc::c_long as H3Index,
        0x20000000 as libc::c_int as H3Index,
        0x5000000000 as libc::c_long as H3Index,
        0x100321 as libc::c_int as H3Index,
        0x20000000 as libc::c_int as H3Index,
        0x5000000000 as libc::c_long as H3Index,
        0x100321 as libc::c_int as H3Index,
        0x2100000000 as libc::c_long as H3Index,
        0x7 as libc::c_int as H3Index,
        0x400000000 as libc::c_long as H3Index,
        0x5000000000 as libc::c_long as H3Index,
        0x100321 as libc::c_int as H3Index,
        0x2100000000 as libc::c_long as H3Index,
        0x7 as libc::c_int as H3Index,
        0x400000000 as libc::c_long as H3Index,
        0x20000000 as libc::c_int as H3Index,
        0x5000000000 as libc::c_long as H3Index,
        0x100321 as libc::c_int as H3Index,
        0x2100000000 as libc::c_long as H3Index,
        0x7 as libc::c_int as H3Index,
        0x400000000 as libc::c_long as H3Index,
        0x20000000 as libc::c_int as H3Index,
        0x5000000000 as libc::c_long as H3Index,
        0x100321 as libc::c_int as H3Index,
        0x20000000 as libc::c_int as H3Index,
        0x5000000000 as libc::c_long as H3Index,
        0x100321 as libc::c_int as H3Index,
        0x2100000000 as libc::c_long as H3Index,
        0x7 as libc::c_int as H3Index,
        0x400000000 as libc::c_long as H3Index,
    ];
    let mut output_5: [H3Index; 43] = [
        0 as libc::c_int as H3Index,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    if !(compactCells(
        bad_1.as_mut_ptr(),
        output_5.as_mut_ptr(),
        numHex_3 as int64_t,
    ) == E_RES_DOMAIN as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            363 as libc::c_int,
            b"H3_EXPORT(compactCells)(bad, output, numHex) == E_RES_DOMAIN\0" as *const u8
                as *const libc::c_char,
            b"compactCells returns E_RES_DOMAIN on bad input (parent error #2)\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"uncompactCells_wrongRes\0" as *const u8 as *const libc::c_char;
    let mut numHex_4: libc::c_int = 3 as libc::c_int;
    let mut someHexagons_0: [H3Index; 3] = [
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
    ];
    let mut i_9: libc::c_int = 0 as libc::c_int;
    while i_9 < numHex_4 {
        setH3Index(
            &mut *someHexagons_0.as_mut_ptr().offset(i_9 as isize),
            5 as libc::c_int,
            i_9,
            CENTER_DIGIT,
        );
        i_9 += 1;
    }
    let mut sizeResult: int64_t = 0;
    if !(uncompactCellsSize(
        someHexagons_0.as_mut_ptr(),
        numHex_4 as int64_t,
        4 as libc::c_int,
        &mut sizeResult,
    ) == E_RES_MISMATCH as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8
                as *const libc::c_char,
            376 as libc::c_int,
            b"H3_EXPORT(uncompactCellsSize)(someHexagons, numHex, 4, &sizeResult) == E_RES_MISMATCH\0"
                as *const u8 as *const libc::c_char,
            b"uncompactCellsSize fails when given illogical resolutions\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(uncompactCellsSize(
        someHexagons_0.as_mut_ptr(),
        numHex_4 as int64_t,
        -(1 as libc::c_int),
        &mut sizeResult,
    ) == E_RES_MISMATCH as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8
                as *const libc::c_char,
            379 as libc::c_int,
            b"H3_EXPORT(uncompactCellsSize)(someHexagons, numHex, -1, &sizeResult) == E_RES_MISMATCH\0"
                as *const u8 as *const libc::c_char,
            b"uncompactCellsSize fails when given illegal resolutions\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(uncompactCellsSize(
        someHexagons_0.as_mut_ptr(),
        numHex_4 as int64_t,
        15 as libc::c_int + 1 as libc::c_int,
        &mut sizeResult,
    ) == E_RES_MISMATCH as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8
                as *const libc::c_char,
            383 as libc::c_int,
            b"H3_EXPORT(uncompactCellsSize)(someHexagons, numHex, MAX_H3_RES + 1, &sizeResult) == E_RES_MISMATCH\0"
                as *const u8 as *const libc::c_char,
            b"uncompactCellsSize fails when given resolutions beyond max\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut uncompressed: [H3Index; 3] = [
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
        0 as libc::c_int as H3Index,
    ];
    let mut uncompactCellsResult: H3Error = uncompactCells(
        someHexagons_0.as_mut_ptr(),
        numHex_4 as int64_t,
        uncompressed.as_mut_ptr(),
        numHex_4 as int64_t,
        0 as libc::c_int,
    );
    if !(uncompactCellsResult == E_RES_MISMATCH as libc::c_int as libc::c_uint) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            389 as libc::c_int,
            b"uncompactCellsResult == E_RES_MISMATCH\0" as *const u8 as *const libc::c_char,
            b"uncompactCells fails when given illogical resolutions\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    uncompactCellsResult = uncompactCells(
        someHexagons_0.as_mut_ptr(),
        numHex_4 as int64_t,
        uncompressed.as_mut_ptr(),
        numHex_4 as int64_t,
        6 as libc::c_int,
    );
    if !(uncompactCellsResult == E_MEMORY_BOUNDS as libc::c_int as libc::c_uint) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            393 as libc::c_int,
            b"uncompactCellsResult == E_MEMORY_BOUNDS\0" as *const u8 as *const libc::c_char,
            b"uncompactCells fails when given too little buffer\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    uncompactCellsResult = uncompactCells(
        someHexagons_0.as_mut_ptr(),
        numHex_4 as int64_t,
        uncompressed.as_mut_ptr(),
        (numHex_4 - 1 as libc::c_int) as int64_t,
        5 as libc::c_int,
    );
    if !(uncompactCellsResult == E_MEMORY_BOUNDS as libc::c_int as libc::c_uint) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            398 as libc::c_int,
            b"uncompactCellsResult == E_MEMORY_BOUNDS\0" as *const u8 as *const libc::c_char,
            b"uncompactCells fails when given too little buffer (same resolution)\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut i_10: libc::c_int = 0 as libc::c_int;
    while i_10 < numHex_4 {
        setH3Index(
            &mut *someHexagons_0.as_mut_ptr().offset(i_10 as isize),
            15 as libc::c_int,
            i_10,
            CENTER_DIGIT,
        );
        i_10 += 1;
    }
    uncompactCellsResult = uncompactCells(
        someHexagons_0.as_mut_ptr(),
        numHex_4 as int64_t,
        uncompressed.as_mut_ptr(),
        (numHex_4 * 7 as libc::c_int) as int64_t,
        15 as libc::c_int + 1 as libc::c_int,
    );
    if !(uncompactCellsResult == E_RES_MISMATCH as libc::c_int as libc::c_uint) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            406 as libc::c_int,
            b"uncompactCellsResult == E_RES_MISMATCH\0" as *const u8 as *const libc::c_char,
            b"uncompactCells fails when given resolutions beyond max\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"someHexagon\0" as *const u8 as *const libc::c_char;
    let mut origin: H3Index = 0;
    setH3Index(
        &mut origin,
        1 as libc::c_int,
        5 as libc::c_int,
        CENTER_DIGIT,
    );
    let mut childrenSz: int64_t = 0;
    if uncompactCellsSize(
        &mut origin,
        1 as libc::c_int as int64_t,
        2 as libc::c_int,
        &mut childrenSz,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            415 as libc::c_int,
            b"!(uncompactCellsSize(&origin, 1, 2, &childrenSz))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut children_3: *mut H3Index = calloc(
        childrenSz as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if uncompactCells(
        &mut origin,
        1 as libc::c_int as int64_t,
        children_3,
        childrenSz,
        2 as libc::c_int,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            418 as libc::c_int,
            b"!(uncompactCells(&origin, 1, children, childrenSz, 2))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut result: *mut H3Index = calloc(
        childrenSz as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if compactCells(children_3, result, childrenSz) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            421 as libc::c_int,
            b"!(compactCells(children, result, childrenSz))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut found: libc::c_int = 0 as libc::c_int;
    let mut i_11: libc::c_int = 0 as libc::c_int;
    while (i_11 as libc::c_longlong) < childrenSz {
        if *result.offset(i_11 as isize) != 0 as libc::c_int as libc::c_ulonglong {
            found += 1;
            if !(*result.offset(i_11 as isize) == origin) {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
                    427 as libc::c_int,
                    b"result[i] == origin\0" as *const u8 as *const libc::c_char,
                    b"compacted to correct origin\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
        }
        i_11 += 1;
    }
    if !(found == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            430 as libc::c_int,
            b"found == 1\0" as *const u8 as *const libc::c_char,
            b"compacted to a single hexagon\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    free(children_3 as *mut libc::c_void);
    free(result as *mut libc::c_void);
    currentTestName = b"uncompactCells_empty\0" as *const u8 as *const libc::c_char;
    let mut uncompactSz: int64_t = 0;
    if uncompactCellsSize(
        std::ptr::null::<H3Index>(),
        0 as libc::c_int as int64_t,
        0 as libc::c_int,
        &mut uncompactSz,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            439 as libc::c_int,
            b"!(uncompactCellsSize(((void *)0), 0, 0, &uncompactSz))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(uncompactSz == 0 as libc::c_int as libc::c_longlong) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            440 as libc::c_int,
            b"uncompactSz == 0\0" as *const u8 as *const libc::c_char,
            b"uncompactCellsSize accepts empty input\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(uncompactCells(
        std::ptr::null::<H3Index>(),
        0 as libc::c_int as int64_t,
        0 as *mut H3Index,
        0 as libc::c_int as int64_t,
        0 as libc::c_int,
    ) == E_SUCCESS as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            442 as libc::c_int,
            b"H3_EXPORT(uncompactCells)(NULL, 0, NULL, 0, 0) == E_SUCCESS\0" as *const u8
                as *const libc::c_char,
            b"uncompactCells accepts empty input\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"uncompactCells_onlyZero\0" as *const u8 as *const libc::c_char;
    let mut origin_0: H3Index = 0 as libc::c_int as H3Index;
    let mut childrenSz_0: int64_t = 0;
    if uncompactCellsSize(
        &mut origin_0,
        1 as libc::c_int as int64_t,
        2 as libc::c_int,
        &mut childrenSz_0,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            454 as libc::c_int,
            b"!(uncompactCellsSize(&origin, 1, 2, &childrenSz))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut children_4: *mut H3Index = calloc(
        childrenSz_0 as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if uncompactCells(
        &mut origin_0,
        1 as libc::c_int as int64_t,
        children_4,
        childrenSz_0,
        2 as libc::c_int,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            457 as libc::c_int,
            b"!(uncompactCells(&origin, 1, children, childrenSz, 2))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    free(children_4 as *mut libc::c_void);
    currentTestName = b"uncompactCells_withZero\0" as *const u8 as *const libc::c_char;
    let mut childrenSz_1: int64_t = 0;
    if uncompactCellsSize(
        uncompactableWithZero.as_mut_ptr(),
        4 as libc::c_int as int64_t,
        10 as libc::c_int,
        &mut childrenSz_1,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            468 as libc::c_int,
            b"!(uncompactCellsSize(uncompactableWithZero, 4, 10, &childrenSz))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut children_5: *mut H3Index = calloc(
        childrenSz_1 as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if uncompactCells(
        uncompactableWithZero.as_mut_ptr(),
        4 as libc::c_int as int64_t,
        children_5,
        childrenSz_1,
        10 as libc::c_int,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            471 as libc::c_int,
            b"!(uncompactCells(uncompactableWithZero, 4, children, childrenSz, 10))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut found_0: libc::c_int = 0 as libc::c_int;
    let mut i_12: libc::c_int = 0 as libc::c_int;
    while (i_12 as libc::c_longlong) < childrenSz_1 {
        if *children_5.offset(i_12 as isize) != 0 as libc::c_int as libc::c_ulonglong {
            found_0 += 1;
        }
        i_12 += 1;
    }
    if !(found_0 as libc::c_longlong == childrenSz_1) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            480 as libc::c_int,
            b"found == childrenSz\0" as *const u8 as *const libc::c_char,
            b"uncompacted with zero to expected number of hexagons\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    free(children_5 as *mut libc::c_void);
    currentTestName = b"pentagon\0" as *const u8 as *const libc::c_char;
    let mut pentagon: H3Index = 0;
    setH3Index(
        &mut pentagon,
        1 as libc::c_int,
        4 as libc::c_int,
        CENTER_DIGIT,
    );
    let mut childrenSz_2: int64_t = 0;
    if uncompactCellsSize(
        &mut pentagon,
        1 as libc::c_int as int64_t,
        2 as libc::c_int,
        &mut childrenSz_2,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            491 as libc::c_int,
            b"!(uncompactCellsSize(&pentagon, 1, 2, &childrenSz))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut children_6: *mut H3Index = calloc(
        childrenSz_2 as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if uncompactCells(
        &mut pentagon,
        1 as libc::c_int as int64_t,
        children_6,
        childrenSz_2,
        2 as libc::c_int,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            494 as libc::c_int,
            b"!(uncompactCells(&pentagon, 1, children, childrenSz, 2))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut result_0: *mut H3Index = calloc(
        childrenSz_2 as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    if compactCells(children_6, result_0, childrenSz_2) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            497 as libc::c_int,
            b"!(compactCells(children, result, childrenSz))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut found_1: libc::c_int = 0 as libc::c_int;
    let mut i_13: libc::c_int = 0 as libc::c_int;
    while (i_13 as libc::c_longlong) < childrenSz_2 {
        if *result_0.offset(i_13 as isize) != 0 as libc::c_int as libc::c_ulonglong {
            found_1 += 1;
            if !(*result_0.offset(i_13 as isize) == pentagon) {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
                    504 as libc::c_int,
                    b"result[i] == pentagon\0" as *const u8 as *const libc::c_char,
                    b"compacted to correct pentagon\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
        }
        i_13 += 1;
    }
    if !(found_1 == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            507 as libc::c_int,
            b"found == 1\0" as *const u8 as *const libc::c_char,
            b"compacted to a single pentagon\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    free(children_6 as *mut libc::c_void);
    free(result_0 as *mut libc::c_void);
    currentTestName = b"large_uncompact_size_hexagon\0" as *const u8 as *const libc::c_char;
    let mut cells: [H3Index; 1] = [0x806dfffffffffff as libc::c_long as H3Index];
    let mut res_2: libc::c_int = 15 as libc::c_int;
    let mut expected: int64_t = 4747561509943 as libc::c_long as int64_t;
    let mut out: int64_t = 0;
    if uncompactCellsSize(
        cells.as_mut_ptr(),
        1 as libc::c_int as int64_t,
        res_2,
        &mut out,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            519 as libc::c_int,
            b"!(uncompactCellsSize(cells, 1, res, &out))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(out == expected) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            521 as libc::c_int,
            b"out == expected\0" as *const u8 as *const libc::c_char,
            b"uncompactCells size needs 64 bit int\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"large_uncompact_size_pentagon\0" as *const u8 as *const libc::c_char;
    let mut cells_0: [H3Index; 1] = [0x8009fffffffffff as libc::c_long as H3Index];
    let mut res_3: libc::c_int = 15 as libc::c_int;
    let mut expected_0: int64_t = 3956301258286 as libc::c_long as int64_t;
    let mut out_0: int64_t = 0;
    if uncompactCellsSize(
        cells_0.as_mut_ptr(),
        1 as libc::c_int as int64_t,
        res_3,
        &mut out_0,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            530 as libc::c_int,
            b"!(uncompactCellsSize(cells, 1, res, &out))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(out_0 == expected_0) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCompactCells.c\0" as *const u8 as *const libc::c_char,
            532 as libc::c_int,
            b"out == expected\0" as *const u8 as *const libc::c_char,
            b"uncompactCells size needs 64 bit int\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0() -> libc::c_int {
    currentSuiteName = b"compactCells\0" as *const u8 as *const libc::c_char;
    printf(
        b"TEST %s\n\0" as *const u8 as *const libc::c_char,
        b"compactCells\0" as *const u8 as *const libc::c_char,
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
