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
    fn _geoToVec3d(geo: *const LatLng, point: *mut Vec3d);
    fn _pointSquareDist(p1: *const Vec3d, p2: *const Vec3d) -> libc::c_double;
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
pub struct LatLng {
    pub lat: libc::c_double,
    pub lng: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vec3d {
    pub x: libc::c_double,
    pub y: libc::c_double,
    pub z: libc::c_double,
}
unsafe extern "C" fn runTests() {
    currentTestName = b"_pointSquareDist\0" as *const u8 as *const libc::c_char;
    let mut v1: Vec3d = {
        let mut init = Vec3d {
            x: 0 as libc::c_int as libc::c_double,
            y: 0 as libc::c_int as libc::c_double,
            z: 0 as libc::c_int as libc::c_double,
        };
        init
    };
    let mut v2: Vec3d = {
        let mut init = Vec3d {
            x: 1 as libc::c_int as libc::c_double,
            y: 0 as libc::c_int as libc::c_double,
            z: 0 as libc::c_int as libc::c_double,
        };
        init
    };
    let mut v3: Vec3d = {
        let mut init = Vec3d {
            x: 0 as libc::c_int as libc::c_double,
            y: 1 as libc::c_int as libc::c_double,
            z: 1 as libc::c_int as libc::c_double,
        };
        init
    };
    let mut v4: Vec3d = {
        let mut init = Vec3d {
            x: 1 as libc::c_int as libc::c_double,
            y: 1 as libc::c_int as libc::c_double,
            z: 1 as libc::c_int as libc::c_double,
        };
        init
    };
    let mut v5: Vec3d = {
        let mut init = Vec3d {
            x: 1 as libc::c_int as libc::c_double,
            y: 1 as libc::c_int as libc::c_double,
            z: 2 as libc::c_int as libc::c_double,
        };
        init
    };
    if !(fabs(_pointSquareDist(&mut v1, &mut v1)) < 2.2204460492503131e-16f64) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVec3d.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            b"fabs(_pointSquareDist(&v1, &v1)) < DBL_EPSILON\0" as *const u8 as *const libc::c_char,
            b"distance to self is 0\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(fabs(_pointSquareDist(&mut v1, &mut v2) - 1 as libc::c_int as libc::c_double)
        < 2.2204460492503131e-16f64)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVec3d.c\0" as *const u8 as *const libc::c_char,
            35 as libc::c_int,
            b"fabs(_pointSquareDist(&v1, &v2) - 1) < DBL_EPSILON\0" as *const u8
                as *const libc::c_char,
            b"distance to <1,0,0> is 1\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(fabs(_pointSquareDist(&mut v1, &mut v3) - 2 as libc::c_int as libc::c_double)
        < 2.2204460492503131e-16f64)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVec3d.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            b"fabs(_pointSquareDist(&v1, &v3) - 2) < DBL_EPSILON\0" as *const u8
                as *const libc::c_char,
            b"distance to <0,1,1> is 2\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(fabs(_pointSquareDist(&mut v1, &mut v4) - 3 as libc::c_int as libc::c_double)
        < 2.2204460492503131e-16f64)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVec3d.c\0" as *const u8 as *const libc::c_char,
            39 as libc::c_int,
            b"fabs(_pointSquareDist(&v1, &v4) - 3) < DBL_EPSILON\0" as *const u8
                as *const libc::c_char,
            b"distance to <1,1,1> is 3\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(fabs(_pointSquareDist(&mut v1, &mut v5) - 6 as libc::c_int as libc::c_double)
        < 2.2204460492503131e-16f64)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVec3d.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            b"fabs(_pointSquareDist(&v1, &v5) - 6) < DBL_EPSILON\0" as *const u8
                as *const libc::c_char,
            b"distance to <1,1,2> is 6\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"_geoToVec3d\0" as *const u8 as *const libc::c_char;
    let mut origin: Vec3d = {
        let mut init = Vec3d {
            x: 0 as libc::c_int as libc::c_double,
            y: 0.,
            z: 0.,
        };
        init
    };
    let mut c1: LatLng = {
        let mut init = LatLng {
            lat: 0 as libc::c_int as libc::c_double,
            lng: 0 as libc::c_int as libc::c_double,
        };
        init
    };
    let mut p1: Vec3d = Vec3d {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    _geoToVec3d(&mut c1, &mut p1);
    if !(fabs(_pointSquareDist(&mut origin, &mut p1) - 1 as libc::c_int as libc::c_double)
        < 0.000000001f64 * 0.0174532925199432957692369076848861271111)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVec3d.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            b"fabs(_pointSquareDist(&origin, &p1) - 1) < EPSILON_RAD\0" as *const u8
                as *const libc::c_char,
            b"Geo point is on the unit sphere\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut c2: LatLng = {
        let mut init = LatLng {
            lat: 1.57079632679489661923132169163975144f64,
            lng: 0 as libc::c_int as libc::c_double,
        };
        init
    };
    let mut p2: Vec3d = Vec3d {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    _geoToVec3d(&mut c2, &mut p2);
    if !(fabs(_pointSquareDist(&mut p1, &mut p2) - 2 as libc::c_int as libc::c_double)
        < 0.000000001f64 * 0.0174532925199432957692369076848861271111)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVec3d.c\0" as *const u8 as *const libc::c_char,
            57 as libc::c_int,
            b"fabs(_pointSquareDist(&p1, &p2) - 2) < EPSILON_RAD\0" as *const u8
                as *const libc::c_char,
            b"Geo point is on another axis\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut c3: LatLng = {
        let mut init = LatLng {
            lat: 3.14159265358979323846264338327950288f64,
            lng: 0 as libc::c_int as libc::c_double,
        };
        init
    };
    let mut p3: Vec3d = Vec3d {
        x: 0.,
        y: 0.,
        z: 0.,
    };
    _geoToVec3d(&mut c3, &mut p3);
    if !(fabs(_pointSquareDist(&mut p1, &mut p3) - 4 as libc::c_int as libc::c_double)
        < 0.000000001f64 * 0.0174532925199432957692369076848861271111)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testVec3d.c\0" as *const u8 as *const libc::c_char,
            63 as libc::c_int,
            b"fabs(_pointSquareDist(&p1, &p3) - 4) < EPSILON_RAD\0" as *const u8
                as *const libc::c_char,
            b"Geo point is the other side of the sphere\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0() -> libc::c_int {
    currentSuiteName = b"Vec3d\0" as *const u8 as *const libc::c_char;
    printf(
        b"TEST %s\n\0" as *const u8 as *const libc::c_char,
        b"Vec3d\0" as *const u8 as *const libc::c_char,
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
