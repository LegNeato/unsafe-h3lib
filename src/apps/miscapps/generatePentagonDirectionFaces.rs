#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(clippy::missing_safety_doc)]

extern crate unsafe_h3lib_miscapps;
use ::libc;
extern "C" {

    fn exit(_: libc::c_int) -> !;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn getPentagons(res: libc::c_int, out: *mut H3Index) -> H3Error;
    fn getBaseCellNumber(h: H3Index) -> libc::c_int;
    fn h3NeighborRotations(
        origin: H3Index,
        dir: Direction,
        rotations: *mut libc::c_int,
        out: *mut H3Index,
    ) -> H3Error;
    fn _h3ToFaceIjk(h: H3Index, fijk: *mut FaceIJK) -> H3Error;
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_off_t = __int64_t;
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
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
pub type H3Index = uint64_t;
pub type H3Error = uint32_t;
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
unsafe extern "C" fn generate() {
    let mut pentagons: [H3Index; 12] =
        [0 as libc::c_int as H3Index, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    getPentagons(2 as libc::c_int, pentagons.as_mut_ptr());
    printf(
        b"static const PentagonDirectionFaces pentagonDirectionFaces[NUM_PENTAGONS] = {\n\0"
            as *const u8 as *const libc::c_char,
    );
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < 12 as libc::c_int {
        let mut pentagon: H3Index = pentagons[i as usize];
        let mut baseCell: libc::c_int = getBaseCellNumber(pentagon);
        printf(
            b"    {%d, {\0" as *const u8 as *const libc::c_char,
            baseCell,
        );
        let mut fijk: FaceIJK = FaceIJK {
            face: 0,
            coord: CoordIJK { i: 0, j: 0, k: 0 },
        };
        let mut dir: Direction = J_AXES_DIGIT;
        while (dir as libc::c_uint) < NUM_DIGITS as libc::c_int as libc::c_uint {
            let mut r: libc::c_int = 0 as libc::c_int;
            let mut neighbor: H3Index = 0;
            h3NeighborRotations(pentagon, dir, &mut r, &mut neighbor);
            _h3ToFaceIjk(neighbor, &mut fijk);
            if dir as libc::c_uint > J_AXES_DIGIT as libc::c_int as libc::c_uint {
                printf(b", \0" as *const u8 as *const libc::c_char);
            }
            printf(b"%d\0" as *const u8 as *const libc::c_char, fijk.face);
            dir += 1;
        }
        printf(b"}},\n\0" as *const u8 as *const libc::c_char);
        i += 1;
    }
    printf(b"};\n\0" as *const u8 as *const libc::c_char);
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
    generate();
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
