extern crate unsafe_h3lib_testapps;
use ::libc;
extern "C" {

    fn fabs(_: libc::c_double) -> libc::c_double;
    fn exit(_: libc::c_int) -> !;
    fn degsToRads(degrees: libc::c_double) -> libc::c_double;
    fn radsToDegs(radians: libc::c_double) -> libc::c_double;
    fn greatCircleDistanceRads(a: *const LatLng, b: *const LatLng) -> libc::c_double;
    fn getHexagonAreaAvgKm2(res: libc::c_int, out: *mut libc::c_double) -> H3Error;
    fn getHexagonAreaAvgM2(res: libc::c_int, out: *mut libc::c_double) -> H3Error;
    fn getHexagonEdgeLengthAvgKm(res: libc::c_int, out: *mut libc::c_double) -> H3Error;
    fn getHexagonEdgeLengthAvgM(res: libc::c_int, out: *mut libc::c_double) -> H3Error;
    fn getNumCells(res: libc::c_int, out: *mut int64_t) -> H3Error;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn constrainLat(lat: libc::c_double) -> libc::c_double;
    fn setGeoDegs(p: *mut LatLng, latDegs: libc::c_double, lngDegs: libc::c_double);
    fn constrainLng(lng: libc::c_double) -> libc::c_double;
    fn geoAlmostEqual(p1: *const LatLng, p2: *const LatLng) -> bool;
    fn geoAlmostEqualThreshold(
        p1: *const LatLng,
        p2: *const LatLng,
        threshold: libc::c_double,
    ) -> bool;
    fn _geoAzDistanceRads(
        p1: *const LatLng,
        az: libc::c_double,
        distance: libc::c_double,
        p2: *mut LatLng,
    );
    static mut globalTestCount: libc::c_int;
    static mut currentSuiteName: *const libc::c_char;
    static mut currentTestName: *const libc::c_char;
}
pub type int64_t = libc::c_longlong;
pub type uint32_t = libc::c_uint;
pub type __int64_t = libc::c_longlong;
pub type __darwin_off_t = __int64_t;
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
unsafe extern "C" fn testDecreasingFunction(
    mut function: Option<unsafe extern "C" fn(libc::c_int, *mut libc::c_double) -> H3Error>,
    mut message: *const libc::c_char,
) {
    let mut last: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut next: libc::c_double = 0.;
    let mut i: libc::c_int = 15 as libc::c_int;
    while i >= 0 as libc::c_int {
        if function.expect("non-null function pointer")(i, &mut next) != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
                43 as libc::c_int,
                b"!(function(i, &next))\0" as *const u8 as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        if !(next > last) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
                44 as libc::c_int,
                b"next > last\0" as *const u8 as *const libc::c_char,
                message,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        last = next;
        i -= 1;
    }
}
unsafe extern "C" fn runTests() {
    currentTestName = b"radsToDegs\0" as *const u8 as *const libc::c_char;
    let mut originalRads: libc::c_double = 1 as libc::c_int as libc::c_double;
    let mut degs: libc::c_double = radsToDegs(originalRads);
    let mut rads: libc::c_double = degsToRads(degs);
    if !(fabs(rads - originalRads) < (0.000000001f64 * 0.0174532925199432957692369076848861271111))
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
            55 as libc::c_int,
            b"fabs(rads - originalRads) < EPSILON_RAD\0" as *const u8 as *const libc::c_char,
            b"radsToDegs/degsToRads invertible\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"distanceRads\0" as *const u8 as *const libc::c_char;
    let mut p1: LatLng = LatLng { lat: 0., lng: 0. };
    setGeoDegs(
        &mut p1,
        10 as libc::c_int as libc::c_double,
        10 as libc::c_int as libc::c_double,
    );
    let mut p2: LatLng = LatLng { lat: 0., lng: 0. };
    setGeoDegs(
        &mut p2,
        0 as libc::c_int as libc::c_double,
        10 as libc::c_int as libc::c_double,
    );
    if !(greatCircleDistanceRads(&mut p1, &mut p1)
        < 0.000000001f64 * 0.0174532925199432957692369076848861271111 * 1000 as f64)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
            67 as libc::c_int,
            b"H3_EXPORT(greatCircleDistanceRads)(&p1, &p1) < EPSILON_RAD * 1000\0" as *const u8
                as *const libc::c_char,
            b"0 distance as expected\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(fabs(
        greatCircleDistanceRads(&mut p1, &mut p2) - degsToRads(10 as libc::c_int as libc::c_double),
    ) < 0.000000001f64 * 0.0174532925199432957692369076848861271111 * 1000 as f64)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8
                as *const libc::c_char,
            70 as libc::c_int,
            b"fabs(H3_EXPORT(greatCircleDistanceRads)(&p1, &p2) - H3_EXPORT(degsToRads)(10)) < EPSILON_RAD * 1000\0"
                as *const u8 as *const libc::c_char,
            b"distance along longitude as expected\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"geoAlmostEqualThreshold\0" as *const u8 as *const libc::c_char;
    let mut a: LatLng = {
        let mut init = LatLng {
            lat: 15 as libc::c_int as libc::c_double,
            lng: 10 as libc::c_int as libc::c_double,
        };
        init
    };
    let mut b: LatLng = {
        let mut init = LatLng {
            lat: 15 as libc::c_int as libc::c_double,
            lng: 10 as libc::c_int as libc::c_double,
        };
        init
    };
    if !geoAlmostEqualThreshold(&mut a, &mut b, 2.2204460492503131e-16f64) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
            76 as libc::c_int,
            b"geoAlmostEqualThreshold(&a, &b, DBL_EPSILON)\0" as *const u8 as *const libc::c_char,
            b"same point\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    b.lat = 15.00001f64;
    b.lng = 10.00002f64;
    if !geoAlmostEqualThreshold(&mut a, &mut b, 0.0001f64) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
            81 as libc::c_int,
            b"geoAlmostEqualThreshold(&a, &b, 0.0001)\0" as *const u8 as *const libc::c_char,
            b"differences under threshold\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    b.lat = 15.00001f64;
    b.lng = 10 as libc::c_int as libc::c_double;
    if geoAlmostEqualThreshold(&mut a, &mut b, 0.000001f64) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
            86 as libc::c_int,
            b"!geoAlmostEqualThreshold(&a, &b, 0.000001)\0" as *const u8 as *const libc::c_char,
            b"lat over threshold\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    b.lat = 15 as libc::c_int as libc::c_double;
    b.lng = 10.00001f64;
    if geoAlmostEqualThreshold(&mut a, &mut b, 0.000001f64) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
            91 as libc::c_int,
            b"!geoAlmostEqualThreshold(&a, &b, 0.000001)\0" as *const u8 as *const libc::c_char,
            b"lng over threshold\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"constrainLatLng\0" as *const u8 as *const libc::c_char;
    if !(constrainLat(0 as libc::c_int as libc::c_double) == 0 as libc::c_int as libc::c_double) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
            95 as libc::c_int,
            b"constrainLat(0) == 0\0" as *const u8 as *const libc::c_char,
            b"lat 0\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(constrainLat(1 as libc::c_int as libc::c_double) == 1 as libc::c_int as libc::c_double) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
            96 as libc::c_int,
            b"constrainLat(1) == 1\0" as *const u8 as *const libc::c_char,
            b"lat 1\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(constrainLat(1.57079632679489661923132169163975144f64)
        == 1.57079632679489661923132169163975144f64)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int,
            b"constrainLat(M_PI_2) == M_PI_2\0" as *const u8 as *const libc::c_char,
            b"lat pi/2\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(constrainLat(3.14159265358979323846264338327950288f64)
        == 0 as libc::c_int as libc::c_double)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
            98 as libc::c_int,
            b"constrainLat(M_PI) == 0\0" as *const u8 as *const libc::c_char,
            b"lat pi\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(constrainLat(
        3.14159265358979323846264338327950288f64 + 1 as libc::c_int as libc::c_double,
    ) == 1 as libc::c_int as libc::c_double)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
            99 as libc::c_int,
            b"constrainLat(M_PI + 1) == 1\0" as *const u8 as *const libc::c_char,
            b"lat pi+1\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(constrainLat(
        2 as libc::c_int as libc::c_double * 3.14159265358979323846264338327950288f64
            + 1 as libc::c_int as libc::c_double,
    ) == 1 as libc::c_int as libc::c_double)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
            100 as libc::c_int,
            b"constrainLat(2 * M_PI + 1) == 1\0" as *const u8 as *const libc::c_char,
            b"lat 2pi+1\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(constrainLng(0 as libc::c_int as libc::c_double) == 0 as libc::c_int as libc::c_double) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
            102 as libc::c_int,
            b"constrainLng(0) == 0\0" as *const u8 as *const libc::c_char,
            b"lng 0\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(constrainLng(1 as libc::c_int as libc::c_double) == 1 as libc::c_int as libc::c_double) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
            103 as libc::c_int,
            b"constrainLng(1) == 1\0" as *const u8 as *const libc::c_char,
            b"lng 1\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(constrainLng(3.14159265358979323846264338327950288f64)
        == 3.14159265358979323846264338327950288f64)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
            104 as libc::c_int,
            b"constrainLng(M_PI) == M_PI\0" as *const u8 as *const libc::c_char,
            b"lng pi\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(constrainLng(
        2 as libc::c_int as libc::c_double * 3.14159265358979323846264338327950288f64,
    ) == 0 as libc::c_int as libc::c_double)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
            105 as libc::c_int,
            b"constrainLng(2 * M_PI) == 0\0" as *const u8 as *const libc::c_char,
            b"lng 2pi\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(constrainLng(
        3 as libc::c_int as libc::c_double * 3.14159265358979323846264338327950288f64,
    ) == 3.14159265358979323846264338327950288f64)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
            106 as libc::c_int,
            b"constrainLng(3 * M_PI) == M_PI\0" as *const u8 as *const libc::c_char,
            b"lng 2pi\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(constrainLng(
        4 as libc::c_int as libc::c_double * 3.14159265358979323846264338327950288f64,
    ) == 0 as libc::c_int as libc::c_double)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
            107 as libc::c_int,
            b"constrainLng(4 * M_PI) == 0\0" as *const u8 as *const libc::c_char,
            b"lng 4pi\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"_geoAzDistanceRads_noop\0" as *const u8 as *const libc::c_char;
    let mut start: LatLng = {
        let mut init = LatLng {
            lat: 15 as libc::c_int as libc::c_double,
            lng: 10 as libc::c_int as libc::c_double,
        };
        init
    };
    let mut out: LatLng = LatLng { lat: 0., lng: 0. };
    let mut expected: LatLng = {
        let mut init = LatLng {
            lat: 15 as libc::c_int as libc::c_double,
            lng: 10 as libc::c_int as libc::c_double,
        };
        init
    };
    _geoAzDistanceRads(
        &mut start,
        0 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
        &mut out,
    );
    if !geoAlmostEqual(&mut expected, &mut out) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
            117 as libc::c_int,
            b"geoAlmostEqual(&expected, &out)\0" as *const u8 as *const libc::c_char,
            b"0 distance produces same point\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"_geoAzDistanceRads_dueNorthSouth\0" as *const u8 as *const libc::c_char;
    let mut start_0: LatLng = LatLng { lat: 0., lng: 0. };
    let mut out_0: LatLng = LatLng { lat: 0., lng: 0. };
    let mut expected_0: LatLng = LatLng { lat: 0., lng: 0. };
    setGeoDegs(
        &mut start_0,
        45 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
    );
    setGeoDegs(
        &mut expected_0,
        90 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
    );
    _geoAzDistanceRads(
        &mut start_0,
        0 as libc::c_int as libc::c_double,
        degsToRads(45 as libc::c_int as libc::c_double),
        &mut out_0,
    );
    if !geoAlmostEqual(&mut expected_0, &mut out_0) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
            130 as libc::c_int,
            b"geoAlmostEqual(&expected, &out)\0" as *const u8 as *const libc::c_char,
            b"due north to north pole produces north pole\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    setGeoDegs(
        &mut start_0,
        45 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
    );
    setGeoDegs(
        &mut expected_0,
        270 as libc::c_int as libc::c_double,
        1 as libc::c_int as libc::c_double,
    );
    _geoAzDistanceRads(
        &mut start_0,
        0 as libc::c_int as libc::c_double,
        degsToRads((45 as libc::c_int + 180 as libc::c_int) as libc::c_double),
        &mut out_0,
    );
    if !geoAlmostEqual(&mut expected_0, &mut out_0) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
            137 as libc::c_int,
            b"geoAlmostEqual(&expected, &out)\0" as *const u8 as *const libc::c_char,
            b"due north to south pole produces south pole\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    setGeoDegs(
        &mut start_0,
        -(45 as libc::c_int) as libc::c_double,
        2 as libc::c_int as libc::c_double,
    );
    setGeoDegs(
        &mut expected_0,
        -(90 as libc::c_int) as libc::c_double,
        0 as libc::c_int as libc::c_double,
    );
    _geoAzDistanceRads(
        &mut start_0,
        degsToRads(180 as libc::c_int as libc::c_double),
        degsToRads(45 as libc::c_int as libc::c_double),
        &mut out_0,
    );
    if !geoAlmostEqual(&mut expected_0, &mut out_0) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
            145 as libc::c_int,
            b"geoAlmostEqual(&expected, &out)\0" as *const u8 as *const libc::c_char,
            b"due south to south pole produces south pole\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    setGeoDegs(
        &mut start_0,
        -(45 as libc::c_int) as libc::c_double,
        10 as libc::c_int as libc::c_double,
    );
    setGeoDegs(
        &mut expected_0,
        -(10 as libc::c_int) as libc::c_double,
        10 as libc::c_int as libc::c_double,
    );
    _geoAzDistanceRads(
        &mut start_0,
        0 as libc::c_int as libc::c_double,
        degsToRads(35 as libc::c_int as libc::c_double),
        &mut out_0,
    );
    if !geoAlmostEqual(&mut expected_0, &mut out_0) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
            152 as libc::c_int,
            b"geoAlmostEqual(&expected, &out)\0" as *const u8 as *const libc::c_char,
            b"due north produces expected result\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"_geoAzDistanceRads_poleToPole\0" as *const u8 as *const libc::c_char;
    let mut start_1: LatLng = LatLng { lat: 0., lng: 0. };
    let mut out_1: LatLng = LatLng { lat: 0., lng: 0. };
    let mut expected_1: LatLng = LatLng { lat: 0., lng: 0. };
    setGeoDegs(
        &mut start_1,
        90 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
    );
    setGeoDegs(
        &mut expected_1,
        -(90 as libc::c_int) as libc::c_double,
        0 as libc::c_int as libc::c_double,
    );
    _geoAzDistanceRads(
        &mut start_1,
        degsToRads(12 as libc::c_int as libc::c_double),
        degsToRads(180 as libc::c_int as libc::c_double),
        &mut out_1,
    );
    if !geoAlmostEqual(&mut expected_1, &mut out_1) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
            168 as libc::c_int,
            b"geoAlmostEqual(&expected, &out)\0" as *const u8 as *const libc::c_char,
            b"some direction to south pole produces south pole\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    setGeoDegs(
        &mut start_1,
        -(90 as libc::c_int) as libc::c_double,
        0 as libc::c_int as libc::c_double,
    );
    setGeoDegs(
        &mut expected_1,
        90 as libc::c_int as libc::c_double,
        0 as libc::c_int as libc::c_double,
    );
    _geoAzDistanceRads(
        &mut start_1,
        degsToRads(34 as libc::c_int as libc::c_double),
        degsToRads(180 as libc::c_int as libc::c_double),
        &mut out_1,
    );
    if !geoAlmostEqual(&mut expected_1, &mut out_1) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            b"geoAlmostEqual(&expected, &out)\0" as *const u8 as *const libc::c_char,
            b"some direction to north pole produces north pole\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"_geoAzDistanceRads_invertible\0" as *const u8 as *const libc::c_char;
    let mut start_2: LatLng = LatLng { lat: 0., lng: 0. };
    setGeoDegs(
        &mut start_2,
        15 as libc::c_int as libc::c_double,
        10 as libc::c_int as libc::c_double,
    );
    let mut out_2: LatLng = LatLng { lat: 0., lng: 0. };
    let mut azimuth: libc::c_double = degsToRads(20 as libc::c_int as libc::c_double);
    let mut degrees180: libc::c_double = degsToRads(180 as libc::c_int as libc::c_double);
    let mut distance: libc::c_double = degsToRads(15 as libc::c_int as libc::c_double);
    _geoAzDistanceRads(&mut start_2, azimuth, distance, &mut out_2);
    if !(fabs(greatCircleDistanceRads(&mut start_2, &mut out_2) - distance)
        < 0.000000001f64 * 0.0174532925199432957692369076848861271111)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
            190 as libc::c_int,
            b"fabs(H3_EXPORT(greatCircleDistanceRads)(&start, &out) - distance) < EPSILON_RAD\0"
                as *const u8 as *const libc::c_char,
            b"moved distance is as expected\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut start2: LatLng = out_2;
    _geoAzDistanceRads(&mut start2, azimuth + degrees180, distance, &mut out_2);
    if !(greatCircleDistanceRads(&mut start_2, &mut out_2) < 0.01f64) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
            196 as libc::c_int,
            b"H3_EXPORT(greatCircleDistanceRads)(&start, &out) < 0.01\0" as *const u8
                as *const libc::c_char,
            b"moved back to origin\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"distanceRads_wrappedLongitude\0" as *const u8 as *const libc::c_char;
    let negativeLongitude: LatLng = {
        let mut init = LatLng {
            lat: 0 as libc::c_int as libc::c_double,
            lng: -(3.14159265358979323846264338327950288f64
                + 1.57079632679489661923132169163975144f64),
        };
        init
    };
    let zero: LatLng = {
        let mut init = LatLng {
            lat: 0 as libc::c_int as libc::c_double,
            lng: 0 as libc::c_int as libc::c_double,
        };
        init
    };
    if !(fabs(
        1.57079632679489661923132169163975144f64
            - greatCircleDistanceRads(&negativeLongitude, &zero),
    ) < 0.000000001f64 * 0.0174532925199432957692369076848861271111)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8
                as *const libc::c_char,
            205 as libc::c_int,
            b"fabs(M_PI_2 - H3_EXPORT(greatCircleDistanceRads)( &negativeLongitude, &zero)) < EPSILON_RAD\0"
                as *const u8 as *const libc::c_char,
            b"Distance with wrapped longitude\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(fabs(
        1.57079632679489661923132169163975144f64
            - greatCircleDistanceRads(&zero, &negativeLongitude),
    ) < 0.000000001f64 * 0.0174532925199432957692369076848861271111)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8
                as *const libc::c_char,
            208 as libc::c_int,
            b"fabs(M_PI_2 - H3_EXPORT(greatCircleDistanceRads)( &zero, &negativeLongitude)) < EPSILON_RAD\0"
                as *const u8 as *const libc::c_char,
            b"Distance with wrapped longitude and swapped arguments\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"doubleConstants\0" as *const u8 as *const libc::c_char;
    testDecreasingFunction(
        Some(
            getHexagonAreaAvgKm2
                as unsafe extern "C" fn(libc::c_int, *mut libc::c_double) -> H3Error,
        ),
        b"getHexagonAreaAvgKm2 ordering\0" as *const u8 as *const libc::c_char,
    );
    testDecreasingFunction(
        Some(
            getHexagonAreaAvgM2
                as unsafe extern "C" fn(libc::c_int, *mut libc::c_double) -> H3Error,
        ),
        b"getHexagonAreaAvgM2 ordering\0" as *const u8 as *const libc::c_char,
    );
    testDecreasingFunction(
        Some(
            getHexagonEdgeLengthAvgKm
                as unsafe extern "C" fn(libc::c_int, *mut libc::c_double) -> H3Error,
        ),
        b"getHexagonEdgeLengthAvgKm ordering\0" as *const u8 as *const libc::c_char,
    );
    testDecreasingFunction(
        Some(
            getHexagonEdgeLengthAvgM
                as unsafe extern "C" fn(libc::c_int, *mut libc::c_double) -> H3Error,
        ),
        b"getHexagonEdgeLengthAvgM ordering\0" as *const u8 as *const libc::c_char,
    );
    currentTestName = b"doubleConstantsErrors\0" as *const u8 as *const libc::c_char;
    let mut out_3: libc::c_double = 0.;
    if !(getHexagonAreaAvgKm2(-(1 as libc::c_int), &mut out_3)
        == E_RES_DOMAIN as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
            226 as libc::c_int,
            b"H3_EXPORT(getHexagonAreaAvgKm2)(-1, &out) == E_RES_DOMAIN\0" as *const u8
                as *const libc::c_char,
            b"getHexagonAreaAvgKm2 resolution negative\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(getHexagonAreaAvgKm2(16 as libc::c_int, &mut out_3)
        == E_RES_DOMAIN as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
            228 as libc::c_int,
            b"H3_EXPORT(getHexagonAreaAvgKm2)(16, &out) == E_RES_DOMAIN\0" as *const u8
                as *const libc::c_char,
            b"getHexagonAreaAvgKm2 resolution too high\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(getHexagonAreaAvgM2(-(1 as libc::c_int), &mut out_3)
        == E_RES_DOMAIN as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
            230 as libc::c_int,
            b"H3_EXPORT(getHexagonAreaAvgM2)(-1, &out) == E_RES_DOMAIN\0" as *const u8
                as *const libc::c_char,
            b"getHexagonAreaAvgM2 resolution negative\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(getHexagonAreaAvgM2(16 as libc::c_int, &mut out_3)
        == E_RES_DOMAIN as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
            232 as libc::c_int,
            b"H3_EXPORT(getHexagonAreaAvgM2)(16, &out) == E_RES_DOMAIN\0" as *const u8
                as *const libc::c_char,
            b"getHexagonAreaAvgM2 resolution too high\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(getHexagonEdgeLengthAvgKm(-(1 as libc::c_int), &mut out_3)
        == E_RES_DOMAIN as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
            234 as libc::c_int,
            b"H3_EXPORT(getHexagonEdgeLengthAvgKm)(-1, &out) == E_RES_DOMAIN\0" as *const u8
                as *const libc::c_char,
            b"getHexagonEdgeLengthAvgKm resolution negative\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(getHexagonEdgeLengthAvgKm(16 as libc::c_int, &mut out_3)
        == E_RES_DOMAIN as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
            236 as libc::c_int,
            b"H3_EXPORT(getHexagonEdgeLengthAvgKm)(16, &out) == E_RES_DOMAIN\0" as *const u8
                as *const libc::c_char,
            b"getHexagonEdgeLengthAvgKm resolution too high\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(getHexagonEdgeLengthAvgM(-(1 as libc::c_int), &mut out_3)
        == E_RES_DOMAIN as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
            238 as libc::c_int,
            b"H3_EXPORT(getHexagonEdgeLengthAvgM)(-1, &out) == E_RES_DOMAIN\0" as *const u8
                as *const libc::c_char,
            b"getHexagonEdgeLengthAvgM resolution negative\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(getHexagonEdgeLengthAvgM(16 as libc::c_int, &mut out_3)
        == E_RES_DOMAIN as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
            240 as libc::c_int,
            b"H3_EXPORT(getHexagonEdgeLengthAvgM)(16, &out) == E_RES_DOMAIN\0" as *const u8
                as *const libc::c_char,
            b"getHexagonEdgeLengthAvgM resolution too high\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"intConstants\0" as *const u8 as *const libc::c_char;
    let mut last: int64_t = 0 as libc::c_int as int64_t;
    let mut next: int64_t = 0;
    let mut i: libc::c_int = 0 as libc::c_int;
    while i <= 15 as libc::c_int {
        if getNumCells(i, &mut next) != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
                248 as libc::c_int,
                b"!(getNumCells(i, &next))\0" as *const u8 as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        if !(next > last) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
                249 as libc::c_int,
                b"next > last\0" as *const u8 as *const libc::c_char,
                b"getNumCells ordering\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        last = next;
        i += 1;
    }
    currentTestName = b"intConstantsErrors\0" as *const u8 as *const libc::c_char;
    let mut out_4: int64_t = 0;
    if !(getNumCells(-(1 as libc::c_int), &mut out_4)
        == E_RES_DOMAIN as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
            257 as libc::c_int,
            b"H3_EXPORT(getNumCells)(-1, &out) == E_RES_DOMAIN\0" as *const u8
                as *const libc::c_char,
            b"getNumCells resolution negative\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(getNumCells(16 as libc::c_int, &mut out_4) == E_RES_DOMAIN as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
            259 as libc::c_int,
            b"H3_EXPORT(getNumCells)(16, &out) == E_RES_DOMAIN\0" as *const u8
                as *const libc::c_char,
            b"getNumCells resolution too high\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"numHexagons\0" as *const u8 as *const libc::c_char;
    static mut expected_2: [int64_t; 16] = [
        122 as libc::c_long as int64_t,
        842 as libc::c_long as int64_t,
        5882 as libc::c_long as int64_t,
        41162 as libc::c_long as int64_t,
        288122 as libc::c_long as int64_t,
        2016842 as libc::c_long as int64_t,
        14117882 as libc::c_long as int64_t,
        98825162 as libc::c_long as int64_t,
        691776122 as libc::c_long as int64_t,
        4842432842 as libc::c_long as int64_t,
        33897029882 as libc::c_long as int64_t,
        237279209162 as libc::c_long as int64_t,
        1660954464122 as libc::c_long as int64_t,
        11626681248842 as libc::c_long as int64_t,
        81386768741882 as libc::c_long as int64_t,
        569707381193162 as libc::c_long as int64_t,
    ];
    let mut r: libc::c_int = 0 as libc::c_int;
    while r <= 15 as libc::c_int {
        let mut num: int64_t = 0;
        if getNumCells(r, &mut num) != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
                283 as libc::c_int,
                b"!(getNumCells(r, &num))\0" as *const u8 as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        if !(num == expected_2[r as usize]) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testLatLng.c\0" as *const u8 as *const libc::c_char,
                284 as libc::c_int,
                b"num == expected[r]\0" as *const u8 as *const libc::c_char,
                b"incorrect numHexagons count\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        r += 1;
    }
}
unsafe fn main_0() -> libc::c_int {
    currentSuiteName = b"latLng\0" as *const u8 as *const libc::c_char;
    printf(
        b"TEST %s\n\0" as *const u8 as *const libc::c_char,
        b"latLng\0" as *const u8 as *const libc::c_char,
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
