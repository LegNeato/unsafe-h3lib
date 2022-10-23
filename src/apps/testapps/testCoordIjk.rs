extern crate unsafe_h3lib;
extern crate unsafe_h3lib_applib;
extern crate unsafe_h3lib_testapps_lib;
use ::libc;
extern "C" {

    fn exit(_: libc::c_int) -> !;
    static mut __stderrp: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn _ijkMatches(c1: *const CoordIJK, c2: *const CoordIJK) -> libc::c_int;
    fn _unitIjkToDigit(ijk: *const CoordIJK) -> Direction;
    fn _neighbor(ijk: *mut CoordIJK, digit: Direction);
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
    currentTestName = b"_unitIjkToDigit\0" as *const u8 as *const libc::c_char;
    let mut zero: CoordIJK = {
        let mut init = CoordIJK {
            i: 0 as libc::c_int,
            j: 0,
            k: 0,
        };
        init
    };
    let mut i: CoordIJK = {
        let mut init = CoordIJK {
            i: 1 as libc::c_int,
            j: 0 as libc::c_int,
            k: 0 as libc::c_int,
        };
        init
    };
    let mut outOfRange: CoordIJK = {
        let mut init = CoordIJK {
            i: 2 as libc::c_int,
            j: 0 as libc::c_int,
            k: 0 as libc::c_int,
        };
        init
    };
    let mut unnormalizedZero: CoordIJK = {
        let mut init = CoordIJK {
            i: 2 as libc::c_int,
            j: 2 as libc::c_int,
            k: 2 as libc::c_int,
        };
        init
    };
    if !(_unitIjkToDigit(&mut zero) as libc::c_uint == CENTER_DIGIT as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCoordIjk.c\0" as *const u8 as *const libc::c_char,
            32 as libc::c_int,
            b"_unitIjkToDigit(&zero) == CENTER_DIGIT\0" as *const u8 as *const libc::c_char,
            b"Unit IJK to zero\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(_unitIjkToDigit(&mut i) as libc::c_uint == I_AXES_DIGIT as libc::c_int as libc::c_uint) {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCoordIjk.c\0" as *const u8 as *const libc::c_char,
            33 as libc::c_int,
            b"_unitIjkToDigit(&i) == I_AXES_DIGIT\0" as *const u8 as *const libc::c_char,
            b"Unit IJK to I axis\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(_unitIjkToDigit(&mut outOfRange) as libc::c_uint
        == INVALID_DIGIT as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCoordIjk.c\0" as *const u8 as *const libc::c_char,
            35 as libc::c_int,
            b"_unitIjkToDigit(&outOfRange) == INVALID_DIGIT\0" as *const u8 as *const libc::c_char,
            b"Unit IJK out of range\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    if !(_unitIjkToDigit(&mut unnormalizedZero) as libc::c_uint
        == CENTER_DIGIT as libc::c_int as libc::c_uint)
    {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCoordIjk.c\0" as *const u8 as *const libc::c_char,
            37 as libc::c_int,
            b"_unitIjkToDigit(&unnormalizedZero) == CENTER_DIGIT\0" as *const u8
                as *const libc::c_char,
            b"Unnormalized unit IJK to zero\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    currentTestName = b"_neighbor\0" as *const u8 as *const libc::c_char;
    let mut ijk: CoordIJK = {
        let mut init = CoordIJK {
            i: 0 as libc::c_int,
            j: 0,
            k: 0,
        };
        init
    };
    let mut zero_0: CoordIJK = {
        let mut init = CoordIJK {
            i: 0 as libc::c_int,
            j: 0,
            k: 0,
        };
        init
    };
    let mut i_0: CoordIJK = {
        let mut init = CoordIJK {
            i: 1 as libc::c_int,
            j: 0 as libc::c_int,
            k: 0 as libc::c_int,
        };
        init
    };
    _neighbor(&mut ijk, CENTER_DIGIT);
    if _ijkMatches(&mut ijk, &mut zero_0) == 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCoordIjk.c\0" as *const u8 as *const libc::c_char,
            47 as libc::c_int,
            b"_ijkMatches(&ijk, &zero)\0" as *const u8 as *const libc::c_char,
            b"Center neighbor is self\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    _neighbor(&mut ijk, I_AXES_DIGIT);
    if _ijkMatches(&mut ijk, &mut i_0) == 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCoordIjk.c\0" as *const u8 as *const libc::c_char,
            49 as libc::c_int,
            b"_ijkMatches(&ijk, &i)\0" as *const u8 as *const libc::c_char,
            b"I neighbor as expected\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
    _neighbor(&mut ijk, INVALID_DIGIT);
    if _ijkMatches(&mut ijk, &mut i_0) == 0 {
        fprintf(
            __stderrp,
            b"%s.%s: t_assert failed at %s:%d, %s, %s\n\0" as *const u8 as *const libc::c_char,
            currentSuiteName,
            currentTestName,
            b"src/apps/testapps/testCoordIjk.c\0" as *const u8 as *const libc::c_char,
            51 as libc::c_int,
            b"_ijkMatches(&ijk, &i)\0" as *const u8 as *const libc::c_char,
            b"Invalid neighbor is self\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    globalTestCount += 1;
    printf(b".\0" as *const u8 as *const libc::c_char);
}
unsafe fn main_0() -> libc::c_int {
    currentSuiteName = b"coordIjk\0" as *const u8 as *const libc::c_char;
    printf(
        b"TEST %s\n\0" as *const u8 as *const libc::c_char,
        b"coordIjk\0" as *const u8 as *const libc::c_char,
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
