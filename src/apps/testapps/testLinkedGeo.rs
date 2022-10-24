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
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn destroyLinkedMultiPolygon(polygon: *mut LinkedGeoPolygon);
    fn setGeoDegs(p: *mut LatLng, latDegs: libc::c_double, lngDegs: libc::c_double);
    fn addNewLinkedPolygon(polygon: *mut LinkedGeoPolygon) -> *mut LinkedGeoPolygon;
    fn addNewLinkedLoop(polygon: *mut LinkedGeoPolygon) -> *mut LinkedGeoLoop;
    fn addLinkedCoord(loop_0: *mut LinkedGeoLoop, vertex: *const LatLng) -> *mut LinkedLatLng;
    fn countLinkedPolygons(polygon: *mut LinkedGeoPolygon) -> libc::c_int;
    fn countLinkedLoops(polygon: *mut LinkedGeoPolygon) -> libc::c_int;
    fn countLinkedCoords(loop_0: *mut LinkedGeoLoop) -> libc::c_int;
    static mut globalTestCount: libc::c_int;
    static mut currentSuiteName: *const libc::c_char;
    static mut currentTestName: *const libc::c_char;
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
pub struct LinkedLatLng {
    pub vertex: LatLng,
    pub next: *mut LinkedLatLng,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LinkedGeoLoop {
    pub first: *mut LinkedLatLng,
    pub last: *mut LinkedLatLng,
    pub next: *mut LinkedGeoLoop,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LinkedGeoPolygon {
    pub first: *mut LinkedGeoLoop,
    pub last: *mut LinkedGeoLoop,
    pub next: *mut LinkedGeoPolygon,
}
static mut vertex1: LatLng = LatLng { lat: 0., lng: 0. };
static mut vertex2: LatLng = LatLng { lat: 0., lng: 0. };
static mut vertex3: LatLng = LatLng { lat: 0., lng: 0. };
static mut vertex4: LatLng = LatLng { lat: 0., lng: 0. };
unsafe extern "C" fn runTests() {
    setGeoDegs(&mut vertex1, 87.372002166f64, 166.160981117f64);
    setGeoDegs(&mut vertex2, 87.370101364f64, 166.160184306f64);
    setGeoDegs(&mut vertex3, 87.369088356f64, 166.196239997f64);
    setGeoDegs(&mut vertex4, 87.369975080f64, 166.233115768f64);
    currentTestName = b"createLinkedGeo\0" as *const u8 as *const libc::c_char;
    let mut polygon: *mut LinkedGeoPolygon = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<LinkedGeoPolygon>() as libc::c_ulong,
    ) as *mut LinkedGeoPolygon;
    let mut loop_0: *mut LinkedGeoLoop = std::ptr::null_mut::<LinkedGeoLoop>();
    let mut coord: *mut LinkedLatLng = std::ptr::null_mut::<LinkedLatLng>();
    loop_0 = addNewLinkedLoop(polygon);
    if loop_0.is_null() {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLinkedGeo.c\0" as *const u8 as *const libc::c_char,
            43 as libc::c_int,
            b"loop != NULL\0" as *const u8 as *const libc::c_char,
            b"Loop created\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    coord = addLinkedCoord(loop_0, &mut vertex1);
    if coord.is_null() {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLinkedGeo.c\0" as *const u8 as *const libc::c_char,
            45 as libc::c_int,
            b"coord != NULL\0" as *const u8 as *const libc::c_char,
            b"Coord created\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    coord = addLinkedCoord(loop_0, &mut vertex2);
    if coord.is_null() {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLinkedGeo.c\0" as *const u8 as *const libc::c_char,
            47 as libc::c_int,
            b"coord != NULL\0" as *const u8 as *const libc::c_char,
            b"Coord created\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    coord = addLinkedCoord(loop_0, &mut vertex3);
    if coord.is_null() {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLinkedGeo.c\0" as *const u8 as *const libc::c_char,
            49 as libc::c_int,
            b"coord != NULL\0" as *const u8 as *const libc::c_char,
            b"Coord created\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    loop_0 = addNewLinkedLoop(polygon);
    if loop_0.is_null() {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLinkedGeo.c\0" as *const u8 as *const libc::c_char,
            52 as libc::c_int,
            b"loop != NULL\0" as *const u8 as *const libc::c_char,
            b"Loop createed\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    coord = addLinkedCoord(loop_0, &mut vertex2);
    if coord.is_null() {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLinkedGeo.c\0" as *const u8 as *const libc::c_char,
            54 as libc::c_int,
            b"coord != NULL\0" as *const u8 as *const libc::c_char,
            b"Coord created\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    coord = addLinkedCoord(loop_0, &mut vertex4);
    if coord.is_null() {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLinkedGeo.c\0" as *const u8 as *const libc::c_char,
            56 as libc::c_int,
            b"coord != NULL\0" as *const u8 as *const libc::c_char,
            b"Coord created\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if countLinkedPolygons(polygon) != 1 as libc::c_int {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLinkedGeo.c\0" as *const u8 as *const libc::c_char,
            58 as libc::c_int,
            b"countLinkedPolygons(polygon) == 1\0" as *const u8 as *const libc::c_char,
            b"Polygon count correct\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if countLinkedLoops(polygon) != 2 as libc::c_int {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLinkedGeo.c\0" as *const u8 as *const libc::c_char,
            59 as libc::c_int,
            b"countLinkedLoops(polygon) == 2\0" as *const u8 as *const libc::c_char,
            b"Loop count correct\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if countLinkedCoords((*polygon).first) != 3 as libc::c_int {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLinkedGeo.c\0" as *const u8 as *const libc::c_char,
            61 as libc::c_int,
            b"countLinkedCoords(polygon->first) == 3\0" as *const u8 as *const libc::c_char,
            b"Coord count 1 correct\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if countLinkedCoords((*polygon).last) != 2 as libc::c_int {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLinkedGeo.c\0" as *const u8 as *const libc::c_char,
            63 as libc::c_int,
            b"countLinkedCoords(polygon->last) == 2\0" as *const u8 as *const libc::c_char,
            b"Coord count 2 correct\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut nextPolygon: *mut LinkedGeoPolygon = addNewLinkedPolygon(polygon);
    if nextPolygon.is_null() {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLinkedGeo.c\0" as *const u8 as *const libc::c_char,
            66 as libc::c_int,
            b"nextPolygon != NULL\0" as *const u8 as *const libc::c_char,
            b"polygon created\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if countLinkedPolygons(polygon) != 2 as libc::c_int {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testLinkedGeo.c\0" as *const u8 as *const libc::c_char,
            68 as libc::c_int,
            b"countLinkedPolygons(polygon) == 2\0" as *const u8 as *const libc::c_char,
            b"Polygon count correct\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    destroyLinkedMultiPolygon(polygon);
    free(polygon as *mut libc::c_void);
}
unsafe fn main_0() -> libc::c_int {
    currentSuiteName = b"linkedGeo\0" as *const u8 as *const libc::c_char;
    printf(
        b"TEST %s\n\0" as *const u8 as *const libc::c_char,
        b"linkedGeo\0" as *const u8 as *const libc::c_char,
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
