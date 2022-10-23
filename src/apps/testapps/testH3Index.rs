extern crate unsafe_h3lib;
extern crate unsafe_h3lib_applib;
extern crate unsafe_h3lib_testapps_lib;
use ::libc;
extern "C" {

    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn latLngToCell(g: *const LatLng, res: libc::c_int, out: *mut H3Index) -> H3Error;
    fn getBaseCellNumber(h: H3Index) -> libc::c_int;
    fn stringToH3(str: *const libc::c_char, out: *mut H3Index) -> H3Error;
    fn h3ToString(h: H3Index, str: *mut libc::c_char, sz: size_t) -> H3Error;
    fn isValidCell(h: H3Index) -> libc::c_int;
    fn isResClassIII(h: H3Index) -> libc::c_int;
    fn setGeoDegs(p: *mut LatLng, latDegs: libc::c_double, lngDegs: libc::c_double);
    fn setH3Index(h: *mut H3Index, res: libc::c_int, baseCell: libc::c_int, initDigit: Direction);
    fn isResolutionClassIII(r: libc::c_int) -> libc::c_int;
    fn _faceIjkToH3(fijk: *const FaceIJK, res: libc::c_int) -> H3Index;
    static mut globalTestCount: libc::c_int;
    static mut currentSuiteName: *const libc::c_char;
    static mut currentTestName: *const libc::c_char;
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_off_t = __int64_t;
pub type size_t = __darwin_size_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FaceIJK {
    pub face: libc::c_int,
    pub coord: CoordIJK,
}
unsafe extern "C" fn runTests() {
    currentTestName = b"latLngToCellExtremeCoordinates\0" as *const u8 as *const libc::c_char;
    let mut h: H3Index = 0;
    let mut g: LatLng = {
        let mut init = LatLng {
            lat: 0 as libc::c_int as libc::c_double,
            lng: 1E45f64,
        };
        init
    };
    if latLngToCell(&mut g, 14 as libc::c_int, &mut h) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
            36 as libc::c_int,
            b"!(latLngToCell(&g, 14, &h))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut g2: LatLng = {
        let mut init = LatLng {
            lat: 1E46f64,
            lng: 1E45f64,
        };
        init
    };
    if latLngToCell(&mut g2, 15 as libc::c_int, &mut h) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
            39 as libc::c_int,
            b"!(latLngToCell(&g2, 15, &h))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut g4: LatLng = LatLng { lat: 0., lng: 0. };
    setGeoDegs(&mut g4, 2 as libc::c_int as libc::c_double, -3E39f64);
    if latLngToCell(&mut g4, 0 as libc::c_int, &mut h) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
            43 as libc::c_int,
            b"!(latLngToCell(&g4, 0, &h))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"faceIjkToH3ExtremeCoordinates\0" as *const u8 as *const libc::c_char;
    let mut fijk0I: FaceIJK = {
        let mut init = FaceIJK {
            face: 0 as libc::c_int,
            coord: {
                let mut init = CoordIJK {
                    i: 3 as libc::c_int,
                    j: 0 as libc::c_int,
                    k: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if !(_faceIjkToH3(&mut fijk0I, 0 as libc::c_int) == 0 as libc::c_int as libc::c_ulonglong) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            b"_faceIjkToH3(&fijk0I, 0) == 0\0" as *const u8 as *const libc::c_char,
            b"i out of bounds at res 0\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut fijk0J: FaceIJK = {
        let mut init = FaceIJK {
            face: 1 as libc::c_int,
            coord: {
                let mut init = CoordIJK {
                    i: 0 as libc::c_int,
                    j: 4 as libc::c_int,
                    k: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if !(_faceIjkToH3(&mut fijk0J, 0 as libc::c_int) == 0 as libc::c_int as libc::c_ulonglong) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
            50 as libc::c_int,
            b"_faceIjkToH3(&fijk0J, 0) == 0\0" as *const u8 as *const libc::c_char,
            b"j out of bounds at res 0\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut fijk0K: FaceIJK = {
        let mut init = FaceIJK {
            face: 2 as libc::c_int,
            coord: {
                let mut init = CoordIJK {
                    i: 2 as libc::c_int,
                    j: 0 as libc::c_int,
                    k: 5 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if !(_faceIjkToH3(&mut fijk0K, 0 as libc::c_int) == 0 as libc::c_int as libc::c_ulonglong) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
            52 as libc::c_int,
            b"_faceIjkToH3(&fijk0K, 0) == 0\0" as *const u8 as *const libc::c_char,
            b"k out of bounds at res 0\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut fijk1I: FaceIJK = {
        let mut init = FaceIJK {
            face: 3 as libc::c_int,
            coord: {
                let mut init = CoordIJK {
                    i: 6 as libc::c_int,
                    j: 0 as libc::c_int,
                    k: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if !(_faceIjkToH3(&mut fijk1I, 1 as libc::c_int) == 0 as libc::c_int as libc::c_ulonglong) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
            55 as libc::c_int,
            b"_faceIjkToH3(&fijk1I, 1) == 0\0" as *const u8 as *const libc::c_char,
            b"i out of bounds at res 1\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut fijk1J: FaceIJK = {
        let mut init = FaceIJK {
            face: 4 as libc::c_int,
            coord: {
                let mut init = CoordIJK {
                    i: 0 as libc::c_int,
                    j: 7 as libc::c_int,
                    k: 1 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if !(_faceIjkToH3(&mut fijk1J, 1 as libc::c_int) == 0 as libc::c_int as libc::c_ulonglong) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            b"_faceIjkToH3(&fijk1J, 1) == 0\0" as *const u8 as *const libc::c_char,
            b"j out of bounds at res 1\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut fijk1K: FaceIJK = {
        let mut init = FaceIJK {
            face: 5 as libc::c_int,
            coord: {
                let mut init = CoordIJK {
                    i: 2 as libc::c_int,
                    j: 0 as libc::c_int,
                    k: 8 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if !(_faceIjkToH3(&mut fijk1K, 1 as libc::c_int) == 0 as libc::c_int as libc::c_ulonglong) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
            b"_faceIjkToH3(&fijk1K, 1) == 0\0" as *const u8 as *const libc::c_char,
            b"k out of bounds at res 1\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut fijk2I: FaceIJK = {
        let mut init = FaceIJK {
            face: 6 as libc::c_int,
            coord: {
                let mut init = CoordIJK {
                    i: 18 as libc::c_int,
                    j: 0 as libc::c_int,
                    k: 0 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if !(_faceIjkToH3(&mut fijk2I, 2 as libc::c_int) == 0 as libc::c_int as libc::c_ulonglong) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
            62 as libc::c_int,
            b"_faceIjkToH3(&fijk2I, 2) == 0\0" as *const u8 as *const libc::c_char,
            b"i out of bounds at res 2\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut fijk2J: FaceIJK = {
        let mut init = FaceIJK {
            face: 7 as libc::c_int,
            coord: {
                let mut init = CoordIJK {
                    i: 0 as libc::c_int,
                    j: 19 as libc::c_int,
                    k: 1 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if !(_faceIjkToH3(&mut fijk2J, 2 as libc::c_int) == 0 as libc::c_int as libc::c_ulonglong) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
            64 as libc::c_int,
            b"_faceIjkToH3(&fijk2J, 2) == 0\0" as *const u8 as *const libc::c_char,
            b"j out of bounds at res 2\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut fijk2K: FaceIJK = {
        let mut init = FaceIJK {
            face: 8 as libc::c_int,
            coord: {
                let mut init = CoordIJK {
                    i: 2 as libc::c_int,
                    j: 0 as libc::c_int,
                    k: 20 as libc::c_int,
                };
                init
            },
        };
        init
    };
    if !(_faceIjkToH3(&mut fijk2K, 2 as libc::c_int) == 0 as libc::c_int as libc::c_ulonglong) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
            66 as libc::c_int,
            b"_faceIjkToH3(&fijk2K, 2) == 0\0" as *const u8 as *const libc::c_char,
            b"k out of bounds at res 2\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"isValidCellAtResolution\0" as *const u8 as *const libc::c_char;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i <= 15 as libc::c_int {
        let mut g_0: LatLng = {
            let mut init = LatLng {
                lat: 0 as libc::c_int as libc::c_double,
                lng: 0 as libc::c_int as libc::c_double,
            };
            init
        };
        let mut h3: H3Index = 0;
        if latLngToCell(&mut g_0, i, &mut h3) != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
                73 as libc::c_int,
                b"!(latLngToCell(&g, i, &h3))\0" as *const u8 as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        let mut failureMessage: [libc::c_char; 256] = [0; 256];
        sprintf(
            failureMessage.as_mut_ptr(),
            b"isValidCell failed on resolution %d\0" as *const u8 as *const libc::c_char,
            i,
        );
        if isValidCell(h3) == 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
                76 as libc::c_int,
                b"H3_EXPORT(isValidCell)(h3)\0" as *const u8 as *const libc::c_char,
                failureMessage.as_mut_ptr(),
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        i += 1;
    }
    currentTestName = b"isValidCellDigits\0" as *const u8 as *const libc::c_char;
    let mut g_1: LatLng = {
        let mut init = LatLng {
            lat: 0 as libc::c_int as libc::c_double,
            lng: 0 as libc::c_int as libc::c_double,
        };
        init
    };
    let mut h3_0: H3Index = 0;
    if latLngToCell(&mut g_1, 1 as libc::c_int, &mut h3_0) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
            83 as libc::c_int,
            b"!(latLngToCell(&g, 1, &h3))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    h3_0 ^= 1 as libc::c_int as libc::c_ulonglong;
    if isValidCell(h3_0) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
            87 as libc::c_int,
            b"!H3_EXPORT(isValidCell)(h3)\0" as *const u8 as *const libc::c_char,
            b"isValidCell failed on invalid unused digits\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"isValidCellBaseCell\0" as *const u8 as *const libc::c_char;
    let mut i_0: libc::c_int = 0 as libc::c_int;
    while i_0 < 122 as libc::c_int {
        let mut h_0: H3Index = 35184372088831 as libc::c_ulonglong;
        h_0 = h_0 & !((15 as libc::c_int as uint64_t) << 59 as libc::c_int)
            | (1 as libc::c_int as uint64_t) << 59 as libc::c_int;
        h_0 = h_0 & !((127 as libc::c_int as uint64_t) << 45 as libc::c_int)
            | (i_0 as uint64_t) << 45 as libc::c_int;
        let mut failureMessage_0: [libc::c_char; 256] = [0; 256];
        sprintf(
            failureMessage_0.as_mut_ptr(),
            b"isValidCell failed on base cell %d\0" as *const u8 as *const libc::c_char,
            i_0,
        );
        if isValidCell(h_0) == 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
                97 as libc::c_int,
                b"H3_EXPORT(isValidCell)(h)\0" as *const u8 as *const libc::c_char,
                failureMessage_0.as_mut_ptr(),
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        if !(getBaseCellNumber(h_0) == i_0) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
                100 as libc::c_int,
                b"H3_EXPORT(getBaseCellNumber)(h) == i\0" as *const u8 as *const libc::c_char,
                b"failed to recover base cell\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        i_0 += 1;
    }
    currentTestName = b"isValidCellBaseCellInvalid\0" as *const u8 as *const libc::c_char;
    let mut hWrongBaseCell: H3Index = 35184372088831 as libc::c_ulonglong;
    hWrongBaseCell = hWrongBaseCell & !((15 as libc::c_int as uint64_t) << 59 as libc::c_int)
        | (1 as libc::c_int as uint64_t) << 59 as libc::c_int;
    hWrongBaseCell = hWrongBaseCell & !((127 as libc::c_int as uint64_t) << 45 as libc::c_int)
        | (122 as libc::c_int as uint64_t) << 45 as libc::c_int;
    if isValidCell(hWrongBaseCell) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
            109 as libc::c_int,
            b"!H3_EXPORT(isValidCell)(hWrongBaseCell)\0" as *const u8 as *const libc::c_char,
            b"isValidCell failed on invalid base cell\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"isValidCellWithMode\0" as *const u8 as *const libc::c_char;
    let mut i_1: libc::c_int = 0 as libc::c_int;
    while i_1 <= 0xf as libc::c_int {
        let mut h_1: H3Index = 35184372088831 as libc::c_ulonglong;
        h_1 = h_1 & !((15 as libc::c_int as uint64_t) << 59 as libc::c_int)
            | (i_1 as uint64_t) << 59 as libc::c_int;
        if i_1 == 1 as libc::c_int {
            if isValidCell(h_1) == 0 {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
                    118 as libc::c_int,
                    b"H3_EXPORT(isValidCell)(h)\0" as *const u8 as *const libc::c_char,
                    b"isValidCell succeeds on valid mode\0" as *const u8 as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
        } else {
            let mut failureMessage_1: [libc::c_char; 256] = [0; 256];
            sprintf(
                failureMessage_1.as_mut_ptr(),
                b"isValidCell failed on mode %d\0" as *const u8 as *const libc::c_char,
                i_1,
            );
            if isValidCell(h_1) != 0 {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
                    122 as libc::c_int,
                    b"!H3_EXPORT(isValidCell)(h)\0" as *const u8 as *const libc::c_char,
                    failureMessage_1.as_mut_ptr(),
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
        }
        i_1 += 1;
    }
    currentTestName = b"isValidCellReservedBits\0" as *const u8 as *const libc::c_char;
    let mut i_2: libc::c_int = 0 as libc::c_int;
    while i_2 < 8 as libc::c_int {
        let mut h_2: H3Index = 35184372088831 as libc::c_ulonglong;
        h_2 = h_2 & !((15 as libc::c_int as uint64_t) << 59 as libc::c_int)
            | (1 as libc::c_int as uint64_t) << 59 as libc::c_int;
        h_2 = h_2 & !((7 as libc::c_int as uint64_t) << 56 as libc::c_int)
            | (i_2 as uint64_t) << 56 as libc::c_int;
        if i_2 == 0 as libc::c_int {
            if isValidCell(h_2) == 0 {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
                    134 as libc::c_int,
                    b"H3_EXPORT(isValidCell)(h)\0" as *const u8 as *const libc::c_char,
                    b"isValidCell succeeds on valid reserved bits\0" as *const u8
                        as *const libc::c_char,
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
        } else {
            let mut failureMessage_2: [libc::c_char; 256] = [0; 256];
            sprintf(
                failureMessage_2.as_mut_ptr(),
                b"isValidCell failed on reserved bits %d\0" as *const u8 as *const libc::c_char,
                i_2,
            );
            if isValidCell(h_2) != 0 {
                fprintf(
                    __stderrp,
                    b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                        as *const libc::c_char,
                    currentSuiteName,
                    currentTestName,
                    b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
                    139 as libc::c_int,
                    b"!H3_EXPORT(isValidCell)(h)\0" as *const u8 as *const libc::c_char,
                    failureMessage_2.as_mut_ptr(),
                );
                exit(1 as libc::c_int);
            }
            globalTestCount += 1;
            printf(b".\0" as *const u8 as *const libc::c_char);
        }
        i_2 += 1;
    }
    currentTestName = b"isValidCellHighBit\0" as *const u8 as *const libc::c_char;
    let mut h_3: H3Index = 35184372088831 as libc::c_ulonglong;
    h_3 = h_3 & !((15 as libc::c_int as uint64_t) << 59 as libc::c_int)
        | (1 as libc::c_int as uint64_t) << 59 as libc::c_int;
    h_3 = h_3 & !((1 as libc::c_int as uint64_t) << 63 as libc::c_int)
        | (1 as libc::c_int as uint64_t) << 63 as libc::c_int;
    if isValidCell(h_3) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
            148 as libc::c_int,
            b"!H3_EXPORT(isValidCell)(h)\0" as *const u8 as *const libc::c_char,
            b"isValidCell failed on high bit\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"h3BadDigitInvalid\0" as *const u8 as *const libc::c_char;
    let mut h_4: H3Index = 35184372088831 as libc::c_ulonglong;
    h_4 = h_4 & !((15 as libc::c_int as uint64_t) << 59 as libc::c_int)
        | (1 as libc::c_int as uint64_t) << 59 as libc::c_int;
    h_4 = h_4 & !((15 as libc::c_ulonglong) << 52 as libc::c_int)
        | (1 as libc::c_int as uint64_t) << 52 as libc::c_int;
    if isValidCell(h_4) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
            157 as libc::c_int,
            b"!H3_EXPORT(isValidCell)(h)\0" as *const u8 as *const libc::c_char,
            b"isValidCell failed on too large digit\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"h3DeletedSubsequenceInvalid\0" as *const u8 as *const libc::c_char;
    let mut h_5: H3Index = 0;
    setH3Index(&mut h_5, 1 as libc::c_int, 4 as libc::c_int, K_AXES_DIGIT);
    if isValidCell(h_5) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
            165 as libc::c_int,
            b"!H3_EXPORT(isValidCell)(h)\0" as *const u8 as *const libc::c_char,
            b"isValidCell failed on deleted subsequence\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"h3ToString\0" as *const u8 as *const libc::c_char;
    let bufSz: size_t = 17 as libc::c_int as size_t;
    let mut buf: [libc::c_char; 17] = [
        0 as libc::c_int as libc::c_char,
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
    if !(h3ToString(
        0x1234 as libc::c_int as H3Index,
        buf.as_mut_ptr(),
        bufSz.wrapping_sub(1 as libc::c_int as libc::c_ulong),
    ) == E_MEMORY_BOUNDS as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
            173 as libc::c_int,
            b"H3_EXPORT(h3ToString)(0x1234, buf, bufSz - 1) == E_MEMORY_BOUNDS\0" as *const u8
                as *const libc::c_char,
            b"h3ToString failed on buffer too small\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if h3ToString(0xcafe as libc::c_int as H3Index, buf.as_mut_ptr(), bufSz) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
            174 as libc::c_int,
            b"!(h3ToString(0xcafe, buf, bufSz))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(strcmp(
        buf.as_mut_ptr(),
        b"cafe\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
            176 as libc::c_int,
            b"strcmp(buf, \"cafe\") == 0\0" as *const u8 as *const libc::c_char,
            b"h3ToString failed to produce base 16 results\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if h3ToString(
        0xffffffffffffffff as libc::c_ulong as H3Index,
        buf.as_mut_ptr(),
        bufSz,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
            177 as libc::c_int,
            b"!(h3ToString(0xffffffffffffffff, buf, bufSz))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(strcmp(
        buf.as_mut_ptr(),
        b"ffffffffffffffff\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
            179 as libc::c_int,
            b"strcmp(buf, \"ffffffffffffffff\") == 0\0" as *const u8 as *const libc::c_char,
            b"h3ToString failed on large input\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(buf[bufSz.wrapping_sub(1 as libc::c_int as libc::c_ulong) as usize] as libc::c_int
        == 0 as libc::c_int)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
            b"buf[bufSz - 1] == 0\0" as *const u8 as *const libc::c_char,
            b"didn't null terminate\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"stringToH3\0" as *const u8 as *const libc::c_char;
    let mut h3_1: H3Index = 0;
    if !(stringToH3(b"\0" as *const u8 as *const libc::c_char, &mut h3_1)
        == E_FAILED as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
            186 as libc::c_int,
            b"H3_EXPORT(stringToH3)(\"\", &h3) == E_FAILED\0" as *const u8 as *const libc::c_char,
            b"no index from nothing\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(stringToH3(b"**\0" as *const u8 as *const libc::c_char, &mut h3_1)
        == E_FAILED as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
            188 as libc::c_int,
            b"H3_EXPORT(stringToH3)(\"**\", &h3) == E_FAILED\0" as *const u8 as *const libc::c_char,
            b"no index from junk\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if stringToH3(
        b"ffffffffffffffff\0" as *const u8 as *const libc::c_char,
        &mut h3_1,
    ) != 0
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
            189 as libc::c_int,
            b"!(stringToH3(\"ffffffffffffffff\", &h3))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(h3_1 == 0xffffffffffffffff as libc::c_ulong as libc::c_ulonglong) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
            190 as libc::c_int,
            b"h3 == 0xffffffffffffffff\0" as *const u8 as *const libc::c_char,
            b"got expected on large input\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"setH3Index\0" as *const u8 as *const libc::c_char;
    let mut h_6: H3Index = 0;
    setH3Index(&mut h_6, 5 as libc::c_int, 12 as libc::c_int, K_AXES_DIGIT);
    if !(((h_6 & (15 as libc::c_ulonglong) << 52 as libc::c_int) >> 52 as libc::c_int)
        as libc::c_int
        == 5 as libc::c_int)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
            196 as libc::c_int,
            b"H3_GET_RESOLUTION(h) == 5\0" as *const u8 as *const libc::c_char,
            b"resolution as expected\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(((h_6 & (127 as libc::c_int as uint64_t) << 45 as libc::c_int) >> 45 as libc::c_int)
        as libc::c_int
        == 12 as libc::c_int)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
            197 as libc::c_int,
            b"H3_GET_BASE_CELL(h) == 12\0" as *const u8 as *const libc::c_char,
            b"base cell as expected\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(((h_6 & (15 as libc::c_int as uint64_t) << 59 as libc::c_int) >> 59 as libc::c_int)
        as libc::c_int
        == 1 as libc::c_int)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
            198 as libc::c_int,
            b"H3_GET_MODE(h) == H3_CELL_MODE\0" as *const u8 as *const libc::c_char,
            b"mode as expected\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut i_3: libc::c_int = 1 as libc::c_int;
    while i_3 <= 5 as libc::c_int {
        if !((h_6 >> (15 as libc::c_int - i_3) * 3 as libc::c_int & 7 as libc::c_int as uint64_t)
            as Direction as libc::c_uint
            == 1 as libc::c_int as libc::c_uint)
        {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
                200 as libc::c_int,
                b"H3_GET_INDEX_DIGIT(h, i) == 1\0" as *const u8 as *const libc::c_char,
                b"digit as expected\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        i_3 += 1;
    }
    let mut i_4: libc::c_int = 6 as libc::c_int;
    while i_4 <= 15 as libc::c_int {
        if !((h_6 >> (15 as libc::c_int - i_4) * 3 as libc::c_int & 7 as libc::c_int as uint64_t)
            as Direction as libc::c_uint
            == 7 as libc::c_int as libc::c_uint)
        {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
                204 as libc::c_int,
                b"H3_GET_INDEX_DIGIT(h, i) == 7\0" as *const u8 as *const libc::c_char,
                b"blanked digit as expected\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        i_4 += 1;
    }
    if !(h_6 == 0x85184927fffffff as libc::c_long as libc::c_ulonglong) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
            206 as libc::c_int,
            b"h == 0x85184927fffffffL\0" as *const u8 as *const libc::c_char,
            b"index matches expected\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"isResClassIII\0" as *const u8 as *const libc::c_char;
    let mut coord: LatLng = {
        let mut init = LatLng {
            lat: 0 as libc::c_int as libc::c_double,
            lng: 0 as libc::c_int as libc::c_double,
        };
        init
    };
    let mut i_5: libc::c_int = 0 as libc::c_int;
    while i_5 <= 15 as libc::c_int {
        let mut h_7: H3Index = 0;
        if latLngToCell(&mut coord, i_5, &mut h_7) != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
                213 as libc::c_int,
                b"!(latLngToCell(&coord, i, &h))\0" as *const u8 as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        if !(isResClassIII(h_7) == isResolutionClassIII(i_5)) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testH3Index.c\0" as *const u8 as *const libc::c_char,
                215 as libc::c_int,
                b"H3_EXPORT(isResClassIII)(h) == isResolutionClassIII(i)\0" as *const u8
                    as *const libc::c_char,
                b"matches existing definition\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        i_5 += 1;
    }
}
unsafe fn main_0() -> libc::c_int {
    currentSuiteName = b"h3Index\0" as *const u8 as *const libc::c_char;
    printf(
        b"TEST %s\n\0" as *const u8 as *const libc::c_char,
        b"h3Index\0" as *const u8 as *const libc::c_char,
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
