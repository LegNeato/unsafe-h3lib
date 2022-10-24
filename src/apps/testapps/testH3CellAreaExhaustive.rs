#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(clippy::missing_safety_doc)]

extern crate unsafe_h3lib_testapps;
use ::libc;
use ::num_traits;
use num_traits::ToPrimitive;
extern "C" {

    fn fabs(_: libc::c_double) -> libc::c_double;
    fn exit(_: libc::c_int) -> !;
    fn getDirectedEdgeDestination(edge: H3Index, out: *mut H3Index) -> H3Error;
    fn getDirectedEdgeOrigin(edge: H3Index, out: *mut H3Index) -> H3Error;
    fn cellToLatLng(h3: H3Index, g: *mut LatLng) -> H3Error;
    fn greatCircleDistanceRads(a: *const LatLng, b: *const LatLng) -> libc::c_double;
    fn greatCircleDistanceKm(a: *const LatLng, b: *const LatLng) -> libc::c_double;
    fn greatCircleDistanceM(a: *const LatLng, b: *const LatLng) -> libc::c_double;
    fn cellAreaRads2(h: H3Index, out: *mut libc::c_double) -> H3Error;
    fn cellAreaKm2(h: H3Index, out: *mut libc::c_double) -> H3Error;
    fn cellAreaM2(h: H3Index, out: *mut libc::c_double) -> H3Error;
    fn edgeLengthRads(edge: H3Index, length: *mut libc::c_double) -> H3Error;
    fn edgeLengthKm(edge: H3Index, length: *mut libc::c_double) -> H3Error;
    fn edgeLengthM(edge: H3Index, length: *mut libc::c_double) -> H3Error;
    fn iterInitRes(res: libc::c_int) -> IterCellsResolution;
    fn iterStepRes(iter: *mut IterCellsResolution);
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    static mut currentTestName: *const libc::c_char;
    static mut currentSuiteName: *const libc::c_char;
    static mut globalTestCount: libc::c_int;
    fn iterateAllIndexesAtRes(
        res: libc::c_int,
        callback: Option<unsafe extern "C" fn(H3Index) -> ()>,
    );
    fn iterateAllDirectedEdgesAtRes(
        res: libc::c_int,
        callback: Option<unsafe extern "C" fn(H3Index) -> ()>,
    );
}
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
pub type __int64_t = libc::c_longlong;
pub type __darwin_off_t = __int64_t;
pub type H3Index = uint64_t;
pub type H3Error = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LatLng {
    pub lat: libc::c_double,
    pub lng: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IterCellsChildren {
    pub h: H3Index,
    pub _parentRes: libc::c_int,
    pub _skipDigit: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IterCellsResolution {
    pub h: H3Index,
    pub _baseCellNum: libc::c_int,
    pub _res: libc::c_int,
    pub _itC: IterCellsChildren,
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
unsafe extern "C" fn haversine_assert(mut edge: H3Index) {
    let mut a: LatLng = LatLng { lat: 0., lng: 0. };
    let mut b: LatLng = LatLng { lat: 0., lng: 0. };
    let mut origin: H3Index = 0;
    let mut destination: H3Index = 0;
    if getDirectedEdgeOrigin(edge, &mut origin) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3CellAreaExhaustive.c\0" as *const u8 as *const libc::c_char,
            47 as libc::c_int,
            b"!(getDirectedEdgeOrigin(edge, &origin))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if cellToLatLng(origin, &mut a) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3CellAreaExhaustive.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            b"!(cellToLatLng(origin, &a))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if getDirectedEdgeDestination(edge, &mut destination) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3CellAreaExhaustive.c\0" as *const u8 as *const libc::c_char,
            50 as libc::c_int,
            b"!(getDirectedEdgeDestination(edge, &destination))\0" as *const u8
                as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if cellToLatLng(destination, &mut b) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3CellAreaExhaustive.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            b"!(cellToLatLng(destination, &b))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut ab: libc::c_double = 0.;
    let mut ba: libc::c_double = 0.;
    ab = greatCircleDistanceRads(&mut a, &mut b);
    ba = greatCircleDistanceRads(&mut b, &mut a);
    if !(ab > 0 as libc::c_int as libc::c_double) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3CellAreaExhaustive.c\0" as *const u8 as *const libc::c_char,
            60 as libc::c_int,
            b"ab > 0\0" as *const u8 as *const libc::c_char,
            b"distance between cell centers should be positive\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(ab == ba) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3CellAreaExhaustive.c\0" as *const u8 as *const libc::c_char,
            61 as libc::c_int,
            b"ab == ba\0" as *const u8 as *const libc::c_char,
            b"pairwise cell distances should be commutative\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    ab = greatCircleDistanceKm(&mut a, &mut b);
    ba = greatCircleDistanceKm(&mut b, &mut a);
    if !(ab > 0 as libc::c_int as libc::c_double) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3CellAreaExhaustive.c\0" as *const u8 as *const libc::c_char,
            65 as libc::c_int,
            b"ab > 0\0" as *const u8 as *const libc::c_char,
            b"distance between cell centers should be positive\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(ab == ba) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3CellAreaExhaustive.c\0" as *const u8 as *const libc::c_char,
            66 as libc::c_int,
            b"ab == ba\0" as *const u8 as *const libc::c_char,
            b"pairwise cell distances should be commutative\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    ab = greatCircleDistanceM(&mut a, &mut b);
    ba = greatCircleDistanceM(&mut b, &mut a);
    if !(ab > 0 as libc::c_int as libc::c_double) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3CellAreaExhaustive.c\0" as *const u8 as *const libc::c_char,
            70 as libc::c_int,
            b"ab > 0\0" as *const u8 as *const libc::c_char,
            b"distance between cell centers should be positive\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(ab == ba) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3CellAreaExhaustive.c\0" as *const u8 as *const libc::c_char,
            71 as libc::c_int,
            b"ab == ba\0" as *const u8 as *const libc::c_char,
            b"pairwise cell distances should be commutative\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(greatCircleDistanceKm(&mut a, &mut b) > greatCircleDistanceRads(&mut a, &mut b)) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8
                as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3CellAreaExhaustive.c\0"
                as *const u8 as *const libc::c_char,
            75 as libc::c_int,
            b"H3_EXPORT(greatCircleDistanceKm)(&a, &b) > H3_EXPORT(greatCircleDistanceRads)(&a, &b)\0"
                as *const u8 as *const libc::c_char,
            b"measurement in kilometers should be greater than in radians\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(greatCircleDistanceM(&mut a, &mut b) > greatCircleDistanceKm(&mut a, &mut b)) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3CellAreaExhaustive.c\0" as *const u8 as *const libc::c_char,
            78 as libc::c_int,
            b"H3_EXPORT(greatCircleDistanceM)(&a, &b) > H3_EXPORT(greatCircleDistanceKm)(&a, &b)\0"
                as *const u8 as *const libc::c_char,
            b"measurement in meters should be greater than in kilometers\0" as *const u8
                as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn edge_length_assert(mut edge: H3Index) {
    let mut length: libc::c_double = 0.;
    if edgeLengthRads(edge, &mut length) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3CellAreaExhaustive.c\0" as *const u8 as *const libc::c_char,
            94 as libc::c_int,
            b"!(edgeLengthRads(edge, &length))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(length > 0 as libc::c_int as libc::c_double) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3CellAreaExhaustive.c\0" as *const u8 as *const libc::c_char,
            95 as libc::c_int,
            b"length > 0\0" as *const u8 as *const libc::c_char,
            b"edge has positive length\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if edgeLengthKm(edge, &mut length) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3CellAreaExhaustive.c\0" as *const u8 as *const libc::c_char,
            96 as libc::c_int,
            b"!(edgeLengthKm(edge, &length))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(length > 0 as libc::c_int as libc::c_double) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3CellAreaExhaustive.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int,
            b"length > 0\0" as *const u8 as *const libc::c_char,
            b"edge has positive length\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if edgeLengthM(edge, &mut length) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3CellAreaExhaustive.c\0" as *const u8 as *const libc::c_char,
            98 as libc::c_int,
            b"!(edgeLengthM(edge, &length))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(length > 0 as libc::c_int as libc::c_double) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3CellAreaExhaustive.c\0" as *const u8 as *const libc::c_char,
            99 as libc::c_int,
            b"length > 0\0" as *const u8 as *const libc::c_char,
            b"edge has positive length\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn cell_area_assert(mut cell: H3Index) {
    let mut areaRads: libc::c_double = 0.;
    if cellAreaRads2(cell, &mut areaRads) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3CellAreaExhaustive.c\0" as *const u8 as *const libc::c_char,
            115 as libc::c_int,
            b"!(cellAreaRads2(cell, &areaRads))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(areaRads > 0 as libc::c_int as libc::c_double) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3CellAreaExhaustive.c\0" as *const u8 as *const libc::c_char,
            116 as libc::c_int,
            b"areaRads > 0\0" as *const u8 as *const libc::c_char,
            b"cell has positive area\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut areaKm2: libc::c_double = 0.;
    if cellAreaKm2(cell, &mut areaKm2) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3CellAreaExhaustive.c\0" as *const u8 as *const libc::c_char,
            118 as libc::c_int,
            b"!(cellAreaKm2(cell, &areaKm2))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(areaKm2 > 0 as libc::c_int as libc::c_double) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3CellAreaExhaustive.c\0" as *const u8 as *const libc::c_char,
            119 as libc::c_int,
            b"areaKm2 > 0\0" as *const u8 as *const libc::c_char,
            b"cell has positive area\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut areaM2: libc::c_double = 0.;
    if cellAreaM2(cell, &mut areaM2) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3CellAreaExhaustive.c\0" as *const u8 as *const libc::c_char,
            121 as libc::c_int,
            b"!(cellAreaM2(cell, &areaM2))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(areaM2 > 0 as libc::c_int as libc::c_double) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3CellAreaExhaustive.c\0" as *const u8 as *const libc::c_char,
            122 as libc::c_int,
            b"areaM2 > 0\0" as *const u8 as *const libc::c_char,
            b"cell has positive area\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(areaRads < areaKm2) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3CellAreaExhaustive.c\0" as *const u8 as *const libc::c_char,
            124 as libc::c_int,
            b"areaRads < areaKm2\0" as *const u8 as *const libc::c_char,
            b"area in rads smaller than area in km2\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(areaKm2 < areaM2) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3CellAreaExhaustive.c\0" as *const u8 as *const libc::c_char,
            125 as libc::c_int,
            b"areaKm2 < areaM2\0" as *const u8 as *const libc::c_char,
            b"area in km2 smaller than area in m2\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn earth_area_test(
    mut res: libc::c_int,
    mut cell_area: Option<unsafe extern "C" fn(H3Index, *mut libc::c_double) -> H3Error>,
    mut target: libc::c_double,
    mut tol: libc::c_double,
) {
    let mut area: libc::c_double = 0.0f64;
    let mut iter: IterCellsResolution = iterInitRes(res);
    while iter.h != 0 {
        let mut cellArea: libc::c_double = 0.;
        if (Some(cell_area.expect("non-null function pointer"))).expect("non-null function pointer")(
            iter.h,
            &mut cellArea,
        ) != 0
        {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testH3CellAreaExhaustive.c\0" as *const u8
                    as *const libc::c_char,
                143 as libc::c_int,
                b"!((*cell_area)(iter.h, &cellArea))\0" as *const u8 as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        area += cellArea;
        iterStepRes(&mut iter);
    }
    if !(fabs(area - target) < tol) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testH3CellAreaExhaustive.c\0" as *const u8 as *const libc::c_char,
            148 as libc::c_int,
            b"fabs(area - target) < tol\0" as *const u8 as *const libc::c_char,
            b"sum of all cells should give earth area\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn runTests() {
    currentTestName = b"haversine_distances\0" as *const u8 as *const libc::c_char;
    iterateAllDirectedEdgesAtRes(
        0 as libc::c_int,
        Some(haversine_assert as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllDirectedEdgesAtRes(
        1 as libc::c_int,
        Some(haversine_assert as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllDirectedEdgesAtRes(
        2 as libc::c_int,
        Some(haversine_assert as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllDirectedEdgesAtRes(
        3 as libc::c_int,
        Some(haversine_assert as unsafe extern "C" fn(H3Index) -> ()),
    );
    currentTestName = b"edge_length\0" as *const u8 as *const libc::c_char;
    iterateAllDirectedEdgesAtRes(
        0 as libc::c_int,
        Some(edge_length_assert as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllDirectedEdgesAtRes(
        1 as libc::c_int,
        Some(edge_length_assert as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllDirectedEdgesAtRes(
        2 as libc::c_int,
        Some(edge_length_assert as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllDirectedEdgesAtRes(
        3 as libc::c_int,
        Some(edge_length_assert as unsafe extern "C" fn(H3Index) -> ()),
    );
    currentTestName = b"cell_area_positive\0" as *const u8 as *const libc::c_char;
    iterateAllIndexesAtRes(
        0 as libc::c_int,
        Some(cell_area_assert as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        1 as libc::c_int,
        Some(cell_area_assert as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        2 as libc::c_int,
        Some(cell_area_assert as unsafe extern "C" fn(H3Index) -> ()),
    );
    iterateAllIndexesAtRes(
        3 as libc::c_int,
        Some(cell_area_assert as unsafe extern "C" fn(H3Index) -> ()),
    );
    currentTestName = b"cell_area_earth\0" as *const u8 as *const libc::c_char;
    let mut rads2: libc::c_double = 4 as libc::c_int as libc::c_double * std::f64::consts::PI;
    let mut km2: libc::c_double = (rads2 * 6371.007180918475 * 6371.007180918475)
        .to_f64()
        .unwrap();
    let mut m2: libc::c_double =
        km2 * 1000 as libc::c_int as libc::c_double * 1000 as libc::c_int as libc::c_double;
    earth_area_test(
        0 as libc::c_int,
        Some(cellAreaRads2 as unsafe extern "C" fn(H3Index, *mut libc::c_double) -> H3Error),
        rads2,
        1e-14f64,
    );
    earth_area_test(
        0 as libc::c_int,
        Some(cellAreaKm2 as unsafe extern "C" fn(H3Index, *mut libc::c_double) -> H3Error),
        km2,
        1e-6f64,
    );
    earth_area_test(
        0 as libc::c_int,
        Some(cellAreaM2 as unsafe extern "C" fn(H3Index, *mut libc::c_double) -> H3Error),
        m2,
        1e0f64,
    );
    earth_area_test(
        1 as libc::c_int,
        Some(cellAreaRads2 as unsafe extern "C" fn(H3Index, *mut libc::c_double) -> H3Error),
        rads2,
        1e-9f64,
    );
    earth_area_test(
        1 as libc::c_int,
        Some(cellAreaKm2 as unsafe extern "C" fn(H3Index, *mut libc::c_double) -> H3Error),
        km2,
        1e-1f64,
    );
    earth_area_test(
        1 as libc::c_int,
        Some(cellAreaM2 as unsafe extern "C" fn(H3Index, *mut libc::c_double) -> H3Error),
        m2,
        1e5f64,
    );
    earth_area_test(
        2 as libc::c_int,
        Some(cellAreaRads2 as unsafe extern "C" fn(H3Index, *mut libc::c_double) -> H3Error),
        rads2,
        1e-12f64,
    );
    earth_area_test(
        2 as libc::c_int,
        Some(cellAreaKm2 as unsafe extern "C" fn(H3Index, *mut libc::c_double) -> H3Error),
        km2,
        1e-5f64,
    );
    earth_area_test(
        2 as libc::c_int,
        Some(cellAreaM2 as unsafe extern "C" fn(H3Index, *mut libc::c_double) -> H3Error),
        m2,
        1e0f64,
    );
    earth_area_test(
        3 as libc::c_int,
        Some(cellAreaRads2 as unsafe extern "C" fn(H3Index, *mut libc::c_double) -> H3Error),
        rads2,
        1e-11f64,
    );
    earth_area_test(
        3 as libc::c_int,
        Some(cellAreaKm2 as unsafe extern "C" fn(H3Index, *mut libc::c_double) -> H3Error),
        km2,
        1e-3f64,
    );
    earth_area_test(
        3 as libc::c_int,
        Some(cellAreaM2 as unsafe extern "C" fn(H3Index, *mut libc::c_double) -> H3Error),
        m2,
        1e3f64,
    );
    earth_area_test(
        4 as libc::c_int,
        Some(cellAreaRads2 as unsafe extern "C" fn(H3Index, *mut libc::c_double) -> H3Error),
        rads2,
        1e-11f64,
    );
    earth_area_test(
        4 as libc::c_int,
        Some(cellAreaKm2 as unsafe extern "C" fn(H3Index, *mut libc::c_double) -> H3Error),
        km2,
        1e-3f64,
    );
    earth_area_test(
        4 as libc::c_int,
        Some(cellAreaM2 as unsafe extern "C" fn(H3Index, *mut libc::c_double) -> H3Error),
        m2,
        1e2f64,
    );
}
unsafe fn main_0() -> libc::c_int {
    currentSuiteName = b"h3CellAreaExhaustive\0" as *const u8 as *const libc::c_char;
    printf(
        b"TEST %s\n\0" as *const u8 as *const libc::c_char,
        b"h3CellAreaExhaustive\0" as *const u8 as *const libc::c_char,
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
