extern crate unsafe_h3lib;
extern crate unsafe_h3lib_applib;
extern crate unsafe_h3lib_testapps_lib;
use ::libc;
extern "C" {

    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn _ijkMatches(c1: *const CoordIJK, c2: *const CoordIJK) -> libc::c_int;
    fn _neighbor(ijk: *mut CoordIJK, digit: Direction);
    fn ijkToIj(ijk: *const CoordIJK, ij: *mut CoordIJ);
    fn ijToIjk(ij: *const CoordIJ, ijk: *mut CoordIJK) -> H3Error;
    fn ijkToCube(ijk: *mut CoordIJK);
    fn cubeToIjk(ijk: *mut CoordIJK);
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
pub type uint32_t = libc::c_uint;
pub type H3Error = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CoordIJ {
    pub i: libc::c_int,
    pub j: libc::c_int,
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
unsafe extern "C" fn runTests() {
    currentTestName = b"ijkToIj_zero\0" as *const u8 as *const libc::c_char;
    let mut ijk: CoordIJK = {
        let mut init = CoordIJK {
            i: 0 as libc::c_int,
            j: 0,
            k: 0,
        };
        init
    };
    let mut ij: CoordIJ = {
        let mut init = CoordIJ {
            i: 0 as libc::c_int,
            j: 0,
        };
        init
    };
    ijkToIj(&mut ijk, &mut ij);
    if !(ij.i == 0 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCoordIj.c\0" as *const u8 as *const libc::c_char,
            41 as libc::c_int,
            b"ij.i == 0\0" as *const u8 as *const libc::c_char,
            b"ij.i zero\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(ij.j == 0 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCoordIj.c\0" as *const u8 as *const libc::c_char,
            42 as libc::c_int,
            b"ij.j == 0\0" as *const u8 as *const libc::c_char,
            b"ij.j zero\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if ijToIjk(&mut ij, &mut ijk) != 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCoordIj.c\0" as *const u8 as *const libc::c_char,
            44 as libc::c_int,
            b"!(ijToIjk(&ij, &ijk))\0" as *const u8 as *const libc::c_char,
            b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(ijk.i == 0 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCoordIj.c\0" as *const u8 as *const libc::c_char,
            45 as libc::c_int,
            b"ijk.i == 0\0" as *const u8 as *const libc::c_char,
            b"ijk.i zero\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(ijk.j == 0 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCoordIj.c\0" as *const u8 as *const libc::c_char,
            46 as libc::c_int,
            b"ijk.j == 0\0" as *const u8 as *const libc::c_char,
            b"ijk.j zero\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(ijk.k == 0 as libc::c_int) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCoordIj.c\0" as *const u8 as *const libc::c_char,
            47 as libc::c_int,
            b"ijk.k == 0\0" as *const u8 as *const libc::c_char,
            b"ijk.k zero\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"ijkToIj_roundtrip\0" as *const u8 as *const libc::c_char;
    let mut dir: Direction = CENTER_DIGIT;
    while (dir as libc::c_uint) < NUM_DIGITS as libc::c_int as libc::c_uint {
        let mut ijk_0: CoordIJK = {
            let mut init = CoordIJK {
                i: 0 as libc::c_int,
                j: 0,
                k: 0,
            };
            init
        };
        _neighbor(&mut ijk_0, dir);
        let mut ij_0: CoordIJ = {
            let mut init = CoordIJ {
                i: 0 as libc::c_int,
                j: 0,
            };
            init
        };
        ijkToIj(&mut ijk_0, &mut ij_0);
        let mut recovered: CoordIJK = {
            let mut init = CoordIJK {
                i: 0 as libc::c_int,
                j: 0,
                k: 0,
            };
            init
        };
        if ijToIjk(&mut ij_0, &mut recovered) != 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testCoordIj.c\0" as *const u8 as *const libc::c_char,
                59 as libc::c_int,
                b"!(ijToIjk(&ij, &recovered))\0" as *const u8 as *const libc::c_char,
                b"expected E_SUCCESS\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        if _ijkMatches(&mut ijk_0, &mut recovered) == 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testCoordIj.c\0" as *const u8 as *const libc::c_char,
                62 as libc::c_int,
                b"_ijkMatches(&ijk, &recovered)\0" as *const u8 as *const libc::c_char,
                b"got same ijk coordinates back\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        dir += 1;
    }
    currentTestName = b"ijkToCube_roundtrip\0" as *const u8 as *const libc::c_char;
    let mut dir_0: Direction = CENTER_DIGIT;
    while (dir_0 as libc::c_uint) < NUM_DIGITS as libc::c_int as libc::c_uint {
        let mut ijk_1: CoordIJK = {
            let mut init = CoordIJK {
                i: 0 as libc::c_int,
                j: 0,
                k: 0,
            };
            init
        };
        _neighbor(&mut ijk_1, dir_0);
        let mut original: CoordIJK = {
            let mut init = CoordIJK {
                i: ijk_1.i,
                j: ijk_1.j,
                k: ijk_1.k,
            };
            init
        };
        ijkToCube(&mut ijk_1);
        cubeToIjk(&mut ijk_1);
        if _ijkMatches(&mut ijk_1, &mut original) == 0 {
            fprintf(
                __stderrp,
                b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
                currentSuiteName,
                currentTestName,
                b"src/apps/testapps/testCoordIj.c\0" as *const u8 as *const libc::c_char,
                76 as libc::c_int,
                b"_ijkMatches(&ijk, &original)\0" as *const u8 as *const libc::c_char,
                b"got same ijk coordinates back\0" as *const u8 as *const libc::c_char,
            );
            exit(1 as libc::c_int);
        }
        globalTestCount += 1;
        printf(b".\0" as *const u8 as *const libc::c_char);
        dir_0 += 1;
    }
}
unsafe fn main_0() -> libc::c_int {
    currentSuiteName = b"coordIj\0" as *const u8 as *const libc::c_char;
    printf(
        b"TEST %s\n\0" as *const u8 as *const libc::c_char,
        b"coordIj\0" as *const u8 as *const libc::c_char,
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
