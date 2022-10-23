extern crate unsafe_h3lib_applib;
use ::libc;
extern "C" {

    static mut __stdinp: *mut FILE;
    static mut __stderrp: *mut FILE;
    fn feof(_: *mut FILE) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(_: *mut libc::c_char, _: libc::c_int, _: *mut FILE) -> *mut libc::c_char;
    fn exit(_: libc::c_int) -> !;
    fn setGeoDegs(p: *mut LatLng, latDegs: libc::c_double, lngDegs: libc::c_double);
    fn stringToH3(str: *const libc::c_char, out: *mut H3Index) -> H3Error;
    static mut globalTestCount: libc::c_int;
    static mut currentSuiteName: *const libc::c_char;
    static mut currentTestName: *const libc::c_char;
    fn t_assertBoundary(h3: H3Index, b1: *const CellBoundary);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CellBoundary {
    pub numVerts: libc::c_int,
    pub verts: [LatLng; 10],
}
#[no_mangle]
pub unsafe extern "C" fn readBoundary(mut f: *mut FILE, mut b: *mut CellBoundary) -> libc::c_int {
    let mut buff: [libc::c_char; 256] = [0; 256];
    if (fgets(buff.as_mut_ptr(), 256 as libc::c_int, f)).is_null() {
        if feof(__stdinp) != 0 {
            return -(1 as libc::c_int);
        } else {
            printf(b"reading CellBoundary from input\0" as *const u8 as *const libc::c_char);
            return -(1 as libc::c_int);
        }
    }
    if buff[0 as libc::c_int as usize] as libc::c_int != '{' as i32 {
        printf(b"missing CellBoundary {\0" as *const u8 as *const libc::c_char);
        return -(2 as libc::c_int);
    }
    (*b).numVerts = 0 as libc::c_int;
    loop {
        if (fgets(buff.as_mut_ptr(), 256 as libc::c_int, f)).is_null() {
            printf(b"reading CellBoundary from input\0" as *const u8 as *const libc::c_char);
            return -(3 as libc::c_int);
        }
        if buff[0 as libc::c_int as usize] as libc::c_int == '}' as i32 {
            if !((*b).numVerts == 0 as libc::c_int) {
                break;
            }
            printf(b"reading empty cell boundary\0" as *const u8 as *const libc::c_char);
            return -(4 as libc::c_int);
        } else {
            if (*b).numVerts == 10 as libc::c_int {
                printf(
                    b"too many vertices in CellBoundary from input\0" as *const u8
                        as *const libc::c_char,
                );
                return -(5 as libc::c_int);
            }
            let mut latDegs: libc::c_double = 0.;
            let mut lngDegs: libc::c_double = 0.;
            if sscanf(
                buff.as_mut_ptr(),
                b"%lf %lf\0" as *const u8 as *const libc::c_char,
                &mut latDegs as *mut libc::c_double,
                &mut lngDegs as *mut libc::c_double,
            ) != 2 as libc::c_int
            {
                printf(b"parsing CellBoundary from input\0" as *const u8 as *const libc::c_char);
                return -(6 as libc::c_int);
            }
            setGeoDegs(
                &mut *((*b).verts).as_mut_ptr().offset((*b).numVerts as isize),
                latDegs,
                lngDegs,
            );
            (*b).numVerts += 1;
        }
    }
    return 0 as libc::c_int;
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
    loop {
        if (fgets(buff.as_mut_ptr(), 256 as libc::c_int, __stdinp)).is_null() {
            if feof(__stdinp) != 0 {
                break;
            }
            error(b"reading input H3 index from stdin\0" as *const u8 as *const libc::c_char);
        }
        let mut h3: H3Index = 0;
        if stringToH3(buff.as_mut_ptr(), &mut h3) != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testCellToBoundary.c\0" as *const u8 as *const libc::c_char,
                114 as libc::c_int,
                b"!(stringToH3(buff, &h3))\0" as *const u8 as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        let mut b: CellBoundary = CellBoundary {
            numVerts: 0,
            verts: [LatLng { lat: 0., lng: 0. }; 10],
        };
        readBoundary(__stdinp, &mut b);
        t_assertBoundary(h3, &mut b);
    }
    return 0;
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
        ) as i32)
    }
}
