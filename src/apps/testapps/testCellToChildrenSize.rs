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

    fn exit(_: libc::c_int) -> !;
    fn cellToChildrenSize(h: H3Index, childRes: libc::c_int, out: *mut int64_t) -> H3Error;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    static mut currentTestName: *const libc::c_char;
    static mut currentSuiteName: *const libc::c_char;
    static mut globalTestCount: libc::c_int;
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
unsafe extern "C" fn runTests() {
    currentTestName = b"cellToChildrenSize_hexagon\0" as *const u8 as *const libc::c_char;
    let mut h: H3Index = 0x87283080dffffff as libc::c_long as H3Index;
    let mut sz: int64_t = 0;
    if cellToChildrenSize(h, 3 as libc::c_int, &mut sz)
        != E_RES_DOMAIN as libc::c_int as libc::c_uint
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToChildrenSize.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            b"H3_EXPORT(cellToChildrenSize)(h, 3, &sz) == E_RES_DOMAIN\0" as *const u8
                as *const libc::c_char,
            b"got expected size for coarser res\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if cellToChildrenSize(h, 7 as libc::c_int, &mut sz) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToChildrenSize.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            b"!(cellToChildrenSize(h, 7, &sz))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if sz != 1 as libc::c_int as libc::c_longlong {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToChildrenSize.c\0" as *const u8 as *const libc::c_char,
            30 as libc::c_int,
            b"sz == 1\0" as *const u8 as *const libc::c_char,
            b"got expected size for same res\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if cellToChildrenSize(h, 8 as libc::c_int, &mut sz) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToChildrenSize.c\0" as *const u8 as *const libc::c_char,
            31 as libc::c_int,
            b"!(cellToChildrenSize(h, 8, &sz))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if sz != 7 as libc::c_int as libc::c_longlong {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToChildrenSize.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            b"sz == 7\0" as *const u8 as *const libc::c_char,
            b"got expected size for child res\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if cellToChildrenSize(h, 9 as libc::c_int, &mut sz) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToChildrenSize.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            b"!(cellToChildrenSize(h, 9, &sz))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if sz != (7 as libc::c_int * 7 as libc::c_int) as libc::c_longlong {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToChildrenSize.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int,
            b"sz == 7 * 7\0" as *const u8 as *const libc::c_char,
            b"got expected size for grandchild res\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"cellToChildrenSize_pentagon\0" as *const u8 as *const libc::c_char;
    let mut h_0: H3Index = 0x870800000ffffff as libc::c_long as H3Index;
    let mut sz_0: int64_t = 0;
    if cellToChildrenSize(h_0, 3 as libc::c_int, &mut sz_0)
        != E_RES_DOMAIN as libc::c_int as libc::c_uint
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToChildrenSize.c\0" as *const u8 as *const libc::c_char,
            42 as libc::c_int,
            b"H3_EXPORT(cellToChildrenSize)(h, 3, &sz) == E_RES_DOMAIN\0" as *const u8
                as *const libc::c_char,
            b"got expected size for coarser res\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if cellToChildrenSize(h_0, 7 as libc::c_int, &mut sz_0) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToChildrenSize.c\0" as *const u8 as *const libc::c_char,
            43 as libc::c_int,
            b"!(cellToChildrenSize(h, 7, &sz))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if sz_0 != 1 as libc::c_int as libc::c_longlong {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToChildrenSize.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int,
            b"sz == 1\0" as *const u8 as *const libc::c_char,
            b"got expected size for same res\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if cellToChildrenSize(h_0, 8 as libc::c_int, &mut sz_0) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToChildrenSize.c\0" as *const u8 as *const libc::c_char,
            45 as libc::c_int,
            b"!(cellToChildrenSize(h, 8, &sz))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if sz_0 != 6 as libc::c_int as libc::c_longlong {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToChildrenSize.c\0" as *const u8 as *const libc::c_char,
            46 as libc::c_int,
            b"sz == 6\0" as *const u8 as *const libc::c_char,
            b"got expected size for child res\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if cellToChildrenSize(h_0, 9 as libc::c_int, &mut sz_0) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToChildrenSize.c\0" as *const u8 as *const libc::c_char,
            47 as libc::c_int,
            b"!(cellToChildrenSize(h, 9, &sz))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if sz_0
        != (5 as libc::c_int * 7 as libc::c_int + 1 as libc::c_int * 6 as libc::c_int)
            as libc::c_longlong
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToChildrenSize.c\0" as *const u8 as *const libc::c_char,
            49 as libc::c_int,
            b"sz == (5 * 7) + (1 * 6)\0" as *const u8 as *const libc::c_char,
            b"got expected size for grandchild res\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"cellToChildrenSize_largest_hexagon\0" as *const u8 as *const libc::c_char;
    let mut h_1: H3Index = 0x806dfffffffffff as libc::c_long as H3Index;
    let mut expected: int64_t = 4747561509943 as libc::c_long as int64_t;
    let mut out: int64_t = 0;
    if cellToChildrenSize(h_1, 15 as libc::c_int, &mut out) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToChildrenSize.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int,
            b"!(cellToChildrenSize(h, 15, &out))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if out != expected {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToChildrenSize.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
            b"out == expected\0" as *const u8 as *const libc::c_char,
            b"got right size for children 15 levels below\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"cellToChildrenSize_largest_pentagon\0" as *const u8 as *const libc::c_char;
    let mut h_2: H3Index = 0x8009fffffffffff as libc::c_long as H3Index;
    let mut expected_0: int64_t = 3956301258286 as libc::c_long as int64_t;
    let mut out_0: int64_t = 0;
    if cellToChildrenSize(h_2, 15 as libc::c_int, &mut out_0) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToChildrenSize.c\0" as *const u8 as *const libc::c_char,
            66 as libc::c_int,
            b"!(cellToChildrenSize(h, 15, &out))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if out_0 != expected_0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToChildrenSize.c\0" as *const u8 as *const libc::c_char,
            69 as libc::c_int,
            b"out == expected\0" as *const u8 as *const libc::c_char,
            b"got right size for children 15 levels below\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0() -> libc::c_int {
    currentSuiteName = b"cellToChildrenSize\0" as *const u8 as *const libc::c_char;
    printf(
        b"TEST %s\n\0" as *const u8 as *const libc::c_char,
        b"cellToChildrenSize\0" as *const u8 as *const libc::c_char,
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
