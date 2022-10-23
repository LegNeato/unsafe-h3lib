use ::libc;
extern "C" {
    pub type __sFILEX;
    static mut __stdinp: *mut FILE;
    static mut __stderrp: *mut FILE;
    fn feof(_: *mut FILE) -> libc::c_int;
    fn fgets(_: *mut libc::c_char, _: libc::c_int, _: *mut FILE) -> *mut libc::c_char;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn stringToH3(str: *const libc::c_char, out: *mut H3Index) -> H3Error;
    fn isValidCell(h: H3Index) -> libc::c_int;
    fn cellToLocalIj(origin: H3Index, h3: H3Index, mode: uint32_t, out: *mut CoordIJ) -> H3Error;
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
    fn error(msg: *const libc::c_char);
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
    pub _extra: *mut __sFILEX,
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
pub struct CoordIJ {
    pub i: libc::c_int,
    pub j: libc::c_int,
}
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
pub unsafe extern "C" fn doCell(mut h: H3Index, mut origin: H3Index) {
    let mut ij: CoordIJ = CoordIJ { i: 0, j: 0 };
    if cellToLocalIj(origin, h, 0 as libc::c_int as uint32_t, &mut ij) != 0 {
        printf(b"NA\n\0" as *const u8 as *const libc::c_char);
    } else {
        printf(b"%d %d\n\0" as *const u8 as *const libc::c_char, ij.i, ij.j);
    };
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut origin: H3Index = 0 as libc::c_int as H3Index;
    let mut helpArg: Arg = {
        let mut init = Arg {
            names: [
                b"-h\0" as *const u8 as *const libc::c_char,
                b"--help\0" as *const u8 as *const libc::c_char,
            ],
            required: false,
            scanFormat: 0 as *const libc::c_char,
            valueName: 0 as *const libc::c_char,
            value: 0 as *mut libc::c_void,
            found: false,
            helpText: b"Show this help message.\0" as *const u8 as *const libc::c_char,
        };
        init
    };
    let mut originArg: Arg = {
        let mut init = Arg {
            names: [
                b"-o\0" as *const u8 as *const libc::c_char,
                b"--origin\0" as *const u8 as *const libc::c_char,
            ],
            required: 1 as libc::c_int != 0,
            scanFormat: b"%llx\0" as *const u8 as *const libc::c_char,
            valueName: b"origin\0" as *const u8 as *const libc::c_char,
            value: &mut origin as *mut H3Index as *mut libc::c_void,
            found: false,
            helpText: b"Origin (anchoring index) for the local coordinate system.\0" as *const u8
                as *const libc::c_char,
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
    let mut args: [*mut Arg; 3] = [&mut helpArg, &mut originArg, &mut indexArg];
    let numArgs: libc::c_int = 3 as libc::c_int;
    let mut helpText: *const libc::c_char =
        b"Converts H3 indexes to local IJ coordinates\0" as *const u8 as *const libc::c_char;
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
    if isValidCell(origin) == 0 {
        printHelp(
            __stderrp,
            *argv.offset(0 as libc::c_int as isize),
            helpText,
            numArgs,
            args.as_mut_ptr(),
            b"Origin is invalid.\0" as *const u8 as *const libc::c_char,
            0 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    if indexArg.found {
        doCell(index, origin);
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
            doCell(h3, origin);
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
