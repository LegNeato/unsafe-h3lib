use ::libc;
extern "C" {
    pub type __sFILEX;
    fn exit(_: libc::c_int) -> !;
    fn pentagonCount() -> libc::c_int;
    fn getPentagons(res: libc::c_int, out: *mut H3Index) -> H3Error;
    fn getResolution(h: H3Index) -> libc::c_int;
    fn isValidCell(h: H3Index) -> libc::c_int;
    fn isPentagon(h: H3Index) -> libc::c_int;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
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
    pub _extra: *mut __sFILEX,
    pub _ur: libc::c_int,
    pub _ubuf: [libc::c_uchar; 3],
    pub _nbuf: [libc::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: libc::c_int,
    pub _offset: fpos_t,
}
pub type FILE = __sFILE;
unsafe extern "C" fn runTests() {
    currentTestName = b"propertyTests\0" as *const u8 as *const libc::c_char;
    let mut expectedCount: libc::c_int = pentagonCount();
    let mut res: libc::c_int = 0 as libc::c_int;
    while res <= 15 as libc::c_int {
        let mut h3Indexes: [H3Index; 16] = [
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
        ];
        if getPentagons(res, h3Indexes.as_mut_ptr()) != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"/Users/legnitto/src/h3/src/apps/testapps/testPentagonIndexes.c\0" as *const u8
                    as *const libc::c_char,
                30 as libc::c_int,
                b"!(getPentagons(res, h3Indexes))\0" as *const u8 as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        let mut numFound: libc::c_int = 0 as libc::c_int;
        let mut i: libc::c_int = 0 as libc::c_int;
        while i < 16 as libc::c_int {
            let mut h3Index: H3Index = h3Indexes[i as usize];
            if h3Index != 0 {
                numFound += 1;
                if isValidCell(h3Index) == 0 {
                    fprintf(
                        __stderrp,
                        b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                            as *const libc::c_char,
                        currentSuiteName,
                        currentTestName,
                        b"/Users/legnitto/src/h3/src/apps/testapps/testPentagonIndexes.c\0"
                            as *const u8 as *const libc::c_char,
                        39 as libc::c_int,
                        b"H3_EXPORT(isValidCell(h3Index))\0" as *const u8 as *const libc::c_char,
                        b"index should be valid\0" as *const u8 as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
                globalTestCount += 1;
                printf(b".\0" as *const u8 as *const libc::c_char);
                if isPentagon(h3Index) == 0 {
                    fprintf(
                        __stderrp,
                        b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                            as *const libc::c_char,
                        currentSuiteName,
                        currentTestName,
                        b"/Users/legnitto/src/h3/src/apps/testapps/testPentagonIndexes.c\0"
                            as *const u8 as *const libc::c_char,
                        41 as libc::c_int,
                        b"H3_EXPORT(isPentagon(h3Index))\0" as *const u8 as *const libc::c_char,
                        b"index should be pentagon\0" as *const u8 as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
                globalTestCount += 1;
                printf(b".\0" as *const u8 as *const libc::c_char);
                if !(getResolution(h3Index) == res) {
                    fprintf(
                        __stderrp,
                        b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                            as *const libc::c_char,
                        currentSuiteName,
                        currentTestName,
                        b"/Users/legnitto/src/h3/src/apps/testapps/testPentagonIndexes.c\0"
                            as *const u8 as *const libc::c_char,
                        43 as libc::c_int,
                        b"H3_EXPORT(getResolution(h3Index)) == res\0" as *const u8
                            as *const libc::c_char,
                        b"index should have correct resolution\0" as *const u8
                            as *const libc::c_char,
                    );
                    exit(1 as libc::c_int);
                }
                globalTestCount += 1;
                printf(b".\0" as *const u8 as *const libc::c_char);
                let mut j: libc::c_int = i + 1 as libc::c_int;
                while j < 16 as libc::c_int {
                    if h3Indexes[j as usize] == h3Index {
                        if 0 as libc::c_int == 0 {
                            fprintf(
                                __stderrp,
                                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                                    as *const libc::c_char,
                                currentSuiteName,
                                currentTestName,
                                b"/Users/legnitto/src/h3/src/apps/testapps/testPentagonIndexes.c\0"
                                    as *const u8
                                    as *const libc::c_char,
                                48 as libc::c_int,
                                b"false\0" as *const u8 as *const libc::c_char,
                                b"index should be seen only once\0" as *const u8
                                    as *const libc::c_char,
                            );
                            exit(1 as libc::c_int);
                        }
                        globalTestCount += 1;
                        printf(b".\0" as *const u8 as *const libc::c_char);
                    }
                    j += 1;
                }
            }
            i += 1;
        }
        if !(numFound == expectedCount) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"/Users/legnitto/src/h3/src/apps/testapps/testPentagonIndexes.c\0" as *const u8
                    as *const libc::c_char,
                55 as libc::c_int,
                b"numFound == expectedCount\0" as *const u8 as *const libc::c_char,
                b"there should be exactly 12 pentagons\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        res += 1;
    }
    currentTestName = b"getPentagonsInvalid\0" as *const u8 as *const libc::c_char;
    let mut h3Indexes_0: [H3Index; 16] = [
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
    ];
    if !(getPentagons(16 as libc::c_int, h3Indexes_0.as_mut_ptr())
        == E_RES_DOMAIN as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testPentagonIndexes.c\0" as *const u8
                as *const libc::c_char,
            62 as libc::c_int,
            b"H3_EXPORT(getPentagons)(16, h3Indexes) == E_RES_DOMAIN\0" as *const u8
                as *const libc::c_char,
            b"getPentagons of invalid resolutions fails\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(getPentagons(100 as libc::c_int, h3Indexes_0.as_mut_ptr())
        == E_RES_DOMAIN as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testPentagonIndexes.c\0" as *const u8
                as *const libc::c_char,
            64 as libc::c_int,
            b"H3_EXPORT(getPentagons)(100, h3Indexes) == E_RES_DOMAIN\0" as *const u8
                as *const libc::c_char,
            b"getPentagons of invalid resolutions fails\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(getPentagons(-(1 as libc::c_int), h3Indexes_0.as_mut_ptr())
        == E_RES_DOMAIN as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testPentagonIndexes.c\0" as *const u8
                as *const libc::c_char,
            66 as libc::c_int,
            b"H3_EXPORT(getPentagons)(-1, h3Indexes) == E_RES_DOMAIN\0" as *const u8
                as *const libc::c_char,
            b"getPentagons of invalid resolutions fails\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"invalidPentagons\0" as *const u8 as *const libc::c_char;
    if isPentagon(0 as libc::c_int as H3Index) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testPentagonIndexes.c\0" as *const u8
                as *const libc::c_char,
            70 as libc::c_int,
            b"!H3_EXPORT(isPentagon)(0)\0" as *const u8 as *const libc::c_char,
            b"0 is not a pentagon\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if isPentagon(0x7fffffffffffffff as libc::c_long as H3Index) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"/Users/legnitto/src/h3/src/apps/testapps/testPentagonIndexes.c\0" as *const u8
                as *const libc::c_char,
            72 as libc::c_int,
            b"!H3_EXPORT(isPentagon)(0x7fffffffffffffff)\0" as *const u8 as *const libc::c_char,
            b"all but high bit is not a pentagon\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0() -> libc::c_int {
    currentSuiteName = b"getPentagons\0" as *const u8 as *const libc::c_char;
    printf(
        b"TEST %s\n\0" as *const u8 as *const libc::c_char,
        b"getPentagons\0" as *const u8 as *const libc::c_char,
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
