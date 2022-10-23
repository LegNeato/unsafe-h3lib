extern crate unsafe_h3lib_testapps;
use ::libc;
extern "C" {

    fn exit(_: libc::c_int) -> !;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn res0CellCount() -> libc::c_int;
    fn getRes0Cells(out: *mut H3Index) -> H3Error;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn _baseCellToCCWrot60(baseCell: libc::c_int, face: libc::c_int) -> libc::c_int;
    fn _isBaseCellPentagon(baseCell: libc::c_int) -> libc::c_int;
    static mut globalTestCount: libc::c_int;
    static mut currentSuiteName: *const libc::c_char;
    static mut currentTestName: *const libc::c_char;
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_off_t = __int64_t;
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
pub type H3Index = uint64_t;
pub type H3Error = uint32_t;
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
    currentTestName = b"getRes0Cells\0" as *const u8 as *const libc::c_char;
    let mut count: libc::c_int = res0CellCount();
    let mut indexes: *mut H3Index = malloc(
        (count as libc::c_ulong).wrapping_mul(::core::mem::size_of::<H3Index>() as libc::c_ulong),
    ) as *mut H3Index;
    if getRes0Cells(indexes) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testBaseCells.c\0" as *const u8 as *const libc::c_char,
            26 as libc::c_int,
            b"!(getRes0Cells(indexes))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(*indexes.offset(0 as libc::c_int as isize)
        == 0x8001fffffffffff as libc::c_long as libc::c_ulonglong)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testBaseCells.c\0" as *const u8 as *const libc::c_char,
            27 as libc::c_int,
            b"indexes[0] == 0x8001fffffffffff\0" as *const u8 as *const libc::c_char,
            b"correct first basecell\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(*indexes.offset(121 as libc::c_int as isize)
        == 0x80f3fffffffffff as libc::c_long as libc::c_ulonglong)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testBaseCells.c\0" as *const u8 as *const libc::c_char,
            28 as libc::c_int,
            b"indexes[121] == 0x80f3fffffffffff\0" as *const u8 as *const libc::c_char,
            b"correct last basecell\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    free(indexes as *mut libc::c_void);
    currentTestName = b"baseCellToCCWrot60\0" as *const u8 as *const libc::c_char;
    if !(_baseCellToCCWrot60(16 as libc::c_int, 0 as libc::c_int) == 0 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testBaseCells.c\0" as *const u8 as *const libc::c_char,
            34 as libc::c_int,
            b"_baseCellToCCWrot60(16, 0) == 0\0" as *const u8 as *const libc::c_char,
            b"got expected rotation\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(_baseCellToCCWrot60(32 as libc::c_int, 0 as libc::c_int) == 3 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testBaseCells.c\0" as *const u8 as *const libc::c_char,
            35 as libc::c_int,
            b"_baseCellToCCWrot60(32, 0) == 3\0" as *const u8 as *const libc::c_char,
            b"got expected rotation\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(_baseCellToCCWrot60(7 as libc::c_int, 3 as libc::c_int) == 1 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testBaseCells.c\0" as *const u8 as *const libc::c_char,
            36 as libc::c_int,
            b"_baseCellToCCWrot60(7, 3) == 1\0" as *const u8 as *const libc::c_char,
            b"got expected rotation\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"baseCellToCCWrot60_invalid\0" as *const u8 as *const libc::c_char;
    if !(_baseCellToCCWrot60(16 as libc::c_int, 42 as libc::c_int) == -(1 as libc::c_int)) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testBaseCells.c\0" as *const u8 as *const libc::c_char,
            40 as libc::c_int,
            b"_baseCellToCCWrot60(16, 42) == INVALID_ROTATIONS\0" as *const u8
                as *const libc::c_char,
            b"should return invalid rotation for invalid face\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(_baseCellToCCWrot60(16 as libc::c_int, -(1 as libc::c_int)) == -(1 as libc::c_int)) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testBaseCells.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            b"_baseCellToCCWrot60(16, -1) == INVALID_ROTATIONS\0" as *const u8
                as *const libc::c_char,
            b"should return invalid rotation for invalid face (negative)\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(_baseCellToCCWrot60(1 as libc::c_int, 0 as libc::c_int) == -(1 as libc::c_int)) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testBaseCells.c\0" as *const u8 as *const libc::c_char,
            42 as libc::c_int,
            b"_baseCellToCCWrot60(1, 0) == INVALID_ROTATIONS\0" as *const u8 as *const libc::c_char,
            b"should return invalid rotation for base cell not appearing on face\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"isBaseCellPentagon_invalid\0" as *const u8 as *const libc::c_char;
    if !(_isBaseCellPentagon(-(1 as libc::c_int)) == 0 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testBaseCells.c\0" as *const u8 as *const libc::c_char,
            46 as libc::c_int,
            b"_isBaseCellPentagon(-1) == false\0" as *const u8 as *const libc::c_char,
            b"isBaseCellPentagon handles negative\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0() -> libc::c_int {
    currentSuiteName = b"baseCells\0" as *const u8 as *const libc::c_char;
    printf(
        b"TEST %s\n\0" as *const u8 as *const libc::c_char,
        b"baseCells\0" as *const u8 as *const libc::c_char,
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
