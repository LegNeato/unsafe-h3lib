extern crate unsafe_h3lib;
extern crate unsafe_h3lib_applib;
extern crate unsafe_h3lib_testapps_lib;
use ::libc;
extern "C" {

    fn fabs(_: libc::c_double) -> libc::c_double;
    fn exit(_: libc::c_int) -> !;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    static mut currentTestName: *const libc::c_char;
    static mut currentSuiteName: *const libc::c_char;
    static mut globalTestCount: libc::c_int;
    fn _v2dMag(v: *const Vec2d) -> libc::c_double;
    fn _v2dIntersect(
        p0: *const Vec2d,
        p1: *const Vec2d,
        p2: *const Vec2d,
        p3: *const Vec2d,
        inter: *mut Vec2d,
    );
    fn _v2dAlmostEquals(p0: *const Vec2d, p1: *const Vec2d) -> bool;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vec2d {
    pub x: libc::c_double,
    pub y: libc::c_double,
}
unsafe extern "C" fn runTests() {
    currentTestName = b"_v2dMag\0" as *const u8 as *const libc::c_char;
    let mut v: Vec2d = {
        let mut init = Vec2d {
            x: 3.0f64,
            y: 4.0f64,
        };
        init
    };
    let mut expected: libc::c_double = 5.0f64;
    let mut mag: libc::c_double = _v2dMag(&mut v);
    if !(fabs(mag - expected) < 2.2204460492503131e-16f64) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVec2d.c\0" as *const u8 as *const libc::c_char,
            29 as libc::c_int,
            b"fabs(mag - expected) < DBL_EPSILON\0" as *const u8 as *const libc::c_char,
            b"magnitude as expected\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"_v2dIntersect\0" as *const u8 as *const libc::c_char;
    let mut p0: Vec2d = {
        let mut init = Vec2d {
            x: 2.0f64,
            y: 2.0f64,
        };
        init
    };
    let mut p1: Vec2d = {
        let mut init = Vec2d {
            x: 6.0f64,
            y: 6.0f64,
        };
        init
    };
    let mut p2: Vec2d = {
        let mut init = Vec2d {
            x: 0.0f64,
            y: 4.0f64,
        };
        init
    };
    let mut p3: Vec2d = {
        let mut init = Vec2d {
            x: 10.0f64,
            y: 4.0f64,
        };
        init
    };
    let mut intersection: Vec2d = {
        let mut init = Vec2d {
            x: 0.0f64,
            y: 0.0f64,
        };
        init
    };
    _v2dIntersect(&mut p0, &mut p1, &mut p2, &mut p3, &mut intersection);
    let mut expectedX: libc::c_double = 4.0f64;
    let mut expectedY: libc::c_double = 4.0f64;
    if !(fabs(intersection.x - expectedX) < 2.2204460492503131e-16f64) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVec2d.c\0" as *const u8 as *const libc::c_char,
            45 as libc::c_int,
            b"fabs(intersection.x - expectedX) < DBL_EPSILON\0" as *const u8 as *const libc::c_char,
            b"X coord as expected\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(fabs(intersection.y - expectedY) < 2.2204460492503131e-16f64) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVec2d.c\0" as *const u8 as *const libc::c_char,
            47 as libc::c_int,
            b"fabs(intersection.y - expectedY) < DBL_EPSILON\0" as *const u8 as *const libc::c_char,
            b"Y coord as expected\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"_v2dAlmostEquals\0" as *const u8 as *const libc::c_char;
    let mut v1: Vec2d = {
        let mut init = Vec2d {
            x: 3.0f64,
            y: 4.0f64,
        };
        init
    };
    let mut v2: Vec2d = {
        let mut init = Vec2d {
            x: 3.0f64,
            y: 4.0f64,
        };
        init
    };
    let mut v3: Vec2d = {
        let mut init = Vec2d {
            x: 3.5f64,
            y: 4.0f64,
        };
        init
    };
    let mut v4: Vec2d = {
        let mut init = Vec2d {
            x: 3.0f64,
            y: 4.5f64,
        };
        init
    };
    let mut v5: Vec2d = {
        let mut init = Vec2d {
            x: 3.0f64 + 2.2204460492503131e-16f64,
            y: 4.0f64 - 2.2204460492503131e-16f64,
        };
        init
    };
    if !_v2dAlmostEquals(&mut v1, &mut v2) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVec2d.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            b"_v2dAlmostEquals(&v1, &v2)\0" as *const u8 as *const libc::c_char,
            b"true for equal vectors\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if _v2dAlmostEquals(&mut v1, &mut v3) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVec2d.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            b"!_v2dAlmostEquals(&v1, &v3)\0" as *const u8 as *const libc::c_char,
            b"false for different x\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if _v2dAlmostEquals(&mut v1, &mut v4) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVec2d.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
            b"!_v2dAlmostEquals(&v1, &v4)\0" as *const u8 as *const libc::c_char,
            b"false for different y\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !_v2dAlmostEquals(&mut v1, &mut v5) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVec2d.c\0" as *const u8 as *const libc::c_char,
            60 as libc::c_int,
            b"_v2dAlmostEquals(&v1, &v5)\0" as *const u8 as *const libc::c_char,
            b"true for almost equal\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0() -> libc::c_int {
    currentSuiteName = b"Vec2d\0" as *const u8 as *const libc::c_char;
    printf(
        b"TEST %s\n\0" as *const u8 as *const libc::c_char,
        b"Vec2d\0" as *const u8 as *const libc::c_char,
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
