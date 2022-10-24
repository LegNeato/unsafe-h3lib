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

    fn fgets(_: *mut libc::c_char, _: libc::c_int, _: *mut FILE) -> *mut libc::c_char;
    static mut __stdinp: *mut FILE;
    static mut __stderrp: *mut FILE;
    fn feof(_: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn latLngToCell(g: *const LatLng, res: libc::c_int, out: *mut H3Index) -> H3Error;
    fn cellToLatLng(h3: H3Index, g: *mut LatLng) -> H3Error;
    fn getResolution(h: H3Index) -> libc::c_int;
    fn stringToH3(str: *const libc::c_char, out: *mut H3Index) -> H3Error;
    fn geoAlmostEqualThreshold(
        p1: *const LatLng,
        p2: *const LatLng,
        threshold: libc::c_double,
    ) -> bool;
    fn setGeoDegs(p: *mut LatLng, latDegs: libc::c_double, lngDegs: libc::c_double);
    static mut globalTestCount: libc::c_int;
    static mut currentSuiteName: *const libc::c_char;
    static mut currentTestName: *const libc::c_char;
    fn error(msg: *const libc::c_char);
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
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
pub type H3Index = uint64_t;
pub type H3Error = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LatLng {
    pub lat: libc::c_double,
    pub lng: libc::c_double,
}
#[no_mangle]
pub unsafe extern "C" fn assertExpected(mut h1: H3Index, mut g1: *const LatLng) {
    let epsilon: libc::c_double = (0.000001f64 * 0.017_453_292_519_943_295).to_f64().unwrap();
    let mut g2: LatLng = LatLng { lat: 0., lng: 0. };
    cellToLatLng(h1, &mut g2);
    if !geoAlmostEqualThreshold(&mut g2, g1, epsilon) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToLatLng.c\0" as *const u8 as *const libc::c_char,
            43 as libc::c_int,
            b"geoAlmostEqualThreshold(&g2, g1, epsilon)\0" as *const u8 as *const libc::c_char,
            b"got expected cellToLatLng output\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut res: libc::c_int = getResolution(h1);
    let mut h2: H3Index = 0;
    if latLngToCell(&mut g2, res, &mut h2) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToLatLng.c\0" as *const u8 as *const libc::c_char,
            48 as libc::c_int,
            b"!(latLngToCell(&g2, res, &h2))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if h1 != h2 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCellToLatLng.c\0" as *const u8 as *const libc::c_char,
            49 as libc::c_int,
            b"h1 == h2\0" as *const u8 as *const libc::c_char,
            b"got expected latLngToCell output\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    if argc > 1 as libc::c_int {
        fprintf(
            __stderrp,
            b"usage: %s\n\0" as *const u8 as *const libc::c_char,
            *argv.offset(0 as libc::c_int as isize),
        );
        exit(1 as libc::c_int);
    }
    let mut buff: [libc::c_char; 256] = [0; 256];
    let mut h3Str: [libc::c_char; 256] = [0; 256];
    loop {
        if (fgets(buff.as_mut_ptr(), 256 as libc::c_int, __stdinp)).is_null() {
            if feof(__stdinp) != 0 {
                break;
            }
            error(b"reading input from stdin\0" as *const u8 as *const libc::c_char);
        }
        let mut latDegs: libc::c_double = 0.;
        let mut lngDegs: libc::c_double = 0.;
        if sscanf(
            buff.as_mut_ptr(),
            b"%s %lf %lf\0" as *const u8 as *const libc::c_char,
            h3Str.as_mut_ptr(),
            &mut latDegs as *mut libc::c_double,
            &mut lngDegs as *mut libc::c_double,
        ) != 3 as libc::c_int
        {
            error(
                b"parsing input (should be \"H3Index lat lng\")\0" as *const u8
                    as *const libc::c_char,
            );
        }
        let mut h3: H3Index = 0;
        if stringToH3(h3Str.as_mut_ptr(), &mut h3) != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testCellToLatLng.c\0" as *const u8 as *const libc::c_char,
                76 as libc::c_int,
                b"!(stringToH3(h3Str, &h3))\0" as *const u8 as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        let mut coord: LatLng = LatLng { lat: 0., lng: 0. };
        setGeoDegs(&mut coord, latDegs, lngDegs);
        assertExpected(h3, &mut coord);
    }
    0
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ))
    }
}
