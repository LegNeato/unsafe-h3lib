use ::libc;
extern "C" {
    
    static mut __stdinp: *mut FILE;
    fn feof(_: *mut FILE) -> libc::c_int;
    fn fgets(_: *mut libc::c_char, _: libc::c_int, _: *mut FILE) -> *mut libc::c_char;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn maxGridDiskSize(k: libc::c_int, out: *mut int64_t) -> H3Error;
    fn gridDiskDistances(
        origin: H3Index,
        k: libc::c_int,
        out: *mut H3Index,
        distances: *mut libc::c_int,
    ) -> H3Error;
    fn stringToH3(str: *const libc::c_char, out: *mut H3Index) -> H3Error;
    fn parseArgs(
        argc: libc::c_int,
        argv: *mut *mut libc::c_char,
        numArgs: libc::c_int,
        args: *mut *mut Arg,
        helpArg: *const Arg,
        helpText: *const libc::c_char,
    ) -> libc::c_int;
    fn error(msg: *const libc::c_char);
    fn h3Print(h: H3Index);
}
pub type __int64_t = libc::c_longlong;
pub type __darwin_off_t = __int64_t;
pub type int64_t = libc::c_longlong;
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
pub unsafe extern "C" fn doCell(
    mut h: H3Index,
    mut k: libc::c_int,
    mut printDistances: libc::c_int,
) {
    let mut maxSize: int64_t = 0;
    maxGridDiskSize(k, &mut maxSize);
    let mut rings: *mut H3Index = calloc(
        maxSize as libc::c_ulong,
        ::core::mem::size_of::<H3Index>() as libc::c_ulong,
    ) as *mut H3Index;
    let mut distances: *mut libc::c_int = calloc(
        maxSize as libc::c_ulong,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
    ) as *mut libc::c_int;
    gridDiskDistances(h, k, rings, distances);
    let mut i: int64_t = 0 as libc::c_int as int64_t;
    while i < maxSize {
        if *rings.offset(i as isize) != 0 as libc::c_int as libc::c_ulonglong {
            h3Print(*rings.offset(i as isize));
            if printDistances != 0 {
                printf(
                    b" %d\n\0" as *const u8 as *const libc::c_char,
                    *distances.offset(i as isize),
                );
            } else {
                printf(b"\n\0" as *const u8 as *const libc::c_char);
            }
        }
        i += 1;
    }
    free(distances as *mut libc::c_void);
    free(rings as *mut libc::c_void);
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut k: libc::c_int = 0 as libc::c_int;
    let mut origin: H3Index = 0 as libc::c_int as H3Index;
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
    let mut kArg: Arg = {
        let mut init = Arg {
            names: [
                b"-k\0" as *const u8 as *const libc::c_char,
                std::ptr::null::<libc::c_char>(),
            ],
            required: 1 as libc::c_int != 0,
            scanFormat: b"%d\0" as *const u8 as *const libc::c_char,
            valueName: b"k\0" as *const u8 as *const libc::c_char,
            value: &mut k as *mut libc::c_int as *mut libc::c_void,
            found: false,
            helpText: b"Radius in hexagons.\0" as *const u8 as *const libc::c_char,
        };
        init
    };
    let mut printDistancesArg: Arg = {
        let mut init = Arg {
            names: [
                b"-d\0" as *const u8 as *const libc::c_char,
                b"--print-distances\0" as *const u8 as *const libc::c_char,
            ],
            required: false,
            scanFormat: std::ptr::null::<libc::c_char>(),
            valueName: std::ptr::null::<libc::c_char>(),
            value: 0 as *mut libc::c_void,
            found: false,
            helpText: b"Print distance from origin after each index.\0" as *const u8
                as *const libc::c_char,
        };
        init
    };
    let mut originArg: Arg = {
        let mut init = Arg {
            names: [
                b"-o\0" as *const u8 as *const libc::c_char,
                b"--origin\0" as *const u8 as *const libc::c_char,
            ],
            required: false,
            scanFormat: b"%llx\0" as *const u8 as *const libc::c_char,
            valueName: b"origin\0" as *const u8 as *const libc::c_char,
            value: &mut origin as *mut H3Index as *mut libc::c_void,
            found: false,
            helpText: b"Origin, or not specified to read origins from standard input.\0"
                as *const u8 as *const libc::c_char,
        };
        init
    };
    let mut args: [*mut Arg; 4] = [
        &mut helpArg,
        &mut kArg,
        &mut printDistancesArg,
        &mut originArg,
    ];
    if parseArgs(
        argc,
        argv,
        4 as libc::c_int,
        args.as_mut_ptr(),
        &mut helpArg,
        b"Print indexes k distance away from the origin\0" as *const u8 as *const libc::c_char,
    ) != 0
    {
        return if helpArg.found as libc::c_int != 0 {
            0 as libc::c_int
        } else {
            1 as libc::c_int
        };
    }
    if originArg.found {
        doCell(origin, k, printDistancesArg.found as libc::c_int);
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
            doCell(h3, k, printDistancesArg.found as libc::c_int);
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
