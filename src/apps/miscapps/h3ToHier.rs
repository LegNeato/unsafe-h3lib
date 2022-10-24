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

    static mut __stderrp: *mut FILE;
    fn isValidCell(h: H3Index) -> libc::c_int;
    fn isPentagon(h: H3Index) -> libc::c_int;
    fn printHelp(
        out: *mut FILE,
        programName: *const libc::c_char,
        helpText: *const libc::c_char,
        numArgs: libc::c_int,
        args: *mut *mut Arg,
        errorMessage: *const libc::c_char,
        errorDetails: *const libc::c_char,
    );
    fn parseArgs(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        numArgs: libc::c_int,
        args: *mut *mut Arg,
        helpArg: *const Arg,
        helpText: *const libc::c_char,
    ) -> libc::c_int;
    fn h3Println(h: H3Index);
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_off_t = __int64_t;
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
pub unsafe extern "C" fn recursiveH3IndexToHier(mut h: H3Index, mut res: libc::c_int) {
    let mut d: libc::c_int = 0 as libc::c_int;
    while d < 7 as libc::c_int {
        if !(isPentagon(h) != 0 && d == 1 as libc::c_int) {
            h = h & !((7 as libc::c_int as uint64_t)
                << ((15 as libc::c_int - res) * 3 as libc::c_int))
                | (d as uint64_t) << ((15 as libc::c_int - res) * 3 as libc::c_int);
            if res
                == ((h & (15 as libc::c_ulonglong) << 52 as libc::c_int) >> 52 as libc::c_int)
                    as libc::c_int
            {
                h3Println(h);
            } else {
                recursiveH3IndexToHier(h, res + 1 as libc::c_int);
            }
        }
        d += 1;
    }
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut res: libc::c_int = 0;
    let mut parentIndex: H3Index = 0 as libc::c_int as H3Index;
    let mut helpArg: Arg = {
        Arg {
            names: [
                b"-h\0" as *const u8 as *const libc::c_char,
                b"--help\0" as *const u8 as *const libc::c_char,
            ],
            required: false,
            scanFormat: std::ptr::null::<libc::c_char>(),
            valueName: std::ptr::null::<libc::c_char>(),
            value: std::ptr::null_mut::<libc::c_void>(),
            found: false,
            helpText: b"Show this help message.\0" as *const u8 as *const libc::c_char,
        }
    };
    let mut resArg: Arg = {
        Arg {
            names: [
                b"-r\0" as *const u8 as *const libc::c_char,
                b"--resolution\0" as *const u8 as *const libc::c_char,
            ],
            required: 1 as libc::c_int != 0,
            scanFormat: b"%d\0" as *const u8 as *const libc::c_char,
            valueName: b"res\0" as *const u8 as *const libc::c_char,
            value: &mut res as *mut libc::c_int as *mut libc::c_void,
            found: false,
            helpText: b"Resolution, 0-15 inclusive.\0" as *const u8 as *const libc::c_char,
        }
    };
    let mut parentArg: Arg = {
        Arg {
            names: [
                b"-p\0" as *const u8 as *const libc::c_char,
                b"--parent\0" as *const u8 as *const libc::c_char,
            ],
            required: false,
            scanFormat: b"%llx\0" as *const u8 as *const libc::c_char,
            valueName: b"parent\0" as *const u8 as *const libc::c_char,
            value: &mut parentIndex as *mut H3Index as *mut libc::c_void,
            found: false,
            helpText: b"Print only indexes descendent from this index.\0" as *const u8
                as *const libc::c_char,
        }
    };
    let mut args: [*mut Arg; 3] = [&mut helpArg, &mut resArg, &mut parentArg];
    let numArgs: libc::c_int = 3 as libc::c_int;
    let mut helpText: *const libc::c_char =
        b"Print all indexes at the specified resolution\0" as *const u8 as *const libc::c_char;
    if parseArgs(
        argc,
        argv,
        numArgs,
        args.as_mut_ptr(),
        &mut helpArg,
        helpText,
    ) != 0
    {
        return if helpArg.found as libc::c_int != 0 {
            0 as libc::c_int
        } else {
            1 as libc::c_int
        };
    }
    if res > 15 as libc::c_int {
        printHelp(
            __stderrp,
            *argv.offset(0 as libc::c_int as isize),
            helpText,
            numArgs,
            args.as_mut_ptr(),
            b"Resolution exceeds maximum resolution.\0" as *const u8 as *const libc::c_char,
            std::ptr::null::<libc::c_char>(),
        );
        return 1 as libc::c_int;
    }
    if parentArg.found as libc::c_int != 0 && isValidCell(parentIndex) == 0 {
        printHelp(
            __stderrp,
            *argv.offset(0 as libc::c_int as isize),
            helpText,
            numArgs,
            args.as_mut_ptr(),
            b"Parent index is invalid.\0" as *const u8 as *const libc::c_char,
            std::ptr::null::<libc::c_char>(),
        );
        return 1 as libc::c_int;
    }
    if parentArg.found {
        if res
            <= ((parentIndex & (15 as libc::c_ulonglong) << 52 as libc::c_int) >> 52 as libc::c_int)
                as libc::c_int
        {
            h3Println(parentIndex);
        } else {
            let mut rootRes: libc::c_int = ((parentIndex
                & (15 as libc::c_ulonglong) << 52 as libc::c_int)
                >> 52 as libc::c_int) as libc::c_int;
            parentIndex = parentIndex & !((15 as libc::c_ulonglong) << 52 as libc::c_int)
                | (res as uint64_t) << 52 as libc::c_int;
            recursiveH3IndexToHier(parentIndex, rootRes + 1 as libc::c_int);
        }
    } else {
        let mut bc: libc::c_int = 0 as libc::c_int;
        while bc < 122 as libc::c_int {
            let mut rootCell: H3Index = 35184372088831 as libc::c_ulonglong;
            rootCell = rootCell & !((15 as libc::c_int as uint64_t) << 59 as libc::c_int)
                | (1 as libc::c_int as uint64_t) << 59 as libc::c_int;
            rootCell = rootCell & !((127 as libc::c_int as uint64_t) << 45 as libc::c_int)
                | (bc as uint64_t) << 45 as libc::c_int;
            if res == 0 as libc::c_int {
                h3Println(rootCell);
            } else {
                rootCell = rootCell & !((15 as libc::c_ulonglong) << 52 as libc::c_int)
                    | (res as uint64_t) << 52 as libc::c_int;
                recursiveH3IndexToHier(rootCell, 1 as libc::c_int);
            }
            bc += 1;
        }
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
