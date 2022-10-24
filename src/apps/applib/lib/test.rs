use ::libc;
extern "C" {

    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn geoAlmostEqual(p1: *const LatLng, p2: *const LatLng) -> bool;
    fn cellToBoundary(h3: H3Index, gp: *mut CellBoundary) -> H3Error;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CellBoundary {
    pub numVerts: libc::c_int,
    pub verts: [LatLng; 10],
}
#[no_mangle]
pub static mut globalTestCount: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut currentSuiteName: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
#[no_mangle]
pub static mut currentTestName: *const libc::c_char = b"\0" as *const u8 as *const libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn t_assertBoundary(mut h3: H3Index, mut b1: *const CellBoundary) {
    let mut b2: CellBoundary = CellBoundary {
        numVerts: 0,
        verts: [LatLng { lat: 0., lng: 0. }; 10],
    };
    cellToBoundary(h3, &mut b2);
    if (*b1).numVerts != b2.numVerts {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/applib/lib/test.c\0" as *const u8 as *const libc::c_char,
            35 as libc::c_int,
            b"b1->numVerts == b2.numVerts\0" as *const u8 as *const libc::c_char,
            b"expected cell boundary count\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    let mut v: libc::c_int = 0 as libc::c_int;
    while v < (*b1).numVerts {
        if !geoAlmostEqual(
            &*((*b1).verts).as_ptr().offset(v as isize),
            &mut *(b2.verts).as_mut_ptr().offset(v as isize),
        ) {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/applib/lib/test.c\0" as *const u8 as *const libc::c_char,
                38 as libc::c_int,
                b"geoAlmostEqual(&b1->verts[v], &b2.verts[v])\0" as *const u8
                    as *const libc::c_char,
                b"got expected vertex\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        v += 1;
    }
}
