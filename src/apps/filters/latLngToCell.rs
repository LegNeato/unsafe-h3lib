extern crate unsafe_h3lib_filters;
use ::libc;
extern "C" {

    static mut __stdinp: *mut FILE;
    static mut __stderrp: *mut FILE;
    fn feof(_: *mut FILE) -> libc::c_int;
    fn fgets(_: *mut libc::c_char, _: libc::c_int, _: *mut FILE) -> *mut libc::c_char;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn error(msg: *const libc::c_char);
    fn latLngToCell(g: *const LatLng, res: libc::c_int, out: *mut H3Index) -> H3Error;
    fn degsToRads(degrees: libc::c_double) -> libc::c_double;
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
pub type C2RustUnnamed = libc::c_uint;
pub const E_OPTION_INVALID: C2RustUnnamed = 15;
pub const E_MEMORY_BOUNDS: C2RustUnnamed = 14;
pub const E_MEMORY_ALLOC: C2RustUnnamed = 13;
pub const E_RES_MISMATCH: C2RustUnnamed = 12;
pub const E_NOT_NEIGHBORS: C2RustUnnamed = 11;
pub const E_DUPLICATE_INPUT: C2RustUnnamed = 10;
pub const E_PENTAGON: C2RustUnnamed = 9;
pub const E_VERTEX_INVALID: C2RustUnnamed = 8;
pub const E_UNDIR_EDGE_INVALID: C2RustUnnamed = 7;
pub const E_DIR_EDGE_INVALID: C2RustUnnamed = 6;
pub const E_CELL_INVALID: C2RustUnnamed = 5;
pub const E_RES_DOMAIN: C2RustUnnamed = 4;
pub const E_LATLNG_DOMAIN: C2RustUnnamed = 3;
pub const E_DOMAIN: C2RustUnnamed = 2;
pub const E_FAILED: C2RustUnnamed = 1;
pub const E_SUCCESS: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LatLng {
    pub lat: libc::c_double,
    pub lng: libc::c_double,
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
pub unsafe extern "C" fn doCoords(
    mut lat: libc::c_double,
    mut lng: libc::c_double,
    mut res: libc::c_int,
) {
    let mut g: LatLng = {
        let mut init = LatLng {
            lat: degsToRads(lat),
            lng: degsToRads(lng),
        };
        init
    };
    let mut h: H3Index = 0;
    let mut e: H3Error = latLngToCell(&mut g, res, &mut h);
    if e == E_SUCCESS as libc::c_int as libc::c_uint {
        h3Println(h);
    } else {
        h3Println(0 as libc::c_int as H3Index);
    };
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut res: libc::c_int = 0 as libc::c_int;
    let mut lat: libc::c_double = 0 as libc::c_int as libc::c_double;
    let mut lng: libc::c_double = 0 as libc::c_int as libc::c_double;
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
    let mut resArg: Arg = {
        let mut init = Arg {
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
        };
        init
    };
    let mut latArg: Arg = {
        let mut init = Arg {
            names: [
                b"--lat\0" as *const u8 as *const libc::c_char,
                b"--latitude\0" as *const u8 as *const libc::c_char,
            ],
            required: false,
            scanFormat: b"%lf\0" as *const u8 as *const libc::c_char,
            valueName: b"lat\0" as *const u8 as *const libc::c_char,
            value: &mut lat as *mut libc::c_double as *mut libc::c_void,
            found: false,
            helpText: b"Latitude in degrees. If not specified, \"latitude longitude\" pairs will be read from standard input.\0"
                as *const u8 as *const libc::c_char,
        };
        init
    };
    let mut lngArg: Arg = {
        let mut init = Arg {
            names: [
                b"--lng\0" as *const u8 as *const libc::c_char,
                b"--longitude\0" as *const u8 as *const libc::c_char,
            ],
            required: false,
            scanFormat: b"%lf\0" as *const u8 as *const libc::c_char,
            valueName: b"lng\0" as *const u8 as *const libc::c_char,
            value: &mut lng as *mut libc::c_double as *mut libc::c_void,
            found: false,
            helpText: b"Longitude in degrees.\0" as *const u8 as *const libc::c_char,
        };
        init
    };
    let mut args: [*mut Arg; 4] = [&mut helpArg, &mut resArg, &mut latArg, &mut lngArg];
    let numArgs: libc::c_int = 4 as libc::c_int;
    let mut helpText: *const libc::c_char =
        b"Convert degrees latitude/longitude coordinates to H3 indexes.\0" as *const u8
            as *const libc::c_char;
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
    if latArg.found as libc::c_int != lngArg.found as libc::c_int {
        printHelp(
            __stderrp,
            *argv.offset(0 as libc::c_int as isize),
            helpText,
            numArgs,
            args.as_mut_ptr(),
            b"Latitude and longitude must both be specified.\0" as *const u8 as *const libc::c_char,
            std::ptr::null::<libc::c_char>(),
        );
        return 1 as libc::c_int;
    }
    if latArg.found {
        doCoords(lat, lng, res);
    } else {
        let mut buff: [libc::c_char; 256] = [0; 256];
        loop {
            if (fgets(buff.as_mut_ptr(), 256 as libc::c_int, __stdinp)).is_null() {
                if feof(__stdinp) != 0 {
                    break;
                }
                error(b"reading lat/lng\0" as *const u8 as *const libc::c_char);
            }
            if sscanf(
                buff.as_mut_ptr(),
                b"%lf %lf\0" as *const u8 as *const libc::c_char,
                &mut lat as *mut libc::c_double,
                &mut lng as *mut libc::c_double,
            ) != 2 as libc::c_int
            {
                error(b"parsing lat/lng\0" as *const u8 as *const libc::c_char);
            }
            doCoords(lat, lng, res);
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
