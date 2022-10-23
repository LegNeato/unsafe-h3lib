extern crate unsafe_h3lib_testapps;
use ::libc;
extern "C" {
    static mut __stderrp: *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn latLngToCell(g: *const LatLng, res: libc::c_int, out: *mut H3Index) -> H3Error;
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
    fn randomGeo(p: *mut LatLng);
    fn geoPrintlnNoFmt(p: *const LatLng);
    fn h3Print(h: H3Index);
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
pub struct Arg {
    pub names: [*const libc::c_char; 2],
    pub required: bool,
    pub scanFormat: *const libc::c_char,
    pub valueName: *const libc::c_char,
    pub value: *mut libc::c_void,
    pub found: bool,
    pub helpText: *const libc::c_char,
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut res: libc::c_int = 0 as libc::c_int;
    let mut numPoints: libc::c_int = 0 as libc::c_int;
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
    let mut numPointsArg: Arg = {
        let mut init = Arg {
            names: [
                b"-n\0" as *const u8 as *const libc::c_char,
                b"--num-points\0" as *const u8 as *const libc::c_char,
            ],
            required: 1 as libc::c_int != 0,
            scanFormat: b"%d\0" as *const u8 as *const libc::c_char,
            valueName: b"num\0" as *const u8 as *const libc::c_char,
            value: &mut numPoints as *mut libc::c_int as *mut libc::c_void,
            found: false,
            helpText: b"Number of random lat/lng pairs to generate.\0" as *const u8
                as *const libc::c_char,
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
    let mut args: [*mut Arg; 3] = [&mut helpArg, &mut numPointsArg, &mut resArg];
    let numArgs: libc::c_int = 3 as libc::c_int;
    let mut helpText: *const libc::c_char =
        b"Generates random lat/lng pairs and indexes them at the specified resolution.\0"
            as *const u8 as *const libc::c_char;
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
    let mut i: libc::c_int = 0 as libc::c_int;
    while i < numPoints {
        let mut g: LatLng = LatLng { lat: 0., lng: 0. };
        randomGeo(&mut g);
        let mut h: H3Index = 0;
        if latLngToCell(&mut g, res, &mut h) == 0 {
            h3Print(h);
            printf(b" \0" as *const u8 as *const libc::c_char);
            geoPrintlnNoFmt(&mut g);
        }
        i += 1;
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
