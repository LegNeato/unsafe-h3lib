#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![allow(clippy::missing_safety_doc)]

extern crate unsafe_h3lib_filters;
use ::libc;
extern "C" {

    fn fgets(_: *mut libc::c_char, _: libc::c_int, _: *mut FILE) -> *mut libc::c_char;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    static mut __stdinp: *mut FILE;
    fn feof(_: *mut FILE) -> libc::c_int;
    fn parseArgs(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        numArgs: libc::c_int,
        args: *mut *mut Arg,
        helpArg: *const Arg,
        helpText: *const libc::c_char,
    ) -> libc::c_int;
    fn error(msg: *const libc::c_char);
    fn stringToH3(str: *const libc::c_char, out: *mut H3Index) -> H3Error;
    fn h3ToString(h: H3Index, str: *mut libc::c_char, sz: size_t) -> H3Error;
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_off_t = __int64_t;
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
pub type size_t = __darwin_size_t;
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
pub struct Arg {
    pub names: [*const libc::c_char; 2],
    pub required: bool,
    pub scanFormat: *const libc::c_char,
    pub valueName: *const libc::c_char,
    pub value: *mut libc::c_void,
    pub found: bool,
    pub helpText: *const libc::c_char,
}
#[no_mangle]
pub unsafe extern "C" fn resDigitToChar(mut d: libc::c_int) -> libc::c_char {
    if d < 0 as libc::c_int || d > 7 as libc::c_int {
        return 'x' as i32 as libc::c_char;
    }
    return ('0' as i32 + d) as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn doCell(mut h: H3Index, mut verboseMode: bool) {
    let mut h3Mode: libc::c_int = ((h & (15 as libc::c_int as uint64_t) << 59 as libc::c_int)
        >> 59 as libc::c_int) as libc::c_int;
    let mut h3Res: libc::c_int =
        ((h & (15 as libc::c_ulonglong) << 52 as libc::c_int) >> 52 as libc::c_int) as libc::c_int;
    let mut h3BaseCell: libc::c_int = ((h & (127 as libc::c_int as uint64_t) << 45 as libc::c_int)
        >> 45 as libc::c_int) as libc::c_int;
    if verboseMode {
        let mut modes: [*const libc::c_char; 16] = [
            b"RESERVED\0" as *const u8 as *const libc::c_char,
            b"Cell\0" as *const u8 as *const libc::c_char,
            b"Directed Edge\0" as *const u8 as *const libc::c_char,
            b"INVALID\0" as *const u8 as *const libc::c_char,
            b"INVALID\0" as *const u8 as *const libc::c_char,
            b"INVALID\0" as *const u8 as *const libc::c_char,
            b"INVALID\0" as *const u8 as *const libc::c_char,
            b"INVALID\0" as *const u8 as *const libc::c_char,
            b"INVALID\0" as *const u8 as *const libc::c_char,
            b"INVALID\0" as *const u8 as *const libc::c_char,
            b"INVALID\0" as *const u8 as *const libc::c_char,
            b"INVALID\0" as *const u8 as *const libc::c_char,
            b"INVALID\0" as *const u8 as *const libc::c_char,
            b"INVALID\0" as *const u8 as *const libc::c_char,
            b"INVALID\0" as *const u8 as *const libc::c_char,
            b"INVALID\0" as *const u8 as *const libc::c_char,
        ];
        printf(
            b"\xE2\x95\x94\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x97\n\0"
                as *const u8 as *const libc::c_char,
        );
        let mut hStr: [libc::c_char; 256] = [0; 256];
        h3ToString(h, hStr.as_mut_ptr(), 256 as libc::c_int as size_t);
        printf(
            b"\xE2\x95\x91 H3Index    \xE2\x95\x91 %s\n\0" as *const u8 as *const libc::c_char,
            hStr.as_mut_ptr(),
        );
        printf(
            b"\xE2\x95\xA0\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\xA3\n\0"
                as *const u8 as *const libc::c_char,
        );
        printf(
            b"\xE2\x95\x91 Mode       \xE2\x95\x91 %s (%i)\n\0" as *const u8 as *const libc::c_char,
            modes[h3Mode as usize],
            h3Mode,
        );
        printf(
            b"\xE2\x95\x91 Resolution \xE2\x95\x91 %i\n\0" as *const u8 as *const libc::c_char,
            h3Res,
        );
        if h3Mode == 2 as libc::c_int {
            printf(
                b"\xE2\x95\x91 Edge       \xE2\x95\x91 %i\n\0" as *const u8 as *const libc::c_char,
                ((h & (7 as libc::c_int as uint64_t) << 56 as libc::c_int) >> 56 as libc::c_int)
                    as libc::c_int,
            );
        }
        printf(
            b"\xE2\x95\x91 Base Cell  \xE2\x95\x91 %i\n\0" as *const u8 as *const libc::c_char,
            h3BaseCell,
        );
        let mut i: libc::c_int = 1 as libc::c_int;
        while i <= h3Res {
            printf(
                b"\xE2\x95\x91%3i Child   \xE2\x95\x91 %c\n\0" as *const u8 as *const libc::c_char,
                i,
                resDigitToChar(
                    (h >> (15 as libc::c_int - i) * 3 as libc::c_int & 7 as libc::c_int as uint64_t)
                        as Direction as libc::c_int,
                ) as libc::c_int,
            );
            i += 1;
        }
        printf(
            b"\xE2\x95\x9A\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x90\xE2\x95\x9D\n\n\0"
                as *const u8 as *const libc::c_char,
        );
    } else if h3Mode == 1 as libc::c_int {
        printf(
            b"%d:%d:%d:\0" as *const u8 as *const libc::c_char,
            h3Mode,
            h3Res,
            h3BaseCell,
        );
        let mut i_0: libc::c_int = 1 as libc::c_int;
        while i_0 <= h3Res {
            printf(
                b"%c\0" as *const u8 as *const libc::c_char,
                resDigitToChar(
                    (h >> (15 as libc::c_int - i_0) * 3 as libc::c_int
                        & 7 as libc::c_int as uint64_t) as Direction
                        as libc::c_int,
                ) as libc::c_int,
            );
            i_0 += 1;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    } else if h3Mode == 2 as libc::c_int {
        printf(
            b"%d:%d:%d:%d:\0" as *const u8 as *const libc::c_char,
            h3Mode,
            ((h & (7 as libc::c_int as uint64_t) << 56 as libc::c_int) >> 56 as libc::c_int)
                as libc::c_int,
            h3Res,
            h3BaseCell,
        );
        let mut i_1: libc::c_int = 1 as libc::c_int;
        while i_1 <= h3Res {
            printf(
                b"%c\0" as *const u8 as *const libc::c_char,
                resDigitToChar(
                    (h >> (15 as libc::c_int - i_1) * 3 as libc::c_int
                        & 7 as libc::c_int as uint64_t) as Direction
                        as libc::c_int,
                ) as libc::c_int,
            );
            i_1 += 1;
        }
        printf(b"\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"INVALID INDEX\n\0" as *const u8 as *const libc::c_char);
    };
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut helpArg: Arg = {
        let mut init = Arg {
            names: [
                b"-h\0" as *const u8 as *const libc::c_char,
                b"--help\0" as *const u8 as *const libc::c_char,
            ],
            required: false,
            scanFormat: std::ptr::null::<libc::c_char>(),
            valueName: std::ptr::null::<libc::c_char>(),
            value: 0 as *mut libc::c_void,
            found: false,
            helpText: b"Show this help message.\0" as *const u8 as *const libc::c_char,
        };
        init
    };
    let mut verboseArg: Arg = {
        let mut init = Arg {
            names: [
                b"-v\0" as *const u8 as *const libc::c_char,
                b"--verbose\0" as *const u8 as *const libc::c_char,
            ],
            required: false,
            scanFormat: std::ptr::null::<libc::c_char>(),
            valueName: std::ptr::null::<libc::c_char>(),
            value: 0 as *mut libc::c_void,
            found: false,
            helpText: b"Verbose output mode.\0" as *const u8 as *const libc::c_char,
        };
        init
    };
    let mut index: H3Index = 0 as libc::c_int as H3Index;
    let mut indexArg: Arg = {
        let mut init = Arg {
            names: [
                b"-i\0" as *const u8 as *const libc::c_char,
                b"--index\0" as *const u8 as *const libc::c_char,
            ],
            required: false,
            scanFormat: b"%llx\0" as *const u8 as *const libc::c_char,
            valueName: b"index\0" as *const u8 as *const libc::c_char,
            value: &mut index as *mut H3Index as *mut libc::c_void,
            found: false,
            helpText: b"Index, or not specified to read indexes from standard input.\0" as *const u8
                as *const libc::c_char,
        };
        init
    };
    let numArgs: libc::c_int = 3 as libc::c_int;
    let mut args: [*mut Arg; 3] = [&mut helpArg, &mut verboseArg, &mut indexArg];
    if parseArgs(
        argc,
        argv,
        numArgs,
        args.as_mut_ptr(),
        &mut helpArg,
        b"Converts H3 indexes to component parts\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        return if helpArg.found as libc::c_int != 0 {
            0 as libc::c_int
        } else {
            1 as libc::c_int
        };
    }
    if indexArg.found {
        doCell(index, verboseArg.found);
    } else {
        let mut buff: [libc::c_char; 256] = [0; 256];
        loop {
            if (fgets(buff.as_mut_ptr(), 256 as libc::c_int, __stdinp)).is_null() {
                if feof(__stdinp) != 0 {
                    break;
                }
                error(b"reading H3 index from stdin\0" as *const u8 as *const libc::c_char);
            }
            let mut h3: H3Index = 0;
            stringToH3(buff.as_mut_ptr(), &mut h3);
            doCell(h3, verboseArg.found);
        }
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
